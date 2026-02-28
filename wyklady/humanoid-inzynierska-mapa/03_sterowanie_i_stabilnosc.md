# Wykład 3: Sterowanie, FOC, impedancja, stabilność

## Czesc I: Wstep teoretyczny — dlaczego sterowanie jest trudne

### 1.1 Geneza — od planowania do ruchu

Masz już zaplanowaną trajektorię — piękną, gładką, optymalną. Uruchamiasz robota — i co?

Robot:
- Wibracje przy przyspieszaniu
- Traci pozycję przy obciążeniu
- Nie może utrzymać kontaktu z obiektem
- W skrajnym przypadku — niestabilność i upadek

**Dlaczego?** Bo trajektoria to "marzenie". Sterowanie to "rzeczywistość".

### 1.2 Dlaczego to jest trudne

Profesor zawsze powtarza: **"Sterowanie to sztuka kompromisu między szybkością a stabilnością."**

Problem: każda warstwa widzi "inny obiekt":
- Pętla prądu widzi: silnik + falownik
- Pętla prędkości widzi: silnik + napęd + obciążenie
- Pętla pozycji widzi: cały łańcuch kinematyczny + dynamika

**Jeśli niższa warstwa jest słabsza — wyższa dostaje "gluta"!**

---

## Czesc II: Architektura warstw sterowania

### 2.1 Typowa struktura

```
+--------------------------------------------------+
|  Warstwa planowania (Hz)                          |
|  - Trajektorie, cele, strategia                  |
+--------------------------------------------------+
            ↓ (co 10-100 Hz)
+--------------------------------------------------+
|  Warstwa zadaniowa (dziesiątki Hz)              |
|  - MPC, WBC, śledzenie CoM, postawa            |
+--------------------------------------------------+
            ↓ (setki Hz)
+--------------------------------------------------+
|  Warstwa przegubowa (setki Hz - kHz)           |
|  - Regulatory prędkości/pozycji                 |
+--------------------------------------------------+
            ↓ (kHz)
+--------------------------------------------------+
|  Warstwa napędowa (kHz)                         |
|  - FOC, PWM, sterowanie prądem                  |
+--------------------------------------------------+
```

### 2.2 Zasada kaskady

> Pętla wewnętrzna musi być istotnie szybsza i stabilna, inaczej wyższe warstwy sterują "glutem".

---

## Czesc III: Modele: ciagłe vs dyskretne

### 3.1 Model ciągły

```text
ẋ = A x + B u
y  = C x + D u
```

### 3.2 Model dyskretny

```text
x_{k+1} = A_d x_k + B_d u_k
y_k     = C x_k + D u_k
```

### 3.3 Co psuje zgodność z teorią

| Problem | Wpływ |
|---------|-------|
| Opóźnienie obliczeń i komunikacji | Zawsze jest |
| Kwantyzacja i saturacje | PWM, prądy, momenty |
| Aliasing i filtracja | Niewłaściwe filtry |

---

## Czesc IV: Regulatory klasyczne: PID

### 4.1 Struktura PID

```python
# Ciągły PID
u(t) = Kp × e(t) + Ki × ∫e(t)dt + Kd × de(t)/dt

# Dyskretny (euler forward)
u_k = u_{k-1} + Kp × (e_k - e_{k-1}) + Ki × e_k + Kd × (e_k - 2e_{k-1} + e_{k-2})
```

### 4.2 Pułapki implementacyjne

| Problem | Rozwiązanie |
|---------|-------------|
| Anti-windup | Back-calculation, clamping |
| Filtracja D | Filtracja szumu (T_F) |
| Kompensacja grawitacji | Feed-forward |

### 4.3 Przykład: PID z anti-windup

```python
def pid_with_antiwindup(e, Kp, Ki, Kd, u_min, u_max):
    global integral, prev_e
    
    # Część proporcjonalna
    p = Kp * e
    
    # Część całkująca z anti-windup
    integral += Ki * e
    integral = clamp(integral, u_min, u_max)
    
    # Część różniczkowa
    d = Kd * (e - prev_e)
    
    # Wyjście
    u = p + integral + d
    
    # Clamping + back-calculation
    if u > u_max:
        integral -= (u - u_max)  # Anti-windup
        u = u_max
    elif u < u_min:
        integral -= (u - u_min)
        u = u_min
    
    prev_e = e
    return u
```

---

## Czesc V: LQR/LQG i MPC

### 5.1 LQR (Linear Quadratic Regulator)

```text
min ∫ (x^T Q x + u^T R u) dt
```

- Dobiera sprzężenie u = -Kx minimalizujące koszt
- Daje kompromis: energia sterowania vs błąd

### 5.2 LQG (Linear Quadratic Gaussian)

LQR + Kalman Filter (gdy nie mierzysz całego stanu)

### 5.3 MPC (Model Predictive Control)

```python
# MPC - rozwiązuje problem optymalizacji na horyzoncie
def mpc(x_current, horizon=20):
    for k in range(horizon):
        # Predykcja
        x_pred = model.predict(x_current, u_k)
        
        # Koszt
        cost += cost_function(x_pred, u_k)
        
        # Ograniczenia
        if violates_constraints(x_pred, u_k):
            cost += penalty
    
    # Rozwiąż QP
    u_optimal = solve(cost, constraints)
    return u_optimal[0]
```

---

## Czesc VI: Impedancja i admisja

### 6.1 Sterowanie impedancyjne

```python
# Impedancja: F = M × (x_d - x) + B × (v_d - v) + K × (x_d - x)

# Relacja siła <-> odchyłka
# Jak sprężyna-tłumik w przestrzeni zadania

def impedance_control(x_meas, x_des, v_meas, v_des, M, B, K):
    x_err = x_des - x_meas
    v_err = v_des - v_meas
    
    F = M * x_err + B * v_err + K * x_err
    return F
```

### 6.2 Sterowanie admisyjne

Przekształca mierzoną siłę w ruch:

```python
# Admisyjność: odwrotność impedancji
x = F / M + ...
```

---

## Czesc VII: FOC (Field-Oriented Control)

### 7.1 Idea

FOC sprowadza sterowanie silnikiem do regulacji prądów w osiach d/q:

```
Clarke:  (Ia, Ib, Ic) → (Iα, Iβ)
Park:    (Iα, Iβ) → (Id, Iq)  [w ramce wirującej]
```

### 7.2 Struktura FOC

```
         +---------+    +---------+    +---------+
I_abc -->|  Clarke |--> |  Park   |--> | PI (Id) |--> SVPWM --> Silnik
         |         |    |         |    +---------+
         |         |    |         |    +---------+
         |         |    |         |--> | PI (Iq) |
         +---------+    +---------+    +---------+
              ↑              ↑
         (theta)         (theta)
```

### 7.3 Zasada pasm

> Pętla prądu ma być istotnie szybsza niż mechanika i pętle wyższego poziomu.

---

## Czesc VIII: Filtry i ich wplyw

### 8.1 Typy filtrów

| Filtr | Zastosowanie |
|-------|--------------|
| Notch | Tłumienie rezonansów |
| Low-pass | Ograniczenie szumu |
| Butterworth | Ogólna filtracja |

### 8.2 Kiedy filtr jest zly

Filtr wprowadza opóźnienie fazowe!

```python
# Przykład: filtr obniża fazę
# Faza = -ω × τ (gdzie τ = opóźnienie grupy)

# Przy krytycznej częstotliwości (ω_c):
# Margines fazy = φ_m = 180° - ω_c × τ

# Zbyt dużo filtracji → niestabilność!
```

---

## Czesc IX: Stabilnosc — jak diagnozowac

### 9.1 Narzędzia klasyczne

| Narzędzie | Do czego |
|-----------|----------|
| Nyquist | Ocena stabilności w pętli |
| Routh-Hurwitz | Stabilność wielomianu |
| Bode | Pasmo, margines fazy/wzmocnienia |

### 9.2 Praktyczne strojenie

> **Najpierw budujesz zapas stabilności (marginesy), dopiero potem gonisz wydajność.**

---

## Czesc X: Praktyka inzynierska

### 10.1 Checklisty

- [ ] Strojenie kaskadowe: najpierw prąd, potem prędkość, potem pozycja
- [ ] Anti-windup w każdym PI/PID z saturacją
- [ ] Ograniczenie jerk i filtracja setpointu
- [ ] Logowanie: saturacje, rezonanse (FFT), statystyki czasu

### 10.2 Co logowac

```python
# Logi diagnostyczne
log("u", u)                    # Sterowanie
log("e", error)               # Błąd
log("sat", saturation_flags)   # Saturacje
log("loop_time_us", dt)       # Czas pętli
```

---

## Czesc XI: Pytania do dyskusji

1. Dlaczego strojenie "od góry" (najpierw pozycja) często kończy się niestabilnością?
2. Jak zmierzysz wpływ opóźnienia i jitteru na margines fazy?
3. Kiedy notch jest dobrym pomysłem, a kiedy lepiej obniżyć pasmo regulatora?
4. Jakie są symptomy braku anti-windup w danych logów?

---

## Czesc XII: Zadania praktyczne

### Zadanie 1: Dyskretny PID z anti-windup

Zaimplementuj dyskretny PID z anti-windup:
- Testy saturacji
- Testy odpowiedzi skokowej

### Zadanie 2: Filtr notch

Zaprojektuj filtr notch na podstawie FFT:
- Walidacja przed/po
- Sprawdzenie stabilności

### Zadanie 3: Mini-MPC

Zaimplementuj mini-MPC dla jednego przegubu:
- Ograniczenia momentu/prędkości
- Limit czasu obliczeń

---

## BONUS: Filtr bez walidacji

Najczęstszy błąd projektowy: **"dodaj filtr" bez policzenia wpływu na fazę.**

W praktyce filtr to koszt fazy — zawsze waliduj wpływ na stabilność, nie tylko na wygląd sygnału!

---

*(Koniec wykladu 3)*
