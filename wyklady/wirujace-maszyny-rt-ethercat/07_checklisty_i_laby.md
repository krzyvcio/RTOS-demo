# Wykład 7: Checklisty wdrożeniowe i mini-laby (praktyka)

## Czesc I: Wstep teoretyczny — po co nam laboratoria

### 1.1 Geneza — od teorii do praktyki

Proszę wyobrazić sobie sytuację: zbudowałeś system sterowania według wszystkich zasad z wykładów 1-6. Wszystko wygląda dobrze — architektura jest poprawna, RTOS skonfigurowany, EtherCAT działa.

**Ale skąd wiesz, że to rzeczywiście działa?**

**Odpowiedź:** Musisz przetestować. I to nie "sprawdzić czy działa", tylko **zmierzyć i udowodnić**.

### 1.2 Cztery elementy kazdego labu

Każdy lab musi mieć:

| Element | Opis | Przykład |
|---------|------|----------|
| **Dane** | Co mierzysz | Czas pętli, wibracje |
| **Metryka** | Jak to przetwarzasz | Histogram, FFT |
| **Kryterium** | Co znaczy "pass/fail" | p99 < 500 μs |
| **Wniosek** | Co to oznacza dla projektu | "Trzeba izolować rdzeń" |

### 1.3 Zasada: dane przed wnioskiem

> **Bez danych — debugujesz na ślepo.**

Profesor zawsze powtarza: **"Nie mów 'działa', pokaż dane."**

---

## Czesc II: Format logow — artefakty

### 2.1 Minimalne pola (jedna próbka)

```c
struct TelemetrySample {
    uint64_t t_wall_ms;      // Czas ścienny (dla człowieka)
    uint64_t t_mono_ns;      // Monotoniczny czas (do opóźnień)
    uint8_t  mode;          // Tryb pracy
    
    // Sygnały sterowania
    float omega_set;         // Prędkość zadana
    float omega_meas;        // Prędkość mierzona
    float omega_err;         // Błąd regulacji
    float torque_cmd;        // Moment sterujący
    
    // Status
    uint8_t sat_flags;       // Czy były saturacje
    uint32_t rt_loop_us;    // Czas iteracji pętli
    uint8_t miss_deadline;  // Czy deadline przekroczony
    uint8_t comm_drop;       // Czy był dropout
    
    // Opcjonalne
    float vib_rms;          // Wibracje RMS
    float temp_drive;       // Temperatura napędu
};
```

### 2.2 Przyklad linii CSV

```csv
t_wall_ms,t_mono_ns,mode,omega_set,omega_meas,torque_cmd,rt_loop_us,miss_deadline
1234567890,9876543210,NORMAL,1000.0,998.5,0.15,450,0
1234567891,9876544210,NORMAL,1000.0,999.1,0.09,445,0
```

### 2.3 Wersjonowanie schematu

```c
#define SCHEMA_VERSION 2

struct TelemetryV2 {
    uint32_t schema_version;  // = 2
    // ... reszta pól
};
```

---

## Czesc III: Mini-lab 1: Pomiar jitteru petli

### 3.1 Cel

Zmierzyć histogram czasu iteracji pętli sterowania i wykryć ogon opóźnień.

### 3.2 Kroki

```
KROK 1: Dodaj timestamp na wejściu i wyjściu pętli (monotoniczny)
KROK 2: Zapisuj rt_loop_us = t_end - t_start do bufora (bez IO w pętli RT!)
KROK 3: Raz na N iteracji lub w wątku nie-RT zrzucaj bufor do pliku
KROK 4: Policz histogram i percentyle (p95/p99/p99.9)
```

### 3.3 Kod

```c
// Rejestrator czasu pętli
#define N_SAMPLES 10000

uint32_t loop_times[N_SAMPLES];
int sample_idx = 0;

void record_loop_time(uint64_t dt_us) {
    loop_times[sample_idx] = dt_us;
    sample_idx = (sample_idx + 1) % N_SAMPLES;
    
    if (sample_idx == 0) {
        // Zrzut do logu (poza pętlą RT!)
        log_histogram_to_file(loop_times, N_SAMPLES);
    }
}
```

### 3.4 Kryteria pass/fail

| Kryterium | Pass | Fail |
|-----------|------|------|
| p99 | < 80% cyklu | > 80% cyklu |
| Missed deadline | < 0.1% | > 0.1% |
| Ogon | Brak serii | Serie correlated z telemetrią/IO |

### 3.5 Wniosek projektowy

> Jeśli ogon rośnie przy telemetrii → izolujesz wątki, ograniczasz logowanie, porządkujesz priorytety.

---

## Czesc IV: Mini-lab 2: FFT bledow predkosci

### 4.1 Cel

Zebrać sygnał błędu prędkości, policzyć FFT i znaleźć piki.

### 4.2 Kroki

```
KROK 1: Zbierz okno danych omega_err (i najlepiej też torque_cmd oraz vib_rms)
KROK 2: Policz FFT, zidentyfikuj 3-5 największych pików
KROK 3: Sprawdź, czy piki są stałe w trybie i czy przesuwają się z prędkością
KROK 4: Zaproponuj reakcję: notch / ograniczenie pasma / zmiana ramp / inspekcja mechaniki
```

### 4.3 Kod

```python
import numpy as np
from scipy import signal

def compute_fft(signal_data, fs):
    # FFT
    f, psd = signal.welch(signal_data, fs=fs, nperseg=1024)
    
    # Znajdź piki
    peaks, properties = signal.find_peaks(psd, height=0.001)
    
    # Zwróć częstotliwości i amplitude
    peak_freqs = f[peaks]
    peak_heights = psd[peaks]
    
    return peak_freqs, peak_heights
```

### 4.4 Kryteria pass/fail

| Kryterium | Pass | Fail |
|-----------|------|------|
| Piki po zmianie | Spadają | Nie spadają lub rosną |
| Stabilność | Brak nowych oscylacji | Nowe oscylacje |
| Błąd regulacji | Bez pogorszenia | Rośnie błąd |

---

## Czesc V: Mini-lab 3: Test dropout komunikacji

### 5.1 Cel

Zasymulować przerwę danych (np. brak aktualizacji) i sprawdzić zachowanie watchdogów.

### 5.2 Kroki

```
KROK 1: Zasymuluj dropout (np. zatrzymaj wysyłkę komend lub wstrzymaj odbiór)
KROK 2: Zmierz czas do wykrycia (watchdog) po stronie slave i master
KROK 3: Sprawdź, w jaki stan przechodzi system (DEGRADED/SAFE_STOP)
KROK 4: Sprawdź warunki powrotu: brak flappingu, jawny recovery
```

### 5.3 Narzędzia

```bash
# Symulacja dropout (iptables)
iptables -A INPUT -m statistic --mode random --probability 0.1 -j DROP

# Symulacja opóźnienia (tc)
tc qdisc add dev eth0 root netem delay 10ms
```

### 5.4 Kryteria pass/fail

| Kryterium | Pass | Fail |
|-----------|------|------|
| Czas reakcji | W zdefiniowanym czasie | Brak reakcji |
| Stan końcowy | Deterministiczny (DEGRADED/SAFE_STOP) | Utrzymanie ostatniej komendy |
| Recovery | Kontrolowany | Flapping |

---

## Czesc VI: Mini-lab 4: Telemetria kontra RT

### 6.1 Cel

Udowodnić, że telemetria i logowanie nie psują deterministyki.

### 6.2 Kroki

```
KROK 1: Uruchom system w baseline (minimalna telemetria), zbierz p99/p99.9 rt_loop_us
KROK 2: Włącz telemetrię (zwiększ częstotliwość / ilość danych), zbierz ponownie
KROK 3: Porównaj ogon opóźnień i liczbę missed deadlines
KROK 4: Wprowadź poprawkę: rate limiting, osobny wątek/proces, backpressure
```

### 6.3 Porównanie

```
Baseline (minimalna telemetria):
  p50:   320 μs
  p95:   380 μs
  p99:   420 μs
  p99.9: 480 μs
  missed: 0

Bogata telemetria (bez izolacji):
  p50:   350 μs
  p95:   650 μs
  p99:   1200 μs    ← PROBLEM!
  p99.9: 3500 μs    ← KATASTROFA!
  missed: 15

Bogata telemetria (z izolacją):
  p50:   325 μs
  p95:   390 μs
  p99:   450 μs
  p99.9: 510 μs
  missed: 0
```

### 6.4 Wniosek projektowy

> Telemetria ma być "najpierw nieszkodliwa", dopiero potem "bogata".

---

## Czesc VII: Checklisty wdrozeniowe

### 7.1 Checklisty (przed wdrożeniem)

- [ ] Zmierz i zapisz budżet czasu end-to-end
- [ ] Zmierz jitter i WCRT w warunkach worst-case
- [ ] Wydziel wątki RT i odseparuj logowanie/telemetrię
- [ ] Zidentyfikuj rezonanse (FFT/sweep) i zaplanuj tłumienie
- [ ] Przetestuj awarie: brak czujnika, przegrzanie, dropout, missed deadline

### 7.2 Checklisty (review architektury)

- [ ] Czy pętla krytyczna ma zdefiniowany okres, deadline i reakcję na missed deadline?
- [ ] Czy dane cykliczne są minimalne, a reszta jest poza cyklem i rate-limited?
- [ ] Czy drive ma watchdog i safe behavior bez mastera?
- [ ] Czy logowanie jest asynchroniczne i nie blokuje RT?
- [ ] Czy masz baseline metryk (przed/po zmianie) i porównujesz ogony rozkładów?

---

## Czesc VIII: Pytania do dyskusji

### 8.1 Jak definiujesz kryterium "pass/fail" dla jitteru?

Odpowiedź zależy od:
- Wymagań mechaniki (jaki jitter jest akceptowalny?)
- Marginesu bezpieczeństwa (ile "zapasu" potrzebujesz?)
- Warunków worst-case (co może się zdarzyć w produkcji?)

### 8.2 Która polityka bufora telemetrii jest lepsza: drop czy overwrite?

| Polityka | Zalety | Wady |
|----------|--------|-------|
| **Drop** | RT niecierpi | tracisz dane |
| **Overwrite** | Zawsze masz dane | tracisz historię |

Wybór zależy od zastosowania.

### 8.3 Jak pokażesz, że telemetria "nie szkodzi" RT?

- Porównanie p99/p99.9 przed/po
- Testy obciążeniowe
- Monitoring missed deadlines

### 8.4 Jakie metryki uznasz za obowiązkowe w raporcie z labów?

1. Percentyle (p50, p95, p99, p99.9)
2. Liczba missed deadlines
3. Histogram
4. Kontekst (tryb, prędkość, obciążenie)

---

## Czesc IX: Projekty studenckie

### 9.1 Lab harness

Zestaw skryptów uruchamiających laby i generujących raport (CSV + wykresy + wnioski).

```python
# Przykład: uruchomienie labu
def run_jitter_lab():
    # Setup
    start_system()
    
    # Lab
    collect_data(duration=60)  # 60 sekund
    
    # Analiza
    results = analyze_jitter()
    
    # Raport
    generate_report(results, format="html")
    
    # Cleanup
    stop_system()
```

### 9.2 Regression kit

Scenariusze regresji uruchamiane automatycznie:
- Dropout komunikacji
- Opóźnienie
- Saturacja
- Telemetria flood

```yaml
# regression.yaml
tests:
  - name: dropout_10%
    action: iptables -A INPUT -m statistic --mode random --probability 0.1 -j DROP
    duration: 60s
    pass_criteria:
      - missed_deadlines < 10
      - recovery_time < 1s
      
  - name: telemetry_flood
    action: increase_telemetry_rate(10x)
    duration: 60s
    pass_criteria:
      - p99 < 500us
      - missed_deadlines < 5
```

### 9.3 Metrics spec

Specyfikacja formatu logu + walidator:

```python
# validator.py
class TelemetryValidator:
    REQUIRED_FIELDS = [
        't_wall_ms', 't_mono_ns', 'mode',
        'omega_set', 'omega_meas', 'torque_cmd',
        'rt_loop_us', 'miss_deadline'
    ]
    
    def validate(self, record):
        for field in self.REQUIRED_FIELDS:
            if field not in record:
                raise ValidationError(f"Missing field: {field}")
        
        if record['rt_loop_us'] > 10000:  # > 10ms
            raise ValidationError("Suspicious rt_loop_us")
```

---

## Czesc X: Podsumowanie

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Dane przed wnioskiem | Mierzysz, potem wnioskujesz |
| Pass/fail kryteria | Musisz wiedzieć kiedy jest dobrze |
| Reprodukowalność | Ten sam lab = ten sam wynik |
| Automatyzacja | Skrypty > ręczne testy |

### Checklisty:

- [ ] Budżet czasu end-to-end zmierzony
- [ ] Jitter/WCRT w worst-case zmierzony
- [ ] Wątki RT odseparowane
- [ ] Awarie przetestowane (fault injection)

---

## BONUS: Traktuj laby jak testy regresji

Jeśli po zmianie systemu nie potrafisz łatwo porównać wyników "przed/po" — lab nie jest jeszcze narzędziem inżynierskim.

**Dobre narzędzie inżynierskie:**
- Uruchamiasz jednym poleceniem
- Dostajesz wynik pass/fail
- Możesz porównać z poprzednimi wynikami
- Widzisz trend (poprawa/pogorszenie)

---

*(Koniec wykladu 7)*
