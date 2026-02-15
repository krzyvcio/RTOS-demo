# Laboratorium 6: Timing, Timery i Delays

**Czas:** 2 godziny
**Punkty:** 10 pkt

---

## Cel ćwiczenia

1. Różne metody opóźnień
2. Timery programowe
3. Periodyczne zadania
4. Pomiar czasu w RTOS

---

## Teoria (15 min)

### Opóźnienia w RTOS

```
vTaskDelay() - Relatywne opóźnienie
"Czekaj X ticków od TERAZ"

vTaskDelayUntil() - Absolutne opóźnienie
"Czekaj do MOMENTU X"
(precyzyjne periodyczne wykonywanie)
```

### Różnica

```
vTaskDelay(100):
    │ Task runs    │ delay │ Task runs    │ delay │
    ├──────────────┼───────┼──────────────┼───────┤
    │              │ 100   │              │ 100   │
    │◄───── Period zależy od czasu wykonania ────►│

vTaskDelayUntil():
    │Task│delay│    │Task│delay│    │Task│delay│
    ├────┼─────┼────┼────┼─────┼────┼────┼─────┤
    │    │     │    │    │     │    │    │     │
    │◄────── Stały period = 100 ticków ──────►│
    │ (niezależnie od czasu wykonania)        │
```

---

## Zadanie 1: vTaskDelay vs vTaskDelayUntil (30 min)

### 1.1 Porównanie praktyczne

```c
#include "FreeRTOS.h"
#include "task.h"
#include <stdio.h>

void vTaskUsingDelay(void *pvParameters) {
    TickType_t start_time;
    int iteration = 0;

    while (1) {
        start_time = xTaskGetTickCount();

        // Symuluj różny czas wykonania
        volatile int work = rand() % 50;  // 0-49 iteracji
        for (volatile int i = 0; i < work * 10000; i++);

        printf("[Delay] Iter %d, work=%d, elapsed=%lu\n",
               iteration++, work, xTaskGetTickCount() - start_time);

        // Relatywne opóźnienie
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vTaskUsingDelayUntil(void *pvParameters) {
    TickType_t last_wake_time = xTaskGetTickCount();
    TickType_t start_time;
    int iteration = 0;

    while (1) {
        start_time = xTaskGetTickCount();

        // Symuluj różny czas wykonania
        volatile int work = rand() % 50;
        for (volatile int i = 0; i < work * 10000; i++);

        printf("[DelayUntil] Iter %d, work=%d, elapsed=%lu\n",
               iteration++, work, xTaskGetTickCount() - start_time);

        // Absolutne opóźnienie - STAŁY PERIOD
        vTaskDelayUntil(&last_wake_time, pdMS_TO_TICKS(100));
    }
}

int main(void) {
    xTaskCreate(vTaskUsingDelay, "Delay", 256, NULL, 1, NULL);
    xTaskCreate(vTaskUsingDelayUntil, "DelayUntil", 256, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 1.2 Obserwacja i analiza

```
[Delay] Iter 0, work=25, elapsed=5       ← vTaskDelay
[DelayUntil] Iter 0, work=30, elapsed=6  ← vTaskDelayUntil

[Delay] Iter 1, work=45, elapsed=9       ← Całkowity period: 100+9=109ms
[DelayUntil] Iter 1, work=40, elapsed=8  ← Całkowity period: ~100ms (stały!)

Wniosek:
- vTaskDelay: period = delay + execution_time (zmienny)
- vTaskDelayUntil: period = stały (niezależnie od execution)
```

---

## Zadanie 2: Software Timers (30 min)

### 2.1 Typy timerów

```c
#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"

// Timer handles
TimerHandle_t one_shot_timer;
TimerHandle_t periodic_timer;
TimerHandle_t auto_reload_timer;

// Callback dla one-shot timer
void vOneShotTimerCallback(TimerHandle_t xTimer) {
    printf("One-shot timer fired!\n");
    // Wykonuje się RAZ, potem timer jest stopped
}

// Callback dla periodic timer
void vPeriodicTimerCallback(TimerHandle_t xTimer) {
    static int count = 0;
    printf("Periodic timer: %d\n", ++count);
    // Wykonuje się CYKLICZNIE
}

// Callback z parametrem
void vParameterizedCallback(TimerHandle_t xTimer) {
    // Pobierz parametr
    int id = (int)pvTimerGetTimerID(xTimer);
    printf("Timer ID=%d fired\n", id);
}

int main(void) {
    // One-shot timer (uruchamia się raz po 2000ms)
    one_shot_timer = xTimerCreate(
        "OneShot",                // Nazwa
        pdMS_TO_TICKS(2000),      // Period (2000ms)
        pdFALSE,                  // Auto-reload = false (one-shot)
        (void*)0,                 // Timer ID
        vOneShotTimerCallback     // Callback
    );

    // Periodic timer (uruchamia się co 500ms)
    periodic_timer = xTimerCreate(
        "Periodic",
        pdMS_TO_TICKS(500),
        pdTRUE,                   // Auto-reload = true (periodic)
        (void*)1,
        vPeriodicTimerCallback
    );

    // Start timers
    xTimerStart(one_shot_timer, 0);
    xTimerStart(periodic_timer, 0);

    vTaskStartScheduler();
    return 0;
}
```

### 2.2 Timer API

```c
// Tworzenie
TimerHandle_t xTimerCreate(name, period, auto_reload, id, callback);

// Start/Stop
BaseType_t xTimerStart(timer, timeout);
BaseType_t xTimerStop(timer, timeout);
BaseType_t xTimerReset(timer, timeout);     // Restart od teraz

// Zmiana parametrów
BaseType_t xTimerChangePeriod(timer, new_period, timeout);

// Stan
BaseType_t xTimerIsTimerActive(timer);
void* pvTimerGetTimerID(timer);
void vTimerSetTimerID(timer, id);

// One-shot z command queue
BaseType_t xTimerPendFunctionCallFromISR(callback, param1, param2, higher_priority_woken);
```

### 2.3 TODO - Stoper z timerem

Zaimplementuj stoper:
- Start/Stop komendy przez UART
- Wyświetlanie czasu co 100ms
- Reset do 0

```c
TimerHandle_t stopwatch_timer;
volatile uint32_t elapsed_ms = 0;
volatile bool running = false;

void vStopwatchCallback(TimerHandle_t xTimer) {
    if (running) {
        elapsed_ms += 100;
        printf("Time: %lu.%03lu s\n", elapsed_ms / 1000, elapsed_ms % 1000);
    }
}

void vCommandTask(void *pvParameters) {
    char command;

    while (1) {
        // Symuluj komendy
        command = getchar();

        switch (command) {
            case 's':  // Start/Stop
                running = !running;
                printf(running ? "Started\n" : "Stopped\n");
                break;
            case 'r':  // Reset
                elapsed_ms = 0;
                printf("Reset\n");
                break;
        }
    }
}
```

---

## Zadanie 3: Periodyczne zadania (25 min)

### 3.1 Wzorzec periodic task

```c
#define TASK_PERIOD_MS 100

void vPeriodicTask(void *pvParameters) {
    TickType_t last_wake_time = xTaskGetTickCount();
    TickType_t execution_time;
    TickType_t slack_time;

    while (1) {
        // Zapisz czas startu
        TickType_t start = xTaskGetTickCount();

        // Wykonaj zadanie
        process_periodic_work();

        // Zmierz czas wykonania
        execution_time = xTaskGetTickCount() - start;

        // Oblicz slack (ile czasu zostało do końca periodu)
        slack_time = pdMS_TO_TICKS(TASK_PERIOD_MS) - execution_time;

        // Loguj timing
        if (slack_time < pdMS_TO_TICKS(10)) {
            printf("WARNING: Low slack! exec=%lu, slack=%lu\n",
                   execution_time, slack_time);
        }

        // Czekaj do następnego periodu
        vTaskDelayUntil(&last_wake_time, pdMS_TO_TICKS(TASK_PERIOD_MS));
    }
}
```

### 3.2 Monitoring deadline

```c
void vDeadlineMonitoredTask(void *pvParameters) {
    TickType_t last_wake_time = xTaskGetTickCount();
    const TickType_t deadline = pdMS_TO_TICKS(50);  // Deadline = 50ms
    uint32_t deadline_misses = 0;
    uint32_t total_executions = 0;

    while (1) {
        TickType_t start = xTaskGetTickCount();

        // Zadanie
        do_work();

        TickType_t elapsed = xTaskGetTickCount() - start;
        total_executions++;

        if (elapsed > deadline) {
            deadline_misses++;
            printf("DEADLINE MISS! took=%lu, deadline=%lu (misses=%u/%u)\n",
                   elapsed, deadline, deadline_misses, total_executions);
        }

        vTaskDelayUntil(&last_wake_time, pdMS_TO_TICKS(100));
    }
}
```

---

## Zadanie 4: Tick Hook (15 min)

### 4.1 Callback przy każdym ticku

```c
// W FreeRTOSConfig.h:
// #define configUSE_TICK_HOOK 1

// Tick hook - wywoływany przy każdym tick interrupt
void vApplicationTickHook(void) {
    static uint32_t tick_counter = 0;

    tick_counter++;

    // Co 1000 ticków
    if (tick_counter % 1000 == 0) {
        // Minimalne operacje! To jest w ISR context!
        flag_1_second_elapsed = true;
    }
}

// Task reagujący na flagę
void vTimekeeperTask(void *pvParameters) {
    uint32_t seconds = 0;

    while (1) {
        if (flag_1_second_elapsed) {
            flag_1_second_elapsed = false;
            seconds++;
            printf("Uptime: %lu seconds\n", seconds);
        }

        vTaskDelay(pdMS_TO_TICKS(10));
    }
}
```

---

## Zadanie 5: Pomiar czasu wykonania (20 min)

### 5.1 Cycle counter (ARM Cortex-M)

```c
// Inicjalizacja
void DWT_Init(void) {
    CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;
    DWT->CYCCNT = 0;
    DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;
}

// Pobierz licznik cykli
static inline uint32_t DWT_GetCycles(void) {
    return DWT->CYCCNT;
}

// Pobierz czas w mikrosekundach
static inline uint32_t DWT_GetUs(void) {
    return DWT_GetCycles() / (SystemCoreClock / 1000000);
}

// Użycie
void measure_execution_time(void) {
    uint32_t start = DWT_GetCycles();

    // Kod do zmierzenia
    process_data();

    uint32_t end = DWT_GetCycles();
    uint32_t cycles = end - start;
    uint32_t us = cycles / (SystemCoreClock / 1000000);

    printf("Execution: %lu cycles, %lu us\n", cycles, us);
}
```

### 5.2 Statystyki wykonania

```c
typedef struct {
    uint32_t min_cycles;
    uint32_t max_cycles;
    uint32_t total_cycles;
    uint32_t count;
} ExecutionStats;

void update_stats(ExecutionStats* stats, uint32_t cycles) {
    if (stats->count == 0) {
        stats->min_cycles = cycles;
        stats->max_cycles = cycles;
    } else {
        if (cycles < stats->min_cycles) stats->min_cycles = cycles;
        if (cycles > stats->max_cycles) stats->max_cycles = cycles;
    }
    stats->total_cycles += cycles;
    stats->count++;
}

void print_stats(const char* name, ExecutionStats* stats) {
    printf("%s: min=%lu, max=%lu, avg=%lu cycles\n",
           name,
           stats->min_cycles,
           stats->max_cycles,
           stats->total_cycles / stats->count);
}
```

---

## Zadanie Bonus: High-Resolution Timer

```c
// Dla potrzeb high-resolution (> tick rate)
// Użyj dodatkowego timer hardware

void TIM2_IRQHandler(void) {
    if (TIM2->SR & TIM_SR_UIF) {
        TIM2->SR &= ~TIM_SR_UIF;

        // High-resolution callback (np. co 10us)
        high_res_timer_callback();
    }
}

void setup_high_res_timer(uint32_t period_us) {
    // Enable clock
    RCC->APB1ENR |= RCC_APB1ENR_TIM2EN;

    // Konfiguracja
    TIM2->PSC = SystemCoreClock / 1000000 - 1;  // 1MHz count
    TIM2->ARR = period_us - 1;                   // Period

    TIM2->DIER |= TIM_DIER_UIE;  // Enable interrupt
    NVIC_EnableIRQ(TIM2_IRQn);
    TIM2->CR1 |= TIM_CR1_CEN;    // Start
}
```

---

## Tabela API

| Funkcja | Opis |
|---------|------|
| `vTaskDelay(ticks)` | Relatywne opóźnienie |
| `vTaskDelayUntil(&time, ticks)` | Absolutne opóźnienie |
| `xTaskGetTickCount()` | Aktualny tick count |
| `pdMS_TO_TICKS(ms)` | Konwersja ms → ticks |
| `xTimerCreate()` | Utwórz timer |
| `xTimerStart/Stop/Reset()` | Sterowanie timerem |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Porównanie delay vs delayUntil | 2 |
| 2 | Software timer działa | 2 |
| 3 | Periodyczne zadanie | 2 |
| 4 | Tick hook | 2 |
| 5 | Pomiar czasu | 2 |

---

## Sprawozdanie

1. Wykres porównujący period dla obu metod delay
2. Statystyki wykonania periodycznego zadania
3. Implementacja stopera
4. Wnioski o wyborze metody opóźnienia

---

## Pytania kontrolne

1. Kiedy użyć `vTaskDelay` a kiedy `vTaskDelayUntil`?
2. Co to jest auto-reload timer?
3. Dlaczego tick hook musi być krótki?
4. Jak mierzyć czas wykonania kodu?
5. Co to jest slack time w periodycznym zadaniu?