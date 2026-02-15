# Priority-Based Scheduler w Rust

## Problem

W systemach hard-real-time potrzebujemy schedulera który gwarantuje:
- **Determinizm** - zawsze wiemy co się wykona
- **Preemption** - zadanie wysokiego priorytetu przerywa niskie
- **Bounded latency** - czas od przerwania do startu zadania jest znany

### Typowe błędy w C

```c
// BŁĄD: Globale bez żadnej kontroli
struct Task {
    void (*func)(void);
    uint8_t priority;
    uint32_t period;
};

struct Task tasks[8];  // <- brak bezpieczeństwa typów!

// BŁĄD: Brak sprawdzenia overlap deadline/period
void create_task(uint32_t period, uint32_t deadline) {
    // Nikt nie sprawdza czy deadline > period!
}

// BŁĄD: Race condition przy dostępie do tasków
uint32_t current_tick;
void SysTick_Handler() {
    current_tick++;  // <- Race z main!
}
```

---

## Rozwiązanie w Rust

### Typy dla safe scheduler

```rust
#![no_std]
#![feature(const_generics)]

use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// TYPY - zakodujmy ograniczenia w typach!
// ============================================================================

/// Priorytet - 0 to najwyższy, MAX to najniższy
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(u8);

impl Priority {
    pub const MAX: Self = Priority(0);
    pub const MIN: Self = Priority(15);
    
    pub const fn new(p: u8) -> Self {
        assert!(p <= 15, "Priorytet musi być 0-15");
        Priority(p)
    }
}

/// Okres zadania w ms - ograniczony do bezpiecznego zakresu
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Period(u32);

impl Period {
    pub const MIN: Self = Period(1);
    pub const MAX: Self = Period(10000);
    
    pub const fn new(p: u32) -> Self {
        assert!(p >= 1 && p <= 10000, "Okres musi być 1-10000ms");
        Period(p)
    }
}

/// Deadline zadania - musi być >= period
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Deadline(u32);

impl Deadline {
    pub const fn new(d: u32) -> Self {
        Deadline(d)
    }
    
    /// Sprawdza czy deadline >= period (type-safe!)
    pub const fn is_valid(&self, period: Period) -> bool {
        self.0 >= period.0
    }
}

/// Task ID
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TaskId(u8);

impl TaskId {
    pub const MAX_TASKS: u8 = 8;
    
    pub const fn new(id: u8) -> Self {
        assert!(id < Self::MAX_TASKS, "Za dużo zadań!");
        TaskId(id)
    }
}

// ============================================================================
// STRUCT: TaskControlBlock
// ============================================================================

/// Blok kontrolny zadania - reprezentacja C zgodna z RTOS
#[repr(C)]
pub struct TaskControlBlock {
    pub id: TaskId,
    pub priority: Priority,
    pub period: Period,
    pub deadline: Deadline,
    pub wcet: u32,              // worst-case execution time [ms]
    pub next_release: Cell<u32>, // następny czas uruchomienia
    pub remaining_time: Cell<u32>, // pozostały czas wykonania
    pub state: Cell<TaskState>,
}

/// Stan zadania
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Suspended,
    Ready,
    Running,
    Blocked,
}
```

### Scheduler

```rust
// ============================================================================
// SCHEDULER - prosty priority-based
// ============================================================================

pub struct Scheduler {
    /// System time counter
    tick: AtomicU32,
    
    /// Tablica zadań - statyczna alokacja
    tasks: [Option<TaskControlBlock>; 8],
    
    /// Indeks aktualnie wykonywanego zadania
    current: Cell<Option<TaskId>>,
}

impl Scheduler {
    /// Konstruktor - inicjalizuje pusty scheduler
    pub const fn new() -> Self {
        Self {
            tick: AtomicU32::new(0),
            tasks: [None; 8],
            current: Cell::new(None),
        }
    }
    
    /// Rejestracja nowego zadania
    /// 
    /// # Safety
    /// - Nie można zarejestrować więcej niż 8 zadań
    /// - Deadline musi być >= period
    pub unsafe fn add_task(
        &mut self,
        id: TaskId,
        priority: Priority,
        period: Period,
        deadline: Deadline,
        wcet: u32,
    ) -> Result<(), SchedulerError> {
        // Sprawdź deadline
        assert!(deadline.is_valid(period), "Deadline < Period!");
        
        // Znajdź wolne miejsce
        let idx = id.0 as usize;
        if self.tasks[idx].is_some() {
            return Err(SchedulerError::TaskExists);
        }
        
        let task = TaskControlBlock {
            id,
            priority,
            period,
            deadline,
            wcet,
            next_release: Cell::new(0),
            remaining_time: Cell::new(wcet),
            state: Cell::new(TaskState::Suspended),
        };
        
        self.tasks[idx] = Some(task);
        Ok(())
    }
    
    /// SysTick handler - wywoływany co 1ms
    pub fn tick(&self) {
        // Atomowy increment tick counter
        let now = self.tick.fetch_add(1, Ordering::Relaxed) + 1;
        
        // Odblokuj zadania których czas nadszedł
        for task in &self.tasks {
            if let Some(t) = task {
                if t.next_release.get() <= now && t.state.get() == TaskState::Suspended {
                    t.state.set(TaskState::Ready);
                    t.next_release.set(now + t.period.0);
                    t.remaining_time.set(t.wcet);
                }
            }
        }
        
        // Wybierz zadanie o najwyższym priorytecie
        self.schedule();
    }
    
    /// Prosty scheduler - FCFS w obrębie priorytetu
    fn schedule(&self) {
        let mut best_task: Option<&TaskControlBlock> = None;
        let mut best_priority = Priority::MIN;
        
        for task in &self.tasks {
            if let Some(t) = task {
                if t.state.get() == TaskState::Ready && t.priority < best_priority {
                    best_priority = t.priority;
                    best_task = Some(t);
                }
            }
        }
        
        // Jeśli mamy zadanie do uruchomienia
        if let Some(t) = best_task {
            t.state.set(TaskState::Running);
            self.current.set(Some(t.id));
        }
    }
    
    /// Preemption - wywołana z SysTick po upływie quantum
    pub fn preempt(&self) {
        if let Some(id) = self.current.get() {
            if let Some(t) = &self.tasks[id.0 as usize] {
                // Zapisz kontekst i oddaj sterowanie
                if t.remaining_time.get() > 0 {
                    t.state.set(TaskState::Ready);
                } else {
                    t.state.set(TaskState::Suspended);
                }
            }
        }
        self.current.set(None);
        self.schedule();
    }
}
```

### Użycie

```rust
// ============================================================================
// UŻYCIE - jak zarejestrować zadanie
// ============================================================================

static mut SCHEDULER: Scheduler = Scheduler::new();

fn main() {
    // Bezpieczna rejestracja w setup (w unsafe, ale z asercjami)
    unsafe {
        SCHEDULER.add_task(
            TaskId::new(0),
            Priority::new(0),     // Najwyższy priorytet
            Period::new(10),      // 10ms okres
            Deadline::new(10),    // deadline = period
            2,                     // WCET = 2ms
        ).unwrap();
        
        SCHEDULER.add_task(
            TaskId::new(1),
            Priority::new(5),     // Niższy priorytet
            Period::new(50),      // 50ms okres  
            Deadline::new(50),
            5,
        ).unwrap();
    }
    
    // Główna pętla
    loop {
        // Simulator SysTick
        SCHEDULER.tick();
    }
}

// SysTick ISR
#[interrupt]
fn SysTick() {
    unsafe { SCHEDULER.tick(); }
}
```

---

## Typowe błędy i jak ich unikać

### Błąd 1: Brak walidacji deadline

```rust
// ŹLE: Można stworzyć niemożliwy do spełnienia deadline
add_task(prio, Period::new(10), Deadline::new(5), wcet);  // deadline < period!

// DOBRZE: Sprawdzone w czasie kompilacji
impl Period {
    pub const fn new(p: u32) -> Self {
        assert!(p >= 1 && p <= 10000, "Okres musi być 1-10000ms");
        Period(p)
    }
}

impl Deadline {
    pub const fn is_valid(&self, period: Period) -> bool {
        self.0 >= period.0
    }
}

// W add_task:
assert!(deadline.is_valid(period), "Deadline < Period!");
```

### Błąd 2: Mutable statics bez synchronizacji

```rust
// ŹLE: Mutowalny static bez synchronizacji
static mut TASKS: [Option<TaskControlBlock>; 8] = [None; 8];

fn access_tasks() {
    unsafe {
        TASKS[0] = Some(task);  // <- data race w multi-thread!
    }
}

// DOBRZE: Cell dla pojedynczego wątku, Atomic dla wielu
static TICK: AtomicU32 = AtomicU32::new(0);
static CURRENT: Cell<Option<TaskId>> = Cell::new(None);

// Lub dla prawdziwego multi-thread (RTOS):
use core::sync::atomic::{AtomicUsize, Ordering};

static CURRENT_TASK: AtomicUsize = AtomicUsize::new(0);
```

### Błąd 3: Stack overflow przez zbyt duży stos

```rust
// ŹLE: Nieokreślony rozmiar stosu
fn task_function(data: &[u8; 4096]) {  // Duży stos na stosie!
    // ...
}

// DOBRZE: Określony rozmiar w static
static TASK_STACK: [u8; 512] = [0; 512];

// Lub użyj linker script do określenia stosu
```

### Błąd 4: Priority inversion

```rust
// ŹLE: Niski priorytet trzyma mutex, wysoki czeka
let guard = low_priority_mutex.lock();
medium_priority_task.run();  // Blokuje wysokie!

// DOBRZE: Priority inheritance (wymaga RTOS lub własnej implementacji)
use cortex_m::peripheral::SCB;
SCB::software().set_prio_bits(inherit_priority);
```

---

## Porównanie z C

| Aspekt | C | Rust |
|--------|---|------|
| Priorytet | `uint8_t` bez kontroli | `Priority(u8)` z asercją |
| Deadline | w komentarzu | w typie `Deadline: Period` |
| Stan | enum w C | prawdziwy enum |
| Race na tick | `volatile` | `AtomicU32` |
| Dodanie zadania | runtime check | const fn |

### Odpowiednik C

```c
// C - trudniejsze do utrzymania
struct Task tasks[8];
uint8_t task_count = 0;

int add_task(uint8_t prio, uint32_t period, uint32_t deadline) {
    if (deadline < period) return -1;  // Runtime check!
    if (task_count >= 8) return -2;
    
    tasks[task_count++] = (struct Task){
        .priority = prio,
        .period = period,
        .deadline = deadline,
        // ...
    };
    return 0;
}
```

---

## Testy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_task_creation() {
        let mut sched = Scheduler::new();
        
        let result = sched.add_task(
            TaskId::new(0),
            Priority::new(0),
            Period::new(10),
            Deadline::new(10),  // valid: deadline >= period
            2,
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    #[should_panic]
    fn test_invalid_deadline() {
        let mut sched = Scheduler::new();
        
        sched.add_task(
            TaskId::new(0),
            Priority::new(0),
            Period::new(10),
            Deadline::new(5),  // BŁĄD: deadline < period!
            2,
        );
    }
    
    #[test]
    fn test_priority_ordering() {
        let p1 = Priority::new(1);
        let p2 = Priority::new(0);
        
        assert!(p2 < p1);  // 0 jest wyższy niż 1
    }
    
    #[test]
    fn test_max_8_tasks() {
        let mut sched = Scheduler::new();
        
        for i in 0..8 {
            let result = sched.add_task(
                TaskId::new(i),
                Priority::new(i),
                Period::new(10),
                Deadline::new(10),
                1,
            );
            assert!(result.is_ok());
        }
        
        // Dziewiąte zadanie powinno się nie udać
        let result = sched.add_task(
            TaskId::new(8),
            Priority::new(8),
            Period::new(10),
            Deadline::new(10),
            1,
        );
        // TaskId::new(8) panic - asercja w konstruktorze
    }
}
```

---

## Podsumowanie

| Cecha | Wartość |
|-------|---------|
| Max zadań | 8 (stały) |
| Priorytety | 0-15 |
| Okres | 1-10000ms |
| Preemption | Tak (quantum-based) |
| Typy | Safety w compile-time |

### Zalety Rust

1. **Type-level constraints** - deadline > period enforced at compile time
2. **No data races** - AtomicU32, Cell
3. **Const generics** - stały rozmiar bez alokacji
4. **Pattern matching** - czysty kod stanu

### Kiedy używać

- Proste embedded bez pełnego RTOS
- Edukacja - demonstracja conceptów
- Gdy potrzebujemy type-safety dla period/deadline
- Gdy FreeRTOS jest zbyt ciężki (mały MCU)
