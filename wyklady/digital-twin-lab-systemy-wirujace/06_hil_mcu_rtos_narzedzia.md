# Wykład 6: HIL w małej skali (MCU + RTOS) i narzędzia warsztatowe

## Cel
Zrobić „pół-realne” testy bez budowania docelowego hardware:
- firmware na MCU (FreeRTOS/Zephyr),
- PC jako „plant” (symulacja obiektu),
- komunikacja (UART/CAN/UDP),
- pomiary na stole (debug, timing, sygnały).

> TL;DR: HIL to sposób na test firmware i RT zachowania, zanim mechanika istnieje.

## MCU i RTOS: co testujesz naprawdę
- deterministykę pętli (deadline, jitter),
- obsługę przerwań i komunikacji,
- watchdogi i reakcje awaryjne,
- działanie filtrów i regulatorów w realnym czasie.

## Narzędzia HW, które przyspieszają debug
- debug probe (J-Link lub podobny),
- logic analyzer (widzisz timing na pinach),
- oscyloskop (widzisz rzeczywiste zakłócenia i sygnały),
- zasilacz laboratoryjny (symulacja spadków/warunków).

## Checklisty
- Firmware nie ma alokacji w pętli krytycznej.
- Masz pomiar czasu iteracji (GPIO toggle, timestamp).
- Watchdog jest testowany przez fault-injection.

## Slajdy (tekstowe)
### Slajd 1: Po co HIL
- test firmware i RT bez mechaniki

### Slajd 2: Co mierzyć na MCU
- deadline, jitter, watchdog

## Pytania do studentów
1. Jak zmierzysz jitter pętli na MCU bez wpływu debuggera na timing?
2. Jak zaprojektujesz komunikację MCU<->PC tak, aby nie blokowała pętli RT?
3. Jakie błędy sprzętowe/firmware chcesz wstrzykiwać w HIL i jak je wykryjesz?
4. Co powinno być „twardo” w firmware niezależnie od mastera (limity, watchdog, safe state)?

## Projekty studenckie
- „MCU loop timing”: pomiar `rt_loop_us` na MCU (GPIO toggle + logic analyzer) + raport.
- „RTOS tasks”: zestaw zadań RTOS (control, comm, diagnostics) z priorytetami i testem przeciążeniowym.
- „Watchdog demo”: watchdog + safe state + testy dropoutów komunikacji.

## BONUS
- Jeśli nie potrafisz odtworzyć tego samego problemu timingowego na stole (HIL), to na sprzęcie docelowym będzie tylko gorzej. HIL ma służyć powtarzalności.
