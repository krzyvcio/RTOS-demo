# Tickless Idle

## Definicja

Mechanizm zarządzania energią w systemach czasu rzeczywistego, który eliminuje regularne przerwania zegarowe (timer ticks) podczas bezczynności procesora, pozwalając na dłuższe okresy uśpienia.

______________________________________________________________________

## Problem regularnych ticków

```
┌───────────────────────────────────────────────────────────────┐
│              TRADYCYJNY TIMER TICK (100Hz)                    │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  LINIA CZASU (tick co 10ms):                                  │
│                                                               │
│  IDLE: ──────────────────────────────────────────────────    │
│         ↑    ↑    ↑    ↑    ↑    ↑    ↑    ↑    ↑           │
│        tick tick tick tick tick tick tick tick tick          │
│                                                               │
│  PROBLEM:                                                     │
│  ├── Co 10ms procesor budzi się na tick                      │
│  ├── Nie może spać głębiej niż 10ms                          │
│  ├── Zmarnowana energia na przebudzenia                      │
│  └── Cache jest flushowany przy każdym przebudzeniu         │
│                                                               │
│  ZUŻYCIE ENERGII:                                             │
│  ├── Idle: 10mA × 10ms = energii na przebudzenie             │
│  └── Deep Sleep: 10μA × nieskończoność = znacznie mniej!     │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Tickless Idle - rozwiązanie

```
┌───────────────────────────────────────────────────────────────┐
│                    TICKLESS IDLE                              │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  Sytuacja: System idle, następne zadanie za 150ms            │
│                                                               │
│  TRADYCYJNY:                                                  │
│  ├── Tick @ 10ms, 20ms, 30ms, ..., 150ms                     │
│  └── 15 przebudzeń zamiast 1                                  │
│                                                               │
│  TICKLESS:                                                    │
│  ├── Ustaw timer na 150ms                                     │
│  ├── Wejdź w deep sleep                                       │
│  ├── Jedno przebudzenie po 150ms                              │
│  └── Oszczędność: ~15x mniej energii!                         │
│                                                               │
│  LINIA CZASU:                                                 │
│  IDLE: ────────────────────────────────────────[TIMER]──     │
│                                                  ↑            │
│                                           Jedno przebudzenie  │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### IoT i Wearables

| Urządzenie | Wymaganie | Oszczędność z tickless |
|------------|-----------|------------------------|
| Smart watch | Dni na baterii | 30-50% oszczędności |
| Sensor node | Miesiące na baterii | 70-90% oszczędności |
| Tracker | Ruch + GPS | 40-60% oszczędności |

**Przykład:** Fitbit, Apple Watch - tickless idle fundamentalne dla czasu pracy

### Automotive (Start-Stop)

- **ECU w trybie sleep** - oszczędność baterii
- **Keyless entry** - budzenie na żądanie
- **Infotainment** - sleep gdy auto wyłączone

### Medical Devices

- **Pacemaker** - lata na baterii
- **Insulin pump** - tygodnie na baterii
- **Hearing aids** - dni na baterii

______________________________________________________________________

## Tryby uśpienia procesora

```
┌─────────────────────────────────────────────────────────────┐
│                   POWER MODES (ARM Cortex-M)               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ACTIVE:                                                    │
│  ├── Wszystkie zegary aktywne                              │
│  ├── Pełna wydajność                                       │
│  └── Pobór: ~50-100mA                                      │
│                                                             │
│  SLEEP:                                                     │
│  ├── CPU zatrzymany, peryferia aktywne                     │
│  ├── Szybki wake-up (~10μs)                                │
│  └── Pobór: ~1-5mA                                         │
│                                                             │
│  DEEP SLEEP:                                                │
│  ├── Większość zegarów wyłączona                           │
│  ├── Tylko RTC lub watchdog aktywny                        │
│  ├── Wake-up: ~100μs - 1ms                                 │
│  └── Pobór: ~10-100μA                                      │
│                                                             │
│  STANDBY / HIBERNATE:                                       │
│  ├── Tylko backup domain (RAM retention)                   │
│  ├── Wake-up: ~1-10ms                                      │
│  └── Pobór: ~1-10μA                                        │
│                                                             │
│  SHUTDOWN:                                                  │
│  ├── Całkowite wyłączenie                                  │
│  ├── Wake-up: reset (~10-100ms)                            │
│  └── Pobór: ~0.1-1μA                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Implementacja

### FreeRTOS Tickless Idle

```c
// Konfiguracja w FreeRTOSConfig.h
#define configUSE_TICKLESS_IDLE         1
#define configEXPECTED_IDLE_TIME_BEFORE_SLEEP   2  // Minimalne ticki

// Callback - zdefiniowany przez użytkownika
void vPortSuppressTicksAndSleep(TickType_t xExpectedIdleTime) {
    // 1. Sprawdź czy można wejść w sleep
    // 2. Oblicz czas uśpienia
    // 3. Skonfiguruj wake-up timer
    // 4. Wejdź w deep sleep
    // 5. Po przebudzeniu: koryguj system tick

    uint32_t ulLowPowerTimeBeforeSleep, ulLowPowerTimeAfterSleep;

    // Zapisz czas
    ulLowPowerTimeBeforeSleep = ulGetExternalTime();

    // Skonfiguruj wake-up
    vSetWakeUpInterrupt(xExpectedIdleTime);

    // Wejdź w deep sleep
    __WFI();  // Wait For Interrupt

    // Po przebudzeniu
    ulLowPowerTimeAfterSleep = ulGetExternalTime();

    // Koryguj system tick
    vTaskStepTick(ulLowPowerTimeAfterSleep - ulLowPowerTimeBeforeSleep);
}
```

### Zephyr RTOS

```c
// W Kconfig
CONFIG_TICKLESS_IDLE=y
CONFIG_TICKLESS_IDLE_THRESH=3  // Minimal ticks for tickless

// Automatyczne w Zephyr - nic więcej nie trzeba!
// System sam zarządza power management
```

### Linux (NO_HZ)

```bash
# Parametry kernela
CONFIG_NO_HZ=y          # Tickless when idle
CONFIG_NO_HZ_IDLE=y     # Tickless idle
CONFIG_NO_HZ_FULL=y     # Tickless on specific CPUs (for RT)

# Boot parameters
nohz=full nohz_full=2,3 rcu_nocbs=2,3
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Jitter po wake-up** | Czas przebudzenia zmienny | Dokładny low-power timer |
| **Latency wake-up** | Opóźnienie wyjścia ze sleep | Wybór odpowiedniego trybu |
| **Timer coalescing** | Zbliżone timery = multiple wake-ups | Łączenie timerów |
| **Peripheral state** | Peryferia może wymagać reinicjalizacji | Backup/restore |
| **Debug difficulty** | Trudne debugowanie w sleep | Logi, trace |

______________________________________________________________________

## Kierunki rozwoju

### 1. Predictive Tickless

- ML/AI do przewidywania długości idle
- Inteligentne wybieranie trybu sleep

### 2. Peripheral-Aware Power

- Automatyczne wyłączanie peryferiów
- Power domains management

### 3. Energy Harvesting

- Systemy zasilane z otoczenia
- Tickless fundamentalne dla energy harvesting

______________________________________________________________________

## Porównanie implementacji

| System | Tickless | Konfiguracja | Low-power modes |
|--------|----------|--------------|-----------------|
| **FreeRTOS** | Tak | configUSE_TICKLESS_IDLE | User-defined |
| **Zephyr** | Tak | Kconfig | Automatic |
| **ThreadX** | Tak | TX_TIMER... | User-defined |
| **Linux** | Tak | NO_HZ | cpuidle |
| **QNX** | Tak | Automatyczne | Power management |

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
System IoT ma następujące wymagania:
- Bateria: 1000mAh
- Średnie zużycie w active: 50mA
- Średnie zużycie w deep sleep: 100μA
- Tick co 1ms w tradycyjnym trybie
- Tickless: średnio 100ms w deep sleep

PYTANIA:
1. Oblicz czas pracy z tradycyjnym tick
2. Oblicz czas pracy z tickless idle (90% idle)
3. Jaki jest zysk?

ROZWIĄZANIE:
1. Tradycyjny tick:
   - Zawsze active = 50mA
   - Czas pracy = 1000mAh / 50mA = 20h

2. Tickless (90% deep sleep):
   - Active: 50mA × 10% = 5mA
   - Sleep: 100μA × 90% = 0.09mA
   - Średnie: 5.09mA
   - Czas pracy = 1000mAh / 5.09mA = 196h

3. Zysk: 196h / 20h = 9.8x więcej!
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego tickless idle jest ważne dla IoT?
1. Jakie są kompromisy między głębokością sleep a czasem wake-up?
1. Jak tickless idle wpływa na jitter?
1. Jak obliczyć minimalny czas idle dla tickless?

______________________________________________________________________

## Literatura

1. FreeRTOS, "Tickless Idle Mode" documentation
1. ARM, "AN482: Low Power Design"
1. Zephyr RTOS, Power Management documentation
1. Linux kernel, "NO_HZ: tickless kernel" documentation
