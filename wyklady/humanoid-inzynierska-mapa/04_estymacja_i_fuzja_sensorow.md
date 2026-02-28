# Wykład 4: Estymacja stanu, fuzja sensorów

## Czesc I: Wstep teoretyczny — dlaczego estymacja jest potrzebna

### 1.1 Geneza — czego nie mozemy zmierzyc bezposrednio

Proszę wyobrazić sobie: robot chodzi po nierównym podłożu. Jak zmierzyć:
- Gdzie jest środek masy (CoM)?
- Czy stopa jest już na podłodze, czy jeszcze w powietrzu?
- Ile waży obiekt trzymany w ręce?

**Nie da się tego zmierzyć bezpośrednio — trzeba estymować!**

### 1.2 Co musimy estymowac

| Co estymujemy | Czujniki | Trudność |
|---------------|----------|----------|
| Pozycja i orientacja bazy | IMU + enkodery + wizja | Średnia |
| Prędkości i przyspieszenia | IMU + enkodery | Średnia |
| Biasy IMU | IMU | Trudna |
| Siły kontaktu | Czujniki + estymacja | Trudna |
| Pozycja obiektów | Kamera | Zależy |

### 1.3 Fuzja sensorow

Każdy czujnik ma wady:
- **IMU** — szybkie, ale dryfuje
- **Enkodery** — dokładne lokalnie, ale nie wiedzą o poślizgu
- **Wizja** — daje pozycję absolutną, ale wolna i zawodna

**Rozwiązanie: fuzja sensorów!**

---

## Czesc II: Modele probabilistyczne

### 2.1 Bayes i modele stanu

```text
x_{k+1} = f(x_k, u_k) + w_k    [model procesu]
z_k     = h(x_k) + v_k          [model pomiaru]
```

gdzie:
- x_k — stan (pozycja, prędkość, biasy)
- u_k — sterowania
- z_k — pomiary
- w_k, v_k — szumy

### 2.2 Hidden Markov Model

Gdy stan jest dyskretny:
- Tryb kontaktu (kontakt / brak)
- Tryb awarii

### 2.3 Factor Graph

Factor graph to "system równań" jako graf:
- Wierzchołki = zmienne (pozy w czasie, landmarki)
- Czynniki = pomiary i więzy

---

## Czesc III: Kalman Filter

### 3.1 Filtry Kalmana

KF zakłada liniowy model z szumami Gaussa:

```text
x_{k+1} = A × x_k + B × u_k + w_k
z_k     = H × x_k + v_k
```

### 3.2 Kroki KF

**Predykcja:**
```python
x_pred = A @ x + B @ u
P_pred = A @ P @ A.T + Q
```

**Aktualizacja:**
```python
S = H @ P_pred @ H.T + R
K = P_pred @ H.T @ np.linalg.inv(S)
x = x_pred + K @ (z - H @ x_pred)
P = (I - K @ H) @ P_pred
```

### 3.3 Interpretacja praktyczna

- **Q** — jak bardzo nie ufasz modelowi
- **R** — jak bardzo nie ufasz pomiarom
- **P** — niepewność stanu

---

## Czesc IV: EKF i UKF

### 4.1 EKF (Extended KF)

Linearyzuje f() i h() przez Jacobiany:

```python
# Linearyzacja
F = jacobian_f(x, u)  # ∂f/∂x
H = jacobian_h(x)      # ∂h/∂x
```

### 4.2 UKF (Unscented KF)

Używa punktów sigma zamiast Jacobianów:

```python
# Generowanie punktów sigma
sigma_points = sigma_points(x, P, kappa)

# Propagacja przez nie liniowy model
sigma_predicted = f(sigma_points)

# Obliczanie średniej i kowariancji
x_pred = weighted_mean(sigma_predicted)
P_pred = weighted_cov(sigma_predicted)
```

### 4.3 Kiedy co wybrac

| Metoda | Zalety | Wady |
|--------|--------|------|
| EKF | Prosta, szybka | Wrażliwa na linearyzację |
| UKF | Lepiej dla nieliniowości | Więcej obliczeń |
| Factor Graph | Modularna | Wolniejsza online |

---

## Czesc V: Particle Filter

### 5.1 Kiedy Gauss nie wystarcza

Gdy rozkład jest wielomodalny:
- Lokalizacja z symetriami
- Obserwacje bardzo nieliniowe

### 5.2 Struktura

```python
# Reprezentacja przez próbki (particles)
particles = []  # lista (x, wagi)

# Propagacja
for p in particles:
    p.x = f(p.x, u) + noise

# Aktualizacja wagi
for p in particles:
    p.w *= likelihood(z, p.x)

# Normalizacja
total_w = sum(p.w for p in particles)
for p in particles:
    p.w /= total_w
```

---

## Czesc VI: Synchronizacja czasu

### 6.1 Problem

Fuzja jest tak dobra, jak dobra jest oś czasu!

```
IMU: timestamp 1.000000s
Kamera: timestamp 1.003512s  # opóźnienie!

Jeśli nie uwzględnisz opóźnień → błędna fuzja!
```

### 6.2 Rozwiazanie

- Stempluj pomiary blisko sprzętu
- Buforuj stany i update "w przeszłości"
- Monitoruj statystyki opóźnień

---

## Czesc VII: Outliery i odpornosc

### 7.1 Problem

Wizja generuje outliery!

### 7.2 Rozwiazania

**Gating:**
```python
# Odrzucanie pomiarów zbyt daleko od predykcji
innovation = z - H @ x_pred
if norm(innovation) > gate_threshold:
    reject_measurement()
```

**Funkcje kosztu odporne:**
- Huber
- Cauchy

---

## Czesc VIII: Obserwowalnosc i dryf

### 8.1 Problem

Jeśli nie masz pomiarów absolutnych → pewne składowe dryfują!

**Przykład:**
```
IMU + enkodery bez wizji:
- Pozycja bazy → dryfuje w czasie
- Orientacja → dryfuje wolniej
```

### 8.2 Rozdzial

- **Szybka estymacja lokalna** (IMU/enkodery) — wysoka częstotliwość
- **Wolna korekta globalna** (wizja/mapa) — niższa częstotliwość

---

## Czesc IX: Praktyka inzynierska

### 9.1 Checklisty

- [ ] Zdefiniuj stan x: co jest estymowane
- [ ] Zdefiniuj ramki i transformacje sensorów
- [ ] Loguj innowację i jej statystyki
- [ ] Dodaj mechanizm resetu/rekonwergencji
- [ ] Mierz koszty obliczeń

### 9.2 Co logowac

```python
# Logi diagnostyczne
log("innovation", z - H @ x_pred)  # Innowacja
log("innovation_norm", norm(innovation))  # Norma innowacji
log("P_diag", np.diag(P))  # Niepewność
log("timestamp_diff", z.timestamp - current_time)  # Opóźnienie
```

---

## Czesc X: Pytania do dyskusji

1. Dlaczego timestamping i synchronizacja czasu są krytyczne dla fuzji sensorów?
2. Kiedy EKF przegrywa z factor graph?
3. Jak zaprojektujesz gating/outlier rejection?
4. Jak rozpoznasz problem obserwowalności w danych?

---

## Czesc XI: Zadania praktyczne

### Zadanie 1: EKF dla IMU

Zaimplementuj EKF dla:
- Model: pozycja + prędkość + bias żyroskopu
- Pomiary: enkodery + IMU
- Logowanie innowacji

### Zadanie 2: Factor graph

Zaimplementuj factor graph dla pose graph:
- Kilka pozy
- Odometria jako czynniki
- Funkcja kosztu odporna (Huber)

### Zadanie 3: Bufor stanów

Zaimplementuj system buforowania:
- Bufor stanów
- Update "w przeszłości" dla opóźnionych pomiarów

---

## BONUS: Innowacja jako metryka

Najszybciej wykryjesz problemy estymatora po statystykach innowacji!

Jeśli innowacja rośnie lub ma nielogiczny rozkład → problem w czasie, kalibracji lub outlierach!

---

*(Koniec wykladu 4)*
