# Wyklad 8: Safety (programistycznie) — watchdog, degradacja, safe stop

## Czesc I: Wstep teoretyczny — czym jest safety w systemach sterowania

### 1.1 Geneza problemu

Prosze wyobrazic sobie wirówkę laboratoryjną pracującą 10 000 RPM. Nagle:

- **Scenariusz A:** Przepięcie sieciowe — sterowanie traci dane
- **Scenariusz B:** Czujnik temperatury pokazuje 200°C (awaria)
- **Scenariusz C:** Wirnik osiąga krytyczną prędkość rezonansową
- **Scenariusz D:** Operator naciska E-stop

Co powinien zrobić system w każdym z tych przypadków?

**Odpowiedź:** Musi mieć zdefiniowane zachowanie. I to niezależnie od tego, czy "reszta systemu działa".

### 1.2 Dlaczego safety jest osobnym kanałem

Profesor zawsze powtarza fundamentalną zasadę:

> **Safety musi działać, gdy system jest w najgorszym stanie: brak danych, zawieszenie, dropouty, wysoka energia kinetyczna.**

To jest kluczowe! Safety nie może zależeć od:
- Komunikacji z nadrzędnym systemem
- Bazy danych
- Sieci
- GUI
- Jakiejkolwiek "sprytnej" logiki

Safety musi działać **lokalnie**, w samej warstwie wykonawczej.

### 1.3 Zasada " Defense in Depth"

System bezpieczeństwa to nie jeden element — to wiele warstw obrony:

```
Poziom 1: Sprzętowe limity
    - Bezpiecznik
    - Thermal cutoff
    - Hardware watchdog
    
Poziom 2: Firmware (W1)
    - Watchdog CPU
    - Sprawdzenie RAM
    - Limity prądu/napięcia
    
Poziom 3: Aplikacja (W2)
    - FSM bezpieczeństwa
    - Watchdog komunikacji
    - Limity programowe
    
Poziom 4: Orkiestrator (W3)
    - Workflow safety
    - Audit trail
```

Każdy poziom ma inną rolę. I każdy może zadziałać niezależnie.

---

## Czesc II: Klasy awarii

### 2.1 Typowe awarie w systemach sterowania

| Awaria | Opis | Reakcja |
|--------|------|---------|
| Missed deadline | Petla RT nie zdążyła w czasie | Safe stop lub degradacja |
| Dropout komunikacji | Brak odpowiedzi od slave/master | Safe stop |
| Utrata sensora | Sensor nie zwraca danych | Safe stop lub degradacja |
| "Zamrożony" pomiar | Sensor zwraca stare dane | Safe stop |
| Przegrzanie | Temperatura > limit | Degradacja (zmniejsz moc) |
| Saturacja prądu | Prąd > limit | Degradacja (zmniejsz moc) |
| Przekroczenie prędkości | Omega > limit | Natychmiast stop |

### 2.2 Drzewo awarii

```
                   +------------------+
                   |   System OK      |
                   +------------------+
                         |
       +-----------------+-----------------+
       |                 |                 |
+------v------+  +-------v-------+  +------v------+
| Deadline   |  |  Comm         |  | Sensor      |
| missed     |  |  dropout      |  | fault       |
+------------+  +--------------+  +-------------+
       |                 |                 |
       v                 v                 v
+------------+  +--------------+  +-------------+
| Safe Stop  |  | Safe Stop    |  | Safe Stop   |
| (natychmiast)| | (po czasie) |  | (po czasie) |
+------------+  +--------------+  +-------------+
```

---

## Czesc III: FSM bezpieczenstwa

### 3.1 Stany

```c
typedef enum {
    STATE_NORMAL,      // Normalna praca
    STATE_WARNING,      // Ostrzeżenie (zbliżanie się do limitu)
    STATE_DEGRADED,    // Ograniczony tryb pracy
    STATE_SAFE_STOP,   // Bezpieczny stop (hamulce załączone)
    STATE_FAULT,       // Awaria (wymaga interwencji)
    STATE_RECOVERY     // Powrót do pracy normalnej
} SafetyState;
```

### 3.2 Przejscia

```
           +----------+
           |  NORMAL  |
           +----------+
                |
    (warning) | (degraded)
                v
           +-----------+
           | WARNING   | <------------------+
           +-----------+                    |
                |                           |
   (powrot)   | (degraded)                 |
                v                           |
           +----------+   (fault)    +-----------+
           | DEGRADED | -----------> |   FAULT   |
           +----------+              +-----------+
                |                           |
    (safe stop)| (fault)                    |
                v                           |
           +------------+                  |
           | SAFE_STOP  |                  |
           +------------+                  |
                |                           |
    (recovery) | (reset)                   |
                v                           |
           +------------+                  |
           | RECOVERY   |------------------+
           +------------+
                |
        (powrot do normal)
                |
                v
           +----------+
           |  NORMAL  |
           +----------+
```

### 3.3 Zasady przejść

**Zasada 1:** Przejścia do bezpieczniejszych stanów są łatwe i natychmiastowe
- NORMAL → WARNING: natychmiast
- NORMAL → DEGRADED: natychmiast
- NORMAL → SAFE_STOP: natychmiast

**Zasada 2:** Powrót wymaga warunków i histerezy
- WARNING → NORMAL: po 10 sekundach bez ostrzeżeń
- DEGRADED → NORMAL: po 30 sekundach bez problemów
- SAFE_STOP → RECOVERY: po ręcznym potwierdzeniu

```c
// Implementacja z histerezą
void safety_update() {
    // Sprawdź warunki
    bool temp_warning = temperature > TEMP_WARNING;
    bool temp_fault = temperature > TEMP_FAULT;
    
    // Logika przejść
    switch (current_state) {
        case STATE_NORMAL:
            if (temp_fault) {
                transition_to(STATE_SAFE_STOP);
            } else if (temp_warning) {
                transition_to(STATE_WARNING);
            }
            break;
            
        case STATE_WARNING:
            if (temp_fault) {
                transition_to(STATE_SAFE_STOP);
            } else if (!temp_warning) {
                // Histereza: musi być OK przez 10s
                if (warning_duration > 10000) {
                    transition_to(STATE_NORMAL);
                }
            } else {
                warning_duration += CYCLE_TIME;
            }
            break;
    }
}
```

---

## Czesc IV: Watchdog wielopoziomowy

### 4.1 Poziom 1: Watchdog w drive/slave

```c
// Wewnątrz slave (firmware)
void watchdog_check() {
    static uint32_t last_heartbeat = 0;
    uint32_t now = get_tick_ms();
    
    // Master wysyła heartbeat co cykl
    if (now - last_heartbeat > WATCHDOG_TIMEOUT_MS) {
        // Brak komunikacji - przejdź do safe state
        set_pwm(0);           // Wyłącz PWM
        engage_brake();       // Załącz hamulec
        set_fault(FAULT_WD);  // Ustaw flagę błędu
    }
}
```

### 4.2 Poziom 2: Watchdog w master

```c
// W master (W2)
void master_watchdog() {
    // Sprawdź każdego slave
    for (int i = 0; i < num_slaves; i++) {
        if (slave[i].timeout_counter > MAX_TIMEOUT) {
            // Slave nie odpowiada - degradacja
            slave[i].state = SLAVE_FAULT;
            
            // Reakcja systemu
            if (critical_slave(i)) {
                trigger_safe_stop();
            } else {
                trigger_degradation();
            }
        }
    }
    
    // Sprawdź własną pętlę
    if (missed_deadline_count > MAX_MISSED) {
        trigger_safe_stop();
    }
}
```

### 4.3 Poziom 3: Application watchdog

```c
// Warstwa aplikacji
void application_watchdog() {
    // Sprawdź czy pętla RT działa
    if (!rt_thread_alive) {
        trigger_safe_stop();
    }
    
    // Sprawdź czy dane są świeże
    if (sensor_data_age > MAX_AGE) {
        trigger_safe_stop();
    }
    
    // Sprawdź stan FSM
    if (fsm_state == STATE_INVALID) {
        trigger_fault();
    }
}
```

### 4.4 Tabela watchdog

| Poziom | Gdzie | Co sprawdza | Timeout | Reakcja |
|--------|-------|-------------|---------|---------|
| 1 | Slave (W1) | Brak komend od master | 1-10 ms | Safe state (hamulce) |
| 2 | Master (W2) | Brak odpowiedzi od slave | 10-100 ms | Degradacja/safe stop |
| 3 | Aplikacja (W2) | Brak iteracji petli | 100-1000 ms | Safe stop |
| 4 | Orkiestrator (W3) | Brak heartbeat | 1-10 s | Alarm, logowanie |

---

## Czesc V: Fault injection jako standard testow

### 5.1 Dlaczego testujemy awarie

Bo **nie można czekać na awarię w produkcji** żeby sprawdzić, czy system działa!

### 5.2 Rodzaje fault injection

| Typ | Co symulujemy | Narzędzie |
|-----|---------------|-----------|
| Network dropout | Utrata pakietów | iptables, tc |
| Delay | Opóźnienie sieci | tc netem |
| CPU overload | Wysokie obciążenie | stress-ng |
| Memory fault | Błędy RAM | mmap /dev/mem |
| Sensor freeze | Zamrożone dane | Injection w kodzie |
| Temperature | Przegrzanie | Symulacja w kodzie |

### 5.3 Przykladowe testy

```python
# Test dropout komunikacji
def test_communication_dropout():
    # Włącz 10% dropout
    os.system("iptables -A INPUT -m statistic --mode random --probability 0.1 -j DROP")
    
    # Uruchom system na 60 sekund
    system.start()
    time.sleep(60)
    system.stop()
    
    # Sprawdź metryki
    assert system.missed_deadline < 10
    assert system.safety_state == "NORMAL"  # lub DEGRADED, ale nie FAULT
    assert system.recovery_time < 1.0  # sekundy
    
    # Wyłącz dropout
    os.system("iptables -F")
```

```python
# Test CPU overload
def test_cpu_overload():
    # Uruchom obciążenie 80% CPU
    subprocess.Popen(["stress-ng", "--cpu=8", "--cpu-load=80"])
    
    # Uruchom system
    system.start()
    time.sleep(60)
    system.stop()
    
    # Sprawdź metryki
    assert system.p99_loop_time < 500  # us
    assert system.missed_deadline_pct < 0.1  # %
```

### 5.4 Kryteria testow

| Kryterium | Opis |
|-----------|------|
| Czas reakcji | Jak szybko system wykrywa awarię |
| Stan końcowy | Czy system jest w znanym, bezpiecznym stanie |
| Recovery | Czy system może wrócić do pracy |
| Logowanie | Czy awaria jest zalogowana |

---

## Czesc VI: Etyka i odpowiedzialnosc

### 6.1 Etyka w systemach krytycznych

Profesor chciałby, żebyście zapamiętali:

> **Etyka to nie jest dodatek — to podstawa projektowania systemów krytycznych.**

### 6.2 Zasady etyczne

1. **Projektuj tak, by awarie były przewidywalne i bezpieczne**
   - Każdy tryb awarii ma zdefiniowaną reakcję
   - System nigdy nie może wejść w stan "niebezpieczny"

2. **Unikaj ukrytych zależności**
   - Safety nie zależy od chmury
   - Safety nie zależy od sieci
   - Safety nie zależy od bazy danych

3. **Transparentność decyzji**
   - Logi, stany, przyczyny
   - Każdy przejść FSM jest zalogowany
   - Można odtworzyć historię

4. **Procedury walidacji**
   - Fault injection jako standard
   - Regresja testów awaryjnych
   - Audyt bezpieczeństwa

### 6.3 Pytania do review

Przed oddaniem systemu zadaj sobie:
- Co się stanie, gdy zniknie sensor?
- Co się stanie, gdy master przestanie odpowiadać?
- Czy system może wejść w stan niebezpieczny przez "ładny" dashboard?
- Czy logi pozwalają odtworzyć awarię?

---

## Czesc VII: Podsumowanie i checklisty

### Checklisty:

- [ ] Każda klasa awarii ma: detekcja → reakcja → warunki powrotu
- [ ] Watchdog jest testowany automatycznie (regresja)
- [ ] FSM bezpieczeństwa jest zdefiniowany i testowalny
- [ ] Logi pozwalają odtworzyć każdą awarię

### Zasady safety:

| Zasada | Wyjasnienie |
|--------|-------------|
| Lokalność | Safety działa lokalnie, bez zależności |
| Histereza | Powrót wymaga warunków |
| Defense in depth | Wiele warstw obrony |
| Testowanie | Fault injection jako standard |

---

## Czesc VIII: Pytania do dyskusji

1. Jakie są minimalne stany FSM bezpieczeństwa i jakie warunki powrotu są krytyczne?
2. Jak projektujesz watchdog tak, by działał nawet przy awarii komunikacji i przeciążeniu CPU?
3. Jakie testy fault injection muszą być w regresji (zawsze) i dlaczego?
4. Jak ograniczasz ryzyko, że "sprytna" logika (np. ML) wejdzie do safety?

---

## Czesc IX: Zadania praktyczne

### Zadanie 1: Safety FSM + logs

Zaimplementuj FSM bezpieczeństwa + logowanie przyczyn przejść.

### Zadanie 2: Fault Injection Suite

Zestaw testów:
- Dropout
- Opóźnienie
- Zamrożony sensor
- Przeciążenie CPU

### Zadanie 3: Release Gate

Pipeline blokujący release bez przejścia testów awaryjnych.

---

## BONUS: Etyka zaczyna się od dowodu

Etyka w systemach krytycznych zaczyna się od tego, że potrafisz powiedzieć: "jak system zachowa się w awarii" i **udowodnić to testem**, a nie deklaracją.

---

*(Koniec wykladu 8)*
