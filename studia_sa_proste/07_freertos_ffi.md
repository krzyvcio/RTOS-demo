# Lekcja 7: Integracja Rust + FreeRTOS (FFI)

## Problem

Chcesz użyć Rust ale Twój projekt używa FreeRTOS w C. Nie chcesz przepisywać całego kodu! [1]

## Rozwiązanie

Stwórz bezpieczne wrappery RAII wokół C API FreeRTOS.

## Krok 1: Deklaruj C API

```rust
#![no_std]

mod freertos {
    use core::ffi::c_void;
    use core::ffi::c_int;

    #[link(name = "freertos")]
    extern "C" {
        pub fn xSemaphoreCreateMutex() -> *mut c_void;
        pub fn xSemaphoreTake(sem: *mut c_void, wait: u32) -> c_int;
        pub fn xSemaphoreGive(sem: *mut c_void) -> c_int;
        
        pub fn xQueueCreate(len: u32, size: u32) -> *mut c_void;
        pub fn xQueueSend(q: *mut c_void, data: *const c_void, wait: u32) -> c_int;
        pub fn xQueueReceive(q: *mut c_void, data: *mut c_void, wait: u32) -> c_int;
    }
}
```

## Krok 2: Wrapper RAII dla Mutex

```rust
pub struct Mutex {
    handle: *mut core::ffi::c_void,
}

impl Mutex {
    pub fn new() -> Option<Self> {
        let handle = unsafe { freertos::xSemaphoreCreateMutex() };
        if handle.is_null() {
            None
        } else {
            Some(Self { handle })
        }
    }
    
    /// Zablokuj mutex - blokuje do skutku
    pub fn lock(&self) {
        unsafe { freertos::xSemaphoreTake(self.handle, u32::MAX) };
    }
    
    /// Spróbuj zablokować z timeout
    pub fn try_lock(&self, ms: u32) -> bool {
        unsafe { freertos::xSemaphoreTake(self.handle, ms) } == 0
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe { freertos::xSemaphoreGive(self.handle) };
    }
}
```

## Krok 3: Użycie

```rust
static SHARED_DATA: Mutex = Mutex::new();

fn main() {
    let mutex = Mutex::new().unwrap();
    
    // RAII - automatyczne zwolnienie!
    {
        mutex.lock();
        // operacje na danych
    } // mutex zwolniony automatycznie
}
```

## Porównanie

| Aspekt | C | Rust |
|--------|---|------|
| Auto-unlock | Ręcznie | RAII |
| Error handling | Brak | Option/Result |
| Type safety | Słaby | Silny |
| Migracja | - | Stopniowa |

## Typowe błędy

| Błąd | Skutek |
|-------|--------|
| Lock bez unlock | Deadlock |
| Null handle | Crash |
| Multi-thread access | Race condition |

## Godny następca

`cortex-m-rtic` - natywny RTOS w Rust zamiast FreeRTOS. [6]

## Źródła

[1] https://ubuntu.com/blog/real-time-linux-vs-rtos-2
[6] https://www.reddit.com/r/linux/comments/1fl88vk/linux_is_now_a_rtos_preempt_rt_real-time_kernel/
