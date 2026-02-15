# RMS (Rate Monotonic Scheduling)

## Definicja

**RMS** to algorytm przydziału priorytetów, w którym taski o krótszym okresie (wyższej częstotliwości) mają wyższy priorytet. Jest to statyczny algorytm - priorytety są stałe i nie zmieniają się w czasie działania.

> RMS to "demokracja częstotliwości": im częściej task musi działać, tym ważniejszy jest. Krótki okres = wysoki priorytet.

```
┌─────────────────────────────────────────────────────────┐
│              RATE MONOTONIC PRIORITIES                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A: Period = 5ms   → Priority: HIGH ────────┐     │
│  Task B: Period = 10ms  → Priority: MEDIUM ─────┐│     │
│  Task C: Period = 20ms  → Priority: LOW ───────┐││     │
│                                                │││     │
│  Krótszy period = Wyższy priorytet             │││     │
│                                                ▼▼▼     │
│  Wykonywanie: A A A A A B A A A A C A A A A A B A...   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### ❤️ Systemy biologiczne

Organizm ma wiele "tasków" o różnych częstotliwościach:

```
Serce:        60-100 uderzeń/min  → Najwyższy priorytet
Oddychanie:   12-20 oddechów/min  → Średni priorytet
Trawienie:    ciągłe, wolne      → Niski priorytet

Serce musi bić częściej niż trawienie działa.
Stąd "wyższy priorytet" dla serca.
```

Gdyby trawienie miało wyższy priorytet - serce mogłoby nie zdążyć bić!

### 🌊 Fale na plaży

```
Fale małe: co 3 sekundy    → Częste
Fale średnie: co 10 sekund → Rzadsze
Fale duże: co 30 sekund    → Najrzadsze

Małe fale "mają priorytet" - przychodzą częściej.
```

### 🐝 Rój pszczół

```
Zbiór nektaru: ciągły       → Wysoka częstotliwość
Budowa plastra: okresowy    → Średnia częstotliwość
Wyprowadzka roju: rzadki    → Niska częstotliwość

Codzienne zadania mają wyższy priorytet.
```

---

## Podobieństwo do systemów informatycznych

### Obsługa klientów

```
Kolejka w banku:
- Klient szybki (wpłata): 2 minuty   → Częsty
- Klient średni (wypłata): 5 minut   → Średni
- Klient długi (kredyt): 30 minut    → Rzadki

RMS: Obsługuj częstszych klientów szybciej.
Inni mogą poczekać - przychodzą rzadziej.
```

### API Rate Limiting

```python
# Endpointy o różnej częstotliwości
endpoints = {
    "/health": period=1s,      # Bardzo częsty → wysoki prio
    "/metrics": period=10s,    # Częsty → średni prio
    "/report": period=3600s,   # Rzadki → niski prio
}

# RMS: Health check ma najwyższy priorytet
#      Report może poczekać
```

### Game Loop

```cpp
// Różne systemy gry o różnej częstotliwości
void gameLoop() {
    while (running) {
        input();       // 60 Hz  → High priority
        physics();     // 60 Hz  → High priority
        ai();          // 30 Hz  → Medium priority
        rendering();   // 60 Hz  → High priority
        audio();       // 44100 Hz → Highest!
        networking();  // 20 Hz  → Lower priority
    }
}
// RMS: Audio ma najwyższy priorytet (najkrótszy period)
```

---

## Matematyka RMS

### Zasada przydziału priorytetów

```
Priorytet(i) ∝ 1 / Period(i)

Krótszy period → Wyższy priorytet
Dłuższy period → Niższy priorytet
```

### RMS Utilization Bound

```
Dla systemu z N tasków:

U ≤ N(2^(1/N) - 1)

Wartości:
N=1:  100.0%
N=2:   82.8%
N=3:   78.0%
N=4:   75.7%
N=5:   74.3%
N=10:  71.8%
N=∞:   69.3% (ln 2)
```

**To jest sufficient condition**: jeśli utilisacja jest poniżej tego limitu, system JEST schedulable.

**Ale nie necessary**: system może być schedulable nawet przy wyższej utilisacji!

### Dlaczego ln(2) ≈ 69.3%?

Limit dla N→∞ wynosi ln(2). To fundamentalne ograniczenie statycznego przydziału priorytetów.

```
lim N(2^(1/N) - 1) = ln(2) ≈ 0.693
N→∞

Interpretacja:
- Zawsze ~30% CPU jest "zmarnowane" przy RMS
- To cena za statyczne priorytety
- EDF może wykorzystać 100%
```

---

## RMS w praktyce

### Przykład 1: Prosty system

```
Task 1: C=1ms, T=4ms → Priority HIGH
Task 2: C=2ms, T=6ms → Priority MEDIUM
Task 3: C=1ms, T=8ms → Priority LOW

Utilization:
U = 1/4 + 2/6 + 1/8 = 0.25 + 0.333 + 0.125 = 0.708 = 70.8%

RMS bound for N=3: 78.0%
70.8% < 78.0% → SCHEDULABLE ✓
```

### Przykład 2: System na granicy

```
Task 1: C=3ms, T=5ms
Task 2: C=3ms, T=10ms
Task 3: C=1ms, T=20ms

Utilization:
U = 3/5 + 3/10 + 1/20 = 0.6 + 0.3 + 0.05 = 0.95 = 95%

RMS bound for N=3: 78.0%
95% > 78.0% → Test sufficient NIE spełniony

Ale czy system jest schedulable? Sprawdźmy RTA:
...
Okazuje się, że MOŻE być schedulable!
RMS bound to wystarczający, nie konieczny warunek.
```

### Wykonywanie w czasie

```
Timeline dla RMS (Task 1: T=4, Task 2: T=6):

ms:  0  1  2  3  4  5  6  7  8  9  10 11 12
     ┌──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
T1:  │██│  │  │  │██│  │  │  │██│  │  │  │██│  Period=4
     └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
     ┌──────┐       ┌──────┐       ┌──────┐
T2:  │██████│       │██████│       │██████│      Period=6
     └──────┘       └──────┘       └──────┘

T1 preempts T2 przy każdym swoim okresie!

ms:  0  1  2  3  4  5  6  7  8  9  10 11 12
     ┌──┬──────┬──┬──────┬──┬──────┬──┬──────┐
     │T1│ T2   │T1│ T2   │T1│ T2   │T1│ T2   │
     └──┴──────┴──┴──────┴──┴──────┴──┴──────┘
```

---

## RMS vs Inne algorytmy

### RMS vs EDF

| Cecha | RMS | EDF |
|-------|-----|-----|
| Priorytety | Statyczne | Dynamiczne |
| Utilization bound | ~69% | 100% |
| Implementacja | Prosta | Złożona |
| Przewidywalność | Wysoka | Średnia |
| Overhead | Niski | Wyższy |

```
RMS: Priorytety z góry, łatwe debugowanie
EDF: Priorytety zmienne, lepsza utilisacja
```

### RMS vs Deadline Monotonic

```
RMS:  Priorytet ∝ 1/Period
DM:   Priorytet ∝ 1/Deadline

Kiedy różne?
- Period ≠ Deadline

Task A: Period=10ms, Deadline=5ms
Task B: Period=20ms, Deadline=3ms

RMS: A > B (krótszy period)
DM:  B > A (krótszy deadline)

Jeśli Deadline < Period → DM może być lepszy
```

---

## Dlaczego RMS jest popularny?

### Zalety

1. **Prostota implementacji**
```c
// Priorytety są stałe
#define TASK_A_PRIORITY  1  // Period=5ms
#define TASK_B_PRIORITY  2  // Period=10ms
#define TASK_C_PRIORITY  3  // Period=20ms

// Konfiguracja na etapie kompilacji
task_create(task_a, TASK_A_PRIORITY);
task_create(task_b, TASK_B_PRIORITY);
task_create(task_c, TASK_C_PRIORITY);
```

2. **Przewidywalność**
```
Priorytety się nie zmieniają
Łatwo przewidzieć zachowanie
Łatwo debugować
```

3. **Niski overhead**
```
Brak przeliczania priorytetów
Brak dynamicznych decyzji
O(N) scheduling decision
```

4. **Dobrze zrozumiany**
```
Dekady badań
Znane właściwości
Dużo narzędzi
```

### Wady

1. **Nieoptymalna utilisacja**
```
Max ~69% utilisacji (vs 100% EDF)
Marnowanie zasobów CPU
```

2. **Nieobsługuje deadline ≠ period**
```
Gdy deadline < period → RMS może nie działać
Potrzebny Deadline Monotonic
```

3. **Priorytety nieintuicyjne**
```
Często "ważniejszy" task ma niższy priorytet
bo ma dłuższy period
```

---

## RMS Implementation

### Pseudokod

```c
// Definicja tasku
typedef struct {
    void (*function)(void);
    uint32_t period;      // W ms
    uint32_t wcet;        // W ms
    uint32_t priority;    // Wyliczone z period
    uint32_t last_run;
} Task;

// Inicjalizacja priorytetów RMS
void init_rms_priorities(Task tasks[], int n) {
    // Sortuj po period (rosnąco)
    for (int i = 0; i < n-1; i++) {
        for (int j = i+1; j < n; j++) {
            if (tasks[j].period < tasks[i].period) {
                // Zamień
                Task temp = tasks[i];
                tasks[i] = tasks[j];
                tasks[j] = temp;
            }
        }
    }

    // Przypisz priorytety
    for (int i = 0; i < n; i++) {
        tasks[i].priority = n - i;  // Krótszy period = wyższy prio
    }
}

// Scheduler
void rms_scheduler(Task tasks[], int n) {
    while (1) {
        uint32_t current_time = get_time();

        // Znajdź gotowy task o najwyższym priorytecie
        Task* highest = NULL;
        for (int i = 0; i < n; i++) {
            if (current_time >= tasks[i].last_run + tasks[i].period) {
                if (highest == NULL || tasks[i].priority > highest->priority) {
                    highest = &tasks[i];
                }
            }
        }

        if (highest) {
            highest->function();
            highest->last_run = current_time;
        }
    }
}
```

---

## RMS Analysis Tool

```python
def rms_analysis(tasks):
    """
    Analiza schedulability dla RMS
    tasks: lista (period, wcet) w ms
    """
    n = len(tasks)

    # Sortuj po period
    tasks_sorted = sorted(tasks, key=lambda t: t[0])

    # Oblicz utilisację
    utilization = sum(wcet/period for period, wcet in tasks)

    # RMS bound
    rms_bound = n * (2 ** (1/n) - 1)

    print(f"Liczba tasków: {n}")
    print(f"Utilisacja: {utilization*100:.1f}%")
    print(f"RMS bound: {rms_bound*100:.1f}%")

    if utilization <= rms_bound:
        print("✓ SCHEDULABLE (sufficient condition)")
        return True
    else:
        print("⚠ Sprawdź dokładniej (RTA)")
        return rta_analysis(tasks_sorted)

def rta_analysis(tasks):
    """Response Time Analysis"""
    for i, (period, wcet) in enumerate(tasks):
        r = wcet
        while True:
            interference = sum(
                math.ceil(r / tasks[j][0]) * tasks[j][1]
                for j in range(i)
            )
            new_r = wcet + interference
            if new_r == r:
                break
            if new_r > period:
                print(f"✗ Task {i}: WCRT={new_r} > Period={period}")
                return False
            r = new_r
        print(f"✓ Task {i}: WCRT={r} ≤ Period={period}")
    return True
```

---

## Jak świat używa RMS?

### Automotive (AUTOSAR)

```
Standard AUTOSAR używa RMS dla:
- Engine control (krótki period, wysoki prio)
- Transmission control (średni period)
- Body control (długi period, niski prio)
```

### Aerospace

```
Flight control loops:
- Inner loop (stability): 1000 Hz → Highest
- Outer loop (guidance): 100 Hz → Medium
- Navigation: 10 Hz → Lower
- Mission: 1 Hz → Lowest
```

### Industrial PLC

```
Priority levels:
- Fast logic: 1-10ms cycle → High
- Normal logic: 10-100ms → Medium
- Slow logic: 100ms-1s → Low
- Communication: sporadic → Lowest
```

---

## Pytania do przemyślenia

1. Jakie taski w Twoim systemie mają najkrótszy period? Czy mają najwyższy priorytet?
2. Jaka jest utilisacja Twojego systemu? Czy poniżej 69%?
3. Czy któryś task ma deadline < period? Może potrzebujesz DM zamiast RMS?

---

## Quiz

**Pytanie**: Masz taski:

```
Task A: Period=5ms, WCET=1ms
Task B: Period=10ms, WCET=2ms
Task C: Period=20ms, WCET=5ms
```

Określ priorytety RMS i sprawdź schedulability.

**Odpowiedź**:

```
Priorytety RMS (krótszy period = wyższy priorytet):
- Task A: Period=5ms  → Priority HIGH (1)
- Task B: Period=10ms → Priority MEDIUM (2)
- Task C: Period=20ms → Priority LOW (3)

Utilisacja:
U = 1/5 + 2/10 + 5/20 = 0.2 + 0.2 + 0.25 = 0.65 = 65%

RMS bound dla N=3: 78%

65% < 78% → SCHEDULABLE ✓ (sufficient condition)
```

---

## Wskazówka zapamiętywania

> **RMS = Rate (częstotliwość) wygrywa**
>
> Wyobraź sobie szpital:
> - Pacjent z zawałem przychodzi co 5 minut → High priority
> - Pacjent z grypą przychodzi co 30 minut → Low priority
>
> Kto ma być obsłużony pierwszego?
> Ten, który przychodzi częściej - bo inaczej kolejka rośnie.
>
> RMS to "kolejka w szpitalu" dla tasków.