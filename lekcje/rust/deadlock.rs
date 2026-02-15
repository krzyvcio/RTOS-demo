// ============================================================================
// #DEADLOCK_001 - Klasyczny AB-BA Deadlock
// ============================================================================
//
// OPIS: Dwa wątki blokują mutexy w przeciwnej kolejności, prowadząc do
// zakleszczenia. Klasyczny przykład z literatury.
//
// OBJAWY:
// - Program zawiesza się po losowym czasie
// - CPU może być 100% lub 0%
// - Czasami działa, czasami nie (race condition)
//
// NORMY: ISO 26262 ASIL-D, DO-178C DAL-A
//
// ============================================================================

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Demonstracja klasycznego deadlocku AB-BA
///
/// Wątek A: lock(A) -> lock(B)
/// Wątek B: lock(B) -> lock(A)
///
/// Jeśli A trzyma A i B trzyma B -> DEADLOCK
pub fn deadlock_ab_ba_demo() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(1));

    // Wątek 1: Lock A, potem B
    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = thread::spawn(move || {
        println!("[Wątek 1] Próba lock(A)...");
        let _guard_a = a1.lock().unwrap();
        println!("[Wątek 1] lock(A) - OK, czekam...");
        thread::sleep(Duration::from_millis(100)); // Daj czas wątkowi 2

        println!("[Wątek 1] Próba lock(B)...");
        let _guard_b = b1.lock().unwrap(); // <- ZAWIESZENIE!
        println!("[Wątek 1] lock(B) - OK"); // Nie wykona się
    });

    // Wątek 2: Lock B, potem A (przeciwna kolejność!)
    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let h2 = thread::spawn(move || {
        println!("[Wątek 2] Próba lock(B)...");
        let _guard_b = b2.lock().unwrap();
        println!("[Wątek 2] lock(B) - OK, czekam...");
        thread::sleep(Duration::from_millis(100));

        println!("[Wątek 2] Próba lock(A)...");
        let _guard_a = a2.lock().unwrap(); // <- ZAWIESZENIE!
        println!("[Wątek 2] lock(A) - OK"); // Nie wykona się
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

/// ROZWIĄZANIE 1: Zawsze blokuj mutexy w tej samej kolejności
pub fn deadlock_solution_ordering() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(1));

    // Oba wątki blokują w tej samej kolejności: A, potem B
    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = thread::spawn(move || {
        let _guard_a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _guard_b = b1.lock().unwrap(); // OK - kolejność zachowana
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let h2 = thread::spawn(move || {
        let _guard_a = a2.lock().unwrap(); // Ta sama kolejność!
        thread::sleep(Duration::from_millis(100));
        let _guard_b = b2.lock().unwrap();
    });

    h1.join().unwrap();
    h2.join().unwrap();
    println!("SUKCES: Brak deadlock dzięki spójnej kolejności");
}

/// ROZWIĄZANIE 2: Użyj try_lock z timeout
pub fn deadlock_solution_try_lock() {
    use std::time::Instant;

    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(1));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = thread::spawn(move || {
        loop {
            let guard_a = a1.try_lock();
            if let Ok(_g) = guard_a {
                thread::sleep(Duration::from_millis(10));
                let guard_b = b1.try_lock();
                if let Ok(_g) = guard_b {
                    println!("[Wątek 1] Sukces - oba locki");
                    return;
                }
                // Nie udało się lock(B), zwolnij A i spróbuj ponownie
                println!("[Wątek 1] try_lock(B) failed, retry...");
                drop(guard_a);
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let h2 = thread::spawn(move || {
        loop {
            let guard_b = b2.try_lock();
            if let Ok(_g) = guard_b {
                thread::sleep(Duration::from_millis(10));
                let guard_a = a2.try_lock();
                if let Ok(_g) = guard_a {
                    println!("[Wątek 2] Sukces - oba locki");
                    return;
                }
                println!("[Wątek 2] try_lock(A) failed, retry...");
                drop(guard_b);
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

/// ROZWIĄZANIE 3: Użyj pojedynczego mutex dla grupy zasobów
pub fn deadlock_solution_single_mutex() {
    #[derive(Default)]
    struct Resources {
        a: i32,
        b: i32,
    }

    let resources = Arc::new(Mutex::new(Resources::default()));

    let r1 = Arc::clone(&resources);
    let h1 = thread::spawn(move || {
        let mut guard = r1.lock().unwrap();
        guard.a += 1;
        guard.b += 1;
    });

    let r2 = Arc::clone(&resources);
    let h2 = thread::spawn(move || {
        let mut guard = r2.lock().unwrap();
        guard.a += 1;
        guard.b += 1;
    });

    h1.join().unwrap();
    h2.join().unwrap();
    println!("SUKCES: Brak deadlock - pojedynczy mutex");
}

// ============================================================================
// #DEADLOCK_002 - Self-Deadlock (Recursive Lock Attempt)
// ============================================================================

/// Demonstracja self-deadlock - próba ponownego zablokowania mutexa
/// przez ten sam wątek
///
/// W Rust Mutex nie jest reentrant, więc to spowoduje panic lub zawieszenie
pub fn self_deadlock_demo() {
    let mutex = Arc::new(Mutex::new(42));

    let guard = mutex.lock().unwrap();
    println!("Pierwszy lock - OK");

    // Próba ponownego lock - PANIC w Rust!
    // let guard2 = mutex.lock().unwrap(); // <- PANIC: deadlock

    drop(guard); // Zwolnij pierwszy
    println!("Self-deadlock uniknięty przez drop()");
}

/// Rozwiązanie: Użyj RwLock dla odczytu lub reorganizuj kod
pub fn self_deadlock_solution() {
    use std::sync::RwLock;

    let lock = Arc::new(RwLock::new(42));

    // Wielokrotny odczyt jest OK
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    println!("Odczyt 1: {}", *r1);
    println!("Odczyt 2: {}", *r2);

    drop(r1);
    drop(r2);

    // Teraz można zapisać
    let mut w = lock.write().unwrap();
    *w = 100;
}

// ============================================================================
// #DEADLOCK_003 - Deadlock z 3 Mutexami
// ============================================================================

/// Deadlock z trzema mutexami - jeszcze bardziej złożony przypadek
pub fn deadlock_three_mutex_demo() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(1));
    let c = Arc::new(Mutex::new(2));

    // Wątek 1: A -> B
    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = thread::spawn(move || {
        let _g1 = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _g2 = b1.lock().unwrap(); // Czeka na B
    });

    // Wątek 2: B -> C
    let b2 = Arc::clone(&b);
    let c2 = Arc::clone(&c);
    let h2 = thread::spawn(move || {
        let _g1 = b2.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _g2 = c2.lock().unwrap(); // Czeka na C
    });

    // Wątek 3: C -> A
    let a3 = Arc::clone(&a);
    let c3 = Arc::clone(&c);
    let h3 = thread::spawn(move || {
        let _g1 = c3.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _g2 = a3.lock().unwrap(); // Czeka na A - DEADLOCK!
    });

    // Wszystkie trzy wątki są w deadlock
    // h1.join().unwrap(); // Nigdy się nie zakończy
}

// ============================================================================
// #DEADLOCK_004 - Deadlock z Condition Variable
// ============================================================================

pub fn deadlock_condvar_demo() {
    use std::sync::Condvar;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let (lock, cvar) = &*pair;

    // Wątek czekający na sygnał
    let pair_clone = Arc::clone(&pair);
    let h = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        while !*started {
            // Jeśli tutaj nastąpi panic, lock nie zostanie zwolniony!
            started = cvar.wait(started).unwrap();
        }
    });

    // Problematyczne: zapomnieliśmy powiadomić!
    // cvar.notify_one(); // <- Brak tego powoduje wieczne czekanie

    // h.join().unwrap(); // Nigdy się nie zakończy
}

// ============================================================================
// TESTY
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ordering() {
        deadlock_solution_ordering();
    }

    #[test]
    fn test_solution_single_mutex() {
        deadlock_solution_single_mutex();
    }

    // UWAGA: Nie uruchamiaj testu deadlock_ab_ba_demo() - zawiesi się!
}