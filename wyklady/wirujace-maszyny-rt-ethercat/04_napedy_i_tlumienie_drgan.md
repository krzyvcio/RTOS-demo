# Wykład 4: Napędy, stabilizacja prędkości i tłumienie drgań

## Czesc I: Wstep teoretyczny — dlaczego drgania sa tak wazne

### 1.1 Geneza problemu

Proszę wyobrazić sobie wirówkę laboratoryjną pracującą przy 15 000 RPM. Nagle, w jednym konkretnym zakresie prędkości (powiedzmy 8000-10000 RPM), pojawiają się intensywne drgania. Poza tym zakresem — wszystko jest w porządku.

**Co się dzieje?** To jest klasyczny rezonans mechaniczny.

**Dlaczego to jest niebezpieczne?**
- Drgania = siły dynamiczne
- Siły = naprężenia w materiałach
- Naprężenia = zmęczenie materiału
- Zmęczenie = pęknięcie przy dużej prędkości

Przy 15 000 RPM i średnicy wirnika 10 cm:
- Przyspieszenie odśrodkowe: a = ω²r = (1570)² × 0.05 ≈ 123 000 m/s² ≈ 12 000 g!
- Nawet 1% niewyważenia generuje siłę 1230 N

**To jest siła, która niszczy łożyska!**

### 1.2 Skąd biorą się drgania

Drgania w maszynach wirujących mają kilka źródeł:

| Źródło | Opis | Przykład |
|--------|------|----------|
| **Niewyważenie** | Środek masy niezgodny z osią obrotu | Brud na wirniku |
| **Rezonans mechaniczny** | Częstotliwość własna konstrukcji | Łożyska, mocowania |
| **Pobudzenie od napędu** | Sterowanie pobudza rezonans | Zbyt "agresywny" PID |
| **Zakłócenia zewnętrzne** | Wibracje od maszyn nearby | Instalacja |

### 1.3 Dlaczego sterowanie może pobudzać rezonans

Profesor zawsze powtarza: **"Często to nie mechanika jest zła, tylko sterowanie jest zbyt agresywne."**

Wyjaśnienie:

```
Regulator PID
     |
     v
Daje moment T do silnika
     |
     v
Silnik przyspiesza/zwalnia
     |
     v
Wirnik na łożyskach
     |
     v
Jeśli częstotliwość zmian momentu ≈ częstotliwość rezonansowa
     |
     v
ROSNĄ AMPLITUDA DRGAŃ!!!
```

---

## Czesc II: Model fizyczny

### 2.1 Model podstawowy — równanie ruchu

```
J × dω/dt = T_motor - T_load - T_losses - T_damping
```

Wyjaśnienie:
- **J** — moment bezwładności (fizyka!)
- **ω** — prędkość kątowa
- **T_motor** — moment od silnika (sterujemy tym!)
- **T_load** — moment obciążenia
- **T_losses** — straty (tarcie)
- **T_damping** — tłumienie

### 2.2 Model drgań — masa-tłumik-sprężyna

```
M × ẍ + D × ẋ + K × x = F(t)

M — masa
D — współczynnik tłumienia  
K — sztywność
x — przemieszczenie
F(t) — siła wymuszająca (od napędu!)
```

**To mówi nam:**
- Częstotliwości własne: ω₀ = √(K/M)
- Tłumienie D decyduje o "ostrości" piku
- Siła F(t) może pochodzić od sterowania!

### 2.3 Wniosek praktyczny

> Jeśli regulator ma pasmo (szybkość reakcji) w okolicy częstotliwości własnej — bardzo łatwo o wzbudzenie rezonansu.

---

## Czesc III: Kaskada pętli

### 3.1 Dlaczego kaskada

Pojedyncza pętla (np. sterowanie prędkością) nie wystarcza, bo:
- Nie ma wystarczającego pasma
- Nie chroni przed przeciążeniem prądowym
- Nie kontroluje momentu

**Rozwiązanie:** Kaskada pętli

```
Setpoint (prędkość)
     |
     v
[ Pętla prędkości ] → Setpoint (moment)
     |                        |
     v                        v
[ Pętla prądu ]        → Sterowanie PWM
```

### 3.2 Typowy schemat

```
+------------------------+
|  Pętla prądu (FOC)    |  ← W napędzie, kHz
|  (q, d current)       |
+------------------------+
           |
           v
+------------------------+
|  Pętla prędkości      |  ← W napędzie lub masterze, kHz
|  (PI controller)       |
+------------------------+
           |
           v
+------------------------+
|  Profil setpointu     |  ← W masterze, Hz
|  (rampy, jerk)        |
+------------------------+
```

### 3.3 Zasada kaskady

> Pętla wewnętrzna musi być istotnie szybsza i stabilna, inaczej wyższe warstwy sterują "glutem".

---

## Czesc IV: Setpoint shaping — rampy i jerk

### 4.1 Problem: agresywne zmiany

Wiele problemów drgań bierze się z agresywnych zmian zadania:

```
ZŁE:           DOBRE:
               
omega           omega
  |              |
  |________      |      _______
  |        \     |     /       \
  |         \    |    /         \____
  |          \   |   /
  |           \__|__/
  |                
  +--------------> czas
     
  Skok         Łagodne przejście
  → Drgania    → Brak drgań
```

### 4.2 Rampy — ograniczenie przyspieszenia

```c
// Generator rampy prędkości
float velocity_ramp(float omega_target, float dt) {
    static float omega_current = 0;
    
    float delta = omega_target - omega_current;
    float max_delta = max_acceleration * dt;
    
    if (delta > max_delta) {
        omega_current += max_delta;
    } else if (delta < -max_delta) {
        omega_current -= max_delta;
    } else {
        omega_current = omega_target;
    }
    
    return omega_current;
}
```

### 4.3 Jerk — ograniczenie przyspieszenia

```c
// Generator z ograniczeniem jerk
float jerk_limited_ramp(float omega_target, float dt) {
    static float omega_current = 0;
    static float accel_current = 0;
    
    float delta = omega_target - omega_current;
    float max_delta = max_acceleration * dt;
    
    // Ograniczenie przyspieszenia
    if (delta > max_delta) {
        accel_current = max_acceleration;
    } else if (delta < -max_delta) {
        accel_current = -max_acceleration;
    } else {
        accel_current = delta / dt;
    }
    
    // Ograniczenie jerk (zmiany przyspieszenia)
    static float prev_accel = 0;
    float jerk = accel_current - prev_accel;
    if (jerk > max_jerk * dt) jerk = max_jerk * dt;
    if (jerk < -max_jerk * dt) jerk = -max_jerk * dt;
    prev_accel = accel_current;
    
    omega_current += accel_current * dt;
    
    return omega_current;
}
```

### 4.4 Dlaczego to działa

> To często daje większy efekt niż dokładanie złożonych filtrów. Tanie i skuteczne!

---

## Czesc V: FFT — jak robic to praktycznie

### 5.1 Sygnały do analizy

| Sygnał | Co pokazuje |
|--------|-------------|
| Błąd prędkości | Czy regulator pobudza rezonans |
| Moment/prąd sterujący | Siła wymuszająca od napędu |
| Wibracje (akcelerometr) | Odpowiedź mechaniki |
| Prądy (opcjonalnie) | Harmoniczne od PWM |

### 5.2 Wskazówki praktyczne

1. **Analizuj na tych samych odcinkach czasowych**
2. **W porównywalnych warunkach** (ten sam tryb, prędkość, obciążenie)
3. **Loguj kontekst**: temperatura, tryb, obciążenie

```python
# Przykład: zbieranie danych do FFT
import numpy as np

def collect_for_fft(signal, window_ms=1000, overlap=0.5):
    """
    signal: tablica próbek z timestampami
    window_ms: długość okna w ms
    overlap: procent nakładania
    """
    window_samples = int(window_ms * sample_rate / 1000)
    step = int(window_samples * (1 - overlap))
    
    windows = []
    for i in range(0, len(signal) - window_samples, step):
        windows.append(signal[i:i+window_samples])
    
    return windows
```

### 5.3 Interpretacja wyników

| Obserwacja | Wniosek |
|------------|---------|
| Pik w błędzie i w sterowaniu | Regulator pobudza rezonans |
| Pik tylko w wibracjach | Mechanika/łożyskowanie lub pobudzenie zewnętrzne |
| Piki rosną z prędkością | Niewyważenie lub element zależny od prędkości |
| Stały pik niezależny od prędkości | Problem konstrukcyjny |

---

## Czesc VI: Notch filter — kiedy pomaga, kiedy szkodzi

### 6.1 Co to jest notch

Notch (filtr grzebieniowy) to filtr, który wycina wąskie pasmo częstotliwości:

```
Amplituda
    ^
    |
    |      ________
    |     /        \
    |    /          \__
    |   /
    |  /
    +------------------> częstotliwość
         ^
         |
    Częstotliwość
    rezonansowa
```

### 6.2 Kiedy notch POMAGA

- Zidentyfikowany, stabilny pik rezonansowy
- Pik jest uciążliwy ale mechanika OK
- Notch wycina dokładnie ten pik

### 6.3 Kiedy notch SZKOŹY

- **NIE jest "za darmo"** — zmienia fazę i opóźnienie
- **Źle dobrany** — pogarsza margines stabilności
- **Pik niestabilny** — przesuwa się z warunkami

### 6.4 Procedura bezpieczna

```
KROK 1: Zidentyfikuj dominujący pik (FFT)

KROK 2: Sprawdź czy pik jest stabilny
   - Czy jest w tym samym miejscu przy różnych prędkościach?
   - Czy jest stabilny w czasie?

KROK 3: Dodaj pojedynczy notch
   - Częstotliwość = częstotliwość piku
   - Szerokość = minimalna (tylko tyle, ile potrzeba)

KROK 4: Zweryfikuj
   - Poprawa widma?
   - Brak pogorszenia stabilności?
   - Brak nowych oscylacji?

KROK 5: Dopiero wtedy rozważ kolejne filtry
```

---

## Czesc VII: Co najczesciej psuje "dobry regulator"

### 7.1 Typowe błędy

| Błąd | Skutek | Rozwiązanie |
|-------|--------|-------------|
| Uśrednianie prędkości | Opóźnienie w pętli pomiarowej | Mniej uśredniania lub predykcja |
| Saturacja bez anti-windup | Integrator "puchnie", potem "wybucha" | Dodaj anti-windup |
| Zbyt duże pasmo | Pobudzenie konstrukcji | Ogranicz pasmo regulatora |
| Brak ograniczeń jerk | Skoki pobudzają mody | Dodaj limiter jerk |
| "Leczenie mechaniki softwarem" | Software nie naprawi luzów | Napraw mechanikę! |

### 7.2 Przykład: anti-windup

```c
// PID z anti-windup (back-calculation)
float pid_compute(float error, float dt) {
    // Część proporcjonalna
    float p = kp * error;
    
    // Część całkująca
    integral += ki * error * dt;
    
    // Wyjście przed saturacją
    float u = p + integral + kd * (error - prev_error) / dt;
    
    // Anti-windup: back-calculation
    if (u > u_max) {
        integral -= (u - u_max) * kc;  // kc = 1/ki
        u = u_max;
    } else if (u < u_min) {
        integral -= (u - u_min) * kc;
        u = u_min;
    }
    
    prev_error = error;
    return u;
}
```

---

## Czesc VIII: Podsumowanie i checklisty

### Checklisty:

- [ ] Pętla prądu jest szybsza niż pętla prędkości (kaskada ma sens)
- [ ] Setpoint ma rampy i ograniczenie jerk (szczególnie w przejściach)
- [ ] Widma (FFT) są częścią diagnostyki
- [ ] Notch jest dobierany z pomiarów i walidowany
- [ ] Logujesz kontekst (tryb, obciążenie, temperaturę)

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Najpierw kaskada | Uporządkuj pętle |
| Potem rampy/jerk | Ogranicz pobudzenie |
| Potem FFT | Zdiagnozuj problem |
| Dopiero notch | Ostatnia linia obrony |

---

## Czesc IX: Pytania do dyskusji

1. Jakie ryzyko niesie dołożenie notcha "na oko"?
2. W jakim miejscu łańcucha najczęściej powstaje pobudzenie rezonansu?
3. Kiedy ograniczenie jerk daje większy efekt niż filtracja?
4. Jak zaprojektujesz procedurę testową przejścia przez zakresy rezonansów?

---

## Czesc X: Zadania praktyczne

### Zadanie 1: Sygnały do FFT

Wybierz 3 sygnały do FFT i uzasadnij, co z nich wywnioskujesz.

### Zadanie 2: Procedura notch

Opisz procedurę: wykrycie piku → weryfikacja → notch → walidacja.

### Zadanie 3: Rampy i jerk

Zdefiniuj gdzie implementujesz rampy/jerk (drive vs master) i dlaczego.

---

## BONUS: Najlepszy filtr

Najlepszy "filtr" to często mechanika: jeśli widzisz narastające piki w stałym paśmie, równolegle do pracy software zaplanuj inspekcję mocowań, niewyważenia i łożysk.

Software może maskować problem, ale go nie naprawi!

---

*(Koniec wykladu 4)*
