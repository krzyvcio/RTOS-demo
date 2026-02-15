//! ============================================================================
//! PRZYKŁAD #41: STARVATION
//! ============================================================================
//!
//! KATEGORIA: Synchronizacja > Starvation
//! POZIOM: Średniozaawansowany
//!
//! OPIS:
//! Zadanie o niskim priorytecie nigdy nie dostaje CPU,
//! bo zadania o wyższym priorytecie ciągle go wywłaszczają.
//!
//! ZAGROŻENIE:
//! - Zadanie nigdy się nie wykonuje
//! - Progress bar się nie porusza
//! - System wydaje się zawieszony
//!
//! URUCHOMIENIE:
//! cargo run --bin starvation_01
//! ============================================================================

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  STARVATION #41: Głodzenie zadania niskiego priorytetu     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let high_counter = Arc::new(AtomicUsize::new(0));
    let low_counter = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

    println!("Uruchamiam zadania przez 5 sekund...");
    println!("WYSOKI priorytet będzie głodził NISKI");
    println!();

    // Zadanie WYSOKIEGO priorytetu (wiele instancji)
    let high_handles: Vec<_> = (0..4)
        .map(|id| {
            let counter = Arc::clone(&high_counter);
            let running = Arc::clone(&running);
            thread::spawn(move || {
                while running.load(Ordering::Relaxed) {
                    // Intensywna praca
                    for _ in 0..1000 {
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                    thread::yield_now();
                }
                println!("[WYSOKI-{}] Zakończono", id);
            })
        })
        .collect();

    // Zadanie NISKIEGO priorytetu (głodzone)
    let low_counter_clone = Arc::clone(&low_counter);
    let running_clone = Arc::clone(&running);
    let low_handle = thread::spawn(move || {
        println!("[NISKI] Startuję...");
        let mut last_count = 0;
        let mut iterations = 0;

        while running_clone.load(Ordering::Relaxed) {
            low_counter_clone.fetch_add(1, Ordering::Relaxed);
            iterations += 1;

            // Raportuj co 100000 iteracji
            if iterations % 100_000 == 0 {
                let current = low_counter_clone.load(Ordering::Relaxed);
                if current != last_count {
                    println!("[NISKI] Iteracja {} (postęp!)", iterations);
                    last_count = current;
                }
            }
        }
        println!("[NISKI] Zakończono po {} iteracjach", iterations);
    });

    // Daj czas na głodzenie
    thread::sleep(Duration::from_secs(5));
    running.store(false, Ordering::Relaxed);

    // Czekaj na zakończenie
    for handle in high_handles {
        handle.join().unwrap();
    }
    low_handle.join().unwrap();

    let high_total = high_counter.load(Ordering::Relaxed);
    let low_total = low_counter.load(Ordering::Relaxed);

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  WYNIKI:                                                    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  WYSOKI priorytet: {:>12} iteracji                   ║", high_total);
    println!("║  NISKI priorytet:  {:>12} iteracji                   ║", low_total);
    println!("║  Stosunek:         {:>12.0}:1                         ║",
             high_total as f64 / low_total.max(1) as f64);
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    if low_total == 0 {
        println!("⚠️  NISKI priorytet NIE WYKONAŁ SIĘ W OGÓLE!");
    } else if high_total / low_total > 1000 {
        println!("⚠️  NISKI priorytet jest drastycznie głodzony!");
    } else {
        println!("✓ NISKI priorytet dostał trochę CPU");
    }
}

// ============================================================================
// ROZWIĄZANIA:
// ============================================================================
//
// 1. AGING (starvation prevention):
//    Zwiększaj priorytet zadania, które długo czeka
//
// 2. FAIR SCHEDULING:
//    Round-robin, Completely Fair Scheduler (CFS)
//
// 3. TIME SLICING:
//    Każde zadanie dostaje kwant czasu
//
// 4. PRIORITY BOOSTS:
//    Zwiększaj priorytet po czasie oczekiwania
//
// 5. LOTTERY SCHEDULING:
//    Losowy wybór z prawdopodobieństwem proporcjonalnym do priorytetu
//
// ============================================================================