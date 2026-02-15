# Lekcja 4: Bariery Pamięci (Memory Barriers)

## Problem

Kompilator i CPU mogą przestawiać operacje dla optymalizacji. Ale czasem kolejność ma znaczenie! Musisz powiedzieć kiedy. [7]

## Rozwiązanie

Użyj `Ordering` w atomowych operacjach jako bariery.

```rust
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static READY: AtomicBool = AtomicBool::new(false);
static DATA: AtomicU32 = AtomicU32::new(0);

// Wątek A (producer)
fn producer() {
    DATA.store(42, Ordering::Release);  // Zapisz dane
    READY.store(true, Ordering::Release); // Powiedz że gotowe
}

// Wątek B (consumer)  
fn consumer() {
    // Najpierw sprawdź czy gotowe
    if READY.load(Ordering::Acquire) {
        // Teraz dane na pewno są widoczne!
        let value = DATA.load(Ordering::Relaxed);
        println!("Otrzymano: {}", value);
    }
}
```

## Jak działają bariery

```
PRODUCER                           CONSUMER
--------                           --------
DATA = 42 (Release)    -------->  if READY (Acquire) == true
                                     |
READY = true (Release)   -------->     |
                                     DATA = 42 (Relexed)
                                     
Bariera gwarantuje: DATA przed READY w czasie!
```

## Typy barier

| Funkcja | Działanie |
|---------|-----------|
| `Acquire` | Wszystkie operacje PRZED tym muszą się zakończyć |
| `Release` | Wszystkie operacje PO tym muszą się zacząć |
| `SeqCst` | Pełna synchronizacja |

## Typowe błędy

| Błąd | Skutek |
|-------|--------|
| Brak barrier | Stare/niespójne dane |
| Zły ordering | Race condition |
| Memory reorder | Nieprzewidywalne zachowanie |

## Porównanie z C

```c
// C - jawne bariery
#include <stdatomic.h>

atomic_store(&DATA, 42);
atomic_store(&READY, 1);
__asm__ __volatile__("dmb ish" ::: "memory"); // Bariera!

// Rust - bariery w ordering
DATA.store(42, Ordering::Release);
READY.store(true, Ordering::Release);
```

## Źródła

[7] https://en.wikipedia.org/wiki/PREEMPT_RT
