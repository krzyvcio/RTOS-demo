# Laboratorium 3: Synchronizacja - Mutexy i Semafor

**Czas:** 2 godziny
**Punkty:** 20 pkt

---

## Cel ćwiczenia

1. Zrozumienie problemu race condition
2. Użycie mutexów do ochrony zasobów
3. Użycie semaforów do sygnalizacji
4. Implementacja producer-consumer

---

## Teoria (20 min)

### Race Condition

**Problem:** Dwa zadania próbują modyfikować te same dane jednocześnie.

```c
// Bez synchronizacji
int counter = 0;

void task_a(void) {
    counter++;  // Nieatomicowe! LOAD → INC → STORE
}

void task_b(void) {
    counter++;  // Może nadpisać!
}
```

**Rozwiązanie:** Mutex - tylko jedno zadanie na raz w sekcji krytycznej.

### Mutex vs Semafor

| Cecha | Mutex | Semafor |
|-------|-------|---------|
| Własność | Tak (właściciel) | Nie |
| Sygnalizacja | Nie | Tak |
| ISR-safe | Nie | Tak (binary) |
| Priorytety | Priority inheritance | Brak |

---

## Zadanie 1: Race Condition - Demonstracja (25 min)

### 1.1 Kod bez synchronizacji

```c
#include "FreeRTOS.h"
#include "task.h"
#include "semphr.h"
#include <stdio.h>

volatile int shared_counter = 0;

void vTaskA(void *pvParameters) {
    for (int i = 0; i < 1000; i++) {
        shared_counter++;
    }
    vTaskDelete(NULL);
}

void vTaskB(void *pvParameters) {
    for (int i = 0; i < 1000; i++) {
        shared_counter++;
    }
    vTaskDelete(NULL);
}

int main(void) {
    xTaskCreate(vTaskA, "TaskA", 128, NULL, 1, NULL);
    xTaskCreate(vTaskB, "TaskB", 128, NULL, 1, NULL);

    vTaskStartScheduler();

    printf("Final counter: %d (expected: 2000)\n", shared_counter);
    return 0;
}
```

### 1.2 Uruchom i obserwuj

```bash
make && ./main
# Final counter: 1587 (expected: 2000)  ← Race condition!
```

**TODO:** Uruchom 10 razy i zapisz wyniki. Czy kiedykolwiek jest 2000?

### 1.3 Analiza

```
Operacja shared_counter++ składa się z 3 kroków:
1. LOAD:  tmp = shared_counter
2. INC:   tmp = tmp + 1
3. STORE: shared_counter = tmp

Możliwy interleaving:
Task A: LOAD (tmp=0)
Task B: LOAD (tmp=0)  ← Przejęcie w środku!
Task A: INC (tmp=1)
Task A: STORE (counter=1)
Task B: INC (tmp=1)
Task B: STORE (counter=1)  ← Nadpisanie!
```

---

## Zadanie 2: Mutex - Rozwiązanie (25 min)

### 2.1 Implementacja z mutexem

```c
SemaphoreHandle_t mutex;
volatile int shared_counter = 0;

void vTaskA(void *pvParameters) {
    for (int i = 0; i < 1000; i++) {
        // Lock mutex
        if (xSemaphoreTake(mutex, portMAX_DELAY) == pdTRUE) {
            shared_counter++;  // Sekcja krytyczna
            xSemaphoreGive(mutex);  // Unlock
        }
    }
    vTaskDelete(NULL);
}

void vTaskB(void *pvParameters) {
    for (int i = 0; i < 1000; i++) {
        if (xSemaphoreTake(mutex, portMAX_DELAY) == pdTRUE) {
            shared_counter++;
            xSemaphoreGive(mutex);
        }
    }
    vTaskDelete(NULL);
}

int main(void) {
    // Utwórz mutex
    mutex = xSemaphoreCreateMutex();

    xTaskCreate(vTaskA, "TaskA", 128, NULL, 1, NULL);
    xTaskCreate(vTaskB, "TaskB", 128, NULL, 1, NULL);

    vTaskStartScheduler();

    printf("Final counter: %d\n", shared_counter);
    vSemaphoreDelete(mutex);
    return 0;
}
```

### 2.2 Uruchom i weryfikuj

```bash
make && ./main
# Final counter: 2000  ✓ Zawsze poprawny!
```

**CHECK:** Pokaż prowadzącemu działający kod z mutexem.

### 2.3 TODO - Timeout na mutexie

```c
// Zamiast czekać wiecznie, użyj timeout
if (xSemaphoreTake(mutex, pdMS_TO_TICKS(100)) == pdTRUE) {
    // Sekcja krytyczna
    xSemaphoreGive(mutex);
} else {
    printf("Mutex timeout!\n");
    // Obsłuż błąd
}
```

---

## Zadanie 3: Semafor binarny - Sygnalizacja (25 min)

### 3.1 ISR → Task sygnalizacja

```c
SemaphoreHandle_t data_ready_sem;

// Symulowany ISR (w rzeczywistości hardware interrupt)
void vSimulatedISR(void *pvParameters) {
    while (1) {
        // Symuluj przerwanie co 500ms
        vTaskDelay(pdMS_TO_TICKS(500));

        printf("ISR: Data available\n");
        xSemaphoreGive(data_ready_sem);  // Signal
    }
}

// Task przetwarzający dane
void vConsumerTask(void *pvParameters) {
    while (1) {
        // Czekaj na sygnał
        if (xSemaphoreTake(data_ready_sem, portMAX_DELAY) == pdTRUE) {
            printf("Consumer: Processing data\n");
            // Przetwórz dane
        }
    }
}

int main(void) {
    data_ready_sem = xSemaphoreCreateBinary();

    xTaskCreate(vSimulatedISR, "ISR", 128, NULL, 2, NULL);
    xTaskCreate(vConsumerTask, "Consumer", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 3.2 TODO - Multiple signalów

Zmień kod tak, aby ISR mógł zgromadzić wiele sygnałów (counting semaphore):

```c
// Zamiast binary, użyj counting
SemaphoreHandle_t data_sem = xSemaphoreCreateCounting(10, 0);

// ISR może dać wiele signal
for (int i = 0; i < 3; i++) {
    xSemaphoreGive(data_sem);  // count = 3
}

// Consumer przetworzy 3 porcje
```

---

## Zadanie 4: Producer-Consumer (30 min)

### 4.1 Problem klasyczny

```
Producer → [Buffer] → Consumer

Producer produkuje dane
Consumer konsumuje dane
Buffer ma ograniczoną wielkość

Wyzwania:
- Synchronizacja bufora
- Sygnalizacja danych gotowych
- Sygnalizacja miejsca wolnego
```

### 4.2 Implementacja

```c
#define BUFFER_SIZE 5

int buffer[BUFFER_SIZE];
int buffer_head = 0;
int buffer_tail = 0;
int buffer_count = 0;

SemaphoreHandle_t mutex;
SemaphoreHandle_t empty_slots;
SemaphoreHandle_t full_slots;

void vProducer(void *pvParameters) {
    int item = 0;

    while (1) {
        // Wyprodukuj element
        item++;
        printf("Producing: %d\n", item);

        // Czekaj na wolne miejsce
        xSemaphoreTake(empty_slots, portMAX_DELAY);

        // Ochrona bufora
        xSemaphoreTake(mutex, portMAX_DELAY);

        // Dodaj do bufora
        buffer[buffer_head] = item;
        buffer_head = (buffer_head + 1) % BUFFER_SIZE;
        buffer_count++;

        printf("Buffer: [%d items]\n", buffer_count);

        xSemaphoreGive(mutex);

        // Zasygnalizuj nowy element
        xSemaphoreGive(full_slots);

        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vConsumer(void *pvParameters) {
    while (1) {
        // Czekaj na element
        xSemaphoreTake(full_slots, portMAX_DELAY);

        // Ochrona bufora
        xSemaphoreTake(mutex, portMAX_DELAY);

        // Pobierz z bufora
        int item = buffer[buffer_tail];
        buffer_tail = (buffer_tail + 1) % BUFFER_SIZE;
        buffer_count--;

        printf("Consuming: %d\n", item);
        printf("Buffer: [%d items]\n", buffer_count);

        xSemaphoreGive(mutex);

        // Zasygnalizuj wolne miejsce
        xSemaphoreGive(empty_slots);

        vTaskDelay(pdMS_TO_TICKS(150));
    }
}

int main(void) {
    mutex = xSemaphoreCreateMutex();
    empty_slots = xSemaphoreCreateCounting(BUFFER_SIZE, BUFFER_SIZE);
    full_slots = xSemaphoreCreateCounting(BUFFER_SIZE, 0);

    xTaskCreate(vProducer, "Producer", 128, NULL, 1, NULL);
    xTaskCreate(vConsumer, "Consumer", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 4.3 Obserwacja

```
Producing: 1
Buffer: [1 items]
Consuming: 1
Buffer: [0 items]
Producing: 2
Buffer: [1 items]
Producing: 3
Buffer: [2 items]
...
Producing: 6
Buffer: [5 items]  ← Pełny!
... (producer czeka na wolne miejsce)
Consuming: 1
Buffer: [4 items]
Producing: 6  ← Teraz może dodać
```

---

## Zadanie 5: Deadlock - Demonstracja i unikanie (20 min)

### 5.1 Kod z deadlock

```c
SemaphoreHandle_t mutex_a;
SemaphoreHandle_t mutex_b;

void vTask1(void *pvParameters) {
    while (1) {
        printf("Task1: Taking mutex A\n");
        xSemaphoreTake(mutex_a, portMAX_DELAY);

        vTaskDelay(pdMS_TO_TICKS(100));  // Symuluj pracę

        printf("Task1: Taking mutex B\n");
        xSemaphoreTake(mutex_b, portMAX_DELAY);  // Może zablokować!

        // Sekcja krytyczna
        printf("Task1: In critical section\n");

        xSemaphoreGive(mutex_b);
        xSemaphoreGive(mutex_a);

        vTaskDelay(pdMS_TO_TICKS(500));
    }
}

void vTask2(void *pvParameters) {
    while (1) {
        printf("Task2: Taking mutex B\n");
        xSemaphoreTake(mutex_b, portMAX_DELAY);

        vTaskDelay(pdMS_TO_TICKS(100));

        printf("Task2: Taking mutex A\n");
        xSemaphoreTake(mutex_a, portMAX_DELAY);  // Może zablokować!

        printf("Task2: In critical section\n");

        xSemaphoreGive(mutex_a);
        xSemaphoreGive(mutex_b);

        vTaskDelay(pdMS_TO_TICKS(500));
    }
}

int main(void) {
    mutex_a = xSemaphoreCreateMutex();
    mutex_b = xSemaphoreCreateMutex();

    xTaskCreate(vTask1, "Task1", 128, NULL, 1, NULL);
    xTaskCreate(vTask2, "Task2", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 5.2 Obserwacja deadlock

```
Task1: Taking mutex A
Task2: Taking mutex B
Task1: Taking mutex B  ← Czeka (Task2 ma B)
Task2: Taking mutex A  ← Czeka (Task1 ma A)
DEADLOCK! Oba czekają na siebie.
```

### 5.3 TODO - Rozwiązanie

Napraw kod, używając jednej z technik:

1. **Kolejność blokowania** - zawsze A potem B
2. **Timeout** - jeśli nie uda się w czasie, zwolnij
3. **Jeden mutex** - dla całego zasobu

```c
// Rozwiązanie 1: Kolejność
void vTask1(void *pvParameters) {
    xSemaphoreTake(mutex_a, portMAX_DELAY);  // A first
    xSemaphoreTake(mutex_b, portMAX_DELAY);  // B second
    // ...
}

void vTask2(void *pvParameters) {
    xSemaphoreTake(mutex_a, portMAX_DELAY);  // A first (TAKE SAMO!)
    xSemaphoreTake(mutex_b, portMAX_DELAY);  // B second
    // ...
}
```

---

## Zadanie Bonus: Dining Philosophers

Zaimplementuj problem 5 filozofów:

```
5 filozofów siedzi przy stole
5 widelców między nimi
Każdy filozof: myśleć → jeść → myśleć
Do jedzenia potrzebne 2 widelce

Problem: Jak uniknąć deadlock?
(Gdy każdy weźmie lewy widelec i czeka na prawy)
```

```c
SemaphoreHandle_t forks[5];

void vPhilosopher(void *pvParameters) {
    int id = (int)pvParameters;
    int left_fork = id;
    int right_fork = (id + 1) % 5;

    while (1) {
        printf("Philosopher %d thinking\n", id);
        vTaskDelay(pdMS_TO_TICKS(random()));

        // Weź widelce
        // TODO: Jak uniknąć deadlock?

        printf("Philosopher %d eating\n", id);
        vTaskDelay(pdMS_TO_TICKS(random()));

        // Odłóż widelce
    }
}
```

**HINT:** Rozwiązanie - numeruj widelce i zawsze bierz najpierw niższy numer.

---

## Tabela API

| Funkcja | Opis |
|---------|------|
| `xSemaphoreCreateMutex()` | Utwórz mutex |
| `xSemaphoreCreateBinary()` | Utwórz semafor binarny |
| `xSemaphoreCreateCounting(max, init)` | Utwórz semafor counting |
| `xSemaphoreTake(sem, timeout)` | Wait (P) |
| `xSemaphoreGive(sem)` | Signal (V) |
| `xSemaphoreGiveFromISR(sem, pxHigher)` | Signal z ISR |
| `vSemaphoreDelete(sem)` | Usuń semafor |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Race condition zademonstrowana | 4 |
| 2 | Mutex naprawia problem | 4 |
| 3 | Semafor binarny działa | 4 |
| 4 | Producer-consumer działa | 4 |
| 5 | Deadlock zrozumiany i naprawiony | 4 |

---

## Sprawozdanie

1. Wyniki eksperymentu race condition (10 uruchomień)
2. Wyjaśnienie dlaczego mutex rozwiązuje problem
3. Diagram producer-consumer
4. Analiza deadlock i rozwiązanie
5. Wnioski: kiedy mutex, kiedy semafor

---

## Pytania kontrolne

1. Co to jest sekcja krytyczna?
2. Dlaczego `volatile` nie wystarcza dla synchronizacji?
3. Kiedy użyć mutex, a kiedy semafor?
4. Co to jest deadlock i jak go uniknąć?
5. Dlaczego mutex nie może być używany w ISR?