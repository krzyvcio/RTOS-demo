# Lekcja 17: Mutex w Rust (Wzajemne Wykluczanie)

## Problem

Gdy wiele wątków chce używać tego samego zasobu, potrzebujemy mechanizmu synchronizacji. Mutex (Mutual Exclusion) blokuje dostęp dla innych. [1]

## Rozwiązanie

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    
    let handles: Vec<_> = (0..5).map(|_| {
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
    }).collect();
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("Wynik: {}", *counter.lock().unwrap());
}
```

## Jak Działa Mutex

```
Wątek A: lock() -> Zablokowany? -> Nie -> Wejdź -> lock()
Wątek B: lock() -> Zablokowany? -> Tak -> Czekaj...
Wątek A: unlock() -> Odblokuj B
Wątek B: lock() -> Teraz wejdź -> unlock()
```

## W Embedded (no_std)

```rust
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

struct Mutex {
    locked: AtomicBool,
}

impl Mutex {
    const fn new() -> Self {
        Self { locked: AtomicBool::new(false) }
    }
    
    fn lock(&self) {
        while self.locked.compare_exchange(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_err() {
            // Czekaj aż będzie dostępny
            cortex_m::asm::wfi();
        }
    }
    
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
```

## Typowe Błędy

| Błąd | Rozwiązanie |
|-------|-------------|
| Deadlock | Zawsze odblokuj |
| Race condition | Używaj lock() |
| Forever wait | Używaj try_lock() |

## Źródła

[1] https://ubuntu.com/blog/real-time-linux-vs-rtos-2
