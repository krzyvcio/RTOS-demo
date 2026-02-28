# Wykład 6: Sieci czasu rzeczywistego, synchronizacja

## Czesc I: Wstep teoretyczny — dlaczego czas jest kluczowy

### 1.1 Geneza — ukryta zmienna stanu

W humanoidzie "czas" jest wszędzie:
- Pętla prądu: co 50 μs
- Pętla sterowania: co 1 ms
- Estymacja: co 5 ms
- Planowanie: co 50 ms

**Co się dzieje gdy opóźnienia rosną?**
- Sterowanie widzi "przeszłość"
- Estymacja rozjeżdża się z rzeczywistością
- Kontakt — stopa "już" na podłodze, ale system nie wie

### 1.2 Skutki jitteru

> **Jitter degraduje margines fazy → niestabilność!**

---

## Czesc II: Podstawowe pojecia

### 2.1 WCET i WCRT

**WCET** (Worst-Case Execution Time):
- Maksymalny czas wykonania zadania

**WCRT** (Worst-Case Response Time):
- Czas od "gotowości" do "zakończenia"
- Uwzględnia oczekiwanie na CPU

### 2.2 Jitter

Dwa rodzaje:
- Jitter próbkowania (czas między iteracjami)
- Jitter opóźnienia komunikacji

---

## Czesc III: Harmonogramowanie

### 3.1 Rate Monotonic (RM)

- Priorytety stałe
- Krótszy okres = wyższy priorytet

### 3.2 Deadline Monotonic (DM)

- Priorytety stałe
- Krótszy deadline = wyższy priorytet

### 3.3 EDF (Earliest Deadline First)

- Priorytety dynamiczne
- Najbliższy deadline wygrywa

---

## Czesc IV: Synchronizacja czasu

### 4.1 Problem zegarów

Każdy węzeł ma własny zegar:
```text
t_local = a × t_true + b
```
- a ≈ 1 (dryft)
- b (offset)

### 4.2 Synchronizacja

- Offset korygujesz często
- Dryft śledzisz wolniej

---

## Czesc V: Grafy w systemie czasu rzeczywistego

### 5.1 Graf sieci

- Węzły = sensory, kontrolery, napędy
- Krawędzie = połączenia komunikacyjne

### 5.2 Graf przeplywu danych

- Węzły = zadania
- Krawędzie = dane

### 5.3 Graf zależnosci czasowych

"Ograniczenia typu: A musi skończyć X ms przed B"

---

## Czesc VI: Praktyka inzynierska

### 6.1 Co mierzyc

- Opóźnienie end-to-end
- Jitter (histogram)
- WCRT

### 6.2 Checklisty

- [ ] Każda pętla ma zdefiniowany okres i deadline
- [ ] Logowanie jest asynchroniczne
- [ ] Testy obciążeniowe są częścią walidacji

---

## Czesc VII: Pytania do dyskusji

1. Jak zmierzysz opóźnienie end-to-end?
2. Kiedy EDF ma sens, a kiedy RM wygrywa?
3. Jak jitter wpływa na stabilność?

---

## Czesc VIII: Zadania praktyczne

### Zadanie 1: Monitor jitteru

Zaimplementuj monitor jitteru w runtime z alarmami.

### Zadanie 2: Symulator harmonogramu

Symulator RM/EDF dla zestawu zadań.

---

## BONUS: WCRT

Najczęściej "tajnym" wąskim gardłem jest kolejka lub lock w miejscu, którego nikt nie uważa za krytyczne!

Mierz WCRT i histogramy, nie tylko średnie czasy!

---

*(Koniec wykladu 6)*
