# Time-Triggered Architecture

## Definicja

**Time-Triggered** to architektura, w której wszystkie akcje są wyzwalane przez upływ czasu, a nie przez zdarzenia zewnętrzne. Wszystko dzieje się w z góry określonych momentach - jest to "orkiestra czasu".

> W time-triggered systemie nie ma "kto pierwszy ten lepszy". Jest "o godzinie 10:00:00.000 wykonaj A, o 10:00:00.100 wykonaj B". Czas jest dyrygentem.

```
┌─────────────────────────────────────────────────────────┐
│                TIME-TRIGGERED SCHEDULE                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Tick 0     Tick 1     Tick 2     Tick 3     Tick 4   │
│  │          │          │          │          │         │
│  ▼          ▼          ▼          ▼          ▼         │
│  ┌────┐     ┌────┐     ┌────┐     ┌────┐     ┌────┐   │
│  │ A  │     │ B  │     │ A  │     │ C  │     │ A  │   │
│  └────┘     └────┘     └────┘     └────┘     └────┘   │
│                                                         │
│  Z góry określone: CO się dzieje i KIEDY.             │
│  Brak niespodzianek. Brak race conditions.            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🌍 Ruch planet

Układ słoneczny działa time-triggered:

```
Ziemia:
- Co 24h: obrót wokół osi
- Co 365 dni: obrót wokół Słońca
- Co ~29 dni: pełnia Księżyca

Wszystko na czas. Żadnych "zdarzeń losowych".
To jest time-triggered kosmosu.
```

### 🌊 Przypływy

Przypływy są przewidywalne na lata do przodu:

```
Tablica przypływów:
01.01: 06:00 - przypływ
01.01: 12:15 - odpływ
01.01: 18:30 - przypływ

Natura działa time-triggered - nie czeka na "zdarzenia".
```

### ❤️ Serce

W zdrowym sercu:

```
Systole: 0.3s (skurcz)
Diastole: 0.5s (rozkurcz)
Cykl: 0.8s = 75 bpm

To jest time-triggered: co 0.8s cykl się powtarza.
Fibrillacja = event-triggered chaos = śmierć.
```

---

## Podobieństwo do systemów informatycznych

### Cron jobs

```bash
# Time-triggered scheduling
0 0 * * *    backup.sh      # Codziennie o północy
0 */6 * * *  sync.sh        # Co 6 godzin
0 0 1 * *    report.sh      # Pierwszy dzień miesiąca

# Z góry określone, przewidywalne, deterministyczne
```

### Animation frames

```javascript
// Time-triggered animation
function animate(timestamp) {
    // Co 16.67ms (60fps)
    drawFrame();
    requestAnimationFrame(animate);
}

// Nie czekamy na "zdarzenie" - czas wyzwala klatkę
```

### Real-time audio

```cpp
// Audio buffer processing co X samples
void processAudio(float* buffer, int samples) {
    // To jest wywoływane CO OKRES
    // Nie "gdy przyjdą dane" ale "co 256 sample"
    for (int i = 0; i < samples; i++) {
        buffer[i] = processSample(buffer[i]);
    }
}
```

---

## Time-Triggered vs Event-Triggered

### Event-Triggered (tradycyjny)

```
Zdarzenia wyzwalają akcje:

Przerwanie A ──► Handler A ──┐
                            ├──► Kto pierwszy?
Przerwanie B ──► Handler B ──┘

Problemy:
- Niedeterministyczne
- Race conditions
- Jitter
- Trudna analiza
```

### Time-Triggered

```
Czas wyzwala akcje:

Timer tick ──► Schedule ──► Task A ──► Task B ──► Task C

Zalety:
- Deterministyczne
- Brak race conditions
- Brak jitter
- Łatwa analiza
```

### Porównanie

| Cecha | Event-Triggered | Time-Triggered |
|-------|-----------------|----------------|
| Wyzwalacz | Zdarzenia | Czas |
| Determinizm | Niski | Wysoki |
| Jitter | Możliwy | Minimalny |
| Złożoność | Niższa | Wyższa |
| Analiza | Trudna | Łatwa |
| Responsywność | Wysoka | Ograniczona |

---

## Architektura Time-Triggered

### Schedule Table

```
┌─────────────────────────────────────────────────────────┐
│                   SCHEDULE TABLE                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Offset   │ Task      │ Action                          │
│  ─────────┼───────────┼────────────────────────────────│
│  0 ms     │ T1        │ Read sensors                    │
│  1 ms     │ T2        │ Process data                    │
│  2 ms     │ T3        │ Update display                  │
│  3 ms     │ T1        │ Read sensors                    │
│  4 ms     │ T4        │ Log data                        │
│  5 ms     │ T1        │ Read sensors                    │
│  ...      │ ...       │ ...                             │
│  10 ms    │           │ CYCLE REPEAT                    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Dispatcher

```c
typedef struct {
    uint32_t offset;
    void (*task)(void);
    uint32_t wcet;
} ScheduleEntry;

ScheduleEntry schedule[] = {
    {0,   read_sensors,   500},  // 0-0.5ms
    {1000, process_data,  800},  // 1-1.8ms
    {2000, update_display, 400}, // 2-2.4ms
    // ...
};

void time_triggered_dispatcher(void) {
    uint32_t cycle_start = get_time();
    uint32_t cycle_time = 10000;  // 10ms cycle

    while (1) {
        // Czekaj na początek cyklu
        while (get_time() < cycle_start);

        // Wykonaj wszystkie taski w schedule
        for (int i = 0; i < sizeof(schedule)/sizeof(schedule[0]); i++) {
            ScheduleEntry* entry = &schedule[i];

            // Czekaj na offset
            while (get_time() < cycle_start + entry->offset);

            // Wykonaj task
            entry->task();

            // Sprawdź overrun
            if (get_time() > cycle_start + entry->offset + entry->wcet) {
                handle_overrun(entry);
            }
        }

        // Następny cykl
        cycle_start += cycle_time;
    }
}
```

---

## Dlaczego Time-Triggered?

### 1. Determinizm

```
Event-triggered:
Przerwanie może przyjść w dowolnym momencie
→ Niedeterministyczne

Time-triggered:
Wszystko dzieje się w określonym momencie
→ Deterministyczne
```

### 2. Brak race conditions

```c
// Event-triggered: RACE CONDITION
volatile int data;
void isr_a(void) { data = 1; }
void isr_b(void) { data = 2; }
// Co jeśli oba przerwania naraz?

// Time-triggered: BRAK RACE
int data;
void task_a(void) { data = 1; }  // Wykonuje się o t=1ms
void task_b(void) { data = 2; }  // Wykonuje się o t=2ms
// Nigdy nie konfliktują
```

### 3. Testowalność

```
Event-triggered:
Musisz przetestować wszystkie kombinacje zdarzeń
= Wykładnicza liczba przypadków

Time-triggered:
Testujesz jeden schedule
= Liniowa liczba przypadków
```

### 4. Analizowalność

```
Event-triggered:
Trudno policzyć WCRT
Nieprzewidywalne interferencje

Time-triggered:
WCRT = suma WCET w schedule
Proste obliczenia
```

---

## Wady Time-Triggered

### 1. Latencja

```
Event przychodzi w t=0.5ms
Schedule ma task o t=2ms
Latencja = 1.5ms (czekanie)

W event-triggered: latencja = ISR time
W time-triggered: latencja ≤ cycle time
```

### 2. Złożoność projektowania

```
Musisz zaprojektować schedule:
- Które taski kiedy
- Jakie WCET
- Jakie zależności
- Margin of safety

To jest NP-hard problem!
```

### 3. Nieefektywność przy rzadkich zdarzeniach

```
Event: przychodzi raz na godzinę
Schedule: sprawdza co 10ms

= 360000 niepotrzebnych sprawdzeń na godzinę
```

### 4. Sztywność

```
Schedule jest statyczny.
Trudno dodać nowy task.
Trudno reagować na zmiany.
```

---

## Time-Triggered w praktyce

### Automotive: TTP/C

```
Time-Triggered Protocol (TTP/C):
- Używany w lotnictwie i automotive
- Wszystkie węzły synchronizowane
- Komunikacja w określonych slotach czasowych
- Deterministyczna latencja
```

### Aerospace: ARINC 653

```c
// ARINC 653 Partitions - Time-Triggered
CREATE_PARTITION(
    .NAME = "FlightControl",
    .PERIOD = 100ms,
    .DURATION = 20ms,  // Gwarantowany slot
    ...
);

CREATE_PARTITION(
    .NAME = "Navigation",
    .PERIOD = 100ms,
    .DURATION = 30ms,
    ...
);

// Każda partycja ma swój slot czasowy
// Brak interferencji między partycjami
```

### Industrial: PLC

```
PLC Cycle:
1. Read inputs (fixed time)
2. Execute logic (fixed time)
3. Write outputs (fixed time)
4. Wait for next cycle

To jest time-triggered!
```

---

## Przykład: Time-Triggered System

```c
// System sterowania silnikiem

typedef enum {
    TASK_READ_SENSORS,
    TASK_FUEL_INJECTION,
    TASK_IGNITION,
    TASK_EXHAUST,
    TASK_DIAGNOSTICS,
    TASK_COUNT
} TaskId;

typedef struct {
    uint32_t offset_us;
    void (*handler)(void);
    uint32_t wcet_us;
    const char* name;
} ScheduleEntry;

// Schedule dla 10ms cycle (silnik 6000 RPM = 100ms/obrót)
ScheduleEntry engine_schedule[] = {
    {0,    read_crank_sensor,    100, "CrankSensor"},
    {100,  read_map_sensor,      100, "MAPSensor"},
    {200,  calculate_fuel,       300, "FuelCalc"},
    {500,  inject_fuel,          200, "FuelInject"},
    {700,  check_ignition_angle, 100, "IgnAngle"},
    {800,  fire_spark,           100, "Ignition"},
    {900,  read_o2_sensor,       100, "O2Sensor"},
    {1000, update_diagnostics,   200, "Diagnostics"},
    // 1200us used, 8800us margin
};

#define CYCLE_TIME_US 10000

void time_triggered_engine_control(void) {
    uint32_t cycle_count = 0;

    while (1) {
        uint32_t cycle_start = get_timestamp_us();

        // Execute schedule
        for (int i = 0; i < sizeof(engine_schedule)/sizeof(engine_schedule[0]); i++) {
            ScheduleEntry* task = &engine_schedule[i];

            // Wait until offset
            while ((get_timestamp_us() - cycle_start) < task->offset_us);

            // Execute task
            uint32_t task_start = get_timestamp_us();
            task->handler();
            uint32_t task_time = get_timestamp_us() - task_start;

            // Monitor timing
            if (task_time > task->wcet_us) {
                log_overrun(task->name, task_time, task->wcet_us);
            }
        }

        // Wait for next cycle
        while ((get_timestamp_us() - cycle_start) < CYCLE_TIME_US);

        cycle_count++;
    }
}
```

---

## Time-Triggered Communication

### TDMA (Time Division Multiple Access)

```
┌─────────────────────────────────────────────────────────┐
│                    TDMA SLOTS                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Time:   0-1ms   1-2ms   2-3ms   3-4ms   4-5ms         │
│          │       │       │       │       │             │
│  Node A: [DATA]  │       │       │       [DATA]        │
│  Node B: │       [DATA]  │       │       │             │
│  Node C: │       │       [DATA]  │       │             │
│  Node D: │       │       │       [DATA]  │             │
│          │       │       │       │       │             │
│  Cycle:  [─── 5ms cycle ───][─── 5ms cycle ───]        │
│                                                         │
│  Każdy node ma swój slot. Brak kolizji.               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Time-Triggered vs Event-Triggered: Decision

### Użyj Time-Triggered gdy:

```
✓ Safety-critical system (lotnictwo, automotive)
✓ Potrzebujesz determinizmu
✓ Potrzebujesz certyfikacji (DO-178C, ISO 26262)
✓ System ma stałą częstotliwość zadań
✓ Potrzebujesz przewidywalnej latencji
```

### Użyj Event-Triggered gdy:

```
✓ System musi reagować natychmiast
✓ Zdarzenia są rzadkie i nieprzewidywalne
✓ Niskie wymagania bezpieczeństwa
✓ Potrzebujesz elastyczności
✓ Prostota implementacji jest ważna
```

---

## Hybrydowe podejście

```
┌─────────────────────────────────────────────────────────┐
│                HYBRID ARCHITECTURE                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Time-Triggered Core:                                   │
│  ┌─────────────────────────────────────────────┐       │
│  │ Control loops, Safety-critical tasks        │       │
│  │ Deterministyczne, certyfikowane             │       │
│  └─────────────────────────────────────────────┘       │
│                                                         │
│  Event-Triggered Periphery:                            │
│  ┌─────────────────────────────────────────────┐       │
│  │ User interface, Logging, Network            │       │
│  │ Responsywne, elastyczne                     │       │
│  └─────────────────────────────────────────────┘       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Pytania do przemyślenia

1. Czy Twój system wymaga determinizmu? Czy time-triggered by pomógł?
2. Jakie są Twoje wymagania latencji? Czy cycle time jest akceptowalny?
3. Czy zdarzenia w Twoim systemie są regularne czy nieprzewidywalne?

---

## Quiz

**Pytanie**: Masz system sterowania dronem:

```
Sensor IMU: 1000 Hz (1ms period)
Motor control: 500 Hz (2ms period)
GPS: 10 Hz (100ms period)
User command: sporadic
```

Zaprojektuj prosty time-triggered schedule.

**Odpowiedź**:

```
Cycle time: 1ms (najkrótszy period)

Schedule (co 1ms):
Slot 0 (0-1ms): IMU read + Motor control
Slot 1 (1-2ms): Motor control only
...powtarzaj...

Co 100ms (co 100 cykli):
Dodatkowo: GPS read

User commands:
- Event-triggered (nie time-critical)
- Lub sprawdzaj flagę w każdym cyklu

Schedule table:
{0, imu_read, 200}
{200, motor_control, 300}
{500, check_gps_flag, 50}  // co 100ms
{550, check_user_cmd, 50}
// Margin: 400us
```

---

## Wskazówka zapamiętywania

> **Time-Triggered = Orkiestra z dyrygentem**
>
> W orkiestrze:
> - Każdy muzyk wie, kiedy grać
> - Dyrygent (czas) wyznacza tempo
> - Nie ma "kto pierwszy ten lepszy"
> - Wszystko jest zaplanowane
>
> Event-triggered = Jazz jam session
> - Muzycy reagują na siebie nawzajem
> - Improwizacja
> - Może być świetnie, może być chaos
>
> Safety-critical system = Orkiestra (time-triggered)
> Kreatywność = Jazz (event-triggered)