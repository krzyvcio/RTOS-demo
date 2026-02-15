# Lekcja 2: Scheduler z Priorytetami w Rust

## Problem

W RTOS proste FIFO nie gwarantuje priorytetów. Zadanie o wysokim priorytecie może być blokowane przez zadanie o niskim priorytecie - to priority inversion i może być katastrofalne! [4]

## Typowe błędy

- **Brak priority inheritance** - bez tego priority inversion prowadzi do deadlock
- **Przepełnienie sterty** - zbyt dużo zadań w heap
- **Niespójne porównania** - błędne implementacje `Ord` [5]

## Rozwiązanie

Użyj `BinaryHeap` z odwróconą kolejnością - wyższy priorytet = pierwszy.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Eq, PartialEq)]
struct Task {
    prio: u8,
    id: u32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.prio.cmp(&self.prio) // wyższy prio = mniejszy w reversed
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Scheduler {
    heap: BinaryHeap<Task>,
}

impl Scheduler {
    fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }
    
    fn add_task(&mut self, prio: u8, id: u32) {
        self.heap.push(Task { prio, id });
    }
    
    fn next(&mut self) -> Option<u32> {
        self.heap.pop().map(|t| t.id)
    }
    
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
```

## Użycie

```rust
fn main() {
    let mut sched = Scheduler::new();
    
    // Dodaj zadania
    sched.add_task(10, 1); // niski priorytet
    sched.add_task(1, 2);  // wysoki priorytet
    sched.add_task(5, 3);  // średni
    
    // Zawsze dostajemy zadanie o najwyższym priorytecie
    while let Some(id) = sched.next() {
        println!("Wykonywanie zadania: {}", id);
    }
}
```

## Typowe błędy i jak ich unikać

| Błąd | Rozwiązanie |
|-------|-------------|
| Brak inheritance | Użyj RTOS z priority inheritance |
| Odwrotna kolejność | Testuj z äußerst priorytetami |
| Przepełnienie | Limituj liczbę zadań |

## Porównanie z C

```c
// C - trudniejsze
typedef struct {
    uint8_t prio;
    uint32_t id;
} Task;

// Trzeba ręcznie sortować lub użyć listy
Task tasks[10];
int task_count = 0;

void add_task(uint8_t prio, uint32_t id) {
    tasks[task_count++] = (Task){.prio = prio, .id = id};
    // Sortowanie...
}
```

## Godny następca

`embassy-scheduler` - async scheduler w Embassy (RTOS dla embedded w Rust). [6]

## Źródła

[4] https://www.embedded.com/the-rtos-renaissance-closing-the-os-gap-with-linux-in-iot/
[5] https://www.perplexity.ai/search/e67efac0-ea48-43f2-b9b6-afc3c2f5e217
[6] https://www.reddit.com/r/linux/comments/1fl88vk/linux_is_now_a_rtos_preempt_rt_real-time_kernel/
