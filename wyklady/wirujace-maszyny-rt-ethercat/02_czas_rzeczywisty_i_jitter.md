# Wykład 2: Czas rzeczywisty, jitter i deterministyczność

## Czesc I: Wstep teoretyczny — dlaczego czas jest najwazniejszy

### 1.1 Geneza problemu

Proszę wyobrazić sobie sytuację: zbudowałeś regulator PID, wszystko wydaje się w porządku — ale wirnik czasami "płynie" w niespodziewany sposób. regulator jest dobrze dostrojony, mechanika jest w porządku — ale coś jest nie tak.

**Co się dzieje?** Najprawdopodobniej — jitter.

Proszę spojrzeć na wykres:

```
Czas wykonania pętli [ms]
     ^
 5.0 |                    *  (outliers)
 4.5 |              *   *
 4.0 |          *   *   *
 3.5 |      *   *   *   *
 3.0 |  *   *   *   *   *   *   *   *   *   *   *   *   *
     +--------------------------------------------------> czas
     
     Srednia = 3ms — wydaje się OK
     Ale piki do 5ms moga zrujnować sterowanie!
```

**To jest sedno problemu:** w sterowaniu "szybko" nie znaczy "dobrze". Liczy się **przewidywalność**.

### 1.2 Dlaczego srednia nie wystarcza

Profesor zawsze powtarza: **"Sterowanie przegrywa przez ogony opóźnień, nie przez średnią."**

Wyjaśnienie:

W regulatorze PID zakładasz stały okres próbkowania T. Jeśli T "pływa":
- Efektywne wzmocnienie "pływa"
- Faza "pływa"
- Margines stabilności maleje

**Wynik:** sporadyczne oscylacje, które pojawiają się "bez powodu".

### 1.3 Dwa rodzaje latency

Proszę zapamiętać:

| Rodzaj | Opis | Co z tym zrobisz |
|--------|------|------------------|
| **Latency stała** | Zawsze np. 2 ms | Możesz skompensować w regulatorze |
| **Latency losowa (jitter)** | Zmienna, np. 1-5 ms | Zabija stabilność |

> Latency stałą da się skompensować. Jitter losowy — nie.

---

## Czesc II: Pojecia ktore musisz miec "w palcach"

### 2.1 Okres pętli (T)

```c
// Okres pętli - co ile wykonujesz sterowanie
uint64_t T = 1000000;  // 1 ms = 1000 μs = 1000000 ns
```

T = 1 ms → pętla wykonuje się 1000 razy na sekundę

### 2.2 Opóźnienie (latency)

```c
// Opóźnienie end-to-end
uint64_t t_measure = get_time();    // Pomiar
process_data();
uint64_t t_actuate = get_time();    // Aktuacja
uint64_t latency = t_actuate - t_measure;
```

Latency = czas od pomiaru do aktuacji.

### 2.3 Jitter

```c
// Jitter - zmienność czasu wykonania
for (int i = 0; i < 10000; i++) {
    t_start = get_time_ns();
    // kod sterowania
    t_end = get_time_ns();
    dt = t_end - t_start;
    samples[i] = dt;
}

// Jitter = max(samples) - min(samples)
```

### 2.4 WCET i WCRT

| Pojęcie | Definicja | Przykład |
|---------|-----------|----------|
| **WCET** | Worst-Case Execution Time — czas samego kodu | 500 μs |
| **WCRT** | Worst-Case Response Time — kod + czekanie na CPU | 800 μs |

WCRT = WCET + czas oczekiwania w kolejce

---

## Czesc III: Model mentalny — opóźnienie jako "dodatkowa faza"

### 3.1 Dlaczego jitter jest groźny

W pętli sterowania opóźnienie zachowuje się jak dodatkowe przesunięcie fazowe.

```
Bez jittera:          Z jitterem:
                    
Sygnał wyjściowy     Sygnał wyjściowy
    |                      |
    |  /----\              |  /----\  /----\
    | /      \             |/      \/      \
    v/        \            v               \
       Czas         Czas
       
    Stabilny         Oscyluje!
```

### 3.2 Wzór na margines fazy

Dla regulatora PID, opóźnienie T wprowadza przesunięcie fazowe:

```
Δφ = -ω × T [radiany]
```

gdzie ω to częstotliwość, a T to opóźnienie.

**Wniosek:** Im większe opóźnienie i wyższa częstotliwość — tym większa utrata fazy.

---

## Czesc IV: Rodzaje opóźnień

### 4.1 Opóźnienie pomiaru

```
Czujnik → Filtr antyaliasing → ADC → Bufor
    |
    +-- Typowy czas: 10-100 μs
    +-- Zależy od typu sensora
```

### 4.2 Opóźnienie transportu

```
Bufor → Kolejka → Sieć (EtherCAT) → DMA → Bufor
    |
    +-- Typowy czas: 50-500 μs
    +-- Zależy od obciążenia sieci
```

### 4.3 Opóźnienie obliczeń

```
Bufor wejściowy → Estymacja → Regulator → Bufor wyjściowy
    |
    +-- Typowy czas: 10-100 μs
    +-- Zależy od złożoności algorytmu
```

### 4.4 Opóźnienie aktuacji

```
Bufor wyjściowy → PWM → Driver → Silnik
    |
    +-- Typowy czas: 10-50 μs
    +-- Zależy od częstotliwości PWM
```

---

## Czesc V: Jak budzetowac czas

### 5.1 Szablon tabeli budżetu

| Etap | Opis | Czas (μs) |
|------|------|----------|
| t1 | Pobranie pomiarów | 20 |
| t2 | Estymacja/filtracja | 30 |
| t3 | Regulator | 50 |
| t4 | Wysyłka na sieć/napęd | 100 |
| m | Margines bezpieczeństwa | 100 |
| **RAZEM** | | **300** |

**Warunek:** t1 + t2 + t3 + t4 + m ≤ T

Przy T = 1000 μs (1 ms): 300 μs < 1000 μs ✓

### 5.2 Przykład praktyczny

```c
// Pomiar budżetu czasu
void control_loop() {
    uint64_t t1 = get_time_ns();  // start
    
    // t1: pobranie pomiarów
    read_sensors();
    uint64_t t2 = get_time_ns();
    
    // t2: estymacja
    estimate_state();
    uint64_t t3 = get_time_ns();
    
    // t3: regulator
    compute_control();
    uint64_t t4 = get_time_ns();
    
    // t4: wysyłka
    send_to_drive();
    uint64_t t5 = get_time_ns();
    
    // Raport
    printf("t1=%lu t2=%lu t3=%lu t4=%lu total=%lu\n",
           t2-t1, t3-t2, t4-t3, t5-t4, t5-t1);
}
```

---

## Czesc VI: Co mierzyc — minimum diagnostyczne

### 6.1 Metryki obowiązkowe

| Metryka | Jak mierzyć | Po co |
|---------|-------------|-------|
| Czas iteracji pętli | t_end - t_start | Podstawowa metryka |
| Histogram jitteru | Zbieraj 10000 próbek | Rozkład opóźnień |
| Percentyle p95/p99/p99.9 | Sortuj, bierz percentyl | Ogony rozkładu |
| Missed deadlines | Licznik | Czy system wyrabia |
| Dropouty komunikacji | Licznik błędów | Jakość połączenia |

### 6.2 Kluczowa zasada

> Jeśli możesz mierzyć tylko jedną rzecz: mierz **p99/p99.9 czasu iteracji**, nie średnią.

**Dlaczego?** Bo p99/p99.9 pokazuje ogon — to, co psuje sterowanie.

### 6.3 Kod do pomiaru

```c
// Prosty recorder jittera
#define N_SAMPLES 10000

uint32_t loop_times[N_SAMPLES];
int sample_idx = 0;

void record_loop_time(uint64_t dt_us) {
    loop_times[sample_idx] = dt_us;
    sample_idx = (sample_idx + 1) % N_SAMPLES;
    
    if (sample_idx == 0) {
        // Zrzut do logu (poza pętlą RT!)
        log_histogram(loop_times, N_SAMPLES);
    }
}
```

---

## Czesc VII: Linux PREEMPT_RT — zasady praktyczne

### 7.1 Konfiguracja wątku RT

```c
#include <sched.h>
#include <sys/mman.h>

void setup_rt_thread() {
    // 1. Ustaw SCHED_FIFO (real-time scheduling)
    struct sched_param param = {.sched_priority = 99};
    pthread_setschedparam(pthread_self(), SCHED_FIFO, &param);
    
    // 2. Zablokuj pamięć (uniknij page faults)
    mlockall(MCL_CURRENT | MCL_FUTURE);
    
    // 3. Przypnij do rdzenia CPU
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(0, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);
}
```

### 7.2 Czego NIE robić w wątku RT

```c
// ZŁE:
void rt_loop() {
    // Alokacja pamięci - NIE!
    double* data = malloc(1024);
    
    // Blokujące IO - NIE!
    write(fd, buffer, len);
    
    // Mutex - NIE! (priority inversion)
    std::lock_guard<std::mutex> lock(m);
    
    // Formatowanie stringów - NIE!
    sprintf(buf, "val=%f", val);
}
```

### 7.3 Najczęstsze przyczyny jitteru

| Przyczyna | Wpływ | Jak wykryć |
|-----------|-------|------------|
| IRQ (sieć, dysk) | 10-100 μs | /proc/interrupts |
| Preemption | 1-10 μs | ftrace |
| Page fault | 1-100 ms | mlockall |
| Alokacja/GC | 1-100 ms | Wyeliminować |
| Kolejki (backpressure) | zmienny | Monitoring |

---

## Czesc VIII: EtherCAT a deterministyczność

### 8.1 Cykl EtherCAT

EtherCAT działa cyklicznie:

```c
// Cykl EtherCAT
void ethercat_cycle() {
    // 1. Przygotuj dane do wysłania
    prepare_pdo_data();
    
    // 2. Wyślij i odbierz (deterministycznie!)
    ec_send_receive();
    
    // 3. Przetwórz dane
    process_pdo_data();
}
```

### 8.2 Problem: beat frequencies

Jeśli cykl komunikacji i cykl sterowania **nie są spójne** (np. 1 ms vs 1.1 ms):

```
t=0ms:    Sterowanie 1, Komunikacja 1
t=1ms:    Sterowanie 2, Komunikacja ~0.9
t=2ms:    Sterowanie 3, Komunikacja ~1.8
...
t=10ms:   Sterowanie 11, Komunikacja ~9.9
t=11ms:   Sterowanie 12, Komunikacja ~11

Efekt: "bicie" = aliasing czasowy = niestabilność
```

### 8.3 Zasada praktyczna

> Jeden "master clock" dla cyklu sterowania i cyklu komunikacji jest mniej ryzykowny niż dwa niezależne "zegary" o podobnych częstotliwościach.

---

## Czesc IX: Strategia degradacji

### 9.1 Dlaczego potrzebujemy degradacji

Gdy RT nie wyrabia — "nic nie robić" jest zwykle najgorszą opcją.

### 9.2 Schemat degradacji

| Poziom | Warunek | Akcja |
|--------|---------|-------|
| **INFO** | Pojedynczy missed deadline | Log + zwiększ diagnostykę |
| **WARN** | Seria missed deadline (np. 5/100ms) | Ogranicz rampy/jerk |
| **DEGRADED** | Ciągłe problemy | Zmniejsz prędkość maksymalną |
| **SAFE_STOP** | Krytyczny poziom | Natychmiastowe zatrzymanie |

### 9.3 Kod FSM degradacji

```c
typedef enum {
    STATE_NORMAL,
    STATE_WARNING,
    STATE_DEGRADED,
    STATE_SAFE_STOP
} SystemState;

void update_state(uint32_t missed_count) {
    switch (current_state) {
        case STATE_NORMAL:
            if (missed_count > 10) {
                transition_to(STATE_WARNING);
            }
            break;
        case STATE_WARNING:
            if (missed_count > 50) {
                transition_to(STATE_DEGRADED);
            } else if (missed_count == 0) {
                // Histereza: powrót po 100ms OK
                if (ok_duration > 100000) {
                    transition_to(STATE_NORMAL);
                }
            }
            break;
        case STATE_DEGRADED:
            if (missed_count > 100) {
                transition_to(STATE_SAFE_STOP);
            }
            break;
    }
}
```

---

## Czesc X: Podsumowanie i checklisty

### Checklisty:

- [ ] Masz twardy limit czasu na iterację pętli i reakcję awaryjną
- [ ] Logowanie nie wpływa na RT (asynchroniczne)
- [ ] Sieć i I/O testowane w warunkach worst-case
- [ ] W razie missed deadline masz zdefiniowany tryb degradacji

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Mierz ogony, nie średnią | p99/p99.9 jest kluczowy |
| Izoluj RT od reszty | Pinning, priorytety, mlockall |
| Jeden zegar dla wszystkiego | Unikaj beat frequencies |

---

## Czesc XI: Pytania do dyskusji

1. Dlaczego średnia latencja jest niewystarczająca do oceny determinizmu?
2. Jaka jest różnica między WCET a WCRT i która miara jest krytyczna?
3. Jaką strategię degradacji zastosujesz przy pojedynczym missed deadline, a jaką przy serii?
4. Jak zaprojektujesz logowanie tak, aby nie wpływało na wątek RT?

---

## Czesc XII: Zadania praktyczne

### Zadanie 1: Budżet czasu

Zdefiniuj budżet czasu dla pętli sterowania (tabela t1..t4 + margines).

### Zadanie 2: Metryki runtime

Zaproponuj zestaw metryk (min. 6) i określ gdzie je zbierasz (drive/master).

### Zadanie 3: Scenariusze worst-case

Opisz 3 scenariusze worst-case (telemetria, obciążenie CPU, dropouty) i jak je zasymulujesz.

---

## BONUS: Co wygrywa projekty

W projektach studenckich najczęściej wygrywa ktoś, kto od pierwszego dnia mierzy czas monotoniczny i loguje percentyle, zamiast debugować "wrażeniami".

---

*(Koniec wykladu 2)*
