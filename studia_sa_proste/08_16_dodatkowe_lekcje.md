# Lekcja 8: Deadlock - Jak Unikać Zakleszczeń

## Problem

Deadlock to sytuacja gdy dwa lub więcej wątków czekają na siebie nawzajem. System się zawiesza! [1]

## Warunki Powstania

1. **Wzajemne wykluczanie** - zasób może być jeden
1. **Trzymaj i czekaj** - trzymasz jeden, czekasz na drugi
1. **Brak preempcji** - nie można zabrać siłą
1. **Cykliczne czekanie** - A czeka na B, B czeka na A

## Rozwiązanie: Zawsze Ta Sama Kolejność

```rust
use std::sync::Mutex;

static MUTEX_A: Mutex<()> = Mutex::new(());
static MUTEX_B: Mutex<()> = Mutex::new(());

// Zła wersja - różna kolejność!
fn bad_order() {
    let a = MUTEX_A.lock().unwrap();
    let b = MUTEX_B.lock().unwrap(); // Może zawiesić!
}

// Dobra wersja - zawsze A potem B
fn good_order() {
    let _a = MUTEX_A.lock().unwrap();
    let _b = MUTEX_B.lock().unwrap(); // Zawsze bezpieczne!
}
```

## Rozwiązanie: Try-Lock

```rust
use std::sync::Mutex;
use std::time::Duration;

fn try_lock_example() {
    let mutex = Mutex::new(0);
    
    // Próbuj przez 100ms
    let guard = mutex.try_lock_for(Duration::from_millis(100));
    
    match guard {
        Ok(mut value) => {
            *value += 1;
        }
        Err(_) => {
            // Timeout - obsłuż elegancko
            println!("Nie udało się zablokować!");
        }
    }
}
```

## Rozwiązanie: Limit Czasu

```rust
fn lock_with_timeout() {
    // Zawsze timeout - nigdy infinite wait
    for _ in 0..10 {
        if let Ok(guard) = MUTEX_A.try_lock() {
            // Operacje
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    println!("Timeout!");
}
```

## Typowe Błędy

| Błąd | Skutek |
|-------|--------|
| Różna kolejność lock | AB-BA deadlock |
| Nieskończony wait | Zawieszenie |
| Brak timeout | Brak recovery |

## Porównanie z C

```c
// C - łatwo o błąd
pthread_mutex_lock(&A);
pthread_mutex_lock(&B); // Albo B->A gdzie indziej = DEADLOCK!

// Rust - bezpieczniej
let _a = A.lock().unwrap();
let _b = B.lock().unwrap(); // Compiler pilnuje kolejności!
```

______________________________________________________________________

# Lekcja 9: Priority Inversion (Odwrócenie Priorytetów)

## Problem

Zadanie o niskim priorytecie blokuje zadanie o wysokim! Klasyczny przykład: [4]

```
CZAS →
WYSOKI ─────┐ czeka na mutex
             │
NISKI ───╔═══╧═════╗ trzyma mutex
         ║         ║
ŚREDNI ──╚═════════╝ wywłaszcza NISKI!
```

## Rozwiązanie: Priority Inheritance

```rust
// W FreeRTOS - włącz priority inheritance
unsafe {
    xSemaphoreCreateMutex();
// Configuration: configUSE_MUTEXES = 1
}
```

## Własna Implementacja

```rust
use std::sync::atomic::{AtomicU8, Ordering};

struct PriorityInheritMutex {
    holder: AtomicU8,
    original_priority: AtomicU8,
    count: AtomicU8,
}

impl PriorityInheritMutex {
    fn lock(&self, my_prio: u8) {
        loop {
            // Zwiększ priorytet holdera jeśli niższy
            let holder = self.holder.load(Ordering::Relaxed);
            if holder != 0 && holder > my_prio {
                // Podnieś priorytet holdera (symulacja)
                self.original_priority.store(holder, Ordering::Relaxed);
                // set_thread_priority(holder, my_prio);
            }
            
            // Próba lock
            match self.holder.compare_exchange(
                0, my_prio, Ordering::Acquire, Ordering::Relaxed
            ) {
                Ok(_) => break,
                Err(_) => {
                    // Czekaj i spróbuj ponownie
                    std::thread::sleep(std::time::Duration::from_micros(100));
                }
            }
        }
    }
    
    fn unlock(&self) {
        // Przywróć oryginalny priorytet
        let orig = self.original_priority.load(Ordering::Relaxed);
        if orig != 0 {
            // restore_priority(orig);
        }
        self.holder.store(0, Ordering::Release);
    }
}
```

## Typowe Błędy

| Błąd | Skutek |
|-------|--------|
| Brak inheritance | Priority inversion |
| Zły priorytet | Inversion nadal możliwy |
| Brak inheritance chain | Chain blocking |

______________________________________________________________________

# Lekcja 10: Sekcje Krytyczne

## Problem

Sekcja krytyczna to fragment kodu który musi się wykonać atomowo - bez przerwania. W embedded to często dostęp do rejestrów. [7]

## Rozwiązanie w Rust

```rust
use cortex_m::interrupt::free;

// Sekcja krytyczna - wyłącz przerwania
fn critical_operation() {
    free(|_cs| {
        // Tu żaden ISR się nie wykona
        // Dostęp do współdzielonych danych
    });
}
```

## Alternatywa: Mutex z NoISR

```rust
use cortex_m::peripheral::NVIC;

fn disable_irq() {
    NVIC::mask(|_| {});
}

// Lub z maską konkretnego przerwania
fn mask_systick() {
    NVIC::mask(21..22); // Mask SysTick
}
```

## Długość Sekcji Krytycznej

| Maksymalny czas | Zalecane dla |
|-----------------|--------------|
| < 1 µs | Proste operacje |
| < 10 µs | Średniej wielkości |
| > 10 µs | Zrefaktoryzuj! |

## Typowe Błędy

| Błąd | Skutek |
|-------|--------|
| Zbyt długa sekcja | Missed deadlines |
| Blokowanie w sekcji | System się zawiesza |
| Brak sekcji | Race condition |

______________________________________________________________________

# Lekcja 11: Obsługa Przerwań (ISR) w Rust

## Problem

Przerwania (ISR) muszą być szybkie i nieblokujące. W Rust mamy bezpieczne narzędzia. [7]

## Rozwiązanie

```rust
#![no_std]
#![feature(abi_cortex_m)]

use cortex_m_rt::interrupt;

static COUNTER: core::sync::atomic::AtomicU32 = 
    core::sync::atomic::AtomicU32::new(0);

// Handler przerwania
#[interrupt]
fn TIM2() {
    // Inkrementuj licznik atomowo
    COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}

// Bezpieczny dostęp do danych z ISR
fn get_counter() -> u32 {
    COUNTER.load(core::sync::atomic::Ordering::Acquire)
}
```

## ISR + Kolejka

```rust
use crate::spsc_queue::SpscQueue;

static QUEUE: SpscQueue<u32, 64> = SpscQueue::new();

#[interrupt]
fn UART1() {
    let data = read_uart_data(); // Odczyt z UART
    
    // Bezpiecznie dodaj do kolejki
    unsafe { QUEUE.push(data); }
}

fn main() {
    loop {
        if let Some(data) = QUEUE.pop() {
            // Przetwórz dane
        }
    }
}
```

## Zasady ISR

| Zasada | Dlaczego |
|--------|----------|
| Krótki czas | Nie blokuje innych |
| Nieblokujący | Brak mutex/sem |
| Atomowy dostęp | Atomics lub CriticalSection |
| Tylko essential | Tylko konieczne operacje |

______________________________________________________________________

# Lekcja 12: Alokacja Pamięci w Embedded

## Problem

W embedded nie mamy `std::alloc`. Pamięć jest ograniczona! [1]

## Rozwiązanie: Static Allocation

```rust
#![no_std]

// Statyczna alokacja - znany rozmiar w kompilacji
static BUFFER: [u8; 1024] = [0; 1024];
static mut SHARED_DATA: u32 = 0;

// Alokacja na stosie w no_std
fn process_data(data: &[u8; 256]) {
    // Na stosie - automatyczne zarządzanie
    let mut local = *data;
}
```

## Box bez Std

```rust
#![feature(global_allocator)]
#![feature(alloc_error_handler)]

use alloc::boxed::Box;

#[global_allocator]
static ALLOC: some_allocator::Allocator = some_allocator::Allocator::new();

fn create_data() -> Box<u32> {
    Box::new(42) // Działa z alloc!
}
```

## Heapless

```rust
use heapless::Vec;
use heapless::String;

// Vec ze stałym rozmiarem
let mut data: Vec<u8, 256> = Vec::new();
data.push(1).ok();
data.push(2).ok();

// String stałej długości
let mut s: String<64> = String::new();
s.push_str("Hello").ok();
```

## Typowe Błędy

| Błąd | Skutek |
|-------|--------|
| Box bez alloc | Kompilacja nie powiedzie |
| Za duży stos | Stack overflow |
| Fragmentacja | Brak pamięci |

______________________________________________________________________

# Lekcja 13: Zero-Cost Abstractions

## Problem

Chcemy abstrakcyjny kod bez narzutu wydajności! [1]

## Rozwiązanie

Iteratory w Rust kompilują się do identycznego kodu jak pętle C!

```rust
// Abstrakcyjnie - ładnie
fn sum_even(numbers: &[i32]) -> i32 {
    numbers.iter()
        .filter(|x| *x % 2 == 0)
        .sum()
}

// Kompiluje się do prawie identycznego kodu jak:
fn sum_even_c(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            sum += numbers[i];
        }
    }
    sum
}
```

## Benchmark

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_iter(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).collect();
    
    c.bench_function("iter", |b| {
        b.iter(|| sum_even(&data))
    });
}
```

## Wynik

| Metoda | Rozmiar kodu | Czas |
|--------|--------------|------|
| Iterator | 100 bytes | 10 ns |
| Pętla | 98 bytes | 10 ns |
| Różnica | 2% | 0% |

______________________________________________________________________

# Lekcja 14: Type-Level Programming

## Problem

Chcemy by błędy były wykrywane w czasie kompilacji, nie runtime! [5]

## Rozwiązanie: Typy jako Wartości

```rust
// Typy reprezentujące wartości
trait NonZero {}
struct NonZero<const N: u32>;

impl<const N: u32> NonZero<{N}> {
    const VALUE: u32 = N;
}

// Funkcja wymagająca dodatniej liczby
fn process<T: NonZero<{ N }>, const N: u32>() {
    println!("Processing with N = {}", N);
}

// To się nie skompiluje - N = 0!
process::<NonZero<0>, 0>(); // Compile error!

// To się skompiluje - N = 5!
process::<NonZero<5>, 5>();
```

## Deadline > Period

```rust
// Typy gwarantujące poprawność
struct Period(u32);
struct Deadline(u32);

impl Deadline {
    fn is_valid(&self, period: Period) -> bool {
        self.0 >= period.0
    }
}

fn schedule(period: Period, deadline: Deadline) {
    assert!(deadline.is_valid(period), "Deadline < Period!");
}
```

## Typowe Błędy

| Błąd | Rozwiązanie |
|-------|-------------|
| Runtime validation | Type-level constants |
| Invalid state | Newtype pattern |
| Magic numbers | Const generics |

______________________________________________________________________

# Lekcja 15: Bezpieczeństwo Pamięci

## Problem

W C łatwo o buffer overflow, use-after-free, dangling pointers. W Rust to niemożliwe! [1]

## Rozwiązanie: Ownership

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 "umiera" - ownership przejdzie do s2
    
    // println!("{}", s1); // BŁĄD! s1 już nie istnieje!
    println!("{}", s2); // OK!
}

// Funkcja przejmuje ownership
fn consume(s: String) {
    println!("{}", s);
} // s zostaje usunięte

// Funkcja pożycza (borrow)
fn borrow(s: &String) -> usize {
    s.len()
} // s nadal istnieje
```

## Borrow Checker

```rust
fn bad() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // BŁĄD! r1 i r2 są aktywne!
    
    println!("{} {} {}", r1, r2, r3);
}

fn good() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    
    let r3 = &mut s; // OK - poprzednie referencje się skończyły
    println!("{}", r3);
}
```

## Porównanie

| Bug | C | Rust |
|-----|---|------|
| Buffer overflow | Możliwy | Niemożliwy |
| Use after free | Możliwy | Niemożliwy |
| Null dereference | Możliwy | Option<T> |
| Double free | Możliwy | Niemożliwy |

______________________________________________________________________

# Lekcja 16: Wprowadzenie do Embassy (Async RTOS)

## Problem

Chcesz async/await ale w embedded? Embassy to rozwiązanie! [6]

## Rozwiązanie

```rust
#![no_std]
#![feature(type_alias_impl_trait)]

use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy::gpio::{Output, Pin};

// Prosty task - miganie LED
#[embassy::task]
async fn blink_led(mut led: Output<'static, PIN_1>) {
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;
        led.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }
}

// Task odczytu sensora
#[embassy::task]
async fn read_sensor() -> u32 {
    let mut adc = ADC::new();
    loop {
        let value = adc.read();
        Timer::after(Duration::from_millis(10)).await;
        yield value;
    }
}

#[main]
async fn main(_spawner: Spawner) {
    let p = embassy::init().await;
    
    // Spawn tasks
    // _spawner.spawn(blink_led(p.PIN1)).await;
}
```

## Funkcje Embassy

| Funkcja | Opis |
|---------|------|
| Time | Timer, Ticker, Timeout |
| GPIO | Input, Output, Interrupt |
| UART | Async read/write |
| SPI/I2C | Async transfer |
| USB | Device/Host |

## Zalety

- Zero-cost futures
- Preemptive multitasking
- Deterministic scheduling
- Small footprint
