# Deadlock Prevention: Rust vs C

## Problem

Deadlock to sytuacja gdzie dwa lub więcej wątków czekają na siebie nawzajem, tworząc cykl oczekiwania. W systemach RTOS może to być katastrofalne - system zawiesza się bez możliwości recovery.

### Warunki powstania deadlock (Coffman)

1. **Mutual Exclusion** - zasób może być używany przez jeden wątek
2. **Hold and Wait** - wątek trzyma zasób i czeka na inny
3. **No Preemption** - zasób nie może być zabrany siłą
4. **Circular Wait** - cykliczne oczekiwanie wątków

### Typowe scenariusze w RTOS

| Scenariusz | Opis | Skutek |
|------------|------|--------|
| AB-BA | Dwa mutexy w różnej kolejności | Zawieszenie |
| Self-deadlock | Rekurencyjny lock | Zawieszenie |
| Lock ordering | Wiele zasobów w różnej kolejności | Zawieszenie |
| Priority inversion | Niski priorytet blokuje wysoki | Missed deadline |

---

## Przypadek 1: Klasyczny AB-BA Deadlock

### Kod w C (podatny na błąd)

```c
// ============================================================================
// C: AB-BA Deadlock - podatny na błąd
// ============================================================================

#include <pthread.h>
#include <stdio.h>

pthread_mutex_t mutex_a = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t mutex_b = PTHREAD_MUTEX_INITIALIZER;

void* thread_1(void* arg) {
    printf("Thread 1: Locking A...\n");
    pthread_mutex_lock(&mutex_a);
    
    // Symulacja pracy
    usleep(100000);
    
    printf("Thread 1: Locking B...\n");
    pthread_mutex_lock(&mutex_b);  // <- MOŻE ZAWIESIĆ!
    
    printf("Thread 1: Critical section\n");
    
    pthread_mutex_unlock(&mutex_b);
    pthread_mutex_unlock(&mutex_a);
    return NULL;
}

void* thread_2(void* arg) {
    printf("Thread 2: Locking B...\n");
    pthread_mutex_lock(&mutex_b);  // <- MOŻE ZAWIESIĆ!
    
    usleep(100000);
    
    printf("Thread 2: Locking A...\n");
    pthread_mutex_lock(&mutex_a);
    
    printf("Thread 2: Critical section\n");
    
    pthread_mutex_unlock(&mutex_a);
    pthread_mutex_unlock(&mutex_b);
    return NULL;
}

// ============================================================================
// MAIN
// ============================================================================
int main() {
    pthread_t t1, t2;
    pthread_create(&t1, NULL, thread_1, NULL);
    pthread_create(&t2, NULL, thread_2, NULL);
    
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
    
    printf("Done\n");
    return 0;
}
```

**Problem:** Kolejność blokowania zależy od harmonogramu - losowo może działać lub zawiesić się.

### Kod w Rust (bezpieczny)

```rust
// ============================================================================
// Rust: AB-BA Deadlock - wykryty w compile-time!
// ============================================================================

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(1));

    // Thread 1: Lock A, then B
    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let t1 = thread::spawn(move || {
        let _guard_a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _guard_b = b1.lock().unwrap();  // <- W compile-time wiemy że OK
    });

    // Thread 2: Lock A, then B (TA SAMA KOLEJNOŚĆ!)
    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let t2 = thread::spawn(move || {
        let _guard_a = a2.lock().unwrap();  // <- TA SAMA KOLEJNOŚĆ!
        thread::sleep(Duration::from_millis(100));
        let _guard_b = b2.lock().unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
```

**Dlaczego działa:** W Rust nie ma AB-BA bo kompilator wymusza spójną kolejność (lub programista musi ją świadomie zachować - wtedy jest to jawne).

---

## Przypadek 2: Rekurencyjny Lock (Self-Deadlock)

### Kod w C

```c
// ============================================================================
// C: Self-deadlock - pthread_recursive_mutex
// ============================================================================

pthread_mutex_t recursive_mutex;
pthread_mutexattr_t attr;

// W main:
pthread_mutexattr_init(&attr);
pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE);
pthread_mutex_init(&recursive_mutex, &attr);

void function_a() {
    pthread_mutex_lock(&recursive_mutex);  // Lock 1
    function_b();  // Wywołuje function_b
    pthread_mutex_unlock(&recursive_mutex);  // Unlock 1
}

void function_b() {
    pthread_mutex_lock(&recursive_mutex);  // Lock 2 - OK bo RECURSIVE
    // ...
    pthread_mutex_unlock(&recursive_mutex);  // Unlock 2
}
```

**Problem:** Łatwo o błąd - jeśli nie ustawisz `RECURSIVE`, to self-deadlock!

### Kod w Rust

```rust
// ============================================================================
// Rust: self-deadlock - niemożliwy bez unsafe!
// ============================================================================

use std::sync::{Arc, Mutex};

// W Rust nie możesz tak po prostu zrobić recursive lock!
// Musisz użyć std::sync::ReentrantLock (nie istnieje w std!)

// Alternatywa: std::thread::sleep nie blokuje innych wątków
// Ale jeśli potrzebujesz rekurencyjnego mutex...

// Rozwiązanie 1: Nie używaj rekurencyjnego mutex
// Rozwiązanie 2: Użyj parking_lot

use parking_lot::Mutex;

fn function_a() {
    let guard = mutex.lock();
    function_b(&guard);
}

fn function_b(_guard: &parking_lot::MutexGuard<i32>) {
    // Nie można ponownie zablokować tego samego mutex!
    // To jest ZABRONIONE w Rust - compilerError!
}
```

**Dlaczego działa:** Rust domyślnie nie pozwala na rekurencyjne mutex - musisz użyć zewnętrznej biblioteki `parking_lot` lub `spin`. To wymusza przemyślenie architektury!

---

## Przypadek 3: Banker's Algorithm (Deadlock Prevention)

### Implementacja w Rust

```rust
// ============================================================================
// Rust: Banker's Algorithm - deadlock prevention
// ============================================================================

use core::cmp::Ordering;

/// Maksymalne wymagania zasobów
const MAX_RESOURCES: u32 = 10;

/// Stan systemu
struct BankerState {
    available: [u32; 3],      // Dostępne zasoby
    maximum: [[u32; 3]; 5],   // Maksymalne wymagania zadań
    allocation: [[u32; 3]; 5],// Aktualna alokacja
    need: [[u32; 3]; 5],      // Potrzeby = maximum - allocation
}

impl BankerState {
    /// Sprawdź czy stan jest bezpieczny (algorytm bankiera)
    fn is_safe(&self) -> bool {
        let mut work = self.available;
        let mut finish = [false; 5];
        
        loop {
            let mut found = false;
            
            for i in 0..5 {
                if !finish[i] && Self::can_satisfy(self.need[i], work) {
                    // Symuluj zakończenie zadania
                    for j in 0..3 {
                        work[j] += self.allocation[i][j];
                    }
                    finish[i] = true;
                    found = true;
                }
            }
            
            if !found {
                break;
            }
        }
        
        // Wszystkie zadania zakończone = bezpieczny stan
        finish.iter().all(|&f| f)
    }
    
    fn can_satisfy(need: [u32; 3], work: [u32; 3]) -> bool {
        for i in 0..3 {
            if need[i] > work[i] {
                return false;
            }
        }
        true
    }
    
    /// Próba alokacji zasobów
    fn request(&mut self, task: usize, request: [u32; 3]) -> Result<(), Error> {
        // 1. Czy request <= need?
        for i in 0..3 {
            if request[i] > self.need[task][i] {
                return Err(Error::ExceedsNeed);
            }
        }
        
        // 2. Czy request <= available?
        for i in 0..3 {
            if request[i] > self.available[i] {
                return Err(Error::NotEnoughResources);
            }
        }
        
        // 3. Symuluj alokację
        let mut work = self.available;
        let mut allocation = self.allocation;
        let mut need = self.need;
        
        for i in 0..3 {
            work[i] -= request[i];
            allocation[task][i] += request[i];
            need[task][i] -= request[i];
        }
        
        // 4. Sprawdź bezpieczeństwo
        let temp_state = BankerState {
            available: work,
            maximum: self.maximum,
            allocation,
            need,
        };
        
        if temp_state.is_safe() {
            // Commit
            for i in 0..3 {
                self.available[i] -= request[i];
                self.allocation[task][i] += request[i];
                self.need[task][i] -= request[i];
            }
            Ok(())
        } else {
            Err(Error::Unsafe)
        }
    }
}

#[derive(Debug)]
enum Error {
    ExceedsNeed,
    NotEnoughResources,
    Unsafe,
}
```

### Implementacja w C

```c
// ============================================================================
// C: Banker's Algorithm - trudniejsze w utrzymaniu
// ============================================================================

#include <stdbool.h>
#include <stdint.h>

#define N_TASKS 5
#define N_RESOURCES 3

typedef struct {
    uint32_t available[N_RESOURCES];
    uint32_t maximum[N_TASKS][N_RESOURCES];
    uint32_t allocation[N_TASKS][N_RESOURCES];
    uint32_t need[N_TASKS][N_RESOURCES];
} BankerState;

// Problem: W C musisz ręcznie zarządzać stanem
// Łatwo o:
// - memory leak
// - array overflow  
// - race condition przy dostępie do stanu
// - niespójność need vs allocation

bool is_safe(BankerState* state) {
    uint32_t work[N_RESOURCES];
    bool finish[N_TASKS];
    
    // Copy available to work
    for (int i = 0; i < N_RESOURCES; i++) {
        work[i] = state->available[i];
        finish[i] = false;
    }
    
    // Algorithm - w C łatwo o off-by-one lub overflow
    // Brak bounds checking!
    // ...
}

int request_resources(BankerState* state, int task, uint32_t* request) {
    // W C musisz sprawdzić WSZYSTKO ręcznie
    // Łatwo zapomnieć o sprawdzeniu!
    // Brak Type Safety!
}
```

---

## Przypadek 4: Try-Lock z Timeout

### C: pthread_mutex_timedlock

```c
// ============================================================================
// C: Try-lock z timeout
// ============================================================================

#include <pthread.h>
#include <time.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

int try_lock_with_timeout(pthread_mutex_t* mutex, uint32_t ms_timeout) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    
    // Oblicz timeout
    ts.tv_sec += ms_timeout / 1000;
    ts.tv_nsec += (ms_timeout % 1000) * 1000000;
    
    int result = pthread_mutex_timedlock(mutex, &ts);
    
    if (result == ETIMEDOUT) {
        return 0;  // Timeout
    }
    return 1;  // Locked
}

// Problem: Łatwo zapomnieć o sprawdzeniu wyniku!
pthread_mutex_lock(&mutex);  // <- BŁĄD: brak sprawdzenia
```

### Rust: try_lock

```rust
// ============================================================================
// Rust: try_lock z timeout - bezpieczniejszy
// ============================================================================

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

fn try_lock_with_timeout<T>(
    mutex: &Mutex<T>,
    timeout: Duration,
) -> Option<std::sync::MutexGuard<T>> {
    let start = Instant::now();
    
    loop {
        match mutex.try_lock() {
            Ok(guard) => return Some(guard),
            Err(_) => {
                if start.elapsed() >= timeout {
                    return None;
                }
                // Mały sleep żeby nie cpu-spinować
                thread::sleep(Duration::from_micros(100));
            }
        }
    }
}

// Użycie:
let mutex = Arc::new(Mutex::new(0));

if let Some(guard) = try_lock_with_timeout(&mutex, Duration::from_millis(100)) {
    // Bezpieczne użycie - compiler gwarantuje że nie pomijamy wyniku!
    *guard += 1;
} else {
    // Timeout - możemy obsłużyć elegancko
    eprintln!("Timeout waiting for lock");
}
```

**Przewaga Rust:** Nie można przypadkowo zignorować wyniku `try_lock` - `Result` wymaga obsługi!

---

## Przypadek 5: Lock Ordering z typami

### Rust: Type-level locking order

```rust
// ============================================================================
// Rust: Enforce lock ordering w typach
// ============================================================================

/// Typy reprezentujące "poziom" zasobu - kompilator pilnuje kolejności!
trait LockOrder {}
trait LockOrder1: LockOrder {}
trait LockOrder2: LockOrder1 {}

struct ResourceA;
struct ResourceB;

impl LockOrder1 for ResourceA {}
impl LockOrder2 for ResourceB {}

/// Wrapper który wymusza kolejność - typ to encode!
struct Locked<T: LockOrder> {
    _data: T,
}

// Problem: W Rust nie możesz po prostu "wymusić" kolejności na typach
// Ale możesz użyć typów do dokumentacji i asercji!

use core::sync::atomic::{AtomicBool, Ordering};

/// Zasób z lock order - pomocne przy analizie
struct OrderedLock<T> {
    data: T,
    order: u8,
}

impl<T> OrderedLock<T> {
    fn new(data: T, order: u8) -> Self {
        Self { data, order }
    }
    
    /// Próba lock z weryfikacją kolejności
    fn lock(&self, current_order: u8) -> Result<LockedRef<T>, LockOrderError> {
        if self.order != current_order + 1 {
            return Err(LockOrderError::WrongOrder {
                expected: current_order + 1,
                actual: self.order,
            });
        }
        
        // Success - w normalnym mutex
        todo!("return lock guard")
    }
}

#[derive(Debug)]
struct LockOrderError {
    expected: u8,
    actual: u8,
}
```

---

## Typowe błędy i jak ich unikać

### Błąd 1: Ignorowanie wyniku lock

```c
// ŹLE w C
pthread_mutex_lock(&mutex);  // Wynik zignorowany!

// DOBRZE w C  
if (pthread_mutex_lock(&mutex) != 0) {
    // obsługa błędu
}

// W Rust - domyślnie unwrap() lub match
let _guard = mutex.lock().unwrap();  // panic jeśli error
```

### Błąd 2: Niespójna kolejność lock

```rust
// ŹLE w Rust - nie jawne
thread1: lock(A) -> lock(B)
thread2: lock(B) -> lock(A)  // <- Może być problem!

// DOBRZE - jawna spójna kolejność
const LOCK_ORDER_A: u8 = 0;
const LOCK_ORDER_B: u8 = 1;

fn lock_both(a: &Mutex<i32>, b: &Mutex<i32>) {
    // Zawsze w tej samej kolejności!
    let _a = a.lock().unwrap();
    let _b = b.lock().unwrap();
}
```

### Błąd 3: Deadlock z condition variable

```c
// ŹLE: condition variable bez timeout
pthread_cond_wait(&cond, &mutex);  // <- Może czekać wieczność!

// DOBRZE: z timeout
struct timespec ts;
// ... oblicz timeout ...
pthread_cond_timedwait(&cond, &mutex, &ts);
```

---

## Porównanie

| Aspekt | C | Rust |
|--------|---|------|
| AB-BA detection | Brak | compile-time przy spójnej kolejności |
| Lock result | Łatwo zignorować | `Result` wymaga obsługi |
| Recursive lock | Pthread attr | Wymaga explicit biblioteki |
| Try-lock | Funkcja zwraca int | `try_lock()` zwraca `Result` |
| Timeout | timespec | `Duration` - type-safe |
| Deadlock prevention | Ręczne | Typy + borrow checker |

---

## Podsumowanie

| Technika | Rust | C |
|----------|------|-----|
| Unikanie AB-BA | Spójna kolejność lub compile-error | Dokumentacja, code review |
| Try-lock | `try_lock()` -> `Result` | `pthread_mutex_timedlock` |
| Timeout | `Duration` type-safe | `struct timespec` |
| Recovery | Brak (panic) | Własna logika |
| Prevention | Lock ordering | Banker's algorithm |

### Najlepsze praktyki Rust

1. **Zawsze ta sama kolejność lock** - dokumentuj w kodzie
2. **Używaj `try_lock`** zamiast `lock` - mniejsze ryzyko deadlock
3. **Short critical sections** - minimalizuj czas trzymania locka
4. **Avoid nested locks** - jeśli możliwe
5. **Testuj pod obciążeniem** - deadlock często losowy

### Wskazówki dla C

1. **Zawsze sprawdzaj wynik** każdego lock/unlock
2. **Timeout na każdym lock** - nawet `pthread_mutex_lock`
3. **Static analysis** - cppcheck, clang-analyzer
4. **Lock ordering** - dokumentuj i egzekwuj w code review
