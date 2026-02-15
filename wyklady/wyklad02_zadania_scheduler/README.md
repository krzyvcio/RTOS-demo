# Wykład 2: Zadania i Scheduler

**Czas:** 90 minut (2 godziny akademickie)

---

## Plan wykładu

| Część | Temat | Czas |
|-------|-------|------|
| 1 | Model zadania | 15 min |
| 2 | Stany zadania | 15 min |
| 3 | Context switch | 15 min |
| 4 | Scheduling algorithms | 20 min |
| 5 | Rate Monotonic | 15 min |
| 6 | Analiza schedulability | 10 min |

---

## Slajd 1: Tytuł

```
Systemy Operacyjne Czasu Rzeczywistego

Wykład 2: Zadania i Scheduler

[Imię Nazwisko]
```

---

## Slajd 2: Model zadania (Task Model)

### Definicja zadania

> **Zadanie (task) to jednostka wykonawcza w RTOS - niezależny wątek z własnym kontekstem, priorytetem i timing requirements.**

### Parametry zadania

```
┌─────────────────────────────────────────────────────────┐
│                    TASK PARAMETERS                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  τ_i - zadanie i                                        │
│  r_i   - release time (czas aktywacji)                 │
│  C_i   - WCET (worst case execution time)              │
│  T_i   - period (okres, dla periodic tasks)            │
│  D_i   - relative deadline                              │
│  P_i   - priority                                       │
│                                                         │
│  Diagram:                                               │
│                                                         │
│       r_i              completion                       │
│         │                   │                           │
│         ▼                   ▼                           │
│  ├──────┼───────────────────┼───────►                   │
│         │◄────── C_i ──────►│                           │
│         │◄───────── R_i (response time) ──────────────►│
│         │◄───────────── D_i (deadline) ───────────────►│
│                                 │                       │
│                                 ▼                       │
│                              deadline                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Typy zadań

```
Periodic (periodyczne):
  - Aktywowane co T
  - Przewidywalne, łatwe do analizy
  - Przykład: sterowanie silnikiem co 10ms

  ├───┼───┼───┼───┼───┼───►
      T   T   T   T   T

Aperiodic (aperiodyczne):
  - Aktywowane przez zdarzenia zewnętrzne
  - Nieprzewidywalne
  - Przykład: obsługa klawiatury

  ├───┼───────┼────────────►
      event   event

Sporadic (sporadyczne):
  - Aperiodyczne z minimalnym interwałem
  - Ograniczone tempo
  - Przykład: obsługa błędów

  ├───┼───────┼───┼─────────►
      │       │   │
      min_interval
```

---

## Slajd 3: Task Control Block (TCB)

### Struktura danych zadania

```c
typedef struct tskTaskControlBlock {
    // Stan
    volatile StackType_t *pxTopOfStack;  // Wskaźnik na stos
    ListItem_t xStateListItem;            // Lista stanów

    // Priorytet
    UBaseType_t uxPriority;

    // Identyfikacja
    char pcTaskName[configMAX_TASK_NAME_LEN];

    // Stos
    StackType_t *pxStack;

    // Parametry
    void *pvParameters;

    // Statystyki
    #ifdef configUSE_TASK_STATISTICS
        uint32_t ulRunTimeCounter;
    #endif

} tskTCB;
```

### Przechowywanie kontekstu

```
Stack zawiera:
┌─────────────────────┐
│ xPSR               │
│ PC (return address)│
│ LR                 │
│ R12                │
│ R3                 │
│ R2                 │
│ R1                 │
│ R0                 │
├─────────────────────┤
│ R11                │
│ R10                │
│ R9                 │
│ R8                 │
│ R7                 │
│ R6                 │
│ R5                 │
│ R4                 │
└─────────────────────┘
```

---

## Slajd 4: Stany zadania

### Maszyna stanów

```
                    ┌─────────────┐
                    │   RUNNING   │
                    │  (1 task)   │
                    └──────┬──────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
    preempt│           block│           terminate│
         │                 │                 │
         ▼                 │                 ▼
  ┌─────────────┐          │         ┌─────────────┐
  │    READY    │◄─────────┘         │ TERMINATED  │
  │  (queue)    │                    │             │
  └──────┬──────┘                    └─────────────┘
         │                 ▲
         │                 │
    timeout/unblock       │
         │                 │
         │          event/timeout
         │                 │
         ▼                 │
  ┌─────────────┐          │
  │   BLOCKED   │──────────┘
  │  (waiting)  │
  └─────────────┘
```

### Opis stanów

| Stan | Opis | Powody przejścia |
|------|------|------------------|
| RUNNING | Wykonuje się na CPU | 1 na raz |
| READY | Gotowy, czeka na CPU | Czeka na scheduler |
| BLOCKED | Czeka na zdarzenie | Delay, mutex, semafor |
| TERMINATED | Zakończony | vTaskDelete() |

---

## Slajd 5: Context Switch

### Proces przełączania

```
┌─────────────────────────────────────────────────────────┐
│                  CONTEXT SWITCH                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. Tick interrupt lub API call                         │
│     ↓                                                   │
│  2. Zapisz kontekst running task                        │
│     - Push registers na stack                           │
│     - Zapisz SP do TCB                                   │
│     ↓                                                   │
│  3. Scheduler decision                                   │
│     - Znajdź highest priority ready task                │
│     ↓                                                   │
│  4. Przywróć kontekst new task                          │
│     - Wczytaj SP z TCB                                   │
│     - Pop registers ze stack                            │
│     ↓                                                   │
│  5. Return from interrupt                                │
│     - CPU kontynuuje new task                           │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### ARM Cortex-M Context Switch

```asm
; PendSV_Handler - context switch
PendSV_Handler:
    ; Save context
    MRS     R0, PSP         ; Get process stack pointer
    STMDB   R0!, {R4-R11}   ; Save R4-R11
    LDR     R1, =pxCurrentTCB
    LDR     R2, [R1]
    STR     R0, [R2]        ; Save SP to TCB

    ; Find next task
    BL      vTaskSwitchContext

    ; Restore context
    LDR     R1, =pxCurrentTCB
    LDR     R2, [R1]
    LDR     R0, [R2]        ; Get SP from TCB
    LDMIA   R0!, {R4-R11}   ; Restore R4-R11
    MSR     PSP, R0         ; Set process stack pointer
    BX      LR              ; Return
```

### Koszt context switch

```
Typowy overhead:
- Save/restore registers: ~10-20 cycles
- Scheduler decision: ~5-10 cycles
- Cache effects: zmienny

Total: ~20-50 cycles (bez cache effects)
Przy 100MHz: ~0.2-0.5μs

Ale z cache pollution: może być 10x więcej!
```

---

## Slajd 6: Scheduling Algorithms

### Klasyfikacja

```
┌─────────────────────────────────────────────────────────┐
│                 SCHEDULING ALGORITHMS                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Preemptive vs Non-preemptive:                          │
│  - Preemptive: scheduler może przerwać task            │
│  - Non-preemptive: task sam oddaje CPU                  │
│                                                         │
│  Static vs Dynamic priority:                            │
│  - Static: priorytety stałe                             │
│  - Dynamic: priorytety zmieniają się                    │
│                                                         │
│  Fixed-priority algorithms:                             │
│  - Rate Monotonic (RMS)                                 │
│  - Deadline Monotonic (DM)                              │
│                                                         │
│  Dynamic-priority algorithms:                           │
│  - Earliest Deadline First (EDF)                        │
│  - Least Laxity First (LLF)                             │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Rysowanie timeline

```
Preemptive priority-based:

Task H (prio 3): ████░░░░░░░░░░░░░░████░░░░░░░░
Task M (prio 2): ░░░░████████░░░░░░░░░░░░████████
Task L (prio 1): ░░░░░░░░░░░░████████░░░░░░░░░░░░

Timeline:
H running → H blocks → M runs → H ready → M preempted
H runs → H blocks → M continues → M blocks → L runs
```

---

## Slajd 7: Rate Monotonic Scheduling (RMS)

### Zasada

> **W RMS, task o krótszym okresie ma wyższy priorytet.**
> `Priority(i) ∝ 1 / Period(i)`

### Przykład

```
Task A: T = 4ms → Priority = HIGH
Task B: T = 8ms → Priority = MEDIUM
Task C: T = 16ms → Priority = LOW

Priorytety statyczne:
A > B > C (zawsze)
```

### Utilization Bound

```
Dla systemu z N zadań periodycznych:

Sufficient condition:
U ≤ N(2^(1/N) - 1)

Gdzie:
U = Σ(Ci/Ti) - całkowita utilisacja

Wartości:
N=1:  U ≤ 100.0%
N=2:  U ≤ 82.8%
N=3:  U ≤ 78.0%
N=∞:  U ≤ 69.3% (ln 2)

Interpretacja:
- RMS jest sub-optymalny (nie wykorzystuje 100%)
- Ale jest prosty i przewidywalny
```

---

## Slajd 8: Schedulability Analysis

### Response Time Analysis (RTA)

```
Dla zadania i, WCRT jest dane przez:

Ri = Ci + Σ⌈Ri/Tj⌉ × Cj
        j∈hp(i)

Gdzie:
- hp(i) = zbiór zadań o wyższym priorytecie
- Ri rozwiązywane iteracyjnie

Iteracja:
Ri^(0) = Ci
Ri^(n+1) = Ci + Σ⌈Ri^(n)/Tj⌉ × Cj

Zbieżność gdy Ri^(n+1) = Ri^(n) ≤ Di
```

### Przykład obliczenia

```
Zadania:
τ1: C1=1, T1=4, D1=4 (highest priority)
τ2: C2=2, T2=6, D2=6
τ3: C3=3, T3=8, D3=8 (lowest priority)

R1 = C1 = 1 ≤ D1=4 ✓

R2:
R2^0 = C2 = 2
R2^1 = 2 + ⌈2/4⌉×C1 = 2 + 1×1 = 3
R2^2 = 2 + ⌈3/4⌉×1 = 2 + 1 = 3 ← Zbieżność!
R2 = 3 ≤ D2=6 ✓

R3:
R3^0 = C3 = 3
R3^1 = 3 + ⌈3/4⌉×1 + ⌈3/6⌉×2 = 3 + 1 + 2 = 6
R3^2 = 3 + ⌈6/4⌉×1 + ⌈6/6⌉×2 = 3 + 2 + 2 = 7
R3^3 = 3 + ⌈7/4⌉×1 + ⌈7/6⌉×2 = 3 + 2 + 2 = 7 ← Zbieżność!
R3 = 7 ≤ D3=8 ✓

System SCHEDULABLE!
```

---

## Slajd 9: Earliest Deadline First (EDF)

### Zasada

> **W EDF, task z najwcześniejszym deadline ma najwyższy priorytet.**
> Priorytety są dynamiczne, zmieniają się w runtime.

### Zalety i wady

```
Zalety EDF:
+ Optymalny - wykorzystuje 100% CPU
+ Obsługuje dowolne deadline
+ Mniej konserwatywny niż RMS

Wady EDF:
- Priorytety dynamiczne = więcej overhead
- Trudniejsza implementacja
- Nieprzewidywalna degradacja przy przeciążeniu
- Mniejsze wsparcie w komercyjnych RTOS
```

### RMS vs EDF - Przykład

```
System: U = 85%

RMS: 85% > 78% (RMS bound dla N=3)
→ Może nie być schedulable
→ Potrzebna RTA do sprawdzenia

EDF: 85% < 100%
→ Gwarantowane schedulable!
```

---

## Slajd 10: Priorytety w FreeRTOS

### API

```c
// Tworzenie zadania z priorytetem
xTaskCreate(
    vTaskFunction,    // Funkcja
    "TaskName",       // Nazwa
    128,              // Stack size
    NULL,             // Parameters
    2,                // Priority ←
    NULL              // Handle
);

// Zmiana priorytetu w runtime
vTaskPrioritySet(taskHandle, 3);

// Pobranie priorytetu
UBaseType_t prio = uxTaskPriorityGet(taskHandle);
```

### Hierarchia

```
FreeRTOS priority levels:

#define configMAX_PRIORITIES 5

Level 4: [Highest priority]
Level 3:
Level 2:
Level 1:
Level 0: [Lowest - IDLE task]

Większy numer = wyższy priorytet
```

---

## Slajd 11: Priorytety a scheduling

### Time slicing

```
Dla zadań tego samego priorytetu:

┌─────────────────────────────────────────────────────────┐
│              ROUND ROBIN TIME SLICING                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A (prio 2): ████░░░░░░░░████░░░░░░░░░████         │
│  Task B (prio 2): ░░░░████░░░░░░░░████░░░░░░░░░░        │
│                    │    │    │    │    │                │
│                    └────┴────┴────┴────┘               │
│                       Time slices (tick)                │
│                                                         │
│  configUSE_TIME_SLICING = 1 (default)                  │
│  Każdy task dostaje równy time slice                    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Wywłaszczanie

```c
// FreeRTOS domyślnie preemptive
#define configUSE_PREEMPTION 1

// Gdy wyższy priorytet become ready:
// → Preemptuje running task
// → Natychmiastowe przełączenie
```

---

## Slajd 12: Podsumowanie

### Kluczowe pojęcia

```
1. Task Model
   - Parametry: r, C, T, D, P
   - Typy: periodic, aperiodic, sporadic

2. Task States
   - Running, Ready, Blocked
   - Maszyna stanów

3. Context Switch
   - Zapis/przywróć kontekst
   - Koszt overhead

4. Scheduling
   - RMS: statyczne priorytety
   - EDF: dynamiczne priorytety
   - Schedulability analysis

5. Utilization
   - RMS bound: ~69%
   - EDF bound: 100%
```

---

## Zadania

### Teoretyczne

1. Oblicz schedulability dla systemu:
   - T1: C=1, T=5
   - T2: C=2, T=10
   - T3: C=2, T=20

2. Czy system jest schedulable pod RMS?

3. Porównaj z EDF.

### Praktyczne

1. Na laboratorium: implementacja zadań z priorytetami
2. Obserwacja context switch
3. Pomiar overhead

---

## Literatura

```
1. "Real-Time Systems" - Jane W. S. Liu
   Rozdział 3: Task Scheduling

2. "Hard Real-Time Computing Systems"
   - Giorgio Buttazzo

3. FreeRTOS Documentation
   - Task Management
```

---

## Pytania kontrolne

```
1. Jakie są główne parametry zadania?
2. Opisz maszynę stanów zadania.
3. Co to jest context switch i ile kosztuje?
4. Wyjaśnij zasadę Rate Monotonic Scheduling.
5. Dlaczego RMS ma utilization bound ~69%?
6. Czym różni się EDF od RMS?
```