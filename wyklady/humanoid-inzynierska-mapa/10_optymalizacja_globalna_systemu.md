# Wykład 10: Optymalizacja globalna systemu

## Czesc I: Wstep teoretyczny — kompromisy w kazdym miejscu

### 1.1 Geneza — kazda decyzja jest kompromisem

W humanoidzie każda trudna decyzja to kompromis:
- Stabilność vs szybkość
- Energia vs dynamika
- Dokładność vs bezpieczeństwo kontaktu
- Komfort (jerk) vs czas wykonania

### 1.2 Optymalizacja jako jezyk

Optymalizacja pozwala te kompromisy opisać jawnie:
- Funkcja kosztu = "co jest ważne"
- Ograniczenia = "czego nie wolno"

---

## Czesc II: Klasy problemow

### 2.1 LP (Linear Programming)

```text
min cᵀx  takie że Ax ≤ b
```

### 2.2 QP (Quadratic Programming)

```text
min ½xᵀHx + cᵀx  takie że Ax ≤ b
```

To koń roboczy humanoidów!

### 2.3 NLP

Gdy nieliniowe ograniczenia/koszty

---

## Czesc III: Wypuklosc

### 3.1 Dlaczego wypuklosc jest wazna

- Optimum globalne
- Stabilne solwery
- Przewidywalne zachowanie

### 3.2 Praktyczna zasada

Modeluj tak, by najwięcej było wypukłe (QP)!

---

## Czesc IV: Metody rozwiazywania

### 4.1 Gradient descent

Prosty, ale wolny i wrażliwy na skalowanie.

### 4.2 Newton

Szybki lokalnie, wymaga Hessianu.

### 4.3 Quasi-Newton (BFGS/L-BFGS)

Kompromis: dobra szybkość bez pełnego Hessianu.

---

## Czesc V: Skalowanie i stabilnosc numeryczna

### 5.1 Typowe problemy

- Różne jednostki (metry vs radiany)
- Źle dobrane wagi
- Słabe uwarunkowanie macierzy
- Sprzeczne ograniczenia

### 5.2 Techniki

- Normalizacja zmiennych
- Regularizacja (εI)
- Slack variables

---

## Czesc VI: Online vs offline

### 6.1 Offline

- Dokładniejsze modele
- Większe NLP/MILP
- Strojenie, generowanie trajektorii

### 6.2 Online

- Krótkie horyzonty
- QP, MPC
- Determinizm runtime

---

## Czesc VII: Praktyka inzynierska

### 7.1 Funkcja kosztu

Typowa struktura:
- Błąd celu (task)
- Regularizacja postawy
- Gładkość (jerk)
- Kary za sterowanie
- Marginesy bezpieczeństwa

### 7.2 Checklisty

- [ ] Każda zmienna ma sensowną skalę
- [ ] Solver ma limit czasu i fallback
- [ ] Loguj: status, naruszenia, kondycję

---

## Czesc VIII: Pytania do dyskusji

1. Dlaczego skalowanie zmiennych jest krytyczne?
2. Kiedy QP wystarczy, a kiedy NLP?

---

## BONUS: Bezpieczeństwo w koszcie

Najlepsza funkcja kosztu koduje priorytety bezpieczeństwa ZANIM zacznie "upiększać" śledzenie!

---

*(Koniec wykladu 10)*
