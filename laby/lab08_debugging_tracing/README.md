# Laboratorium 8: Debugging i Tracing

**Czas:** 2 godziny
**Punkty:** 10 pkt

---

## Cel ćwiczenia

1. Techniki debugowania systemów RTOS
2. Task statistics i monitoring
3. Stack overflow detection
4. Tracing przełączania kontekstu

---

## Teoria (15 min)

### Specyfika debugowania RTOS

```
Problemy:
1. Non-determinizm - błąd pojawia się "czasem"
2. Race conditions - zależne od timing
3. Heisenbugs - debugowanie zmienia zachowanie
4. Deadlocks - trudne do reprodukowania

Narzędzia:
- Printf debugging (z ostrożnością)
- Task statistics
- Stack monitoring
- Trace analysis
- Assert hooks
```

---

## Zadanie 1: Task Statistics (25 min)

### 1.1 Wbudowane funkcje statystyczne

```c
#include "FreeRTOS.h"
#include "task.h"
#include <stdio.h>

// Wymaga w FreeRTOSConfig.h:
// #define configUSE_TRACE_FACILITY          1
// #define configGENERATE_RUN_TIME_STATS     1
// #define configUSE_STATS_FORMATTING_FUNCTIONS 1

void vPrintTaskStats(void) {
    char buffer[512];

    // Prosty format
    vTaskList(buffer);
    printf("Name          State  Priority  Stack  #\n");
    printf("%s\n", buffer);
}

// Bardziej szczegółowe stats
void vPrintRuntimeStats(void) {
    char buffer[512];

    // Wymaga configGENERATE_RUN_TIME_STATS
    vTaskGetRunTimeStats(buffer);
    printf("Name          Abs Time    %% Time\n");
    printf("%s\n", buffer);
}

void vMonitorTask(void *pvParameters) {
    while (1) {
        printf("\n=== Task Statistics ===\n");
        vPrintTaskStats();

        // Co 5 sekund
        vTaskDelay(pdMS_TO_TICKS(5000));
    }
}

int main(void) {
    xTaskCreate(vMonitorTask, "Monitor", 256, NULL, 1, NULL);

    // Inne taski...

    vTaskStartScheduler();
    return 0;
}
```

### 1.2 Wyjście

```
=== Task Statistics ===
Name          State  Priority  Stack  #
Monitor       R      1         200    1
Sensor        B      2         150    2
Control       R      3         180    3
IDLE          R      0         100    4

Legenda:
R = Ready/Running
B = Blocked
S = Suspended
D = Deleted
```

### 1.3 TODO - Custom statistics

Zaimplementuj własny monitor:

```c
typedef struct {
    const char* name;
    uint32_t executions;
    uint32_t total_time_us;
    uint32_t max_time_us;
} TaskMetrics;

void update_task_metrics(TaskMetrics* m, uint32_t elapsed_us) {
    m->executions++;
    m->total_time_us += elapsed_us;
    if (elapsed_us > m->max_time_us) {
        m->max_time_us = elapsed_us;
    }
}
```

---

## Zadanie 2: Stack Overflow Detection (25 min)

### 2.1 Konfiguracja

```c
// W FreeRTOSConfig.h:
// #define configCHECK_FOR_STACK_OVERFLOW  2  // Max checking

// Stack overflow hook
void vApplicationStackOverflowHook(TaskHandle_t xTask, char* pcTaskName) {
    // CRITICAL ERROR!
    printf("STACK OVERFLOW in task: %s\n", pcTaskName);

    // Loguj
    log_error("Stack overflow", pcTaskName);

    // Reset lub hang dla debug
    while (1) {
        // Zatrzymaj system dla debugowania
    }
}
```

### 2.2 Symulacja stack overflow

```c
void vStackOverflowTask(void *pvParameters) {
    printf("Starting stack overflow task\n");

    // Rekursja - zużywa stack
    void recursive_call(int depth) {
        char buffer[256];  // 256 bajtów na każde wywołanie!

        if (depth > 0) {
            printf("Depth: %d, stack usage ~%d bytes\n",
                   depth, depth * 256);
            recursive_call(depth - 1);
        }
    }

    // To spowoduje overflow przy małym stosie
    recursive_call(20);  // 20 × 256 = 5120 bajtów
}

int main(void) {
    // Mały stack - spowoduje overflow
    xTaskCreate(vStackOverflowTask, "Stack", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 2.3 Stack High Water Mark

```c
void vCheckStackUsage(void) {
    TaskStatus_t* task_list;
    UBaseType_t num_tasks;

    num_tasks = uxTaskGetNumberOfTasks();
    task_list = pvPortMalloc(num_tasks * sizeof(TaskStatus_t));

    if (task_list != NULL) {
        uxTaskGetSystemState(task_list, num_tasks, NULL);

        printf("\n=== Stack Usage ===\n");
        printf("%-12s %8s %8s\n", "Task", "Stack", "HighWater");
        printf("---------------------------------\n");

        for (int i = 0; i < num_tasks; i++) {
            // High water mark = najmniejsza wartość stack pointer
            // Im mniejsza, tym więcej stacku użyto
            printf("%-12s %8u %8u\n",
                   task_list[i].pcTaskName,
                   task_list[i].usStackHighWaterMark,
                   // Jeśli blisko 0 → stack prawie pełny!
                   task_list[i].usStackHighWaterMark);
        }

        vPortFree(task_list);
    }
}
```

---

## Zadanie 3: Assert Hooks (20 min)

### 3.1 Konfiguracja assert

```c
// W FreeRTOSConfig.h:
// #define configASSERT(x) if((x) == 0) vAssertCalled(__FILE__, __LINE__)

void vAssertCalled(const char* file, int line) {
    printf("ASSERT FAILED: %s:%d\n", file, line);

    // Log
    log_assert_failure(file, line);

    // Break dla debugger
    __BKPT(0);

    // Lub reset
    NVIC_SystemReset();
}
```

### 3.2 Malloc failed hook

```c
// W FreeRTOSConfig.h:
// #define configSUPPORT_STATIC_ALLOCATION  0
// #define configSUPPORT_DYNAMIC_ALLOCATION 1
// #define configAPPLICATION_ALLOCATED_HEAP 0

void vApplicationMallocFailedHook(void) {
    printf("MALLOC FAILED!\n");

    // Log memory state
    size_t free_heap = xPortGetFreeHeapSize();
    size_t min_ever = xPortGetMinimumEverFreeHeapSize();

    printf("Free heap: %zu, Min ever: %zu\n", free_heap, min_ever);

    // Reset
    while (1);
}
```

---

## Zadanie 4: Trace i Logging (25 min)

### 4.1 Prosty system logów

```c
#include <stdarg.h>

typedef enum {
    LOG_ERROR,
    LOG_WARNING,
    LOG_INFO,
    LOG_DEBUG
} LogLevel;

typedef struct {
    TickType_t timestamp;
    LogLevel level;
    const char* task_name;
    char message[64];
} LogEntry;

#define LOG_BUFFER_SIZE 100
LogEntry log_buffer[LOG_BUFFER_SIZE];
int log_index = 0;
SemaphoreHandle_t log_mutex;

void log_message(LogLevel level, const char* fmt, ...) {
    if (xSemaphoreTake(log_mutex, pdMS_TO_TICKS(10)) == pdTRUE) {
        LogEntry* entry = &log_buffer[log_index % LOG_BUFFER_SIZE];

        entry->timestamp = xTaskGetTickCount();
        entry->level = level;
        entry->task_name = pcTaskGetName(NULL);

        va_list args;
        va_start(args, fmt);
        vsnprintf(entry->message, sizeof(entry->message), fmt, args);
        va_end(args);

        log_index++;

        xSemaphoreGive(log_mutex);
    }
}

void vPrintLogs(void) {
    for (int i = 0; i < log_index && i < LOG_BUFFER_SIZE; i++) {
        LogEntry* e = &log_buffer[i];
        printf("[%lu] [%s] %s: %s\n",
               e->timestamp,
               e->task_name,
               e->level == LOG_ERROR ? "ERR" : "INF",
               e->message);
    }
}

// Makra dla wygody
#define LOG_E(fmt, ...) log_message(LOG_ERROR, fmt, ##__VA_ARGS__)
#define LOG_W(fmt, ...) log_message(LOG_WARNING, fmt, ##__VA_ARGS__)
#define LOG_I(fmt, ...) log_message(LOG_INFO, fmt, ##__VA_ARGS__)
```

### 4.2 Context switch tracing

```c
// W FreeRTOSConfig.h:
// #define configUSE_TRACE_FACILITY 1
// #define INCLUDE_xTaskGetIdleTaskHandle 1

// Trace buffer
#define TRACE_SIZE 50
typedef struct {
    TickType_t time;
    const char* from_task;
    const char* to_task;
} ContextSwitchTrace;

ContextSwitchTrace trace_buffer[TRACE_SIZE];
int trace_index = 0;

// Hook przy przełączeniu (symulowany)
void trace_context_switch(TaskHandle_t from, TaskHandle_t to) {
    if (trace_index < TRACE_SIZE) {
        trace_buffer[trace_index].time = xTaskGetTickCount();
        trace_buffer[trace_index].from_task = pcTaskGetName(from);
        trace_buffer[trace_index].to_task = pcTaskGetName(to);
        trace_index++;
    }
}

void vPrintTrace(void) {
    printf("\n=== Context Switch Trace ===\n");
    for (int i = 0; i < trace_index; i++) {
        printf("[%lu] %s -> %s\n",
               trace_buffer[i].time,
               trace_buffer[i].from_task,
               trace_buffer[i].to_task);
    }
    trace_index = 0;
}
```

---

## Zadanie 5: Watchdog Integration (20 min)

### 5.1 Hardware watchdog + RTOS

```c
#include "FreeRTOS.h"
#include "task.h"

#define TASK_COUNT 3

typedef struct {
    TaskHandle_t handle;
    const char* name;
    TickType_t last_checkin;
    TickType_t timeout;
    bool alive;
} WatchdogEntry;

WatchdogEntry watchdog_list[TASK_COUNT];
SemaphoreHandle_t watchdog_mutex;

void watchdog_init(void) {
    watchdog_mutex = xSemaphoreCreateMutex();

    for (int i = 0; i < TASK_COUNT; i++) {
        watchdog_list[i].handle = NULL;
        watchdog_list[i].alive = false;
        watchdog_list[i].timeout = pdMS_TO_TICKS(2000);
    }
}

void watchdog_register(TaskHandle_t handle, const char* name) {
    xSemaphoreTake(watchdog_mutex, portMAX_DELAY);

    for (int i = 0; i < TASK_COUNT; i++) {
        if (watchdog_list[i].handle == NULL) {
            watchdog_list[i].handle = handle;
            watchdog_list[i].name = name;
            watchdog_list[i].alive = true;
            watchdog_list[i].last_checkin = xTaskGetTickCount();
            break;
        }
    }

    xSemaphoreGive(watchdog_mutex);
}

void watchdog_checkin(void) {
    TaskHandle_t current = xTaskGetCurrentTaskHandle();

    xSemaphoreTake(watchdog_mutex, portMAX_DELAY);

    for (int i = 0; i < TASK_COUNT; i++) {
        if (watchdog_list[i].handle == current) {
            watchdog_list[i].last_checkin = xTaskGetTickCount();
            watchdog_list[i].alive = true;
            break;
        }
    }

    xSemaphoreGive(watchdog_mutex);
}

void vWatchdogTask(void *pvParameters) {
    while (1) {
        TickType_t now = xTaskGetTickCount();
        bool all_ok = true;

        xSemaphoreTake(watchdog_mutex, portMAX_DELAY);

        for (int i = 0; i < TASK_COUNT; i++) {
            if (watchdog_list[i].handle != NULL) {
                TickType_t elapsed = now - watchdog_list[i].last_checkin;

                if (elapsed > watchdog_list[i].timeout) {
                    printf("WATCHDOG: Task '%s' hung!\n", watchdog_list[i].name);
                    all_ok = false;
                    watchdog_list[i].alive = false;

                    // Action: reset task, reset system, etc.
                }
            }
        }

        xSemaphoreGive(watchdog_mutex);

        if (all_ok) {
            // Feed hardware watchdog
            // WDT_Kick();
        }

        vTaskDelay(pdMS_TO_TICKS(500));
    }
}

// W każdym tasku:
void vMonitoredTask(void *pvParameters) {
    watchdog_register(NULL, "Monitored");

    while (1) {
        do_work();
        watchdog_checkin();  // Zgłoś się
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}
```

---

## Zadanie Bonus: Tracealyzer Integration

Dla studentów z dostępem do Tracealyzer:

```c
// Tracealyzer configuration
#define TRC_CFG_RECORDER_MODE TRC_RECORDER_MODE_START

#include "trcRecorder.h"

void vTask1(void *pvParameters) {
    while (1) {
        // Oznacz punkt w trace
        vTracePrint(0, "Task1 starting work");

        do_work();

        vTracePrint(0, "Task1 done");
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

// Po uruchomieniu:
// 1. Pobierz trace przez JTAG/UART
// 2. Otwórz w Tracealyzer
// 3. Analizuj timeline, CPU usage, blocking
```

---

## Tabela narzędzi debug

| Narzędzie | Opis |
|-----------|------|
| `vTaskList()` | Lista tasków ze stanami |
| `vTaskGetRunTimeStats()` | CPU time per task |
| `uxTaskGetStackHighWaterMark()` | Stack usage |
| `vApplicationStackOverflowHook()` | Stack overflow callback |
| `vApplicationMallocFailedHook()` | Malloc fail callback |
| `configASSERT()` | Custom assert |
| `xPortGetFreeHeapSize()` | Wolna pamięć |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Task statistics | 2 |
| 2 | Stack overflow detection | 2 |
| 3 | Logging system | 2 |
| 4 | Context trace | 2 |
| 5 | Watchdog | 2 |

---

## Sprawozdanie

1. Screenshot task statistics
2. Analiza stack usage dla wszystkich tasków
3. Opis systemu logowania
4. Opis watchdog implementation

---

## Pytania kontrolne

1. Dlaczego printf w ISR jest problemem?
2. Co to jest stack high water mark?
3. Jak wykryć memory leak w RTOS?
4. Co to jest Heisenbug?
5. Jak zintegrować hardware watchdog z RTOS?