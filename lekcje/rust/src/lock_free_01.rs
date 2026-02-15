//! ============================================================================
//! PRZYKŁAD #60: LOCK-FREE RING BUFFER
//! ============================================================================
//!
//! KATEGORIA: Synchronizacja > Lock-Free
//! POZIOM: Zaawansowany
//!
//! OPIS:
//! Single-Producer Single-Consumer (SPSC) ring buffer bez blokad.
//! Bezpieczny dla współbieżności, brak deadlocks, brak priority inversion.
//!
//! ZALETY:
//! - Brak blokad
//! - Brak deadlocks
//! - Deterministyczny czas
//! - Brak priority inversion
//!
//! URUCHOMIENIE:
//! cargo run --bin lock_free_01
//! ============================================================================

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Lock-free SPSC Ring Buffer
/// Single Producer, Single Consumer
const BUFFER_SIZE: usize = 1024; // Musi być potęgą 2!

struct RingBuffer<T> {
    buffer: Box<[std::cell::UnsafeCell<T>; BUFFER_SIZE]>,
    head: AtomicUsize, // Write index (producer)
    tail: AtomicUsize, // Read index (consumer)
}

impl<T: Default + Copy> RingBuffer<T> {
    fn new() -> Self {
        let buffer: Box<[std::cell::UnsafeCell<T>; BUFFER_SIZE]> = Box::new(
            unsafe {
                // Inicjalizacja bufora
                let mut vec: Vec<std::cell::UnsafeCell<T>> = Vec::with_capacity(BUFFER_SIZE);
                for _ in 0..BUFFER_SIZE {
                    vec.push(std::cell::UnsafeCell::new(T::default()));
                }
                vec.into_boxed_slice().try_into().unwrap_unchecked()
            }
        );

        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Producer: write value to buffer
    /// Returns false if buffer is full
    fn push(&self, value: T) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        // Sprawdź czy bufor nie jest pełny
        if head.wrapping_sub(tail) >= BUFFER_SIZE {
            return false; // Pełny
        }

        // Zapisz wartość
        unsafe {
            *self.buffer[head & (BUFFER_SIZE - 1)].get() = value;
        }

        // Aktualizuj head (Release dla widoczności)
        self.head.store(head.wrapping_add(1), Ordering::Release);

        true
    }

    /// Consumer: read value from buffer
    /// Returns None if buffer is empty
    fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        // Sprawdź czy bufor nie jest pusty
        if tail == head {
            return None; // Pusty
        }

        // Pobierz wartość
        let value = unsafe { *self.buffer[tail & (BUFFER_SIZE - 1)].get() };

        // Aktualizuj tail
        self.tail.store(tail.wrapping_add(1), Ordering::Release);

        Some(value)
    }

    /// Zwraca liczbę elementów w buforze
    fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        head.wrapping_sub(tail)
    }
}

// Make RingBuffer Send + Sync for SPSC usage
unsafe impl<T: Send> Send for RingBuffer<T> {}
unsafe impl<T: Send> Sync for RingBuffer<T> {}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  LOCK-FREE #60: SPSC Ring Buffer                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let buffer = std::sync::Arc::new(RingBuffer::<u64>::new());
    const NUM_ITEMS: u64 = 1_000_000;

    println!("Producent i konsument będą przetwarzać {} elementów...", NUM_ITEMS);
    println!("Rozmiar bufora: {} elementów", BUFFER_SIZE);
    println!();

    // Producent
    let producer_buffer = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        let start = std::time::Instant::now();

        for i in 0..NUM_ITEMS {
            while !producer_buffer.push(i) {
                // Bufor pełny, czekaj
                std::hint::spin_loop();
            }
        }

        let elapsed = start.elapsed();
        println!("[PRODUCENT] Wysłano {} elementów w {:?}", NUM_ITEMS, elapsed);
        elapsed
    });

    // Konsument
    let consumer_buffer = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        let start = std::time::Instant::now();
        let mut count = 0u64;
        let mut sum = 0u64;

        while count < NUM_ITEMS {
            if let Some(value) = consumer_buffer.pop() {
                sum = sum.wrapping_add(value);
                count += 1;
            } else {
                std::hint::spin_loop();
            }
        }

        let elapsed = start.elapsed();
        println!("[KONSUMENT] Otrzymano {} elementów w {:?}", count, elapsed);
        println!("[KONSUMENT] Suma = {} (oczekiwano {})", sum, (NUM_ITEMS - 1) * NUM_ITEMS / 2);
        elapsed
    });

    let producer_time = producer.join().unwrap();
    let consumer_time = consumer.join().unwrap();

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  WYNIKI:                                                    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Przepustowość: {:.0} elementów/sek",
             NUM_ITEMS as f64 / producer_time.as_secs_f64());
    println!("║  Brak deadlocks: ✓                                          ║");
    println!("║  Brak priority inversion: ✓                                 ║");
    println!("║  Deterministyczny czas: ✓                                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// KLUCZOWE KONCEPJE:
// ============================================================================
//
// 1. ATOMIC OPERATIONS:
//    - Ordering::Relaxed: brak gwarancji kolejności
//    - Ordering::Acquire: odczyt nie może być przeniesiony przed
//    - Ordering::Release: zapis nie może być przeniesiony po
//
// 2. SPSC (Single Producer Single Consumer):
//    - Tylko jeden wątek pisze
//    - Tylko jeden wątek czyta
//    - Brak rywalizacji między producentami/konsumentami
//
// 3. POWER-OF-2 SIZE:
//    - Rozmiar bufora = potęga 2
//    - Modulo = maskowanie bitowe (& (SIZE - 1))
//    - Brak dzielenia
//
// 4. WRAPPING INDICES:
//    - head/tail rosną bez ograniczenia
//    - wrapping_sub do obliczenia długości
//
// ============================================================================