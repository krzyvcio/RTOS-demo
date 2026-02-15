# Laboratorium 7: Priority Inversion

**Czas:** 2 godziny
**Punkty:** 15 pkt

---

## Cel ćwiczenia

1. Zrozumienie problemu priority inversion
2. Demonstracja klasycznego przypadku
3. Poznanie rozwiązań: Priority Inheritance, Priority Ceiling
4. Implementacja i testowanie

---

## Teoria (25 min)

### Priority Inversion - Problem

```
Task H (High priority)
Task M (Medium priority)
Task L (Low priority)

Scenariusz:
1. L bierze mutex
2. L wykonuje się (sekcja krytyczna)
3. H becomes ready → preemptuje L
4. H chce mutex → BLOCKED (L ma mutex)
5. M becomes ready → preemptuje L
6. M wykonuje się...
7. L nie może się wykonać → nie może zwolnić mutex
8. H czeka na M! (M ma niższy priorytet!)

To jest Priority Inversion: High czeka na Medium!
```

### Rozwiązania

1. **Priority Inheritance**
   - Gdy H czeka na mutex trzymany przez L
   - L dostaje tymczasowo priorytet H
   - L wykonuje się szybciej, zwalnia mutex
   - H może kontynuować

2. **Priority Ceiling Protocol**
   - Każdy mutex ma "ceiling" = najwyższy priorytet tasków go używających
   - Task biorący mutex dostaje priorytet ceiling
   - Zapobiega inversion i deadlock

---

## Zadanie 1: Demonstracja Priority Inversion (35 min)

### 1.1 Kod demonstracyjny

```c
#include "FreeRTOS.h"
#include "task.h"
#include "semphr.h"
#include <stdio.h>

SemaphoreHandle_t mutex;
TaskHandle_t task_l_handle;
TaskHandle_t task_m_handle;
TaskHandle_t task_h_handle;

void vTaskL(void *pvParameters) {
    printf("[L] Starting, priority %d\n", uxTaskPriorityGet(NULL));

    // Weź mutex
    printf("[L] Taking mutex...\n");
    xSemaphoreTake(mutex, portMAX_DELAY);
    printf("[L] Mutex acquired\n");

    // Symuluj długą sekcję krytyczną
    printf("[L] Working in critical section...\n");
    for (volatile int i = 0; i < 5000000; i++) {
        // Nie yieldujemy - pozwalamy na preemption
    }

    printf("[L] Releasing mutex\n");
    xSemaphoreGive(mutex);

    printf("[L] Done\n");
    vTaskDelete(NULL);
}

void vTaskM(void *pvParameters) {
    printf("[M] Starting, priority %d\n", uxTaskPriorityGet(NULL));

    // Symuluj pracę - blokuje L!
    printf("[M] Working (blocking L)...\n");
    for (volatile int i = 0; i < 3000000; i++) {
        // Tylko praca, bez yield
    }

    printf("[M] Done\n");
    vTaskDelete(NULL);
}

void vTaskH(void *pvParameters) {
    printf("[H] Starting, priority %d\n", uxTaskPriorityGet(NULL));

    // Czekaj chwilę, żeby L zdążył wziąć mutex
    vTaskDelay(pdMS_TO_TICKS(10));

    printf("[H] Trying to take mutex...\n");
    TickType_t start = xTaskGetTickCount();

    xSemaphoreTake(mutex, portMAX_DELAY);

    TickType_t wait_time = xTaskGetTickCount() - start;
    printf("[H] Mutex acquired after %lu ticks\n", wait_time);

    printf("[H] Working\n");
    xSemaphoreGive(mutex);

    printf("[H] Done\n");
    vTaskDelete(NULL);
}

int main(void) {
    // Utwórz ZWYKŁY mutex (bez priority inheritance)
    mutex = xSemaphoreCreateMutex();

    // Taski o różnych priorytetach
    xTaskCreate(vTaskL, "TaskL", 128, NULL, 1, &task_l_handle);  // Low
    xTaskCreate(vTaskM, "TaskM", 128, NULL, 2, &task_m_handle);  // Medium
    xTaskCreate(vTaskH, "TaskH", 128, NULL, 3, &task_h_handle);  // High

    printf("Starting scheduler...\n");
    vTaskStartScheduler();

    return 0;
}
```

### 1.2 Uruchom i obserwuj

```
Starting scheduler...
[L] Starting, priority 1
[L] Taking mutex...
[L] Mutex acquired
[L] Working in critical section...
[H] Starting, priority 3
[H] Trying to take mutex...
[M] Starting, priority 2    ← M preemptuje L!
[M] Working (blocking L)... ← H czeka na M!
[M] Done
[L] Working in critical section...  ← L może kontynuować
[L] Releasing mutex
[H] Mutex acquired after XXX ticks  ← Długo czekał!
```

**TODO:** Zmierz czas oczekiwania H. Ile to ticków?

---

## Zadanie 2: Priority Inheritance (35 min)

### 2.1 Włączenie priority inheritance

```c
// FreeRTOS domyślnie używa priority inheritance dla mutexów
// Upewnij się w FreeRTOSConfig.h:
#define configUSE_MUTEXES                   1
#define configUSE_PRIORITY_INHERITANCE      1  // Jeśli dostępne
```

### 2.2 Implementacja z PI

```c
// W FreeRTOS mutex MA priority inheritance domyślnie!
// Użyj xSemaphoreCreateMutex() i PI jest automatyczne

SemaphoreHandle_t mutex_pi;

void vTaskL_PI(void *pvParameters) {
    printf("[L] Starting, priority %d\n", uxTaskPriorityGet(NULL));

    printf("[L] Taking mutex...\n");
    xSemaphoreTake(mutex_pi, portMAX_DELAY);
    printf("[L] Mutex acquired\n");

    printf("[L] Priority after lock: %d\n", uxTaskPriorityGet(NULL));

    // Symuluj pracę
    printf("[L] Working...\n");
    for (volatile int i = 0; i < 5000000; i++) {
        // W trakcie: H próbuje wziąć mutex
        // L inherits H's priority!
    }

    printf("[L] Priority before unlock: %d\n", uxTaskPriorityGet(NULL));
    printf("[L] Releasing mutex\n");
    xSemaphoreGive(mutex_pi);
    printf("[L] Priority after unlock: %d\n", uxTaskPriorityGet(NULL));

    vTaskDelete(NULL);
}

// Task H i M bez zmian...

int main(void) {
    mutex_pi = xSemaphoreCreateMutex();  // Z PI!

    xTaskCreate(vTaskL_PI, "TaskL", 128, NULL, 1, &task_l_handle);
    xTaskCreate(vTaskM, "TaskM", 128, NULL, 2, &task_m_handle);
    xTaskCreate(vTaskH, "TaskH", 128, NULL, 3, &task_h_handle);

    vTaskStartScheduler();
    return 0;
}
```

### 2.3 Obserwacja z PI

```
[L] Starting, priority 1
[L] Taking mutex...
[L] Mutex acquired
[L] Working...
[H] Starting, priority 3
[H] Trying to take mutex...
[L] Priority before unlock: 3  ← L odziedziczył priorytet H!
[M] Starting, priority 2
[M] Working...                  ← M NIE preemptuje L (bo L ma prio 3!)
[L] Releasing mutex
[H] Mutex acquired after Y ticks  ← Krócej!
```

**CHECK:** Pokaż prowadzącemu różnicę czasów oczekiwania H.

---

## Zadanie 3: Priority Ceiling Protocol (25 min)

### 3.1 Koncept

```
Priority Ceiling = najwyższy priorytet tasków używających mutex

Gdy task bierze mutex:
- Jego priorytet zostaje podniesiony do ceiling
- Natychmiast, nie dopiero gdy ktoś czeka

Zalety:
- Zapobiega inversion (task już ma wysoki priorytet)
- Zapobiega deadlock
- Bardziej przewidywalne
```

### 3.2 Implementacja (symulowana)

```c
// FreeRTOS nie ma natywnego PCP
// Symulujemy ręcznie:

#define MUTEX_CEILING_PRIORITY 3

SemaphoreHandle_t mutex_ceiling;
UBaseType_t original_priority;

void mutex_take_ceiling(SemaphoreHandle_t mutex) {
    // Zapisz oryginalny priorytet
    original_priority = uxTaskPriorityGet(NULL);

    // Podnieś do ceiling
    vTaskPrioritySet(NULL, MUTEX_CEILING_PRIORITY);

    // Weź mutex
    xSemaphoreTake(mutex, portMAX_DELAY);
}

void mutex_give_ceiling(SemaphoreHandle_t mutex) {
    // Zwolnij mutex
    xSemaphoreGive(mutex);

    // Przywróć oryginalny priorytet
    vTaskPrioritySet(NULL, original_priority);
}

void vTaskL_Ceiling(void *pvParameters) {
    printf("[L] Starting, priority %d\n", uxTaskPriorityGet(NULL));

    mutex_take_ceiling(mutex_ceiling);
    printf("[L] Mutex acquired, priority now: %d\n", uxTaskPriorityGet(NULL));

    // Praca...
    for (volatile int i = 0; i < 5000000; i++);

    mutex_give_ceiling(mutex_ceiling);
    printf("[L] Released, priority back to: %d\n", uxTaskPriorityGet(NULL));

    vTaskDelete(NULL);
}
```

### 3.3 Porównanie

| Metoda | Inversion | Deadlock | Overhead |
|--------|-----------|----------|----------|
| Brak | Tak | Możliwy | Brak |
| PI | Nie | Możliwy | Mały |
| PCP | Nie | Nie | Średni |

---

## Zadanie 4: Pomiar czasu inversion (25 min)

### 4.1 Setup pomiarowy

```c
typedef struct {
    TickType_t start_time;
    TickType_t end_time;
    TickType_t wait_time;
} InversionStats;

InversionStats stats;

void vTaskH_Measure(void *pvParameters) {
    vTaskDelay(pdMS_TO_TICKS(10));

    stats.start_time = xTaskGetTickCount();
    xSemaphoreTake(mutex, portMAX_DELAY);
    stats.end_time = xTaskGetTickCount();
    stats.wait_time = stats.end_time - stats.start_time;

    printf("Wait time: %lu ticks\n", stats.wait_time);

    xSemaphoreGive(mutex);
    vTaskDelete(NULL);
}
```

### 4.2 TODO - Porównanie metod

Zmierz i porównaj czasy oczekiwania:

```
| Metoda | Średni wait (ticks) | Max wait (ticks) |
|--------|---------------------|------------------|
| Brak   | ?                   | ?                |
| PI     | ?                   | ?                |
| PCP    | ?                   | ?                |
```

Uruchom każdy test 10 razy i oblicz średnią.

---

## Zadanie Bonus: Mars Pathfinder Case

### Historia

W 1997 Mars Pathfinder miał problem priority inversion:
- Task zbierający dane meteorologiczne (low prio)
- Task komunikacyjny (high prio)
- Task sterujący (medium prio)
- Inversion powodował reset systemu co kilka dni

**Zadanie:** Zrekonstruuj problem Pathfinder i pokaż, jak PI go rozwiązuje.

```c
// Szkic:
// Task meteo: niski priorytet, trzyma mutex na długo
// Task comm: wysoki priorytet, potrzebuje mutex
// Task bus: średni priorytet, CPU-intensive

// Bez PI: comm czeka na bus
// Z PI: meteo dostaje priorytet comm, zwalnia mutex szybko
```

---

## Tabela API

| Funkcja | Opis |
|---------|------|
| `xSemaphoreCreateMutex()` | Mutex z PI |
| `uxTaskPriorityGet()` | Pobierz priorytet |
| `vTaskPrioritySet()` | Ustaw priorytet |
| `xTaskGetTickCount()` | Aktualny tick |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Inversion zademonstrowana | 3 |
| 2 | PI działa i skraca czas | 4 |
| 3 | PCP zaimplementowany | 4 |
| 4 | Pomiary przeprowadzone | 4 |

---

## Sprawozdanie

1. Diagram timeline pokazujący inversion
2. Pomiary czasów oczekiwania (tabela)
3. Wyjaśnienie dlaczego PI działa
4. Porównanie PI vs PCP
5. Wnioski o wyborze metody

---

## Pytania kontrolne

1. Dlaczego priority inversion jest problemem w RTOS?
2. Jak działa priority inheritance?
3. Co to jest priority ceiling?
4. Dlaczego semafor binarny nie ma PI?
5. Kiedy użyć PI, a kiedy PCP?