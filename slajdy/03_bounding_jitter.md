# Bounding Jitter (Ograniczenie Jittera)

## Definicja

Maksymalna odchyłka czasu wykonania zadania od nominalnego czasu rozpoczęcia lub zakończenia. Jitter jest kluczowym parametrem w systemach czasu rzeczywistego, gdzie determinizm jest ważniejszy niż średnia wydajność.

______________________________________________________________________

## Wizualizacja jittera

```
┌───────────────────────────────────────────────────────────────┐
│                    JITTER W SYSTEMACH RT                      │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  IDEALNY (Jitter = 0):                                        │
│  ├──T1──┼──T2──┼──T3──┼──T4──┼──T5──┤                        │
│    10ms   10ms   10ms   10ms   10ms                           │
│                                                               │
│  RZECZYWISTY (Jitter > 0):                                    │
│  ├──T1──┼─T2─┼───T3───┼T4┼───T5───┤                         │
│    10ms   8ms   12ms   6ms   14ms                             │
│           ↑          ↑     ↑                                  │
│         jitter    jitter  jitter                              │
│                                                               │
│  DEFINICJA:                                                   │
│    Jitter = |t_rzeczywisty - t_nominalny|                    │
│    Max Jitter = max(jitter) dla wszystkich okresów           │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Typy jittera

| Typ | Opis | Wpływ |
|-----|------|-------|
| **Release Jitter** | Opóźnienie startu zadania | Kaskadowe opóźnienia |
| **Completion Jitter** | Wahania czasu zakończenia | Niestabilność output |
| **Inter-arrival Jitter** | Wahania okresu przyjścia | Nieprzewidywalne obciążenie |
| **Execution Time Jitter** | Wahania czasu wykonania | Trudności w analizie WCET |

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Aerospace (DO-178C)

| System | Wymagany Max Jitter | Konsekwencja przekroczenia |
|--------|---------------------|---------------------------|
| Fly-by-Wire | < 1ms | Utrata kontroli lotu |
| Autopilot | < 5ms | Oscylacje trajektorii |
| Instrumentacja | < 10ms | Błędne wskazania |
| FCS (Flight Control) | < 100μs | Instabilność |

**Przykład:** Boeing 787 Dreamliner - jitter < 500μs dla systemów krytycznych

### Audio/Video Professional

- **Audio:** Jitter < 1μs dla 192kHz/24bit
- **Video:** Jitter < 1 frame time (16.67ms @ 60fps)
- **Broadcasting:** Synchronizacja SMPTE timecode

### Komunikacja przemysłowa

- **EtherCAT:** Jitter < 100ns dla cycle time 250μs
- **PROFINET IRT:** Jitter < 1μs
- **EtherNet/IP:** Jitter < 1ms

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Konsekwencje |
|------------|------|--------------|
| **Kaskadowy** | Jitter jednego zadania wpływa na inne | System-wide timing failure |
| **Akumulacja** | Jitter narasta w długich łańcuchach | Deadline miss |
| **Trudny do pomiaru** | Wymaga specjalistycznego sprzętu | Ukryte problemy |
| **Nieliniowy** | Małe zmiany mogą dać duży jitter | Nieprzewidywalność |
| **Zależny od obciążenia** | Jitter rośnie z obciążeniem systemu | Degradacja w peak loads |

______________________________________________________________________

## Źródła jittera

```
┌─────────────────────────────────────────────────────────────┐
│                    ŹRÓDŁA JITTERA                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  SPRZĘTOWE:                                                 │
│  ├── Cache misses (L1/L2/L3)                               │
│  ├── Pipeline stalls                                        │
│  ├── Branch misprediction                                   │
│  ├── Memory bus contention                                  │
│  ├── Interrupt latency                                      │
│  └── DMA transfers                                          │
│                                                             │
│  SYSTEMOWE:                                                 │
│  ├── Preemption przez inne zadania                         │
│  ├── Scheduler overhead                                     │
│  ├── Context switch time                                    │
│  ├── Interrupt handling                                     │
│  └── Power management (DVFS)                                │
│                                                             │
│  APLIKACYJNE:                                               │
│  ├── Zmienne ścieżki wykonania (if/else, pętle)            │
│  ├── Dynamiczna alokacja pamięci                           │
│  ├── I/O operations                                         │
│  └── System calls                                           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Metody ograniczania jittera

### 1. Tickless Kernel

```c
// Zamiast regularnych ticków co 1ms:
// Tick tylko gdy potrzebny (next timer event)
CONFIG_NO_HZ=y
CONFIG_NO_HZ_IDLE=y
```

### 2. CPU Isolation

```bash
# Linux kernel parameters
isolcpus=2,3    # Izoluj CPU 2 i 3 dla zadań RT
nohz_full=2,3   # Wyłącz tick na izolowanych CPU
rcu_nocbs=2,3   # Przenieś RCU callbacks
```

### 3. Memory Locking

```c
#include <sys/mman.h>

// Zablokuj pamięć w RAM (brak page fault)
mlockall(MCL_CURRENT | MCL_FUTURE);

// Prealokuj stack
char stack[STACK_SIZE];
memset(stack, 0, STACK_SIZE);
```

### 4. Priority Boost

```c
// Wyłącz preemption w sekcji krytycznej
sched_lock();
// ... kod z minimalnym jitterem ...
sched_unlock();
```

### 5. Cache Warming

```c
// Pre-fetch danych do cache
__builtin_prefetch(data);
// Lub ręczne "rozgrzanie" cache
volatile int dummy = critical_data[0];
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Time-Deterministic Architectures

- Procesory bez cache (lub z locked cache)
- Predykcyjne pamięci
- Hardware support dla RT

### 2. Static Scheduling

- Time-Triggered Architecture (TTA)
- Zerowy jitter przez statyczny harmonogram
- TTP/C, FlexRay

### 3. Hardware Timestamping

- IEEE 1588 PTP (Precision Time Protocol)
- Hardware-assisted timestamping
- Sub-microsecond synchronization

______________________________________________________________________

## Narzędzia pomiaru jittera

| Narzędzie | Zastosowanie | Precyzja |
|-----------|--------------|----------|
| **SymTA/S** | Analiza timing | Model-based |
| **Chronos** | WCET analysis | Static analysis |
| **ftrace** | Linux kernel tracing | Sub-μs |
| **perf** | Performance counters | Hardware counters |
| **LatencyTOP** | Latency sources | System-wide |
| **oscilloscope** | Hardware measurement | ns precision |

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
System sterowania silnikiem wymaga okresu 10ms z jitterem < 100μs.

POMIARY (czas startu zadania w ms):
  0.0, 10.05, 20.02, 30.15, 40.08, 50.12, 60.03, 70.18

PYTANIA:
1. Oblicz jitter dla każdego okresu
2. Czy wymaganie jest spełnione?
3. Zidentyfikuj potencjalne źródła jittera

ROZWIĄZANIE:
1. Jitter (μs): 0, 50, 20, 150, 80, 120, 30, 180
2. NIE - max jitter = 180μs > 100μs
3. Możliwe źródła:
   - Okresy 30.15 i 70.18 mają duży jitter
   - Może to być interferencja z innymi zadaniami
   - Sprawdzić: cache misses, interrupts, inne zadania
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego jitter jest ważniejszy od średniego czasu odpowiedzi w systemach RT?
1. Jak cache wpływa na jitter?
1. Jakie jest znaczenie tickless kernel dla jittera?
1. Jak zmierzyć jitter w systemie produkcyjnym?

______________________________________________________________________

## Literatura

1. DO-178C, "Software Considerations in Airborne Systems"
1. ISO 26262-6, "Automotive Software Safety"
1. Buttazzo, "Hard Real-Time Computing Systems"
1. Klein et al., "On the Measurement of Operating System Latency"
