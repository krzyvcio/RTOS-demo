// ============================================================================
// RTOS HAZARDS - RUST EXAMPLES
// ============================================================================
// Kolekcja przykładów zagrożeń w systemach czasu rzeczywistego
// ============================================================================

pub mod deadlock;
pub mod race_conditions;
pub mod priority_inversion;
pub mod memory_issues;
pub mod timing_issues;

// ============================================================================
// INDEKS PRZYPADKÓW (300 zagrożeń)
// ============================================================================

/*
KATEGORIA 1: DEADLOCK I SYNCHRONIZACJA (50 przypadków)
========================================================

#DEADLOCK_001  - Klasyczny AB-BA deadlock
#DEADLOCK_002  - Self-deadlock (recursive lock)
#DEADLOCK_003  - Deadlock z 3 mutexami
#DEADLOCK_004  - Deadlock z condition variable
#DEADLOCK_005  - Deadlock z timeout
#DEADLOCK_006  - Deadlock z try_lock
#DEADLOCK_007  - Deadlock z RwLock
#DEADLOCK_008  - Deadlock w producent-konsument
#DEADLOCK_009  - Deadlock z barrier
#DEADLOCK_010  - Deadlock z mpsc channel

KATEGORIA 2: RACE CONDITIONS (50 przypadków)
==============================================

#RACE_001      - Data race na zmiennej
#RACE_002      - Race condition z lazy initialization
#RACE_003      - Race w singletonie
#RACE_004      - Race z check-then-act
#RACE_005      - Race z read-modify-write
#RACE_006      - Race na liczniku
#RACE_007      - Race w hashmap
#RACE_008      - Race z iterator
#RACE_009      - Race z Arc
#RACE_010      - Race z RefCell

KATEGORIA 3: PRIORITY INVERSION (30 przypadków)
=================================================

#PRIO_INV_001  - Klasyczna inwersja priorytetów
#PRIO_INV_002  - Inwersja z 3 zadaniami
#PRIO_INV_003  - Inwersja z wieloma mutexami
#PRIO_INV_004  - Chain blocking
#PRIO_INV_005  - Inwersja z condvar

KATEGORIA 4: MEMORY ISSUES (40 przypadków)
============================================

#MEM_001       - Stack overflow przez rekurencję
#MEM_002       - Heap exhaustion
#MEM_003       - Memory leak w pętli
#MEM_004       - Fragmentacja pamięci
#MEM_005       - Use after free (simulacja)
#MEM_006       - Double free (simulacja)

KATEGORIA 5: TIMING ISSUES (50 przypadków)
============================================

#TIMING_001    - Deadline miss
#TIMING_002    - Jitter burst
#TIMING_003    - WCET violation
#TIMING_004    - Starvation
#TIMING_005    - Livelock

KATEGORIA 6: ISR I PRZERWANIA (40 przypadków)
===============================================

#ISR_001       - Reentrancy issue
#ISR_002       - ISR latency
#ISR_003       - Nested interrupt overflow
#ISR_004       - Priority inversion w ISR

KATEGORIA 7: KOMUNIKACJA (40 przypadków)
==========================================

#COMM_001      - Queue overflow
#COMM_002      - Producer-consumer deadlock
#COMM_003      - Message loss
#COMM_004      - Priority inversion w kolejce

KATEGORIA 8: BEZPIECZEŃSTWO (30 przypadków)
=============================================

#SEC_001       - Buffer overflow
#SEC_002       - Integer overflow
#SEC_003       - Side-channel timing
#SEC_004       - Unsafe code abuse
*/