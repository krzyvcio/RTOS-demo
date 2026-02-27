# Wykład 9: DevOps/CI dla Digital Twin Lab (testy regulatorów, deterministyki, regresja)

## Cel
Zrobić z labu narzędzie inżynierskie, a nie jednorazową demonstrację:
- testy jednostkowe regulatorów,
- testy integracyjne harness,
- regresja na scenariuszach,
- testy deterministyki (tam, gdzie to ma sens).

> TL;DR: Jeśli nie masz scenariuszy regresji i metryk, to każda zmiana jest ruletką.

## Co testować automatycznie
- matematyka regulatora (PI/PID, anti-windup),
- saturacje i ograniczenia (rampy, jerk),
- reakcja na skok zakłócenia,
- reakcja na dropout i opóźnienie,
- stabilność „w sensie metryk” (nie rośnie błąd i nie pojawiają się oscylacje).

## Deterministyka w CI
W CI (szczególnie w wirtualizacji) nie zawsze zmierzysz twarde RT wiarygodnie.
Co nadal ma sens:
- testy logiki degradacji i watchdog,
- testy czasów w obrębie procesu (w przybliżeniu),
- testy scenariuszy i metryk jakości sterowania.

## Checklisty
- Każdy PR uruchamia scenariusze regresji.
- Wyniki są porównywane do baseline (metryki).
- Testy są powtarzalne (seed, replay).

## Slajdy (tekstowe)
### Slajd 1: Po co CI w labie
- żeby nie „psuć” sterowania zmianami

### Slajd 2: Co weryfikujemy
- metryki jakości i bezpieczeństwa
- regresja scenariuszy

## Pytania do studentów
1. Które testy są „must-have” w CI dla sterowania (nie tylko unit testy)?
2. Jak zaprojektujesz metryki jakości sterowania tak, aby były stabilne między runami (seed, replay)?
3. Co w CI ma sens mierzyć timingowo, a co trzeba zostawić na testy na docelowym sprzęcie?
4. Jak wprowadzisz gate na safety (np. brak release bez testów fault-injection)?

## Projekty studenckie
- „Regression suite”: zestaw scenariuszy + porównanie metryk do baseline i raport w CI.
- „Artifact store”: przechowywanie logów/regresji z wersjonowaniem schematu i parametrów.
- „Safety gate”: pipeline blokujący merge/release bez przejścia testów awaryjnych.

## BONUS
- Jeśli CI nie potrafi wykryć regresji „zwiększył się jitter/pogorszyły się oscylacje”, to tak naprawdę CI nie chroni Twojego produktu, tylko daje złudzenie jakości.
