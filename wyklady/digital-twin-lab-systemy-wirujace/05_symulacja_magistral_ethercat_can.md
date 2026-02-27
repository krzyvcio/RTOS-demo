# Wykład 5: Symulacja magistral przemysłowych (EtherCAT, CAN/CAN-FD)

## Cel
Umieć przetestować architekturę komunikacji zanim kupisz sprzęt:
- wymiana danych cyklicznych,
- opóźnienia i jitter,
- dropouty,
- watchdog i reakcje awaryjne.

> TL;DR: Najpierw definiujesz „co jest cykliczne”, potem testujesz co się dzieje, gdy cykl nie jest dotrzymany.

## EtherCAT: co warto symulować
- wymianę PDO (dane sterujące/pomiarowe w cyklu),
- wpływ obciążenia CPU na jitter,
- wpływ „dodatkowych danych” (telemetria) na deterministykę,
- zachowanie watchdogów.

## CAN/CAN-FD: co warto symulować
- opóźnienia wynikające z arbitrażu,
- straty/kolizje (w modelu obciążenia),
- wpływ priorytetów ramek na czas dostarczenia sygnału krytycznego.

## Zasada projektowa: budżet czasu end-to-end
Sygnał krytyczny ma:
- deadline,
- budżet opóźnienia,
- reakcję w razie przekroczenia.

## Checklisty
- Masz listę sygnałów cyklicznych i acyklicznych.
- Masz test dropout + test przeciążeniowy.
- Watchdog ma zdefiniowane zachowanie i warunki powrotu.

## Slajdy (tekstowe)
### Slajd 1: Co testujemy w magistrali
- cykl, jitter, dropouty
- watchdog i safe behavior

### Slajd 2: EtherCAT vs CAN
- EtherCAT: cykliczność i synchronizacja
- CAN: arbitraż i priorytety

## Pytania do studentów
1. Które sygnały muszą mieć twardy deadline i jak policzysz budżet end-to-end przez magistralę?
2. Jakie zachowanie watchdogów jest minimalnie wymagane, aby system był bezpieczny przy dropoutach?
3. Jakie są konsekwencje mieszania telemetrii z cyklem sterowania?
4. Jak przetestujesz arbitraż i priorytety w CAN (na poziomie architektury, nie tylko teorii)?

## Projekty studenckie
- „Bus simulator”: symulator opóźnień/jitteru/dropoutów i wpływu na sterowanie.
- „Signal classification”: narzędzie do klasyfikacji sygnałów (cykliczne/acykliczne) + rate limiting.
- „Watchdog harness”: testy fault-injection dla zachowania magistrali i reakcji systemu.

## BONUS
- W projektach integracyjnych najczęściej wygrywa zespół, który najpierw definiuje kontrakt sygnałów i zachowanie na awarię, a dopiero potem „podłącza kabel”.
