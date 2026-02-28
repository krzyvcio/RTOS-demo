# Wykład 2: Dynamika, siły, momenty, kontakt

## Czesc I: Wstep teoretyczny — dlaczego dynamika jest potrzebna

### 1.1 Geneza — kinematyka to za malo

Proszę wyobrazić sobie sytuację: zaprojektowałeś trajektorię ruchu ramienia — piękna, gładka, bez kolizji. Uruchamiasz robota — i co się dzieje?

Robot albo:
- Zbyt wolno przyspiesza (za mało momentu)
- Traci równowagę przy chodzie
- Wibracje przy wysokich prędkościach

**Dlaczego?** Bo trajektoria była "geometrycznie poprawna", ale nie uwzględniała **fizyki**.

### 1.2 Kinematyka vs dynamika

| Aspekt | Kinematyka | Dynamika |
|--------|-----------|----------|
| Pytanie | Gdzie jest? | Dlaczego się porusza? |
| Dane wejściowe | Pozycje, prędkości przegubów | Siły, momenty |
| Dane wyjściowe | Pozycje | Przyspieszenia |
| Równania | Algebra | Różniczkowe |

**Kinematyka mówi "gdzie". Dynamika mówi "dlaczego i ile trzeba przyłożyć".**

### 1.3 Co daje dynamika w praktyce

Profesor zawsze powtarza: **"Dynamika to most między planowaniem a sterowaniem."**

Bez dynamiki nie da się:
- Sterować momentem (torque control)
- Kompensować grawitacji
- Projektować stabilnego chodu
- Estymować sił kontaktu

---

## Czesc II: Rownania ruchu

### 2.1 Forma standardowa

Dla układu z przegubami (uogólnione współrzędne q):

```text
M(q) × q̈ + C(q, q̇) × q̇ + g(q) = τ + J_c(q)^T × f_c
```

**Wyjaśnienie:**

| Symbol | Znaczenie |
|--------|-----------|
| M(q) | Macierz mas (symetryczna, dodatnio określona) |
| C(q, q̇) × q̇ | Siły Coriolisa i odśrodkowe |
| g(q) | Grawitacja |
| τ | Moment/siły napędów |
| J_c | Jacobian kontaktu |
| f_c | Siły reakcji (kontakty) |

### 2.2 Przyklad dla jednego przegubu

Dla wahadła (1 stopień swobody):

```
m × l² × θ̈ + m × g × l × sin(θ) = τ
```

- Lewa strona: M × q̈ + g(q)
- Prawa strona: τ (moment silnika)

---

## Czesc III: Metody wyprowadzania

### 3.1 Lagrange vs Newton-Euler

**Równania Lagrange'a:**
- Wygodne do wyprowadzeń teoretycznych
- Naturalne dla q i energii
- Prowadzą bezpośrednio do postaci M, C, g

**Newton-Euler:**
- Bardziej "fizyczny" obraz: siły i momenty na każdej bryle
- Świetny do obliczeń rekursywnych w drzewach
- Algorytmy typu RNEA

### 3.2 Praktyczna implementacja

W bibliotekach robotyki najczęściej:
- Wewnątrz: Newton-Euler
- Na zewnątrz: M(q), g(q), inverseDynamics(q, q̇, q̈)

---

## Czesc IV: Algorytmy rekursywne

### 4.1 RNEA (Recursive Newton-Euler Algorithm)

```python
def RNEA(q, qd, qdd):
    # Forward pass: obliczanie prędkości i przyspieszeń
    for link in links:
        v_w = ...  # prędkość kątowa
        a_w = ...  # przyspieszenie kątowe
        
    # Backward pass: obliczanie sił i momentów
    for link in reversed(links):
        F = ...  # siła
        tau = ...  # moment
        
    return tau
```

**Służy do:** obliczania momentów dla zadanych (q, q̇, q̈) — inverse dynamics

### 4.2 CRBA (Composite Rigid Body Algorithm)

```python
def CRBA(q):
    # Obliczanie macierzy mas M(q)
    M = zeros(n, n)
    for link in links:
        # ... dodawanie wkładów do M
    return M
```

**Służy do:** obliczania M(q) w czasie O(n)

### 4.3 ABA (Articulated Body Algorithm)

```python
def ABA(q, qd, tau):
    # Obliczanie qdd dla zadanego tau
    # (forward dynamics)
    return qdd
```

**Służy do:** symulacji i predykcji

---

## Czesc V: Kontakty w humanoidzie

### 5.1 Fazy kontaktu

W humanoidzie kontakty nie są "stałe":

| Faza | Opis | Przykład |
|------|------|----------|
| Lot | Brak kontaktu | Noga w powietrzu |
| Pojedyncze podparcie | Jedna noga na podłożu | Kroki |
| Podwójne podparcie | Obie nogi na podłożu | Stojanie |
| Dodatkowe podpory | Ręka na ścianie | Podpieranie się |

### 5.2 Modele kontaktu

**Twardy kontakt (constraint):**
```text
J_c × q̇ = 0           # Prędkość w kontakcie = 0
J_c × q̈ + J̇_c × q̇ = 0  # Przyspieszenie w kontakcie = 0
```

**Miękki kontakt (sprężyna-tłumik):**
```python
# Model Kelvin-Voigt
F = k × penetration + d × penetration_velocity
# k - sztywność
# d - tłumienie
```

---

## Czesc VI: Tarcie i stożek tarcia

### 6.1 Model Coulomba

```text
||f_t|| ≤ μ × f_n
```

gdzie:
- f_t — składowa tangentialna siły
- f_n — składowa normalna
- μ — współczynnik tarcia

### 6.2 Stożek tarcia

```
        f_n
         |
         |
    ----+----
   /    |    \
  /     |     \
 /      |      \
f_tmin  |    f_tmax

Tarcie kinetyczne: |f_t| = μ × f_n
Tarcie statyczne: |f_t| < μ × f_n
```

### 6.3 Aproksymacja w QP

W optymalizacji (QP) stożek tarcia aproksymuje się wielokątem (kilka półprzestrzeni):

```python
# Aproksymacja 4-ścienna
# Zamiast stożka - 4 ograniczenia liniowe
# Szybsze i stabilniejsze numerycznie
```

---

## Czesc VII: QP dla whole-body control

### 7.1 Problem optymalizacji

Kiedy masz wiele kontaktów i wiele zadań:

```text
min    ||zadania_kinematyczne||² + ||regularizacja||²
takie że
      dynamika (równania ruchu)
      ograniczenia kontaktu i tarcia
      limity τ, q̇, q̈
```

### 7.2 Przyklad QP

```python
# QP dla whole-body
H = ...  # Hessian (koszt kwadratowy)
f = ...   # Linearny czlon kosztu

A_ub = ...  # Ograniczenia nierówności
b_ub = ...

result = solve_qp(H, f, A_ub, b_ub)
```

---

## Czesc VIII: LCP/MCP i komplementarnosc

### 8.1 Problem komplementarności

Model "albo kontakt, albo brak kontaktu":

```text
0 ≤ f_n ⟂ φ(q) ≥ 0
```

gdzie φ(q) to funkcja odległości (gap function).

### 8.2 LCP (Linear Complementary Problem)

Rozwiązywanie kontaktów jako LCP:
- Kosztowne numerycznie
- W sterowaniu online często stosuje się przybliżenia

---

## Czesc IX: Praktyka inzynierska

### 9.1 Najczestsze problemy

| Problem | Skutek | Rozwiązanie |
|--------|--------|-------------|
| Błędne masy z CAD | Niestabilność | Kalibracja |
| Brak kompensacji | Dryft | Feed-forward + feedback |
| Twardy kontakt | Niemożne w realnym świecie | Model miękki + marginesy |
| Brak logowania | Niemożność debugowania | Loguj f_n, margines tarcia |

### 9.2 Co logowac

```python
# Logi diagnostyczne
log("f_n", f_n)           # Siła normalna
log("margin_friction", margin)  # Margines tarcia
log("tau_saturation", tau)     # Saturacje momentów
log("residuum", residuals)    # Residuum dynamiki
```

---

## Czesc X: Podsumowanie i checklisty

### Checklisty:

- [ ] Sprawdzenie M(q) (symetria, dodatnia określoność) na kilku pozycjach
- [ ] Testy RNEA/ABA/CRBA w porównaniu z symulatorem
- [ ] Monitorowanie: saturacje τ, naruszenia tarcia, residua dynamiki
- [ ] Walidacja kontaktu: histereza/filtrowanie, logowanie przełączeń

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Model + kompensacja | Nominalny model + feedback |
| Kontakt hybrydowy | Różne fazy = różne modele |
| Loguj residuum | Metryka zdrowia modelu |

---

## Czesc XI: Pytania do dyskusji

1. Jak odróżnisz błąd modelu dynamiki od błędu estymacji stanu (np. q̇/q̈)?
2. Kiedy model kontaktu "twardy" jest lepszy od "miękkiego" i dlaczego?
3. Jakie informacje muszą być logowane, żeby debugować poślizg i naruszenia stożka tarcia?
4. Co może pójść źle numerycznie w QP/LCP i jak to wykryjesz w runtime?

---

## Czesc XII: Zadania praktyczne

### Zadanie 1: Porównanie RNEA z symulacją

Zaimplementuj RNEA dla prostego łańcucha i porównaj z symulatorem:
- Feed-forward grawitacji
- Ocena jakości śledzenia

### Zadanie 2: QP dla rozdziału sił

Zaimplementuj QP z aproksymowanym stożkiem tarcia:
- Marginesy tarcia jako metryki
- Testy na różnych scenariuszach

### Zadanie 3: Kontakt hybrydowy

Zaimplementuj FSM kontaktu z histerezą:
- Logowanie przełączeń
- Analiza stabilności

---

## BONUS: Residuum jako metryka

Loguj residuum równania ruchu (różnica LHS - RHS):

```python
residuum = M(q) @ qdd + C(q, qd) @ qd + g(q) - tau
```

To szybciej niż "czucie", czy kontroler jedzie na fizyce czy na przypadkowych korektach!

---

*(Koniec wykladu 2)*
