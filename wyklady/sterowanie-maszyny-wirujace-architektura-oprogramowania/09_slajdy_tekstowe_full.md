# Wyklad 9: Podsumowanie — slajdy tekstowe

## Slajd 1: O co chodzi

**Sterowanie w czasie rzeczywistym — trzy zasady:**

- **Jeden system, wiele warstw** — podział odpowiedzialności
- **Różne wymagania czasowe** — hard RT vs firm RT vs soft RT
- **RT psuje się przez jitter i blokady** — nie przez brak mocy CPU

> "W sterowaniu RT przegrywasz przez ogony opóźnień i błędy architektury, a nie przez brak mocy CPU."

---

## Slajd 2: Warstwy — mapowanie odpowiedzialnosci

```
Warstwa 4: modelowanie/symulacja (offline)
    |
    |  Kontrakt: dane historyczne, scenariusze, parametry
    v
Warstwa 3: nadzor/HMI (soft-RT)
    |
    |  Kontrakt: setpointy, stany, zdarzenia
    v
Warstwa 2: kontroler RT / master (firm-RT)
    |
    |  Kontrakt: probki pomiarowe, komendy, statusy
    v
Warstwa 1: drive/MCU (hard-RT)
    |
    |  Kontrakt: prad, moment, stany bezpieczenstwa
    v
Warstwa 0: sprzet (opcjonalnie FPGA)
```

| Warstwa | Odpowiedzialnosc | Przyklad |
|---------|------------------|----------|
| W1 | "Trzymam prad/moment i reaguję bezpiecznie" | PWM, ADC, safety |
| W2 | "Koordynuje cykl, liczę regulacje, trzymam timing" | EtherCAT master, PID |
| W3 | "Wizualizuje, loguje, alarmuje, nie psuje RT" | GUI, bazy, API |
| W4 | "Projektuje i waliduje algorytmy offline" | MATLAB, Python, symulacja |

---

## Slajd 3: Kontekst 2035

**Trend: Komórki autonomiczne**

W 2035 typowe jest:
- Roboty + moduły procesu + diagnostyka = jedna komórka autonomiczna
- Wygrywa deterministyka, safety i diagnozowalnosc
- Każdy moduł ma własny FSM i watchdog

**Co to oznacza dla architektury:**
- Hard RT w warstwie wykonawczej
- Safety jako osobny kanał decyzyjny
- Predykcyjna diagnostyka (ale poza pętlą RT)
- Audit trail i compliance

---

## Slajd 4: EtherCAT w jednym zdaniu

**EtherCAT to narzędzie do cyklicznej wymiany danych sterujących i pomiarowych.**

Kluczowe pojęcia:
- **Cykl** — regularna wymiana danych (100 μs do 10 ms)
- **Distributed Clocks (DC)** — synchronizacja czasu między slave'ami (gdy liczy się faza)
- **PDO** — Process Data Object (dane wymieniane w cyklu)
- **SDO** — Service Data Object (dane konfiguracyjne, acykliczne)

**Zasada:**
- Cykliczne = minimum potrzebne do stabilności i safety
- Acykliczne = rate-limited i odseparowane

---

## Slajd 5: RTOS vs Linux RT

| Kryterium | RTOS (MCU) | Linux PREEMPT_RT |
|-----------|------------|------------------|
| Jitter | < 1 μs | 10-100 μs |
| Cykl | do 100 kHz | do 10 kHz |
| Ekosystem | Ograniczony | Pełny |
| Debugowanie | Trudniejsze | Łatwiejsze |

**Wybór:**
- RTOS: gdy potrzebujesz mikrosekundowego jitteru, prostego runtime
- Linux PREEMPT_RT: gdy potrzebujesz ekosystemu, sieci, narzędzi

**Klucz do Linux RT:**
- SCHED_FIFO + pinning + mlockall + kontrola IRQ
- Bez tego = losowy system z jitterem

---

## Slajd 6: Watek RT — wzorzec

```
Wzorzec implementacyjny:

1. Czekaj na tick (clock_nanosleep)
2. Odczytaj wejścia (z bufora, nie blokująco)
3. Oblicz regulator (PID, FOC)
4. Zapisz wyjscia (do bufora, nie blokująco)
5. Opublikuj telemetrię (do ring buffer)

Zasady:
- ZERO IO
- ZERO alokacji
- ZERO mutexow
```

---

## Slajd 7: IPC RT->nonRT

**Shared memory + lock-free ring buffer (SPSC)**

Struktura:
- Producer: wątek RT (zapisuje)
- Consumer: logger/GUI (czyta)
- Brak mutexów = brak blokad

**Polityka przepełnienia:**
- **Drop** — gubisz próbki (dla logów)
- **Overwrite** — nadpisujesz najstarsze (dla "live" danych)

**Kluczowe metryki:**
- Licznik dropów
- p99.9 czasu odczytu

---

## Slajd 8: Kontenery

**Gdzie TAK:**
- HMI, GUI
- Bazy danych
- API, serwisy
- Modelowanie, symulacja

**Gdzie NIE:**
- Petla hard-RT
- Kod z wymaganiem determinizmu

**Zasada:**
Kontener nie powinien owijać wątku hard-RT. Możesz konteneryzować nadzór i tooling, ale nie sterowanie.

---

## Slajd 9: Roadmap implementacji

```
KROK 1: Model ODE w Python/MATLAB
    ↓
KROK 2: Projekt regulatora (PI/PID) + saturacje + anti-windup
    ↓
KROK 3: Prototyp W2 na Linux PREEMPT_RT (wątek SCHED_FIFO)
    ↓
KROK 4: Integracja EtherCAT master + slave
    ↓
KROK 5: Dodanie W3 (nadzor) i IPC (shared mem + ring)
    ↓
KROK 6: Digital twin (porównanie model vs rzeczywistość)
    ↓
KROK 7: Safety (watchdog, safe stop, fault injection)
```

**Każdy krok ma definition of done!**

---

## Slajd 10: Safety — podsumowanie

**Trzy filary:**

1. **Watchdog wielopoziomowy**
   - Slave watchdog (brak komend → safe state)
   - Master watchdog (brak odpowiedzi → safe stop)
   - Application watchdog (brak iteracji → fault)

2. **FSM bezpieczeństwa**
   - NORMAL → WARNING → DEGRADED → SAFE_STOP → FAULT
   - Przejścia do bezpieczniejszych stanów = natychmiast
   - Powrót = po warunkach + histerezie

3. **Fault injection jako standard**
   - Testuj awarie zanim wystąpią w produkcji
   - Regression testy = obowiązkowe

---

## Slajd 11: Integracja robot + modul procesu

**Architektura:**

- Robot i moduł mają osobne FSM
- Jest nadrzędny orkiestrator (workflow) który spina stany
- Krytyczne reakcje (safe stop) są lokalne w każdym module

**Minimalny kontrakt:**

```c
// Stan modułu
enum ModuleState {
    MODULE_FAULT,
    MODULE_SAFE_STOP,
    MODULE_READY,
    MODULE_RUNNING
};

// Stan robota
enum RobotState {
    ROBOT_IDLE,
    ROBOT_MOVING,
    ROBOT_INTERLOCK,
    ROBOT_FAULT
};

// Zdarzenia
enum Event {
    EVENT_NONE,
    EVENT_ALARM,
    EVENT_COMPLETE,
    EVENT_INTERLOCK,
    EVENT_E_STOP,
    EVENT_FAULT
};
```

---

## Slajd 12: Diagnostyka predykcyjna

**Baseline per tryb pracy:**
- Zbieraj metryki podczas normalnej pracy
- Buduj profil "normalności"

**Trend:**
- WARNING → DEGRADED → SAFE_STOP
- Progi: 80% limitu → 90% limitu → 100% limitu

**Zasada:**
Diagnostyka musi być asynchroniczna (nie psuje RT)
- Rate-limited
- Osobny wątek
- Wyniki do planowania/nadzoru, nie do sterowania

---

## Slajd 13: Etyka i odpowiedzialnosc

**Zasady:**

1. **Safety jako osobny kanał decyzyjny**
   - Działa lokalnie
   - Nie zależy od sieci, chmury, GUI

2. **ML poza krytyczną ścieżką RT**
   - Timeout, fallback, degradacja
   - Wynik do planowania, nie do pętli momentu

3. **Logi i audit trail**
   - Każde zdarzenie zalogowane
   - Można odtworzyć historię

4. **Testy awaryjne jako standard**
   - Fault injection w regresji
   - Udowodnione zachowanie w awarii

---

## Slajd 14: Pojecia-klucze do zapamietania

| Pojęcie | Definicja |
|---------|-----------|
| T (cykl) | Okres pętli sterowania |
| Deadline | Maksymalny czas na iterację |
| WCET | Worst-Case Execution Time (kod) |
| WCRT | Worst-Case Response Time (kod + czekanie) |
| Latency | Opóźnienie end-to-end |
| Jitter | Zmiennosc latency |
| SPSC | Single Producer Single Consumer |
| DC | Distributed Clocks |

---

## Slajd 15: Najczestsze bledy (TOP 5)

1. **"Szybszy cykl = lepiej"** — bez pomiaru ogonów
2. **Mutex w pętli RT** — priority inversion
3. **GC w wątku RT** — losowe pauzy
4. **Alokacja w pętli** — page fault
5. **Brak telemetrii** — debugowanie w ciemno

---

## Slajd 16: BONUS — zasady-architektoniczne

> **Najpierw narysuj krytyczną ścieżkę!**

```
sensor -> timestamp/bufor -> filtr/estymacja -> regulator -> transport -> drive -> plant
```

> **W RT przegrywasz przez ogony opóźnień, nie przez brak mocy CPU.**

> **Linux RT jest ok dla mastera, ale tylko gdy stosujesz dyscyplinę.**

> **Lock-free to nie fancy — to najprostsza droga do przewidywalności.**

> **Etyka zaczyna się od dowodu, nie deklaracji.**

---

*(Koniec wykladu 9)*
