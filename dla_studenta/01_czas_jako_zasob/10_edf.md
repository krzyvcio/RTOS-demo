# EDF (Earliest Deadline First)

## Definicja

**EDF** to dynamiczny algorytm schedulingu, w którym task z najwcześniejszym deadline zawsze ma najwyższy priorytet. W przeciwieństwie do RMS, priorytety zmieniają się w czasie działania.

> EDF to "dyktatura deadline": kto ma najwcześniejszy deadline, ten rządzi. Priorytety są płynne, deadline jest królem.

```
┌─────────────────────────────────────────────────────────┐
│              EDF - DYNAMIC PRIORITIES                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Time: 0ms                                              │
│  Task A: deadline=5ms   ─► Earliest! ─► Run A          │
│  Task B: deadline=10ms                                  │
│  Task C: deadline=8ms                                   │
│                                                         │
│  Time: 3ms (A completed)                                │
│  Task B: deadline=10ms                                  │
│  Task C: deadline=8ms   ─► Earliest! ─► Run C          │
│                                                         │
│  Time: 6ms (C completed)                                │
│  Task B: deadline=10ms  ─► Earliest! ─► Run B          │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🏥 Szpitalna kolejka

Każdy pacjent ma "deadline" - czas, do którego musi być obsłużony:

```
Pacjent A: Zawał serca   → Deadline: TERAZ (5 min)
Pacjent B: Złamana noga  → Deadline: 1 godzina
Pacjent C: Grypa         → Deadline: 3 godziny

EDF: Obsługuj pacjenta z najwcześniejszym deadline.
= Triage w szpitalu
```

To nie jest "kto pierwszy ten lepszy" - to "kogo najbardziej się spieszy".

### 🐝 Pszczoły i kwiaty

Pszczoły zbierają nektar z kwiatów:

```
Kwiat A: Otwarty do 10:00  → Deadline: 10:00
Kwiat B: Otwarty do 12:00  → Deadline: 12:00
Kwiat C: Otwarty do 11:00  → Deadline: 11:00

EDF: Odwiedź kwiat A, potem C, potem B.
= Maksymalizuj zbiory
```

### 🌅 Zwierzęta i zachód słońca

Zwierzęta muszą znaleźć schronienie przed zachodem:

```
Wilk:       Musi wrócić do 20:00
Królik:     Musi schować się do 19:00
Jeleń:      Musi być w stadzie do 19:30

EDF: Królik → Jeleń → Wilk
= Kto ma najwcześniejszy "deadline"
```

---

## Podobieństwo do systemów informatycznych

### Task Management w project management

```
Task A: Deadline piątek 17:00
Task B: Deadline czwartek 12:00
Task C: Deadline piątek 10:00

EDF: B → C → A
= Pracuj nad taskiem z najwcześniejszym deadline
```

### Food Delivery

```
Zamówienie 1: Deadline 12:30 (20 min)
Zamówienie 2: Deadline 12:45 (35 min)
Zamówienie 3: Deadline 12:25 (15 min) ← Najwcześniejszy!

EDF: Dostarcz zamówienie 3 pierwsze
= Minimalizuj spóźnienia
```

### Video Rendering

```cpp
// Klatki do renderowania
Frame 1: Deadline t=33ms (frame 1)
Frame 2: Deadline t=66ms (frame 2)
Frame 3: Deadline t=33ms (frame 1, retry) ← Deadline już minął!

EDF: Renderuj frame 3 first (najwcześniejszy deadline)
= Ratuj spóźnione klatki
```

---

## Matematyka EDF

### Zasada działania

```
Priorytet(i, t) = 1 / Deadline(i, t)

Deadline(i, t) = Release(i) + Relative_Deadline(i)

W każdym momencie t:
- Wybierz task z najmniejszym Deadline(i, t)
- Ten task ma najwyższy priorytet
```

### EDF Utilization Bound

```
Dla EDF:
U ≤ 1.0 (100%)

To jest warunek konieczny I wystarczający!

Jeśli utilisacja ≤ 100% → System JEST schedulable pod EDF.
Jeśli utilisacja > 100% → System NIE JEST schedulable pod żadnym algorytmem.
```

### EDF vs RMS Efficiency

```
RMS: max ~69% utilisacji
EDF: max 100% utilisacji

EDF jest OPTYMALNY:
- Jeśli system jest schedulable pod jakimkolwiek algorytmem,
- to jest schedulable pod EDF.

RMS jest SUB-OPTYMALNY:
- Może nie schedulować systemu, który jest schedulable pod EDF.
```

---

## EDF w praktyce

### Przykład 1: Prosty system

```
Time 0:
Task A: Release=0, Deadline=4, WCET=1
Task B: Release=0, Deadline=6, WCET=2
Task C: Release=0, Deadline=8, WCET=3

EDF scheduling:
t=0: A ma deadline 4 (earliest) → Run A (t=0-1)
t=1: B ma deadline 6 (earliest of remaining) → Run B (t=1-3)
t=3: C ma deadline 8 → Run C (t=3-6)
Wszystkie deadline dotrzymane ✓
```

### Przykład 2: Dynamiczne priorytety

```
Time 0:
Task A: Release=0, Deadline=5, WCET=2
Task B: Release=0, Deadline=10, WCET=4

Time 0: A (deadline=5) → Run A (t=0-2)
Time 2: B (deadline=10) → Run B (t=2-4)

Time 4: Nowy A release! A (deadline=9)
B ma deadline=10, A ma deadline=9
EDF: A earlier → Switch to A!

Time 4-6: Run A (deadline=9)
Time 6-8: Run B (remaining 2 units)
```

### Wykonywanie w czasie

```
┌─────────────────────────────────────────────────────────┐
│              EDF TIMELINE                               │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  t=0    t=2    t=4    t=6    t=8    t=10                │
│  │      │      │      │      │      │                  │
│  ├──────┼──────┼──────┼──────┼──────┤                  │
│  │  A   │  B   │  A   │  B   │  B   │                  │
│  │      │      │(new) │      │      │                  │
│                                                         │
│  Priorytety dynamiczne:                                 │
│  t=0: A(d=5) < B(d=10) → A first                       │
│  t=2: B(d=10) only → B                                  │
│  t=4: A(d=9) < B(d=10) → A preempts B!                 │
│  t=6: B(d=10) only → B                                  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## EDF vs RMS - Porównanie

### Przykład: RMS fail, EDF success

```
Task A: Period=2, WCET=1
Task B: Period=5, WCET=2

Utilisacja: 1/2 + 2/5 = 0.5 + 0.4 = 0.9 = 90%

RMS:
- RMS bound dla N=2: 82.8%
- 90% > 82.8% → Może nie być schedulable
- RTA: A ma deadline 2, interference od A na B...
- B miss deadline przy t=5!

EDF:
- U = 90% < 100%
- Gwarantowane schedulable ✓

EDF wykorzystuje CPU lepiej!
```

### Graficzne porównanie

```
┌─────────────────────────────────────────────────────────┐
│              RMS vs EDF UTILIZATION                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  100% ├─────────────────────────────────────────────    │
│       │                                    EDF          │
│   90% │                                   ████████     │
│       │                                  ██████████    │
│   80% │                    RMS          ████████████   │
│       │                  ████████      ██████████████  │
│   70% │                ████████████   ████████████████ │
│       │              ██████████████████████████████████│
│   60% │            ████████████████████████████████████│
│       │          ██████████████████████████████████████│
│       └─────────────────────────────────────────────    │
│        1 task    2 tasks    5 tasks    10 tasks         │
│                                                         │
│  EDF zawsze 100%, RMS maleje z liczbą tasków           │
└─────────────────────────────────────────────────────────┘
```

---

## Dlaczego EDF nie jest zawsze używany?

### Wady EDF

#### 1. Złożoność implementacji

```c
// RMS: proste
priority = 1 / period;  // Stałe!

// EDF: skomplikowane
for each task:
    deadline = release_time + relative_deadline;
priority = 1 / min(all_deadlines);  // Dynamiczne!
// Musi być przeliczane przy każdym scheduling decision
```

#### 2. Overhead

```
RMS: O(1) scheduling decision (stały priorytet)
EDF: O(n) scheduling decision (znajdź min deadline)

Dla wielu tasków, EDF ma wyższy overhead.
```

#### 3. Nieprzewidywalność przy przeciążeniu

```
Gdy U > 100%:

RMS:
- Low priority tasks miss deadline
- High priority tasks OK
- Przewidywalna degradacja

EDF:
- Wszystkie taski mogą miss deadline
- Nieprzewidywalna degradacja
- Domino effect
```

#### 4. Brak implementacji w wielu RTOS

```
FreeRTOS: Tylko RMS (priority-based)
VxWorks: Tylko RMS
Zephyr: Tylko RMS

EDF jest rzadziej wspierany.
```

---

## EDF Implementation

### Pseudokod

```c
typedef struct {
    void (*function)(void);
    uint32_t period;
    uint32_t wcet;
    uint32_t relative_deadline;
    uint32_t next_release;
    uint32_t absolute_deadline;
} Task;

void edf_scheduler(Task tasks[], int n) {
    while (1) {
        uint32_t current_time = get_time();

        // Aktualizuj release times i deadlines
        for (int i = 0; i < n; i++) {
            if (current_time >= tasks[i].next_release) {
                tasks[i].next_release += tasks[i].period;
                tasks[i].absolute_deadline =
                    tasks[i].next_release + tasks[i].relative_deadline;
            }
        }

        // Znajdź task z najwcześniejszym deadline (który jest gotowy)
        Task* earliest = NULL;
        for (int i = 0; i < n; i++) {
            if (current_time >= tasks[i].next_release - tasks[i].period) {
                // Task jest gotowy
                if (earliest == NULL ||
                    tasks[i].absolute_deadline < earliest->absolute_deadline) {
                    earliest = &tasks[i];
                }
            }
        }

        if (earliest) {
            // Preempt jeśli inny task ma wcześniejszy deadline
            earliest->function();
        }
    }
}
```

---

## EDF Variants

### 1. EDF with Preemption

```
Standard EDF:
- Może preemptować running task
- Gdy nowy task ma wcześniejszy deadline
- Maksymalna responsywność
```

### 2. Non-preemptive EDF

```
Gdy task startuje:
- Biega do completion
- Brak preemption
- Prostszy, ale może miss deadline
```

### 3. EDF with Budget

```
Każdy task ma budżet:
- Gdy budżet exhausted → suspend
- Chroni przed runaway tasks
- ARINC 653 style
```

---

## EDF w świecie rzeczywistym

### Linux with SCHED_DEADLINE

```c
// Linux EDF scheduling
struct sched_attr attr = {
    .size = sizeof(attr),
    .sched_policy = SCHED_DEADLINE,
    .sched_runtime = 10 * 1000 * 1000,  // 10ms WCET
    .sched_deadline = 30 * 1000 * 1000, // 30ms deadline
    .sched_period = 30 * 1000 * 1000,   // 30ms period
};

sched_setattr(0, &attr, 0);
```

### Real-time frameworks

```
Some RTOS z EDF:
- MarteOS (Minimal Real-Time OS)
- Erika Enterprise
- Some research RTOS

Ale większość komercyjnych RTOS używa RMS/priority-based.
```

---

## Kiedy używać EDF?

### Użyj EDF gdy:

```
✓ Wysoka utilisacja (> 70%)
✓ Zmienne deadline (nie równe period)
✓ Sporadic tasks (nieregularne)
✓ Systemy z dynamicznym obciążeniem
✓ Możesz pozwolić sobie na złożoność
```

### Użyj RMS gdy:

```
✓ Prostota implementacji
✓ Przewidywalność ważniejsza niż utilisacja
✓ Stałe priorytety pożądane
✓ Niska utilisacja (< 70%)
✓ Standardowy RTOS (FreeRTOS, VxWorks)
```

---

## EDF Analysis

```python
def edf_analysis(tasks):
    """
    Analiza schedulability dla EDF
    tasks: lista (period, wcet, deadline) w ms
    """
    utilization = sum(wcet/period for period, wcet, _ in tasks)

    print(f"Utilisacja: {utilization*100:.1f}%")

    if utilization <= 1.0:
        print("✓ SCHEDULABLE pod EDF")
        return True
    else:
        print("✗ NIE schedulable (przeciążenie)")
        return False

# Przykład
tasks = [
    (2, 1, 2),   # Period=2, WCET=1, Deadline=2
    (5, 2, 5),   # Period=5, WCET=2, Deadline=5
    (10, 3, 10), # Period=10, WCET=3, Deadline=10
]

edf_analysis(tasks)
# Utilisacja: 90%
# ✓ SCHEDULABLE pod EDF
```

---

## Pytania do przemyślenia

1. Czy Twój system ma wysoką utilisację (>70%)? Może EDF byłby lepszy?
2. Czy deadline są różne od periodów? EDF to obsługuje lepiej.
3. Czy potrzebujesz przewidywalnej degradacji przy przeciążeniu? RMS może być lepszy.

---

## Quiz

**Pytanie**: Masz system:

```
Task A: Period=4ms, WCET=2ms
Task B: Period=6ms, WCET=2ms
Task C: Period=8ms, WCET=2ms
```

Czy system jest schedulable pod EDF? A pod RMS?

**Odpowiedź**:

```
Utilisacja:
U = 2/4 + 2/6 + 2/8 = 0.5 + 0.333 + 0.25 = 1.083 = 108.3%

EDF: U > 100% → NIE schedulable
RMS: U > 100% → NIE schedulable

Żaden algorytm nie uratuje tego systemu!
System jest fundamentalnie przeciążony.

Rozwiązanie:
- Zmniejsz WCET (optymalizuj kod)
- Zwiększ periody (zmniejsz częstotliwość)
- Dodaj CPU (więcej zasobów)
```

---

## Wskazówka zapamiętywania

> **EDF = Earliest Deadline First = Kto się najbardziej spieszy**
>
> Wyobraź siebie szefa kuchni:
> - Danie A: musi wyjść za 5 minut
> - Danie B: musi wyjść za 15 minut
> - Danie C: musi wyjść za 10 minut
>
> Które robisz pierwsze? A (deadline najwcześniejszy).
>
> W RMS patrzyłbyś na "jak często to danie zamawiają" - co nie ma sensu w kuchni!
>
> EDF to "kuchenny rozsądek" dla CPU.