//! ============================================================================
//! PRZYKŁAD #12: DEADLOCK - Rozwiązanie z try_lock
//! ============================================================================
//!
//! KATEGORIA: Synchronizacja > Deadlock
//! POZIOM: Średniozaawansowany
//!
//! OPIS:
//! Rozwiązanie deadlocku używając try_lock() z timeout.
//! Jeśli nie uda się zdobyć drugiego mutexa, zwalniamy pierwszy.
//!
//! URUCHOMIENIE:
//! cargo run --bin deadlock_02
//! ============================================================================

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

fn try_lock_both<T>(
    mutex_a: &Mutex<T>,
    mutex_b: &Mutex<T>,
    timeout: Duration,
) -> Option<(MutexGuard<T>, MutexGuard<T>)> {
    let start = Instant::now();

    loop {
        // Próbuj zdobyć pierwszy mutex
        if let Ok(guard_a) = mutex_a.try_lock() {
            // Teraz próbuj zdobyć drugi
            if let Ok(guard_b) = mutex_b.try_lock() {
                return Some((guard_a, guard_b));
            }
            // Nie udało się - zwolnij pierwszy i spróbuj ponownie
            drop(guard_a);
        }

        if start.elapsed() > timeout {
            return None;
        }

        // Krótka przerwa przed ponowną próbą
        thread::yield_now();
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  DEADLOCK #12: Rozwiązanie z try_lock()                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let mutex_a = Arc::new(Mutex::new(0i32));
    let mutex_b = Arc::new(Mutex::new(0i32));

    // Wątek 1: używa bezpiecznej metody
    let a1 = Arc::clone(&mutex_a);
    let b1 = Arc::clone(&mutex_b);
    let handle1 = thread::spawn(move || {
        println!("[WĄTEK 1] Próbuję zablokować A i B...");

        match try_lock_both(&*a1, &*b1, Duration::from_secs(5)) {
            Some((_guard_a, _guard_b)) => {
                println!("[WĄTEK 1] ✓ Zablokowałem oba mutexy!");
                thread::sleep(Duration::from_millis(100));
                println!("[WĄTEK 1] ✓ Praca zakończona");
            }
            None => {
                println!("[WĄTEK 1] ✗ Timeout - nie udało się zdobyć mutexów");
            }
        }
    });

    // Wątek 2: używa bezpiecznej metody
    let a2 = Arc::clone(&mutex_a);
    let b2 = Arc::clone(&mutex_b);
    let handle2 = thread::spawn(move || {
        println!("[WĄTEK 2] Próbuję zablokować B i A...");

        // Uwaga: ZAWSZE blokuj w tej samej kolejności (A, B)!
        match try_lock_both(&*a2, &*b2, Duration::from_secs(5)) {
            Some((_guard_a, _guard_b)) => {
                println!("[WĄTEK 2] ✓ Zablokowałem oba mutexy!");
                thread::sleep(Duration::from_millis(100));
                println!("[WĄTEK 2] ✓ Praca zakończona");
            }
            None => {
                println!("[WĄTEK 2] ✗ Timeout - nie udało się zdobyć mutexów");
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  LEKCJA: Zawsze blokuj mutexy w STAŁEJ KOLEJNOŚCI!         ║");
    println!("║  Lub używaj try_lock() z timeout                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// INNE ROZWIĄZANIA:
// ============================================================================
//
// 1. LOCK ORDERING:
//    Zawsze blokuj mutexy w tej samej kolejności globalnie
//    np. zawsze (A, B), nigdy (B, A)
//
// 2. LOCK HIERARCHY:
//    Przypisz poziomy mutexom, blokuj od niższego do wyższego
//
// 3. DEADLOCK DETECTION:
//    Użyj biblioteki z wykrywaniem deadlocku
//
// 4. LOCK-FREE:
//    Użyj struktur bez blokad (lock-free queues, atomics)
//
// ============================================================================