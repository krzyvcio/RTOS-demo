# Budgety czasowe (Time Budgets)

## Definicja

**Budget czasowy** to przydzielony, gwarantowany zasób czasu procesora dla zadania. To "budżet" w dosłownym sensie - możesz wydać tylko tyle, ile masz, i nie więcej.

> Budget to umowa: "Masz 2ms na wykonanie zadania. Zużyj więcej = naruszenie kontraktu. System Cię zatrzyma."

```
┌─────────────────────────────────────────────────────────┐
│                   TIME BUDGET                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A: [████████] 8ms budget                          │
│  Task B: [████] 4ms budget                              │
│  Task C: [██] 2ms budget                                │
│                                                         │
│  ┌────────────────────────────────────────────┐        │
│  │████████████████████████████                │        │
│  └────────────────────────────────────────────┘        │
│   │◄───────────── Total: 14ms ─────────────►│          │
│                                                         │
│  Jeśli Task A zużyje > 8ms → Budget exceeded!          │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🍯 Niedźwiedź i zapasy na zimę

Niedźwiedź musi zgromadzić zapas tłuszczu na zimę. To jego "budżet energetyczny":

```
Budżet: 50 kg tłuszczu
Zużycie: 0.5 kg/dzień
Czas: 100 dni zimy

Jeśli zużyje > budżet → nie przetrwa zimy
```

Matka natura egzekwuje budżet brutalnie: przekroczenie = śmierć.

### 🌱 Roślina i woda

Roślina w suchym klimacie ma "budżet wodny":

```
Budżet: woda w liściach i łodydze
Zużycie: transpiracja + fotosynteza
Musi dotrwać do następnego deszczu

Przekroczenie budżetu → więdnięcie → śmierć
```

Kaktusy są mistrzami zarządzania budżetem wodnym.

### 🐦 Ptaki wędrówne

Ptaki migracyjne mają "budżet energetyczny" na przelot:

```
Budżet: zapas tłuszczu
Zużycie: energia lotu
Dystans: 5000 km

Przekroczenie budżetu w połowie drogi → spadają do oceanu
```

---

## Podobieństwo do systemów informatycznych

### API Rate Limiting

```python
# Twitter API
rate_limit = {
    "requests": 900,
    "window": "15min"
}

# Budżet: 900 requestów na 15 minut
# Przekroczenie → 429 Too Many Requests
```

To jest budget na poziomie API. Przekraczasz = odcięcie.

### Container Resource Limits (Docker/K8s)

```yaml
# Kubernetes pod spec
resources:
  limits:
    cpu: "500m"      # Max 0.5 CPU
    memory: "256Mi"  # Max 256MB RAM

# Przekroczenie CPU → throttling
# Przekroczenie memory → OOM kill
```

Budget w kontenerach jest twardo egzekwowany.

### Database Query Budget

```sql
-- PostgreSQL
SET statement_timeout = '5s';
SET lock_timeout = '2s';

-- Query budżet: 5 sekund
-- Przekroczenie → query cancelled
```

---

## Rodzaje budgetów w RTOS

### 1. Execution Time Budget

```c
// Zadanie ma 2ms na wykonanie
task_create("motor_control",
            period=10ms,
            budget=2ms);  // Execution budget

// Jeśli zużyje > 2ms → watchdog lub termination
```

### 2. CPU Utilization Budget

```
Task A: 30% CPU budget
Task B: 20% CPU budget
Task C: 10% CPU budget
───────
Total:  60% utilized, 40% available

Żaden task nie może przekroczyć swojego %
```

### 3. Memory Budget

```c
// Zadanie ma 4KB pamięci
task_create("sensor",
            memory_budget=4096);

// Próba alokacji > 4KB → allocation failure
```

### 4. I/O Budget

```c
// Zadanie może wykonać 100 operacji I/O na sekundę
io_budget = {
    .max_reads = 50,
    .max_writes = 50,
    .window = 1000ms
};
```

---

## Dlaczego budgety są potrzebne?

### Problem 1: Runaway Tasks

```c
void buggy_task(void) {
    while (true) {
        // Oops, brak warunku wyjścia!
        process_data();
    }
}
// Bez budgetu: zawiesza cały system
// Z budgetem: system zabija task po przekroczeniu
```

### Problem 2: Priority Misbehavior

```c
void high_prio_task(void) {
    // Przejął cały CPU!
    while (has_work()) {
        process();
    }
}
// Inne taski głodują
// Z budgetem: dostaje tylko swój przydział
```

### Problem 3: Nieprzewidywalne obciążenie

```
Normal: Task A zużywa 1ms
Burst:  Task A zużywa 100ms (dużo danych)

Bez budgetu: system nieprzewidywalny
Z budgetem: Task A odcięty po 2ms
```

---

## Egzekwowanie budgetów

### Timer-based Enforcement

```c
void task_with_budget(void* arg) {
    Timer budget_timer;

    // Start timer z budżetem
    timer_start(&budget_timer, BUDGET_MS);

    while (has_work()) {
        if (timer_expired(&budget_timer)) {
            // Budget exceeded!
            log_error("Budget exceeded");
            return;  // lub task_terminate()
        }
        process_one_item();
    }
}
```

### OS-level Enforcement (ARINC 653)

```c
// ARINC 653 Partition
CREATE_PROCESS(
    .NAME = "ControlTask",
    .PERIOD = 100ms,
    .TIME_CAPACITY = 20ms,  // Budget!
    ...
);

// OS automatycznie egzekwuje:
// - Jeśli task zużyje > 20ms → OS go wstrzymuje
// - System kontynuuje działanie
```

### Watchdog Timer

```c
void critical_task(void) {
    watchdog_start(5ms);  // Budget = 5ms

    do_work();

    watchdog_feed();  // Potwierdź zakończenie w budżecie
    // Jeśli nie feed → watchdog resetuje system
}
```

---

## Budget Monitoring

### Runtime Monitoring

```c
struct BudgetStats {
    uint32_t executions;
    uint32_t budget_exceeded;
    uint32_t max_used;
    uint32_t avg_used;
};

void monitor_budget(Task* task, BudgetStats* stats) {
    uint32_t start = get_time();

    task_run(task);

    uint32_t elapsed = get_time() - start;

    stats->executions++;
    if (elapsed > task->budget) {
        stats->budget_exceeded++;
    }
    stats->max_used = max(stats->max_used, elapsed);
    stats->avg_used = update_avg(stats->avg_used, elapsed);
}
```

### Budget Violations Dashboard

```
┌─────────────────────────────────────────────────────────┐
│                  BUDGET MONITOR                         │
├─────────────────────────────────────────────────────────┤
│  Task        │ Budget │ Max Used │ Violations │ Status │
│──────────────│────────│──────────│────────────│────────│
│  Control     │ 2ms    │ 1.8ms    │ 0          │ OK ✓   │
│  Sensor      │ 1ms    │ 0.9ms    │ 0          │ OK ✓   │
│  Network     │ 5ms    │ 8.2ms    │ 47         │ ERROR! │
│  Logging     │ 3ms    │ 2.1ms    │ 0          │ OK ✓   │
└─────────────────────────────────────────────────────────┘
```

---

## Jak ustalać budgety?

### Metoda 1: WCET + Margin

```
Budget = WCET × (1 + margin)

Przykład:
WCET = 1.5ms
Margin = 20%
Budget = 1.5 × 1.2 = 1.8ms
```

### Metoda 2: Utilization-based

```
Dla systemu z określoną utilisacją:

Task A: okres 10ms, util. 20% → budget = 2ms
Task B: okres 20ms, util. 15% → budget = 3ms
Task C: okres 50ms, util. 10% → budget = 5ms
```

### Metoda 3: Empirical

```
1. Zmierz rzeczywiste czasy wykonania
2. Znajdź max + std dev
3. Budget = max + 3×std_dev

Przykład:
Max observed: 1.2ms
Std dev: 0.1ms
Budget: 1.2 + 0.3 = 1.5ms
```

---

## Budgety w Mixed-Criticality Systems

```
┌─────────────────────────────────────────────────────────┐
│               MIXED-CRITICALITY BUDGETS                 │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────┐       │
│  │ HI-criticality (Safety)     │ Budget: 10ms  │       │
│  │ - Flight control            │ Guaranteed    │       │
│  │ - Engine management         │               │       │
│  └─────────────────────────────────────────────┘       │
│                                                         │
│  ┌─────────────────────────────────────────────┐       │
│  │ LO-criticality (Comfort)    │ Budget: 5ms   │       │
│  │ - Infotainment              │ Best-effort   │       │
│  │ - Climate control           │ Can be cut    │       │
│  └─────────────────────────────────────────────┘       │
│                                                         │
│  Jeśli HI potrzebuje więcej → LO dostaje mniej         │
└─────────────────────────────────────────────────────────┘
```

---

## Jak świat radzi sobie z budgetami?

### Automotive: AUTOSAR

```c
// AUTOSAR OsTask
TASK(ControlTask) {
    // Budget zdefiniowany w konfiguracji
    // OS monitoruje execution time
    // Exceedance → Error hook
}

// Error handling
void ErrorHook(void) {
    if (GetLastError() == E_OS_PROTECTION_TIME) {
        // Budget exceeded!
        LogError("Task exceeded time budget");
    }
}
```

### Aerospace: ARINC 653

```c
// Partition scheduling z budgetami
SCHEDULE_TABLE = {
    {PARTITION_A, duration=10ms, period=20ms},
    {PARTITION_B, duration=5ms, period=20ms},
    {PARTITION_C, duration=3ms, period=20ms},
    {IDLE, duration=2ms, period=20ms}
};

// Każda partycja ma gwarantowany budżet
// Inne partycje nie mogą go przekroczyć
```

### Consumer Electronics: iOS/macOS

```
App Budgets:
- CPU time: limity dla background apps
- Memory: limity z OOM kill
- Network: limity dla background data
- Battery: limity dla wake-ups

Przekroczenie → app termination lub throttling
```

---

## Budget Anti-Patterns

### ❌ Anti-pattern: Budget = WCET

```
ŹLE:
Budget = WCET = 2ms

Dlaczego źle?
- WCET to teoretyczne maximum
- Rzeczywistość może być gorsza (cache, interrupts)
- Zero marginesu bezpieczeństwa
```

### ❌ Anti-pattern: Brak monitoringu

```
ŹLE:
Budget zdefiniowany, ale nie monitorowany

Dlaczego źle?
- Nie wiesz, jak blisko budżetu działasz
- Naruszenia nie są wykrywane
- Debugging jest niemożliwy
```

### ❌ Anti-pattern: Budgety tylko dla "problematic" tasków

```
ŹLE:
Budgety tylko dla tasków, które już zawiesiły system

Dlaczego źle?
- Wszystkie taski powinny mieć budgety
- Prewencja > reakcja
```

---

## Pytania do przemyślenia

1. Czy wszystkie taski w Twoim systemie mają zdefiniowane budgety?
2. Jakie jest WCET Twojego najbardziej krytycznego tasku? A budget?
3. Co się dzieje, gdy task przekracza budżet?

---

## Quiz

**Pytanie**: Masz system z trzema taskami:

```
Task A: period=10ms, WCET=1ms, budget=?
Task B: period=20ms, WCET=2ms, budget=?
Task C: period=50ms, WCET=5ms, budget=?

Całkowita utilisacja: 70%
Margines bezpieczeństwa: 20%
```

Jakie budgety powinien mieć każdy task?

**Odpowiedź**:

```
Task A:
- WCET = 1ms
- Budget = 1ms × 1.2 = 1.2ms

Task B:
- WCET = 2ms
- Budget = 2ms × 1.2 = 2.4ms

Task C:
- WCET = 5ms
- Budget = 5ms × 1.2 = 6ms

Sprawdzenie utilisacji z budgetami:
U = 1.2/10 + 2.4/20 + 6/50
  = 0.12 + 0.12 + 0.12
  = 0.36 = 36%

OK! Miejsce na overhead i nieprzewidziane przypadki.
```

---

## Wskazówka zapamiętywania

> **Budget = Limit na kartę kredytową**
>
> Masz limit 10,000 zł. Możesz wydać mniej, ale nie więcej.
>
> W RTOS: Task ma budżet 2ms. Może zużyć 1.5ms, ale nie 2.5ms.
>
> Przekroczenie limitu = blokada karty (task termination).
>
> Bank (OS) nie pyta "czy to ważne". Po prostu odcina.