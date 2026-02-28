# Wykład 6: Bezpieczeństwo funkcjonalne (SIL) i reakcje awaryjne

## Czesc I: Wstep teoretyczny — czym jest safety

### 1.1 Geneza — dlaczego safety jest osobnym kanałem

Proszę wyobrazić sobie sytuację: wirówka laboratoryjna pracuje 15 000 RPM. Nagle:

- **Master się zawiesza** — co robi napęd?
- **Komunikacja EtherCAT pada** — co robi system?
- **Czujnik prędkości pokazuje głupoty** — co robi sterowanie?
- **Temperatura rośnie do 100°C** — co robi system?

**Kluczowe pytanie:** Czy te reakcje zależą od "normalnego" sterowania?

**Odpowiedź brzmi: NIE!**

Bezpieczeństwo musi działać **niezależnie** od tego, czy:
- Master jest żywy
- Komunikacja działa
- Dane są sensowne
- Ktoś patrzy na HMI

### 1.2 Zasada podstawowa

Profesor zawsze powtarza: **"Safety to osobny kanał decyzyjny."**

**Co to oznacza?**

```
Kanał funkcjonalny:    Kanał safety:
                       
Regulator PID    →     Watchdog
     |                    |
     v                    v
Sterowanie        →     Limity
     |                    |
     v                    v
Telemetria        →     Safe Stop
```

**Te dwa kanały muszą być niezależne!**

### 1.3 Co znaczy "SIL" w praktyce

Bezpieczeństwo funkcjonalne (SIL) odpowiada na pytanie:
> "Co system zrobi, gdy coś pójdzie źle?"

**Niezależnie od poziomu formalnej certyfikacji, praktycznie:**

1. **Identyfikacja zagrożeń** — co może się zepsuć?
2. **Funkcje bezpieczeństwa** — co ma się zadziać?
3. **Niezależność kanału** — safety nie zależy od funkcjonalnego
4. **Testowalność** — czy umiesz zweryfikować w runtime?

---

## Czesc II: Typowe zdarzenia awaryjne

### 2.1 Klasy awarii

| Awaria | Opis | Potencjalny skutek |
|--------|------|---------------------|
| Przegrzanie | Temp. > limit | Pożar, uszkodzenie łożysk |
| Przekroczenie prądu | Prąd > limit | Uszkodzenie napędu |
| Utrata sensora | Brak danych z enkodera | Utrata kontroli |
| Utrata synchronizacji | DC się rozjeżdża | Niespójne dane |
| Missed deadline | Pętla nie zdążyła | Niestabilność |
| Błąd komunikacji | EtherCAT timeout | Utrata sterowania |

### 2.2 Reakcje muszą być zdefiniowane

**Dla każdej awarii musisz mieć zdefiniowane:**

1. **Detekcja** — jak wykrywamy?
2. **Reakcja** — co robimy?
3. **Warunki powrotu** — kiedy wracamy do normalnej pracy?

---

## Czesc III: Warstwy zabezpieczen

### 3.1 Poziom 1: Sprzętowe limity

```
Najniższa warstwa — w samym napędzie:

- Bezpiecznik (prąd)
- Thermal cutoff (temperatura)
- Hardware watchdog
```

### 3.2 Poziom 2: Watchdog w napędzie

```c
// Watchdog w napędzie
void watchdog_check() {
    static uint32_t last_heartbeat = 0;
    uint32_t now = get_tick_ms();
    
    // Master wysyła heartbeat co cykl
    if (now - last_heartbeat > WATCHDOG_TIMEOUT_MS) {
        // Brak komunikacji → safe state
        set_pwm(0);           // Wyłącz PWM natychmiast
        engage_brake();       // Załącz hamulec
        set_fault(FAULT_WD); // Ustaw flagę błędu
    }
}
```

### 3.3 Poziom 3: Watchdog w kontrolerze

```c
// Watchdog w master
void master_watchdog() {
    // Sprawdź każdego slave
    for (int i = 0; i < num_slaves; i++) {
        if (slave[i].timeout_counter > MAX_TIMEOUT) {
            // Slave nie odpowiada
            slave[i].state = SLAVE_FAULT;
            
            if (critical_slave(i)) {
                trigger_safe_stop();  // Natychmiast!
            } else {
                trigger_degradation(); // Ogranicz osiągi
            }
        }
    }
}
```

### 3.4 Poziom 4: Logika awaryjna

```c
// Logika awaryjna
void emergency_logic() {
    if (overtemperature) {
        // Ogranicz moc, potem stop
        reduce_power(50);  // Zmniejsz o 50%
        if (temp > TEMP_CRITICAL) {
            safe_stop();
        }
    }
    
    if (overspeed) {
        // Natychmiastowy stop
        safe_stop();
    }
}
```

---

## Czesc IV: Safe stop — rozne strategie

### 4.1 Dwie podstawowe strategie

| Strategia | Opis | Zalety | Wady |
|-----------|------|--------|-------|
| **Odcięcie momentu** | Natychmiast PWM = 0 | Proste, szybkie | Obiekt "pójdzie własną drogą" |
| **Kontrolowane hamowanie** | Stop według sekwencji | Bezpieczniejsze | Wymaga sprawnych napędów |

### 4.2 Kiedy ktora strategia

**Odcięcie momentu:**
- Gdy obiekt ma małą energię kinetyczną
- Gdy hamowanie może być niebezpieczne (np. ruchomy podzespoł)
- Gdy potrzebujesz absolutnej pewności

**Kontrolowane hamowanie:**
- Gdy obiekt ma dużą energię (wirnik 15k RPM!)
- Gdy nagłe zatrzymanie może uszkodzić mechanikę
- Gdy masz redundancję pomiarów

### 4.3 Zasada

> "Odcięcie" bywa niebezpieczne, jeśli obiekt ma dużą energię kinetyczną.
> "Kontrolowane hamowanie" wymaga sprawnych napędów i pomiarów.

**Projektujesz tryb awaryjny tak, by minimalizować ryzyko!**

---

## Czesc V: FSM bezpieczenstwa

### 5.1 Stany

```c
typedef enum {
    STATE_NORMAL,      // Normalna praca
    STATE_WARNING,    // Ostrzeżenie
    STATE_DEGRADED,   // Ograniczony tryb
    STATE_SAFE_STOP,  // Bezpieczny stop
    STATE_FAULT,      // Awaria
    STATE_RECOVERY    // Powrót do pracy
} SafetyState;
```

### 5.2 Przejscia

```
           +----------+
           |  NORMAL  |
           +----------+
                |
    (warning) | (degraded)
                v
           +-----------+
           | WARNING   |
           +-----------+
                |
   (powrot) | (degraded)
                v
           +----------+   (fault)   +-----------+
           | DEGRADED | ---------> |   FAULT   |
           +----------+             +-----------+
                |                           |
   (safe stop)| (fault)                    |
                v                           |
           +------------+                    |
           | SAFE_STOP  |                    |
           +------------+                    |
                |                           |
   (recovery) | (reset)                    |
                v                           |
           +------------+                   |
           | RECOVERY   |------------------+
           +------------+
                |
        (powrot do normal)
                v
           +----------+
           |  NORMAL  |
           +----------+
```

### 5.3 Zasady przejść

1. **Przejścia do bezpieczniejszych stanów = natychmiastowe**
2. **Powrót wymaga warunków i histerezy**

```c
void safety_update() {
    switch (current_state) {
        case STATE_NORMAL:
            if (any_fault()) {
                transition_to(STATE_SAFE_STOP);  // Natychmiast!
            } else if (any_warning()) {
                transition_to(STATE_WARNING);
            }
            break;
            
        case STATE_WARNING:
            if (any_fault()) {
                transition_to(STATE_SAFE_STOP);
            } else if (!any_warning()) {
                // Histereza: musi być OK przez 10s
                if (ok_duration > 10000) {
                    transition_to(STATE_NORMAL);
                }
            }
            break;
    }
}
```

---

## Czesc VI: Separacja kanalu funkcjonalnego i bezpieczenstwa

### 6.1 Kluczowa zasada

To jest klucz do odporności:

```
Kanał funkcjonalny:     Kanał bezpieczeństwa:
                         
Regulacja      →        Proste reguły
Optymalizacja  →        Watchdogi
Telemetria     →        Limity
                →        Safe Stop
```

### 6.2 Co to oznacza praktycznie

| Element | Kanał funkcjonalny | Kanał safety |
|---------|-------------------|--------------|
| Regulator PID | Tak | Nie |
| Telemetria | Tak | Nie (nie czekaj na dane) |
| HMI | Tak | Nie |
| ML/AI | Tak | Nie |
| Limity prądu | Niezależnie | Tak |
| Watchdog | Niezależnie | Tak |
| Safe stop | Niezależnie | Tak |

### 6.3 Zasady separacji

- **Niezależne limity w napędzie** (sprzętowo/firmware)
- **Watchdogi wielopoziomowe** (slave + master)
- **Minimalne zależności** (safety nie czeka na "ładne dane")

---

## Czesc VII: Wstrzykiwanie bledow — jak testowac safety

### 7.1 Dlaczego testujemy awarie

Bo **nie można czekać na awarię w produkcji** żeby sprawdzić, czy system działa!

### 7.2 Typowe testy

| Test | Co symulujesz | Narzędzie |
|------|---------------|-----------|
| Dropout komunikacji | Brak odpowiedzi od slave | iptables, tc |
| Opóźnienie | Opóźnienie komunikacji | tc netem |
| Missed deadline | Pętla się nie wykonuje | Symulacja w kodzie |
| Brak sensora | Sensor przestaje działać | Symulacja w kodzie |
| Zamrożony pomiar | Sensor zwraca stałe wartości | Symulacja w kodzie |
| Przegrzanie | Temperatura rośnie | Symulacja w kodzie |
| Saturacja | Prąd/moment = limit | Symulacja w kodzie |

### 7.3 Kryteria testów

Test musi sprawdzać nie tylko, że "zatrzymało":

| Kryterium | Opis |
|-----------|------|
| Czas reakcji | Jak szybko system wykrywa awarię? |
| Stan końcowy | Czy system jest w znanym, bezpiecznym stanie? |
| Flapping | Czy nie przełącza się ciągle? |
| Recovery | Czy może wrócić do normalnej pracy? |
| Logowanie | Czy awaria jest zalogowana? |

---

## Czesc VIII: Checklisty

### Checklisty:

- [ ] Każda klasa awarii ma przypisaną reakcję i warunki powrotu
- [ ] Watchdogi są wielopoziomowe (napęd + kontroler)
- [ ] Bezpieczeństwo nie zależy od telemetrii i "miękkich" usług
- [ ] Testujesz awarie przez wstrzykiwanie błędów

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Safety jest niezależny | Działa gdy "główny" system pada |
| Proste reguły | Złożona logika = błędy |
| Testuj awarie | Wstrzykiwanie błędów jako standard |
| Histereza | Unikaj flappingu |

---

## Czesc IX: Pytania do dyskusji

1. Dlaczego kanał bezpieczeństwa musi działać bez "smart" modułów?
2. Kiedy "odcięcie momentu" może być gorsze niż kontrolowane hamowanie?
3. Jak zapewnisz, że powrót z SAFE_STOP do NORMAL nie spowoduje flappingu?
4. Jakie testy fault-injection uznasz za obowiązkowe w każdym wydaniu?

---

## Czesc X: Zadania praktyczne

### Zadanie 1: Klasy awarii

Zdefiniuj 6 klas awarii i dla każdej:
- Detekcja
- Reakcja
- Warunki powrotu

### Zadanie 2: FSM bezpieczeństwa

Narysuj FSM bezpieczeństwa z co najmniej 5 stanami:
- NORMAL
- WARNING
- DEGRADED
- SAFE_STOP
- RECOVERY

### Zadanie 3: Plan testów fault-injection

Ułóż plan testów i kryteria "pass/fail":
- Czas reakcji
- Brak flappingu
- Kontrolowany powrót

---

## BONUS: Jesli nie da sie wyjasnic

Jeśli nie da się łatwo opisać reakcji na awarię w jednym zdaniu i jednej tabeli — logika safety jest prawdopodobnie zbyt złożona i powinna zostać uproszczona.

Proste rzeczy są testowalne. Złożone rzeczy — nie.

---

*(Koniec wykladu 6)*
