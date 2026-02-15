# Lekcja 1: Kolejka SPSC (Lock-Free) w Rust

## Problem

W systemach czasu rzeczywistego (RTOS) tradycyjne kolejki z blokadami (mutex) powodują nieprzewidywalne opóźnienia przez kontekstowe przełączanie wątków, co łamie determinizm. [1]

## Typowe błędy

- **Race conditions** - nadpisanie elementów bez pełnego sprawdzenia
- **Overflow/underflow** - ignorowanie pełnej/pustej kolejki
- **ABI mismatch** - błędne użycie UnsafeRust bez align [2]

## Rozwiązanie

Użyj atomowych wskaźników w ring buffer dla Single Producer-Single Consumer (SPSC), z maską dla cykliczności.

```rust
use std::sync::atomic::{AtomicU32, Ordering};

pub struct SpscQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    head: AtomicU32,
    tail: AtomicU32,
}

impl<T: Copy, const N: usize> SpscQueue<T, N> {
    pub const fn new() -> Self {
        assert!(N.is_power_of_two());
        Self {
            buffer: [const { None }; N],
            head: AtomicU32::new(0),
            tail: AtomicU32::new(0),
        }
    }

    // Producer (ISR) - nie blokuje się!
    pub unsafe fn push(&self, value: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next = (tail + 1) & (N - 1);
        
        if next != self.head.load(Ordering::Acquire) {
            self.buffer[tail as usize] = Some(value);
            self.tail.store(next, Ordering::Release);
            true
        } else {
            false // kolejka pełna
        }
    }

    // Consumer (main)
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        
        if head != self.tail.load(Ordering::Acquire) {
            let value = self.buffer[head as usize].take();
            self.head.store((head + 1) & (N - 1), Ordering::Release);
            value
        } else {
            None // kolejka pusta
        }
    }
}
```

## Porównanie z C

```c
// C - może blokować w ISR!
void isr_handler() {
    xQueueSendFromISR(queue, &data, NULL); // NIEBEZPIECZNE!
}

// Rust - bezpiecznie i szybko
unsafe { queue.push(data); } // nie blokuje!
```

## Typowe błędy i jak ich unikać

| Błąd | Rozwiązanie |
|-------|-------------|
| Nadpisanie danych | Zawsze sprawdzaj `head != tail` |
| Race condition | Używaj `Ordering::Acquire/Release` |
| Zły rozmiar | N musi być potęgą dwójki |
| Wiele producerów | SPSC = tylko 1 ISR producer |

## Godny następca

`crossbeam::queue::SegQueue` - bezpieczna, wielowątkowa wersja. [3]

## Źródła

[1] https://ubuntu.com/blog/real-time-linux-vs-rtos-2
[2] https://www.perplexity.ai/search/0a7b8300-9680-4ac3-94e9-4f153c27d9f3
[3] https://www.perplexity.ai/search/284cb83b-2058-4263-b48f-d97fe167e9e9
