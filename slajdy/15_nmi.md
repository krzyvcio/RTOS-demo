# NMI - Non-Maskable Interrupt

## Definicja

Przerwanie o najwyższym priorytecie, które nie może być maskowane (zablokowane) przez oprogramowanie. Używane do obsługi krytycznych awarii sprzętowych takich jak błędy pamięci, awarie zasilania, błędy CRC.

______________________________________________________________________

## Architektura NMI

```
┌───────────────────────────────────────────────────────────────┐
│                    PRIORYTETY PRZERWAŃ                        │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  PRIORYTET (Cortex-M):                                        │
│                                                               │
│  ┌───────────────────────────────────────────────────────┐   │
│  │ -3  │ NMI (Non-Maskable Interrupt)     │ NIEZALEŻNE  │   │
│  │     │ - Hard Fault                      │             │   │
│  │     │ - Memory Management Fault         │             │   │
│  ├─────┼────────────────────────────────────┼─────────────┤   │
│  │ -2  │ PendSV, SysTick                    │ System      │   │
│  ├─────┼────────────────────────────────────┼─────────────┤   │
│  │ -1  │ Reserved                           │             │   │
│  ├─────┼────────────────────────────────────┼─────────────┤   │
│  │ 0   │ External Interrupt #0              │             │   │
│  │ 1   │ External Interrupt #1              │ Maskowalne  │   │
│  │ ... │ ...                                │             │   │
│  │ n   │ External Interrupt #n              │             │   │
│  └─────┴────────────────────────────────────┴─────────────┘   │
│                                                               │
│  NMI jest ZAWSZE aktywowane, nawet gdy:                       │
│  ├── PRIMASK = 1 (global interrupt disable)                  │
│  ├── BASEPRI != 0 (priority masking)                         │
│  └── Inny handler jest aktywny                               │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Źródła NMI

```
┌─────────────────────────────────────────────────────────────┐
│                    ŹRÓDŁA NMI                               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  SPRZĘTOWE:                                                 │
│  ├── Clock Failure Detection                               │
│  │   └── Zegar systemowy przestał działać                  │
│  │                                                         │
│  ├── Brown-Out Detection (BOD)                             │
│  │   └── Spadek napięcia zasilania                         │
│  │                                                         │
│  ├── ECC Error (Memory)                                    │
│  │   └── Błąd korekcji pamięci (nieodwracalny)             │
│  │                                                         │
│  ├── External NMI pin                                      │
│  │   └── Zewnętrzny sygnał awarii                          │
│  │                                                         │
│  └── Watchdog Timeout (niektóre MCU)                       │
│      └── Hardware watchdog overflow                        │
│                                                             │
│  PROGRAMOWE:                                                │
│  ├── Hard Fault                                            │
│  │   └── Błąd wykonania (niepoprawna instrukcja)           │
│  │                                                         │
│  ├── Memory Management Fault                               │
│  │   └── Naruszenie ochrony pamięci (MPU)                  │
│  │                                                         │
│  ├── Bus Fault                                             │
│  │   └── Błąd na szynie (timeout, błąd adresu)             │
│  │                                                         │
│  └── Usage Fault                                           │
│      └── Błąd użycia (dzielenie przez zero, unaligned)     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Automotive (ISO 26262 ASIL-D)

| Źródło NMI | Reakcja | System |
|------------|---------|--------|
| ECC Error | Safe state | Engine Control |
| Clock Failure | Failsafe mode | Brake System |
| Brown-Out | Immediate shutdown | Steering |
| MPU Violation | Reset | ADAS |

### Medical (IEC 62304 Class C)

- **Infusion Pump:** NMI na błąd dawkowania
- **Ventilator:** NMI na awarię sensora
- **Defibrillator:** NMI na błąd ładowania

### Industrial (IEC 61508 SIL-3)

- **Safety PLC:** NMI na błąd programu
- **Motor Drive:** NMI na overcurrent
- **Emergency Stop:** NMI na przycisk E-stop

______________________________________________________________________

## Implementacja

### Cortex-M NMI Handler

```c
// NMI Handler - najwyższy priorytet
void NMI_Handler(void) {
    uint32_t flags = NMI->STATUS;

    if (flags & NMI_CLOCK_FAILURE) {
        // Zegar padł - przełącz na backup
        switch_to_backup_clock();
        log_fault("Clock failure");
    }

    if (flags & NMI_BROWN_OUT) {
        // Spadek napięcia - natychmiastowy shutdown
        emergency_shutdown();
    }

    if (flags & NMI_ECC_ERROR) {
        // Błąd pamięci - nieodwracalny
        safe_state();
        system_reset();
    }

    // Jeśli nic nie pasuje - reset
    NVIC_SystemReset();
}

// Konfiguracja NMI
void nmi_init(void) {
    // Włącz detekcję clock failure
    NMI->CTRL = NMI_CTRL_CLK_FAIL_EN;

    // Włącz brown-out detection
    NMI->CTRL |= NMI_CTRL_BOD_EN;

    // Ustaw próg brown-out
    NMI->BOD_THRESHOLD = BOD_THRESHOLD_2_7V;
}
```

### Safe State Pattern

```c
typedef enum {
    SAFE_STATE_NONE = 0,
    SAFE_STATE_CLOCK_FAIL,
    SAFE_STATE_BROWN_OUT,
    SAFE_STATE_ECC_ERROR,
    SAFE_STATE_EXTERNAL
} safe_state_reason_t;

void enter_safe_state(safe_state_reason_t reason) {
    // 1. Wyłącz wszystkie wyjścia
    disable_all_outputs();

    // 2. Ustaw wyjścia w stanie bezpiecznym
    set_outputs_safe();

    // 3. Zapisz przyczynę w pamięci nieulotnej
    backup_register_write(BACKUP_SAFE_STATE_REASON, reason);

    // 4. Loguj awarię
    fault_log_add(reason, get_timestamp());

    // 5. Czekaj na reset lub interwencję
    while (1) {
        // Opcjonalnie: miganie LED awarii
        __WFI();  // Wait For Interrupt
    }
}
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **NMI storm** | Lawina NMI | Debounce, rate limiting |
| **NMI handler hang** | Pętla w handler | Timeout, hardware reset |
| **Stack overflow w NMI** | Brak stacka | Osobny stack dla NMI |
| **Race condition** | Konflikt z main | Atomic operations |
| **Zła konfiguracja** | Źródło NMI | Dokładna konfiguracja |

______________________________________________________________________

## Dobre praktyki

```
┌─────────────────────────────────────────────────────────────┐
│                    DOBRE PRAKTYKI NMI                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. MINIMALNY KOD                                           │
│     ├── NMI handler powinien być krótki                    │
│     ├── Tylko najpotrzebniejsze akcje                      │
│     └── Bez pętli nieskończonych                            │
│                                                             │
│  2. OSOBNY STACK                                            │
│     ├── Rezerwuj stack dla exception handlers              │
│     ├── Sprawdź MSP (Main Stack Pointer)                   │
│     └── Minimum 256 bajtów                                 │
│                                                             │
│  3. ATOMIC OPERATIONS                                       │
│     ├── Używaj __disable_irq() tylko gdy konieczne         │
│     ├── Lepiej: ldrex/strex (Cortex-M)                     │
│     └── Unikaj blokowania w NMI                            │
│                                                             │
│  4. FAULT LOGGING                                           │
│     ├── Zapisz przyczynę w backup registers                │
│     ├── Zapisz stan CPU (R0-R3, PC, LR)                    │
│     └── Ułatwia diagnostykę po resecie                     │
│                                                             │
│  5. SAFE STATE                                              │
│     ├── Zawsze przejdź do znanego stanu bezpiecznego       │
│     ├── Wyłącz actuators                                   │
│     └── Komunikuj awarię                                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Dual-Core Lockstep

- Dwa rdzenie wykonują ten sam kod
- Hardware comparator wykrywa rozbieżność
- NMI na mismatch

### 2. ECC z korekcją

- Single-bit: korekcja
- Double-bit: NMI
- Logging do pamięci nieulotnej

### 3. Predictive Fault Detection

- Trend monitoring
- Wczesne ostrzeganie przed NMI

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj obsługę NMI dla systemu sterowania hamulcami:

WYMAGANIA:
├── Brown-out detection: próg 2.7V
├── Clock failure: przełączenie na backup
├── ECC error: safe state + reset
└── External NMI: emergency stop

PYTANIA:
1. Jakie akcje podjąć w każdym przypadku?
2. Jak zapewnić, że NMI handler się nie zawiesi?
3. Jak zapisać stan do diagnostyki?

ROZWIĄZANIE:
1. Akcje:
   - Brown-out: Natychmiastowe otwarcie zaworów, hydraulic backup
   - Clock failure: Przełączenie na RC oscillator, degraded mode
   - ECC error: Safe state, system reset
   - External: Emergency stop, hydraulic backup

2. Zabezpieczenia handlera:
   - Timeout counter w hardware
   - Osobny stack (MSP)
   - Minimalna logika, bez pętli

3. Diagnostyka:
   - Backup registers (nieulotne)
   - Zapisz: reason, timestamp, stack trace
   - CRC na danych diagnostycznych
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego NMI nie może być zablokowane?
1. Jakie są typowe źródła NMI w systemach automotive?
1. Jak zapewnić, że NMI handler zawsze się wykona?
1. Co to jest safe state i kiedy się go stosuje?

______________________________________________________________________

## Literatura

1. ARM Cortex-M4 Technical Reference Manual, "Exception Handling"
1. ISO 26262-5, "Hardware Safety Requirements"
1. IEC 61508-2, "Safety-related hardware"
1. NXP, "Kinetis MCU Safety Manual"
