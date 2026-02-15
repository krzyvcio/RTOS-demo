# Preempcja (Wywłaszczanie)

## Definicja

**Preempcja** to przymusowe zatrzymanie aktualnie wykonywanego tasku i przekazanie kontroli innemu tasku. To "agresywne" przejęcie CPU przez task o wyższym priorytecie lub przez scheduler po upływie time slice.

> Preempcja to "radykalne rozwiązanie" - task nie ma nic do gadania. Ktoś ważniejszy chce CPU i go dostaje. Natychmiast.

```
┌─────────────────────────────────────────────────────────┐
│                   PREEMPTION                            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task L (low priority) running:                        │
│  ████████████████████████████████████████              │
│                                          │              │
│  Task H (high priority) becomes ready:  │              │
│                                          ▼              │
│  Task L preempted:                       │              │
│  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░│              │
│                   │                      │              │
│                   └── Context switch ────┘              │
│                                                         │
│  Task H running:                                        │
│                   ████████████████████████████          │
│                                                         │
│  Task L resumed (when H blocks):                       │
│                                                       ░░░░░░░░░░░░░░░░░░
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🦁 Drapieżnik atakuje stado

Zebra pasie się spokojnie (task L running). Nagle lew atakuje (task H becomes ready):

```
Normalne: Zebra pasie się (task L)
Sygnał: Lew atakuje (interrupt/preemption)
Reakcja: Zebra ucieka natychmiast (preempted)
Po zagrożeniu: Zebra wraca do pasienia (resume)

Preempcja to instynkt przetrwania:
- Nie czekasz
- Nie kończysz
- Uciekasz NATYCHMIAST
```

### 🚨 Alarm pożarowy

Pracujesz w biurze (task L). Włącza się alarm (task H):

```
Normalne: Praca w biurze (task L running)
Sygnał: Alarm (preemption signal)
Reakcja: Ewakuacja natychmiastowa (preempted)
Po pożarze: Powrót do pracy (resume)

Alarm preempts wszystko inne.
Nie kończysz maila. Nie kończysz rozmowy.
Idziesz NATYCHMIAST.
```

### 🏥 Szpital - nagły wypadek

Lekarz przeprowadza rutynowe badanie (task L). Przyjeżdża karetka z krytycznym pacjentem (task H):

```
Normalne: Rutynowe badanie (task L running)
Sygnał: Krytyczny pacjent (preemption)
Reakcja: Przerwanie badania, bieg do sali (preempted)
Po operacji: Dokończenie badania (resume)

Krytyczny pacjent preemptuje wszystko.
```

---

## Podobieństwo do systemów informatycznych

### CPU Preemption (Linux)

```bash
# Linux preemptive scheduling
cat /proc/sys/kernel/sched_min_granularity_ns
# 10,000,000 (10ms minimum time slice)

# Każdy proces może być preemptowany po time slice
# Lub gdy proces z wyższym priorytetem becomes ready
```

### JavaScript Event Loop (Cooperative)

```javascript
// JavaScript NIE ma preemption!
// Cooperative multitasking

function longRunningTask() {
    // Ten kod blokuje wszystko
    // Inne eventy czekają
    while (true) {
        // Nieskończona pętla = zawieszenie
    }
}

// Brak preemption = jeden kod może zablokować całą aplikację
```

### Thread Interruption

```java
// Java thread interruption
Thread thread = new Thread(() -> {
    while (!Thread.interrupted()) {
        doWork();
    }
});

thread.start();
// Później...
thread.interrupt();  // "Preemption signal"
```

---

## Preemptive vs Cooperative

### Preemptive Scheduling

```
┌─────────────────────────────────────────────────────────┐
│                   PREEMPTIVE                            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A running:                                        │
│  ████████████████████████████████████████████          │
│                                                  │      │
│  Timer interrupt:                               ▼      │
│  ███████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░          │
│                      │                                  │
│  Task B running:     └──────────────────────────────►  │
│                      ████████████████████████████       │
│                                                         │
│  Scheduler MOŻE przerwać task w dowolnym momencie.     │
│  Task nie ma kontroli nad tym kiedy.                   │
│                                                         │
│  Zalety:                                                │
│  ✓ Responsywność                                       │
│  ✓ Sprawiedliwość (fairness)                          │
│  ✓ High priority taski otrzymują CPU szybko           │
│                                                         │
│  Wady:                                                  │
│  ✗ Context switch overhead                             │
│  ✗ Trudniejsza synchronizacja                          │
│  ✗ Race conditions                                     │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Cooperative Scheduling

```
┌─────────────────────────────────────────────────────────┐
│                   COOPERATIVE                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A running:                                        │
│  ████████████████████████████████████████████████████  │
│                                                          │
│  Task A yields:                                    │    │
│  ████████████████████████████████████████████████░░░░  │
│                                                    │    │
│  Task B running:                                   ▼    │
│                                                    ████ │
│                                                         │
│  Task SAM decyduje kiedy oddać CPU.                    │
│  Musi współpracować (cooperate).                       │
│                                                         │
│  Zalety:                                                │
│  ✓ Prosta synchronizacja                               │
│  ✓ Mniejszy overhead                                   │
│  ✓ Deterministyczne punkty switch                      │
│                                                         │
│  Wady:                                                  │
│  ✗ Jeden task może zablokować wszystko                │
│  ✗ Gorsza responsywność                               │
│  ✗ Niebezpieczne dla RTOS                              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Punkty preempcji

### Gdzie preempcja może wystąpić?

```
1. TICK INTERRUPT
   - Cykliczny timer
   - Scheduler sprawdza czy switch jest potrzebny
   - Time slice expired?

2. HIGHER PRIORITY TASK READY
   - Task o wyższym priorytecie staje się ready
   - Natychmiastowa preempcja (w preemptive scheduling)

3. CURRENT TASK BLOCKS
   - Task czeka na mutex/semafor/I/O
   - Preempted, inny task running

4. CURRENT TASK YIELDS
   - Task dobrowolnie oddaje CPU
   - Cooperative preempcja

5. CURRENT TASK TERMINATES
   - Task się kończy
   - Inny task running

6. ISR COMPLETES
   - Przerwanie zakończone
   - Może unblock task o wyższym priorytecie
   - Preempcja przy return from interrupt
```

---

## Implementacja preempcji

### Tick Interrupt

```c
// Tick interrupt handler
void SysTick_Handler(void) {
    // Inkrementuj tick count
    xTickCount++;

    // Sprawdź time slice
    if (--current_task->time_slice == 0) {
        // Time slice expired - request context switch
        xPendSV = 1;
    }

    // Sprawdź czy taski stały się ready
    check_delayed_tasks();

    // Context switch (jeśli potrzebny)
    if (xPendSV) {
        SCB->ICSR |= SCB_ICSR_PENDSVSET_Msk;
    }
}
```

### Context Switch

```c
// ARM Cortex-M PendSV Handler
// To jest właściwy context switch
__asm void PendSV_Handler(void) {
    // Save current context
    MRS     R0, PSP
    STMDB   R0!, {R4-R11}
    LDR     R1, =current_task
    LDR     R2, [R1]
    STR     R0, [R2]

    // Load next task
    LDR     R0, =next_task
    LDR     R1, [R0]
    LDR     R0, [R1]
    LDMIA   R0!, {R4-R11}
    MSR     PSP, R0

    // Return to next task
    BX      LR
}
```

---

## Preempcja a sekcje krytyczne

### Problem

```c
// Problem: preempcja w sekcji krytycznej
int shared_counter = 0;

void task_a(void) {
    shared_counter++;  // Nieatomicowe!
    // Jeśli preempcja tutaj → race condition
}

void task_b(void) {
    shared_counter++;  // Nieatomicowe!
}
```

### Rozwiązanie: Wyłączenie preempcji

```c
void task_a(void) {
    taskENTER_CRITICAL();
    shared_counter++;  // Bezpieczne - brak preempcji
    taskEXIT_CRITICAL();
}

void task_b(void) {
    taskENTER_CRITICAL();
    shared_counter++;
    taskEXIT_CRITICAL();
}
```

### Wyłączenie preempcji w FreeRTOS

```c
// Wyłącz preempcję
vTaskSuspendAll();

// Kod bezpieczny od preempcji
shared_counter++;
shared_data = 42;

// Włącz preempcję
xTaskResumeAll();
```

### Wyłączenie przerwań

```c
// Wyłącz wszystkie przerwania
portENTER_CRITICAL();
// Kod bezpieczny od wszystkiego
portEXIT_CRITICAL();

// ARM Cortex-M:
__disable_irq();
// Kod
__enable_irq();
```

---

## Preempcja a RTOS

### Gdy preempcja jest konieczna

```
RTOS wymaga preemption dla:

1. RESPONSIVENESS
   - High-priority task musi otrzymać CPU szybko
   - Event response time

2. DETERMINISM
   - Gwarantowany response time
   - WCRT analysis wymaga preemption

3. REAL-TIME GUARANTEES
   - Deadline muszą być dotrzymane
   - Low priority task nie może blokować high priority

4. FAIRNESS
   - Każdy task dostaje czas CPU
   - Żaden task nie monopolizuje
```

### Gdy preempcja jest problemem

```
Preempcja powoduje problemy:

1. RACE CONDITIONS
   - Shared data corruption
   - Nieatomowe operacje

2. PRIORITY INVERSION
   - Low priority blokuje high priority
   - Przez mutex/semafor

3. OVERHEAD
   - Context switch kosztuje czas
   - Cache pollution

4. COMPLEXITY
   - Trudniejsze do debugowania
   - Trudniejsze do analizy
```

---

## Preempcja w praktyce

### Minimalizacja preempcji

```c
// ŹLE: Długa sekcja bez yield
void task_long(void) {
    for (int i = 0; i < 1000000; i++) {
        process(i);  // Inne taski czekają!
    }
}

// DOBRZE: Okresowe yield
void task_nice(void) {
    for (int i = 0; i < 1000000; i++) {
        process(i);
        if (i % 1000 == 0) {
            taskYIELD();  // Pozwól innym taskom
        }
    }
}

// NAJLEPIEJ: Krótkie taski
void task_short(void) {
    process_one_batch();  // Krótkie wykonanie
    vTaskDelay(10);       // Czekaj do następnego okresu
}
```

### Preemption-safe data structures

```c
// Lock-free queue dla ISR/task communication
typedef struct {
    volatile uint32_t head;
    volatile uint32_t tail;
    uint8_t buffer[SIZE];
} LockFreeQueue;

// Safe dla preemption
bool queue_push(LockFreeQueue* q, uint8_t data) {
    uint32_t next = (q->head + 1) % SIZE;
    if (next == q->tail) return false;  // Full

    q->buffer[q->head] = data;
    __sync_synchronize();  // Memory barrier
    q->head = next;
    return true;
}

uint8_t queue_pop(LockFreeQueue* q, uint8_t* data) {
    if (q->head == q->tail) return false;  // Empty

    *data = q->buffer[q->tail];
    __sync_synchronize();  // Memory barrier
    q->tail = (q->tail + 1) % SIZE;
    return true;
}
```

---

## Mierzenie preempcji

```c
// Licznik preempcji
volatile uint32_t preempt_count = 0;

// Hook dla context switch
void vApplicationSwitchOutHook(TaskHandle_t task) {
    preempt_count++;
}

// Statystyki
void print_preempt_stats(void) {
    printf("Total preemptions: %lu\n", preempt_count);
    printf("Preemptions per second: %lu\n",
           preempt_count / get_elapsed_seconds());
}
```

---

## Preempcja a priorytety

```
┌─────────────────────────────────────────────────────────┐
│           PREEMPTION AND PRIORITIES                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task H (high priority):                                │
│  ████░░░░░░░░░░░░░░████████████████░░░░░░░░░░░░░░░░░░  │
│       │              │                   │              │
│       │              │                   │              │
│  Task M (medium priority):                             │
│  ░░░░████████████░░░░░░░░░░░░░░░░░░████████░░░░░░░░░░  │
│            │                               │            │
│            │ H preempts M                  │ H blocks   │
│            ▼                               ▼            │
│  Task L (low priority):                                │
│  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████████░░  │
│                                                         │
│  Timeline:                                              │
│  1. H starts                                            │
│  2. H blocks → M starts                                 │
│  3. H becomes ready → preempts M                        │
│  4. H blocks → M continues                              │
│  5. M blocks → L starts                                 │
│  6. L runs until done                                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Pytania do przemyślenia

1. Czy Twój RTOS używa preemptive czy cooperative scheduling?
2. Jak często następuje preempcja w Twoim systemie?
3. Jak zabezpieczasz sekcje krytyczne przed preempcją?

---

## Quiz

**Pytanie**: Masz system z trzema taskami:

```
Task H: priority 3, running
Task M: priority 2, blocked on semaphore
Task L: priority 1, ready
```

Co się stanie gdy H wykona `vTaskDelay(100)`?

**Odpowiedź**:

```
1. H wywołuje vTaskDelay(100)
2. H przechodzi do blocked state
3. Scheduler szuka highest priority ready task
4. M jest blocked (na semaforze)
5. L jest ready
6. L przejmuje CPU (preempted H, teraz running)

Kolejność:
- H running → H blocked (delay)
- M blocked (semaphore) - nie może run
- L ready → L running

L będzie running dopóki:
- M becomes ready (semaphore released)
- H delay expires (po 100 ticks)
- L blocks/yields
```

---

## Wskazówka zapamiętywania

> **Preempcja = Wyrzucenie z restauracji**
>
> Siedzisz w restauracji (task L running).
> Jesz spokojnie obiad.
>
> Nagle przychodzi VIP (task H).
> Kelner mówi: "Musisz zwolnić stolik TERAZ."
> Nie możesz dokończyć obiadu.
> Musisz wstać NATYCHMIAST.
>
> To jest preempcja.
>
> W cooperative: mógłbyś dokończyć obiad.
> W preemptive: wstajesz w połowie kęsa.
>
> VIP dostaje stolik. Ty czekasz.
> Gdy VIP skończy - możesz wrócić.