# Scheduler (Planista)

## Definicja

**Scheduler** to serce systemu operacyjnego - komponent decydujący, który task ma być wykonywany w danym momencie. Scheduler to "dyrektor orkiestry" CPU - dyryguje taskami i przydziela im czas procesora.

> Scheduler to arbiter zasobów: decyduje kto, kiedy i jak długo używa CPU. W RTOS ta decyzja musi być deterministyczna i przewidywalna.

```
┌─────────────────────────────────────────────────────────┐
│                    SCHEDULER                            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Ready Queue:                                           │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                       │
│  │ T1  │ │ T2  │ │ T3  │ │ T4  │                       │
│  │prio3│ │prio2│ │prio3│ │prio1│                       │
│  └─────┘ └─────┘ └─────┘ └─────┘                       │
│     │                                                   │
│     ▼                                                   │
│  ┌─────────────────────────────────────────┐           │
│  │              SCHEDULER                   │           │
│  │                                         │           │
│  │  1. Który task ma najwyższy priorytet?  │           │
│  │  2. Czy obecny task powinien zostać?    │           │
│  │  3. Czy powinno być context switch?     │           │
│  │                                         │           │
│  └─────────────────────────────────────────┘           │
│     │                                                   │
│     ▼                                                   │
│  Running: T1 (highest priority ready)                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🏥 Szpitalna kolejka

Scheduler w RTOS to jak triage w szpitalu:

```
Pacjenci (taski):
- Krytyczny (priorytet wysoki): zawał → natychmiast
- Pilny (priorytet średni): złamanie → czeka
- Routine (priorytet niski): kontrola → czeka długo

Scheduler (pielęgniarka triage):
- Decyduje kto idzie pierwszy
- Może przerwać jednego pacjenta dla krytycznego
- Preemptive scheduling!
```

### 🐜 Mrowisko

Każda mrówka ma swoje zadanie, ale królowa (scheduler) decyduje:

```
Zagrożenie → Żołnierze idą pierwsi (high priority)
Budowa → Robotnice kontynuują (medium priority)
Sprzątanie → Mrówki sprzątające na końcu (low priority)

Królowa/sygnały chemiczne = scheduler
```

### 🎭 Teatr

W teatrze reżyser (scheduler) decyduje kto jest na scenie:

```
Scena (CPU) może pomieścić tylko jednego aktora na raz.
Reżyser (scheduler):
- Daje sygnał aktorom (taskom) kiedy wejść
- Może przerwać scenę (preempt)
- Decyduje o kolejności (scheduling)

Aktorzy czekają w "ready queue" za kulisami.
```

---

## Podobieństwo do systemów informatycznych

### Load Balancer

```nginx
# Nginx jako "scheduler" dla requestów
upstream backend {
    server backend1:8080;
    server backend2:8080;
    server backend3:8080;
}

# Round-robin scheduling (domyślny)
# Decyduje który serwer obsłuży request
```

### Process Scheduler (Linux)

```bash
# Linux CFS (Completely Fair Scheduler)
# Scheduler dla procesów

ps -eo pid,comm,pri,nice
# PID  COMMAND    PRI  NI
# 1    init        20   0
# 1234 firefox     15   0
# 5678 gcc         25  -5

# PRI i NI wpływają na scheduling
```

### Thread Pool

```java
// Thread pool jako "scheduler" zadań
ExecutorService executor = Executors.newFixedThreadPool(4);
executor.submit(task1);  // Scheduler przydziela wątek
executor.submit(task2);
executor.submit(task3);
```

---

## Rodzaje schedulerów

### 1. Preemptive vs Non-preemptive

```
PREEMPTIVE:
Task A running: ████████████
Task B arrives:      │
                   ├──┤ B preempts A
                   ▼
Result:        ████┌────┐████
                    │ B  │
Task B przerywa A i przejmuje CPU.

NON-PREEMPTIVE:
Task A running: ████████████
Task B arrives:      │
                   └── waits
                   ▼
Result:        ████████████┌────┐
                            │ B  │
Task B czeka aż A skończy.
```

### 2. Priority-based

```
┌─────────────────────────────────────────────────────────┐
│              PRIORITY SCHEDULING                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Priority 3: ┌─────┐ ┌─────┐                           │
│              │ T1  │ │ T3  │                           │
│              └─────┘ └─────┘                           │
│                                                         │
│  Priority 2: ┌─────┐                                   │
│              │ T2  │                                   │
│              └─────┘                                   │
│                                                         │
│  Priority 1: ┌─────┐                                   │
│              │ T4  │  ← T4 czeka na wszystkich        │
│              └─────┘                                   │
│                                                         │
│  Execution: T1 → T3 → T2 → T4                          │
│  (najwyższy priorytet pierwszy)                        │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 3. Round-Robin

```
┌─────────────────────────────────────────────────────────┐
│               ROUND-ROBIN SCHEDULING                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Ready Queue: [T1] → [T2] → [T3] → [T1] → ...         │
│                                                         │
│  Time slice = 10ms                                      │
│                                                         │
│  Execution:                                             │
│  T1: 10ms → T2: 10ms → T3: 10ms → T1: 10ms → ...      │
│                                                         │
│  Każdy task dostaje równy czas (time quantum).         │
│  Świetne dla fairness, słabe dla RT.                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 4. EDF (Earliest Deadline First)

```
┌─────────────────────────────────────────────────────────┐
│                EDF SCHEDULING                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Time t=0:                                              │
│  T1: deadline=5ms → earliest → run T1                  │
│  T2: deadline=10ms                                      │
│  T3: deadline=8ms                                       │
│                                                         │
│  Time t=2 (T1 done):                                    │
│  T2: deadline=10ms                                      │
│  T3: deadline=8ms → earliest → run T3                  │
│                                                         │
│  Dynamic priorities based on deadline.                  │
│  Optimal utilization.                                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Scheduler w RTOS

### FreeRTOS Scheduler

```c
// FreeRTOS używa preemptive priority-based scheduling
// z opcjonalnym round-robin dla tasków tego samego priorytetu

void vApplicationIdleHook(void) {
    // Wywoływane gdy brak ready tasków
    // Low-power mode
}

// Scheduler start
vTaskStartScheduler();

// Scheduler decisions:
// 1. Zawsze wybierz ready task z najwyższym priorytetem
// 2. Preempt jeśli wyższy priorytet becomes ready
// 3. Round-robin dla tasków tego samego priorytetu (time slicing)
```

### Context Switch

```
┌─────────────────────────────────────────────────────────┐
│                  CONTEXT SWITCH                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A running:                                        │
│  ┌─────────────────────────────────┐                   │
│  │ R0=0x01 R1=0x02 R2=0x03 ...    │                   │
│  │ SP=0x2000 PC=0x0800 ...        │                   │
│  └─────────────────────────────────┘                   │
│                    │                                    │
│                    │ Save context                       │
│                    ▼                                    │
│  Task A TCB:                                           │
│  ┌─────────────────────────────────┐                   │
│  │ Saved: R0-R15, SP, PC, PSR      │                   │
│  └─────────────────────────────────┘                   │
│                                                         │
│  Scheduler decision: Switch to Task B                   │
│                                                         │
│  Task B TCB:                                           │
│  ┌─────────────────────────────────┐                   │
│  │ Saved: R0-R15, SP, PC, PSR      │                   │
│  └─────────────────────────────────┘                   │
│                    │                                    │
│                    │ Restore context                    │
│                    ▼                                    │
│  Task B running:                                        │
│  ┌─────────────────────────────────┐                   │
│  │ R0=0x10 R1=0x20 R2=0x30 ...    │                   │
│  │ SP=0x3000 PC=0x1000 ...        │                   │
│  └─────────────────────────────────┘                   │
│                                                         │
│  Context switch kosztuje ~10-100 cycles.               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Scheduling Decisions

### Kiedy scheduler podejmuje decyzję?

```
1. TASK CREATION
   Nowy task created → czy ma wyższy priorytet?

2. TASK TERMINATION
   Task skończył → wybierz następny

3. BLOCKING
   Task czeka na zasób → wybierz następny

4. UNBLOCKING
   Task becomes ready → czy ma wyższy priorytet?

5. TICK INTERRUPT
   Time slice expired → round robin?

6. YIELD
   Task dobrowolnie oddaje CPU
```

### Pseudokod scheduler

```c
void schedule(void) {
    // Znajdź task z najwyższym priorytetem w ready queue
    Task* highest_priority_task = find_highest_priority_ready();

    if (highest_priority_task == NULL) {
        // Brak tasków - idle
        run_idle_task();
        return;
    }

    Task* current = get_current_task();

    // Czy trzeba switch?
    if (current == NULL ||
        highest_priority_task->priority > current->priority ||
        (current->state != RUNNING)) {

        // Context switch
        context_switch(current, highest_priority_task);
    }
}
```

---

## Scheduling Policies

### Fixed Priority Preemptive (FreeRTOS default)

```
Zalety:
✓ Prosty
✓ Przewidywalny
✓ Mały overhead

Wady:
✗ Priority inversion
✗ Nieoptymalna utilisacja
✗ Nie obsługuje deadline
```

### Rate Monotonic (RMS)

```
Priorytety = f(period)
Krótszy period = wyższy priorytet

Zalety:
✓ Deterministyczny
✓ Łatwa analiza

Wady:
✗ Max ~69% utilisacji
✗ Nie obsługuje deadline ≠ period
```

### Earliest Deadline First (EDF)

```
Priorytety = f(deadline)
Najwcześniejszy deadline = najwyższy priorytet

Zalety:
✓ Optymalna utilisacja (100%)
✓ Obsługuje dowolne deadline

Wady:
✗ Dynamiczne priorytety (overhead)
✗ Skomplikowany
✗ Nieprzewidywalna degradacja przy przeciążeniu
```

---

## Scheduler Overhead

```
┌─────────────────────────────────────────────────────────┐
│                 SCHEDULER OVERHEAD                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Tick interrupt:     ~5-10 cycles                      │
│  Scheduler decision:  ~10-50 cycles                    │
│  Context switch:     ~10-100 cycles                    │
│                                                         │
│  Total per switch:    ~25-160 cycles                   │
│                                                         │
│  Przy tick rate 1000 Hz:                               │
│  - 1000 ticków/s                                       │
│  - ~100 cycles/tick = 100,000 cycles/s                 │
│  - Na CPU 100 MHz = 0.1% overhead                      │
│                                                         │
│  Ale przy wielu context switches:                      │
│  - 10,000 switches/s × 100 cycles = 1,000,000 cycles   │
│  - = 1% overhead na 100 MHz                            │
│                                                         │
│  Overhead może być znaczący w systemach RT!            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Priorytety i problemy

### Priority Inversion

```
High Priority Task H
Medium Priority Task M
Low Priority Task L

1. L trzyma mutex
2. H chce mutex → czeka na L
3. M jest ready → preemptuje L
4. H czeka na L, L czeka na M

H czeka na M! (M ma niższy priorytet!)

To jest Priority Inversion.
```

### Starvation

```
High Priority Tasks: ciągle ready
Low Priority Task: nigdy nie dostaje CPU

Task L "głoduje" - nigdy się nie wykonuje.

Rozwiązanie: Aging (stopniowe zwiększanie priorytetu)
```

### Priority Inheritance

```c
// Rozwiązanie Priority Inversion
// Kiedy H czeka na mutex trzymany przez L:
// L tymczasowo dostaje priorytet H

void mutex_lock(mutex_t* m) {
    if (m->owner != NULL) {
        // Priority Inheritance
        if (current_task->priority > m->owner->priority) {
            m->owner->inherited_priority = current_task->priority;
        }
        // Block...
    }
}
```

---

## Scheduler Tuning

### Tick Rate

```c
// FreeRTOS tick rate configuration
#define configTICK_RATE_HZ    1000  // 1ms tick

// Wyższy tick rate:
// + Lepsza responsywność
// + Dokładniejsze timing
// - Więcej overhead
// - Więcej context switches

// Niższy tick rate:
// + Mniejszy overhead
// + Mniej context switches
// - Gorsza responsywność
// - Gorsze timing resolution
```

### Time Slicing

```c
// FreeRTOS time slicing
#define configUSE_TIME_SLICING  1

// Time slicing ON (default):
// Taski tego samego priorytetu轮流ują

// Time slicing OFF:
// Task działa aż do block/yield
```

### Minimal Stack Size

```c
// FreeRTOS stack size
#define configMINIMAL_STACK_SIZE  128  // words

// Za mały stack:
// - Stack overflow
// - Korupcja danych

// Za duży stack:
// - Marnowanie RAM
// - Mniej tasków possible
```

---

## Scheduler w praktyce

### Monitorowanie scheduler

```c
// FreeRTOS task stats
void print_task_stats(void) {
    char buffer[512];
    vTaskList(buffer);
    printf("Task          State  Priority  Stack  #%\n");
    printf("%s\n", buffer);
}

// Output:
// Task          State  Priority  Stack  #%
// IDLE          R      0         100    1
// SensorTask    B      2         50     2
// ControlTask   R      3         80     3
// LogTask       B      1         90     4
```

### Hook functions

```c
// FreeRTOS hooks

// Idle hook - gdy brak tasków
void vApplicationIdleHook(void) {
    __WFI();  // Low power mode
}

// Tick hook - każdy tick
void vApplicationTickHook(void) {
    // Monitorowanie, counters, etc.
}

// Stack overflow hook
void vApplicationStackOverflowHook(TaskHandle_t task, char* name) {
    printf("Stack overflow in task %s!\n", name);
    while(1);  // Hang
}

// Malloc failed hook
void vApplicationMallocFailedHook(void) {
    printf("Malloc failed!\n");
    while(1);
}
```

---

## Pytania do przemyślenia

1. Jaki typ scheduler używa Twój RTOS?
2. Jakie są priorytety Twoich tasków?
3. Jak często scheduler podejmuje decyzję (tick rate)?

---

## Quiz

**Pytanie**: Masz trzy taski:

```
Task A: Priority 3 (high), ready
Task B: Priority 2 (medium), ready
Task C: Priority 2 (medium), blocked on semaphore
Task D: Priority 1 (low), ready
```

Jaka będzie kolejność wykonywania? Co się stanie gdy C stanie się ready?

**Odpowiedź**:

```
Początkowa kolejność:
1. Task A (highest priority ready)
2. Task B (drugi w kolejności)
3. Task D (najniższy)

Gdy C becomes ready:
- C ma priority 2 (tak samo jak B)
- Z z time slicing: C i B轮流ują
- Z time slicing: C czeka aż B yield/block

Gdyby C miał priority > 2:
- C preemptuje B (lub A jeśli > 3)
- Natychmiastowo przejmuje CPU
```

---

## Wskazówka zapamiętywania

> **Scheduler = Sędzia w tenisie**
>
> Sędzia (scheduler) decyduje:
> - Kto gra teraz (running task)
> - Kto czeka (ready queue)
> - Kiedy zmiana (context switch)
> - Kto ma priorytet (priority)
>
> Gracze (taski) proszą o uwagę:
> - "Jestem gotowy!" (ready)
> - "Muszę odpocząć" (blocked)
> - "Skończyłem" (terminated)
>
> Sędzia jest sprawiedliwy (fairness) ale uznaje priorytety.
> Wimbledon (grand slam) ma innych sędziów niż lokalny turniej (małe wymagania).
> Tak jak RTOS ma inne schedulery niż Linux.