# Lekcja 1: Kolejka SPSC (Lock-Free)

## Problem

W systemach czasu rzeczywistego (RTOS) potrzebujemy przekazywać dane między przerwaniami (ISR) a głównym programem. Użycie mutexów powoduje nieprzewidywalne opóźnienia - to niedopuszczalne!

## Rozwiązanie

Kolejka SPSC (Single Producer, Single Consumer) działa bez blokad. Jest bardzo szybka i deterministyczna.

## Kod w Rust

```rust
use std::sync::atomic::{AtomicU32, Ordering};

pub struct SpscQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    head: AtomicU32,  // odczyt
    tail: AtomicU32, // zapis
}

impl<T: Copy, const N: usize> SpscQueue<T, N> {
    pub const fn new() -> Self {
        assert!(N.is_power_of_two());
        Self {
            buffer: [const { None }; N],
            head: AtomicU32::new(0),
            tail: AtomicU32::new(0),
        }
    }

    // Producer (ISR) - nie blokuje się!
    pub unsafe fn push(&self, value: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next = (tail + 1) & (N - 1);
        
        if next != self.head.load(Ordering::Acquire) {
            self.buffer[tail as usize] = Some(value);
            self.tail.store(next, Ordering::Release);
            true
        } else {
            false // kolejka pełna
        }
    }

    // Consumer (main)
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        
        if head != self.tail.load(Ordering::Acquire) {
            let value = self.buffer[head as usize].take();
            self.head.store((head + 1) & (N - 1), Ordering::Release);
            value
        } else {
            None // kolejka pusta
        }
    }
}
```

## Typowe błędy

| Błąd | Rozwiązanie |
|-------|-------------|
| Nadpisanie danych | Zawsze sprawdzaj `is_full()` |
| Race condition | Używaj `Ordering::Acquire/Release` |
| Zły rozmiar | N musi być potęgą dwójki |

## Porównanie z C

```c
// C - trudniej o bezpieczeństwo
void isr() {
    xQueueSendFromISR(q, &data, NULL); // może blokować!
}

// Rust - bezpieczniej
unsafe { queue.push(data); } // nie blokuje!
```

## Następca

`crossbeam::queue::SegQueue` - bezpieczna, wielowątkowa wersja.

______________________________________________________________________

# Lekcja 2: Scheduler z Priorytetami

## Problem

W RTOS zadania muszą być wykonywane według priorytetów. Zadanie o wysokim priorytecie nie może czekać na niskie!

## Kod w Rust

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Eq, PartialEq)]
struct Task {
    prio: u8,
    id: u32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.prio.cmp(&self.prio) // wyższy prio = pierwszy
    }
}

struct Scheduler {
    heap: BinaryHeap<Task>,
}

impl Scheduler {
    fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }
    
    fn add_task(&mut self, prio: u8, id: u32) {
        self.heap.push(Task { prio, id });
    }
    
    fn next(&mut self) -> Option<u32> {
        self.heap.pop().map(|t| t.id)
    }
}
```

## Typowe błędy

- **Brak priority inheritance** - powoduje inversion
- **Przepełnienie sterty** - limituj liczbę zadań
- **Zła kolejność** - używaj `Ord` poprawnie

## Następca

`embassy` - async RTOS dla embedded w Rust.

______________________________________________________________________

# Lekcja 3: Atomowe Operacje

## Problem

Wielowątkowość wymaga synchronizacji. Musimy wiedzieć kiedy zmiana jest widoczna dla innych wątków.

## Kod w Rust

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

// Increment atomowy
COUNTER.fetch_add(1, Ordering::SeqCst);

// Compare-and-swap
let old = COUNTER.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst);
```

## Ordering - co wybrać?

| Ordering | Kiedy użyć |
|----------|------------|
| Relaxed | Liczniki, brak synchronizacji |
| Acquire | Odczyt po write Release |
| Release | Zapis przed read Acquire |
| SeqCst | Gdy potrzebujesz gwarancji |

## Typowe błędy

- **Zmieszanie orderings** - prowadzi do race conditions
- **Brak synchronizacji** - dane niespójne
- **Za mocny ordering** - niepotrzebny narzut

______________________________________________________________________

# Lekcja 4: Bariery Pamięci

## Problem

Kompilator i CPU mogą przestawiać operacje. Musisz powiedzieć kiedy kolejność ma znaczenie!

## Kod w Rust

```rust
use std::sync::atomic::{AtomicBool, Ordering};

static READY: AtomicBool = AtomicBool::false;
static DATA: AtomicU32 = AtomicU32::new(0);

// Wątek A (producer)
DATA.store(42, Ordering::Release);
READY.store(true, Ordering::Release);

// Wątek B (consumer)
if READY.load(Ordering::Acquire) {
    println!("{}", DATA.load(Ordering::Relaxed)); // 42!
}
```

## Typowe błędy

| Błąd | Skutek |
|-------|---------|
| Brak barrier | Stare dane czytane |
| Za słaby ordering | Niespójność |
| Memory reorder | Race condition |

______________________________________________________________________

# Lekcja 5: Async/Await w Embedded

## Problem

Tradycyjne wątki zużywają za dużo pamięci. W embedded chcemy lżejsze rozwiązania.

## Kod w Rust

```rust
#![no_std]
#![feature(async_fn_in_trait)]

use embassy::executor::Spawner;
use embassy_time::{Duration, Timer};

async fn blink_led() {
    loop {
        // toggle LED
        Timer::after(Duration::from_millis(100)).await;
    }
}
```

## Zalety async

- Mniejsze zużycie pamięci
- Latwy concurrent I/O
- Zero koszt przy czekaniu

## Wady

- Niepreemptive (współpracujące)
- Trudniejsze o hard real-time

## Następca

`embassy-nrf` - pełne async API dla ARM Cortex-M.

______________________________________________________________________

# Lekcja 6: Obsługa Panic w no_std

## Problem

W embedded nie mamy std. Co robić przy panic? Nie możemy użyć domyślnego println!

## Kod w Rust

```rust
#![no_std]
#![panic_handler]

fn panic_info(_info: &core::panic::PanicInfo) -> ! {
    // Zatrzymaj system lub wyślij na debug UART
    loop {
        cortex_m::asm::wfi();
    }
}
```

## Opcje

| Metoda | Zalety | Wady |
|--------|--------|------|
| `panic_halt` | Mały rozmiar | Brak info |
| `panic_abort` | Najmniejszy | Brak recovery |
| Custom handler | Pełna kontrola | Więcej kodu |

## Następca

`defmt` - formatted printing dla embedded przez debug probe.

______________________________________________________________________

# Lekcja 7: Integracja Rust + FreeRTOS

## Problem

Chcesz użyć Rust ale Twój projekt używa FreeRTOS w C.

## Rozwiązanie

Stwórz bezpieczne wrappery RAII wokół C API.

```rust
#[link(name = "freertos")]
extern "C" {
    fn xSemaphoreCreateMutex() -> *mut c_void;
    fn xSemaphoreTake(sem: *mut c_void, wait: u32) -> i32;
    fn xSemaphoreGive(sem: *mut c_void) -> i32;
}

// Bezpieczny wrapper
pub struct Mutex {
    handle: *mut c_void,
}

impl Mutex {
    pub fn new() -> Self {
        Self { handle: unsafe { xSemaphoreCreateMutex() } }
    }
    
    pub fn lock(&self) {
        unsafe { xSemaphoreTake(self.handle, u32::MAX) };
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe { xSemaphoreGive(self.handle) };
    }
}
```

## Zalety

- Stopniowa migracja
- Bezpieczne RAII
- Lepsze error handling

## Następca

`cortex-m-rtic` - Rust RTOS zamiast FreeRTOS.
