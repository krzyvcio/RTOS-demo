# Wykład 5: Planowanie ruchu, trajektorie

## Czesc I: Wstep teoretyczny — czym jest planowanie ruchu

### 1.1 Geneza — od celu do ruchu

Masz zadanie: "podnieś kubek ze stołu". Co musisz zrobić?

1. **Gdzie jest kubek?** (percepcja)
2. **Jak do niego podejść?** (planowanie)
3. **Jaką trasą?** (trajektoria)
4. **Co jeśli przeszkoda?** (unikanie kolizji)

To jest planowanie ruchu!

### 1.2 Pipeline planowania

```
[Cel] → [Strategia] → [Plan geometryczny] → [Trajektoria] → [Wykonanie]
```

Każdy etap może zawieść — system musi być odporny.

### 1.3 Specyfika humanoida

W humanoidzie dodatkowo:
- Planowanie kontaktów (kiedy i gdzie stawiać stopę)
- Stabilność (nie wywrócić się)
- Bezpieczeństwo (praca obok człowieka)

---

## Czesc II: Grafy w planowaniu

### 2.1 Graf stanów

Stan może być:
- Konfiguracja przegubów q
- Pozycja bazy + konfiguracja
- Abstrakcyjny stan chodu

### 2.2 Graf C-space

Przestrzeń wszystkich możliwych konfiguracji:
- Wymiar = liczba przegubów
- Przeszkody = obszary niedozwolone

### 2.3 RRT i RRT*

**RRT (Rapidly-exploring Random Tree):**
- Losowe próbkowanie
- Szybko znajduje ścieżkę
- Nie gwarantuje optimum

**RRT*:**
- Z czasem poprawia rozwiązanie
- Asymptotycznie optymalne

---

## Czesc III: Planowanie na grafach

### 3.1 A* i Dijkstra

**Dijkstra:** Najkrótsza ścieżka bez heurystyki

**A*:** Dijkstra + heurystyka (odległość do celu)

### 3.2 Kiedy uzywac

| Metoda | Zastosowanie |
|--------|-------------|
| A* | Siatki, map occupancy |
| RRT | Wysokie wymiary, brak siatki |
| PRM | Środowisko stałe, wiele zapytań |

---

## Czesc IV: Trajektorie

### 4.1 Gładkość

Trajektoria musi być:
- Ciągła w pozycji (C0)
- Ciągła w prędkości (C1)
- Ciągła w przyspieszeniu (C2)

### 4.2 Typy splajnów

| Typ | Stopień | Ciągłość |
|-----|---------|-----------|
| Linear | 1 | C0 |
| Cubic | 3 | C2 |
| Quintic | 5 | C4 |

### 4.3 Jerk

> Jerk (pochodna przyspieszenia) jest krytyczny!

Duży jerk = uderzenia w przekładnie = rezonanse!

---

## Czesc V: Optymalizacja trajektorii

### 5.1 CHOMP

Gradientowo minimalizuje:
- Koszt gładkości
- Koszt kolizji

### 5.2 TrajOpt

Formułowany jako SQP (Sequential Quadratic Programming):
- Integruje ograniczenia
- Popularny w praktyce

---

## Czesc VI: Ograniczenia w planowaniu

### 6.1 Typowe ograniczenia

- Limity przegubów
- Limity prędkości/przyspieszenia
- Ograniczenia kontaktu
- Kolizje

### 6.2 QP w planowaniu

```python
# QP: min ||Ax - b||² takie że Cx ≤ d
H = A.T @ A
f = -A.T @ b

result = solve_qp(H, f, C, d)
```

---

## Czesc VII: Planowanie krokow

### 7.1 Warstwy chodu

1. **Footstep planning** — gdzie postawić stopę
2. **CoM planning** — jak przesuwać masę
3. **Faza** — kiedy kontakt, kiedy lot

### 7.2 W praktyce

- Globalny plan bywa prosty (kolejne kroki)
- Lokalny kontroler (WBC/MPC) robi fizykę na krótkim horyzoncie

---

## Czesc VIII: Praktyka inzynierska

### 8.1 Checklisty

- [ ] Metryki jakości: margines kolizji, margines tarcia, max jerk
- [ ] Testy regresji na scenariuszach
- [ ] Monitorowanie czasu planowania

### 8.2 Degradacja

Gdy planowanie nie zdąży → tryb bezpieczny!

---

## Czesc IX: Pytania do dyskusji

1. Kiedy plan geometryczny jest bezużyteczny bez dynamiki?
2. Jakie są zalety i wady RRT/RRT* vs PRM?
3. Dlaczego jerk jest "ukrytym" ograniczeniem stabilności?
4. Jak budujesz funkcję kosztu?

---

## Czesc X: Zadania praktyczne

### Zadanie 1: RRT

Zaimplementuj RRT dla prostego układu 2D.

### Zadanie 2: Splajny

Zaimplementuj generator trajektorii na splajnach quintic z ograniczeniem jerk.

### Zadanie 3: TrajOpt

Zaimplementuj TrajOpt z ograniczeniami i kolizjami.

---

## BONUS: Hybryda planowania

W praktycznych systemach wygrywa hybryda: plan globalny + lokalny kontroler reaktywny!

Projektuj od początku punkty replanningu!

---

*(Koniec wykladu 5)*
