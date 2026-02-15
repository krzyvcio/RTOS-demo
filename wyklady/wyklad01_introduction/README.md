# Wykład 1: Wprowadzenie do Systemów Czasu Rzeczywistego

**Czas:** 90 minut (2 godziny akademickie)
**Prowadzący:** [imię nazwisko]

---

## Plan wykładu

| Część | Temat | Czas |
|-------|-------|------|
| 1 | Definicja i motywacja | 15 min |
| 2 | Klasyfikacja systemów RT | 15 min |
| 3 | Wymagania czasowe | 20 min |
| 4 | RTOS vs GPOS | 15 min |
| 5 | Architektura RTOS | 15 min |
| 6 | Przykłady i podsumowanie | 10 min |

---

## Slajd 1: Tytuł

```
Systemy Operacyjne Czasu Rzeczywistego

Wykład 1: Wprowadzenie

[Imię Nazwisko]
[Politechnika/Uniwersytet]
[Semestr/Rok]
```

---

## Slajd 2: Dlaczego nas to obchodzi?

### Gdzie są systemy RT?

```
✈️ Lotnictwo
   - Fly-by-wire (Airbus, Boeing)
   - Nawigacja, sterowanie silnikami

🚗 Automotive
   - ABS, ESP, wtrysk paliwa
   - Autonomiczne pojazdy

🚀 Kosmonautyka
   - Satelity, stacje kosmiczne
   - Łaziki marsjańskie

🏭 Przemysł
   - Sterowniki PLC
   - Robotyka

🏥 Medycyna
   - Rozruszniki serca
   - Respiratory

📱 Embedded
   - Smartfony (audio, radio)
   - IoT, wearables
```

---

## Slajd 3: Definicja systemu RT

### Czym jest "Real-Time"?

> **System czasu rzeczywistego to system, w którym poprawność działania zależy nie tylko od wyniku logicznego, ale również od czasu, w jakim ten wynik jest wyprodukowany.**

```
Wynik poprawny + Na czas = SUKCES
Wynik poprawny + Za późno = PORAŻKA
Wynik błędny + Na czas = PORAŻKA
```

### Przykład: Airbag

```
Zderzenie wykryte → T0
Airbag napełniony → T0 + 30ms (deadline)

Jeśli airbag napełni się za 50ms:
- Kierowca już uderzył w kierownicę
- Airbag nie chroni = PORAŻKA

Jeśli airbag napełni się za 10ms:
- Kierowca jeszcze w bezpiecznej pozycji
- Airbag może zranić = PORAŻKA
```

---

## Slajd 4: Klasyfikacja systemów RT

### Hard Real-Time

```
Przekroczenie deadline = KATASTROFA

Przykłady:
- Airbag (śmierć kierowcy)
- Sterownik silnika lotniczego ( katastrofa)
- Rozrusznik serca (śmierć pacjenta)
- Hamulce pociągu (wypadek)

Wymagania:
- Gwarantowane czasy odpowiedzi
- Formalna weryfikacja
- Certyfikacja (DO-178C, ISO 26262)
```

### Firm Real-Time

```
Przekroczenie deadline = DEGRADACJA JAKOŚCI

Przykłady:
- Streaming video (klatka gubiona)
- Audio processing (trzęsienie)
- Gry online (lag)

Wymagania:
- Statistical guarantees
- Best effort
- Graceful degradation
```

### Soft Real-Time

```
Przekroczenie deadline = NIEWYGODA

Przykłady:
- Interfejs użytkownika
- Print spooler
- Email client

Wymagania:
- Average performance
- No hard guarantees
```

---

## Slajd 5: Timing Requirements

### Kluczowe pojęcia

```
┌─────────────────────────────────────────────────────────┐
│                 TIMING TERMINOLOGY                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Release time (r) - moment gdy zadanie staje się gotowe│
│  Deadline (d)     - moment do którego zadanie musi     │
│                     zostać ukończone                    │
│  Execution time (e) - czas potrzebny na wykonanie      │
│  Response time (R) - czas od release do completion     │
│                                                         │
│  r ──►[======= e =======]──► completion                │
│  │                          │                          │
│  └──────────── R ───────────┘                          │
│  │                          │                          │
│  └──────────── d ───────────┘ (deadline)              │
│                                                         │
│  R ≤ d → Deadline dotrzymane ✓                         │
│  R > d → Deadline miss ✗                               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Deadline vs Period

```
Periodic task:
  ┌────┐     ┌────┐     ┌────┐
  │Task│     │Task│     │Task│
  └────┘     └────┘     └────┘
  ├────T────┼────T────┼────T────►
  r    d    r    d    r    d

Deadline = Period (typowo)

Aperiodic task:
                ┌────┐
                │Task│
                └────┘
  ├─────────────┼────d────►
                r

Deadline ≠ Period
```

---

## Slajd 6: WCET - Worst Case Execution Time

### Dlaczego WCET jest ważne?

```
Average case ≠ Good enough w RT!

Przykład:
- Średni czas wykonania: 5ms
- Najgorszy przypadek: 50ms
- Deadline: 10ms

Średnia wygląda OK (5ms < 10ms)
Ale WCET = 50ms > 10ms = DEADLINE MISS!

W RTOS: tylko WCET się liczy
```

### Składniki WCET

```
WCET = longest_path_time + cache_effects + interrupt_interference

Factors:
- Control flow (loops, branches)
- Cache hit/miss
- Pipeline stalls
- Memory access latency
- Preemption by higher priority
```

---

## Slajd 7: Latencja i Jitter

### Latencja (Opóźnienie)

```
Latencja = czas od zdarzenia do rozpoczęcia obsługi

Typy:
- Interrupt latency: sygnał → ISR start
- Scheduling latency: task ready → task running
- End-to-end latency: event → result

Przykład:
Przycisk wciśnięty → LED zapalony
Latencja = 100μs (OK dla interfejsu)
Latencja = 100ms (zły dla sterowania)
```

### Jitter (Wahania)

```
Jitter = zmienność latencji/okresu

Idealnie (brak jitter):
  │  │  │  │  │  │
  └──┴──┴──┴──┴──┴──►
      każdy cykl identyczny

Z jitter:
  │ │   ││    │ │
  └─┴───┴┴────┴─┴──►
     wahania

Wpływ jitter:
- Destabilizacja pętli sterowania
- Błędy w komunikacji
- Nieprzewidywalność
```

---

## Slajd 8: RTOS vs GPOS

### Porównanie

| Cecha | RTOS | GPOS (Linux, Windows) |
|-------|------|------------------------|
| Determinizm | Gwarantowany | Niegwarantowany |
| Scheduling | Priority preemptive | Complex policies |
| Latencja | Znana, gwarantowana | Zmienna |
| Overhead | Minimalny | Może być duży |
| Memory | Static, pre-allocated | Dynamic allocation |
| Priorytety | Fixed, real | Dynamic, nice values |
| API | Proste, deterministyczne | Rich, ale nieRT |

### GPOS z patchami RT

```
Linux + PREEMPT_RT patch:
- Zmienia Linux w soft-RT system
- Threaded interrupts
- Priority inheritance mutexes
- Latencja: ~10-100μs (vs ms bez patcha)

Ale:
- Nadal nie jest hard-RT
- Brak formalnych gwarancji
- Brak certyfikacji
```

---

## Slajd 9: Architektura RTOS

### Podstawowe komponenty

```
┌─────────────────────────────────────────────────────────┐
│                    RTOS ARCHITECTURE                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │                  APPLICATION                    │   │
│  │            (Tasks / Threads)                    │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                               │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   KERNEL                       │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐       │   │
│  │  │Scheduler │ │Sync Prims│ │  Timers  │       │   │
│  │  └──────────┘ └──────────┘ └──────────┘       │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐       │   │
│  │  │  Memory  │ │   IPC    │ │   I/O    │       │   │
│  │  └──────────┘ └──────────┘ └──────────┘       │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                               │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   HAL / BSP                     │   │
│  │           (Hardware Abstraction)                │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                               │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   HARDWARE                     │   │
│  │          (CPU, Memory, Peripherals)             │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Mikrokernel vs Monolithic

```
Monolithic (Linux tradycyjny):
- Wszystko w kernel space
- Szybkie syscalls
- Ale: błąd w driverze = crash systemu

Microkernel (seL4, QNX):
- Minimalny kernel
- Drivers w user space
- Izolacja błędów
- Ale: więcej context switches
```

---

## Slajd 10: FreeRTOS - Architektura

### Struktura

```
FreeRTOS:
├── tasks.c       # Task management
├── queue.c       # Queues, semaphores
├── timers.c      # Software timers
├── list.c        # Linked lists (internal)
├── portable/     # Port-specific code
│   └── GCC/ARM/
│       ├── port.c      # Context switch
│       └── portmacro.h # Port macros
└── FreeRTOSConfig.h    # Configuration
```

### Konfiguracja

```c
// FreeRTOSConfig.h
#define configUSE_PREEMPTION            1
#define configTICK_RATE_HZ             1000
#define configMAX_PRIORITIES            5
#define configMINIMAL_STACK_SIZE      128
#define configUSE_MUTEXES               1
#define configUSE_COUNTING_SEMAPHORES   1
```

---

## Slajd 11: Hello RTOS

### Pierwszy program

```c
#include "FreeRTOS.h"
#include "task.h"
#include <stdio.h>

void vTask1(void *pvParameters) {
    while (1) {
        printf("Hello from RTOS!\n");
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}

int main(void) {
    xTaskCreate(vTask1, "Task1", 128, NULL, 1, NULL);
    vTaskStartScheduler();

    // Never reaches here
    for (;;);
    return 0;
}
```

### Co się dzieje?

```
1. main() tworzy zadanie
2. vTaskStartScheduler() przejmuje kontrolę
3. Scheduler uruchamia Task1
4. Task1 wypisuje, potem vTaskDelay
5. Task1 BLOCKED na 1000 ticków
6. Scheduler przełącza na Idle Task
7. Po 1000 ticków Task1 READY
8. Scheduler uruchamia Task1
9. ... (powtórka)
```

---

## Slajd 12: Podsumowanie

### Kluczowe punkty

```
1. RT ≠ Fast
   - RT oznacza przewidywalność, nie szybkość
   - Late = Wrong w RT

2. Deadline są bezwzględne
   - Hard RT: miss = katastrofa
   - Soft RT: miss = degradacja

3. WCET > Average
   - Tylko najgorszy przypadek się liczy
   - Analiza musi być pesymistyczna

4. Determinizm jest królem
   - Latencja znana
   - Jitter minimalny
   - Behavior przewidywalny

5. RTOS ≠ GPOS
   - Inne cele, inne narzędzia
   - Prostota > Funkcjonalność
```

---

## Slajd 13: Literatura i zadania

### Literatura obowiązkowa

```
1. "Mastering the FreeRTOS Real Time Kernel"
   - Richard Barry
   - Dostępne online (freertos.org)

2. "Real-Time Systems"
   - Jane W. S. Liu
   - Prentice Hall

3. FreeRTOS Documentation
   - https://www.freertos.org/Documentation/
```

### Zadania na laboratorium

```
1. Skonfigurować środowisko FreeRTOS
2. Uruchomić pierwszą aplikację
3. Eksperymenty z priorytetami
4. Obserwacja przełączania zadań
```

---

## Slajd 14: Pytania

```
1. Czym różni się hard RT od soft RT?
2. Dlaczego średni czas wykonania jest bezużyteczny w RT?
3. Co to jest determinizm i dlaczego jest ważny?
4. Jakie są typowe aplikacje systemów RT?
5. Dlaczego Linux (bez patcha) nie jest RTOS?
```

---

## Materiały dodatkowe

### Video

- "What is Real-Time?" - embeddedrelated.com
- FreeRTOS Tutorial Series - YouTube

### Narzędzia

- FreeRTOS Windows Simulator
- STM32CubeIDE + FreeRTOS
- QEMU ARM emulation

---

## Notatki dla prowadzącego

### Punkty do podkreślenia

1. **RT ≠ Fast** - to najważniejsze nieporozumienie
2. **Late = Wrong** - nawet poprawny wynik za późno jest błędem
3. **Determinizm** - to odróżnia RTOS od GPOS

### Typowe pytania studentów

- "Czy Windows jest RTOS?" - Nie, brak determinizmu
- "Czy Python może być RT?" - Teoretycznie tak, ale GC = problem
- "Dlaczego nie zawsze używać RTOS?" - Koszt, złożoność, overhead

### Demo na żywo

- FreeRTOS simulator pokazujący task switching
- Porównanie latencji Linux vs RTOS