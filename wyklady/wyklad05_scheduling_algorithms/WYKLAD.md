# Wykład 5: Algorytmy schedulingu

**Czas:** 2 godziny (90 min)
**Tydzień:** 10

---

## Plan wykładu

1. **Wprowadzenie** (10 min) - Problem schedulingu
2. **Metryki** (10 min) - Utilization, response time
3. **Rate Monotonic Scheduling** (25 min) - RMS
4. **Earliest Deadline First** (25 min) - EDF
5. **Porównanie** (15 min) - RMS vs EDF
6. **Inne algorytmy** (10 min) - DM, Round Robin
7. **Podsumowanie** (5 min)

---

## Slajd 1: Tytuł

```
ALGORYTMY SCHEDULINGU W RTOS
Wykład 5: RMS i EDF

Rate Monotonic Scheduling
Earliest Deadline First
Analiza schedulability
```

---

## Slajd 2: Problem schedulingu

```
Mamy:
- N zadań z okresami Ti
- Czas wykonania Ci
- Deadline Di (zazwyczaj = Ti)

Pytanie:
Czy system jest SCHEDULABLE?
Czy wszystkie deadline będą dotrzymane?

To NIE jest pytanie "czy działa?"
To JEST pytanie "czy ZAWSZE działa?"
```

---

## Slajd 3: Metryki

### Utilization

```
Utilization = Σ(Ci / Ti)

Przykład:
Task A: C=2ms, T=10ms → U = 0.2
Task B: C=3ms, T=20ms → U = 0.15
Task C: C=5ms, T=50ms → U = 0.1

Total: U = 0.45 (45% CPU)

Pytanie: Czy 45% jest OK?
Odpowiedź: ZALEŻY OD ALGORYTMU!
```

### Response Time

```
Response Time = Release → Completion

Wczesniej: WCRT (Worst Case Response Time)

WCRT musi być ≤ Deadline
```

---

## Slajd 4: Rate Monotonic Scheduling (RMS)

### Zasada

```
Priorytet ∝ 1 / Period

KRÓTSZY OKRES = WYŻSZY PRIORYTET

Przykład:
Task A: T=10ms → High priority
Task B: T=20ms → Medium priority
Task C: T=50ms → Low priority

Statyczne priorytety - nie zmieniają się w runtime
```

### Właściwość

```
RMS jest OPTIMALNY wśród algorytmów
ze statycznymi priorytetami.

Jeśli system NIE jest schedulable pod RMS,
to NIE jest schedulable pod ŻADNYM
algorytmem ze statycznymi priorytetami.
```

---

## Slajd 5: RMS Utilization Bound

### Twierdzenie Liu & Layland (1973)

```
Dla systemu z N zadań:

U ≤ N(2^(1/N) - 1)

Jeśli utilization jest PONIŻEJ tego limitu,
system JEST schedulable.

Wartości:
N=1:  100.0%
N=2:   82.8%
N=3:   78.0%
N=5:   74.3%
N=10:  71.8%
N=∞:   69.3% = ln(2)
```

### Graficznie

```
Utilization Bound
100% ─┬────────────────────────────────
      │  •  (N=1)
 80% ─┤      •  (N=2)
      │          •  (N=3)
 70% ─┤              ••••••••••••  (N→∞)
      │                  = ln(2) ≈ 69.3%
 60% ─┤
      │
      └──────────────────────────────────
        1   2   3   5   10  ∞   Liczba tasków

WNIOSEK: Zawsze zostaw ~30% CPU jako margines!
```

---

## Slajd 6: RMS - Przykład

### System

```
Task 1: C=1ms, T=4ms
Task 2: C=2ms, T=6ms
Task 3: C=1ms, T=8ms
```

### Analiza

```
Priorytety (RMS):
T1 (T=4ms) → Highest
T2 (T=6ms) → Medium
T3 (T=8ms) → Lowest

Utilization:
U = 1/4 + 2/6 + 1/8 = 0.25 + 0.333 + 0.125 = 0.708 = 70.8%

RMS bound (N=3): 78.0%

70.8% < 78.0% → SCHEDULABLE ✓
```

### Timeline

```
t:  0   1   2   3   4   5   6   7   8   9  10  11  12
    ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
T1: │███│   │   │   │███│   │   │   │███│   │   │   │███
    │   │███│███│   │   │███│███│   │   │███│███│   │
T2: │   │   │   │███│   │   │   │   │   │   │   │███│
    │   │   │   │   │   │   │   │███│   │   │   │   │
T3: │   │   │   │   │   │   │   │   │   │   │   │   │
                                      ↑
                            T3 wykonuje się gdy T1 i T2 done
```

---

## Slajd 7: Earliest Deadline First (EDF)

### Zasada

```
Priorytet ∝ 1 / Deadline

NAJWCZEŚNIEJSZY DEADLINE = NAJWYŻSZY PRIORYTET

Dynamiczne priorytety - zmieniają się w runtime
```

### Przykład

```
Time t=0:
Task A: deadline=5ms  ← Earliest → Run
Task B: deadline=10ms
Task C: deadline=8ms

Time t=2 (A completed):
Task B: deadline=10ms
Task C: deadline=8ms   ← Earliest → Run

Priorytety DYNAMICZNE!
```

---

## Slajd 8: EDF Utilization Bound

### Twierdzenie

```
Dla EDF:

U ≤ 1.0 (100%)

To jest WARUNEK KONIECZNY I WYSTARCZAJĄCY!

Jeśli U ≤ 100% → SCHEDULABLE
Jeśli U > 100% → NIE SCHEDULABLE

EDF jest OPTIMALNY wśród WSZYSTKICH algorytmów.
```

### Dowód intuicyjny

```
EDF zawsze wybiera zadanie z najwcześniejszym deadline.
Gdyby istniał lepszy algorytm, musiałby wybrać
zadanie z PÓŹNIEJSZYM deadline w pewnym momencie.
Ale to pogarsza sytuację!
EDF jest więc optymalny.
```

---

## Slajd 9: EDF vs RMS - Porównanie

### Przykład: RMS fail, EDF success

```
Task A: C=1ms, T=2ms
Task B: C=2ms, T=5ms

Utilization:
U = 1/2 + 2/5 = 0.5 + 0.4 = 0.9 = 90%

RMS bound (N=2): 82.8%
90% > 82.8% → Test sufficient NIE spełniony

RMS timeline (MOŻE miss deadline):
t=0: A(d=2), B(d=5) → A runs
t=1: A(d=2), B(d=5) → A runs
t=2: A(d=4), B(d=5) → A runs
t=3: A(d=4), B(d=5) → B runs
t=5: A(d=6), B(d=10) → A runs
...
B może miss deadline!

EDF timeline (ZAWSZE OK):
t=0: A(d=2), B(d=5) → A runs (earliest deadline)
t=1: A(d=2), B(d=5) → A runs
t=2: A(d=4), B(d=5) → B runs (B has earlier deadline now!)
...
```

---

## Slajd 10: RMS vs EDF - Tabela

| Cecha | RMS | EDF |
|-------|-----|-----|
| Priorytety | Statyczne | Dynamiczne |
| Max utilization | ~69% (N→∞) | 100% |
| Implementacja | Prosta | Złożona |
| Overhead | Niski | Wyższy |
| Predictability | Wysoka | Średnia |
| Degradacja | Łagodna | Nagła |

---

## Slajd 11: Dlaczego RMS jest popularny?

```
ZALETY RMS:
✓ Prosty - priorytety stałe
✓ Przewidywalny - łatwa analiza
✓ Niski overhead - brak dynamicznych decyzji
✓ Łatwy debug - wiadomo co się wykonuje
✓ Przewidywalna degradacja przy przeciążeniu

WADE EDF:
✗ Złożony - ciągłe przeliczanie
✗ Nagła degradacja - przy U>100% chaos
✗ Wyższy overhead
✗ Trudniejszy debug
```

---

## Slajd 12: Deadline Monotonic (DM)

### Kiedy używać?

```
Gdy Deadline ≠ Period

Task A: T=10ms, D=5ms
Task B: T=20ms, D=3ms

RMS: A > B (krótszy period)
DM:  B > A (krótszy deadline)

Jeśli D < T → DM może być lepszy
```

### Twierdzenie

```
DM jest optimalny gdy D ≤ T dla wszystkich zadań.
(Jeśli deadline ≤ period)
```

---

## Slajd 13: Response Time Analysis (RTA)

### Dla dokładniejszej analizy

```
Dla zadania i:

R_i = C_i + Σ⌈R_i / T_j⌉ × C_j
      (dla j o wyższym priorytecie)

Iteracyjnie:
R_i^(n+1) = C_i + Σ⌈R_i^n / T_j⌉ × C_j

Gdy R stabilizuje się → WCRT

Jeśli WCRT ≤ Deadline → Schedulable
```

### Przykład

```
Task A: C=1, T=4, D=4, prio=high
Task B: C=2, T=6, D=6, prio=low

WCRT(A):
R^0 = 1
R^1 = 1 (brak wyższych) → WCRT(A) = 1 ≤ 4 ✓

WCRT(B):
R^0 = 2
R^1 = 2 + ⌈2/4⌉ × 1 = 3
R^2 = 2 + ⌈3/4⌉ × 1 = 3 → WCRT(B) = 3 ≤ 6 ✓
```

---

## Slajd 14: Praktyczne wskazówki

```
1. Używaj RMS gdy:
   - Utilization < 70%
   - Priorytety stałe są OK
   - Prostota ważna

2. Używaj EDF gdy:
   - Utilization > 70%
   - Deadline ≠ Period
   - Możesz zaakceptować złożoność

3. Zawsze zostaw margines:
   - Nie targetuj 100% utilization
   - 60-70% to bezpieczny zakres

4. Weryfikuj empirycznie:
   - RTA to teoria
   - Mierz na rzeczywistym hardware
```

---

## Slajd 15: Podsumowanie

```
KLUCZOWE WNIOSKI:

1. RMS: Statyczne priorytety, max ~69% util
2. EDF: Dynamiczne priorytety, max 100% util
3. RMS prostszy, EDF optymalniejszy
4. Zawsze analizuj schedulability PRZED implementacją
5. Zostaw margines bezpieczeństwa

NASTĘPNY WYKŁAD:
Priority Inversion - problem i rozwiązania
```

---

## Zadania dla studentów

```
1. Oblicz schedulability dla:
   Task A: C=3ms, T=10ms
   Task B: C=2ms, T=20ms
   Task C: C=2ms, T=50ms
   (RMS i EDF)

2. Dlaczego RMS bound maleje z liczbą tasków?

3. Zaimplementuj prosty scheduler EDF w pseudokodzie
```

---

## Pytania egzaminacyjne

1. Wyjaśnij zasadę Rate Monotonic Scheduling.
2. Podaj utilization bound dla RMS i EDF.
3. Porównaj RMS i EDF - zalety i wady każdego.
4. Co to jest Response Time Analysis?
5. Kiedy użyć DM zamiast RMS?