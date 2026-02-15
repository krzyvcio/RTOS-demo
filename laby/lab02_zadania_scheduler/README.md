# Laboratorium 2: Zadania i Scheduler

**Czas:** 2 godziny
**Punkty:** 15 pkt

---

## Cel ćwiczenia

1. Tworzenie zadań z różnymi parametrami
2. Zrozumienie stanów zadań
3. Manipulacja priorytetami w runtime
4. Obserwacja działania schedulera

---

## Teoria (15 min)

### Stany zadania w FreeRTOS

```
                 ┌─────────────┐
                 │   RUNNING   │
                 └──────┬──────┘
                        │
           ┌────────────┼────────────┐
           │            │            │
           ▼            │            ▼
    ┌─────────────┐     │     ┌─────────────┐
    │    READY    │◄────┴────►│   BLOCKED   │
    └─────────────┘           └─────────────┘
           ▲                        │
           │                        │
           └────────────────────────┘
```

**Stany:**
- **Running** - Zadanie wykonuje się na CPU
- **Ready** - Zadanie gotowe, czeka na CPU
- **Blocked** - Zadanie czeka na zdarzenie (delay, semafor, queue)

### Priorytety

W FreeRTOS:
- Wyższy numer = wyższy priorytet
- `configMAX_PRIORITIES` definiuje zakres
- Zalecane używanie `tskIDLE_PRIORITY` jako bazy

---

## Zadanie 1: Tworzenie zadań (30 min)

### 1.1 Podstawowe tworzenie zadania

```c
#include "FreeRTOS.h"
#include "task.h"
#include <stdio.h>

// Handler zadania
TaskHandle_t task1_handle;

void vTask1(void *pvParameters) {
    const char *name = (const char *)pvParameters;

    while (1) {
        printf("%s running\n", name);
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

int main(void) {
    xTaskCreate(
        vTask1,                    // Funkcja zadania
        "Task1",                   // Nazwa (do debugowania)
        configMINIMAL_STACK_SIZE,  // Rozmiar stosu (słowa)
        "Task1",                   // Parametr przekazywany do zadania
        tskIDLE_PRIORITY + 1,      // Priorytet
        &task1_handle              // Handler (opcjonalny)
    );

    vTaskStartScheduler();
    for (;;);
    return 0;
}
```

### 1.2 TODO - Utwórz zadania

Utwórz 3 zadania o różnych priorytetach:

```c
// TODO: Utwórz zadania
// Task A: priorytet 1, okres 100ms
// Task B: priorytet 2, okres 200ms
// Task C: priorytet 3, okres 500ms
```

**CHECK:** Pokaż prowadzącemu działający program.

---

## Zadanie 2: Przełączanie kontekstu (30 min)

### 2.1 Obserwacja przełączania

```c
volatile uint32_t task_a_counter = 0;
volatile uint32_t task_b_counter = 0;

void vTaskA(void *pvParameters) {
    while (1) {
        task_a_counter++;
        // Nie ma delay - ciągłe wykonywanie
    }
}

void vTaskB(void *pvParameters) {
    while (1) {
        task_b_counter++;
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

// Monitor task - wypisuje statystyki
void vMonitorTask(void *pvParameters) {
    while (1) {
        printf("Task A: %lu, Task B: %lu\n",
               task_a_counter, task_b_counter);
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}
```

### 2.2 Pytania

1. Dlaczego Task A ma znacznie więcej wykonań?
2. Co się stanie, jeśli Task A też ma delay?
3. Dlaczego Task B w ogóle się wykonuje (ma niższy priorytet)?

### 2.3 TODO - Eksperyment

Dodaj `vTaskDelay(pdMS_TO_TICKS(1))` w Task A. Jak to wpłynie na Task B?

---

## Zadanie 3: Manipulacja priorytetami (25 min)

### 3.1 Zmiana priorytetu w runtime

```c
TaskHandle_t task_a_handle;
TaskHandle_t task_b_handle;

void vTaskA(void *pvParameters) {
    while (1) {
        printf("Task A (prio: %d)\n", uxTaskPriorityGet(NULL));
        vTaskDelay(pdMS_TO_TICKS(500));
    }
}

void vTaskB(void *pvParameters) {
    int counter = 0;

    while (1) {
        printf("Task B (prio: %d)\n", uxTaskPriorityGet(NULL));
        vTaskDelay(pdMS_TO_TICKS(500));

        counter++;
        if (counter == 5) {
            // Zmień priorytet Task A
            vTaskPrioritySet(task_a_handle, 3);
            printf("Changed Task A priority to 3!\n");
        }
    }
}

int main(void) {
    xTaskCreate(vTaskA, "TaskA", 128, NULL, 1, &task_a_handle);
    xTaskCreate(vTaskB, "TaskB", 128, NULL, 2, &task_b_handle);

    vTaskStartScheduler();
    for (;;);
    return 0;
}
```

### 3.2 TODO - Zmiana priorytetu

Zaimplementuj:
- Po 10 iteracjach Task B obniża swój priorytet do 0
- Obserwuj co się dzieje z wykonywaniem zadań

---

## Zadanie 4: Zawieszanie i wznawianie zadań (20 min)

### 4.1 API dla zawieszania

```c
// Zawieś zadanie
void vTaskSuspend(TaskHandle_t xTaskToSuspend);

// Wznów zadanie
void vTaskResume(TaskHandle_t xTaskToResume);

// Wznów z ISR
BaseType_t xTaskResumeFromISR(TaskHandle_t xTaskToResume);
```

### 4.2 Przykład

```c
TaskHandle_t worker_handle;

void vWorkerTask(void *pvParameters) {
    int counter = 0;

    while (1) {
        printf("Worker: %d\n", counter++);
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vControllerTask(void *pvParameters) {
    while (1) {
        printf("Suspending worker...\n");
        vTaskSuspend(worker_handle);
        vTaskDelay(pdMS_TO_TICKS(2000));

        printf("Resuming worker...\n");
        vTaskResume(worker_handle);
        vTaskDelay(pdMS_TO_TICKS(2000));
    }
}
```

### 4.3 TODO - Zawieszanie cykliczne

Zaimplementuj zadanie, które:
- Co 5 sekund zawiesza wszystkie zadania robocze
- Po 2 sekundach je wznawia
- Wypisuje stan systemu

---

## Zadanie 5: Informacje o zadaniach (15 min)

### 5.1 Task statistics

```c
void vPrintTaskInfo(void) {
    TaskStatus_t *task_array;
    UBaseType_t num_tasks;
    uint32_t total_runtime;

    // Pobierz liczbę zadań
    num_tasks = uxTaskGetNumberOfTasks();

    // Alokuj pamięć
    task_array = pvPortMalloc(num_tasks * sizeof(TaskStatus_t));

    // Pobierz dane
    num_tasks = uxTaskGetSystemState(
        task_array,
        num_tasks,
        &total_runtime
    );

    printf("\n%-12s %-8s %-8s %-8s\n",
           "Name", "State", "Prio", "Stack");
    printf("----------------------------------------\n");

    for (int i = 0; i < num_tasks; i++) {
        printf("%-12s %-8d %-8d %-8d\n",
               task_array[i].pcTaskName,
               task_array[i].eCurrentState,
               task_array[i].uxCurrentPriority,
               task_array[i].usStackHighWaterMark);
    }

    vPortFree(task_array);
}
```

### 5.2 TODO - Monitor zasobów

Stwórz zadanie monitora, które co 5 sekund:
- Wypisuje informacje o wszystkich zadaniach
- Wyświetla zużycie stosu (stack high water mark)
- Oblicza procent czasu CPU dla każdego zadania

---

## Zadanie Bonus: Watchdog dla zadań

Zaimplementuj system watchdog:
- Każde zadanie musi się "zgłosić" co określony czas
- Zadanie watchdog sprawdza, czy zadania się zgłaszają
- Jeśli zadanie nie zgłosi się w czasie → restart systemu

```c
// Szkic rozwiązania
#define TASK_COUNT 3
uint32_t last_checkin[TASK_COUNT];

void vWorkerTask(void *pvParameters) {
    int id = (int)pvParameters;

    while (1) {
        do_work();
        last_checkin[id] = xTaskGetTickCount();  // Zgłoszenie
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vWatchdogTask(void *pvParameters) {
    while (1) {
        for (int i = 0; i < TASK_COUNT; i++) {
            if (xTaskGetTickCount() - last_checkin[i] > TIMEOUT) {
                printf("Task %d hung! Rebooting...\n", i);
                // System reset
            }
        }
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}
```

---

## Tabela API

| Funkcja | Opis |
|---------|------|
| `xTaskCreate()` | Utwórz zadanie |
| `vTaskDelete()` | Usuń zadanie |
| `vTaskDelay()` | Opóźnienie |
| `vTaskDelayUntil()` | Opóźnienie do czasu absolutnego |
| `vTaskSuspend()` | Zawieś zadanie |
| `vTaskResume()` | Wznów zadanie |
| `vTaskPrioritySet()` | Ustaw priorytet |
| `uxTaskPriorityGet()` | Pobierz priorytet |
| `uxTaskGetNumberOfTasks()` | Liczba zadań |
| `uxTaskGetSystemState()` | Stan systemu |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | 3 zadania utworzone | 3 |
| 2 | Przełączanie kontekstu zrozumiałe | 3 |
| 3 | Manipulacja priorytetami | 3 |
| 4 | Zawieszanie/wznawianie | 3 |
| 5 | Informacje o zadaniach | 3 |

---

## Sprawozdanie

1. Opisz przebieg przełączania kontekstu (diagram)
2. Wyjaśnij wpływ priorytetów na działanie systemu
3. Zmierz i wykresluj "stack high water mark" dla zadań
4. Wnioski o konieczności `vTaskDelay`

---

## Pytania kontrolne

1. Co się dzieje z zadaniem w stanie Blocked?
2. Dlaczego nie należy używać `vTaskSuspend` do synchronizacji?
3. Jak obliczyć minimalny rozmiar stosu?
4. Co to jest "stack high water mark"?
5. Dlaczego Idle Task jest potrzebny?