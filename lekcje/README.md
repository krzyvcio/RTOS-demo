# Lekcje RTOS - Zagrożenia i Przypadki Krytyczne

Ten folder zawiera przykłady kodu demonstrujące typowe zagrożenia w systemach czasu rzeczywistego.

## Struktura

```
lekcje/
├── rust/           # Przykłady w Rust (deadlock, race conditions)
├── python/         # Symulacje i demonstracje
├── c/              # Przykłady niskopoziomowe w C (RTOS, ISR, mutex)
└── README.md       # Ten plik
```

## Kategorie zagrożeń (300 przypadków)

### 1. Synchronizacja (50 przypadków)

- Priority Inversion
- Deadlock
- Livelock
- Race Conditions
- Starvation

### 2. Pamięć (40 przypadków)

- Stack Overflow
- Heap Fragmentation
- Memory Leaks
- Buffer Overflow
- Cache Coherency

### 3. Timing (50 przypadków)

- Jitter
- Deadline Miss
- WCET Violation
- Priority Inversion
- ISR Latency

### 4. ISR i Przerwania (40 przypadków)

- ISR Storm
- Nested Interrupts
- Reentrancy Issues
- Interrupt Priority

### 5. Komunikacja (40 przypadków)

- Message Queue Overflow
- Producer-Consumer
- CAN Bus Errors
- AFDX Timing

### 6. Zasilanie i Hardware (30 przypadków)

- Brown-out
- Watchdog Timeout
- NMI Handling
- DMA Contention

### 7. Bezpieczeństwo (30 przypadków)

- MPU Violations
- Code Injection
- Side-Channel
- Secure Boot

### 8. Partycjonowanie (20 przypadków)

- ARINC 653 Violations
- Mixed-Criticality
- Temporal Violations

______________________________________________________________________

## Jak używać

Każdy plik zawiera:

1. Opis problemu
1. Kod demonstracyjny
1. Objawy awarii
1. Rozwiązanie/mitigacja
1. Powiązane normy (ISO 26262, DO-178C, etc.)

______________________________________________________________________

## Tagowanie

Przykłady są tagowane według:

- `#ASIL-D` - krytyczne dla automotive
- `#DAL-A` - krytyczne dla aerospace
- `#SIL-3` - krytyczne dla przemysłu
- `#ISR` - związane z przerwaniami
- `#DEADLOCK` - zakleszczenia
- `#TIMING` - problemy czasowe
