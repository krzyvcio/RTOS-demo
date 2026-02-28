# Wyklad 10 (2035): Robotyka + wirowki + RTOS — zintegrowany obraz systemow autonomicznych

## Czesc I: Wstep teoretyczny — wizja 2035

### 1.1 Geneza — skad pochodzi ta wizja

Prosze wyobrazic sobie rok 2035. Laboratorium farmaceutyczne:

```
Kamera 360° widzi:
- Manipulator ABB wyjmuje probke z statywu
- Przenosi ja do wirówki Eppendorf
- Wirówka automatycznie uruchamia protokól
- Po 5 minutach — wirówka sygnalizuje koniec
- Manipulator odbiera probke i umieszcza w chlodziarce
- System LIMS zapisuje wszystko do bazy

Cały proces bez udziału czlowieka. Przez 24 godziny na dobe.
```

To nie jest science fiction — to kierunek, w którym zmierza przemysł.

### 1.2 Co sie zmienilo

| 2020 | 2035 |
|------|------|
| Sterowanie ręczne | Automatyzacja pełna |
| Separacja IT/OT | Integracja |
| Jeden robot | Komórki autonomiczne |
| Diagnostyka po awarii | Predykcja awarii |
| Paper trails | Digital audit trail |
| Czlowiek w petli | Czlowiek nad petla |

### 1.3 Co NIE sie zmienilo

Mimo wszystkich zmian, **zasady architektury RT pozostają takie same:**

- Hard RT w W1 (naped, safety)
- Firm RT w W2 (master, sterowanie)
- Soft RT w W3 (nadzor, workflow)
- Offline w W4 (modelowanie)

**Bo fizyka się nie zmieniła:**
- Prędkość światła nadal ogranicza komunikację
- CPU nadal ma skończoną moc
- Sprzęt nadal może się zepsuć

---

## Architektura robota

###  Czesc II:2.1 Pipeline robota autonomicznego

```
sensory -> timestamping -> percepcja -> fuzja stanu -> plan -> kontrola -> napedy -> swiat
    |            |            |           |         |         |
    v            v            v           v         v         v
  IMU,        Czas         Wykrywanie  Estymacja  Trajekt.  PWM,
  enkodery    (DC)         obiektow    pozycji    czas      prad
```

### 2.2 Sensoryka — co jest typowe w 2035

| Sensor | Czestotliwosc | Latency | Zastosowanie |
|--------|---------------|---------|--------------|
| IMU | 1 kHz | < 1 ms | Stabilizacja |
| Enkodery | 10-100 kHz | < 0.1 ms | Pozycja |
| Lidary | 10-40 Hz | 20-50 ms | Mapa, wykrywanie |
| Kamery | 30-120 Hz | 30-100 ms | Rozpoznawanie |
| Czujniki sily | 1-10 kHz | < 1 ms | Kontakt |
| Bezpieczenstwa | Ciagle | < 1 ms | E-stop, strefy |

**Kluczowe wyzwanie:**
Każdy sensor ma inny czas, opóźnienie i zawodność. Dlatego timestamping i synchronizacja czasu są "pierwszą klasą" problemu.

### 2.3 Poziomy sterowania w robotyce

```
Poziom 1: Naped (prad/moment)
    - Czestotliwosc: 10-100 kHz
    - Jitter: < 1 μs
    - Technika: FOC (Field Oriented Control)

Poziom 2: Przegub (predkosc/pozycja)
    - Czestotliwosc: 1-10 kHz
    - Jitter: < 10 μs
    - Technika: PID, feedforward

Poziom 3: Kontrola w przestrzeni (trajektorie, kontakt)
    - Czestotliwosc: 100 Hz - 1 kHz
    - Jitter: < 1 ms
    - Technika: MPC, impedance control

Poziom 4: Percepcja i planowanie
    - Czestotliwosc: 1-30 Hz
    - Jitter: nie krytyczny
    - Technika: ML, path planning
```

---

## Czesc III: Real-time w robotyce

### 3.1 Dlaczego RT to nie tylko "szybko"

Profesor chciałby, żebyście zapamiętali:

> **RT w robotyce to nie jest "szybko" — to jest "przewidywalnie".**

**Dlaczego?**

Wyobraź sobie ramię robota podnoszące kieliszek:

```
Wersja A: Czas wykonania = 100ms ± 10ms (jitter = 10%)
    - Ramię delikatnie drży
    - Kieliszek może się rozbić
    
Wersja B: Czas wykonania = 110ms ± 1ms (jitter = 1%)
    - Ramię idzie płynnie
    - Kieliszek bezpieczny
```

**Jitter niszczy stabilność i jakość!**

### 3.2 Harmonogramowanie i izolacja

W 2035 standardem jest "budgeted RT":

```c
// Konfiguracja budget
struct RTBudget {
    uint64_t period_ns;       // 1 ms
    uint64_t wcet_ns;         // 500 μs (budget)
    uint64_t deadline_ns;     // 800 μs
    int      priority;        // 99
    int      cpu_core;        // 0
};

struct RTThread threads[] = {
    { .period_ns = 100000,   .wcet_ns = 50000,  .priority = 99, .cpu_core = 0 },  // Naped
    { .period_ns = 1000000,  .wcet_ns = 200000, .priority = 80, .cpu_core = 0 },  // Sterowanie
    { .period_ns = 10000000, .wcet_ns = 1000000,.priority = 50, .cpu_core = 1 },  // Planowanie
};
```

**Zasady:**
- Wątki RT mają zdefiniowane deadline'y
- Non-RT jest izolowane (osobne rdzenie/procesy)
- Telemetria i ML inference nie mogą zabierać budżetu pętli sterowania

---

## Czesc IV: Wirowki jako maszyny wirujace

### 4.1 Czym sie wyróżniaja

Wirowki laboratoryjne i przemysłowe (jako klasa maszyn wirujących) wyróżniają się:

| Cecha | Implikacja dla sterowania |
|-------|--------------------------|
| Wysoka energia kinetyczna | Bezpieczny stop krytyczny |
| Wysoka czułość na drgania | Filtracja, ograniczenie pasma |
| Rezonanse mechaniczne | Ograniczenie pasma przy przejściu |
| Wymaganie bezpiecznego zato | Redundancja, watchdog |
| Stan mechaniki (łożyska) | Predykcyjna diagnostyka |

### 4.2 Co sterujemy w praktyce

```c
// Parametry sterowania wirówki
struct CentrifugeControl {
    float omega_set;           // Zadana predkosc [rad/s]
    float omega_ramp_rate;     // Maksymalna zmiana [rad/s^2]
    float max_jerk;            // Ograniczenie jerk [rad/s^3]
    
    float current_limit;       // Limit pradu [A]
    float torque_limit;        // Limit momentu [Nm]
    
    float vibration_threshold; // Próg wibracji [g]
    float temperature_limit;   // Limit temperatury [°C]
};
```

### 4.3 Profil predkosci

```
omega
  ^
  |        /¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
  |       /          praca ustalona
  |      /
  |     /
  |    /
  |___/_________________________> t
    start   rampa    hamowanie
```

**Fazy:**
1. **Start** — inicjalizacja, sprawdzenie
2. **Rampa** — rozpędzanie z ograniczeniem jerk
3. **Praca ustalona** — utrzymanie predkosci
4. **Przejście przez rezonans** — ominięcie pasm krytycznych
5. **Hamowanie** — kontrolowane zatrzymanie

---

## Czesc V: Predykcyjna diagnostyka

### 5.1 Idea

W 2035 "condition monitoring" jest integralną częścią systemu:

```
Normalna praca:
    v < 0.5 g, T < 40°C, jitter < 50 μs
    
Zblizanie sie do limitu:
    v > 0.7 g LUB T > 60°C LUB jitter > 100 μs
    → WARNING
    
Ograniczony tryb:
    v > 0.9 g LUB T > 70°C
    → DEGRADED (zmniejsz predkosc)
    
Bezpieczny stop:
    v > 1.0 g LUB T > 80°C
    → SAFE_STOP
```

### 5.2 Metryki do monitorowania

| Metryka | Źródło | Czestotliwosc | Próg warning |
|---------|--------|---------------|---------------|
| Wibracje | Akcelerometr | 1 kHz | 0.5 g |
| Temperatura | Termistor | 10 Hz | 60°C |
| Jitter | Pomiar RT | 1 kHz | 100 μs |
| Prad | ADC | 10 kHz | 80% limitu |
| Predkosc | Enkoder | 10 kHz | 95% max |

### 5.3 Zasada projektowania

**Klucz:** Diagnostyka musi być tak zaprojektowana, by nie psuła RT!

```python
# DIAGNOSTYKA JEST ASYNCHRONICZNA

# Watek RT - tylko zbiera dane
def rt_loop():
    data = read_sensors()
    control = compute_control(data)
    publish_to_ringbuffer(data)  # Szybko, nieblokująco
    
    # NIE wykonuj diagnostyki tutaj!

# Watek diagnostyki - non-RT
def diagnostic_loop():
    while True:
        data = read_from_ringbuffer()
        
        # Analiza - moze byc wolna
        analyze_trends(data)
        check_thresholds(data)
        
        # Jesli przekroczone → wyślij do nadzoru
        if data.vibration > WARNING_THRESHOLD:
            send_warning("Vibration high")
        
        sleep(1)  # 1 Hz - wystarczy dla diagnostyki
```

---

## Czesc VI: MCU + RTOS w urzadzeniach wirujacych

### 6.1 Dlaczego to sie nie zmieni

W urządzeniach z twardymi wymaganiami:
- Szybkie pętle (np. naped) siedzą blisko sprzętu
- RTOS daje przewidywalność i prostotę reakcji awaryjnych
- Watchdog i limity sprzętowe są niezależne od "smart" warstwy wyżej

**To jest wspólne dla:**
- Robotyki (serwonapędy)
- Maszyn wirujących (napędy, stabilizacja)
- Automatyki krytycznej (safety)

### 6.2 Architektura 2035

```
                    +------------------+
                    |   Chmura/Edge    |
                    |   (ML inference) |
                    +--------+---------+
                             |
                    +--------v---------+
                    |   Gateway/Edge   |
                    |   (aggregacja)   |
                    +--------+---------+
                             |
         +-------------------+-------------------+
         |                   |                   |
+--------v-------+  +-------v--------+  +------v-------+
|  Wirówka 1    |  |   Wirówka 2    |  |   Wirówka N   |
|  +---------+  |  |   +---------+  |  |   +---------+ |
|  |  Master |  |  |   |  Master |  |  |   |  Master | |
|  |  (Linux)|  |  |   |  (Linux)|  |  |   |  (Linux)| |
|  +----+----+  |  |   +----+----+  |  |   +----+----+ |
|       |       |  |        |       |  |        |       |
|  +----v----+  |  |   +----v----+  |  |   +----v----+ |
|  | MCU/RTOS|  |  |   | MCU/RTOS|  |  |   | MCU/RTOS| |
|  +---------+  |  |   +---------+  |  |   +---------+ |
+------------------+  +---------------+  +--------------+
```

**Zasada:** Edge computing przetwarza dane lokalnie, chmura służy do modelowania i aktualizacji.

---

## Czesc VII: Integracja robot + modul wirówki

### 7.1 Komórka autonomiczna — wzorzec 2035

W 2035 typowy wzorzec to "komórka autonomiczna":

```
+------------------+     +-------------------+
|     Robot        |     |   Moduł procesu   |
|  (manipulator)   |     |   (wirówka)       |
+------------------+     +-------------------+
        |                         |
        |  Kontrakt:             |
        |  - Stan                |
        |  - Zdarzenia          |
        |  - Setpointy          |
        v                         v
+------------------+     +-------------------+
|   Orkiestrator   |     |   (część robota) |
|  (workflow)      |     +-------------------+
+------------------+
        |
        v
+------------------+
|  LIMS / MES      |
|  (system nadrz.) |
+------------------+
```

### 7.2 Interfejsy — praktyczne

**Ważniejsze od "jakiego protokołu" jest:**

1. **Jawny kontrakt danych** (wersjonowany)
2. **Stan maszyny jako FSM** (READY/RUNNING/FAULT/SAFE_STOP)
3. **Zdarzenia i alarmy jako strumień** (nie polling!)

```python
# Przykladowy kontrakt
class ModuleContract:
    STATE_SCHEMA = {
        "module_state": ["FAULT", "SAFE_STOP", "READY", "RUNNING"],
        "robot_state": ["IDLE", "MOVING", "INTERLOCK", "FAULT"],
        "current_speed": 0.0,  # rad/s
        "temperature": 25.0,   # °C
    }
    
    EVENT_SCHEMA = {
        "type": ["ALARM", "COMPLETE", "INTERLOCK", "E_STOP", "FAULT"],
        "timestamp": 1234567890,  # Unix
        "details": {}  # slownik
    }
```

### 7.3 Odpornosc na bledy

**Przykładowe błędy:**

| Blad | Reakcja |
|------|---------|
| Robot upuszcza próbkę | Moduł raportuje FAULT, orkiestrator powiadamia operatora |
| Moduł procesu zgłasza FAULT | Robot wycofuje się do strefy bezpiecznej |
| Komunikacja ma dropout | Lokalny watchdog wyzwala SAFE_STOP |
| Pętla RT ma missed deadline | Lokalny watchdog wyzwala SAFE_STOP |

**Wymagania:**
- Deterministyczne zachowanie (nie "dziwnie się zachował")
- Logi i audit trail
- Mechanizmy recovery, które nie naruszają bezpieczeństwa

---

## Czesc VIII: Trendy 2035

### 8.1 Roboty kolaboracyjne

**Rok 2035:**
- Więcej pracy obok człowieka
- Więcej czujników bezpieczeństwa i ograniczeń energii
- Większa rola formalizacji zachowań bezpiecznych

**Implikacje dla architektury:**
- Safety jako fundamentalna warstwa
- Redundancja czujników
- Formalna weryfikacja zachowań

### 8.2 Autonomiczne laboratoria

**Rok 2035:**
- Automatyzacja przepływu pracy (workflow)
- Integracja wielu modułów procesu
- Nacisk na powtarzalność, audyt i zgodność (compliance)

**Implikacje:**
- Digital twin jako standard
- Pełna ścieżka audytu (od zamówienia do wyniku)
- Reprodukowalność warunków

### 8.3 Inteligentna diagnostyka

**Rok 2035:**
- Detekcja anomalii i predykcja awarii (modele + dane)
- Ale z twardą zasadą: **ML nie może być w krytycznej ścieżce RT bez barier bezpieczeństwa**

**Zasada:**
```
ML inference → timeout → fallback → degradacja
     |              |          |
     v              v          v
  Wynik        Poprzedni    Safe stop
              wynik (OK)
```

---

## Czesc IX: Etyka, bezpieczenstwo i odpowiedzialnosc inzynierska

### 9.1 Odpowiedzialnosc praktyczna

W systemach krytycznych odpowiedzialność jest praktyczna:

1. **Projektujesz tak, by awarie były przewidywalne i bezpieczne**
2. **Unikasz "czarnych skrzynek" w kanale bezpieczeństwa**
3. **Dbasz o prywatność i integralność danych**
4. **Masz procedury: testy awaryjne, fault injection, regresja**

### 9.2 Minimalne zasady

| Zasada | Implikacja |
|--------|------------|
| Safety jest oddzielnym kanałem decyzyjnym | Działa lokalnie, niezależnie |
| Każdy tryb awarii ma zdefiniowaną reakcję | FSM dla każdego scenariusza |
| Metryki determinizmu są mierzone stale | Telemetria od pierwszego dnia |
| Testy awaryjne = regresja | CI/CD zawiera fault injection |

### 9.3 Pytania do review (zanim oddasz system)

1. Co się stanie, gdy zniknie sensor?
2. Co się stanie, gdy master przestanie odpowiadać?
3. Czy system może wejść w stan niebezpieczny przez "ładny" dashboard?
4. Czy logi pozwalają odtworzyć każdą awarię?
5. Czy "sprytna" logika (ML) może zagrozić bezpieczeństwu?

---

## Czesc X: Podsumowanie i checklisty

### Checklisty:

- [ ] Czy pipeline sens→control ma spójność czasu (timestamping, synchronizacja)?
- [ ] Czy wątki RT są izolowane od telemetrii/ML/HMI?
- [ ] Czy moduł wirówki ma watchdog i safe behavior bez systemu nadrzędnego?
- [ ] Czy integracja robot+moduł ma FSM i deterministyczne zachowanie na błędy?
- [ ] Czy masz audytowalne logi i testy fault injection jako regresję?

### Slajd podsumowujacy: 2035 w jednym zdaniu

> **Systemy autonomiczne wygrywają deterministyka + safety + diagnozowalnosc**

---

## Czesc XI: Pytania do dyskusji

1. Które elementy "autonomii" powinny być ograniczone przez twarde bariery safety i dlaczego?
2. Jak zaprojektujesz integrację robota z modułem procesu, by awarie były przewidywalne (FSM, events, audit)?
3. Jakie metryki determinizmu i diagnostyki uznasz za obowiązkowe w komórce autonomicznej?
4. Jakie decyzje (np. ML) mogą być pomocnicze, ale nie mogą być krytyczne dla safety?

---

## Czesc XII: Projekty studenckie

### Projekt 1: Autonomous Cell

Projekt architektury komórki (robot + moduł procesu) z kontraktami i FSM.

### Projekt 2: Predictive Monitoring

Pipeline trendów (wibracje/termika/jitter) z eskalacją i raportem.

### Projekt 3: Audit Trail

Format logów i zdarzeń z wersjonowaniem, żeby dało się odtworzyć decyzje systemu.

---

## BONUS: Przewaga inzynierska 2035

> **W 2035 przewaga inżynierska to nie "więcej AI", tylko umiejętność projektowania granic: co jest deterministyczne, co jest probabilistyczne, i jak system zachowuje się bezpiecznie, gdy probabilistyczne zawiedzie.**

---

*(Koniec wykladu 10)*
