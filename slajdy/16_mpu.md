# MPU - Memory Protection Unit

## Definicja

Sprzętowy mechanizm ochrony pamięci, który pozwala na kontrolę dostępu do regionów pamięci per-task. Umożliwia izolację zadań i wykrywanie błędów dostępu do pamięci w czasie rzeczywistym.

______________________________________________________________________

## Architektura MPU

```
┌───────────────────────────────────────────────────────────────┐
│                    MPU ARCHITECTURE                           │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  MEMORY MAP:                                                  │
│  ┌───────────────────────────────────────────────────────┐   │
│  │ 0x0000_0000 │ Code Flash         │ R      │ Region 0 │   │
│  │ 0x2000_0000 │ SRAM (Kernel)      │ RW     │ Region 1 │   │
│  │ 0x2001_0000 │ SRAM (Task A)      │ RW     │ Region 2 │   │
│  │ 0x2002_0000 │ SRAM (Task B)      │ RW     │ Region 3 │   │
│  │ 0x4000_0000 │ Peripherals        │ RW     │ Region 4 │   │
│  │ 0xE000_0000 │ System Control     │ Priv   │ Region 5 │   │
│  └───────────────────────────────────────────────────────┘   │
│                                                               │
│  UPRAWNIENIA:                                                 │
│  ├── R (Read)   - odczyt                                      │
│  ├── W (Write)  - zapis                                       │
│  ├── X (Execute)- wykonywanie                                 │
│  ├── P (Privileged) - tylko tryb uprzywilejowany             │
│  └── C (Cache)  - cacheable                                   │
│                                                               │
│  TYPY BŁĘDÓW:                                                 │
│  ├── Background region access (brak regionu)                 │
│  ├── Permission violation (brak uprawnień)                   │
│  ├── Unaligned access (niepoprawne wyrównanie)               │
│  └── Execute from non-executable region                      │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Regiony MPU

```
┌─────────────────────────────────────────────────────────────┐
│                    KONFIGURACJA REGIONÓW                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  PRZYKŁAD (Cortex-M4, 8 regionów):                         │
│                                                             │
│  Region │ Start      │ Size   │ Attributes                │
│  ───────┼────────────┼────────┼───────────────────────────│
│  0      │ 0x00000000 │ 256 KB │ R   X   (Flash - Code)    │
│  1      │ 0x20000000 │ 128 KB │ RW  XN  (SRAM - Data)     │
│  2      │ 0x40000000 │ 512 MB │ RW  XN  (Peripherals)     │
│  3      │ 0xE0000000 │ 1 MB   │ RP  XN  (System - Priv)   │
│  4-7    │ Task-specific regions            │              │
│                                                             │
│  SIZE ALIGNMENT:                                            │
│  ├── Minimalny rozmiar: 32 bajty                          │
│  ├── Musi być potęgą 2                                     │
│  └── Adres startowy aligned do rozmiaru                    │
│                                                             │
│  SUBREGIONS:                                                │
│  ├── Każdy region może mieć 8 subregionów                  │
│  ├── Pozwala na dzielenie regionu                          │
│  └── Przydatne dla "dziur" w pamięci                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### RTOS Task Isolation

| Region | Task | Pamięć | Uprawnienia |
|--------|------|--------|-------------|
| Flash | Wszystkie | Code | R, X |
| Kernel | Privileged | Stack, Data | RW, XN, Priv |
| Task A | Task A | Stack, Data | RW, XN |
| Task B | Task B | Stack, Data | RW, XN |
| Shared | Wszystkie | Buffers | RW, XN |

### IoT Security

- **Code Injection Prevention:** Region kodu tylko R+X
- **Data Execution Prevention:** Dane RW+XN
- **Peripheral Protection:** Tylko kernel ma dostęp

### Medical Devices (IEC 62304)

- **Task Isolation:** Błąd w jednym task nie wpływa na inne
- **Safety Critical:** Osobny region dla kodu krytycznego
- **Audit Trail:** Logowanie naruszeń MPU

______________________________________________________________________

## Implementacja

### Konfiguracja MPU (Cortex-M)

```c
#include "core_cm4.h"

#define MPU_REGION_NUMBER   8

// Region attributes
#define MPU_ATTR_READONLY   (MPU_RASR_AP_RO_RO | MPU_RASR_C | MPU_RASR_XN)
#define MPU_ATTR_READWRITE  (MPU_RASR_AP_RW_RW | MPU_RASR_XN)
#define MPU_ATTR_PRIVONLY   (MPU_RASR_AP_RW_RO | MPU_RASR_XN)
#define MPU_ATTR_EXECUTABLE (0)  // No XN bit

typedef struct {
    uint32_t base_addr;
    uint32_t size;         // MPU_RASR_SIZE_xxx
    uint32_t attributes;
} mpu_region_t;

void mpu_init(void) {
    // Disable MPU during configuration
    MPU->CTRL = 0;

    // Region 0: Flash (Code) - Read + Execute
    MPU->RNR = 0;
    MPU->RBAR = 0x00000000;
    MPU->RASR = MPU_RASR_ENABLE | MPU_RASR_SIZE_256KB |
                MPU_RASR_AP_RO_RO | MPU_RASR_C;

    // Region 1: SRAM (Data) - Read/Write, No Execute
    MPU->RNR = 1;
    MPU->RBAR = 0x20000000;
    MPU->RASR = MPU_RASR_ENABLE | MPU_RASR_SIZE_128KB |
                MPU_RASR_AP_RW_RW | MPU_RASR_XN;

    // Region 2: Privileged System - Privileged Only
    MPU->RNR = 2;
    MPU->RBAR = 0xE0000000;
    MPU->RASR = MPU_RASR_ENABLE | MPU_RASR_SIZE_1MB |
                MPU_RASR_AP_RW_RO | MPU_RASR_XN;

    // Enable MPU with privileged default
    MPU->CTRL = MPU_CTRL_ENABLE | MPU_CTRL_PRIVDEFENA;

    __ISB();
    __DSB();
}

// Task switch - update MPU regions
void mpu_task_switch(task_t *task) {
    // Disable MPU
    MPU->CTRL = 0;

    // Set task-specific region
    MPU->RNR = 3;
    MPU->RBAR = task->stack_base;
    MPU->RASR = MPU_RASR_ENABLE | task->mpu_size |
                MPU_RASR_AP_RW_RW | MPU_RASR_XN;

    // Enable MPU
    MPU->CTRL = MPU_CTRL_ENABLE | MPU_CTRL_PRIVDEFENA;

    __ISB();
    __DSB();
}
```

### MPU Fault Handler

```c
void HardFault_Handler(void) {
    uint32_t mmfsr = SCB->CFSR & 0xFF;  // MemManage Fault Status
    uint32_t mmfar = SCB->MMFAR;        // Faulting address

    if (mmfsr & 0x01) {  // IACCVIOL - Instruction access violation
        log_fault("MPU: Instruction access violation at 0x%08X", mmfar);
    }

    if (mmfsr & 0x02) {  // DACCVIOL - Data access violation
        log_fault("MPU: Data access violation at 0x%08X", mmfar);
    }

    if (mmfsr & 0x08) {  // MUNSTKERR - Unstacking error
        log_fault("MPU: Unstacking error");
    }

    // Get faulting task
    task_t *faulting_task = get_current_task();

    // Log detailed info
    log_fault("Task: %s, PC: 0x%08X, LR: 0x%08X",
              faulting_task->name,
              faulting_task->context.pc,
              faulting_task->context.lr);

    // Terminate task or reset
    task_terminate(faulting_task);
    NVIC_SystemReset();
}
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Region overlap** | Nakładające się regiony | Dokładna konfiguracja |
| **Alignment error** | Źle wyrównany region | Sprawdzenie alignment |
| **Context switch overhead** | Czas przełączania MPU | Minimalizacja regionów per-task |
| **Privilege escalation** | Task dostaje uprawnienia | Minimalne uprawnienia |
| **MPU disabled** | Błąd w inicjalizacji | Weryfikacja w runtime |

______________________________________________________________________

## Kierunki rozwoju

### 1. PMP (RISC-V)

- Physical Memory Protection
- Bardziej elastyczne regiony
- Variable granularity

### 2. MPU v8-M (Cortex-M)

- Secure/Non-secure regions
- TrustZone support
- Więcej regionów

### 3. Hardware-enforced isolation

- Capability-based security
- CHERI architecture

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj konfigurację MPU dla systemu IoT z 4 zadaniami:

ZADANIA:
├── Kernel (Privileged): 64 KB code, 32 KB data
├── Task A (User): 8 KB stack, dostęp do UART
├── Task B (User): 8 KB stack, dostęp do SPI
└── Task C (User): 4 KB stack, tylko RAM

PYTANIA:
1. Jakie regiony zdefiniować?
2. Jakie uprawnienia nadać?
3. Jak przełączać regiony przy task switch?

ROZWIĄZANIE:
1. Regiony (8 dostępnych):
   R0: Flash (code) - R+X, wszystkie taski
   R1: Kernel data - RW+XN, Privileged only
   R2: Task A stack - RW+XN, Task A only
   R3: Task B stack - RW+XN, Task B only
   R4: Task C stack - RW+XN, Task C only
   R5: UART - RW+XN, Task A + Kernel
   R6: SPI - RW+XN, Task B + Kernel
   R7: Shared buffers - RW+XN, wszystkie

2. Uprawnienia:
   - Kernel: pełny dostęp (Privileged)
   - Task A: R0, R2, R5, R7
   - Task B: R0, R3, R6, R7
   - Task C: R0, R4, R7

3. Task switch:
   - Zapisz aktualną konfigurację MPU
   - Wyłącz regiony R2-R4
   - Włącz region właściwego tasku
   - Włącz regiony peryferiów dla tasku
```

______________________________________________________________________

## Pytania kontrolne

1. Jaka jest różnica między MPU a MMU?
1. Dlaczego MPU jest ważne dla bezpieczeństwa?
1. Jakie są ograniczenia MPU w Cortex-M?
1. Jak obsłużyć MPU fault?

______________________________________________________________________

## Literatura

1. ARM Cortex-M4 Technical Reference Manual, "Memory Protection Unit"
1. ARM Application Note, "Using the MPU on Cortex-M"
1. ISO 26262-6, "Software Safety Requirements"
1. IEC 62443, "Industrial Cybersecurity"
