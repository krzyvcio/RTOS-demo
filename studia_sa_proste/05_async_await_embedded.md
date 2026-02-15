# Lekcja 5: Async/Await w Embedded

## Problem

Tradycyjne wątki (threads) zużywają za dużo pamięci. W embedded mamy ograniczone zasoby! Chcemy lżejsze rozwiązania. [6]

## Rozwiązanie

Async/await w Rust pozwala na współbieżność bez pełnych wątków. Zadania (futures) są znacznie lżejsze.

```rust
#![no_std]
#![feature(async_fn_in_trait)]

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

// Prosty executor
struct Executor {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Executor {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    
    fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        self.tasks.push(Box::pin(fut));
    }
    
    fn run(&mut self) {
        loop {
            for task in &mut self.tasks {
                let waker = noop_waker();
                let mut cx = Context::from_waker(&waker);
                
                if let Poll::Ready(()) = task.as_mut().poll(&mut cx) {
                    // Zadanie zakończone
                }
            }
        }
    }
}

fn noop_waker() -> core::task::Waker {
    // Pusty waker - dla demonstracji
    todo!("Użyj embassy lub tokio")
}
```

## Użycie z async fn

```rust
async fn blink_led() {
    loop {
        // Włącz LED
        // Czekaj 100ms
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        // Wyłącz LED
    }
}

async fn read_sensor() -> u32 {
    // Czytaj z sensora
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    42
}

async fn main_task() {
    // Równolegle: blink + read
    tokio::join!(
        blink_led(),
        read_sensor()
    );
}
```

## Zalety async w embedded

| Zaleta | Opis |
|--------|------|
| Mniejsze zużycie pamięci | Zadanie = stan maszyny stanów |
| Latwy concurrent I/O | Pisz synchronicznie, działaj asynchronicznie |
| Zero koszt przy czekaniu | Nie zajmujesz wątku |

## Wady

| Wada | Opis |
|------|------|
| Niepreemptive | Zadania muszą oddać sterowanie (cooperacyjne) |
| Trudniejsze hard real-time | Trudniej o gwarancje czasowe |
| Większy kod | Stapler = narzut |

## Typowe błędy

| Błąd | Skutek |
|-------|--------|
| Brak .await | Zadanie się nie wykonuje |
| Nieoddanie sterowania | Inne zadania głodują |
| Za duże stany | Przepełnienie stosu |

## Godny następca

`embassy-nrf` - pełne async API dla ARM Cortex-M z obsługą GPIO, UART, SPI, I2C, Timer, RTC. [6]

## Źródła

[6] https://www.reddit.com/r/linux/comments/1fl88vk/linux_is_now_a_rtos_preempt_rt_real-time_kernel/
