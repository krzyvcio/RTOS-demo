# Izolacja Rdzeni (Core Isolation - AMP/Big-Little)

## Definicja

Techniki izolacji zadań na różnych rdzeniach procesora w celu zapewnienia determinizmu, bezpieczeństwa i optymalizacji zużycia energii. Obejmuje AMP (Asymmetric Multi-Processing) i architekturę Big-LITTLE.

______________________________________________________________________

## Modele wielordzeniowości

```
┌───────────────────────────────────────────────────────────────┐
│                    MODELE WIELORDZENIOWOŚCI                   │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  SMP (Symmetric Multi-Processing):                           │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                       │ │
│  │  │ CPU │ │ CPU │ │ CPU │ │ CPU │   Wspólny OS          │ │
│  │  │  0  │ │  1  │ │  2  │ │  3  │   Wspólna pamięć      │ │
│  │  └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘                       │ │
│  │     └───────┴───────┴───────┘                           │ │
│  │                  │                                       │ │
│  │           ┌──────▼──────┐                                │ │
│  │           │   MEMORY    │                                │ │
│  │           └─────────────┘                                │ │
│  └─────────────────────────────────────────────────────────┘ │
│  Zalety: Proste programowanie                                │
│  Wady: Cache coherency overhead, trudny determinizm          │
│                                                               │
│  AMP (Asymmetric Multi-Processing):                          │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                       │ │
│  │  │ RTOS│ │Linux│ │RTOS │ │Linux│   Różne OS            │ │
│  │  │CPU 0│ │CPU 1│ │CPU 2│ │CPU 3│   Izolowane           │ │
│  │  └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘                       │ │
│  │     │       │       │       │                           │ │
│  │  ┌──▼──┐ ┌──▼──┐ ┌──▼──┐ ┌──▼──┐                       │ │
│  │  │ MEM │ │ MEM │ │ MEM │ │ MEM │   Osobna pamięć       │ │
│  │  │ RT  │ │ LIN │ │ RT  │ │ LIN │   (lub współdzielona) │ │
│  │  └─────┘ └─────┘ └─────┘ └─────┘                       │ │
│  └─────────────────────────────────────────────────────────┘ │
│  Zalety: Pełna izolacja, determinizm                         │
│  Wady: Trudniejsza komunikacja, kompleksowość                │
│                                                               │
│  Big-LITTLE:                                                  │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  ┌─────────────┐ ┌─────────────┐                        │ │
│  │  │   BIG cores │ │ LITTLE cores│                        │ │
│  │  │ ┌───┐┌───┐  │ │ ┌───┐┌───┐  │                        │ │
│  │  │ │A72││A72│  │ │ │A53││A53│  │                        │ │
│  │  │ └───┘└───┘  │ │ └───┘└───┘  │                        │ │
│  │  │ Performance │ │ Efficiency  │                        │ │
│  │  └─────────────┘ └─────────────┘                        │ │
│  └─────────────────────────────────────────────────────────┘ │
│  Zalety: Energooszczędność + wydajność                       │
│  Wady: Migration overhead, cache thrashing                   │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Xilinx Zynq UltraScale+ (MPSoC)

| Rdzeń | Zastosowanie | OS |
|-------|--------------|-----|
| Cortex-A53 #0-1 | Linux, UI | Linux |
| Cortex-A53 #2-3 | RT Linux, Percepcja | RT-Linux |
| Cortex-R5 #0 | Real-time control | FreeRTOS/Zephyr |
| Cortex-R5 #1 | Safety monitor | FreeRTOS/SafeRTOS |
| MicroBlaze | Custom logic | Bare-metal |

### NXP i.MX8

- 4x Cortex-A53 (Linux)
- 2x Cortex-A72 (High-performance)
- 1x Cortex-M4 (RT tasks)
- GPU, VPU dla multimedia

### Automotive (Infotainment + ADAS)

```
┌─────────────────────────────────────────────────────────────┐
│              TYPICAL AUTOMOTIVE SOC                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ CLUSTER 1: Real-Time                                 │   │
│  │ ├── Cortex-R5 #0: Engine Control (ASIL-D)           │   │
│  │ └── Cortex-R5 #1: Brake Control (ASIL-D)            │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ CLUSTER 2: Application                               │   │
│  │ ├── Cortex-A53 #0-1: Linux + ADAS                    │   │
│  │ └── Cortex-A53 #2-3: Infotainment                    │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ CLUSTER 3: Safety                                    │   │
│  │ └── Cortex-M7: Safety Monitor (ASIL-D)              │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Cache coherency** | Rdzenie różnią się cache | Explicit cache management |
| **Memory contention** | Wspólna pamięć | Memory partitioning |
| **Migration overhead** | Przenoszenie zadań Big→LITTLE | CPU affinity |
| **Interrupt routing** | Przerwania na złe rdzenie | IRQ affinity |
| **Synchronization** | IPC między różnymi OS | Mailbox, RPMsg |

______________________________________________________________________

## Komunikacja między rdzeniami (IPC)

```
┌─────────────────────────────────────────────────────────────┐
│                    INTER-CORE COMMUNICATION                 │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  MECHANIZMY:                                                │
│                                                             │
│  1. Shared Memory + Mailbox                                 │
│     ┌─────────┐         ┌─────────┐                        │
│     │  CPU A  │────┬───▶│  CPU B  │                        │
│     │ (RTOS)  │    │    │ (Linux) │                        │
│     └─────────┘    │    └─────────┘                        │
│                    ▼                                        │
│             ┌─────────────┐                                 │
│             │ SHARED MEM  │                                 │
│             │ + Mailbox   │                                 │
│             └─────────────┘                                 │
│                                                             │
│  2. RPMsg (Remote Processor Messaging)                      │
│     ├── OpenAMP framework                                   │
│     ├── VirtIO queues                                       │
│     └── Standardized API                                    │
│                                                             │
│  3. Hardware Mailbox                                        │
│     ├── IP block w SoC                                      │
│     ├── Interrupt-based notification                        │
│     └── Low latency                                         │
│                                                             │
│  4. Ring Buffer (lock-free)                                 │
│     ├── Single Producer, Single Consumer                    │
│     ├── Zero-copy                                           │
│     └── Minimal overhead                                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## CPU Affinity i Izolacja w Linux

```bash
# Izolacja CPU dla zadań RT
# /proc/cmdline:
isolcpus=2,3 nohz_full=2,3 rcu_nocbs=2,3

# Przypisanie zadania do konkretnego CPU
taskset -c 2-3 ./rt_application

# Sprawdzenie affinity
taskset -p <pid>

# IRQ affinity - przerwania na inne CPU
echo 0-1 > /proc/irq/45/smp_affinity
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Heterogeneous Compute

- CPU + GPU + NPU + FPGA
- OpenCL, CUDA dla RT
- Offload obliczeń do akceleratorów

### 2. Safety Islands

- Dedykowane rdzenie safety
- Hardware isolation
- Dual-core lockstep

### 3. Virtualization

- Hypervisor na multicore
- VM per core/cluster
- Mixed-criticality na jednym SoC

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj architekturę dla systemu robotycznego:

WYMAGANIA:
├── Motor control: 1 kHz, WCET 200 µs, ASIL-D
├── Sensor fusion: 500 Hz, WCET 1 ms, ASIL-B
├── Planning: 20 Hz, WCET 30 ms, QM
├── UI: Best effort, QM
└── Platform: 4x Cortex-A53 + 2x Cortex-R5

PYTANIA:
1. Jak rozdzielić zadania na rdzenie?
2. Jakie OS na każdym klastrze?
3. Jak zapewnić komunikację?

ROZWIĄZANIE:
1. Rozdział:
   - Cortex-R5 #0: Motor control (RT, izolowany)
   - Cortex-R5 #1: Safety monitor (RT, izolowany)
   - Cortex-A53 #0: Sensor fusion (RT-Linux)
   - Cortex-A53 #1-2: Planning (Linux)
   - Cortex-A53 #3: UI (Linux)

2. OS:
   - R5: FreeRTOS (deterministyczny, mały footprint)
   - A53: Linux + PREEMPT_RT

3. Komunikacja:
   - R5 ↔ A53: RPMsg (OpenAMP)
   - A53 wewnętrznie: POSIX queues
```

______________________________________________________________________

## Pytania kontrolne

1. Czym różni się AMP od SMP?
1. Jakie są zalety architektury Big-LITTLE?
1. Jak zapewnić determinizm na rdzeniu z Linux?
1. Jakie mechanizmy IPC stosuje się między rdzeniami?

______________________________________________________________________

## Literatura

1. ARM, "Big-LITTLE Technology"
1. Xilinx, "Zynq UltraScale+ Device Technical Reference Manual"
1. NXP, "i.MX 8 Applications Processor Reference Manual"
1. OpenAMP, "Inter-Processor Communication Framework"
