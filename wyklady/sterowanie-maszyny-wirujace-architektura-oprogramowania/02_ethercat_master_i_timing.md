# Wyklad 2: EtherCAT master i timing (cykl, DC, watchdog)

## Cel
Opisac EtherCAT od strony implementacji systemu:
- co musi byc cykliczne,
- jak dobrac cykl bez "max speed",
- co robi Distributed Clocks (DC),
- jak projektowac zachowanie na awarie.

> TL;DR: EtherCAT jest narzedziem do cyklicznej wymiany danych. Jak nie mierzysz jitteru i WCRT, to nie wiesz czy Twoj cykl istnieje.

## Model mentalny: "w cyklu" vs "poza cyklem"
Projektuj dwie klasy danych:
- cykliczne: sterowanie + pomiary do petli
- acykliczne: konfiguracja, diagnostyka, serwis

Zasada:
- cykliczne = minimum potrzebne do stabilnosci i safety
- acykliczne = rate-limited i odseparowane

## Dobor cyklu
Kompromis:
- za wolno: spada pasmo, rosną bledy
- za szybko: rośnie obciazenie CPU, jitter, dropouty

Metodyka:
1. ustaw cykl startowy konserwatywnie
2. zmierz p99/p99.9 `rt_loop_us` i jakosc regulacji
3. iteruj, tylko jesli deterministyka trzyma

## DC (Distributed Clocks) - kiedy ma znaczenie
DC ma sens, gdy:
- chcesz spojnego samplingu wielu modulow
- porownujesz sygnaly miedzy slave'ami (faza)
- chcesz zmniejszyc bledy timestampingu

## Watchdog: co MUSI byc zdefiniowane
Scenariusz: master nie wysyla w cyklu.
Wtedy:
- slave/drive musi przejsc w stan bezpieczny/ograniczony
- master musi zareagowac degradacja lub safe stop
- powrot musi miec warunki i histereze (brak flappingu)

## Testy obciazeniowe (bez hardware "docelowego")
W labie weryfikujesz:
- przeciążenie CPU mastera
- duza telemetria
- dropout komunikacji

Kryteria:
- brak serii missed deadlines dla krytycznej petli
- deterministyczna reakcja watchdogow

## Checklisty
- Masz tabele sygnalow: cykliczne/acykliczne.
- Masz metryki jitter/WCRT i liczysz percentyle.
- Masz zdefiniowany watchdog i zachowanie awaryjne.

## Kontekst robotyki: siec ruchu w komorce produkcyjnej
W komorce z robotem i wieloma osiami:
- sterowanie ruchem ma wlasna siec (np. EtherCAT),
- czujniki bezpieczenstwa i IT sa odseparowane,
- synchronizacja czasu jest kluczowa dla korelacji danych (diagnostyka, audit).

Wniosek:
- EtherCAT nie jest "tylko do napedow", to tez kragoslup deterministycznego czasowania calej komorki.

## Pytania do studentow
1. Jak sklasyfikujesz sygnaly na cykliczne/acykliczne i jakie bedzie tego konsekwencje dla jitteru?
2. Jak dobierzesz cykl tak, aby nie "zabic" CPU i nie pogorszyc deterministyki?
3. Jak wyglada zachowanie systemu przy awarii komunikacji (slave i master) i jak unikasz "hold last command"?
4. Kiedy DC ma realny zwrot, a kiedy tylko komplikuje system?

## Projekty studenckie
- "Signal table generator": generator tabeli sygnalow i budzetu w cyklu.
- "Watchdog spec + tests": specyfikacja watchdogow i testy fault-injection.
- "Load test plan": plan testow (CPU load, telemetry flood, dropout) z metrykami pass/fail.

## BONUS
- Najczestszy blad: "szybszy cykl = lepiej". W praktyce szybszy cykl bez pomiaru ogonow opoznien czesto jest gorszy.
