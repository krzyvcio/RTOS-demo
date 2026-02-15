//! ============================================================================
//! PRZYKŁAD #26: RACE CONDITION - Data Race
//! ============================================================================
//!
//! KATEGORIA: Synchronizacja > Race Conditions
//! POZIOM: Początkujący
//!
//! OPIS:
//! Wielu producentów i konsumentów współdzieli licznik bez synchronizacji.
//! Wynik jest niedeterministyczny i błędny.
//!
//! ZAGROŻENIE:
//! - Utrata danych
//! - Niespójność
//! - Niedeterministyczne zachowanie
//!
//! URUCHOMIENIE:
//! cargo run --bin race_condition_01
//! ============================================================================

use std::sync::Arc;
use std::thread;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  RACE CONDITION #26: Data Race                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Współdzielony licznik BEZ synchronizacji
    let counter = Arc::new(std::cell::UnsafeCell::new(0u64));

    const NUM_THREADS: usize = 10;
    const INCREMENTS: u64 = 1_000_000;

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|id| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..INCREMENTS {
                    // ⚠️ RACE CONDITION! Niezsynchronizowany dostęp
                    unsafe {
                        let c = &mut *counter.get();
                        *c += 1; // READ-MODIFY-WRITE bez atomowości
                    }
                }
                println!("[WĄTEK {}] Zakończyłem", id);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = unsafe { *counter.get() };
    let expected = NUM_THREADS as u64 * INCREMENTS;

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  WYNIKI:                                                    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Oczekiwano: {:>15}                              ║", expected);
    println!("║  Otrzymano:  {:>15}                              ║", final_count);
    println!("║  Różnica:    {:>15} ({:.1}% błędnych)            ║",
             expected.saturating_sub(final_count),
             100.0 * (expected - final_count) as f64 / expected as f64);
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("⚠️  Utracono {} inkrementacji z powodu race condition!",
             expected.saturating_sub(final_count));
}

// ============================================================================
// WYJAŚNIENIE:
// ============================================================================
//
// Operacja *c += 1 składa się z trzech kroków:
// 1. READ:  Wczytaj wartość z pamięci
// 2. MODIFY: Dodaj 1
// 3. WRITE:  Zapisz wynik
//
// Jeśli dwa wątki wykonają te kroki równolegle:
//
// Wątek A: READ (0) → MODIFY (1) → WRITE (1)
// Wątek B:    READ (0) → MODIFY (1) → WRITE (1)
//
// Wynik: 1 (powinno być 2!)
//
// ============================================================================