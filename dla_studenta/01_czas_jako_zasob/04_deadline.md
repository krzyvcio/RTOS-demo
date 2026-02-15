# Deadline

## Definicja

**Deadline** to bezwzględny termin wykonania zadania. W systemach czasu rzeczywistego deadline jest wymaganiem, nie sugestią.

> Deadline to linia, której nie wolno przekroczyć. Przekroczenie = awaria, nie "trochę za późno".

```
Zadanie start ────► Wykonanie ────► Deadline
     │                                    │
     │◄──────────────────────────────────►│
              Available time

     │◄─────────────────►│
        Execution time
                        │◄──►│
                         OK!
```

---

## Analogia do przyrody

### 🌅 Świt i zmierzch

Rośliny muszą wykonać fotosyntezę między świtem a zmierzchem. To jest ich deadline - 12-16 godzin na "zadanie". Gdyby przekraczały deadline, nie przetrwałyby nocy bez energii.

### 🦠 Odporność immunologiczna

Gdy wirus atakuje, układ odpornościowy ma deadline na wyprodukowanie przeciwciał. Jeśli przekroczy deadline - organizm przegrywa. To dlatego niektóre choroby są śmiertelne: system immunologiczny nie zdążył.

### 🐋 Wieloryb nurkujący

Wieloryb może nurkować na 90 minut. To jego deadline na znalezienie pożywienia i powrót na powierzchnię. Przekroczenie = śmierć przez uduszenie.

---

## Podobieństwo do systemów informatycznych

### SLA (Service Level Agreement)

```
"99.9% zapytań musi zostać obsłużonych w ciągu 200ms"
```

To jest deadline w świecie IT. Różnica: w RTOS deadline jest **bezwzględne**, w SLA może być **procentowe**.

### CI/CD Pipeline

```
Commit ──► Build ──► Test ──► Deploy
    │                             │
    │◄────────── 10 min ─────────►│
              Pipeline deadline
```

Jeśli pipeline trwa > 10 min, blokuje developerów. To jest "soft deadline" w IT.

### Timeout

```python
response = requests.get(url, timeout=5.0)  # 5s deadline
```

Timeout to najprostsza forma deadline w kodzie. Po przekroczeniu - błąd, nie czekamy dalej.

---

## Rodzaje deadline

### Hard Deadline

Przekroczenie = **katastrofa** (śmierć, zniszczenie, awaria systemu)

```
Przykłady:
- Airbag: 30ms od wykrycia zderzenia
- Sterownik silnika: zapłon przed GMP
- Zatrzymanie pociągu: przed przeszkodą
```

### Firm Deadline

Przekroczenie = **znaczna utrata jakości/usługi**, ale system działa

```
Przykłady:
- Video streaming: klatka nie zdążyła → stutter
- Audio processing: przerwa w dźwięku
- Trading: stracona okazja
```

### Soft Deadline

Przekroczenie = **spadek wydajności**, system nadal użyteczny

```
Przykłady:
- Odświeżenie UI: opóźniona animacja
- Logowanie: wpis pojawi się później
- Analytics: dane przetworzone z opóźnieniem
```

---

## Deadline w RTOS - graficznie

### Task z deadline

```
         Release time
              │
              ▼
    ┌─────────────────────────────────┐
    │         TASK                    │
    │                                 │
    └─────────────────────────────────┘
              │                       │
              │◄──── Execution ──────►│
              │                       │
              │◄───────── Deadline ───┼────►│
                                      │
                                      ▼
                                   Deadline
                                    point
```

### Deadline miss

```
    ┌─────────────────────────────────┐
    │         TASK                    │
    │                                 │
    └─────────────────────────────────────────┐
              │                                │
              │◄─────────────────── Deadline ──┼──►│
              │                                │
              │◄──────────── Execution ────────┼──►│
                                               │
                                          MISS! ▼
```

---

## Dlaczego deadline są problemem?

### Problem 1: Nieznany czas wykonania

```c
void process_data(int* data, int count) {
    // Ile to potrwa?
    // Zależy od danych!
    for (int i = 0; i < count; i++) {
        if (data[i] > threshold) {
            complex_calculation(data[i]);  // długo?
        } else {
            simple_update(data[i]);  // szybko?
        }
    }
}
```

### Problem 2: Interferencja od innych tasków

```
Task A (high priority) ───►│     │◄── Deadline
                           │     │
Task B (low priority) ──►──┴─────┴──►
                              │
                              └──► Task A preemptuje B
                                   B może miss deadline!
```

### Problem 3: Zależności między taskami

```
Task A ──► Task B ──► Task C ──► Deadline
   │          │          │
   └──────────┴──────────┘
          Każde opóźnienie propaguje!
```

---

## Jak zagwarantować deadline?

### WCET Analysis (Worst Case Execution Time)

```c
// Musisz znać najgorszy możliwy czas wykonania!
// WCET tasku musi być < deadline
```

Więcej o WCET w osobnym pliku.

### Schedulability Analysis

```
Dla N tasków z okresami Ti i czasami wykonania Ci:

Σ(Ci/Ti) ≤ Umax

Gdzie Umax zależy od algorytmu schedulingu:
- RMS (Rate Monotonic): Umax = N(2^(1/N) - 1)
- EDF (Earliest Deadline First): Umax = 1.0
```

### Resource Reservation

```c
// Zarezerwuj czas na task
task_create("critical",
            priority=HIGH,
            budget=2ms,      // max execution time
            period=10ms,     // deadline
            deadline=10ms);
```

---

## Strategie radzenia sobie z deadline

### 1. Over-provisioning

```
WCET = 5ms
Dostępny czas = 10ms
Margin = 100%

Bezpiecznie, ale marnotrawstwo zasobów.
```

### 2. Monitoring i recovery

```c
void task_with_monitoring(void) {
    start_time = get_time();

    process_data();

    elapsed = get_time() - start_time;
    if (elapsed > deadline * 0.9) {
        // 90% deadline zużyte!
        log_warning("Approaching deadline");
    }
    if (elapsed > deadline) {
        // Deadline miss!
        emergency_recovery();
    }
}
```

### 3. Graceful degradation

```c
void video_decoder(void) {
    if (time_remaining() < estimated_decode_time) {
        // Nie zdążymy w pełnej jakości!
        decode_at_lower_quality();  // Szybciej, gorzej
        // Ale deadline zachowane!
    } else {
        decode_full_quality();
    }
}
```

### 4. Task shedding

```c
void overloaded_system(void) {
    if (cpu_load > 90%) {
        // Odrzuć mało ważne taski
        skip_low_priority_tasks();
        // Ratuj krytyczne deadline
    }
}
```

---

## Jak świat radzi sobie z deadline?

### Automotive: Brake-by-wire

```
Sensor ──► Processing ──► Brake actuator
    │                          │
    └──────── 30ms ────────────┘
              deadline

Jeśli 30ms nie wystarcza → hydraulic backup
Triple redundancy + fail-safe
```

### Aerospace: Fly-by-wire

```
Control input ──► Flight computer ──► Actuator
     │                                      │
     └──────────── 50ms ────────────────────┘
                   deadline

Jeśli deadline miss → drugi komputer przejmuje
Triple modular redundancy
```

### Industrial: PLC (Programmable Logic Controller)

```
Input scan ──► Program execution ──► Output scan
     │                                     │
     └────────────── Cycle time ───────────┘
                   (zazwyczaj 1-100ms)

Jeśli program execution zbyt długo → watchdog reset
```

---

## Deadline w różnych domenach

| Domena | Typowy deadline | Hard/Soft |
|--------|-----------------|-----------|
| Airbag | 30ms | Hard |
| ABS | 5-10ms | Hard |
| Engine control | 0.1-1ms | Hard |
| Video codec | 33ms (30fps) | Firm |
| Audio | 5-20ms | Firm |
| Web server | 100-500ms | Soft |

---

## Pytania do przemyślenia

1. Jakie deadline mają taski w Twoim systemie? Hard czy soft?
2. Co się dzieje, gdy deadline jest przekroczone? Czy system ma recovery?
3. Czy znasz WCET wszystkich krytycznych tasków?

---

## Quiz

**Pytanie**: Masz system z trzema taskami:
- Task A: period=10ms, WCET=3ms
- Task B: period=20ms, WCET=4ms
- Task C: period=50ms, WCET=10ms

Czy system jest schedulable pod RMS?

**Odpowiedź**:

```
Utilization = Σ(Ci/Ti)
            = 3/10 + 4/20 + 10/50
            = 0.30 + 0.20 + 0.20
            = 0.70

RMS bound for N=3: Umax = 3(2^(1/3) - 1) = 0.78

0.70 < 0.78 ✓

System jest schedulable! (zakładając, że priorytety są zgodne z RMS)
```

---

## Wskazówka zapamiętywania

> **Deadline = Linia graniczna czasu**
>
> Wyobraź sobie pociąg jadący na stację:
> - Musi się zatrzymać PRZED semaforem
> - Przekroczenie linii = wypadek
> - To nie jest "sugestia" - to fizyka
>
> W RTOS: deadline to semafor. Przejechanie = katastrofa.