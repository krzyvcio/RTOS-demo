# Wyklad 0: Pojecia na start i zasady architektury RT

## Czesc I: Wstep teoretyczny — dlaczego sterowanie w czasie rzeczywistym jest inne

### 1.1 Geneza problemu

Prosze sobie wyobrazic sytuacje: projektujemy system sterowania dla wirnika wirówki laboratoryjnej. Mamy zadac predkosc obrotowa 10 000 obrotow na minute, czyli 166 obrotow na sekunde. Kazda milisekunda opoznienia pomiedzy pomiarem predkosci a wydaniem nowego sygnalu sterujacego powoduje:

- blad polozenia kata ~1 stopnia
- w przypadku niestabilnosci — narastanie oscylacji
- przy duzych predkosciach — ryzyko utraty kontroli i uszkodzenia mechanicznego

W zwyklym systemie informatycznym — powiedzmy serwerze webowym — milisekunda opoznienia to zadna historia. Strona laduje sie 100ms albo 200ms i uzytkownik tego nie zauwazy. W sterowaniu predkosci wirnika ta sama milisekunda moze byc roznica pomiedzy stabilna praca a katastrofa.

**To jest sedno czasu rzeczywistego**: nie chodzi o bycie "szybkim", chodzi o bycie *przewidywalnym*.

### 1.2 Dlaczego "srednia wydajnosc" nie wystarcza

Prosze zwrocic uwage na wykres typowego systemu:

```
Czas wykonania iteracji [ms]
     ^
 5.0 |                    *  ( outliers - sporadyczne opoznienia )
 4.5 |              *   *
 4.0 |          *   *   *
 3.5 |      *   *   *   *
 3.0 |  *   *   *   *   *   *   *   *   *   *   *   *   *   *
     +-------------------------------------------------> czas
```

Srednia moze byc rownie 3ms. Ale te sporadyczne piki do 5ms moga zrujnowac caly system.

**Dlaczego tak sie dzieje?**

Prosze wyobrazic sobie procedure sterowania:

```c
void control_loop() {
    // 1. odczyt predkosci z enkodera
    omega = read_encoder();           // ~0.1ms
    
    // 2. filtr (np. filtr Kalmana lub srednia ruchoma)
    omega_filtered = filter(omega);   // ~0.2ms
    
    // 3. obliczenie bledu
    error = setpoint - omega_filtered; // ~0.01ms
    
    // 4. regulator PID
    u = pid_compute(error);            // ~0.1ms
    
    // 5. zapis do PWM
    set_pwm(u);                        // ~0.1ms
    
    // RAZEM: ~0.5ms przy zalozeniu "wszystko dziala"
}
```

A teraz wyobrazmy sobie, ze w tle dziala inny proces:

```c
void background_task() {
    log_message("Iteration completed");  // Syscall - moze zabrac 1-5ms!
    malloc(1024);                        // Alokacja - moze byc wolna
    read_from_network();                 // Blokujace IO
}
```

I wtedy nasza petla sterowania nagle musi czekac. I to jest wlasnie ten moment, kiedy "srednia wydajnosc" nie ma znaczenia — liczy sie *ogon* rozkadu, czyli najgorsze przypadki.

### 1.3 Pojecie determinizmu

Profesor w swojej praktyce przemyslowej wielokrotnie spotykal sie z takim scenariuszem:

Inzynierowie zbudowali system sterowania, wszystko dzialalo pieknie w laboratorium. Sredni czas wykonania petli: 1.2ms. Wydajnosc 99%. System oddany do produkcji — i nagle, sporadycznie, wirnik zaczynal sie wibracyjnie zachowywac. Nikt nie mogl zrozumieć dlaczego.

Problem: w laboratorium bylo cicho, w produkcji — inne procesy, siec, logowanie, dysk. W 1% przypadkow czas wykonania skakal do 8ms. I to wystarczylo, zeby system sterowania stal sie niestabilny.

**Wniosek**: w sterowaniu RT mówimy o *determinizmie*, czyli o gwarancji, ze kazda iteracja zakonczy sie w zdefiniowanym czasie. Nie chodzi o to, "ile srednio to trwa", chodzi o to, "ile to trwa w najgorszym przypadku".

---

## Czesc II: Wyklad — pojecia podstawowe

### 2.1 Cykl (T) i deadline

Prosze zapamietac dwie fundamentalne definicje:

**Cykl sterowania (T)** to okres, z jaka wykonywana jest iteracja petli sterowania. Jezeli mamy T = 1ms, to nasza petla wykonuje sie 1000 razy na sekunde.

**Deadline** to maksymalny dopuszczalny czas od startu iteracji do momentu, gdy sterowanie musi byc gotowe. Deadline zawsze musi byc mniejszy niz T — w przeciwnym razie nastepna iteracja zacznie sie przed zakonczeniem poprzedniej.

```
t=0ms                    t=1ms                   t=2ms
   |------------------------|------------------------|
   | start -> deadline      | start -> deadline      |
   | (musi byc < 1ms)      | (musi byc < 1ms)       |
```

W praktyce wymagamy:
```
t_iteracji <= deadline  (dla prawie wszystkich iteracji; liczymy percentyle)
```

**Dlaczego "prawie wszystkich"?**

W kazdym systemie rzeczywistym zdarzaja sie sytuacje wyjatkowe — przerwanie sprzetowe, diagnistyka, przejscie do trybu awaryjnego. Nie mozemy wymagac 100% determinizmu. Ale mozemy i musimy wymagac, by 99.9% albo 99.99% iteracji miescilo sie w deadline.

### 2.2 Latency i jitter

**Latency** (opoznienie) to czas od zdarzenia do reakcji. Na przyklad:
- latency sensora: czas od fizycznego zdarzenia do jego odczytu
- latency aktuatora: czas od wydania komendy do fizycznej reakcji
- latency calkowita (end-to-end): czas od pomiaru do aktuacji

**Jitter** to zmiennosc tego opoznienia. To jest kluczowa różnica:

- Latency stala da sie *kompensowac* — mozemy zmierzyc, ze system zawsze ma 2ms opoznienia i wziac to pod uwage w regulatorze.
- Jitter losowy *rozwala* margines fazy i stabilnosc — nigdy nie wiadomo, ile czasu bedzie trwac dana iteracja.

Prosze spojrzec na przyklad:

```
System A: latency = 3ms ± 0.1ms (jitter = 0.1ms)
System B: latency = 3ms ± 2ms   (jitter = 2ms)

Dla regulatora PID z czasem calkowania Ti = 10ms:
- System A: dziala stabilnie
- System B: niestabilny — jitter > 20% latency
```

### 2.3 WCET i WCRT

To sa dwa pojecia, ktore poczatkujący często mylą:

**WCET** (Worst-Case Execution Time) to czas wykonania *samego kodu* w najgorszym przypadku. Jezeli nasza petla sterowania ma 100 linii kodu i kazda linia moze trwac do 10μs, to WCET = 1ms.

Ale to nie jest pelny obraz! Bo kod musi tez *czekac* na CPU, jesli inne zadania sa w systemie.

**WCRT** (Worst-Case Response Time) to czas od "chce wykonac" do "wykonane", czyli WCET plus czas oczekiwania na CPU.

```
WCRT = WCET + czas oczekiwania w kolejce
```

W RT interesuje nas WCRT, bo to jest faktyczny czas reakcji systemu.

**Jak to zmierzyc w praktyce?**

```c
void control_loop() {
    uint64_t t_start = get_time_ns();
    
    // ... kod sterowania ...
    
    uint64_t t_end = get_time_ns();
    uint64_t dt = t_end - t_start;
    
    // Statystyka
    if (dt > max_observed) max_observed = dt;
    if (dt > DEADLINE) missed_deadline_count++;
    
    // Histogram (do analizy offline)
    histogram[dt / 1000]++;  // bins w mikrosekundach
}
```

---

## Czesc III: Krytyczna sciezka — złota zasada projektowania

### 3.1 Definicja

Prosze zapamietac jeden schemat, ktory bedzie wracal w kazdym wykladzie:

```
Krytyczna sciezka end-to-end:
sensor -> timestamp/bufor -> filtr/estymacja -> regulator -> transport -> drive -> plant
```

Kazdy element na tej sciezce ma wlasny budzet czasu. Jezeli nie potrafisz podac budzetu dla kazdego elementu, to twoja architektura jest "na wiare".

### 3.2 Przyklad praktyczny

Wirówka laboratoryjna — prototypowa sciezka:

| Element | Typowy budżet | Uwagi |
|---------|---------------|-------|
| Encoder + odczyt | 10-50 μs | Zależy od interfejsu (SPI, Quadrature) |
| Timestamp + bufor | 1-5 μs | Monotoniczny zegar |
| Filtr (np. srednia 8 próbek) | 5-20 μs | Zaleza od zlozonosci |
| Regulator PI | 10-50 μs | Proste operacje arytmetyczne |
| Transport (EtherCAT) | 100-500 μs | Zależy od cyklu |
| Drive (PWM update) | 5-20 μs | Sprzetowe |
| RAZEM | ~200-600 μs | Przy cyklu 1ms mamy zapas |

Jezeli sumujemy i wychodzi wiecej niz deadline — mamy problem architektoniczny, nie algorytmiczny.

---

## Czesc IV: Czego NIE robic w sciezce RT

To sa twarde zasady, nie "zalecenia":

### 4.1 Brak alokacji dynamicznej

```c
// ZŁE:
void control_loop() {
    double* buffer = new double[1024];  // NIE W PETLI RT!
}

// DOBRE:
static double buffer[1024];  // Alokacja statyczna raz na start
```

Dlaczego? `new`/`malloc` moze:
- zająć mikrosekundy
- wywołać systemowy `brk()` lub `mmap()`
- zwrócić NULL (co robimy wtedy?)

### 4.2 Brak IO blokującego

```c
// ZŁE:
void control_loop() {
    write_to_file(data);    // BLOKUJĄCE!
    char* response = http_get(url);  // NIE!
}

// DOBRE:
void control_loop() {
    // Tylko operacje w pamieci
}
```

### 4.3 Brak mutexow na danych krytycznych

```c
// ZŁE:
std::mutex data_mutex;
double measured_speed;

void control_loop() {
    std::lock_guard<std::mutex> lock(data_mutex);  // Może blokować!
    error = setpoint - measured_speed;
}
```

Problem: *priority inversion*. Watek o niskim priorytecie trzyma mutex, watek RT czeka. W skrajnym przypadku — deadlock.

### 4.4 Brak GC w watku RT

```c
// ZŁE (Python):
def control_loop():
    data = process()  # Python runtime moze wywolac GC!
    return data
```

Garbage Collector w Python, Java, JavaScript moze zatrzymac caly watek na dziesiatki milisekund.

---

## Czesc V: Telemetria — co logowac od pierwszego dnia

To jest lista minimum, bez ktorego nie warto zaczynac debugowania:

```c
struct TelemetrySample {
    uint64_t t_start;        // Czas startu iteracji
    uint64_t t_end;          // Czas zakonczenia
    uint32_t rt_loop_us;     // Czas wykonania w mikrosekundach
    uint8_t  miss_deadline; // Czy deadline przekroczony (0/1)
    float    omega_set;      // Zadana predkosc
    float    omega_meas;     // Zmierzona predkosc
    float    omega_err;      // Blad
    float    u_cmd;         // Sterowanie (moment/prad)
    uint8_t  saturation;    // Czy nasycenie
    uint8_t  comm_status;   // Status komunikacji
};
```

**Dlaczego to jest wazne?**

Bez telemetrii — debugujesz w ciemno. Z telemetria:
- widzisz, czy petla mieści sie w deadline
- widzisz, czy regulator jest nienasycony
- widzisz, czy komunikacja nie gubi paketow

---

## Czesc VI: Podsumowanie i checklisty

### Pojecia do zapamietania:

| Pojęcie | Definicja |
|---------|-----------|
| Cykl (T) | Okres petli sterowania |
| Deadline | Maksymalny czas na iteracje |
| Latency | Opoznienie end-to-end |
| Jitter | Zmiennosc latency |
| WCET | Najgorszy czas wykonania kodu |
| WCRT | Najgorszy czas od startu do zakonczenia |

### Checklisty:

- [ ] Masz definicje: T, deadline, WCRT, jitter
- [ ] Masz diagram sciezki end-to-end i miejsce, gdzie mierzysz czas
- [ ] Masz zasade: RT nie blokuje sie na niczym poza wlasnym zegarem

---

## Czesc VII: Pytania do dyskusji

1. Dlaczego w RT kluczowe sa ogony opoznien, a nie srednia wydajnosc?
2. Jak odroznic WCRT od WCET w praktyce (na danych z systemu)?
3. Co w Twojej architekturze jest "twarde RT", a co moze byc soft-RT, i dlaczego?
4. Jakie 5 sygnalow musisz logowac, zeby nie debugowac w ciemno?

---

## Czesc VIII: Zadania praktyczne

### Zadanie 1: Krytyczna sciezka dla robota wirujacego

Narysuj krytyczna sciezke dla robota, ktory pobiera probki i uruchamia modul procesu (urzadzenie wirujace). Zaznacz, ktore elementy to:
- hard RT (np. petla napedu)
- firm RT (np. master EtherCAT)
- soft RT (np. HMI, logowanie)

Dla kazdego elementu podaj przykladowy budzet czasu.

### Zadanie 2: Analiza istniejacego systemu

Wez dowolny system sterowania (np. Arduino z silnikiem DC) i:
1. Zidentyfikuj elementy krytycznej sciezki
2. Zmierz czasy wykonania kazdego elementu
3. Zbuduj histogram czasow wykonania calej petli
4. Oszacuj WCET i porownaj z oczekiwanym deadline

---

## BONUS: Zanim dotkniesz konfiguracji kernela...

...wywal z petli RT wszystko, co moze blokowac (IO, mutex, alokacje). To daje zazwyczaj wiecej niz tuning systemu.

Najczestszy błąd poczatkujacych inzynierow: "Mój system nie jest wystarczajaco deterministyczny, musze przekonfigurowac kernel Linuxa".

Prawda: 90% problemow z determinizmem to kod w petli RT, ktory nie powinien tam byc.

---

*(Koniec wykladu 0)*
