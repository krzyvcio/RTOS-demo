# Partycjonowanie Czasowe (Temporal Partitioning)

## Definicja

Mechanizm izolacji czasowej między zadaniami różnego poziomu krytyczności, zapewniający że zadania w jednej partycji nie mogą wpływać na timing zadań w innej partycji. Fundamentalne dla systemów mixed-criticality.

______________________________________________________________________

## Wizualizacja partycjonowania

```
┌───────────────────────────────────────────────────────────────┐
│              TEMPORAL PARTITIONING - ARINC 653               │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  LINIA CZASU (Major Frame = 100ms):                          │
│                                                               │
│  ├───Partycja A (Krytyczna)───┼───Partycja B───┼───P.C────┤  │
│  │    ASIL-D / DAL-A          │    ASIL-B      │   Idle    │  │
│  │                             │                │           │  │
│  │  ┌─────┐ ┌─────┐ ┌─────┐   │  ┌───┐ ┌───┐  │           │  │
│  │  │ T1  │ │ T2  │ │ T3  │   │  │T4 │ │T5 │  │           │  │
│  │  │20ms │ │15ms │ │10ms │   │  │15ms│ │20ms│  │           │  │
│  │  └─────┘ └─────┘ └─────┘   │  └───┘ └───┘  │           │  │
│  │                             │                │           │  │
│  ├─────────────────────────────┴────────────────┴───────────┤  │
│  │                    MAJOR FRAME (powtarza się)             │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                               │
│  KLUCZOWE: Partycja B nie może "ukraść" czasu Partycji A!     │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Główne koncepcje

| Pojęcie | Opis |
|---------|------|
| **Partycja** | Izolowana jednostka wykonawcza z własnym budżetem czasowym |
| **Major Frame** | Okres powtarzania harmonogramu partycji |
| **Minor Frame** | Podokres wewnątrz major frame |
| **Window** | Przydzielony przedział czasowy dla partycji |
| **Partition Scheduler** | Harmonogram przydzielający CPU partycjom |

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Avionics (ARINC 653)

| Partycja | Poziom krytyczności | Zadania |
|----------|---------------------|---------|
| Flight Control | DAL-A (catastrophic) | Autopilot, FCS |
| Navigation | DAL-B (hazardous) | GPS, INS |
| Communication | DAL-C (major) | Radio, ACARS |
| Entertainment | DAL-E (no effect) | IFE, Wi-Fi |

**Przykłady:** Airbus A350, Boeing 787, F-35 Lightning II

### Automotive (ADAS + Infotainment)

```
┌─────────────────────────────────────────────────────────────┐
│                    HIPERWIZOR + PARTYCJE                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Partycja 1: ADAS (ASIL-D)                           │   │
│  │ - Detekcja przeszkód: 50ms window                   │   │
│  │ - Lane keeping: 20ms window                         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Partycja 2: Powertrain (ASIL-C)                     │   │
│  │ - Engine control: 30ms window                       │   │
│  │ - Transmission: 20ms window                         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Partycja 3: Infotainment (QM)                       │   │
│  │ - Multimedia: pozostały czas                        │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Medical Devices

- **Pacemaker:** Partycja krytyczna (detekcja + stymulacja) vs. telemetria
- **Infusion Pump:** Dawkowanie vs. interface użytkownika
- **MRI Scanner:** Akwizycja danych vs. wizualizacja

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Overrun** | Zadanie przekracza okno czasowe | Watchdog, partition reset |
| **Scheduling Jitter** | Wahania w przełączaniu partycji | Tickless, hardware timer |
| **Context Switch Overhead** | Czas przełączania między partycjami | Minimalizacja, hardware support |
| **Inter-Partition Communication** | Opóźnienia komunikacji | Well-defined APIs |
| **Starvation** | Partycja nie dostaje czasu | Gwarantowane okna czasowe |

______________________________________________________________________

## Komunikacja między partycjami

```
┌─────────────────────────────────────────────────────────────┐
│                INTER-PARTITION COMMUNICATION                │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Partycja A (Wysoka krytyczność)                           │
│  ┌──────────────────┐                                       │
│  │  .Producer..     │──────┐                               │
│  └──────────────────┘      │                               │
│                            ▼                               │
│                     ┌──────────────┐                        │
│                     │  SAMPLING    │  ← Port typu Sampling │
│                     │  PORT        │    (ostatnia wartość) │
│                     └──────────────┘                        │
│                            │                               │
│                            ▼                               │
│  Partycja B (Niższa krytyczność)                           │
│  ┌──────────────────┐                                       │
│  │  .Consumer..     │◄─────┘                               │
│  └──────────────────┘                                       │
│                                                             │
│  RODZAJE PORTÓW (ARINC 653):                                │
│  ├── Sampling Port: ostatnia wartość, bez kolejki         │
│  ├── Queuing Port: FIFO, z kolejką                        │
│  └── Event Port: sygnał zdarzenia                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Implementacje ARINC 653

| System | Platforma | Certyfikacja |
|--------|-----------|--------------|
| **VxWorks 653** | PowerPC, ARM | DO-178C DAL-A |
| **INTEGRITY-178** | PowerPC, ARM | DO-178C DAL-A |
| **LynxOS-178** | x86, PowerPC | DO-178C DAL-A |
| **PikeOS** | ARM, PowerPC | DO-178C, ISO 26262 |
| **QNX** | ARM, x86 | IEC 61508, ISO 26262 |

______________________________________________________________________

## Harmonogram partycji - przykład

```
MAJOR FRAME = 100ms

Okno czasowe:
├── [0-30ms]   Partycja A (Flight Control)
├── [30-50ms]  Partycja B (Navigation)
├── [50-70ms]  Partycja C (Communication)
├── [70-85ms]  Partycja D (Diagnostics)
└── [85-100ms] Idle / Spare

Harmonogram cyklicznie powtarza się co 100ms.

GWARANCJE:
- Partycja A MA ZAWSZE 30ms, niezależnie od innych
- Jeśli Partycja B zawiedzie, A nadal działa
- Izolacja czasowa = izolacja awarii
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Adaptive Partitioning

- Dynamiczne dostosowanie okien czasowych
- Reakcja na zmieniające się obciążenie

### 2. Multi-Core Partitioning

- Partycje rozproszone na wiele rdzeni
- AMP (Asymmetric Multi-Processing)

### 3. Virtualization

- Hipernadzorca (hypervisor) zamiast monolitu
- PikeOS, XtratuM, seL4

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj harmonogram partycji dla systemu automotive:

ZADANIA:
├── Brake Control (ASIL-D): okres 10ms, WCET 3ms
├── Engine Control (ASIL-C): okres 20ms, WCET 5ms
├── ADAS (ASIL-B): okres 50ms, WCET 15ms
└── Infotainment (QM): okres 100ms, WCET 20ms

PYTANIA:
1. Jak pogrupować zadania w partycje?
2. Jaki Major Frame wybrać?
3. Czy wszystkie zadania zmieszczą się?

ROZWIĄZANIE:
1. Partycje:
   - P1 (ASIL-D): Brake Control
   - P2 (ASIL-C): Engine Control
   - P3 (ASIL-B): ADAS
   - P4 (QM): Infotainment

2. Major Frame = LCM(10, 20, 50, 100) = 100ms

3. Sprawdzenie obciążenia:
   - Brake: 100/10 * 3ms = 30ms
   - Engine: 100/20 * 5ms = 25ms
   - ADAS: 100/50 * 15ms = 30ms
   - Info: 100/100 * 20ms = 20ms
   - SUMA: 105ms > 100ms - NIE MIESZCZĄ SIĘ!

   Konieczna optymalizacja lub rozszerzenie Major Frame.
```

______________________________________________________________________

## Pytania kontrolne

1. Czym się różni temporal partitioning od spatial partitioning?
1. Dlaczego ARINC 653 wymaga partycjonowania?
1. Jakie są zalety i wady stałego harmonogramu partycji?
1. Jak komunikacja między partycjami wpływa na determinizm?

______________________________________________________________________

## Literatura

1. ARINC 653 Specification, "Avionics Application Software Standard Interface"
1. ISO 26262-6, "Road vehicles - Functional safety - Part 6: Software"
1. Rushby, "Partitioning in Avionics Architectures" (1999)
1. PikeOS Documentation, "Temporal Partitioning"
