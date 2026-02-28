# Wyklad 2: EtherCAT master i timing (cykl, DC, watchdog)

## Czesc I: Wstep teoretyczny — czym jest EtherCAT i dlaczego jest taki specjalny

### 1.1 Geneza problemu komunikacyjnego

Prosze wyobrazic sobie sytuacje: budujemy system sterowania wirówka laboratoryjna z wieloma osiami:

- Oś 1: naped wirnika
- Oś 2: naped pokrywy
- Oś 3: pompa chlodzenia
- Czujniki: 3x termometr, 1x wibrometr, 1x czujnik pozycji

Mamy do wyboru kilka opcji komunikacyjnych:

**Opcja A: Klasyczny Ethernet**
```
Kazdy sensor/aktuator = osobne polaczenie TCP/IP
Zalety: standard, latwe
Wady: 
  - Latencja: 1-10ms (polaczenie, TCP handshake, routing)
  - Jitter: duzy (zalezny od obciazenia sieci)
  - Przepustowosc: ograniczona przy wielu polaczeniach
```

**Opcja B: EtherCAT**
```
Jeden kabel ethernetowy = jedna siec pol-dzwonowa
Dane ida "w kole" przez wszystkie urzadzenia
Kazde urzadzenie "wyciaga" swoje dane i "wklada" odpowiedz w locie
```

**Jak to dziala w praktyce:**

```
Master                     Slave 1                  Slave 2
   |                          |                        |
   |-----[ramka ETH + dane]--->|                        |
   |    (Slave1 odczytuje)    |----[ramka]--->         |
   |                          |    (Slave2 odczytuje)  |
   |                          |<---[ramka z odp.]----- |
   |<--[ramka z wszystkim]----|                        |
```

Czas okrążenia (cycle time): typowo 100 μs do 1 ms dla 10-100 slave'ów.

### 1.2 Dlaczego EtherCAT jest "typowy" w sterowaniu

Profesor chcialby, zebysmy rozumieli, ze EtherCAT to nie jedyny protokoly, ale reprezentuje pewna klase rozwiazan:

| Cecha | Ethernet klasyczny | EtherCAT | CAN | EtherNet/IP |
|-------|-------------------|-----------|-----|-------------|
| Topologia | Punkt-punkt | Drzewo/linia | Szyna | Punkt-punkt |
| Cycle time | 1-100 ms | 0.1-1 ms | 1-10 ms | 1-10 ms |
| Determinizm | Niekontrolowany | Kontrolowany | Kontrolowany | Sredni |
| Predkosc | 1 Gbps | 100 Mbps | 1 Mbps | 1 Gbps |

EtherCAT wygrywa tam, gdzie potrzebujemy:
- Bardzo krotkich cykli (kHz)
- Determinizmu (jitter < 1 μs)
- Latwej topologii (linia zamiast skomplikowanego okablowania)

### 1.3 Model mentalny: "w cyklu" vs "poza cyklem"

To jest kluczowa koncepcja, ktora bedzie wracac w kazdym projekcie:

**Dane cykliczne** (cyclic):
- Potrzebne do sterowania w kazdej iteracji
- Zadane predkosci, zmierzone predkosci, stany
- Wymiana kazdego cyklu (lub co N-ty cykl)

**Dane acykliczne** (acyclic):
- Konfiguracja, parametry, diagnostyka
- Moga czekac na nastepny cykl
- Rate-limited (np. raz na 100 cykli)

**Zasada:**
- Cykliczne = minimum potrzebne do stabilnosci i safety
- Acykliczne = rate-limited i odseparowane

### 1.4 Przykladowa tabela sygnalow

| Sygnal | Typ | Czestotliwosc | Uzasadnienie |
|--------|-----|---------------|--------------|
| omega_set | Cykliczny | Co cykl | Sterowanie |
| omega_meas | Cykliczny | Co cykl | Sprzezenie zwrotne |
| u_cmd | Cykliczny | Co cykl | Aktuacja |
| temperature | Cykliczny | Co 10 cykli | Wystarczy wolniej |
| fault_status | Cykliczny | Co cykl | Bezpieczenstwo |
| config_Kp | Acykliczny | Na zadanie | Zmiana parametrow |
| log_data | Acykliczny | Co 100 cykli | Diagnostyka |

---

## Czesc II: Dobor cyklu — metodologia

### 2.1 Kompromis "wolno vs. szybko"

**Za wolny cykl:**
- Spada jakosc regulacji (wieksze bledy)
- Mniejsze pasmo sterowania
- Moze byc niestabilny przy zakloceniach

**Za szybki cykl:**
- Wiecej obciazenie CPU mastera
- Wiecej jitter (bo CPU ma mniej czasu na kazda iteracje)
- Wiecej dropoutow przy obciazeniu

### 2.2 Metodyka doboru

Profesor zawsze stosuje metode "konserwatywnego startu":

1. **Ustaw cykl startowy konserwatywnie**
   - Np. dla silnika BLDC: 1 ms (zamiast "optymalistycznego" 0.1 ms)
   
2. **Zmierz p99/p99.9 rt_loop_us**
   ```c
   // Pseudokod
   for (int i = 0; i < 10000; i++) {
       t_start = get_time_ns();
       control_loop();
       t_end = get_time_ns();
       dt = (t_end - t_start) / 1000;
       samples[i] = dt;
   }
   sort(samples);
   p99 = samples[9900];
   p99_9 = samples[9990];
   ```

3. **Iteruj tylko jesli deterministyka trzyma**
   - Jezeli p99.9 < 80% cyklu, mozna probowac przyspieszac
   - Jezeli nie — szukaj problemow w kodzie, nie w systemie

### 2.3 Przykladowe cykle dla roznych aplikacji

| Aplikacja | Typowy cykl | Uzasadnienie |
|-----------|-------------|--------------|
| Serwonaped BLDC | 50-200 μs | FOC wymaga czestego próbkowania |
| Sterowanie wirnika | 0.5-2 ms | Wystarczajace dla mechaniki |
| Robot manipulacyjny | 1-4 ms | Pozycja, trajectory |
| Wizja + sterowanie | 10-100 ms | Wizja jest wolna |

---

## Czesc III: Distributed Clocks (DC)

### 3.1 Problem synchronizacji

Wyobrazmy sobie sytuacje: mamy 3 osie, kazda z wlasnym enkoderem. Chcemy:
- Zmierzyc predkosc kazdej osi
- Porownac faze (czy sa synchroniczne)
- Zidentyfikowac drgania skretne

Problem: kazdy enkoder ma wlasny zegar. I te zegary sie rozjeżdżają:

```
Czas mastera:  0.000000 s
Czas slave 1:  0.000001 s  (offset: +1 μs)
Czas slave 2:  -0.000002 s (offset: -2 μs)
Czas slave 3:  0.000003 s  (offset: +3 μs)
```

Przy cyklu 1 ms i precyzji 1 μs — to jest problem.

### 3.2 Rozwiazanie: Distributed Clocks

EtherCAT ma wbudowany mechanizm synchronizacji zegarow:

1. **Jeden zegar referencyjny** (DC Master albo jeden slave)
2. **Offset compensation** — kazdy slave zna roznice pomiedzy swoim zegarem a DC
3. **Sync pulse** — sygnal synchronizacyjny, ktory mowi "teraz"

```
Bez DC:              Z DC:
                    
Slave 1:  ----  ----  ----      Slave 1:  |---| |---| |---|
Slave 2:  ---  ---  ---  ---     Slave 2:  |---| |---| |---|
Slave 3:  ----  ----  ----      Slave 3:  |---| |---| |---|
                     ^
               rozjechane            zsynchr.
```

### 3.3 Kiedy DC ma sens

DC ma sens, gdy:
- Chcesz spojnego samplingu wielu modulow
- Porownujesz sygnaly pomiedzy slaveami (faza)
- Chcesz zmniejszyc bledy timestampingu
- Potrzebujesz synchronizacji z innymi systemami (np. wizja)

DC NIE ma sensu, gdy:
- Masz jeden slave
- Wystarcza "w przyblizeniu" synchroniczne dane
- Nie porownujesz danych pomiedzy slaveami

---

## Czesc IV: Watchdog — co MUSI byc zdefiniowane

### 4.1 Scenariusz awaryjny

Prosze wyobrazic sobie sytuacje:

```
t=0ms:    Master wysyla dane do Slave 1
t=0.5ms:  Slave 1 odbiera, przetwarza
t=1.0ms:  Master wysyla nastepne dane
t=10ms:   ... (master sie zawiesil)
t=11ms:   Slave 1 czeka na dane...
t=12ms:   Slave 1 czeka...
t=100ms:  Slave 1 nadal czeka, co robi?
```

Co sie stanie z wirnikiem obracajacym sie 10 000 RPM, gdy nagle przestaje dostawac polecenia?

**Odpowiedz**: zalezy od konfiguracji, ale moze byc bardzo zle.

### 4.2 Zachowanie slave przy braku komunikacji

Kazdy slave powinien miec zdefiniowane:
- **Watchdog timeout** — czas, po ktorym stwierdza brak komunikacji
- **Safe state** — co robi po timeout (np. PWM = 0, hamulce załaczone)

```c
// Przykladowa konfiguracja w firmware slave
struct SlaveConfig {
    uint32_t watchdog_timeout_us = 10000;  // 10 ms
    uint8_t  safe_state = SAFE_STATE_BRAKE;  // Hamowanie
    uint8_t  watchdog_action = WDOG_FAULT;    // Przejdz do fault
};
```

### 4.3 Zachowanie master przy braku komunikacji

Master tez musi wiedziec, co robic:

```c
// Reakcja mastera na brak odpowiedzi od slave
enum MasterReaction {
    MASTER_IGNORE,      // Ignoruj (niebezpieczne!)
    MASTER_DEGRADE,     // Ogranicz predkosc, tryb bezpieczny
    MASTER_SAFE_STOP,   // Natychmiast stop
    MASTER_FAULT        // Przejdz do fault
};
```

### 4.4 Watchdog wielopoziomowy

W dobrym systemie sa co najmniej dwa poziomy watchdog:

| Poziom | Gdzie | Co sprawdza | Reakcja |
|--------|-------|-------------|---------|
| Slave watchdog | Wewnetrzny slave | Brak komend od master | Safe state (hamulce) |
| Master watchdog | W2 (master) | Brak odpowiedzi od slave | Degradacja/safe stop |
| Application watchdog | W2 (aplikacja) | Brak iteracji petli | Safe stop |

### 4.5 Histereza i flapping

**Problem**: co jesli komunikacja "miga" (raz jest, raz nie ma)?

```
Good -> Bad -> Good -> Bad -> Good -> Bad
```

To jest "flapping" — system ciagle przejscia, nie moze sie ustabilizowac.

**Rozwiazanie**: histereza

- Przejscie DOBRY -> ZLY: natychmiaste
- Przejscie ZLY -> DOBRY: po czasie T i N probkach sukcesu

```c
// Przykladowa histereza
uint32_t consecutive_ok = 0;
const uint32_t OK_THRESHOLD = 10;  // 10 probek OK

if (communication_ok) {
    consecutive_ok++;
    if (consecutive_ok >= OK_THRESHOLD && state == STATE_FAULT) {
        state = STATE_RECOVERY;  // Powrot po histerezie
    }
} else {
    consecutive_ok = 0;
    if (state != STATE_FAULT) {
        state = STATE_FAULT;  // Natychmiaste przejscie
    }
}
```

---

## Czesc V: Testy obciazeniowe

### 5.1 Dlaczego testujemy obciazenie

W labie wszystko dziala. W produkcji:
- Inne procesy na tym samym CPU
- Siec obciazona innym ruchem
- Telemetria zalewa system

Musimy zweryfikowac, ze system "trzyma" przy obciazeniu.

### 5.2 Scenariusze testowe

| Test | Opis | Kryterium sukcesu |
|------|------|-------------------|
| CPU overload | 100% obciazenie CPU poza watkiem RT | Brak serii missed deadlines |
| Telemetry flood | 10x wiecej danych niz normalnie | Petla RT nadal mieści się w deadline |
| Dropout | Symulacja utraty 1%, 5%, 10% paketow | Deterministyczna reakcja |

### 5.3 Jak mierzyc determinizm

```c
// Pelna struktura do testowania
struct TimingStats {
    uint64_t min_us;
    uint64_t max_us;
    uint64_t mean_us;
    uint64_t p50_us;   // Mediana
    uint64_t p95_us;   // 95 percentyl
    uint64_t p99_us;   // 99 percentyl
    uint64_t p99_9_us; // 99.9 percentyl
    
    uint32_t missed_deadline_count;
    uint32_t total_iterations;
    
    // Histogram
    uint32_t histogram[100];  // 0-100 us
};
```

---

## Czesc VI: Kontekst robotyki

### 6.1 Siec ruchu w komorce produkcyjnej

W komorce z robotem i wieloma osiami:
- Sterowanie ruchem ma wlasna siec (np. EtherCAT)
- Czujniki bezpieczenstwa i IT sa odseparowane
- Synchronizacja czasu jest kluczowa dla korelacji danych

### 6.2 EtherCAT jako "kregoslup czasowy"

EtherCAT nie jest "tylko do napedow" — to tez kregoslup deterministycznego czasowania calej komorki:

```
           EtherCAT Network
           
    +----+   +----+   +----+   +----+
    | S1 |   | S2 |   | S3 |   | S4 |
    |Joint|  |Joint|  |Tool|  |Grip|
    +----+   +----+   +----+   +----+
         \   |   /       \   |   /
          \  |  /         \  |  /
           +----+          +----+
           |Master|        |Safety|
           | (RT) |        | PLC  |
           +----+          +----+
           
Wszystkie slave'y = jeden zegar DC = spojne dane
```

---

## Czesc VII: Podsumowanie i checklisty

### Checklisty:

- [ ] Masz tabele sygnalow: cykliczne/acykliczne
- [ ] Masz metryki jitter/WCRT i liczysz percentyle
- [ ] Masz zdefiniowany watchdog i zachowanie awaryjne
- [ ] Masz histereze dla powrotu z fault

---

## Czesc VIII: Pytania do dyskusji

1. Jak sklasyfikujesz sygnaly na cykliczne/acykliczne i jakie bedzie tego konsekwencje dla jitteru?
2. Jak dobierzesz cykl tak, aby nie "zabic" CPU i nie pogorszyc deterministyki?
3. Jak wyglada zachowanie systemu przy awarii komunikacji (slave i master) i jak unikasz "hold last command"?
4. Kiedy DC ma realny zwrot, a kiedy tylko komplikuje system?

---

## Czesc IX: Zadania praktyczne

### Zadanie 1: Tabela sygnalow

Dla wirówki z trzema osiami (wirnik, pokrywa, chlodzenie) i czterema czujnikami (2x temp, 1x wibracje, 1x predkosc):
1. Podziel sygnaly na cykliczne/acykliczne
2. Oszacuj rozmiar danych na cykl
3. Zaproponuj budzet czasu

### Zadanie 2: Watchdog spec

Zaprojektuj specyfikacje watchdog dla systemu:
- Timeout na poziomie slave
- Timeout na poziomie master
- Zachowanie przy "migajacej" komunikacji
- Procedura recovery

---

## BONUS: Najczestszy blad...

..."szybszy cykl = lepiej". W praktyce szybszy cykl bez pomiaru ogonow opoznien czesto jest gorszy — bo nie wiemy, co sie dzieje, a obciazenie rosnie.

Zawsze mierz, zanim przyspieszasz.

---

*(Koniec wykladu 2)*
