# Lock-Free SPSC Queue w Rust

## Problem

W systemach embedded i RTOS często potrzebujemy przekazywać dane między ISR (producer) a taskiem (consumer) bez blokowania. Klasyczne rozwiązania (mutexy, semafory) są niebezpieczne w ISR - mogą powodować priority inversion lub deadlock.

### Wymagania

- **Single Producer, Single Consumer** (SPSC)
- **Bez blokad** - ISR nie może czekać na zasób
- **Bounded** - stały rozmiar bufora (unikamy dynamicznej alokacji)
- **Deterministyczny** - stały czas operacji O(1)

### Typowe błędy w C

```c
// BŁĄD: mutex w ISR - może blokować!
void isr_handler() {
    xSemaphoreTake(queue_mutex, portMAX_DELAY); // ZAWIESZENIE!
    // ...
}

// BŁĄD: nieatomowy dostęp - race condition!
volatile int head = 0;
void isr() {
    buffer[head] = data;  // Może się nie wykonać przed odczytem
    head++;               // Race z taskiem!
}
```

---

## Rozwiązanie w Rust

### Implementacja

```rust
use core::sync::atomic::{AtomicUsize, Ordering};
use core::marker::PhantomData;

// ============================================================================
// STRUCT: SpscQueue<T, N>
// ============================================================================
// N musi być potęgą dwójki dla efektywnego modulo przez AND
// ============================================================================

pub struct SpscQueue<T, const N: usize> {
    // Bufor przechowujący elementy - rozmiar znany w czasie kompilacji
    buffer: [Option<T>; N],
    
    // Indeks odczytu (consumer - task)
    head: AtomicUsize,
    
    // Indeks zapisu (producer - ISR)  
    tail: AtomicUsize,
    
    // Marker dla typu T - gwarantuje Send + Sync tylko gdy T: Send
    _marker: PhantomData<T>,
}

impl<T, const N: usize> SpscQueue<T, N>
where
    T: Copy + Default,
{
    // ------------------------------------------------------------------------
    // Konstruktor - stały rozmiar, brak alokacji
    // ------------------------------------------------------------------------
    pub const fn new() -> Self {
        assert!(N.is_power_of_two(), "Rozmiar musi być potęgą dwójki");
        
        Self {
            buffer: [const { None }; N],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            _marker: PhantomData,
        }
    }

    // ------------------------------------------------------------------------
    // ISR Push (Producer) - nieblokujący
    // ------------------------------------------------------------------------
    /// Zwraca true jeśli wstawiono, false jeśli kolejka pełna
    /// 
    /// # Safety
    /// - Tylko jeden producer (ISR) może wywoływać tę metodę
    /// - T musi implementować Copy
    pub unsafe fn isr_push(&self, value: T) -> bool {
        // Odczyt tail z memory ordering Relaxed (wystarczy dla SPSC)
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & (N - 1);
        
        // Sprawdź czy jest miejsce -Acquire synchronizuje z consumer
        if next_tail != self.head.load(Ordering::Acquire) {
            // Wstawienie - Release synchronizuje z consumer
            // Nie potrzebujemy atomic bo tylko ISR pisze
            self.buffer[tail] = Some(value);
            self.tail.store(next_tail, Ordering::Release);
            true
        } else {
            false // Kolejka pełna
        }
    }

    // ------------------------------------------------------------------------
    // Task Pop (Consumer) - nieblokujący
    // ------------------------------------------------------------------------
    /// Zwraca Some(value) jeśli jest element, None jeśli pusta
    pub fn task_pop(&self) -> Option<T> {
        // Relaced wystarcza dla synchronizacji wewnątrz wątku
        let head = self.head.load(Ordering::Relaxed);
        
        if head != self.tail.load(Ordering::Acquire) {
            // Jest element do pobrania
            let value = self.buffer[head].take();
            let next_head = (head + 1) & (N - 1);
            
            // Release synchronizuje z producer
            self.head.store(next_head, Ordering::Release);
            value
        } else {
            None // Kolejka pusta
        }
    }

    // ------------------------------------------------------------------------
    // Sprawdzenie stanu - przydatne do debugowania
    // ------------------------------------------------------------------------
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }
    
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        (tail - head) & (N - 1)
    }
}
```

### Alternatywna implementacja z volatile (dla Cortex-M)

```rust
use core::ptr::{read_volatile, write_volatile};

impl<T, const N: usize> SpscQueue<T, N>
where
    T: Copy,
{
    /// Wersja z volatile - dla bezpośredniego dostępu do rejestrów
    /// Używane gdy T to np. u32, f32, struktura repr(C)
    pub unsafe fn isr_push_volatile(&self, value: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & (N - 1);
        
        if next_tail != self.head.load(Ordering::Acquire) {
            // Volatile write - kompilator nie może zoptymalizować
            write_volatile(&mut self.buffer[tail] as *mut _ as *mut T, value);
            self.tail.store(next_tail, Ordering::Release);
            true
        } else {
            false
        }
    }
    
    pub unsafe fn task_pop_volatile(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        
        if head != self.tail.load(Ordering::Acquire) {
            let value = read_volatile(&self.buffer[head] as *const _ as *const T);
            let next_head = (head + 1) & (N - 1);
            self.head.store(next_head, Ordering::Release);
            Some(value)
        } else {
            None
        }
    }
}
```

---

## Typowe błędy i jak ich unikać

### Błąd 1: Niewłaściwy Memory Ordering

```rust
// ŹLE: Brak synchronizacji
pub unsafe fn isr_push_bad(&self, value: T) -> bool {
    let tail = self.tail.load(Ordering::Relaxed);  // ← za słaby!
    // ...
}

// DOBRZE: Acquire/Release dla synchronizacji z consumer
pub unsafe fn isr_push(&self, value: T) -> bool {
    let tail = self.tail.load(Ordering::Relaxed);
    // ... sprawdzenie ...
    self.head.load(Ordering::Acquire);  // ← synchronizacja!
    // ... zapis ...
    self.tail.store(next_tail, Ordering::Release);  // ← widoczność dla consumer
}
```

### Błąd 2: Niezaokrąglony rozmiar

```rust
// ŹLE: N = 10 nie jest potęgą dwójki
// modulo N musi być zastąpione przez AND (N-1)
let next = (index + 1) % 10;  // Działa, ale wolniejsze

// DOBRZE: N = 8 (potęga dwójki)
let next = (index + 1) & 7;   // = (index + 1) % 8, ale szybsze
```

### Błąd 3: Wiele producerów

```rust
// ŹLEDWOŻE: Multi-producer wymaga dodatkowej logiki
// Dwa ISR mogą pisać jednocześnie -> overwrite!

// ROZWIĄZANIE: Użyj osobnych kolejek lub dodaj warstwę synchronizacji
static QUEUE: SpscQueue<u32, 64> = SpscQueue::new();

// W ISR1
unsafe { QUEUE.isr_push(data1); }

// W ISR2  
unsafe { QUEUE.isr_push(data2); }  // Może nadpisać dane z ISR1!
```

### Błąd 4: Typ bez Copy

```rust
// ŹLE: String nie implementuje Copy
struct Message {
    data: String,  // heap-allocated
}

impl SpscQueue<Message, 64> {
    pub unsafe fn isr_push(&self, value: Message) -> bool {
        // BŁĄD: cannot move out of dereference of raw pointer
        // Musiałbyś użyć Clone zamiast Copy
    }
}

// DOBRZE: Użyj Copy dla prostych typów
type SensorData = u32;  // lub [u8; 32], f32, etc.
type CanFrame = u64;
```

---

## Porównanie z C

| Aspekt | C (typowe) | Rust |
|--------|-----------|------|
| Race condition | Zależy od kompilatora/CPU | Wykryte w compile-time |
| Memory ordering | `volatile` + `memory_barrier()` | `Ordering::*` |
| Rozmiar | dynamiczny albo stały z makrami | stały w czasie kompilacji |
| Bezpieczeństwo | brak | borrow checker chroni |
| Wydajność | zależy od implementacji | identyczny kodAssembly |

### Odpowiednik w C ( FreeRTOS)

```c
// FreeRTOS implementation - wymaga configASSERT
BaseType_t xQueueSendFromISR(QueueHandle_t xQueue, const void *pvItemToQueue, BaseType_t *pxHigherPriorityTaskWoken) {
    // Złożoność: ~100 linii kodu
    // Ryzyko: priority inversion, blocking
}
```

---

## Testy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_push_pop() {
        const N: usize = 8;
        let queue: SpscQueue<u32, N> = SpscQueue::new();
        
        // Push 3 elementy
        assert!(unsafe { queue.isr_push(1) });
        assert!(unsafe { queue.isr_push(2) });
        assert!(unsafe { queue.isr_push(3) });
        
        // Pop 3 elementy
        assert_eq!(queue.task_pop(), Some(1));
        assert_eq!(queue.task_pop(), Some(2));
        assert_eq!(queue.task_pop(), Some(3));
        
        assert!(queue.is_empty());
    }
    
    #[test]
    fn test_overflow() {
        const N: usize = 4;
        let queue: SpscQueue<u32, N> = SpscQueue::new();
        
        // Napełnij kolejkę
        for i in 0..N {
            assert!(unsafe { queue.isr_push(i) });
        }
        
        // Następny push fails
        assert!(!unsafe { queue.isr_push(99) });
    }
    
    #[test]
    fn test_underflow() {
        const N: usize = 4;
        let queue: SpscQueue<u32, N> = SpscQueue::new();
        
        // Pusta kolejka zwraca None
        assert_eq!(queue.task_pop(), None);
    }
}
```

---

## Podsumowanie

| Cecha | Wartość |
|-------|---------|
| Złożoność | O(1) push/pop |
| Alokacja | 0 - stały rozmiar |
| Locking | 0 - lock-free |
| Thread-safe | Tak (SPSC) |
|Determinizm| Tak - stały czas |

### Zalety Rust

1. **Compile-time size** - rozmiar znany w czasie kompilacji
2. **Type safety** - borrow checker zapobiega błędom
3. **Memory ordering** - jawne `Ordering` zamiast niejawnych barrier
4. **Zero-cost abstraction** - kod kompiluje się do identycznego assembly jak C

### Kiedy używać

- Komunikacja ISR ↔ Task
- Buforowanie danych sensorowych
- Proste kolejki w embedded bez RTOS
- Fire-and-forget event passing
