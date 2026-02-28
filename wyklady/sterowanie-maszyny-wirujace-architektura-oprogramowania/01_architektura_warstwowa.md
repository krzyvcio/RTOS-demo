# Wyklad 1: Architektura warstwowa (od sprzetu do HMI)

## Czesc I: Wstep teoretyczny — filozofia podzialu warstw

### 1.1 Dlaczego w ogole dzielimy na warstwy?

Prosze wyobrazic sobie sytuacje, w ktorej cały system sterowania wirówka jest jednym wielkim programem:

```c
// JEDEN PLIK - wszystko razem
void main() {
    while(1) {
        read_sensors();       // Encoder, temperatury
        compute_control();   // PID, saturacje
        send_to_actuator();  // PWM, EtherCAT
        update_display();     // LCD, GUI
        log_to_file();        // Logi
        check_safety();       // Watchdog
        handle_network();      // Ethernet, JSON
    }
}
```

Co jest zlego w tym podejsciu?

**Problem 1: Konflikt czasow**

Petla sterowania potrzebuje 1ms. Logowanie do pliku moze zabrac 10ms. Update GUI — 50ms. Siec — 100ms. Jezeli to wszystko jest w jednej petli, to albo:
- sterowanie czeka na logike (opoznienia)
- logika czeka na sterowanie (nieresponsywnosc)

**Problem 2: Brak izolacji bledow**

Blad w logowaniu (np. przepełnienie dysku) wisi caly system. Blad w GUI (np. wyciek pamieci) wisi caly system.

**Problem 3: Niemożliwosc testowania**

Nie mozna przetestowac samego sterowania bez calej reszty. Nie mozna wymienić GUI bez dotykania logiki sterowania.

**Problem 4: Rozne wymagania**

| Funkcja | Wymaganie czasowe | Ryzyko przy opoznieniu |
|---------|-------------------|------------------------|
| Sterowanie | < 1ms | Katastrofa (niestabilnosc) |
| GUI | < 100ms | Dyskomfort uzytkownika |
| Logowanie | < 1s | Utrata danych diagnostycznych |
| Siec | < 10s | Timeout, retransmisja |

Mieszanie ich razem to jak wbijanie sruby młotkiem — technicznie mozliwe, ale nie jest to wlasciwe narzedzie.

### 1.2 Analogie z zycia codziennego

Prosze pomyslec o organizacji fabryki:

- Na hali produkcyjnej (warstwa napedow) roboty wykonuja precyzyjne ruchy w milisekundach. To jest "hard RT".
- Mistrz zmianowy (warstwa sterowania) koordynuje prace, ale nie wykonuje samych operacji. To jest "firm RT".
- Biuro planowania (warstwa nadzoru) optymalizuje harmonogram, ale nie musi reagowac w czasie rzeczywistym. To jest "soft RT".
- Centrala korporacyjna (warstwa biznesowa) dostaje raporty raz na dzien. To jest "offline".

Kazdy poziom ma swoj rytm, swoje narzedzia, swoje ryzyka. I to jest dobre — bo kazdy moze sie skupic na swoim.

### 1.3 Warstwy w systemach sterowania

Profesor chcialby, abyscie zapamietali jeden schemat:

```
Warstwa 4: modelowanie/symulacja (offline)
    |
    |  <-- Kontrakt: dane historyczne, scenariusze, parametry
    v
Warstwa 3: nadzor/HMI (soft-RT)
    |
    |  <-- Kontrakt: setpointy, stany, zdarzenia
    v
Warstwa 2: kontroler RT / master (firm-RT)
    |
    |  <-- Kontrakt: probki pomiarowe, komendy, statusy
    v
Warstwa 1: drive/MCU (hard-RT)
    |
    |  <-- Kontrakt: prad, moment, stany bezpieczeństwa
    v
Warstwa 0: sprzet (opcjonalnie FPGA)
```

Kazda warstwa ma jedna glowna odpowiedzialnosc. I to jest klucz do zrozumienia calej architektury.

---

## Czesc II: Szczegółowy opis warstw

### Warstwa 1: Drive / MCU (Hard RT)

**Odpowiedzialnosc**: "Trzymam prad/moment i reaguję bezpiecznie"

To jest najnizsza warstwa, najblizej sprzetu. Jej zadania:

```c
// Typowe zadania W1:
void pwm_update();          // Aktualizacja PWM (co 20-50us)
void current_measurement(); // Pomiar pradu (ADC)
void safety_check();        // Sprawdzenie limitow (natychmiast)
void encoder_read();        // Odczyt enkodera
void watchdog_feed();       // Zasypianie watchdog
```

**Charakterystyka czasowa**:
- Cykl: 20 μs do 1 ms (zaleznie od aplikacji)
- Jitter: < 1 μs (najtwardsze wymagania)
- Latency: < cyklu

**Co NIE moze sie dziac w W1**:
- Czekanie na odpowiedz z zewnatrz
- Alokacja pamieci
- Logowanie
- Dowolna komunikacja sieciowa

**Przyklad**: W wirówce laboratoryjnej, W1 to mikrokontroler STM32 albo podobny, ktory:
- generuje sygnaly PWM dla trzech faz silnika BLDC
- mierzy prady fazowe przez ADC
- sprawdza limity temperatury i pradu
- reaguje na sygnal stop (natychmiast)

### Warstwa 2: Master / RT Controller (Firm RT)

**Odpowiedzialnosc**: "Koordynuje cykl, licze regulacje, trzymam timing"

Ta warstwa to "mozg" systemu. Jej zadania:

```c
// Typowe zadania W2:
void EtherCAT_cycle();      // Cykl EtherCAT (typowo 1ms)
void controller_compute();  // Regulator (PI/PID/FOC)
void trajectory_update();   // Aktualizacja trajektorii
void state_machine();       // FSM (Start/Stop/Run/Fault)
void publish_telemetry();  // Wysylka do ring buffer
void watchdog_manage();     // Zarzadzanie watchdog
```

**Charakterystyka czasowa**:
- Cykl: 100 μs do 10 ms
- Jitter: < 10 μs (dobry), < 100 μs (akceptowalny)
- Latency: < cyklu

**Co moze sie dziac, ale z ostroznoscia**:
- Komunikacja z W3 (ale asynchronicznie, nie blokujaco)
- Obliczenia zmiennoprzecinkowe (ale krotkie)
- Latwe algorytmy (filtry, PID, proste transformacje)

**Co NIE moze sie dziac w W2**:
- Blokujace IO (siec, dysk)
- Dlugie obliczenia (> 50% cyklu)
- Czekanie na odpowiedz

**Przyklad**:ówce, W2 to Linux PRE W wirEMPT_RT z watkiem SCHED_FIFO, ktory:
- odbiera dane z EtherCAT (setpointy)
- oblicza sterowanie PI dla predkosci
- wysyja komendy do W1
- publikuje telemetrie do W3

### Warstwa 3: Nadzor / HMI (Soft RT)

**Odpowiedzialnosc**: "Wizualizuje, loguje, alarmuje, nie psuje RT"

Ta warstwa jest dla ludzi:

```c
// Typowe zadania W3:
void update_gui();          // GUI (Qt, web, ncurses)
void log_telemetry();       // Zapis do bazy/pliku
void handle_user_input();   // Obsluga klawiatury/myszy
void alarm_manager();       // Obsluga alarmow
void configuration_ui();   // Konfiguracja parametrow
```

**Charakterystyka czasowa**:
- Cycle: 10 ms do 1 s
- Jitter: nie krytyczny
- Latency: moze byc opoznione

**Co moze sie dziac w W3**:
- Wszystko! JSON, HTTP, bazy danych, GUI, logowanie
- Ale NIE wplywac na W1/W2

**Kluczowa zasada**: W3 moze "nie dzialac" (zawiesic sie, wolno reagowac) i system sterowania nadal dziala.

**Przyklad**: W wirówce, W3 to aplikacja webowa albo desktopowa, ktora:
- pokazuje aktualna predkosc i temperature
- pozwala ustawic zadana predkosc
- zapisuje logi do bazy danych
- wyswietla alarmy

### Warstwa 4: Modelowanie / Symulacja (Offline)

**Odpowiedzialnosc**: "Projektuje i waliduje algorytmy offline"

```c
// Typowe zadania W4:
void system_identification();  // Identyfikacja modelu
void controller_design();       // Projekt regulatora
void simulation();              // Symulacja SIL/HIL
void parameter_tuning();        // Dostrajanie parametrow
void offline_analysis();       // Analiza danych historycznych
```

**Charakterystyka czasowa**:
- Cycle: sekundy, minuty, godziny
- Latency: nie krytyczna

---

## Czesc III: Kontrakty Miedzy Warstwami

### 3.1 Pojecie kontraktu

Kontrakt to formalne porozumienie pomiedzy dwiema warstwami, ktore mowi:

- Jakie dane sa wymieniane
- W jakim formacie
- Z jakim znaczeniem
- Jak sa wersjonowane

Bez kontraktu — kazda zmiana w jednej warstwie psuje druga.

### 3.2 Przykladowy kontrakt W2 <-> W3

**W2 publikuje do W3** (telemetria):

```c
struct Telemetry {
    uint32_t    schema_version;  // Wersja schematu
    uint64_t    timestamp_ns;    // Monotoniczny czas
    float       omega_set;       // Zadana predkosc [rad/s]
    float       omega_meas;      // Zmierzona predkosc
    float       u_cmd;          // Sterowanie
    uint8_t     state;          // Stan FSM
    uint8_t     fault_code;     // Kod bledu (0 = OK)
};
```

**W3 wysyla do W2** (komendy):

```c
struct Command {
    uint32_t    schema_version;
    float       omega_target;    // Zadana predkosc
    uint8_t     cmd;            // START/STOP/RESET
    float       ramp_rate;      // Maksymalna zmiana predkosci
};
```

### 3.3 Zasady wersjonowania

1. Kazdy kontrakt ma numer wersji
2. Nowa wersja musi byc kompatybilna wsteczna (albo jawnie zmieniona)
3. Jesli nie mozna byc kompatybilnym — zmien nazwe pola

---

## Czesc IV: Tryby pracy (FSM) jako element "klejacy"

### 4.1 Dlaczego FSM jest wazny

Maszyny wirujace (i w ogole wiekszosc systemow mechatronicznych) nie dzialaja w trybie "ciaglego ruchu". Mają stany:

- Start (uruchomienie, initializacja)
- Rampa (rozpedzanie do zadanej predkosci)
- Praca ustalona (zadana predkosc)
- Hamowanie (zatrzymywanie)
- Stop kontrolowany
- Stop awaryjny (E-stop)
- Fault (blad)

Przejscia pomiedzy stanami musza byc:
- Zdefiniowane (wiemy, co sie dzieje)
- Bezpieczne (nie mozna przejsc w stan niebezpieczny)
- Deterministic (kazde zdarzenie daje jeden, znany rezultat)

### 4.2 Przykladowy FSM dla wirówki

```
        +---------+    start     +--------+
        |  IDLE   | -----------> |  READY |
        +---------+              +--------+
              ^                        |
              |                        | run
              |            +---------+ |      +----------+
              +-- stop ----|  FAULT  |<- fault| RUNNING  |
               reset       +---------+        +----------+
                                              ^         |
                                              |         | stop
                                              +----+----+
                                                   |
                                              +---------+
                                              | STOPPING|
                                              +---------+
                                                   |
                                              +---------+
                                              | STOPPED |
                                              +---------+
```

### 4.3 Gdzie umieszczac FSM?

FSM powinien byc po stronie warstwy, ktora ma najtwardsze wymagania na reakcje.

W przypadku wirówki:
- Bezpieczenstwo (E-stop) — w W1 (najszybsza reakcja)
- Sterowanie (start/stop) — w W2 (koordynacja)
- Workflow (pelna sekwencja) — w W3 (dla uzytkownika)

---

## Czesc V: Integracja robot + modul procesu

### 5.1 Kontekst

W nowoczesnych systemach (rok 2035) mamy do czynienia z "komorkami autonomicznymi":

- Robot (manipulator) — przenosi probki, elementy
- Modul procesu (wirówka, mieszalnik) — wykonuje operacje fizyczne
- System nadrzedny — orkiestruje calosc

Kazdy z nich ma wlasny FSM!

### 5.2 Minimalny kontrakt integracyjny

```c
// Stan modulu procesu
enum ModuleState {
    MODULE_FAULT = 0,
    MODULE_SAFE_STOP,
    MODULE_READY,
    MODULE_RUNNING
};

// Stan robota
enum RobotState {
    ROBOT_IDLE = 0,
    ROBOT_MOVING,
    ROBOT_INTERLOCK,
    ROBOT_FAULT
};

// Zdarzenia (eventy)
enum Event {
    EVENT_NONE = 0,
    EVENT_ALARM,
    EVENT_COMPLETE,
    EVENT_INTERLOCK,
    EVENT_E_STOP,
    EVENT_FAULT
};
```

### 5.3 Zasady integracji

1. **Robot i modul maja osobne FSM** — nie ma jednego "super-FSM"
2. **Orkiestrator (workflow) spina stany** — ale tylko na poziomie soft-RT
3. **Krytyczne reakcje (safe stop) sa lokalne** — kazdy modul reaguje sam, nie czeka na orkiestratora

To jest kluczowe: jeśli orkiestrator sie zawiesi, kazdy modul musi byc w stanie sam siebie zabezpieczyc.

---

## Czesc VI: Trend 2035 — komorki autonomiczne

W 2035 typowe jest, ze:
- Modul procesu ma wbudowana predykcyjna diagnostyke
- Robot ma ograniczenia energii i aktywne monitorowanie kolizji
- Cala komorka ma audit trail i powtarzalnosc (compliance)

Ale zasady architektoniczne sie nie zmienily:
- Hard RT w W1
- Firm RT w W2
- Soft RT w W3
- Offline w W4

---

## Czesc VII: Podsumowanie i checklisty

### Zasady architektoniczne:

| Zasada | Wyjasnienie |
|--------|-------------|
| Jedna warstwa = jedna odpowiedzialnosc | W1: bezpieczenstwo, W2: sterowanie, W3: wizualizacja |
| Kontrakty sa wersjonowane | Kazda zmiana jest jawna i kompatybilna |
| Krytyczne reakcje sa lokalne | Safe stop w kazdym module |
| W3 moze "umrzec" — W1/W2 dzialaja | Izolacja bledow |

### Checklisty:

- [ ] Kazda warstwa ma osobny budzet czasu
- [ ] Watek RT jest oddzielony od logowania i sieci IT
- [ ] Interfejsy sa wersjonowane i testowane
- [ ] FSM jest zdefiniowany i testowalny

---

## Czesc VIII: Pytania do dyskusji

1. Jakie sa granice odpowiedzialnosci kazdej warstwy i co sie stanie, gdy warstwa wyzsza przestanie dzialac?
2. Jakie kontrakty danych sa minimalne, zeby robot i modul procesu byly integrowalne (FSM + events)?
3. Co jest single point of failure w tej architekturze i jak je ograniczasz?
4. Jak zapewnisz, ze nadzor/HMI nigdy nie wplynie na timing petli RT?

---

## Czesc IX: Zadania praktyczne

### Zadanie 1: Projekt FSM

Zaprojektuj FSM dla modulu procesu (wirówka) i FSM dla robota (manipulator), a potem opisz ich synchronizacje:
- Jakie sa stany kazdego FSM?
- Jakie sa przejscia?
- Jakie sa warunki przejscia?
- Jak wyglada komunikacja pomiedzy nimi?

### Zadanie 2: Zdarzenia krytyczne

Wypisz 5 zdarzen, ktore musza skutkowac natychmiastowa reakcja lokalna (watchdog/safe stop), bez czekania na orkiestratora.

Przyklady:
1. Przekroczenie temperatury silnika
2. Utrata sygnalu z enkodera
3. Przekroczenie zadanej predkosci
4. Zadzialanie czujnika wibracji
5. Brak komunikacji z nadrzednym systemem

---

## BONUS: Jesli interfejs jest za duzy...

...to zwykle jest przerośniety.

Jezeli interfejs pomiedzy warstwami ma wiecej niz kilka kluczowych sygnalow w cyklu, to prawdopodobnie:
- Za duzo odpowiedzialnosci w jednej warstwie
- Za czesto przesylane dane, ktore nie sa potrzebne
- Zla dekompozycja

Redukcja kontraktu czesto poprawia deterministyke i niezawodnosc.

---

*(Koniec wykladu 1)*
