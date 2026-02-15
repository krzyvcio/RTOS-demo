# Lekcja 3: Atomowe Operacje w Rust

## Problem

Wielowątkowość wymaga synchronizacji. Zmienne współdzielone muszą być atomowe - musisz wiedzieć kiedy zmiana jest widoczna dla innych wątków. [1]

## Rozwiązanie

Użyj `std::sync::atomic` - typy które gwarantują atomowość operacji.

```rust
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);
static FLAGS: AtomicBool = AtomicBool::new(false);

fn increment() {
    // Atomowe dodawanie - zwraca starą wartość
    let old = COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Stara wartość: {}, nowa: {}", old, COUNTER.load(Ordering::SeqCst));
}

fn compare_exchange() {
    // Atomowe porównanie i zamiana
    let result = COUNTER.compare_exchange(
        5,          // oczekiwana wartość
        10,         // nowa wartość
        Ordering::SeqCst,  // success
        Ordering::SeqCst   // failure
    );
    
    match result {
        Ok(old) => println!("Zamieniono {} na 10", old),
        Err(current) => println!("Oczekiwano 5, ale jest {}", current),
    }
}

fn load_store() {
    // Prosty atomowy odczyt i zapis
    FLAGS.store(true, Ordering::SeqCst);
    
    if FLAGS.load(Ordering::SeqCst) {
        println!("Flag jest ustawiony!");
    }
}
```

## Typy atomowe

| Typ | Rozmiar | Użycie |
|-----|---------|--------|
| `AtomicBool` | 1 byte | Flagi |
| `AtomicU8/U16/U32/U64` | odpowiednio | Liczniki |
| `AtomicUsize` | zależy od arch | Wskaźniki |
| `AtomicPtr<T>` | zależy od arch | Wskaźniki |

## Ordering - co wybrać?

| Ordering | Przypadek użycia | Narzut |
|----------|------------------|--------|
| `Relaxed` | Liczniki, brak synchronizacji | Minimalny |
| `Acquire` | Odczyt po write Release | Mały |
| `Release` | Zapis przed read Acquire | Mały |
| `SeqCst` | Gdy potrzebujesz gwarancji | Największy |

## Typowe błędy

| Błąd | Skutek |
|-------|--------|
| Zły ordering | Race condition |
| Mieszany ordering | Niespójne dane |
| Za mocny | Niepotrzebny narzut |

## Porównanie z C

```c
// C
volatile int counter = 0;
counter++; // nie atomowe!

// Rust - atomowe
use std::sync::atomic::AtomicU32;
static COUNTER: AtomicU32 = AtomicU32::new(0);
COUNTER.fetch_add(1, Ordering::SeqCst);
```

## Godny następca

`crossbeam::atomic::AtomicConsume` - consumed reads dla lepszej optymalizacji.

## Źródła

[1] https://ubuntu.com/blog/real-time-linux-vs-rtos-2
