# Safety Monitor / Watchdog

## Definicja

Niezależny mechanizm kontroli systemu, który wykrywa awarie i inicjuje działania naprawcze. Watchdog to sprzętowy lub programowy licznik, który resetuje system jeśli nie zostanie "nakarmiony" w określonym czasie.

______________________________________________________________________

## Typy watchdogów

```
┌───────────────────────────────────────────────────────────────┐
│                    TYPY WATCHDOGÓW                            │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  1. SIMPLE WATCHDOG (Windowless)                             │
│     ┌─────────────────────────────────────────────────────┐  │
│     │  Counter ──▶ Timeout ──▶ Reset                      │  │
│     │                                                     │  │
│     │  Task: pet_watchdog() co < timeout                  │  │
│     │                                                     │  │
│     │  ZALETY: Prosty, przewidywalny                      │  │
│     │  WADY: Może być "karmiony" przez wiszący task       │  │
│     └─────────────────────────────────────────────────────┘  │
│                                                               │
│  2. WINDOWED WATCHDOG                                        │
│     ┌─────────────────────────────────────────────────────┐  │
│     │  Counter musi być resetowany w OKNIE:               │  │
│     │  ├── Za wcześnie → Reset                            │  │
│     │  ├── W oknie → OK                                   │  │
│     │  └── Za późno → Reset                               │  │
│     │                                                     │  │
│     │  ┌────────┼────────┼────────┐                       │  │
│     │  │  TOO   │ WINDOW │  TOO   │                       │  │
│     │  │ EARLY  │  OK    │  LATE  │                       │  │
│     │  └────────┴────────┴────────┘                       │  │
│     │                                                     │  │
│     │  ZALETY: Wykrywa pętle nieskończone                 │  │
│     │  WADY: Trudniejszy w użyciu                         │  │
│     └─────────────────────────────────────────────────────┘  │
│                                                               │
│  3. EXTERNAL WATCHDOG (Hardware)                             │
│     ┌─────────────────────────────────────────────────────┐  │
│     │  MCU ◀────▶ External Watchdog IC                    │  │
│     │               │                                     │  │
│     │               └──▶ Reset MCU (niezależny)           │  │
│     │                                                     │  │
│     │  ZALETY: Działa nawet jak MCU wisi                  │  │
│     │  WADY: Dodatkowy hardware                            │  │
│     └─────────────────────────────────────────────────────┘  │
│                                                               │
│  4. SAFETY MONITOR (Software/Hardware)                      │
│     ┌─────────────────────────────────────────────────────┐  │
│     │  ┌─────────┐    ┌─────────┐    ┌─────────┐         │  │
│     │  │ Task 1  │───▶│ Health  │───▶│ Action  │         │  │
│     │  │ Task 2  │───▶│ Monitor │───▶│ (Reset/ │         │  │
│     │  │ Task 3  │───▶│         │───▶│  Failsafe)│        │  │
│     │  └─────────┘    └─────────┘    └─────────┘         │  │
│     │                                                     │  │
│     │  ZALETY: Granularne monitorowanie                   │  │
│     │  WADY: Większa złożoność                            │  │
│     └─────────────────────────────────────────────────────┘  │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Automotive (ASIL-D)

| System | Watchdog | Timeout | Akcja |
|--------|----------|---------|-------|
| Engine Control | Windowed | 10 ms | Failsafe limp mode |
| Brake Control | External | 5 ms | Hydraulic backup |
| Steering | Dual watchdog | 2 ms | Torque overlay |

### Industrial (IEC 61508 SIL-3)

- **Safety PLC:** Watchdog dla każdej pętli
- **Emergency Stop:** Niezależny watchdog + przycisk
- **Motor Control:** Windowed watchdog + thermal protection

### Medical (IEC 62304)

- **Infusion Pump:** Czasowy watchdog na dawkowanie
- **Ventilator:** Dual watchdog + mechanical backup
- **Patient Monitor:** Heartbeat monitor per sensor

______________________________________________________________________

## Implementacja

### Simple Watchdog

```c
// Hardware watchdog
#define WDT_TIMEOUT_MS  1000

void wdt_init(void) {
    WDT->CTRL = WDT_CTRL_ENABLE;
    WDT->TIMEOUT = WDT_TIMEOUT_MS;
}

void wdt_feed(void) {
    WDT->FEED = 0x55AA;  // Magic sequence
}

// Task
void main_task(void) {
    while (1) {
        do_critical_work();
        wdt_feed();  // Musi być wywołane przed timeout
    }
}
```

### Windowed Watchdog

```c
#define WDT_WINDOW_MIN_MS   50   // Min time between feeds
#define WDT_WINDOW_MAX_MS   100  // Max time between feeds

static uint32_t last_feed_time = 0;

void wdt_feed_windowed(void) {
    uint32_t now = get_time_ms();
    uint32_t elapsed = now - last_feed_time;

    if (elapsed < WDT_WINDOW_MIN_MS) {
        // Too early - ERROR!
        system_error("Watchdog fed too early");
    } else if (elapsed > WDT_WINDOW_MAX_MS) {
        // Too late - will reset
        system_reset();
    }

    WDT->FEED = 0x55AA;
    last_feed_time = now;
}
```

### Safety Monitor Pattern

```c
typedef struct {
    uint32_t expected_period_ms;
    uint32_t max_jitter_ms;
    uint32_t last_checkin;
    bool     is_healthy;
} task_health_t;

static task_health_t tasks[MAX_TASKS];

void task_checkin(task_id_t id) {
    uint32_t now = get_time_ms();
    tasks[id].last_checkin = now;
    tasks[id].is_healthy = true;
}

void safety_monitor(void) {
    uint32_t now = get_time_ms();

    for (int i = 0; i < MAX_TASKS; i++) {
        uint32_t elapsed = now - tasks[i].last_checkin;

        if (elapsed > tasks[i].expected_period_ms + tasks[i].max_jitter_ms) {
            tasks[i].is_healthy = false;
            handle_task_failure(i);
        }
    }

    // Feed watchdog only if all tasks healthy
    if (all_tasks_healthy()) {
        wdt_feed();
    } else {
        // Watchdog will reset system
        log_error("Task failure - allowing watchdog reset");
    }
}
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Dead task feeding** | Martwy task nadal karmi watchdog | Per-task monitoring |
| **Too fast feeding** | Pętla nieskończona karmi watchdog | Windowed watchdog |
| **Watchdog disabled** | Debug mode z wyłączonym watchdog | Force enable w produkcji |
| **Race condition** | Wielowątkowe karmienie | Atomic operations |
| **Short timeout** | Zbyt agresywny reset | Proper WCET analysis |

______________________________________________________________________

## Kierunki rozwoju

### 1. Dual-Modular Redundancy

```
┌─────────────────────────────────────────────────────────────┐
│                    DUAL WATCHDOG                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐     ┌─────────────┐                       │
│  │  Watchdog A │     │  Watchdog B │                       │
│  └──────┬──────┘     └──────┬──────┘                       │
│         │                   │                               │
│         └─────────┬─────────┘                               │
│                   │                                         │
│             ┌─────▼─────┐                                   │
│             │   Voter   │                                   │
│             └─────┬─────┘                                   │
│                   │                                         │
│             ┌─────▼─────┐                                   │
│             │   Reset   │                                   │
│             └───────────┘                                   │
│                                                             │
│  Reset tylko gdy OBA watchdogi timeoutują                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 2. Predictive Watchdog

- AI/ML do przewidywania awarii
- Trend analysis tasków

### 3. External Safety MCU

- Osobny mikrokontroler safety
- Komunikacja heartbeat
- Niezależny reset głównego CPU

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj Safety Monitor dla systemu automotive:

ZADANIA:
├── Motor Control: okres 1 ms, max jitter 100 µs
├── Sensor Fusion: okres 5 ms, max jitter 500 µs
├── Communication: okres 10 ms, max jitter 1 ms
└── Diagnostics: okres 100 ms, max jitter 10 ms

PYTANIA:
1. Jaki timeout watchdog wybrać?
2. Jak wykryć martwy task?
3. Co zrobić gdy task zawiedzie?

ROZWIĄZANIE:
1. Timeout: max(motor_control) + margin = 1.5 ms

2. Per-task checkin:
   - Każdy task raportuje się do Safety Monitor
   - Safety Monitor sprawdza deadline
   - Tylko gdy wszystkie OK → feed watchdog

3. Akcje przy awarii:
   - Motor Control: Failsafe (hydraulic backup)
   - Sensor Fusion: Degraded mode
   - Communication: Log + continue
   - Diagnostics: Ignore
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego windowed watchdog jest bezpieczniejszy?
1. Jak zapewnić, że martwy task nie karmi watchdog?
1. Co to jest heartbeat monitoring?
1. Jakie są wymagania ASIL dla watchdog?

______________________________________________________________________

## Literatura

1. IEC 61508, "Functional Safety of Electrical/Electronic Systems"
1. ISO 26262-5, "Hardware Safety Requirements"
1. Texas Instruments, "Watchdog Timer Implementation Guide"
1. NXP, "Safety Manual for MPC574x MCUs"
