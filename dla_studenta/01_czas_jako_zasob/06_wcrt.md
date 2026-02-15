# WCRT (Worst Case Response Time)

## Definicja

**WCRT** to najdłuższy możliwy czas od momentu, gdy zadanie staje się gotowe do wykonania, do momentu jego zakończenia. WCRT = czas oczekiwania + czas wykonania.

> WCRT to "pełna historia" zadania: ile czekało na procesor, ile razy zostało wywłaszczone, i ile faktycznie wykonywało kod.

```
Zadanie gotowe
     │
     ├────────── Czekanie ──────────┼── Wykonanie ──┤
     │                              │               │
     │◄──────────── WCRT ──────────────────────────►│
     │                              │               │
     ▼                              ▼               ▼
  Release                      Start           Complete
```

---

## Analogia do przyrody

### 🏥 Szpitalna SOR

Pacjent przychodzi na SOR:

```
Przyjście (release) ──► Czekanie ──► Leczenie ──► Wypis
                              │
                              ▼
                     WCRT = czas od przyjścia do wypisu
```

WCRT zależy od:
- Ilu pacjentów jest przed nim (innych tasków)
- Jak pilny jest przypadek (priorytet)
- Jak długo trwa leczenie (WCET)
- Czy przyjdzie ktoś bardziej pilny (preempcja)

### 🚦 Sygnalizacja świetlna

Samochód dojeżdża do światła:

```
Dojazd ──► Czerwone ──► Zielone ──► Przejazd
              │            │
              └────────────┘
                    WCRT
```

WCRT zależy od:
- Czasu do zmiany świateł
- Ile aut czeka przed tobą
- Czy nadjeżdża karetka (preempcja)

### 🐕 Psy zaprzęgowe

Każdy pies w zaprzęgu ma swoją rolę. WCRT "zadania" psa to czas od komendy do wykonania, uwzględniający:
- Inne psy (inne taski)
- Teren (zasoby)
- Zmęczenie (interferencja)

---

## Podobieństwo do systemów informatycznych

### Web Request

```
User click ──► DNS ──► TCP ──► Queue ──► Server ──► DB ──► Response
                 │                               │
                 └─────────── WCRT ──────────────┘

WCRT = Network latency + Queue wait + Processing + DB query
```

W SLA to jest "Response Time" - najbardziej miarodajna metryka.

### CI/CD Pipeline

```
Commit ──► Queue ──► Build ──► Test ──► Deploy
              │                            │
              └────────── WCRT ────────────┘

WCRT = Queue time + Build time + Test time + Deploy time
```

Jeśli queue jest zapchane, WCRT rośnie mimo że build/test są szybkie.

### Database Query

```
Query arrives ──► Parse ──► Plan ──► Execute ──► Return
                    │                           │
                    └───────── WCRT ────────────┘

WCRT zależy od:
- Obciążenia serwera (queue)
- Złożoności zapytania (execution)
- Blokad (locks)
```

---

## WCRT vs WCET

```
┌─────────────────────────────────────────────────────────────┐
│                         WCRT                                │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  │◄───── Waiting ─────►│◄── Preemption ──►│◄── WCET ──►│   │
│  │                     │                  │            │   │
│  ▼                     ▼                  ▼            ▼   │
│ Release ──────────► Scheduler ──────► Resume ──────► Complete│
│                                                             │
│  WCRT = Waiting + Preemption + WCET                        │
└─────────────────────────────────────────────────────────────┘
```

**WCET** = czas samego wykonania kodu (bez czekania)
**WCRT** = czas od release do complete (z czekaniem)

---

## Jak obliczyć WCRT?

### Dla pojedynczego zadania

```
WCRT = WCET

(Prosty przypadek - brak innych zadań)
```

### Z wywłaszczaniem (preemption)

```
WCRT = WCET + Σ(Interference)

Gdzie Interference to czas, gdy wyższy priorytet zabiera CPU
```

### Response Time Analysis (RTA)

Dla zadania i w systemie z zadaniami o wyższym priorytecie:

```
R_i = C_i + Σ(R_i / T_j) × C_j

Gdzie:
- R_i = Response time zadania i (to szukamy!)
- C_i = WCET zadania i
- T_j = Period zadań o wyższym priorytecie
- C_j = WCET zadań o wyższym priorytecie

Iteracyjne rozwiązanie:
R_i^(n+1) = C_i + Σ(ceil(R_i^n / T_j) × C_j)
```

### Przykład RTA

```
Zadanie A: priorytet wysoki, T=10ms, C=2ms
Zadanie B: priorytet średni, T=20ms, C=3ms
Zadanie C: priorytet niski, T=50ms, C=10ms

WCRT(A) = C(A) = 2ms  (brak wyższego priorytetu)

WCRT(B):
R(B)^0 = C(B) = 3ms
R(B)^1 = 3 + ceil(3/10) × 2 = 3 + 2 = 5ms
R(B)^2 = 3 + ceil(5/10) × 2 = 3 + 2 = 5ms  ← Zbieżność!
WCRT(B) = 5ms

WCRT(C):
R(C)^0 = C(C) = 10ms
R(C)^1 = 10 + ceil(10/10) × 2 + ceil(10/20) × 3 = 10 + 2 + 3 = 15ms
R(C)^2 = 10 + ceil(15/10) × 2 + ceil(15/20) × 3 = 10 + 4 + 3 = 17ms
R(C)^3 = 10 + ceil(17/10) × 2 + ceil(17/20) × 3 = 10 + 4 + 3 = 17ms  ← Zbieżność!
WCRT(C) = 17ms
```

---

## Dlaczego WCRT jest trudny?

### Problem 1: Priorytety

```
Niski priorytet może czekać bardzo długo:
│
│ High prio task: ████░░░░░░░░████░░░░░░░░████
│ Med prio task:  ░░░░████░░░░░░░░████░░░░░░░░
│ Low prio task:  ░░░░░░░░████████████████░░░░
│                              │
│                              └── Low musi czekać!
```

### Problem 2: Blokowanie

```
Zadanie A (high) ──► Mutex ──► BLOCKED
                              │
Zadanie B (low)  ──► Hold mutex ──► Blokuje A!
                              │
                              └── Priority Inversion!
```

### Problem 3: Kaskada

```
A czeka na B, B czeka na C, C czeka na D...
│
└──► WCRT(A) zależy od całego łańcucha!
```

---

## WCRT a Schedulability

Zadanie jest schedulable jeśli:

```
WCRT ≤ Deadline
```

Sprawdzenie dla wszystkich zadań:

```
Dla każdego zadania i:
  WCRT(i) ≤ Deadline(i)
```

Jeśli wszystkie zadania spełniają ten warunek → system jest schedulable.

---

## Jak zmniejszyć WCRT?

### 1. Zmniejsz WCET

```c
// Optymalizacja kodu
void optimized_task(void) {
    // Zamiast:
    for (int i = 0; i < n; i++) {
        process(data[i]);
    }

    // Użyj:
    process_batch(data, n);  // Szybciej, mniejszy WCET
}
```

### 2. Zwiększ priorytet

```
Low prio:  WCRT = 100ms
High prio: WCRT = 5ms

Ale uwaga: zbyt wiele wysokich priorytetów → problemy innych tasków!
```

### 3. Podziel zadanie

```c
// ZŁE: Długie zadanie
void big_task(void) {
    process_all_data();  // WCET = 50ms
}

// DOBRE: Podzielone zadanie
void small_task(void) {
    process_one_batch();  // WCET = 5ms
    schedule_next_batch();
}
```

### 4. Unikaj blokowania

```c
// ZŁE: Długa sekcja krytyczna
mutex_lock(&m);
process_all_data();  // Inni czekają!
mutex_unlock(&m);

// DOBRE: Krótka sekcja krytyczna
Data* data;
mutex_lock(&m);
data = get_pointer();  // Szybko!
mutex_unlock(&m);
process_data(data);  // Poza mutexem
```

---

## Narzędzia do analizy WCRT

### Static Analysis Tools

- **MAST** (Modeling and Analysis Suite for Real-Time Applications)
- **SymTA/S** (Symbolic Timing Analysis)
- **RapidRMA** (Rate Monotonic Analysis)

### Simulation

```python
# Symulacja systemu RTOS
def simulate_wcrt(tasks, duration):
    for t in range(duration):
        # Symuluj scheduling
        # Śledź czasy oczekiwania
        # Znajdź najgorszy przypadek
    return worst_case_response_time
```

### Trace Analysis

```
Trace log:
Task A: release=0, start=0, complete=2
Task B: release=0, start=2, complete=7  ← waited 2ms
Task C: release=5, start=7, complete=10 ← waited 2ms
...
WCRT = max(complete - release)
```

---

## Jak świat radzi sobie z WCRT?

### Automotive: Engine Control

```
Deadline: 1 engine cycle (np. 10ms przy 6000 RPM)

WCRT musi być < deadline dla każdego cyklu:
- Odczyt sensorów
- Obliczenia
- Aktuator (wtrysk, zapłon)

Naruszenie deadline = nierówna praca silnika lub awaria
```

### Aerospace: Flight Control

```
Control loop: 50Hz (20ms period)

WCRT analysis:
- Sensor fusion: 2ms WCET
- Control law: 3ms WCET
- Actuator command: 1ms WCET
- Interference: max 5ms

WCRT = 2 + 3 + 1 + 5 = 11ms < 20ms deadline ✓
```

### Industrial: PLC

```
Cycle time: 10ms

Każdy rung ladder logic musi mieć znany WCRT.
Jeśli WCRT > cycle time → watchdog reset.
```

---

## WCRT w praktyce - checklist

```
□ Czy znasz WCET każdego zadania?
□ Czy znasz priorytety wszystkich zadań?
□ Czy przeprowadziłeś RTA?
□ Czy WCRT < Deadline dla wszystkich zadań?
□ Czy uwzględniłeś blokowanie (mutexy)?
□ Czy uwzględniłeś przerwania?
□ Czy masz margines bezpieczeństwa (np. 20%)?
```

---

## Pytania do przemyślenia

1. Jakie jest WCRT dla Twojego najbardziej krytycznego zadania?
2. Jak to obliczyłeś - pomiar czy analiza?
3. Jakie zadania mogą interferować z Twoim zadaniem?

---

## Quiz

**Pytanie**: Masz trzy zadania:

```
Task A: T=5ms, C=1ms, prio=high
Task B: T=10ms, C=2ms, prio=medium
Task C: T=20ms, C=4ms, prio=low
```

Czy system jest schedulable? (Deadline = Period)

**Odpowiedź**:

```
WCRT(A) = 1ms < 5ms ✓

WCRT(B):
R^0 = 2
R^1 = 2 + ceil(2/5) × 1 = 3
R^2 = 2 + ceil(3/5) × 1 = 3 ← OK
WCRT(B) = 3ms < 10ms ✓

WCRT(C):
R^0 = 4
R^1 = 4 + ceil(4/5) × 1 + ceil(4/10) × 2 = 4 + 1 + 2 = 7
R^2 = 4 + ceil(7/5) × 1 + ceil(7/10) × 2 = 4 + 2 + 2 = 8
R^3 = 4 + ceil(8/5) × 1 + ceil(8/10) × 2 = 4 + 2 + 2 = 8 ← OK
WCRT(C) = 8ms < 20ms ✓

System jest schedulable! ✓
```

---

## Wskazówka zapamiętywania

> **WCRT = WCET + Czekanie na kolegów**
>
> Wyobraź sobie kolejkę w sklepie:
> - WCET = czas obsługi przy kasie
> - WCRT = czas od wejścia do sklepu do wyjścia
>
> WCET mówi, jak szybko kasjer cię obsłuży.
> WCRT mówi, jak szybko wyjdziesz z zakupami.
>
> WCRT zawsze ≥ WCET.