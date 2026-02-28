# Wykład 5: Diagnostyka i condition monitoring (praktycznie)

## Czesc I: Wstep teoretyczny — po co nam diagnostyka

### 1.1 Geneza — dlaczego "naprawa po awarii" to za mało

Proszę wyobrazić sobie sytuację: wirówka laboratoryjna pracuje w fabryce przez 3 lata. Nagle, bez ostrzenia, ulega awarii — pęka łożysko, wirnik uderza o obudowę.

**Koszt:**
- Nowe łożyska + wirnik: 50 000 PLN
- Przestój produkcji: 100 000 PLN
- Naprawa: 20 000 PLN
- **RAZEM: 170 000 PLN**

**A gdybyśmy wiedzieli 2 tygodnie wcześniej?**

Gdybyśmy widzieli, że:
- Temperatura łożysk rośnie o 0.5°C tydzień
- Wibracje w paśmie 2 kHz rosną o 20%
- Prąd napędu ma "dziwne" harmoniczne

→ Mogliśmy zaplanować serwis, zamówić części, zrobić naprawę w nocy.

**To jest cel diagnostyki: wykryć degradację ZANIM stanie się awarią.**

### 1.2 Diagnostyka vs monitoring

| Pojęcie | Definicja | Cel |
|---------|-----------|-----|
| **Monitoring** | Ciągłe zbieranie danych | Widzieć co się dzieje |
| **Diagnostyka** | Analiza danych, wykrywanie problemów | Zrozumieć dlaczego |
| **Prognostics** | Przewidywanie awarii | Wiedzieć kiedy |

### 1.3 Dlaczego kontekst jest kluczowy

Profesor zawsze powtarza: **"Ten sam sygnał może oznaczać co innego w różnych warunkach."**

Przykład:

```
Prąd = 10A
   |
   +-- W trybie ROZRUCH: NORMALNE (rozpędzanie masy)
   +-- W trybie PRACA USTALONA: DEGRADACJA (zwiększone tarcie)
   +-- W trybie ZATRZYMANIE: NORMALNE (hamowanie)
```

**Bez kontekstu — bez diagnostyki!**

---

## Czesc II: Co monitorujemy w maszynach wirujących

### 2.1 Kategorie metryk

| Kategoria | Co mierzymy | Po co |
|-----------|-------------|-------|
| **Termika** | Temperatura łożysk, napędu, elektroniki | Przegrzanie, degradacja |
| **Mechanika** | Wibracje (RMS, FFT), luzy, niewyważenie | Rezonanse, zużycie |
| **Napęd** | Prąd, moment, napięcie | Obciążenie, degradacja |
| **Czas rzeczywisty** | Jitter, missed deadlines | Stabilność sterowania |
| **Komunikacja** | Dropouty, błędy | Jakość połączenia |

### 2.2 Typowe progi (orientacyjne)

| Metryka | Normalny | Ostrzeżenie | Alarm | Stop |
|---------|----------|-------------|-------|------|
| Temp. łożyska | < 60°C | 60-70°C | 70-80°C | > 80°C |
| Wibracje RMS | < 2 mm/s | 2-4 mm/s | 4-8 mm/s | > 8 mm/s |
| Temp. napędu | < 50°C | 50-65°C | 65-80°C | > 80°C |
| Jitter p99 | < 50 μs | 50-100 μs | 100-200 μs | > 200 μs |

---

## Czesc III: Online vs offline

### 3.1 Diagnostyka online

**Charakterystyka:**
- Szybka i konserwatywna
- Wspiera bezpieczeństwo i utrzymanie pracy
- **Nie może destabilizować RT** (musi być lekka obliczeniowo)

```c
// Przykład: prosta diagnostyka online
void online_diagnostics() {
    // Tylko proste porównania - lekkie!
    
    // Sprawdź temperaturę
    if (temperature > TEMP_WARNING) {
        set_warning(FLAG_TEMP_HIGH);
    }
    
    // Sprawdź wibracje RMS
    if (vib_rms > VIB_WARNING) {
        set_warning(FLAG_VIB_HIGH);
    }
    
    // Sprawdź jitter
    if (jitter_p99 > JITTER_WARNING) {
        set_warning(FLAG_JITTER_HIGH);
    }
}
```

### 3.2 Diagnostyka offline

**Charakterystyka:**
- Cięższa analiza (korelacje, modele, porównania)
- Służy do znalezienia przyczyny
- Korzysta z pełnych logów i danych surowych

```python
# Przykład: analiza offline (Python)
import numpy as np
from scipy import signal

def offline_analysis(log_data):
    # FFT dla wielu sygnałów
    for signal_name in ['omega_err', 'torque', 'vib_rms']:
        f, psd = signal.welch(log_data[signal_name], fs=1000)
        
        # Znajdź piki
        peaks, _ = signal.find_peaks(psd, height=0.1)
        
        # Korelacja z innymi sygnałami
        corr = np.corrcoef(log_data['omega_err'], log_data['torque'])
        
    # Analiza trendów
    for metric in ['temp_bearing', 'vib_rms', 'current']:
        trend = compute_trend(log_data[metric])
        
    return report
```

### 3.3 Zasada rozdziału

> **Online musi być lekko. Offline może być ciężko.**

---

## Czesc IV: Anomalie — proste metody

### 4.1 Progi z histerezą

```c
// Threshold z histerezą
bool check_threshold(float value, float low, float high, bool *state) {
    if (value > high) {
        *state = true;
        return true;  // Alarm!
    } else if (value < low) {
        *state = false;
        return false;  // OK
    }
    // W strefie histerezy - bez zmiany
    return *state;
}
```

**Dlaczego histereza?**

Bez histerezy:
```
Wartość:  99  100  101  100  99  100  101  100
Alarm:    OK  ALARM OK ALARM OK ALARM OK ALARM  (flapping!)
```

Z histerezą (high=100, low=90):
```
Wartość:  99  100  101  100  99  90  89
Alarm:    OK  OK   ALARM ALARM ALARM OK  OK
```

### 4.2 Trend (pochodna)

```c
// Wykrywanie trendu
float compute_trend(float *buffer, int n) {
    // Prosta regresja liniowa
    float sum_x = 0, sum_y = 0, sum_xy = 0, sum_xx = 0;
    
    for (int i = 0; i < n; i++) {
        sum_x += i;
        sum_y += buffer[i];
        sum_xy += i * buffer[i];
        sum_xx += i * i;
    }
    
    float slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    return slope;  // Dodatni = rosnący
}
```

### 4.3 Porównanie do baseline

```python
# Baseline jako funkcja trybu i prędkości
def get_expected(current_mode, current_speed):
    baseline = BASELINE[current_mode]  # słownik
    scale = baseline['scale_factor'] * (current_speed / baseline['ref_speed'])
    return baseline['expected_torque'] * scale

def check_anomaly(actual, expected, tolerance=0.2):
    diff = abs(actual - expected) / expected
    return diff > tolerance
```

---

## Czesc V: Metryki o najwiekszej wartosci

### 5.1 Wibracje

| Metryka | Opis | Użycie |
|---------|------|--------|
| RMS | Średnia energia drgań | Ogólna kondycja |
| Pik FFT | Amplituda dominującej częstotliwości | Identyfikacja rezonansu |
| Trend RMS | Zmiana w czasie | Degradacja |
| Trend pików | Które piki rosną? | Specyficzny problem |

### 5.2 Napęd i obciążenie

| Metryka | Opis | Użycie |
|---------|------|--------|
| Średni moment/prąd vs prędkość | Profil "normalny" | Porównanie |
| Residuum | Odchyłka od profilu | Anomalia |
| Liczba saturacji | Ile razy limiter aktywny | Ograniczenia |

### 5.3 Termika

| Metryka | Opis | Użycie |
|---------|------|--------|
| Temperatura absolutna | Aktualna wartość | Limit |
| Tempo narastania | °C/min | Szybka zmiana |
| Czas w wysokiej temp. | Integral (°C × min) | Długoterminowe zużycie |

### 5.4 Czas rzeczywisty

| Metryka | Opis | Użycie |
|---------|------|--------|
| p99/p99.9 czasu iteracji | Ogon opóźnień | Stabilność |
| Liczba missed deadlines | Ile razy nie zdążono | Degradacja |
| Dropouty komunikacji | Błędy połączenia | Sieć |

---

## Czesc VI: Baseline i "profil normalny"

### 6.1 Po co baseline

Żeby wykrywać anomalie, potrzebujesz punktu odniesienia:

```
              Wibracje
                 ^
    4 mm/s  ----|          ***  (anomalia!)
                 |       **
    2 mm/s  ----|------**------ (normalne)
                 |
                 +-------------> Prędkość
                  5k   10k   15k
```

### 6.2 Profil normalny

```python
# Budowanie profilu normalnego
class Baseline:
    def __init__(self):
        self.profiles = {}  # { mode: { speed_range: (mean, std) } }
    
    def add_measurement(self, mode, speed, metric_value):
        key = (mode, int(speed / 1000) * 1000)  # grupuj co 1k RPM
        
        if key not in self.profiles:
            self.profiles[key] = []
        self.profiles[key].append(metric_value)
    
    def get_expected(self, mode, speed):
        key = (mode, int(speed / 1000) * 1000)
        if key in self.profiles:
            return np.mean(self.profiles[key])
        return None  # Brak baseline
    
    def is_anomaly(self, mode, speed, value, threshold=2.0):
        expected = self.get_expected(mode, speed)
        if expected is None:
            return False
        std = np.std(self.profiles[key])
        return abs(value - expected) > threshold * std
```

### 6.3 Wersjonowanie baseline

**Ważne:** Baseline zmienia się po serwisie!

```python
# Wersjonowanie
baseline = Baseline()
baseline.add_measurement(..., version="before_service_2024_01")

# Po serwisie - nowy baseline
baseline_new = Baseline()
baseline_new.add_measurement(..., version="after_service_2024_02")
```

---

## Czesc VII: Alarmy — eskalacja

### 7.1 Trzy poziomy

| Poziom | Kiedy | Akcja |
|--------|-------|-------|
| **WARNING** | Trend rośnie, ale margines jest | Log, zwiększ częstotliwość pomiarów |
| **DEGRADED** | Przekroczony próg bezpieczny | Ogranicz osiągi (rampy, prędkość) |
| **SAFE_STOP** | Ryzyko uszkodzenia | Natychmiast stop |

### 7.2 Implementacja

```c
typedef enum {
    STATE_OK,
    STATE_WARNING,
    STATE_DEGRADED,
    STATE_SAFE_STOP
} DiagnosticState;

DiagnosticState evaluate_alarms(
    float temperature,
    float vibration,
    float jitter_p99
) {
    // Sprawdź każdą metrykę
    bool temp_warn = temperature > TEMP_WARNING;
    bool vib_warn = vibration > VIB_WARNING;
    bool jitter_warn = jitter_p99 > JITTER_WARNING;
    
    // Logika eskalacji
    if (temperature > TEMP_STOP || vibration > VIB_STOP) {
        return STATE_SAFE_STOP;
    }
    
    if (temperature > TEMP_DEGRADED || vibration > VIB_DEGRADED) {
        return STATE_DEGRADED;
    }
    
    if (temp_warn || vib_warn || jitter_warn) {
        return STATE_WARNING;
    }
    
    return STATE_OK;
}
```

---

## Czesc VIII: Typowe pulapki

### 8.1 Alarmy bez histerezy

→ Flapping (ciągłe przełączanie OK/warning)

### 8.2 Brak okien czasowych

→ Reakcja na pojedyncze piki, nie na trendy

### 8.3 Brak rozróżnienia trybów

→ Alarm w rozruchu, który jest normalny

### 8.4 Logi wpływające na RT

→ Diagnostyka psuje to, co ma mierzyć!

### 8.5 Metryki bez kontekstu

→ "Ładne w dashboardzie", ale bez wartości przy debugowaniu

---

## Czesc IX: Podsumowanie i checklisty

### Checklisty:

- [ ] Metryki spójne online i offline (te same definicje)
- [ ] Alarmy trendowe, nie tylko progowe
- [ ] Logi nie rozwalają RT (asynchroniczne)
- [ ] Tryb degradacji z ograniczeniami (redukcja prędkości/momentu)

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Kontekst jest kluczowy | Tryb, prędkość, temperatura |
| Online = lekko | Proste progi |
| Offline = ciężko | Korelacje, FFT, modele |
| Baseline wersjonowany | Po serwisie zmiana |

---

## Czesc X: Pytania do dyskusji

1. Dlaczego kontekst (tryb, prędkość, temperatura) jest konieczny?
2. Jakie metryki diagnostyczne mogą wpływać na RT?
3. Jak dobrać okno czasowe i histerezę?
4. Jak zaprojektować baseline odporny na zmiany po serwisie?

---

## Czesc XI: Zadania praktyczne

### Zadanie 1: Metryki

Zdefiniuj 10 metryk (wibracje/napęd/termika/RT/komunikacja) i określ które są online a które offline.

### Zadanie 2: Baseline

Zaproponuj baseline dla 2 trybów pracy i sposób wersjonowania.

### Zadanie 3: Eskalacja

Zaprojektuj eskalację alertów: progi + trend + okno czasowe + histereza.

---

## BONUS: Alarmy ktore daja wartosc

Najbardziej wartościowe alarmy to te, które mówią **"dlaczego"** i **"co dalej"**:

Przy każdym alercie loguj:
- Wartość
- Metrykę trendu
- Tryb
- **Rekomendowaną akcję** (degradacja/inspekcja)

---

*(Koniec wykladu 5)*
