# Task (Zadanie)

## Definicja

**Task** to podstawowa jednostka wykonawcza w RTOS - niezależny wątek wykonania z własnym kontekstem (stosem, rejestrami, stanem). Task to "program wewnątrz programu" - ma swój kod, swoje dane, swój czas procesora.

> Task to wątek z supermoceami: priorytet, deadline, możliwość wywłaszczania, i deterministyczne zachowanie.

```
┌─────────────────────────────────────────────────────────┐
│                    TASK STRUCTURE                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────┐                                   │
│  │    Task Control │   ┌─────────────────────────────┐│
│  │       Block     │   │        Stack                ││
│  │  ─────────────  │   │  ┌───────────────────────┐ ││
│  │  Stack Pointer  │──►│  │ Local variables       │ ││
│  │  PC Register    │   │  │ Return addresses      │ ││
│  │  Registers      │   │  │ Function parameters   │ ││
│  │  Priority       │   │  │ ─────────────────────│ ││
│  │  State          │   │  │ Stack grows down      │ ││
│  │  ─────────────  │   │  └───────────────────────┘ ││
│  │  Task ID        │   └─────────────────────────────┘│
│  │  Entry Point    │                                   │
│  └─────────────────┘                                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🐜 Kolonia mrówek

Każda mrówka to "task":

```
Mrówka-żołnierz: chroni mrowisko (task ochrony)
Mrówka-robotnica: buduje (task konstruktora)
Mrówka-zbieraczka: szuka jedzenia (task forager)
Mrówka-królowa: składa jaja (task reprodukcji)

Każda mrówka ma:
- Swój "kod" (instynkt)
- Swój priorytet (królowa > żołnierz > robotnica)
- Swój stan (aktywny, odpoczywa, je)
```

### 🧬 Komórki organizmu

Każda komórka to "task":

```
Kardiomiocyt: bije serce (task pump)
Neuron: przewodzi sygnały (task signal)
Hepatocyt: metabolizuje (task detox)
Leukocyt: walczy z infekcjami (task immune)

Każda komórka ma:
- Własne "programowanie" (DNA)
- Własne zasoby (mitochondria, rybosomy)
- Własny cykl życia
```

### 🐕 Suki zaprzęgowe

Każdy pies to "task":

```
Pies przewodnik: dyktuje tempo (high priority)
Pies pomocniczy: ciągnie (medium priority)
Pies tylny: stabilizuje (low priority)

Wszystkie psy współpracują, ale każde ma swoją rolę.
```

---

## Podobieństwo do systemów informatycznych

### Threads (wątki)

```c
// Linux/POSIX thread
pthread_t thread;
pthread_create(&thread, NULL, thread_function, NULL);

// To jest task, ale bez gwarancji RTOS:
// - Brak gwarantowanego priorytetu
// - Brak gwarantowanego czasu wykonania
// - Preemption może być nieprzewidywalne
```

### Processes (procesy)

```c
// Linux process
fork();

// Proces to task z własną przestrzenią adresową
// Większa izolacja, ale większy overhead
```

### JavaScript async functions

```javascript
// "Task" w JavaScript
async function fetchData() {
    const response = await fetch(url);
    return response.json();
}

// To nie jest prawdziwy task (single-threaded)
// Ale zachowuje się jak task z cooperative multitasking
```

---

## Stany tasku

### Maszyna stanów

```
                    ┌─────────────┐
                    │   CREATED   │
                    └──────┬──────┘
                           │ create
                           ▼
                    ┌─────────────┐
           ready ◄──┤    READY    ├──► running
                    └──────┬──────┘
                           │ dispatch
                           ▼
                    ┌─────────────┐
           blocked ◄──┤   RUNNING   ├──► preempted
                    └──────┬──────┘
                           │ block (wait)
                           ▼
                    ┌─────────────┐
           ready ◄──┤   BLOCKED   │
                    └──────┬──────┘
                           │ terminate
                           ▼
                    ┌─────────────┐
                    │  TERMINATED │
                    └─────────────┘
```

### Opis stanów

```
CREATED: Task utworzony, ale nie startował
READY: Task gotowy do wykonania, czeka na CPU
RUNNING: Task wykonuje się na CPU
BLOCKED: Task czeka na zasób (mutex, semafor, I/O)
TERMINATED: Task zakończony
```

---

## Atrybuty tasku

```c
typedef struct {
    // Identyfikacja
    uint32_t id;                    // Unikalny ID
    const char* name;               // Nazwa

    // Stan
    TaskState state;                // Ready/Running/Blocked
    uint32_t priority;              // 0 (low) to N (high)

    // Kontekst
    void* stack;                    // Wskaźnik na stos
    uint32_t stack_size;            // Rozmiar stosu
    CPU_Context context;            // Rejestry

    // Timing
    uint32_t period;                // Okres (dla periodic tasks)
    uint32_t wcet;                  // Worst-case execution time
    uint32_t deadline;              // Deadline

    // Statystyki
    uint32_t cpu_time;              // Zużyty czas CPU
    uint32_t wakeups;               // Liczba wybudzeń
    uint32_t preemptions;           // Liczba wywłaszczeń
} Task;
```

---

## Tworzenie tasku

### FreeRTOS

```c
void sensor_task(void* pvParameters) {
    while (1) {
        read_sensor();
        process_data();
        vTaskDelay(pdMS_TO_TICKS(100));  // Czekaj 100ms
    }
}

void main(void) {
    xTaskCreate(
        sensor_task,          // Funkcja tasku
        "SensorTask",         // Nazwa
        256,                  // Stack size (words)
        NULL,                 // Parameters
        2,                    // Priority
        NULL                  // Handle
    );

    vTaskStartScheduler();    // Start RTOS
}
```

### Zephyr

```c
K_THREAD_DEFINE(sensor_thread,      // Nazwa
                1024,                // Stack size
                sensor_task,         // Funkcja
                NULL, NULL, NULL,    // Parametry
                2,                   // Priority
                0,                   // Options
                100);                // Delay (ms)
```

### AUTOSAR

```c
TASK(SensorTask) {
    // Task body
    read_sensor();
    process_data();

    // Terminate task (AUTOSAR style)
    TerminateTask();
}

// Konfiguracja w .arxml (XML)
```

---

## Priorytety tasków

### Hierarchia priorytetów

```
┌─────────────────────────────────────────────────────────┐
│                 PRIORITY LEVELS                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  HIGH ─────────┬─ ISR (highest)                        │
│                ├─ Critical tasks (safety)              │
│                ├─ High priority tasks                  │
│                ├─ Normal tasks                         │
│                ├─ Low priority tasks                   │
│  LOW ──────────┴─ Idle task (lowest)                   │
│                                                         │
│  Wyższy priorytet = częstsze wykonywanie               │
│  (w preemptive scheduling)                             │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Priorytety w praktyce

```c
// FreeRTOS: Wyższy numer = wyższy priorytet
#define PRIORITY_IDLE       0
#define PRIORITY_LOW        1
#define PRIORITY_NORMAL     2
#define PRIORITY_HIGH       3
#define PRIORITY_CRITICAL   4

xTaskCreate(task_a, "A", 256, NULL, PRIORITY_HIGH, NULL);
xTaskCreate(task_b, "B", 256, NULL, PRIORITY_NORMAL, NULL);
xTaskCreate(task_c, "C", 256, NULL, PRIORITY_LOW, NULL);
```

---

## Cykl życia tasku

### Periodic Task

```c
void periodic_task(void* pvParameters) {
    // Inicjalizacja (raz)
    init_hardware();

    // Pętla nieskończona
    while (1) {
        // Czekaj na start okresu
        wait_for_period();

        // Wykonaj pracę
        do_work();

        // Czekaj do końca okresu
        wait_until_end_of_period();
    }
}
```

### Sporadic Task

```c
void sporadic_task(void* pvParameters) {
    while (1) {
        // Czekaj na zdarzenie (niezależne od czasu)
        wait_for_event();

        // Obsłuż zdarzenie
        handle_event();

        // Czekaj na następne zdarzenie
    }
}
```

### Aperiodic Task

```c
void aperiodic_task(void* pvParameters) {
    while (1) {
        // Czekaj na sygnał (zewnętrzny trigger)
        ulTaskNotifyTake(pdTRUE, portMAX_DELAY);

        // Wykonaj jednorazową pracę
        handle_request();

        // Wróć do czekania
    }
}
```

---

## Task Communication

### Shared Memory

```c
// Niebezpieczne bez synchronizacji!
int shared_data;

void task_a(void) {
    shared_data = 42;  // Race condition!
}

void task_b(void) {
    int local = shared_data;  // Może być 0 lub 42
}
```

### Message Queue

```c
QueueHandle_t queue;

void task_producer(void) {
    int data = 42;
    xQueueSend(queue, &data, portMAX_DELAY);
}

void task_consumer(void) {
    int received;
    xQueueReceive(queue, &received, portMAX_DELAY);
}
```

### Task Notification

```c
TaskHandle_t task_b_handle;

void task_a(void) {
    // Zasygnalizuj task B
    xTaskNotifyGive(task_b_handle);
}

void task_b(void) {
    // Czekaj na sygnał od task A
    ulTaskNotifyTake(pdTRUE, portMAX_DELAY);
}
```

---

## Task vs Process vs Thread

| Cecha | Task (RTOS) | Thread (Linux) | Process (Linux) |
|-------|-------------|----------------|-----------------|
| Własna pamięć | Nie (współdzielona) | Nie | Tak |
| Własny stos | Tak | Tak | Tak |
| Preemption | Deterministyczna | Może być | Może być |
| Overhead | Bardzo mały | Mały | Duży |
| Izolacja | Brak/mała | Mała | Duża |
| Priorytety | Deterministyczne | "Nice" values | "Nice" values |

---

## Common Patterns

### Producer-Consumer

```c
QueueHandle_t data_queue;

void producer_task(void) {
    while (1) {
        Data data = read_sensor();
        xQueueSend(data_queue, &data, portMAX_DELAY);
    }
}

void consumer_task(void) {
    while (1) {
        Data data;
        xQueueReceive(data_queue, &data, portMAX_DELAY);
        process_data(data);
    }
}
```

### Worker Pool

```c
void worker_task(void* params) {
    WorkQueue* wq = (WorkQueue*)params;
    while (1) {
        WorkItem* item = get_work(wq);
        process_work(item);
        free_work(item);
    }
}

// Utwórz N workerów
for (int i = 0; i < NUM_WORKERS; i++) {
    xTaskCreate(worker_task, "Worker", 256, &work_queue, 2, NULL);
}
```

### State Machine Task

```c
typedef enum { STATE_INIT, STATE_RUN, STATE_ERROR } State;

void state_machine_task(void) {
    State state = STATE_INIT;

    while (1) {
        switch (state) {
            case STATE_INIT:
                if (init_hardware()) state = STATE_RUN;
                else state = STATE_ERROR;
                break;

            case STATE_RUN:
                if (run_iteration()) state = STATE_RUN;
                else state = STATE_ERROR;
                break;

            case STATE_ERROR:
                handle_error();
                state = STATE_INIT;
                break;
        }
        vTaskDelay(pdMS_TO_TICKS(10));
    }
}
```

---

## Dlaczego taski są trudne?

### Problem 1: Race Conditions

```c
int counter = 0;

void task_a(void) {
    for (int i = 0; i < 1000; i++) {
        counter++;  // Nieatomicowe!
    }
}

void task_b(void) {
    for (int i = 0; i < 1000; i++) {
        counter++;  // Race condition!
    }
}

// counter może być < 2000!
```

### Problem 2: Deadlock

```c
Mutex m1, m2;

void task_a(void) {
    lock(m1);
    lock(m2);  // Może zablokować jeśli B trzyma m2
    // ...
    unlock(m2);
    unlock(m1);
}

void task_b(void) {
    lock(m2);
    lock(m1);  // Może zablokować jeśli A trzyma m1
    // ...
    unlock(m1);
    unlock(m2);
}

// Deadlock: A czeka na m2, B czeka na m1
```

### Problem 3: Priority Inversion

```c
// Low priority trzyma mutex
// Medium priority preempts low
// High priority czeka na mutex
// Result: High czeka na Medium! (Inversion!)
```

---

## Pytania do przemyślenia

1. Ile tasków ma Twój system? Jakie mają priorytety?
2. Czy znasz WCET każdego tasku?
3. Jak taski komunikują się między sobą? Czy jest bezpieczne?

---

## Quiz

**Pytanie**: Masz system z trzema taskami:

```
Task A: priorytet 1 (niski), period 100ms
Task B: priorytet 2 (średni), period 50ms
Task C: priorytet 3 (wysoki), period 20ms
```

Który task będzie wykonywany najczęściej?

**Odpowiedź**:

```
W preemptive scheduling z priorytetami:
- Task C ma najwyższy priorytet
- Będzie wykonywany gdy jest ready
- Ale również ma najkrótszy period (20ms)

Więc:
- C wykonuje się co 20ms (gdy ready)
- B wykonuje się gdy C nie jest ready
- A wykonuje się gdy C i B nie są ready

Task C będzie wykonywany najczęściej - zarówno ze względu
na priorytet jak i period.
```

---

## Wskazówka zapamiętywania

> **Task = Pracownik w firmie**
>
> Każdy pracownik (task) ma:
> - Swoje biurko (stos)
> - Swoje zadania (kod)
> - Swoje narzędzia (rejestry)
> - Swój priorytet (stanowisko)
>
> Dyrektor (high priority task) może przerwać pracę pracownika.
> Pracownik może czekać na zasoby (blocked).
> Wszyscy współpracują, ale mogą też konkurować.
>
> Dobra firma = dobre zarządzanie taskami.