# Struktury Bez Blokad (Lock-Free Structures)

## Definicja

Struktury danych i algorytmy, które zapewniają bezpieczny dostęp współbieżny bez użycia mechanizmów blokowania (mutexy, semafory). Wykorzystują operacje atomowe procesora.

______________________________________________________________________

## Porównanie z podejściem z blokadami

```
┌───────────────────────────────────────────────────────────────┐
│              LOCK-BASED vs LOCK-FREE                          │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  Z MUTEXEM:                                                   │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Wątek A ──┐                                             │ │
│  │           ├── LOCK ──┐                                  │ │
│  │           │          ├── SEKCJA KRYTYCZNA ──┐           │ │
│  │           │          │                     │           │ │
│  │ Wątek B ──┼──────────┘ WAIT... WAIT... WAIT─┘ BLOCKED   │ │
│  │           │                                             │ │
│  │ Problemy:  │                                             │ │
│  │ ├── Priority Inversion                                 │ │
│  │ ├── Deadlock possible                                  │ │
│  │ ├── Convoying (wątki czekają w kolejce)               │ │
│  │ └── Preemption w sekcji krytycznej                     │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  LOCK-FREE:                                                   │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Wątek A ──┐                                             │ │
│  │           ├── CAS(0→1) ──┐                              │ │
│  │           │              ├── OPERACJA ──┐              │ │
│  │           │              │             │              │ │
│  │ Wątek B ──┼── CAS(0→1) FAIL → RETRY → CAS(0→1) OK      │ │
│  │           │                                             │ │
│  │ Zalety:    │                                             │ │
│  │ ├── Brak blokowania                                    │ │
│  │ ├── Brak deadlock                                      │ │
│  │ ├── Brak priority inversion                            │ │
│  │ └── Progress gwarantowany                              │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Operacje atomowe

| Operacja | Opis | Dostępność |
|----------|------|------------|
| **CAS** (Compare-And-Swap) | Atomowe porównanie i zamiana | x86, ARM, RISC-V |
| **LL/SC** (Load-Linked/Store-Conditional) | Atomowa para operacji | ARM, MIPS, PowerPC |
| **FAA** (Fetch-And-Add) | Atomowe dodawanie | x86, ARM |
| **Test-And-Set** | Atomowe ustawienie bitu | Wszystkie |
| **Atomic Exchange** | Atomowa zamiana wartości | Wszystkie |

### CAS w praktyce

```c
// Compare-And-Swap (pseudokod)
bool CAS(int* addr, int expected, int new_value) {
    atomically {
        if (*addr == expected) {
            *addr = new_value;
            return true;  // Sukces
        }
        return false;  // Porażka, addr zmienione przez inny wątek
    }
}

// Użycie w lock-free increment
void increment(int* counter) {
    int old_value;
    do {
        old_value = *counter;
    } while (!CAS(counter, old_value, old_value + 1));
}
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### IoT i Embedded

| Zastosowanie | Struktura | Zaleta |
|--------------|-----------|--------|
| ISR → Task communication | Lock-free ring buffer | Brak blokowania w ISR |
| Sensor data collection | Lock-free queue | Determinizm |
| Event logging | Lock-free stack | Brak priority inversion |

### Automotive

- **CAN bus message queue** - lock-free FIFO
- **Sensor data fusion** - lock-free ring buffer
- **ADAS pipeline** - lock-free message passing

### Audio/Video Processing

- **Audio buffer** - SPSC (Single Producer Single Consumer)
- **Video frame queue** - MPMC (Multi Producer Multi Consumer)
- **Real-time streaming** - brak jittera z blokad

______________________________________________________________________

## Przykładowe struktury

### 1. Lock-Free Stack (Treiber Stack)

```c
struct Node {
    int data;
    struct Node* next;
};

struct Stack {
    struct Node* head;
};

void push(struct Stack* s, struct Node* new_node) {
    struct Node* old_head;
    do {
        old_head = s->head;
        new_node->next = old_head;
    } while (!CAS(&s->head, old_head, new_node));
}

struct Node* pop(struct Stack* s) {
    struct Node* old_head;
    do {
        old_head = s->head;
        if (old_head == NULL) return NULL;
    } while (!CAS(&s->head, old_head, old_head->next));
    return old_head;
}
```

### 2. Lock-Free Ring Buffer (SPSC)

```c
#define SIZE 1024  // Musi być potęgą 2!

struct RingBuffer {
    int buffer[SIZE];
    atomic_uint head;  // Write index (producer)
    atomic_uint tail;  // Read index (consumer)
};

// Producer (tylko jeden!)
bool write(struct RingBuffer* rb, int value) {
    uint head = rb->head;
    uint tail = rb->tail;

    if ((head - tail) == SIZE) return false;  // Pełny

    rb->buffer[head & (SIZE - 1)] = value;
    rb->head = head + 1;  // Atomic write
    return true;
}

// Consumer (tylko jeden!)
bool read(struct RingBuffer* rb, int* value) {
    uint head = rb->head;
    uint tail = rb->tail;

    if (head == tail) return false;  // Pusty

    *value = rb->buffer[tail & (SIZE - 1)];
    rb->tail = tail + 1;  // Atomic write
    return true;
}
```

### 3. Lock-Free Queue (MPMC)

```c
// Michael-Scott Queue
// Bardziej złożona, wspiera wielu producentów i konsumentów

struct Node {
    int data;
    atomic_ptr next;
};

struct Queue {
    atomic_ptr head;
    atomic_ptr tail;
};

// Push i Pop wykorzystują CAS na head i tail
// Szczegóły: M. Michael, M. Scott (1996)
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **ABA Problem** | Wartość zmienia się A→B→A, CAS myśli że bez zmian | Hazard pointers, epoch-based reclamation |
| **Memory Reclamation** | Kiedy zwolnić pamięć? | RCU, hazard pointers |
| **Starvation** | Wątek ciągle przegrywa CAS | Backoff, exponential |
| **Cache Thrashing** | Duży ruch na zmiennej atomowej | Per-CPU variables |
| **Live-lock** | Wszystkie wątki próbują, żaden nie postępuje | Limit retries, fallback to lock |

### ABA Problem - przykład

```
Wątek A:                          Wątek B:
1. read head = X (data=A)
                                  2. pop() → X (data=A)
                                  3. pop() → Y (data=B)
                                  4. push(X) → X wraca na head
                                  5. X->next = Y (zmienione!)
6. CAS(head, X, X->next)
   - X nadal na head
   - ALE X->next jest inny!
   - CAS succeeds, ale struktura uszkodzona!
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Hazard Pointers

```c
// Każdy wątek ma listę "niebezpiecznych" wskaźników
// Pamięć jest zwalniana gdy żaden wątek jej nie używa

__thread void* hazard_pointers[MAX_HAZARDS];

void safe_free(void* ptr) {
    // Sprawdź czy żaden wątek nie ma tego wskaźnika
    // jako hazard pointer
    if (!is_hazardous(ptr)) {
        free(ptr);
    }
}
```

### 2. RCU (Read-Copy-Update)

- Używane w Linux kernel
- Czytelnicy bez blokad
- Pisarze tworzą kopię
- Stara wersja zwalniana po grace period

### 3. Persistent Memory Lock-Free

- Lock-free dla NVRAM
- Crash consistency
- Nowe algorytmy dla persistent memory

______________________________________________________________________

## Biblioteki i narzędzia

| Biblioteka | Platforma | Struktury |
|------------|-----------|-----------|
| **boost::lockfree** | C++ | queue, stack, spsc_queue |
| **concurrentqueue** | C++ | MPMC queue |
| **crossbeam** | Rust | queue, stack, channels |
| **Disruptor** | Java | Ring buffer |
| **FreeRTOS Queue** | Embedded | Queue (opcjonalnie lock-free) |
| **Zephyr** | Embedded | Kernel objects |

______________________________________________________________________

## Zadanie praktyczne: Deadlock w Rust

```rust
// ZADANIE: Znajdź i napraw deadlock

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = thread::spawn(move || {
        let _g1 = a1.lock().unwrap();  // Lock A
        thread::sleep_ms(100);
        let _g2 = b1.lock().unwrap();  // Wait for B - DEADLOCK!
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let h2 = thread::spawn(move || {
        let _g1 = b2.lock().unwrap();  // Lock B
        thread::sleep_ms(100);
        let _g2 = a2.lock().unwrap();  // Wait for A - DEADLOCK!
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

// ROZWIĄZANIA:
// 1. Zawsze blokuj w tej samej kolejności (A potem B)
// 2. Użyj try_lock() z timeout
// 3. Użyj lock-free queue zamiast dwóch mutexów
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego lock-free struktury eliminują priority inversion?
1. Co to jest ABA problem i jak go rozwiązać?
1. Kiedy warto użyć lock-free, a kiedy mutex?
1. Jakie są wymagania dla lock-free w systemach RT?

______________________________________________________________________

## Literatura

1. Herlihy, Shavit, "The Art of Multiprocessor Programming"
1. Michael, Scott, "Simple, Fast, and Practical Non-Blocking Queue" (1996)
1. Drepper, "Futexes are Tricky" (2011)
1. boost::lockfree documentation
