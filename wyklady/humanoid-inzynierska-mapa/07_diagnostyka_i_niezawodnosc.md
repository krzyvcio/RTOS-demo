# Wykład 7: Diagnostyka, testy, niezawodność

## Czesc I: Wstep teoretyczny — po co nam diagnostyka

### 1.1 Geneza — degradacja zanim stanie sie awaria

Humanoid ma wiele elementów, które degradują "po cichu":
- Łożyska i przekładnie (tarcie, luzy)
- Złącza i wiązki (przerywane połączenia)
- Sensory (dryf, utrata kalibracji)
- Chłodzenie (przegrzewanie)

### 1.2 Cele diagnostyki

1. **Bezpieczeństwo** — szybko wykryć zagrożenie
2. **Dostępność** — utrzymać działanie w trybie degradacji
3. **Serwis** — dane do naprawy

---

## Czesc II: Modele przyczynowo-skutkowe

### 2.1 Fault Tree

Model top-down:
- Zdarzenie krytyczne (np. upadek)
- Przyczyny logiczne (AND/OR)
- Elementarne przyczyny

### 2.2 Graf przyczynowo-skutkowy

Węzły = symptomy i przyczyny
Krawędzie = wpływ

---

## Czesc III: Metryki niezawodnosci

### 3.1 MTBF i MTTF

**MTBF** (Mean Time Between Failures):
- Średni czas między awariami

**MTTF** (Mean Time To Failure):
- Średni czas do awarii

### 3.2 Pułapka

> Sama średnia jest mało informacyjna bez rozkładu!

---

## Czesc IV: Detekcja anomalii

### 4.1 PCA i Mahalanobis

**PCA** — redukcja wymiaru

**Odległość Mahalanobisa:**
```text
d(x) = √((x - μ)ᵀ Σ⁻¹ (x - μ))
```

### 4.2 FFT i widmo

- Wykrywanie rezonansów
- Analiza uszkodzeń mechanicznych

---

## Czesc V: Praktyka inzynierska

### 5.1 Eskalacja alertow

```
WARNING → DEGRADED → SAFE_STOP
```

### 5.2 Checklisty

- [ ] Wspólne metryki i format logów
- [ ] Loguj kontekst: stan, obciążenie, temperatura
- [ ] Health score dla podsystemów

---

## Czesc VI: Pytania do dyskusji

1. Dlaczego bez kontekstu detekcja anomalii daje fałszywe alarmy?
2. Jak zbudujesz fault tree dla "utraty stabilności"?

---

## Czesc VII: Zadania praktyczne

### Zadanie 1: Health score

Agregacja metryk do wskaźnika stanu.

### Zadanie 2: Detektor anomalii

Mahalanobis na cechach (prąd, temp, jitter).

---

## BONUS: Odporność diagnostyki

Najlepsze systemy diagnostyki są odporne na outliery i nie destabilizują systemu który obserwują!

---

*(Koniec wykladu 7)*
