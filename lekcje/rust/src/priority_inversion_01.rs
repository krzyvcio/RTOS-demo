//! ============================================================================
//! PRZYKŁAD #01: PRIORITY INVERSION
//! ============================================================================
//!
//! KATEGORIA: Synchronizacja > Priority Inversion
//! POZIOM: Średniozaawansowany
//!
//! OPIS:
//! Klasyczny problem inwersji priorytetów:
//! - Zadanie NISKIE trzyma mutex
//! - Zadanie WYSOKIE czeka na mutex
//! - Zadanie ŚREDNIE wywłaszcza NISKIE
//! - WYSOKIE czeka na ŚREDNIE (niski priorytet blokuje wysoki!)
//!
//! ROZWIĄZANIE:
//! Priority Inheritance Protocol (PIP)
//!
//! URUCHOMIENIE:
//! cargo run --bin priority_inversion_01
//! ============================================================================

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

struct Task {
    name: String,
    priority: Priority,
}

impl Task {
    fn new(name: &str, priority: Priority) -> Self {
        Self {
            name: name.to_string(),
            priority,
        }
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  PRIORITY INVERSION #01: Klasyczny przykład                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("SCENARIUSZ:");
    println!("1. Zadanie NISKIE (priorytet 1) zdobywa mutex");
    println!("2. Zadanie WYSOKIE (priorytet 3) chce mutex → CZEKA");
    println!("3. Zadanie ŚREDNIE (priorytet 2) wywłaszcza NISKIE");
    println!("4. WYSOKIE czeka na ŚREDNIE → INWERSJA!");
    println!();
    println!("Symulacja:");
    println!();

    let shared_resource = Arc::new(Mutex::new(0));
    let start_time = Arc::new(Instant::now());

    // Zadanie NISKIE
    let resource_low = Arc::clone(&shared_resource);
    let time_low = Arc::clone(&start_time);
    let low_handle = thread::spawn(move || {
        println!("[{:>4}ms] [NISKI ] Startuję", time_low.elapsed().as_millis());

        let _guard = resource_low.lock().unwrap();
        println!("[{:>4}ms] [NISKI ] Mam mutex!", time_low.elapsed().as_millis());

        // Symuluj długą pracę
        thread::sleep(Duration::from_millis(200));

        println!("[{:>4}ms] [NISKI ] Zwalniam mutex", time_low.elapsed().as_millis());
    });

    // Krótkie opóźnienie, żeby NISKI zdążył zdobyć mutex
    thread::sleep(Duration::from_millis(50));

    // Zadanie WYSOKIE
    let resource_high = Arc::clone(&shared_resource);
    let time_high = Arc::clone(&start_time);
    let high_handle = thread::spawn(move || {
        println!("[{:>4}ms] [WYSOKI] Startuję", time_high.elapsed().as_millis());

        println!("[{:>4}ms] [WYSOKI] Chcę mutex...", time_high.elapsed().as_millis());
        let wait_start = Instant::now();

        let _guard = resource_high.lock().unwrap();

        let wait_time = wait_start.elapsed();
        println!("[{:>4}ms] [WYSOKI] Mam mutex po {}ms!",
                 time_high.elapsed().as_millis(),
                 wait_time.as_millis());

        // Krótka praca
        thread::sleep(Duration::from_millis(10));

        println!("[{:>4}ms] [WYSOKI] Koniec", time_high.elapsed().as_millis());
    });

    // Zadanie ŚREDNIE (wywłaszcza NISKI)
    thread::sleep(Duration::from_millis(50));

    let time_med = Arc::clone(&start_time);
    let medium_handle = thread::spawn(move || {
        println!("[{:>4}ms] [ŚREDNI] Startuję i wywłaszczam NISKI!",
                 time_med.elapsed().as_millis());

        // ŚREDNI wywłaszcza NISKI, WYSOKI nadal czeka!
        thread::sleep(Duration::from_millis(150));

        println!("[{:>4}ms] [ŚREDNI] Koniec", time_med.elapsed().as_millis());
    });

    low_handle.join().unwrap();
    medium_handle.join().unwrap();
    high_handle.join().unwrap();

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  WYNIK: INWERSJA PRIORYTETÓW!                               ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  WYSOKI priorytet czekał na ŚREDNI (niski priorytet)        ║");
    println!("║  To jest klasyczny przykład Priority Inversion             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// ROZWIĄZANIA:
// ============================================================================
//
// 1. PRIORITY INHERITANCE PROTOCOL (PIP):
//    Zadanie trzymające mutex dziedziczy priorytet czekającego
//
// 2. PRIORITY CEILING PROTOCOL (PCP):
//    Mutex ma przypisany "pułap" priorytetu
//    Zadanie trzymające mutex działa na tym pułapie
//
// 3. LOCK-FREE STRUCTURES:
//    Użyj struktur bez blokad (ring buffer, lock-free queue)
//
// 4. AVOID SHARED RESOURCES:
//    Projektuj system bez współdzielonych zasobów
//
// ============================================================================