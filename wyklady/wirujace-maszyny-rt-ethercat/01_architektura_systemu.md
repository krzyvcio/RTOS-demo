# Wykład 1: Architektura sterowania maszyną wirującą (praktycznie)

## Czesc I: Wstep teoretyczny — czym jest maszyna wirująca i dlaczego jest specjalna

### 1.1 Geneza — dlaczego maszyny wirujące są trudne

Proszę wyobrazić sobie sytuację: budujemy sterowanie dla wirówki laboratoryjnej, która ma osiągać 15 000 obrotów na minutę. Wirnik waży 2 kg i jest zamontowany na precyzyjnych łożyskach.

Co się dzieje przy takiej prędkości:

- **Energia kinetyczna**: E = ½Jω² = ½ × 2 × (15000 × 2π/60)² ≈ 123 000 J
- **Siła odśrodkowa na nierównowagę 1g**: F = m × r × ω² ≈ 2 × 0.001 × (1570)² ≈ 5000 N

To jest energia, która może zniszczyć łożyska, obudowę, a nawet zranić operatora.

**Dlaczego o tym mówię?** Bo sterowanie taką maszyną to nie jest "po prostu regulacja prędkości". To jest zarządzanie energią, drganiami, bezpieczeństwem — wszystko jednocześnie.

### 1.2 Elementy maszyny wirującej

Proszę zapamiętać siedem elementów, które zawsze występują:

```
Maszyna wirująca:
    |
    +-- Wirnik (bezwładność J)
    |       - masa, geometryczne centrum
    |       - rezonanse mechaniczne
    |
    +-- Napęd (silnik + falownik)
    |       - sterowanie prądem/momentem (FOC)
    |       - limity prądowe, termiczne
    |
    +-- Łożyskowanie i konstrukcja
    |       - rezonanse, drgania
    |       - tarcie, luzy
    |
    +-- Czujniki
    |       - prędkość (enkoder, tachometr)
    |       - wibracje (akcelerometr)
    |       - temperatury (termistory, PT100)
    |
    +-- System sterowania
    |       - pętle regulacji (prąd, prędkość)
    |       - profile ruchu (rampy, jerk)
    |
    +-- System nadzoru
    |       - limity, watchdog
    |       - safe stop
    |
    +-- System komunikacji
            - EtherCAT, Ethernet, CAN
            - synchronizacja
```

### 1.3 Minimalny model fizyczny

Na początek potrzebujemy równania, które opisuje ruch wirnika:

```
J × dω/dt = T_motor - T_load - T_losses
```

Wyjaśnienie:
- **J** — moment bezwładności wirnika (fizyka, nie software!)
- **ω** (omega) — prędkość kątowa
- **T_motor** — moment generowany przez silnik (sterujemy tym)
- **T_load** — moment obciążenia (zakłócenie)
- **T_losses** — straty (tarcie, wentylacja)

**Dlaczego to nie wystarczy?** Bo w prawdziwym świecie:
- Rezonanse mechaniczne powodują, że model jest nieliniowy
- Opóźnienia i jitter w sterowaniu psują stabilność
- Saturacje prądu/momentu wprowadzają nieliniowości

---

## Czesc II: Łańcuch sygnałowy — najważniejszy rysunek

### 2.1 Krytyczna ścieżka end-to-end

Proszę zapamiętać ten schemat, bo będzie wracał w każdym wykładzie:

```
(sensor) -> (timestamp/buffer) -> (filter/estimator) -> (controller) -> (transport) -> (drive) -> (plant)
   |              |                    |               |            |          |           |
   v              v                    v               v            v          v           v
odczyt       buforowanie         estymacja       regulator    transport    PWM      mechanika
pomiaru      (opóźnienie)        (opóźnienie)    (obliczenia) (sieć)    (opóźn.)  (obiekt)
```

**Każdy element na tej ścieżce dokłada:**
- Opóźnienie (latency)
- Jitter (zmienność)
- Ryzyko dropoutów (utraty danych)

### 2.2 Dlaczego to jest kluczowe

Profesor zawsze mówi: **"W debugowaniu najczęściej nie debugujesz modelu, tylko opóźnienia i zależności między blokami."**

Jeśli gdziekolwiek ten łańcuch ma:
- **Losowe opóźnienie** → zobaczysz niestabilność
- **Sporadyczne dropouty** → zobaczysz "dziwne" oscylacje
- **Brak spójnego czasu** → zobaczysz błędy estymacji

---

## Czesc III: Warstwy systemu

### 3.1 Warstwa napędowa (kHz)

Najniższa warstwa, najbliżej sprzętu:

```c
// Typowe zadania:
void pwm_update();          // Aktualizacja PWM (20-50 μs)
void current_measurement(); // Pomiar prądu ADC
void safety_check();        // Sprawdzenie limitów (natychmiast)
```

**Charakterystyka:**
- Częstotliwość: 1-100 kHz
- Jitter: < 1 μs (najtwardsze wymagania)
- Zawiera pętlę prądu (FOC — Field Oriented Control)

### 3.2 Warstwa regulacji ruchu (100 Hz – kHz)

```c
// Typowe zadania:
void velocity_controller(); // Regulator prędkości PI/PID
void trajectory_update();   // Aktualizacja trajektorii
void resonance_filter();    // Filtr notch/ograniczenie pasma
```

**Charakterystyka:**
- Częstotliwość: 100 Hz – 10 kHz
- Jitter: < 10-100 μs
- Zawiera pętlę prędkości

### 3.3 Warstwa procesu (Hz – 100 Hz)

```c
// Typowe zadania:
void recipe_manager();      // Zarządzanie recepturami
void sequence_control();    // Sekwencje start/stop
void ramp_generator();      // Generowanie ramp prędkości
```

**Charakterystyka:**
- Częstotliwość: 1-100 Hz
- Jitter: niekrytyczny
- Logika biznesowa, receptury

### 3.4 Warstwa diagnostyki i bezpieczeństwa

```c
// Typowe zadania:
void vibration_monitor();  // Monitoring wibracji
void temperature_check();  // Sprawdzenie temperatury
void watchdog_manager();   // Zarządzanie watchdog
void safety_fsm();         // FSM bezpieczeństwa
```

---

## Czesc IV: Granice odpowiedzialnosci — drive vs master

### 4.1 Kluczowa decyzja architektoniczna

To jest najczęstsze źródło problemów — zły podział pętli.

**Praktyczna heurystyka:**

| Pętla | Gdzie umieścić | Uzasadnienie |
|--------|---------------|--------------|
| Prąd/moment | **W napędzie** | Najwyższe pasmo, najkrótsze opóźnienia |
| Zabezpieczenia prądowe/termiczne | **W napędzie** | Reakcja natychmiastowa |
| Prędkość | Zależnie | W napędzie (proste) lub master (zaawansowane) |
| Koordynacja, profile | **W masterze** | Wymaga widoku całego systemu |

### 4.2 Pytania, na które musisz odpowiedzieć

Jeśli przenosisz pętlę prędkości do mastera:

1. **Jak duże jest opóźnienie i jitter całego toru?**
   - Pomiar → Sterowanie → Napęd → PWM

2. **Czy cykl komunikacji jest spójny z cyklem sterowania?**
   - Niespójność = "beat frequencies" = oscylacje

3. **Co się dzieje przy dropoutach?**
   - Watchdog? Degradacja? Safe stop?

---

## Czesc V: EtherCAT w systemie

### 5.1 Gdzie EtherCAT jest elementem "kręgosłupa"

EtherCAT służy do:
- Podłączenia napędów (drive'ów)
- Podłączenia I/O (czujniki, moduły analogowe)
- Synchronizacji (Distributed Clocks)
- Deterministycznej wymiany danych w cyklu

### 5.2 Kiedy EtherCAT ma sens

EtherCAT jest atrakcyjny, gdy:
- Potrzebujesz stałego cyklu (< 1 ms)
- Potrzebujesz niskiego jitteru (< 1 μs)
- Synchronizujesz wiele osi/modułów
- Masz wiele urządzeń na jednej sieci

---

## Czesc VI: RTOS vs Linux PREEMPT_RT

### 6.1 Kiedy RTOS wygrywa

**RTOS** (FreeRTOS, Zephyr, VxWorks):
- Przewidywalność (mniej "niespodzianek")
- Łatwiejsza certyfikacja w systemach krytycznych
- Mniejszy komfort w rozwoju aplikacji

**Wybierz RTOS gdy:**
- Pętla prądu musi być bardzo szybka (< 50 μs)
- Potrzebujesz pełnej kontroli nad sprzętem
- System jest prosty (jeden mikrokontroler)

### 6.2 Kiedy Linux PREEMPT_RT wygrywa

**Linux PREEMPT_RT:**
- Dobry kompromis "RT + ekosystem"
- Wymaga dyscypliny: izolacja rdzeni, priorytety
- Łatwiejsza integracja z aplikacjami wyższymi

**Wybierz Linux PREEMPT_RT gdy:**
- Potrzebujesz sieci, GUI, bazy danych
- System jest złożony (wiele modułów)
- Potrzebujesz narzędzi debugowania

### 6.3 Zasada praktyczna

> Jeśli pętla sterowania jest bardzo krytyczna i twarda czasowo, umieszczasz ją jak najbliżej sprzętu (drive/MCU/RTOS), a Linux RT wykorzystujesz na poziomie koordynacji.

---

## Czesc VII: "Twardosc" wymagan — hard RT, firm RT, soft RT

### 7.1 Trzy poziomy

| Poziom | Definicja | Przykład |
|--------|-----------|----------|
| **Hard RT** | Missed deadline = błąd bezpieczeństwa | Aktualizacja PWM, safety |
| **Firm RT** | Missed deadline = degradacja jakości | Filtracja diagnostyczna |
| **Soft RT** | Missed deadline = funkcja biznesowa | Telemetria, logowanie |

### 7.2 Dlaczego to jest ważne

To pomaga decydować:
- Które wątki dostają priorytet RT
- Co idzie osobnym procesem
- Gdzie dajesz watchdog i jaką reakcję

---

## Czesc VIII: Co logowac — minimum diagnostyczne

**Bez telemetrii debugujesz "na ślepo".**

Minimalny zestaw sygnałów:
```c
struct TelemetrySample {
    uint64_t t_start;           // Czas startu iteracji
    uint64_t t_end;             // Czas końca
    uint32_t rt_loop_us;        // Czas iteracji
    
    float omega_set;            // Prędkość zadana
    float omega_meas;           // Prędkość mierzona
    float omega_err;            // Błąd regulacji
    float u_cmd;               // Sterowanie (moment)
    
    uint8_t  saturation;       // Czy była saturacja
    uint8_t  miss_deadline;    // Czy deadline przekroczony
    uint8_t  comm_status;      // Status komunikacji
    
    float vib_rms;             // Wibracje RMS
    float temperature;         // Temperatura
};
```

---

## Czesc IX: Pulapki wdrożeniowe

### 9.1 "Wrzućmy wszystko na Linuxa"

Bez izolacji rdzeni i priorytetów → jitter 1-50 ms.

### 9.2 "EtherCAT najszybciej jak się da"

Zbyt mały cykl → CPU nie wyrabia → jitter rośnie.

### 9.3 "Dodajmy filtr"

Filtr zwiększa opóźnienie → może zjeść margines fazy → niestabilność.

### 9.4 "Telemetria w tej samej pętli"

Klasyczny sposób na sporadyczne oscylacje.

---

## Czesc X: Podsumowanie i checklisty

### Checklisty:

- [ ] Zdefiniuj pętle i ich częstotliwości (prąd / prędkość / proces)
- [ ] Zmapuj krytyczną ścieżkę end-to-end i jej budżet czasu
- [ ] Oddziel komunikację krytyczną od telemetrii
- [ ] Zidentyfikuj rezonanse i zaplanuj strategię tłumienia

### Zasady architektoniczne:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Najpierw krytyczna ścieżka | Rysujesz łańcuch sygnałowy |
| Potem podział warstw | Drive vs master |
| Na końcu technologia | RTOS vs Linux, EtherCAT vs inne |

---

## Czesc XI: Pytania do dyskusji

1. Jakie są 3 najważniejsze źródła opóźnień end-to-end w Twojej architekturze i jak je zmierzysz?
2. Które pętle umieścisz w napędzie, a które w masterze, i co jest Twoim kryterium decyzji?
3. Co w Twoim systemie jest hard RT, co firm RT, a co soft RT?
4. Jak odróżnisz problem rezonansu mechanicznego od problemu jitteru czasowego?

---

## Czesc XII: Zadania praktyczne

### Zadanie 1: Architektura blokowa

Narysuj architekturę dla przykładowej maszyny wirującej:
- Zidentyfikuj warstwy
- Pokaż przepływ danych
- Zaznacz krytyczną ścieżkę

### Zadanie 2: Podział pętli

Zdefiniuj 3 pętle (prąd, prędkość, proces) i opisz:
- Gdzie je umieszczasz (drive vs master)
- Częstotliwość
- Budżet czasu

### Zadanie 3: Telemetria

Wypisz 5 sygnałów do logowania i uzasadnij dlaczego.

---

## BONUS: Jedna metryka

Jeśli masz tylko jedną metrykę do ciągłego monitorowania, wybierz **p99/p99.9 czasu iteracji pętli + licznik missed deadlines**.

To najszybciej demaskuje "niewidzialne" problemy architektury.

---

*(Koniec wykladu 1)*
