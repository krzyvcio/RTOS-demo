# Schedulability (Planowalność)

## Definicja

**Schedulability** to właściwość systemu określająca, czy wszystkie zadania będą w stanie dotrzymać swoich deadline. System jest schedulable jeśli wszystkie taski mają gwarantowane czas procesora w wymaganych terminach.

> Schedulability to odpowiedź na pytanie: "Czy ten system ma szansę działać poprawnie?" Jeśli nie, nie ma sensu go implementować.

```
┌─────────────────────────────────────────────────────────┐
│                  SCHEDULABILITY TEST                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task A: period=10ms, WCET=3ms  ──► Deadline OK? ✓      │
│  Task B: period=20ms, WCET=5ms  ──► Deadline OK? ✓      │
│  Task C: period=50ms, WCET=10ms ──► Deadline OK? ✓      │
│                                                         │
│  System Utilization: 3/10 + 5/20 + 10/50 = 75%         │
│                                                         │
│  ┌─────────────────────────────────────────────┐       │
│  │           SCHEDULABLE ✓                     │       │
│  │                                             │       │
│  │  Wszystkie deadline będą dotrzymane         │       │
│  └─────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🐜 Mrówcza kolonia

Mrówki zbierają jedzenie dla kolonii:

```
Mrówka A: potrzebuje 3g/s, może wracać co 10s
Mrówka B: potrzebuje 2g/s, może wracać co 15s
Mrówka C: potrzebuje 1g/s, może wracać co 30s

Czy kolonia przetrwa?
= Schedulability test dla mrówek
```

Jeśli wymagania mrówek > dostępne jedzenie → kolonia upadnie.

### 🌳 Las i słońce

Drzewa w lesie konkurują o światło:

```
Drzewo A: potrzebuje 4h słońca/dzień
Drzewo B: potrzebuje 3h słońca/dzień
Drzewo C: potrzebuje 2h słońca/dzień

Dzień ma 12h nasłonecznienia
4 + 3 + 2 = 9h < 12h → Wszystkie przetrwają ✓
```

To jest schedulability analysis dla drzew!

### 🐝 Pszczoły i nektar

```
Pszczoły potrzebują nektaru:
- Na zimę: 20kg miodu
- Sezon: 100 dni
- Każda pszczoła zbiera: 0.5g/dzień

Potrzeba: 200,000 pszczołodni
Dostępne: 150,000

Nie schedulable! Kolonia nie przetrwa zimy.
```

---

## Podobieństwo do systemów informatycznych

### Server Capacity Planning

```
Serwer WWW:
- Request A: 50ms CPU time, 100 req/s
- Request B: 20ms CPU time, 500 req/s
- Request C: 100ms CPU time, 50 req/s

Wymagania: 50×100 + 20×500 + 100×50 = 20,000ms/s = 20 CPU cores

Dostępne: 16 cores

Nie schedulable! Potrzebujesz więcej serverów.
```

### CI/CD Pipeline

```
Build jobs:
- Unit tests: 5min, co 10min
- Integration tests: 15min, co 30min
- Deploy: 2min, co 1h

Czy jeden runner wystarczy?

U = 5/10 + 15/30 + 2/60 = 0.5 + 0.5 + 0.033 = 103.3%

Nie schedulable! Potrzeba więcej runnerów.
```

### Database Connections

```
Connection pool: 100 connections

Aplikacje:
- App A: potrzebuje 30 connections
- App B: potrzebuje 40 connections
- App C: potrzebuje 50 connections

Razem: 120 > 100

Nie schedulable! Connection starvation.
```

---

## Matematyka Schedulability

### Utilization-based Test (Necessary Condition)

```
Dla systemu z N tasków:

U = Σ(Ci / Ti)

Gdzie:
- Ci = WCET tasku i
- Ti = Period tasku i
- U = Całkowita utilisacja

U ≤ 1.0 (100%) jest warunkiem koniecznym

Jeśli U > 1.0 → Nie schedulable (nie ma szans)
```

### Rate Monotonic Scheduling (RMS)

Dla RMS (taski o krótszym okresie mają wyższy priorytet):

```
Sufficient condition:
U ≤ N(2^(1/N) - 1)

Dla N tasków:
N=1: U ≤ 1.000 (100%)
N=2: U ≤ 0.828 (82.8%)
N=3: U ≤ 0.780 (78.0%)
N=4: U ≤ 0.757 (75.7%)
N→∞: U ≤ 0.693 (69.3% = ln(2))
```

### Earliest Deadline First (EDF)

Dla EDF (dynamiczny priorytet wg deadline):

```
Sufficient and necessary condition:
U ≤ 1.0 (100%)

EDF jest optymalny - jeśli system jest schedulable
pod jakimkolwiek algorytmem, jest schedulable pod EDF.
```

---

## Schedulability Test - Przykład

### System do analizy

```
Task T1: C=1ms, T=4ms, D=4ms (deadline=period)
Task T2: C=2ms, T=6ms, D=6ms
Task T3: C=3ms, T=8ms, D=8ms
```

### Test 1: Utilization

```
U = C1/T1 + C2/T2 + C3/T3
  = 1/4 + 2/6 + 3/8
  = 0.25 + 0.333 + 0.375
  = 0.958 (95.8%)

U ≤ 1.0 ✓ (Warunek konieczny spełniony)
```

### Test 2: RMS Sufficient

```
Dla N=3:
Umax = 3(2^(1/3) - 1) = 0.780

U = 0.958 > 0.780

Test RMS sufficient NIE spełniony.
Może być schedulable, może nie - trzeba sprawdzić dokładniej.
```

### Test 3: Response Time Analysis

```
Priorytety wg RMS (krótszy period = wyższy priorytet):
T1 (prio high), T2 (prio med), T3 (prio low)

WCRT(T1) = C1 = 1ms ≤ D1=4ms ✓

WCRT(T2):
R2 = C2 + ceil(R2/T1) × C1
R2⁰ = 2
R2¹ = 2 + ceil(2/4) × 1 = 2 + 1 = 3
R2² = 2 + ceil(3/4) × 1 = 2 + 1 = 3 ← Zbieżność
WCRT(T2) = 3ms ≤ D2=6ms ✓

WCRT(T3):
R3 = C3 + ceil(R3/T1) × C1 + ceil(R3/T2) × C2
R3⁰ = 3
R3¹ = 3 + ceil(3/4) × 1 + ceil(3/6) × 2 = 3 + 1 + 2 = 6
R3² = 3 + ceil(6/4) × 1 + ceil(6/6) × 2 = 3 + 2 + 2 = 7
R3³ = 3 + ceil(7/4) × 1 + ceil(7/6) × 2 = 3 + 2 + 2 = 7 ← Zbieżność
WCRT(T3) = 7ms ≤ D3=8ms ✓

SCHEDULABLE! ✓
```

---

## Dlaczego schedulability jest trudne?

### Problem 1: Nieregularne taski

```
Task A: period=10ms ALE czasem burst 100 razy w 1ms
Task B: sporadic, nie ma okresu

Trudno zastosować klasyczne testy.
```

### Problem 2: Zależności

```
Task A produkuje dane dla Task B.
B nie może startować przed A.

Deadline B zależy od czasu wykonania A.
Klasyczna analiza nie wystarcza.
```

### Problem 3: Shared resources

```
Task A i B dzielą mutex.
Może wystąpić blokowanie.

WCRT musi uwzględniać blocking time.
```

### Problem 4: Overhead

```
Context switch: 10μs
ISR handling: 5μs
Scheduler: 20μs

Te narzuty nie są w modelu,
ale wpływają na schedulability.
```

---

## Jak zapewnić schedulability?

### 1. Redukcja WCET

```c
// Zanim:
void process(void) {
    for (int i = 0; i < 1000; i++) {
        slow_function(data[i]);
    }
}
// WCET = 1000 × 100μs = 100ms

// Po optymalizacji:
void process(void) {
    fast_batch_process(data, 1000);
}
// WCET = 10ms
```

### 2. Zmiana periodów

```
Zanim:
Task A: period=10ms, WCET=3ms → U=30%

Po:
Task A: period=20ms, WCET=3ms → U=15%

Mniejsza utilisacja = łatwiej schedulable
```

### 3. Zmiana priorytetów

```
RMS: Krótszy period = wyższy priorytet
Ale jeśli to nie działa → DM (Deadline Monotonic)

Deadline < Period → DM może być lepszy
```

### 4. Podział systemu

```
System A (krytyczny): Taski 1, 2, 3 → U = 60%
System B (niekrytyczny): Taski 4, 5, 6 → U = 40%

Odseparowane systemy = łatwiejsza analiza
```

---

## Schedulability w praktyce

### Automotive

```
Engine Control Unit:
- 50+ tasków
- Periody: 1ms - 100ms
- Utilisacja celowa: 60-70%

Dlaczego nie 95%? Margines na:
- Narzut systemowy
- Nieprzewidziane sytuacje
- Future extensions
```

### Aerospace

```
Flight Control:
- Triple modular redundancy
- Każda kopia: U = 40%
- Margines na:
  - Failover
  - Reconfiguration
  - Safety checks
```

### Consumer Electronics

```
Smartphone:
- 100+ procesów
- Dynamic priorities
- "Best effort" scheduling
- Schedulability "good enough"

Nie safety-critical → mniej rygorystyczne
```

---

## Narzędzia do analizy schedulability

| Narzędzie | Opis |
|-----------|------|
| MAST | Modelowanie i analiza RT |
| SymTA/S | Symulacja timing |
| Cheddar | Scheduling analysis |
| RapiTime | WCET + schedulability |
| ptolemy | Modelowanie systemów |

---

## Schedulability Dashboard

```
┌─────────────────────────────────────────────────────────┐
│                SCHEDULABILITY STATUS                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Utilization:     ████████████████░░░░  78.2%          │
│  RMS Bound:       █████████████░░░░░░░  69.3%          │
│                                                         │
│  Status: MARGINAL ⚠️                                    │
│                                                         │
│  Tasks at risk:                                         │
│  - Task_C: WCRT=48ms, Deadline=50ms (96% used)         │
│  - Task_F: WCRT=95ms, Deadline=100ms (95% used)        │
│                                                         │
│  Recommendation: Reduce WCET for Task_C and Task_F     │
└─────────────────────────────────────────────────────────┘
```

---

## Pytania do przemyślenia

1. Czy przeprowadziłeś schedulability analysis dla swojego systemu?
2. Jaka jest całkowita utilisacja? Jaki margines bezpieczeństwa?
3. Co się stanie, gdy dodasz nowy task?

---

## Quiz

**Pytanie**: Masz system z taskami:

```
Task 1: C=2ms, T=5ms
Task 2: C=3ms, T=10ms
Task 3: C=2ms, T=20ms
Task 4: C=1ms, T=50ms
```

Czy system jest schedulable pod RMS?

**Odpowiedź**:

```
Utilization:
U = 2/5 + 3/10 + 2/20 + 1/50
  = 0.4 + 0.3 + 0.1 + 0.02
  = 0.82 (82%)

RMS Bound dla N=4:
Umax = 4(2^(1/4) - 1) = 0.757 (75.7%)

82% > 75.7% → Test sufficient NIE spełniony

Sprawdź RTA:
WCRT(1) = 2 ≤ 5 ✓
WCRT(2) = 3 + ceil(R2/5)×2
  R2¹ = 3 + 2 = 5
  R2² = 3 + 2 = 5 ← OK
  WCRT(2) = 5 ≤ 10 ✓

WCRT(3) = 2 + ceil(R3/5)×2 + ceil(R3/10)×3
  R3¹ = 2 + 2 + 3 = 7
  R3² = 2 + 4 + 3 = 9
  R3³ = 2 + 4 + 3 = 9 ← OK
  WCRT(3) = 9 ≤ 20 ✓

WCRT(4) = 1 + ceil(R4/5)×2 + ceil(R4/10)×3 + ceil(R4/20)×2
  R4¹ = 1 + 2 + 3 + 2 = 8
  R4² = 1 + 4 + 3 + 2 = 10
  R4³ = 1 + 4 + 3 + 2 = 10 ← OK
  WCRT(4) = 10 ≤ 50 ✓

SCHEDULABLE! ✓ (mimo że U > RMS bound)
```

---

## Wskazówka zapamiętywania

> **Schedulability = Czy zmieści się w plecaku?**
>
> Masz plecak o pojemności 100%.
> Każdy task to przedmiot zajmujący Ci/Ti miejsca.
>
> Jeśli suma > 100% → Nie zmieścisz się (nie schedulable).
> Jeśli suma ≤ 100% → Może się zmieści (sprawdź dokładnie).
>
> RMS bound to jakby "bezpieczny limit" - 69.3% dla wielu przedmiotów.
> Jeśli spakujesz więcej, może się nie zmieścić mimo że teoretycznie mieści.