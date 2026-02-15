//! ============================================================================
//! PRZYKŁAD #11: DEADLOCK - Klasyczny AB-BA Deadlock
//! ============================================================================
//!
//! KATEGORIA: Synchronizacja > Deadlock
//! POZIOM: Początkujący
//!
//! OPIS:
//! Dwa wątki próbują zdobyć dwa mutexy w różnej kolejności.
//! Wątek A: lock(A) -> lock(B)
//! Wątek B: lock(B) -> lock(A)
//!
//! ZAGROŻENIE:
//! - Zakleszczenie (deadlock)
//! - System zawieszony
//! - Brak możliwości odzyskania
//!
//! ROZWIĄZANIE:
//! 1. Zawsze blokuj mutexy w tej samej kolejności
//! 2. Użyj try_lock() z timeout
//! 3. Użyj lock-free structures
//!
//! URUCHOMIENIE:
//! cargo run --bin deadlock_01
//! ============================================================================

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  DEADLOCK #11: Klasyczny AB-BA Deadlock                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let mutex_a = Arc::new(Mutex::new(0));
    let mutex_b = Arc::new(Mutex::new(0));

    // Wątek 1: A -> B
    let a1 = Arc::clone(&mutex_a);
    let b1 = Arc::clone(&mutex_b);
    let handle1 = thread::spawn(move || {
        println!("[WĄTEK 1] Próbuję zablokować A...");
        let _guard_a = a1.lock().unwrap();
        println!("[WĄTEK 1] ✓ Zablokowałem A");

        thread::sleep(Duration::from_millis(100)); // Daj czas wątkowi 2

        println!("[WĄTEK 1] Próbuję zablokować B...");
        let _guard_b = b1.lock().unwrap(); // ⚠️ DEADLOCK!
        println!("[WĄTEK 1] ✓ Zablokowałem B");
    });

    // Wątek 2: B -> A (odwrotna kolejność!)
    let a2 = Arc::clone(&mutex_a);
    let b2 = Arc::clone(&mutex_b);
    let handle2 = thread::spawn(move || {
        println!("[WĄTEK 2] Próbuję zablokować B...");
        let _guard_b = b2.lock().unwrap();
        println!("[WĄTEK 2] ✓ Zablokowałem B");

        thread::sleep(Duration::from_millis(100)); // Daj czas wątkowi 1

        println!("[WĄTEK 2] Próbuję zablokować A...");
        let _guard_a = a2.lock().unwrap(); // ⚠️ DEADLOCK!
        println!("[WĄTEK 2] ✓ Zablokowałem A");
    });

    println!();
    println!("⚠️  Oczekuję na zakończenie wątków...");
    println!("   (Naciśnij Ctrl+C jeśli program się zawiesi)");
    println!();

    // To się NIE zakończy!
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("✓ Program zakończony (to się nie wyświetli)");
}

// ============================================================================
// DIAGNOZA:
// ============================================================================
//
// Objawy deadlocku:
// 1. Program zawieszony
// 2. Wątki w stanie WAIT
// 3. Brak postępu
//
// Jak wykryć:
// 1. `gdb` - pstack
// 2. `strace` - futex_wait
// 3. Thread sanitizers
// 4. Deadlock detection tools
//
// ============================================================================