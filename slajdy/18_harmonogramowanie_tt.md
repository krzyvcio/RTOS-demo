# Harmonogramowanie Time-Triggered

## Definicja

Paradygmat harmonogramowania, w którym wszystkie zadania są uruchamiane w z góry określonych momentach czasowych, w przeciwieństwie do podejścia event-triggered, gdzie zadania są uruchamiane w odpowiedzi na zdarzenia.

______________________________________________________________________

## Time-Triggered vs Event-Triggered

```
┌───────────────────────────────────────────────────────────────┐
│           TIME-TRIGGERED vs EVENT-TRIGGERED                   │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  EVENT-TRIGGERED:                                             │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Zdarzenia:  ▼      ▼▼           ▼      ▼▼▼               │ │
│  │ Timeline:   ─┼──────┼────────────┼──────┼─────────────   │ │
│  │ Task A:     ──┬────┬────────────┬────┬────────────────   │ │
│  │               └──┘  └──┘         └──┘  └──┘              │ │
│  │                                                         │ │
│  │ CECHY:                                                  │ │
│  │ ├── Reakcja na zdarzenia                               │ │
│  │ ├── Nieprzewidywalne obciążenie                        │ │
│  │ ├── Jitter zależy od zdarzeń                           │ │
│  │ └── Trudna analiza WCET                                │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  TIME-TRIGGERED:                                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Timeline:   ─┼────┼────┼────┼────┼────┼────┼────┼───    │ │
│  │ Slot:        0    1    2    3    4    5    6    7       │ │
│  │ Task A:     ──┬─────────┬─────────┬─────────┬───────    │ │
│  │              └──┘       └──┘       └──┘       └──┘       │ │
│  │ Task B:     ──────┬─────────┬─────────┬─────────────    │ │
│  │                    └──┘       └──┘       └──┘            │ │
│  │                                                         │ │
│  │ CECHY:                                                  │ │
│  │ ├── Sztywny harmonogram                                 │ │
│  │ ├── Przewidywalne obciążenie                            │ │
│  │ ├── Zero jitter strukturalny                            │ │
│  │ └── Łatwa analiza WCET/WCRT                             │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## TTP/C i FlexRay

```
┌─────────────────────────────────────────────────────────────┐
│                    TTP/C (Time-Triggered Protocol)          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ARCHITEKTURA:                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    TDMA Round                        │   │
│  │  ┌──────┬──────┬──────┬──────┬──────┬──────┐       │   │
│  │  │ Node │ Node │ Node │ Node │ Node │ Node │       │   │
│  │  │  1   │  2   │  3   │  4   │  5   │  6   │       │   │
│  │  └──────┴──────┴──────┴──────┴──────┴──────┘       │   │
│  │       ↑                                              │   │
│  │    Każdy node ma swój slot czasowy                   │   │
│  │    Deterministyczny dostęp do szyny                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ZALETY:                                                    │
│  ├── Deterministyczny dostęp do szyny                     │
│  ├── Zero kolizji                                          │
│  ├── Łatwa certyfikacja (DO-178C, ISO 26262)              │
│  └── Fault tolerance przez redundancję                    │
│                                                             │
│  ZASTOSOWANIA:                                              │
│  ├── Aerospace (fly-by-wire)                              │
│  ├── Automotive (x-by-wire)                               │
│  └── Industrial control                                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    FlexRay (Automotive)                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  CYKL KOMUNIKACYJNY:                                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    Communication Cycle              │   │
│  │  ┌─────────────────────────┬─────────────────────┐ │   │
│  │  │   STATIC SEGMENT        │   DYNAMIC SEGMENT   │ │   │
│  │  │  ┌───┬───┬───┬───┬───┐ │  ┌─────┬─────┬────┐ │ │   │
│  │  │  │S1 │S2 │S3 │S4 │S5 │ │  │minislot│minislot│ │ │   │
│  │  │  └───┴───┴───┴───┴───┘ │  └─────┴─────┴────┘ │ │   │
│  │  │   Time-Triggered       │   Event-Triggered   │ │   │
│  │  └─────────────────────────┴─────────────────────┘ │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  PARAMETRY:                                                 │
│  ├── Data rate: 10 Mbps                                    │
│  ├── Cycle: 1-16 ms                                        │
│  ├── Static slots: 0-1023                                  │
│  └── Dual channel (redundancy)                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Aerospace (Fly-by-Wire)

| System | Okres | Zastosowanie |
|--------|-------|--------------|
| Flight Control | 5 ms | Sztywny harmonogram |
| Sensor Acquisition | 2 ms | Time-triggered |
| Actuator Command | 5 ms | Deterministyczne wyjście |

### Automotive (X-by-Wire)

- **Steer-by-Wire:** FlexRay, TT-CAN
- **Brake-by-Wire:** TTP/C, deterministic Ethernet
- **Chassis Control:** Mixed static/dynamic

### Industrial (Real-Time Ethernet)

- **EtherCAT:** Distributed clocks
- **PROFINET IRT:** Isochronous real-time
- **EtherNet/IP:** CIP Motion

______________________________________________________________________

## Implementacja Time-Triggered

### Harmonogram TT

```c
// Definicja harmonogramu time-triggered
typedef struct {
    uint32_t start_time;    // Start slotu (µs)
    uint32_t duration;      // Czas trwania (µs)
    void (*task_func)(void);// Funkcja zadania
    const char *name;       // Nazwa
} tt_slot_t;

// Harmonogram (Major Frame = 10 ms)
static const tt_slot_t schedule[] = {
    {    0,  500, motor_control,   "Motor"   },   // 0-0.5 ms
    {  500,  300, sensor_read,     "Sensor"  },   // 0.5-0.8 ms
    {  800,  200, safety_check,    "Safety"  },   // 0.8-1.0 ms
    { 1000, 1000, communication,   "Comm"    },   // 1.0-2.0 ms
    { 2000, 5000, planning,        "Plan"    },   // 2.0-7.0 ms
    { 7000, 2000, diagnostics,     "Diag"    },   // 7.0-9.0 ms
    { 9000, 1000, idle,            "Idle"    },   // 9.0-10.0 ms
};

#define SCHEDULE_SIZE (sizeof(schedule) / sizeof(schedule[0]))
#define MAJOR_FRAME_US 10000

// Dispatcher time-triggered
void tt_dispatcher(void) {
    static uint32_t current_slot = 0;
    uint32_t cycle_start = get_time_us();
    uint32_t cycle_time;

    while (1) {
        cycle_time = get_time_us() - cycle_start;

        // Sprawdź czy jesteśmy w odpowiednim slocie
        if (current_slot < SCHEDULE_SIZE) {
            const tt_slot_t *slot = &schedule[current_slot];

            if (cycle_time >= slot->start_time) {
                // Wykonaj zadanie
                uint32_t task_start = get_time_us();
                slot->task_func();
                uint32_t task_time = get_time_us() - task_start;

                // Sprawdź deadline
                if (task_time > slot->duration) {
                    handle_overrun(slot->name, task_time, slot->duration);
                }

                current_slot++;
            }
        }

        // Koniec cyklu
        if (cycle_time >= MAJOR_FRAME_US) {
            current_slot = 0;
            cycle_start = get_time_us();
        }
    }
}
```

### Time-Triggered na Timerze

```c
// Hardware timer interrupt (co 100 µs)
void TIM2_IRQHandler(void) {
    static uint32_t tick = 0;

    if (TIM2->SR & TIM_SR_UIF) {
        TIM2->SR &= ~TIM_SR_UIF;

        // Time-triggered dispatch
        switch (tick) {
            case 0:   // Co 1 ms
                trigger_task(&motor_control_task);
                break;
            case 5:   // Co 5 ms (offset 500 µs)
                trigger_task(&sensor_task);
                break;
            case 50:  // Co 50 ms
                trigger_task(&planning_task);
                break;
        }

        tick = (tick + 1) % 1000;  // 100 ms cycle
    }
}
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Overrun** | Zadanie przekracza slot | Monitoring, fallback |
| **Clock drift** | Rozsynchronizacja | Global time sync (PTP) |
| **Rigid schedule** | Brak elastyczności | Hybrid (static + dynamic) |
| **Wasted slots** | Nieużywany czas | Padding lub low-priority tasks |
| **Complexity** | Trudna zmiana harmonogramu | Tools do generowania schedule |

______________________________________________________________________

## Kierunki rozwoju

### 1. TSN (Time-Sensitive Networking)

- Ethernet z time-triggered
- IEEE 802.1Qbv (Time-Aware Shaper)
- Gate Control Lists

### 2. Mixed-Criticality TT

- Static dla krytycznych
- Dynamic dla pozostałych
- ARINC 653 style

### 3. Adaptive TT

- Dynamiczne dostosowanie harmonogramu
- Machine learning do optymalizacji

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj harmonogram time-triggered dla systemu sterowania:

ZADANIA:
├── Motor Control: okres 1 ms, WCET 200 µs
├── Sensor Read: okres 2 ms, WCET 150 µs
├── Safety Check: okres 5 ms, WCET 100 µs
├── Communication: okres 10 ms, WCET 500 µs
└── Planning: okres 20 ms, WCET 2 ms

PYTANIA:
1. Jaki Major Frame wybrać?
2. Jak rozmieścić zadania w slocie?
3. Jaki jest utilization?

ROZWIĄZANIE:
1. Major Frame = LCM(1, 2, 5, 10, 20) = 20 ms

2. Harmonogram:
   Slot 0-0.2 ms: Motor Control (cykl 1 ms)
   Slot 0.3-0.45 ms: Sensor Read (cykl 2 ms)
   Slot 0.5-0.6 ms: Safety Check (cykl 5 ms)
   Slot 1-1.5 ms: Communication (cykl 10 ms)
   Slot 2-4 ms: Planning (cykl 20 ms)

   W każdym 1 ms:
   - Motor Control
   - Co 2 ms: Sensor Read
   - Co 5 ms: Safety Check
   - Co 10 ms: Communication
   - Co 20 ms: Planning

3. Utilization:
   Motor: 0.2/1 = 20%
   Sensor: 0.15/2 = 7.5%
   Safety: 0.1/5 = 2%
   Comm: 0.5/10 = 5%
   Plan: 2/20 = 10%
   SUMA: 44.5%
```

______________________________________________________________________

## Pytania kontrolne

1. Jakie są zalety time-triggered względem event-triggered?
1. Czym różni się TTP/C od FlexRay?
1. Jak zapewnić synchronizację czasu w systemie TT?
1. Kiedy warto zastosować podejście hybrydowe?

______________________________________________________________________

## Literatura

1. Kopetz, "Real-Time Systems: Design Principles for Distributed Embedded Applications"
1. FlexRay Consortium, "FlexRay Protocol Specification"
1. TTP Group, "Time-Triggered Protocol TTP/C"
1. IEEE 802.1Qbv, "Time-Aware Shaper"
