# Wyklad 8: Safety (programistycznie) — watchdog, degradacja, safe stop

## Cel
Zaprojektowac safety jako osobny kanal decyzyjny:
- proste reguly, szybkie reakcje,
- niezalezne od telemetrii i "sprytnej" logiki,
- testowalne przez fault injection.

> TL;DR: Safety ma dzialac, gdy system jest w najgorszym stanie (brak danych, zawieszenie, dropouty, wysoka energia kinetyczna).

## Klasy awarii (typowe)
- missed deadline (RT)
- dropout komunikacji (magistrala)
- utrata czujnika lub "zamrozony" pomiar
- przegrzanie
- saturacja pradu/momentu

## FSM bezpieczenstwa
Przykladowe stany:
- NORMAL
- WARNING
- DEGRADED
- SAFE_STOP
- RECOVERY

Zasady:
- przejscia do bezpieczniejszych stanow sa latwe i natychmiastowe
- powrot wymaga warunkow i histerezy (brak flappingu)

## Watchdog wielopoziomowy
- watchdog w drive/slave: brak komend -> stan bezpieczny
- watchdog w master: brak cyklu -> safe stop/degradacja

## Fault injection jako standard testow
Testy:
- dropout w losowych miejscach
- opoznienie i jitter
- przeciazenie CPU

Kryteria:
- czas reakcji
- deterministyczny stan koncowy
- kontrolowany recovery

## Checklisty
- Kazda klasa awarii ma: detekcja -> reakcja -> warunki powrotu.
- Watchdog jest testowany automatycznie (regresja).

## Etyka i odpowiedzialnosc (praktycznie)
W systemach krytycznych etyka to:
- projektowanie tak, by awarie byly przewidywalne i bezpieczne,
- unikanie ukrytych zaleznosci (np. safety zalezy od chmury),
- transparentnosc decyzji w warstwie safety (logi, stany, przyczyny),
- procedury walidacji (fault injection, regresja).

Minimalne pytania do review:
- co sie stanie, gdy zniknie sensor?
- co sie stanie, gdy master przestanie odpowiadac?
- czy system moze wejsc w stan niebezpieczny przez "ladny" dashboard?

## Pytania do studentow
1. Jakie sa minimalne stany FSM bezpieczenstwa i jakie warunki powrotu sa krytyczne?
2. Jak projektujesz watchdog tak, by dzialal nawet przy awarii komunikacji i przeciążeniu CPU?
3. Jakie testy fault injection musza byc w regresji (zawsze) i dlaczego?
4. Jak ograniczasz ryzyko, ze "sprytna" logika (np. ML) wejdzie do safety?

## Projekty studenckie
- "Safety FSM + logs": implementacja FSM bezpieczenstwa + logowanie przyczyn przejsc.
- "Fault injection suite": zestaw testow dropout/opoznienie/zamrozony sensor/przeciazenie.
- "Release gate": pipeline blokujacy release bez przejscia testow awaryjnych.

## BONUS
- Etyka w systemach krytycznych zaczyna sie od tego, ze potrafisz powiedziec: "jak system zachowa sie w awarii" i udowodnic to testem, a nie deklaracja.
