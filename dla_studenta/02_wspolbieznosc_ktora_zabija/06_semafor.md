# Semafor

## Definicja

**Semafor** to mechanizm synchronizacji zliczający - zmienna całkowita z dwiema operacjami: wait (P) i signal (V). Może być używany do sygnalizacji zdarzeń lub kontroli dostępu do ograniczonej liczby zasobów.

> Semafor to "licznik miejsc": "Mamy 3 miejsca parkingowe. Ktoś wjeżdża - 2 miejsca. Ktoś wyjeżdża - 3 miejsca. Gdy 0 - czekaj."

```
┌─────────────────────────────────────────────────────────┐
│                  SEMAPHORE                              │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Counting Semaphore (count = 3):                        │
│                                                         │
│  Initial: [●][●][●]  count = 3                          │
│                                                         │
│  Task A wait: [●][●][ ]  count = 2                      │
│  Task B wait: [●][ ][ ]  count = 1                      │
│  Task C wait: [ ][ ][ ]  count = 0                      │
│  Task D wait: BLOCKED (czeka)                           │
│                                                         │
│  Task A signal: [●][ ][ ]  count = 1                    │
│  Task D unblocks: [●][ ][●]  Task D teraz ma zasób     │
│                                                         │
│  Binary Semaphore (count = 0 or 1):                     │
│                                                         │
│  Initial: [ ]  count = 0                                │
│  Task A wait: BLOCKED (czeka na signal)                 │
│  Task B signal: [●]  count = 1                          │
│  Task A unblocks: [ ]  count = 0                        │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🅿️ Parking

To klasyczna analogia semafora counting:

```
Parking = zasób
Miejsca parkingowe = semafor count

Początek: 5 wolnych miejsc (count = 5)

Samochód wjeżdża: wait(semaphore)
- count = 4
- Samochód zaparkował

Samochód wjeżdża: wait(semaphore)
- count = 3

...

Samochód wjeżdża: wait(semaphore)
- count = 0
- Parking pełny

Samochód próbuje wjechać: wait(semaphore)
- count = 0
- BLOCKED (czeka)

Samochód wyjeżdża: signal(semaphore)
- count = 1
- Czekający samochód wjeżdża

Semafor = licznik miejsc parkingowych
```

### 🎫 Bilety do muzeum

```
Muzeum może przyjąć max 100 osób.
Bilety = semafor count = 100

Osoba wchodzi: wait(semaphore)
- count--
- Gdy count = 0: "Przepraszamy, muzeum pełne"

Osoba wychodzi: signal(semaphore)
- count++
- Kolejna osoba może wejść

Semafor = licznik miejsc w muzeum
```

### 🏊 Basen z ograniczoną liczbą szafek

```
Basen ma 50 szafek.
Semafor count = 50

Osoba bierze szafkę: wait(semaphore)
- count--
- Gdy count = 0: brak szafek, czekaj

Osoba oddaje szafkę: signal(semaphore)
- count++
- Ktoś może wziąć szafkę

Semafor = licznik dostępnych szafek
```

---

## Podobieństwo do systemów informatycznych

### Thread Pool

```java
// Semafor ograniczający liczbę wątków
Semaphore threadPoolSem = new Semaphore(10);

void processRequest(Request r) {
    threadPoolSem.acquire();  // wait
    try {
        process(r);
    } finally {
        threadPoolSem.release();  // signal
    }
}
// Max 10 requestów processed concurrently
```

### Connection Pool

```python
# Semafor dla połączeń z bazą
connection_pool_sem = Semaphore(5)  # Max 5 connections

def get_connection():
    connection_pool_sem.acquire()
    return create_connection()

def release_connection(conn):
    close_connection(conn)
    connection_pool_sem.release()
```

### Rate Limiting

```javascript
// Semafor dla rate limiting
const rateLimitSem = new Semaphore(10);  // Max 10 concurrent requests

async function makeRequest(url) {
    await rateLimitSem.acquire();
    try {
        return await fetch(url);
    } finally {
        rateLimitSem.release();
    }
}
```

---

## Rodzaje semaforów

### Binary Semaphore

```c
// Tylko 0 lub 1
SemaphoreHandle_t bin_sem = xSemaphoreCreateBinary();

// Używany do sygnalizacji zdarzeń
// Początkowo 0 (brak zdarzenia)

// Producent:
void producer(void) {
    produce_data();
    xSemaphoreGive(bin_sem);  // Signal: dane gotowe
}

// Konsument:
void consumer(void) {
    xSemaphoreTake(bin_sem, portMAX_DELAY);  // Wait na dane
    consume_data();
}
```

### Counting Semaphore

```c
// Może mieć dowolną wartość nieujemną
SemaphoreHandle_t count_sem = xSemaphoreCreateCounting(10, 0);

// Używany do:
// - Liczenia zdarzeń
// - Pool zasobów
// - Multiple producer/consumer

// Producer (może dać wiele signal):
void producer(void) {
    for (int i = 0; i < 5; i++) {
        produce_item();
        xSemaphoreGive(count_sem);  // count++
    }
}

// Consumer (bierze tyle ile jest):
void consumer(void) {
    while (xSemaphoreTake(count_sem, portMAX_DELAY)) {
        consume_item();
    }
}
```

---

## Operacje na semaforze

### Wait (P, Take, Acquire)

```c
// FreeRTOS
xSemaphoreTake(sem, portMAX_DELAY);

// Zasada działania:
// 1. Jeśli count > 0:
//    - count--
//    - return success
// 2. Jeśli count = 0:
//    - BLOCK task
//    - Dodaj task do waiting queue
//    - Switch do innego tasku
```

### Signal (V, Give, Release)

```c
// FreeRTOS
xSemaphoreGive(sem);

// Zasada działania:
// 1. Jeśli waiting queue nie jest pusty:
//    - Wybudź jeden task
//    - Task przejmuje semafor
// 2. Jeśli waiting queue jest pusty:
//    - count++
```

---

## Semafor w RTOS

### FreeRTOS

```c
// Binary semaphore
SemaphoreHandle_t bin_sem = xSemaphoreCreateBinary();

// Counting semaphore (max count = 10, initial count = 0)
SemaphoreHandle_t count_sem = xSemaphoreCreateCounting(10, 0);

// Wait
xSemaphoreTake(sem, pdMS_TO_TICKS(100));  // Timeout 100ms
xSemaphoreTake(sem, portMAX_DELAY);        // Wait forever

// Signal
xSemaphoreGive(sem);

// Signal z ISR
BaseType_t higher_priority_woken = pdFALSE;
xSemaphoreGiveFromISR(sem, &higher_priority_woken);
portYIELD_FROM_ISR(higher_priority_woken);
```

### Zephyr

```c
// Binary semaphore
K_SEM_DEFINE(my_sem, 0, 1);  // initial=0, max=1

// Counting semaphore
K_SEM_DEFINE(my_sem, 0, 10);  // initial=0, max=10

// Wait
k_sem_take(&my_sem, K_FOREVER);
k_sem_take(&my_sem, K_MSEC(100));  // Timeout 100ms

// Signal
k_sem_give(&my_sem);
```

---

## Zastosowania semaforów

### 1. Sygnalizacja zdarzeń (Binary Semaphore)

```c
SemaphoreHandle_t data_ready = xSemaphoreCreateBinary();

// ISR: produkuje dane
void UART_IRQHandler(void) {
    buffer[index++] = UART->DATA;
    if (index == BUFFER_SIZE) {
        xSemaphoreGiveFromISR(data_ready, NULL);
    }
}

// Task: konsumuje dane
void data_processor(void) {
    while (1) {
        xSemaphoreTake(data_ready, portMAX_DELAY);
        process_buffer(buffer);
        index = 0;
    }
}
```

### 2. Resource Pool (Counting Semaphore)

```c
#define POOL_SIZE 5
typedef struct { /* ... */ } Resource;

Resource resource_pool[POOL_SIZE];
SemaphoreHandle_t pool_sem;

void init_pool(void) {
    pool_sem = xSemaphoreCreateCounting(POOL_SIZE, POOL_SIZE);
    // Wszystkie zasoby dostępne
}

Resource* acquire_resource(void) {
    xSemaphoreTake(pool_sem, portMAX_DELAY);
    return find_free_resource();
}

void release_resource(Resource* r) {
    mark_as_free(r);
    xSemaphoreGive(pool_sem);
}
```

### 3. Multiple Producer / Single Consumer

```c
SemaphoreHandle_t items_sem = xSemaphoreCreateCounting(100, 0);
QueueHandle_t queue = xQueueCreate(100, sizeof(Data));

// Producer (wiele tasków)
void producer(void) {
    Data data = produce();
    xQueueSend(queue, &data, portMAX_DELAY);
    xSemaphoreGive(items_sem);
}

// Consumer (jeden task)
void consumer(void) {
    while (1) {
        xSemaphoreTake(items_sem, portMAX_DELAY);
        Data data;
        xQueueReceive(queue, &data, portMAX_DELAY);
        process(data);
    }
}
```

### 4. Barrier Synchronization

```c
#define NUM_TASKS 4
SemaphoreHandle_t barrier_sem;
volatile int barrier_count = 0;
SemaphoreHandle_t count_mutex;

void init_barrier(void) {
    barrier_sem = xSemaphoreCreateBinary();
    count_mutex = xSemaphoreCreateMutex();
}

void barrier_wait(void) {
    xSemaphoreTake(count_mutex, portMAX_DELAY);
    barrier_count++;

    if (barrier_count == NUM_TASKS) {
        // Ostatni task - zwalnia wszystkich
        barrier_count = 0;
        xSemaphoreGive(count_mutex);
        for (int i = 0; i < NUM_TASKS - 1; i++) {
            xSemaphoreGive(barrier_sem);
        }
    } else {
        // Czekaj na resztę
        xSemaphoreGive(count_mutex);
        xSemaphoreTake(barrier_sem, portMAX_DELAY);
    }
}
```

---

## Semafor vs Mutex

```
┌─────────────────────────────────────────────────────────┐
│           SEMAPHORE vs MUTEX                            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  SEMAPHORE:                                             │
│  ✓ Sygnalizacja zdarzeń                                │
│  ✓ Liczenie zasobów                                    │
│  ✓ ISR-safe (binary)                                   │
│  ✓ Brak właściciela                                    │
│  ✓ Ktokolwiek może signal                              │
│                                                         │
│  MUTEX:                                                 │
│  ✓ Ochrona zasobu (exclusive access)                   │
│  ✓ Priority inheritance                                │
│  ✓ Właściciel (ownership)                              │
│  ✓ Recursive lock                                      │
│  ✗ Nie w ISR                                           │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Kiedy używać czego?

```
Użyj SEMAPHORE gdy:
- Sygnalizacja między ISR a task
- Multiple producer/consumer
- Resource pool
- Counting events
- Nie potrzebujesz ownership

Użyj MUTEX gdy:
- Ochrona shared data
- Priority inheritance potrzebne
- Recursive lock potrzebne
- Ownership ma znaczenie
```

---

## Pułapki semaforów

### 1. Zapomniane signal

```c
// ŹLE: Zapomniane signal
void producer(void) {
    produce_data();
    // xSemaphoreGive(sem);  ← ZAPOMNIANE!
}

void consumer(void) {
    xSemaphoreTake(sem, portMAX_DELAY);  // Czeka na zawsze!
    consume_data();
}
```

### 2. Nadmiarowe signal

```c
// ŹLE: Nadmiarowe signal
SemaphoreHandle_t sem = xSemaphoreCreateBinary();  // count = 0

xSemaphoreGive(sem);  // count = 1
xSemaphoreGive(sem);  // count = 1 (binary - nie zwiększa się)

// A jeśli counting:
SemaphoreHandle_t sem = xSemaphoreCreateCounting(5, 0);
xSemaphoreGive(sem);  // count = 1
xSemaphoreGive(sem);  // count = 2
// Zbyt wiele signal = zła liczba
```

### 3. Timeout handling

```c
// ŹLE: Brak obsługi timeout
xSemaphoreTake(sem, portMAX_DELAY);
// Jeśli signal nigdy nie przyjdzie = deadlock

// DOBRZE: Timeout z obsługą
if (xSemaphoreTake(sem, pdMS_TO_TICKS(1000)) == pdTRUE) {
    // OK, got signal
} else {
    // Timeout - handle error
    log_error("Semaphore timeout");
    recovery();
}
```

---

## Producer-Consumer z semaforami

```c
#define BUFFER_SIZE 10

typedef struct {
    int buffer[BUFFER_SIZE];
    int head;
    int tail;
} CircularBuffer;

CircularBuffer cb;
SemaphoreHandle_t mutex;      // Ochrona bufora
SemaphoreHandle_t empty_sem;  // Licznik pustych miejsc
SemaphoreHandle_t full_sem;   // Licznik pełnych miejsc

void init(void) {
    mutex = xSemaphoreCreateMutex();
    empty_sem = xSemaphoreCreateCounting(BUFFER_SIZE, BUFFER_SIZE);
    full_sem = xSemaphoreCreateCounting(BUFFER_SIZE, 0);
    cb.head = 0;
    cb.tail = 0;
}

void producer(int item) {
    // Czekaj na puste miejsce
    xSemaphoreTake(empty_sem, portMAX_DELAY);

    // Ochrona bufora
    xSemaphoreTake(mutex, portMAX_DELAY);
    cb.buffer[cb.head] = item;
    cb.head = (cb.head + 1) % BUFFER_SIZE;
    xSemaphoreGive(mutex);

    // Zasygnalizuj pełne miejsce
    xSemaphoreGive(full_sem);
}

int consumer(void) {
    // Czekaj na pełne miejsce
    xSemaphoreTake(full_sem, portMAX_DELAY);

    // Ochrona bufora
    xSemaphoreTake(mutex, portMAX_DELAY);
    int item = cb.buffer[cb.tail];
    cb.tail = (cb.tail + 1) % BUFFER_SIZE;
    xSemaphoreGive(mutex);

    // Zasygnalizuj puste miejsce
    xSemaphoreGive(empty_sem);

    return item;
}
```

---

## Pytania do przemyślenia

1. Używasz semaforów czy mutexów? W jakich sytuacjach?
2. Czy masz potencjalne problemy z sygnalizacją (zapomniane/nadmiarowe signal)?
3. Jak obsługujesz timeout na semaforach?

---

## Quiz

**Pytanie**: Masz system z 3 taskami produkującymi dane i 1 taskiem konsumującym. Jak użyć semaforów do synchronizacji?

**Odpowiedź**:

```c
// Counting semaphore do liczenia gotowych elementów
SemaphoreHandle_t items_ready = xSemaphoreCreateCounting(100, 0);

// Queue do przechowywania danych
QueueHandle_t data_queue = xQueueCreate(100, sizeof(Data));

// Producer (3 taski)
void producer(void) {
    while (1) {
        Data data = produce();
        xQueueSend(data_queue, &data, portMAX_DELAY);
        xSemaphoreGive(items_ready);  // Signal: nowy element
    }
}

// Consumer (1 task)
void consumer(void) {
    while (1) {
        xSemaphoreTake(items_ready, portMAX_DELAY);  // Wait na element
        Data data;
        xQueueReceive(data_queue, &data, portMAX_DELAY);
        process(data);
    }
}

// Alternatywnie: Queue już ma wewnętrzną synchronizację
// xQueueSend blokuje gdy pełna, xQueueReceive blokuje gdy pusta
// Semafor może być zbędny w tym prostym przypadku
```

---

## Wskazówka zapamiętywania

> **Semafor = Licznik biletów**
>
> Wyobraź sobie muzeum z 50 biletami:
>
> count = 50: "Witamy, proszę wejść"
> count = 0: "Przepraszamy, pełne, proszę czekać"
>
> Wejście = wait(semaphore)
> Wyjście = signal(semaphore)
>
> Binary semaphore = muzeum tylko dla 1 osoby na raz
> Counting semaphore = muzeum dla max N osób
>
> Mutex = klucz do jednej łazienki (zawsze 0 lub 1)
> Semafor = licznik miejsc w szatni (0 do N)
>
> Pamiętaj:
> wait = "chcę wejść" (może czekać)
> signal = "wychodzę" (zwalniam miejsce)