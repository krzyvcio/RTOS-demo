# Rust + FreeRTOS Integration (FFI)

## Problem

Wielu producentów używa FreeRTOS (lub innego komercyjnego RTOS) i chce stopniowo migrować do Rust. Zamiast przepisywać cały system, można:
- Zostawić RTOS w C
- Pisać nowy kod w Rust
- Łączyć przez bezpieczne FFI

### Korzyści

| Aspekt | Tylko C | Rust + FreeRTOS |
|--------|---------|-----------------|
| Bezpieczeństwo pamięci | Manualne | borrow checker |
| Typy | struct bez guarantee | RAII wrappers |
| Migracja | całość | stopniowa |
| Istniejący kod | bez zmian | bez zmian |

---

## Rozwiązanie

### Krok 1: Deklaruj C API w Rust

```rust
#![no_std]

// ============================================================================
// FREERTOS C API - deklaracje
// ============================================================================

mod freertos {
    use core::ffi::c_void;
    use core::ffi::c_int;
    
    // Typy FreeRTOS
    pub type QueueDefinition = c_void;
    pub type QueueHandle = *mut QueueDefinition;
    pub type TaskHandle = *mut QueueDefinition;
    pub type TickType = u32;
    
    // Kolejki
    #[link(name = "freertos")]
    extern "C" {
        pub fn xQueueCreate(uxQueueLength: u32, uxItemSize: u32) -> QueueHandle;
        pub fn xQueueSend(
            xQueue: QueueHandle,
            pvItemToQueue: *const c_void,
            xTicksToWait: TickType,
        ) -> c_int;
        pub fn xQueueReceive(
            xQueue: QueueHandle,
            pvBuffer: *mut c_void,
            xTicksToWait: TickType,
        ) -> c_int;
        pub fn xQueueGiveFromISR(
            xQueue: QueueHandle,
            pxHigherPriorityTaskWoken: *mut c_int,
        ) -> c_int;
        pub fn xQueueTakeFromISR(
            xQueue: QueueHandle,
            pxHigherPriorityTaskWoken: *mut c_int,
        ) -> c_int;
    }
    
    // Taski
    #[link(name = "freertos")]
    extern "C" {
        pub fn xTaskCreate(
            pvTaskCode: Option<unsafe extern "C" fn(*mut c_void)>,
            pcName: *const u8,
            usStackDepth: u32,
            pvParameters: *mut c_void,
            uxPriority: u32,
            pxCreatedTask: *mut TaskHandle,
        ) -> c_int;
        pub fn vTaskDelay(xTicksToDelay: TickType);
        pub fn vTaskSuspend(xTaskToSuspend: TaskHandle);
        pub fn vTaskResume(xTaskToResume: TaskHandle);
    }
    
    // Semafor (binary/mutex)
    #[link(name = "freertos")]
    extern "C" {
        pub fn xSemaphoreCreateBinary() -> QueueHandle;
        pub fn xSemaphoreCreateMutex() -> QueueHandle;
        pub fn xSemaphoreTake(xSemaphore: QueueHandle, xTicksToWait: TickType) -> c_int;
        pub fn xSemaphoreGive(xSemaphore: QueueHandle) -> c_int;
        pub fn xSemaphoreGiveFromISR(
            xSemaphore: QueueHandle,
            pxHigherPriorityTaskWoken: *mut c_int,
        ) -> c_int;
    }
}
```

### Krok 2: Bezpieczne wrappery RAII

```rust
use core::ffi::c_void;
use freertos::{QueueHandle, TaskHandle, TickType};

// ============================================================================
// RAII WRAPPER: Queue
// ============================================================================

/// Bezpieczna kolejka - RAII wrapper
pub struct Queue<T> {
    handle: QueueHandle,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Queue<T> {
    /// Utwórz kolejkę o określonej długości
    /// 
    /// # Safety
    /// - T musi mieć stały rozmiar
    /// - Nie można wysyłać wartości Option<T> bez specjalnej obsługi
    pub unsafe fn new(length: u32) -> Result<Self, QueueError> {
        let handle = freertos::xQueueCreate(length, core::mem::size_of::<T>() as u32);
        
        if handle.is_null() {
            Err(QueueError::OutOfMemory)
        } else {
            Ok(Self { 
                handle, 
                _phantom: core::marker::PhantomData 
            })
        }
    }
    
    /// Wyślij element (blocking)
    pub fn send(&self, value: &T, timeout: TickType) -> Result<(), QueueError> {
        let result = unsafe {
            freertos::xQueueSend(
                self.handle,
                value as *const T as *const c_void,
                timeout,
            )
        };
        
        if result == 0 { Ok(()) } else { Err(QueueError::SendFailed) }
    }
    
    /// Odbierz element (blocking)
    pub fn receive(&mut self, timeout: TickType) -> Result<T, QueueError> {
        let mut value: T = unsafe { core::mem::zeroed() };
        
        let result = unsafe {
            freertos::xQueueReceive(
                self.handle,
                &mut value as *mut T as *mut c_void,
                timeout,
            )
        };
        
        if result == 0 { Ok(value) } else { Err(QueueError::ReceiveFailed) }
    }
    
    /// Wyślij z ISR (non-blocking)
    pub unsafe fn send_from_isr(&self) -> Result<bool, IsrError> {
        let mut woken: core::ffi::c_int = 0;
        let result = freertos::xQueueGiveFromISR(self.handle, &mut woken);
        
        if result == 0 {
            Ok(woken != 0)
        } else {
            Err(IsrError::Failed)
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // FreeRTOS queue jest automatycznie usuwany gdy task kończy
        // W praktyce trzeba ręcznie usunąć przez vQueueDelete()
    }
}

// ============================================================================
// BŁĘDY
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub enum QueueError {
    OutOfMemory,
    SendFailed,
    ReceiveFailed,
    Timeout,
}

#[derive(Debug, Clone, Copy)]
pub enum IsrError {
    Failed,
}
```

### Krok 3: Mutex z RAII

```rust
// ============================================================================
// RAII WRAPPER: Mutex z dziedziczeniem priorytetów
// ============================================================================

/// Bezpieczny mutex - automatycznie zwalnia przy drop
pub struct Mutex<T> {
    handle: freertos::QueueHandle,
    data: T,
}

impl<T> Mutex<T> {
    /// Utwórz mutex z początkową wartością
    pub unsafe fn new(data: T) -> Result<Self, MutexError> {
        let handle = freertos::xSemaphoreCreateMutex();
        
        if handle.is_null() {
            Err(MutexError::CreationFailed)
        } else {
            Ok(Self { handle, data })
        }
    }
    
    /// Zablokuj i pobierz mutable reference
    /// 
    /// # Safety
    /// - Tylko jeden wątek może trzymać mutex w danym momencie
    /// - Nie można używać w ISR
    pub unsafe fn lock(&self) -> MutexGuard<T> {
        // portMAX_DELAY =blocking
        freertos::xSemaphoreTake(self.handle, 0xFFFFFFFF);
        MutexGuard { 
            mutex: self,
            _marker: core::marker::PhantomData,
        }
    }
}

/// Guard - automatycznie zwalnia mutex przy drop
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.mutex.data
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mutex.data
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { freertos::xSemaphoreGive(self.mutex.handle) };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MutexError {
    CreationFailed,
}
```

### Krok 4: Użycie

```rust
// ============================================================================
// PRZYKŁAD UŻYCIA
// ============================================================================

static mut SHARED_DATA: i32 = 0;

fn rust_task_main(_params: *mut c_void) {
    unsafe {
        // Utwórz kolejkę
        let mut queue: Queue<u32> = Queue::new(10).unwrap();
        
        // Wyślij dane
        queue.send(&42, 100).unwrap();
        
        // Użyj mutex
        let mutex = Mutex::new(0).unwrap();
        {
            let mut guard = mutex.lock();
            *guard += 1;
            SHARED_DATA = *guard;
        }  // Mutex automatycznie zwolniony!
        
        // Odbierz z kolejki
        if let Ok(value) = queue.receive(100) {
            println!("Odebrano: {}", value);
        }
    }
    
    loop {
        unsafe { freertos::vTaskDelay(1000) };
    }
}

// ============================================================================
// EKSport do C
// ============================================================================

/// Eksport funkcji Rust do C
#[no_mangle]
pub unsafe extern "C" fn rust_task_start() {
    use core::ffi::c_char;
    
    let name: *const c_char = b"rust_task\0".as_ptr() as *const c_char;
    let mut handle: freertos::TaskHandle = core::ptr::null_mut();
    
    freertos::xTaskCreate(
        Some(rust_task_main),
        name,
        1024,  // stack depth (words)
        core::ptr::null_mut(),
        1,     // priority
        &mut handle,
    );
}
```

---

## Typowe błędy i jak ich unikać

### Błąd 1: UB przez źle dopasowane typy

```rust
// ŹLE: Rozmiar nie zgadza się z C
struct BigStruct { a: u64, b: u64, c: u64 }  // 24 bytes
freertos::xQueueCreate(length, 8);  // BŁĄD! 

// DOBRZE: Użyj size_of
freertos::xQueueCreate(length, core::mem::size_of::<BigStruct>() as u32);
```

### Błąd 2: Lock w ISR

```rust
// ŹLE: Mutex w ISR - może się zawiesić!
#[interrupt]
fn UART1() {
    let guard = mutex.lock();  // BLOKADA W ISR!
    // ...
}

// DOBRZE: Użyj semafora binarnego
#[interrupt]
fn UART1() {
    unsafe {
        queue.send_from_isr();  // Non-blocking
    }
}
```

### Błąd 3: Przekroczenie stosu

```rust
// ŹLE: Duży stos na stosie
fn task_with_big_buffer() {
    let buffer = [0u8; 4096];  // 4KB na stosie!
    // ...
}

// DOBRZE: Static allocation
static BUFFER: [u8; 4096] = [0; 4096];

fn task_with_static_buffer() {
    // Użyj BUFFER
}
```

### Błąd 4: Memory leak przez Box

```rust
// ŹLE: Box w no_std bez global allocator
fn create_data() -> Box<BigData> {
    Box::new(BigData::new())  // BŁĄD w no_std bez alloc!
}

// DOBRZE: Static lub heapless
static DATA: Mutex<BigData> = Mutex::new(BigData::new());
```

---

## Porównanie z C

### Czysty C

```c
// C - łatwo o błędy
QueueHandle_t queue;
int data;

void task() {
    queue = xQueueCreate(10, sizeof(int));
    
    // Łatwo zapomnieć o sprawdzeniu!
    xQueueSend(queue, &data, portMAX_DELAY);
    
    // Lock bez auto-unlock!
    xSemaphoreTake(mutex, portMAX_DELAY);
    data++;  // Błąd: jeśli panic tu, mutex nie zwolniony!
    xSemaphoreGive(mutex);
}
```

### Rust + FreeRTOS

```rust
// Rust - bezpieczniej
let queue: Queue<i32> = Queue::new(10).unwrap();
let mutex: Mutex<i32> = Mutex::new(0).unwrap();

fn task() {
    queue.send(&data, MAX_DELAY).unwrap();  // Result = sprawdzone
    
    // RAII = auto-unlock nawet przy panic!
    let mut guard = mutex.lock();
    *guard += 1;
}
```

---

## Testy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Testy jednostkowe (bez FreeRTOS - mock)
    
    #[test]
    fn test_queue_error_handling() {
        // Symuluj brak pamięci
        let result: Result<Queue<u32>, _> = Queue::new(u32::MAX);
        // Może się nie udać w prawdziwym systemie
    }
    
    #[test]
    fn test_mutex_raii() {
        unsafe {
            let mutex = Mutex::new(0i32).unwrap();
            {
                let mut guard = mutex.lock();
                *guard = 42;
                assert_eq!(*guard, 42);
            }  // Automatyczne zwolnienie
            
            let guard = mutex.lock();
            assert_eq!(*guard, 42);  // Wartość zachowana
        }
    }
}
```

---

## Podsumowanie

| Cecha | C | Rust + FreeRTOS |
|-------|---|-----------------|
| Auto-unlock mutex | Manualne | RAII |
| Error handling | - | Result<T, E> |
| Type safety | struct | Generics |
| Memory safety | Manualne | borrow checker |
| Migracja | - | Stopniowa |

### Zalety podejścia

1. **Stopniowa migracja** - nie przepisuj wszystkiego naraz
2. **Bezpieczeństwo** - RAII guards, Result types
3. **Interoperacyjność** - C API działa bez zmian
4. **Isolacja** - nowy kod w Rust, stary w C

### Kiedy używać

- Istniejący projekt C z FreeRTOS
- Stopniowa migracja do Rust
- Gdy potrzebujesz Rust safety ale nie możesz porzucić RTOS
- Gdy zespół zna C/FreeRTOS a Rust jest nowy
