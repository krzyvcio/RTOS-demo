# Mutex (Mutual Exclusion)

## Definicja

**Mutex** (Mutual Exclusion) to mechanizm synchronizacji zapewniający wyłączny dostęp do zasobu. Tylko jeden task może posiadać mutex w danym momencie. To "klucz do łazienki" - kto go ma, ten korzysta, inni czekają.

> Mutex to strażnik zasobu: "Jeden na raz, reszta czeka. Gdy skończysz, oddaj klucz."

```
┌─────────────────────────────────────────────────────────┐
│                    MUTEX FLOW                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A                          Task B                 │
│    │                               │                    │
│    │ lock(mutex)                   │                    │
│    │───────┐                       │                    │
│    │       │ Mutex locked by A     │                    │
│    │       ▼                       │                    │
│    │   [CRITICAL SECTION]          │                    │
│    │   shared_data = 42;           │ lock(mutex)        │
│    │                               │───────┐            │
│    │                               │       │ BLOCKED!   │
│    │   unlock(mutex)               │       ▼            │
│    │───────┐                       │    [WAITING]       │
│    │       │ Mutex unlocked        │       │            │
│    │       ▼                       │◄──────┘            │
│    │                               │ Mutex acquired!    │
│    │                               │ [CRITICAL SECTION] │
│    │                               │ shared_data = 100; │
│    │                               │ unlock(mutex)      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🚪 Klucz do łazienki

To klasyczna analogia mutexu:

```
Łazienka = zasób współdzielony
Klucz = mutex

Osoba A bierze klucz → lock(mutex)
Osoba A w łazience → critical section
Osoba B chce klucz → czeka (blocked)
Osoba A oddaje klucz → unlock(mutex)
Osoba B bierze klucz → lock(mutex)
Osoba B w łazience → critical section

JEDNA osoba w łazience na raz.
Mutex = klucz.
```

### 🦁 Terytorium lwa

```
Terytorium = zasób
Lew = mutex holder

Lew A zajmuje terytorium → lock(mutex)
Lew B chce wejść → czeka lub walczy (blocked)
Lew A odchodzi → unlock(mutex)
Lew B może wejść → lock(mutex)

Mutex = "to jest moje terytorium"
```

### 🧵 Jedyna igła w wiosce

```
Igła = zasób współdzielony
Osoba z igłą = mutex holder

Ktoś pożycza igłę → lock(mutex)
Inni potrzebują igły → czekają
Osoba oddaje igłę → unlock(mutex)
Następna osoba bierze igłę → lock(mutex)

Mutex = "kto ma igłę"
```

---

## Podobieństwo do systemów informatycznych

### Database Lock

```sql
-- Database mutex (row lock)
BEGIN TRANSACTION;
SELECT * FROM accounts WHERE id = 1 FOR UPDATE;  -- Lock
-- Critical section: modify row
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
COMMIT;  -- Unlock
```

### File Lock

```c
// File mutex
int fd = open("file.txt", O_RDWR);
flock(fd, LOCK_EX);  // Lock
// Critical section: modify file
write(fd, data, size);
flock(fd, LOCK_UN);  // Unlock
close(fd);
```

### Spinlock

```c
// Spinlock - busy-wait mutex
volatile int lock = 0;

void spin_lock(volatile int* lock) {
    while (__atomic_test_and_set(lock, __ATOMIC_ACQUIRE)) {
        // Busy wait - spin
    }
}

void spin_unlock(volatile int* lock) {
    __atomic_clear(lock, __ATOMIC_RELEASE);
}
```

---

## Mutex w RTOS

### FreeRTOS

```c
// Tworzenie mutex
SemaphoreHandle_t mutex = xSemaphoreCreateMutex();

// Lock (take)
if (xSemaphoreTake(mutex, portMAX_DELAY) == pdTRUE) {
    // Critical section
    shared_data++;

    // Unlock (give)
    xSemaphoreGive(mutex);
}

// Cleanup
vSemaphoreDelete(mutex);
```

### Zephyr

```c
// Tworzenie mutex
K_MUTEX_DEFINE(my_mutex);

// Lock
k_mutex_lock(&my_mutex, K_FOREVER);

// Critical section
shared_data++;

// Unlock
k_mutex_unlock(&my_mutex);
```

### POSIX

```c
// Tworzenie mutex
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

// Lock
pthread_mutex_lock(&mutex);

// Critical section
shared_data++;

// Unlock
pthread_mutex_unlock(&mutex);

// Cleanup
pthread_mutex_destroy(&mutex);
```

---

## Race Condition - Problem

```c
// Bez mutexu: RACE CONDITION!
int shared_counter = 0;

void task_a(void) {
    for (int i = 0; i < 1000; i++) {
        shared_counter++;  // Nieatomicowe!
    }
}

void task_b(void) {
    for (int i = 0; i < 1000; i++) {
        shared_counter++;  // Nieatomicowe!
    }
}

// shared_counter może być < 2000!
```

### Dlaczego nieatomicowe?

```
shared_counter++ składa się z 3 operacji:

1. READ:  tmp = shared_counter
2. INC:   tmp = tmp + 1
3. WRITE: shared_counter = tmp

Interleaving:

Task A: READ (tmp_A = 0)
Task B: READ (tmp_B = 0)  ← Przejęcie w środku!
Task A: INC  (tmp_A = 1)
Task A: WRITE (shared_counter = 1)
Task B: INC  (tmp_B = 1)
Task B: WRITE (shared_counter = 1)  ← Nadpisanie!

Wynik: shared_counter = 1 zamiast 2!
```

### Rozwiązanie z mutexem

```c
int shared_counter = 0;
SemaphoreHandle_t mutex = xSemaphoreCreateMutex();

void task_a(void) {
    for (int i = 0; i < 1000; i++) {
        xSemaphoreTake(mutex, portMAX_DELAY);
        shared_counter++;  // Bezpieczne!
        xSemaphoreGive(mutex);
    }
}

void task_b(void) {
    for (int i = 0; i < 1000; i++) {
        xSemaphoreTake(mutex, portMAX_DELAY);
        shared_counter++;  // Bezpieczne!
        xSemaphoreGive(mutex);
    }
}

// shared_counter zawsze = 2000 ✓
```

---

## Mutex wewnątrz ISR?

```c
// ŹLE: Mutex w ISR
void UART_IRQHandler(void) {
    xSemaphoreTake(mutex, portMAX_DELAY);  // NIEBEZPIECZNE!
    // ISR nie może blokować!
    xSemaphoreGive(mutex);
}

// DOBRZE: Użyj semafora binarnego lub task notification
SemaphoreHandle_t sem = xSemaphoreCreateBinary();

void UART_IRQHandler(void) {
    // Sygnalizuj task
    xSemaphoreGiveFromISR(sem, NULL);
}

void uart_task(void) {
    while (1) {
        xSemaphoreTake(sem, portMAX_DELAY);
        // Przetwarzaj dane bezpiecznie
    }
}
```

---

## Priority Inversion

### Problem

```
Task H (high priority)
Task M (medium priority)
Task L (low priority)

1. L lock(mutex)
2. L executing in critical section
3. H becomes ready → preempts L
4. H wants mutex → BLOCKED (L has it)
5. H is waiting for L
6. M becomes ready → preempts L
7. M runs...
8. L cannot run → cannot release mutex
9. H cannot run → waiting for mutex
10. M runs indefinitely

H (high) czeka na M (medium)!
To jest Priority Inversion!
```

### Rozwiązanie: Priority Inheritance

```c
// FreeRTOS mutex z priority inheritance
SemaphoreHandle_t mutex = xSemaphoreCreateMutex();

// FreeRTOS domyślnie używa priority inheritance
// Gdy H czeka na mutex trzymany przez L:
// L tymczasowo dostaje priorytet H

// Timeline z priority inheritance:
// 1. L lock(mutex)
// 2. L executing
// 3. H becomes ready, wants mutex
// 4. L inherits H's priority (L becomes high priority)
// 5. L continues running (now high priority)
// 6. L unlock(mutex)
// 7. L returns to low priority
// 8. H acquires mutex

// H nie czeka na M!
```

---

## Deadlock

### Problem

```c
SemaphoreHandle_t mutex_a = xSemaphoreCreateMutex();
SemaphoreHandle_t mutex_b = xSemaphoreCreateMutex();

void task_1(void) {
    xSemaphoreTake(mutex_a, portMAX_DELAY);  // Lock A
    // ... jakiś kod ...
    xSemaphoreTake(mutex_b, portMAX_DELAY);  // Wait for B
    // Critical section using both
    xSemaphoreGive(mutex_b);
    xSemaphoreGive(mutex_a);
}

void task_2(void) {
    xSemaphoreTake(mutex_b, portMAX_DELAY);  // Lock B
    // ... jakiś kod ...
    xSemaphoreTake(mutex_a, portMAX_DELAY);  // Wait for A
    // Critical section using both
    xSemaphoreGive(mutex_a);
    xSemaphoreGive(mutex_b);
}

// DEADLOCK:
// Task 1: has A, wants B
// Task 2: has B, wants A
// Oba czekają na siebie = DEADLOCK!
```

### Rozwiązania

#### 1. Kolejność blokowania

```c
// Zawsze blokuj w tej samej kolejności
void task_1(void) {
    xSemaphoreTake(mutex_a, portMAX_DELAY);  // A first
    xSemaphoreTake(mutex_b, portMAX_DELAY);  // B second
    // Critical section
    xSemaphoreGive(mutex_b);  // Unlock w odwrotnej kolejności
    xSemaphoreGive(mutex_a);
}

void task_2(void) {
    xSemaphoreTake(mutex_a, portMAX_DELAY);  // A first (TAKE SAMO!)
    xSemaphoreTake(mutex_b, portMAX_DELAY);  // B second
    // Critical section
    xSemaphoreGive(mutex_b);
    xSemaphoreGive(mutex_a);
}
```

#### 2. Timeout

```c
void task_1(void) {
    if (xSemaphoreTake(mutex_a, pdMS_TO_TICKS(100)) == pdTRUE) {
        if (xSemaphoreTake(mutex_b, pdMS_TO_TICKS(100)) == pdTRUE) {
            // Critical section
            xSemaphoreGive(mutex_b);
        } else {
            // Timeout on B, release A
            xSemaphoreGive(mutex_a);
            // Handle error
        }
        xSemaphoreGive(mutex_a);
    }
}
```

#### 3. Try-lock

```c
// Non-blocking attempt
if (xSemaphoreTake(mutex, 0) == pdTRUE) {
    // Got lock immediately
} else {
    // Lock not available, do something else
}
```

---

## Mutex vs Semafor

| Cecha | Mutex | Semafor binarny |
|-------|-------|-----------------|
| Własność | Ma właściciela | Brak właściciela |
| Recursive | Może być | Nie |
| Priority Inheritance | Tak | Nie |
| Zastosowanie | Ochrona zasobu | Sygnalizacja |
| Unlock | Tylko właściciel | Każdy |

```c
// Mutex: właściciel
xSemaphoreTake(mutex, ...);
// Tylko ten task może give
xSemaphoreGive(mutex);  // OK (ten sam task)

// Semafor: brak właściciela
xSemaphoreTake(sem, ...);
// Ktokolwiek może give
xSemaphoreGive(sem);  // OK (dowolny task/ISR)
```

---

## Recursive Mutex

```c
// Normal mutex: DEADLOCK przy rekurencji
void func_a(void) {
    xSemaphoreTake(mutex, ...);
    func_b();  // Wywołuje func_a
    xSemaphoreGive(mutex);
}

// Recursive mutex: OK
SemaphoreHandle_t recursive_mutex = xSemaphoreCreateRecursiveMutex();

void func_recursive(int depth) {
    xSemaphoreTakeRecursive(recursive_mutex, portMAX_DELAY);

    // Critical section
    shared_data++;

    if (depth > 0) {
        func_recursive(depth - 1);  // Rekurencja - OK!
    }

    xSemaphoreGiveRecursive(recursive_mutex);
}
```

---

## Mutex Best Practices

### 1. Minimalizuj sekcję krytyczną

```c
// ŹLE: Długa sekcja krytyczna
xSemaphoreTake(mutex, portMAX_DELAY);
read_sensor();       // Nie wymaga mutexu!
process_data();      // Nie wymaga mutexu!
update_shared();     // Wymaga mutexu
write_log();         // Nie wymaga mutexu!
xSemaphoreGive(mutex);

// DOBRZE: Minimalna sekcja krytyczna
read_sensor();
process_data();
xSemaphoreTake(mutex, portMAX_DELAY);
update_shared();     // Tylko to!
xSemaphoreGive(mutex);
write_log();
```

### 2. Sprawdzaj wynik

```c
// Zawsze sprawdzaj czy lock się udał
if (xSemaphoreTake(mutex, pdMS_TO_TICKS(100)) == pdTRUE) {
    // Critical section
    xSemaphoreGive(mutex);
} else {
    // Handle timeout
    log_error("Mutex timeout");
}
```

### 3. Unikaj zagnieżdżania

```c
// ŹLE: Zagnieżdżone mutexy
xSemaphoreTake(mutex_a, ...);
xSemaphoreTake(mutex_b, ...);  // Deadlock risk!
// ...
xSemaphoreGive(mutex_b);
xSemaphoreGive(mutex_a);

// DOBRZE: Jeden mutex dla całego zasobu
xSemaphoreTake(resource_mutex, ...);
// Operacje na zasobie
xSemaphoreGive(resource_mutex);
```

### 4. Używaj RAII (C++)

```cpp
class MutexGuard {
public:
    MutexGuard(SemaphoreHandle_t m) : mutex(m) {
        xSemaphoreTake(mutex, portMAX_DELAY);
    }
    ~MutexGuard() {
        xSemaphoreGive(mutex);
    }
private:
    SemaphoreHandle_t mutex;
};

void safe_function() {
    MutexGuard guard(mutex);  // Lock
    // Critical section
    // Automatyczny unlock przy wyjściu
}
```

---

## Pytania do przemyślenia

1. Jakie zasoby współdzielone mają taski w Twoim systemie?
2. Czy używasz mutexów poprawnie (minimalne sekcje krytyczne)?
3. Czy masz potencjalne deadlocki w kodzie?

---

## Quiz

**Pytanie**: Masz kod:

```c
SemaphoreHandle_t mutex;
int counter = 0;

void task1(void) {
    xSemaphoreTake(mutex, portMAX_DELAY);
    counter++;
    xSemaphoreGive(mutex);
}

void task2(void) {
    xSemaphoreTake(mutex, portMAX_DELAY);
    counter--;
    xSemaphoreGive(mutex);
}
```

Czy counter jest bezpieczny? Co jeśli counter++ nie jest atomowe?

**Odpowiedź**:

```
Tak, counter jest bezpieczny dzięki mutexowi.

Gdyby mutexu nie było:
counter++ = LOAD, INC, STORE (3 operacje)
counter-- = LOAD, DEC, STORE (3 operacje)

Możliwy interleaving:
Task1: LOAD counter (0)
Task2: LOAD counter (0)
Task1: INC (1)
Task1: STORE (1)
Task2: DEC (-1)
Task2: STORE (-1)
Wynik: counter = -1 zamiast 0!

Z mutexem:
Mutex zapewnia, że tylko jeden task wykonuje
counter++ lub counter-- naraz.
Wynik zawsze poprawny.
```

---

## Wskazówka zapamiętywania

> **Mutex = Klucz do jednej łazienki**
>
> W hotelu jest jedna łazienka na piętro.
> Klucz wisi u concierge.
>
> 1. Bierzesz klucz → lock(mutex)
> 2. Korzystasz z łazienki → critical section
> 3. Oddajesz klucz → unlock(mutex)
>
> Gdy ktoś ma klucz, inni czekają.
> Gdy oddasz klucz, ktoś inny może go wziąć.
>
> Ale uwaga:
> - Nie zgub klucza (memory leak)
> - Nie zabierz klucza, gdy ktoś w środku (nie twój mutex)
> - Nie czekaj na klucz do łazienki A, trzymając klucz do B (deadlock)