# Wykład: Kolejka SPSC (Lock-Free) - Dyskusja ze Studentami

## 📋 Agenda

1. Wprowadzenie do problemu
1. Dlaczego zwykłe kolejki nie działają w RTOS?
1. Implementacja kolejki SPSC
1. ❓ Pytania studentów
1. Demonstracja kodu
1. Typowe błędy
1. ❓ Dyskusja
1. Podsumowanie

______________________________________________________________________

## 1. Wprowadzenie

**Prowadzący:** Wyobraźmy sobie sytuację: piszemy system sterowania lotem w dronie. Mamy przerwanie (ISR) które odbiera dane z czujników co 1ms. Te dane muszą być przekazane do głównego programu który steruje silnikami.

Jak przekazać te dane bezpiecznie?

**Student 1:** Możemy użyć kolejki z mutexem!

**Prowadzący:** Zobaczmy co się stanie...

______________________________________________________________________

## 2. Dlaczego zwykłe kolejki nie działają w RTOS?

**Prowadzący:** Czy mutex jest bezpieczny w przerwaniu?

```c
// C - PROBLEM!
void ISR() {
    xQueueSendFromISR(queue, &data, NULL); 
    // ❌ Może zawiesić system!
    // ❌ ISR nie może czekać na mutex!
}
```

**Student 2:** A co jeśli użyjemy semafora binarnego?

**Prowadzący:** Też problem - semafor może blokować. W RTOS mamy surowe wymagania czasowe. Każda blokada to potencjalne missed deadline.

### Co jest nie tak z blokadami?

| Problem | Skutek |
|---------|--------|
| Nieokreślony czas oczekiwania | Missed deadline |
| Priority inversion | Zadanie wysokie czeka na niskie |
| Stack usage | Każdy wątek = stos |
| Przełączanie kontekstu | Narzut CPU |

______________________________________________________________________

## 3. Implementacja Kolejki SPSC

**Prowadzący:** Zobaczmy rozwiązanie:

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

    // Producer (ISR) - NIE BLOKUJE SIĘ!
    pub unsafe fn push(&self, value: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next = (tail + 1) & (N - 1);
        
        if next != self.head.load(Ordering::Acquire) {
            self.buffer[tail as usize] = Some(value);
            self.tail.store(next, Ordering::Release);
            true
        } else {
            false // Kolejka pełna - nie czekamy!
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
            None
        }
    }
}
```

______________________________________________________________________

## 4. ❓ Pytania Studentów

### Student 3: Dlaczego `N` musi być potęgą dwójki?

**Prowadzący:** Bardzo dobre pytanie! Chodzi o optymalizację:

```rust
// Zamiast modulo (wolne):
let next = (tail + 1) % N;

// Używamy AND (szybkie):
let next = (tail + 1) & (N - 1);

// Działa bo N-1 to maska z samymi 1 na dole
// N=8: 7 = 0b0111
// tail+1=5: 5 & 7 = 5
// tail+1=8: 8 & 7 = 0 (zapętlenie!)
```

### Student 4: Co to jest `Ordering`?

**Prowadzący:** Wyjaśnijmy na obrazku:

```
Ordering::Relaxed  - bez synchronizacji, tylko atomowość
Ordering::Acquire  - "widzę wszystko co było PRZED zapisem"
Ordering::Release  - "wszystko co zrobię BĘDZIE widoczne PO zapisie"
```

### Student 5: Czy ten kod jest bezpieczny?

**Prowadzący:** Prawie! Dlaczego `push` jest `unsafe`?

```rust
pub unsafe fn push(&self, value: T) -> bool
```

Bo:

1. **Jeden producer** - tylko ISR może wywoływać push
1. **Brak aliasingu** - nikt inny nie pisze do bufora
1. **Copy trait** - wymagamy T: Copy

______________________________________________________________________

## 5. Demonstracja Kodu

**Prowadzący:** Zobaczmy pełny przykład:

```rust
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

const N: usize = 64;

static QUEUE: SpscQueue<u32, N> = SpscQueue::new();

fn main() {
    // Producer - symulacja ISR
    let producer = thread::spawn(|| {
        for i in 0..1000 {
            while !unsafe { QUEUE.push(i) } {
                thread::yield_now(); // Czekaj aż miejsce
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Consumer - main loop
    let consumer = thread::spawn(|| {
        let mut count = 0;
        while count < 1000 {
            if let Some(value) = QUEUE.pop() {
                println!("Odebrano: {}", value);
                count += 1;
            }
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

______________________________________________________________________

## 6. Typowe Błędy

### Błąd 1: Nadpisanie danych

```rust
// ŹLE - nadpisujemy bez sprawdzenia!
pub unsafe fn push_bad(&self, value: T) -> bool {
    let tail = self.tail.load(Ordering::Relaxed);
    self.buffer[tail as usize] = Some(value); // NADPISANIE!
    self.tail.fetch_add(1, Ordering::Release);
    true
}

// DOBRZE - sprawdzamy czy jest miejsce
pub unsafe fn push(&self, value: T) -> bool {
    let tail = self.tail.load(Ordering::Relaxed);
    let next = (tail + 1) & (N - 1);
    
    if next != self.head.load(Ordering::Acquire) {
        self.buffer[tail as usize] = Some(value);
        self.tail.store(next, Ordering::Release);
        true
    } else {
        false // Kolejka pełna
    }
}
```

### Błąd 2: Zły ordering

```rust
// ŹLE - brak synchronizacji
pub unsafe fn push_bad(&self, value: T) -> bool {
    let tail = self.tail.load(Ordering::Relaxed);
    // ... zapis ...
    self.tail.store(next, Ordering::Relaxed); // Za słaby!
}

// DOBRZE - z ordering
pub unsafe fn push(&self, value: T) -> bool {
    // ...
    self.head.load(Ordering::Acquire); // Synchronizacja!
    // ...
    self.tail.store(next, Ordering::Release);
}
```

______________________________________________________________________

## 7. ❓ Dyskusja

### Student 6: A co z wieloma producerami?

**Prowadzący:** To wymaga innego podejścia! SPSC = Single Producer, Single Consumer. Dla wielu producerów potrzebujemy:

- **MPMC** (Multi-Producer, Multi-Consumer) - np. `crossbeam::queue`
- **SPSC z numerami sekwencyjnymi**
- **Osobne kolejki dla każdego producenta**

### Student 7: Jak mierzyć wydajność?

**Prowadzący:** Możemy zmierzyć:

```rust
use std::time::Instant;

fn benchmark() {
    let start = Instant::now();
    
    // Test
    for _ in 0..1_000_000 {
        unsafe { QUEUE.push(42) };
    }
    
    println!("Czas: {:?}", start.elapsed());
}
```

### Student 8: Czy to jest deterministyczne?

**Prowadzący:** TAK! Czas operacji jest stały:

- `push`: O(1) - zawsze stała liczba operacji
- `pop`: O(1) - zawsze stała liczba operacji

Brak alokacji, brak blokad = determinizm!

______________________________________________________________________

## 8. Podsumowanie

**Prowadzący:** Podsumujmy:

| Cecha | Kolejka SPSC | Zwykła kolejka |
|-------|--------------|----------------|
| Blokowanie | ❌ Nie | ✅ Tak |
| Deterministyczny czas | ✅ O(1) | ❌ Zmienny |
| Wydajność | ✅ Bardzo wysoka | ⚠️ Zmienna |
| RTOS-safe | ✅ Tak | ❌ Nie |
| Producer | 1 | Wiele |

### Co zapamiętać?

1. ✅ SPSC = bez blokad, bez problemów
1. ✅ `N` musi być potęgą dwójki
1. ✅ `Ordering::Acquire/Release` = synchronizacja
1. ✅ `unsafe` = wymagane przez Rust (1 producer!)

### Następna lekcja

**Prowadzący:** Następnym razem: **Scheduler z priorytetami** - jak zagwarantować że zadanie o wysokim priorytecie zawsze się wykona!

______________________________________________________________________

## 📚 Materiały dodatkowe

- Dokumentacja `std::sync::atomic`
- "Lock-Free Data Structures" - Alexander Michaelidis
- kod źródłowy `crossbeam::queue`
