// ============================================================================
// #RACE_001 - Race Condition na zmiennej współdzielonej
// ============================================================================
//
// OPIS: Wielowątkowy dostęp do zmiennej bez synchronizacji prowadzi
// do nieprzewidywalnych wyników.
//
// OBJAWY:
// - Różne wyniki przy każdym uruchomieniu
// - Błędne sumy/liczniki
// - Trudne do reprodukcji błędy
//
// NORMY: ISO 26262 ASIL-B, DO-178C DAL-B
//
// ============================================================================

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// Demonstracja race condition - niepoprawna implementacja
pub fn race_condition_demo() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Race condition: read-modify-write bez atomowości
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    println!("Wynik: {} (oczekiwano: 100000)", result);
    // Z Mutex wynik będzie poprawny, ale pokażemy atomic
}

/// Demonstracja prawdziwej race condition bez synchronizacji
/// UWAGA: To jest unsafe i tylko dla demonstracji!
pub fn race_condition_unsafe_demo() {
    use std::cell::UnsafeCell;

    struct UnsafeCounter(UnsafeCell<usize>);
    unsafe impl Sync for UnsafeCounter {}

    let counter = Arc::new(UnsafeCounter(UnsafeCell::new(0)));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..10000 {
                unsafe {
                    // Race condition! Wielu wątków pisze bez synchronizacji
                    let ptr = counter.0.get();
                    *ptr = *ptr + 1;
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let result = unsafe { *counter.0.get() };
    println!("Wynik (z race): {} (oczekiwano: 100000)", result);
    // Wynik będzie MNIEJSZY niż 100000 przez race condition!
}

/// ROZWIĄZANIE 1: Atomic operations
pub fn race_condition_solution_atomic() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Atomic - bezpieczne!
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let result = counter.load(Ordering::SeqCst);
    println!("Wynik (atomic): {} (oczekiwano: 100000)", result);
    assert_eq!(result, 100000);
}

/// ROZWIĄZANIE 2: Mutex z proper scoping
pub fn race_condition_solution_mutex() {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                // Guard jest automatycznie zwalniany
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    println!("Wynik (mutex): {}", result);
    assert_eq!(result, 100000);
}

/// ROZWIĄZANIE 3: RwLock dla read-heavy workload
pub fn race_condition_solution_rwlock() {
    let data = Arc::new(RwLock::new(vec![0u32; 1000]));
    let mut handles = vec![];

    // 10 writerów
    for _ in 0..10 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            for i in 0..100 {
                let mut w = data.write().unwrap();
                w[i % 1000] += 1;
            }
        }));
    }

    // 90 readerów
    for _ in 0..90 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let r = data.read().unwrap();
                let _sum: u32 = r.iter().sum();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let result: u32 = data.read().unwrap().iter().sum();
    println!("Suma (rwlock): {}", result);
}

// ============================================================================
// #RACE_002 - Check-Then-Act Race Condition
// ============================================================================

/// Demonstracja check-then-act pattern - często spotykany błąd
pub fn check_then_act_demo() {
    let cache = Arc::new(Mutex::new(None::<String>));
    let mut handles = vec![];

    for i in 0..10 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            // Check-then-act - race condition!
            // Problem: sprawdzenie i akcja to dwie operacje
            {
                let guard = cache.lock().unwrap();
                if guard.is_none() {
                    // MIĘDZY TYMI DWIEMA LINIAMI inny wątek może wejść!
                    // To jest race condition
                }
            }

            // "Rozwiązanie" - wszystko w jednym locku
            let mut guard = cache.lock().unwrap();
            if guard.is_none() {
                *guard = Some(format!("Thread {}", i));
                println!("Thread {} initialized cache", i);
            } else {
                println!("Thread {} found existing: {}", i, guard.as_ref().unwrap());
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

/// Rozwiązanie: Once cell dla jednokrotnej inicjalizacji
pub fn check_then_act_solution_once() {
    use std::sync::Once;

    static mut VALUE: Option<String> = None;
    static INIT: Once = Once::new();

    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            INIT.call_once(|| {
                unsafe { VALUE = Some(format!("Initialized by thread {}", i)); }
            });

            let val = unsafe { VALUE.as_ref().unwrap() };
            println!("Thread {} sees: {}", i, val);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

/// Nowoczesne rozwiązanie: std::sync::OnceLock (Rust 1.70+)
pub fn check_then_act_solution_once_lock() {
    use std::sync::OnceLock;

    static VALUE: OnceLock<String> = OnceLock::new();

    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let val = VALUE.get_or_init(|| format!("Initialized by thread {}", i));
            println!("Thread {} sees: {}", i, val);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

// ============================================================================
// #RACE_003 - Lazy Initialization Race
// ============================================================================

/// Niepoprawna lazy initialization - double-checked locking bez atomowości
pub fn lazy_init_wrong() {
    use std::cell::UnsafeCell;

    struct Lazy<T> {
        initialized: UnsafeCell<bool>,
        value: UnsafeCell<Option<T>>,
    }

    unsafe impl<T: Send> Sync for Lazy<T> {}

    // To jest NIEBEZPIECZNE - data race na 'initialized'
}

/// Poprawna lazy initialization z OnceLock
pub fn lazy_init_correct() {
    use std::sync::OnceLock;

    fn get_expensive_value() -> &'static Vec<u64> {
        static CACHE: OnceLock<Vec<u64>> = OnceLock::new();
        CACHE.get_or_init(|| {
            println!("Computing expensive value...");
            (0..1000).filter(|n| n % 7 == 0).collect()
        })
    }

    let mut handles = vec![];
    for i in 0..5 {
        handles.push(thread::spawn(move || {
            let val = get_expensive_value();
            println!("Thread {} got {} items", i, val.len());
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    // "Computing expensive value..." pojawi się tylko RAZ!
}

// ============================================================================
// TESTY
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_solution() {
        race_condition_solution_atomic();
    }

    #[test]
    fn test_mutex_solution() {
        race_condition_solution_mutex();
    }

    #[test]
    fn test_once_lock_solution() {
        check_then_act_solution_once_lock();
    }

    #[test]
    fn test_lazy_init() {
        lazy_init_correct();
    }
}