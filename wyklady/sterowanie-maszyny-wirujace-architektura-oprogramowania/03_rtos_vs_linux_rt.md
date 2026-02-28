# Wyklad 3: RTOS vs Linux PREEMPT_RT (watki, priorytety, izolacja)

## Czesc I: Wstep teoretyczny — wybor systemu operacyjnego

### 1.1 Dwa swiaty

Prosze wyobrazic sobie dwie skrajne sytuacje:

**Sytuacja A: Prosty kontroler silnika**
```
STM32F4 + FreeRTOS
- Prosty silnik BLDC
- 500 linii kodu
- Jeden watek sterowania
- Brak systemu plikow, sieci, GUI
- Koszt: $5
```

**Sytuacja B: Zlozony system robotyczny**
```
PC z Linux PREEMPT_RT + ROS2
- 6-osiowy manipulator
- Kamery, lidary, czujniki
- Wizja, planowanie ruchu, sterowanie
- GUI, logowanie, siec
- Koszt: $2000
```

Oba sa systemami "RT", ale sa calkowicie inne.

### 1.2 Co to jest RTOS?

**RTOS** (Real-Time Operating System) to system operacyjny zaprojektowany z mysla o determinizmie:

- Minimalny overhead
- Przewidywalne czasy reakcji
- Brak "niespodzianek" (GC, defragmentacja pamieci)

**Przyklady:**
- FreeRTOS
- Zephyr
- RT-Thread
- ChibiOS
- VxWorks
- QNX

**Cechy charakterystyczne:**
- Maly rozmiar (kB zamiast MB)
- Brak lub minimalny stos sieciowy
- Brak GUI
- Wszystko jest "statyczne" (konfiguracja w czasie kompilacji)

### 1.3 Co to jest Linux PREEMPT_RT?

Linux to "zwykly" system operacyjny, ale z dodatkowymi poprawkami:

```
Linux = System ogolnego przeznaczenia + PREEMPT_RT patch
```

**PREEMPT_RT** dodaje:
- Preemptive scheduling (przerywanie procesow)
- Wysokie priority dla krytycznych watkow
- Latencja IRQ w kernelu
- Izolacja przerwan

**Zalety:**
- Pelny ekosystem (sieć, USB, sterowniki, narzedzia)
- Latwy development (standardowe narzedzia)
- Wsparcie sprzetowe (x86, ARM, RISC-V)

**Wady:**
- Nie jest "twardym" RT (sa pewne nieprzewidywalne opoznienia)
- Wymaga dyscypliny w programowaniu

### 1.4 Kiedy ktory wybrac

| Kryterium | RTOS | Linux PREEMPT_RT |
|-----------|------|------------------|
| Jitter | < 1 μs | 10-100 μs |
| Cykl | do 100 kHz | do 10 kHz |
| Zlozonosc systemu | Mala-średnia | Średnia-duza |
| Ekosystem | Ograniczony | Pelny |
| Debugowanie | Trudniejsze | Latwiejsze |
| Sterowniki | Ręczne | Gotowe |

---

## Czesc II: Linux PREEMPT_RT — jak go uzywac poprawnie

### 2.1 Klasyczna pulapka: "Wrzucmy wszystko na Linuxa"

Bardzo czesty błąd:

```c
// Main bez zadnej konfiguracji
int main() {
    while(1) {
        // Sterowanie
        control_loop();
        
        // Logowanie
        log_to_file();
        
        // GUI
        update_gui();
        
        // Siec
        handle_network();
    }
}
```

Efekt: losowy jitter 1-50 ms.

**Dlaczego tak sie dzieje?**

Linux domyslnie:
- Przydziela czas CPU "sprawiedliwie" (fair scheduling)
- Pozwala na przerwania od karty sieciowej, dysku, USB
- Ma background tasks (kswapd, pdflush)
- Moze alokowac pamiec w tle

**Bez izolacji — system jest losowy.**

### 2.2 Watek RT — wzorzec implementacyjny

Profesor zawsze stosuje ten sam wzorzec dla watku sterowania:

```c
#include <sched.h>
#include <sys/mman.h>
#include <pthread.h>

// Struktura danych dla watku RT
struct RTThread {
    pthread_t thread;
    int       cpu_core;
    uint64_t  period_ns;
    // ...
};

// Inicjalizacja watku RT
int rt_thread_init(RTThread* rt, int cpu_core, uint64_t period_ns) {
    // 1. Ustawienie SCHED_FIFO (real-time scheduling)
    struct sched_param param;
    param.sched_priority = 99;  // Najwyzszy priorytet
    pthread_setschedparam(pthread_self(), SCHED_FIFO, &param);
    
    // 2. Memory locking (zabronienie swap)
    mlockall(MCL_CURRENT | MCL_FUTURE);
    
    // 3. Pinning do rdzenia CPU
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(cpu_core, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpu_set_t), &cpuset);
    
    // 4. Statystyki
    rt->cpu_core = cpu_core;
    rt->period_ns = period_ns;
}
```

### 2.3 Petla RT — wzorzec

```c
void* rt_control_loop(void* arg) {
    RTThread* rt = (RTThread*)arg;
    
    // Inicjalizacja
    rt_thread_init(rt, CPU_CORE_0, 1000000);  // 1ms
    
    uint64_t t_next;
    clock_gettime(CLOCK_MONOTONIC, (struct timespec*)&t_next);
    
    while (running) {
        // Czekaj do nastepnego tick
        t_next += rt->period_ns;
        clock_nanosleep(CLOCK_MONOTONIC, TIM_ABSTIME, (struct timespec*)&t_next, NULL);
        
        uint64_t t_start = get_time_ns();
        
        // === WŁASCIWY KOD STEROWANIA ===
        
        // 1. Odczyt wejsc (z bufora, nie blokujaco)
        read_inputs();
        
        // 2. Obliczenia regulatora
        compute_control();
        
        // 3. Zapis wyjsc (do bufora, nie blokujaco)
        write_outputs();
        
        // 4. Publikacja telemetrii (do ring buffer, nie blokujaco)
        publish_telemetry();
        
        // === KONIEC KODU STEROWANIA ===
        
        uint64_t t_end = get_time_ns();
        uint64_t dt = (t_end - t_start) / 1000;  // us
        
        // Statystyki
        if (dt > rt->max_loop_us) rt->max_loop_us = dt;
        if (dt > DEADLINE_US) rt->missed_deadline++;
        
        // Histogram
        int bucket = dt / 10;  // 10 us buckets
        if (bucket < 100) rt->histogram[bucket]++;
    }
    
    return NULL;
}
```

### 2.4 Czego NIE robic w watku RT

```c
// ZŁE - wszystko to blokuje lub alokuje:
void rt_loop() {
    // === NIE ROB TEGO W WĄTKU RT ===
    
    // 1. Alokacja pamieci
    double* data = malloc(1024);        // NIE!
    std::vector<double> v;               // NIE!
    
    // 2. Blokujace IO
    read(fd, buffer, 1024);              // NIE!
    write(logfile, data, len);           // NIE!
    http_post(url, data);                 // NIE!
    
    // 3. Mutexy na wspoldzielonych danych
    std::lock_guard<std::mutex> lock(m); // NIE!
    
    // 4. Formatowanie stringow
    sprintf(buf, "val=%f", value);       // NIE! (malloc wewnatrz)
    std::string s = "val=" + std::to_string(value);  // NIE!
    
    // 5. Logging synchroniczny
    printf("debug: %d\n", val);          // NIE!
}
```

---

## Czesc III: Izolacja — jak oddzielic RT od reszty

### 3.1 Izolacja rdzeni

Jedna z najwazniejszych technik:

```bash
# W Linux (np. w grubie):
isolcpus=2,3,4,5

# Albo dynamicznie:
echo 2 > /sys/devices/system/cpu/cpu2/online
echo 0 > /sys/devices/system/cpu/cpu2/online
```

**Zasada**: Rdzenie dla RT = tylko RT. Reszta systemu nie ma dostepu.

### 3.2 Izolacja IRQ

```bash
# Przeniesienie IRQ do innego rdzenia
echo 1 > /proc/irq/<irq_number>/smp_affinity
```

Typowe IRQ do przeniesienia:
- Karta sieciowa (nie dla RT!)
- Dysk (nie dla RT!)
- USB (nie dla RT!)

### 3.3 Watki non-RT

Reszta systemu (GUI, logowanie, siec) powinna byc na innych rdzeniach:

```c
// Non-RT thread (logowanie)
void* logger_thread(void* arg) {
    // Niskie priority
    setpriority(PRIO_PROCESS, 0, 19);
    
    while(running) {
        // Czekaj na dane w ring buffer
        Sample* s = ringbuffer_read(&rb);
        if (s) {
            // Zapis do pliku - moze byc wolne
            write_to_csv(s);
        } else {
            usleep(1000);  // Czekaj 1ms
        }
    }
}
```

---

## Czesc IV: Mierzenie i weryfikacja determinizmu

### 4.1 Co mierzyc

```c
struct RTMetrics {
    // Czas wykonania
    uint64_t min_us;
    uint64_t max_us;
    uint64_t mean_us;
    uint64_t p50_us;
    uint64_t p95_us;
    uint64_t p99_us;
    uint64_t p99_9_us;
    
    // Deadline
    uint32_t total_iterations;
    uint32_t missed_deadline;
    float    missed_deadline_pct;  // %
    
    // Jitter
    uint64_t jitter_min_us;
    uint64_t jitter_max_us;
    uint64_t jitter_mean_us;
};
```

### 4.2 Jak mierzyc

```c
void measure_and_log(RTThread* rt) {
    // Histogram czasow wykonania
    printf("=== RT Thread Statistics ===\n");
    printf("Iterations: %u\n", rt->total_iterations);
    printf("Min/Max/Mean: %lu / %lu / %lu us\n", 
           rt->min_us, rt->max_us, rt->mean_us);
    printf("P50/P95/P99: %lu / %lu / %lu us\n",
           rt->p50_us, rt->p95_us, rt->p99_us);
    printf("P99.9: %lu us\n", rt->p99_9_us);
    printf("Missed deadline: %u (%.2f%%)\n", 
           rt->missed_deadline, rt->missed_deadline_pct);
    
    // Histogram
    printf("\nHistogram (10us buckets):\n");
    for (int i = 0; i < 50; i++) {
        if (rt->histogram[i] > 0) {
            printf("  %d0-%d0 us: %u\n", i, i+1, rt->histogram[i]);
        }
    }
}
```

### 4.3 Kryteria "firm-RT"

Kiedy mozna powiedziec, ze Linux PREEMPT_RT jest "firm-RT"?

| Kryterium | Wymaganie |
|-----------|-----------|
| P99.9 | < 50% cyklu |
| Missed deadline | < 0.1% |
| Jitter | < 20% cyklu |

Przyklad: cykl = 1 ms
- P99.9 < 500 μs
- Missed deadline < 1 na 1000 iteracji
- Jitter < 200 μs

---

## Czesc V: AI/ML obok RT — bezpieczne granice

### 5.1 Problem

W 2035 inference (wizja, detekcja anomalii) bywa kosztowne obliczeniowo:

```
Watek RT:       |----|----|----|----|----|----|----|----|  (1ms)
Inferencja ML:       |=========================|          (10ms)
```

Co jesli inference nie zdazy?

### 5.2 Zasada architektury

ML nie moze wchodzic do krytycznej petli RT bez barier:

```c
// Architektura bezpieczna:

// Watek RT - czysty, deterministyczny
void rt_loop() {
    // Sterowanie - tylko na podstawie danych lokalnych
    float u = pid_compute(error);
    
    // Nigdy nie czeka na ML!
}

// Osobny watek ML - non-RT
void ml_inference_loop() {
    while(running) {
        // Inference - moze byc wolny
        AnomalyResult r = detect_anomaly(data);
        
        // Zapisz wynik do zmiennej globalnej (bez mutex!)
        // RT watek MOZE to przeczytac, ale NIE CZEKA
        atomic_store(&last_anomalyResult, r);
    }
}

// Watek planowania - non-RT, czyta wyniki ML
void planning_loop() {
    while(running) {
        AnomalyResult r = atomic_load(&last_anomalyResult);
        if (r.is_anomaly) {
            // Reakcja: ogranicz predkosc, alarm
            set_degraded_mode();
        }
    }
}
```

### 5.3 Praktyczne zasady

1. **Inference w osobnym procesie** (non-RT)
2. **Wynik trafia do planowania/nadzoru**, nie do petli momentu/pradu
3. **Timeout z fallbackiem** — jesli inference za dlugo, uzyj poprzedniego wyniku
4. **Rate limiting** — nie wiecej niz N inferencji na sekunde

---

## Czesc VI: Najczestsze przyczyny jitteru w Linux

### 6.1 Ranking przyczyn

| Przyczyna | Wplyw | Czestotliwosc |
|-----------|-------|---------------|
| IRQ (sieć, dysk, USB) | 10-100 μs | Czesta |
| Preemption | 1-10 μs | Bardzo czesta |
| Page fault (alokacja) | 1-100 ms | Rzadka, ale katastrofalna |
| Scheduler "fairness" | 1-50 ms | Czesta |
| GC / JIT | 1-100 ms | Zalezy od jezyka |

### 6.2 Jak wykrywac

```bash
# 1. /proc/interrupts - sprawdz IRQ
cat /proc/interrupts | head -20

# 2. /proc/softirqs - sprawdz soft IRQ
cat /proc/softirqs

# 3. Latencja IRQ
irq_latency /dev/rtc0

# 4. Cyclictest - standardowy test determinizmu
cyclictest -t 1 -n -p 99 -i 1000 -l 10000
```

---

## Czesc VII: Podsumowanie i checklisty

### Checklisty:

- [ ] Masz pomiar rt_loop_us i histogram
- [ ] Masz reakcje na missed deadline (degradacja/safe stop)
- [ ] Masz oddzielone watki: RT vs logowanie/HMI
- [ ] Masz izolacje rdzeni dla watku RT

### Zasady "firm-RT na Linux":

| Technika | Cel |
|----------|-----|
| SCHED_FIFO | Gwarantowany czas CPU |
| Pinning | Izolacja od innych procesow |
| mlockall | Brak page fault |
| Brak IO w petli | Brak blokad |
| Ring buffer | Komunikacja non-blocking |

---

## Czesc VIII: Pytania do dyskusji

1. Jakie sa minimalne wymagania, zeby uznac, ze petla na Linux PREEMPT_RT jest "firm-RT"?
2. Jak zmierzysz i udowodnisz, ze logowanie/telemetria nie wplywa na p99/p99.9 petli?
3. Jak zaprojektujesz fallback, gdy inference ML nie zdazy w czasie?
4. Jakie sa 3 najczestsze przyczyny jitteru w systemach Linux i jak je wykryjesz?

---

## Czesc IX: Zadania praktyczne

### Zadanie 1: Szablon watku RT

Stworz szablon watku RT w C z:
- SCHED_FIFO
- Pinning do rdzenia
- mlockall
- Pomiarem czasu i histogramem
- Reakcja na missed deadline

### Zadanie 2: Izolacja telemetrii

Zaprojektuj pipeline:
- Watek RT produkuje dane do ring buffer
- Watek logger konsumuje dane
- Demonstruj, ze logger nie wplywa na timing RT

### Zadanie 3: Jitter hunt

Przeprowadz serie eksperymentow:
1. Petla bez obciazenia — zmierz baseline
2. Petla + siec (ping flood) — zmierz wplyw
3. Petla + disk I/O (dd if=/dev/zero) — zmierz wplyw
4. Petla + malloc w tle — zmierz wplyw

---

## BONUS: Wolniejsza, ale przewidywalna

W praktyce lepiej miec wolniejsza, ale przewidywalna petle, niz szybsza z losowym jitterem.

Determinizm jest funkcja architektury, nie tylko scheduler'a.

---

*(Koniec wykladu 3)*
