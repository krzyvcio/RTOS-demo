# Wykład 9: DevOps/CI dla Digital Twin Lab (testy regulatorów, deterministyki, regresja)

## Część I: Wstęp teoretyczny - Dlaczego automatyzacja jest niezbędna

### Geneza: od "ręcznego testowania" do "zautomatyzowanego nadzoru"

Zbudowaliśmy już:
- Model obiektu
- Sterowanie z filtrem i anti-windup
- Środowisko RT
- Komunikację
- HIL z harnessem
- Narzędzia do analizy FFT

Ale wszystko to testujemy "ręcznie": uruchamiamy, patrzymy, zmieniamy, uruchamiamy znowu. Problem: to nie skaluje się i nie daje powtarzalnych wyników.

DevOps i CI (Continuous Integration) to rozwiązania tych problemów:
- **Automatyczne testy** uruchamiane przy każdej zmianie
- **Testy regresji** porównujące wyniki "przed" i "po"
- **Powtarzalność** - te same warunki za każdym razem
- **Wczesne wykrywanie problemów** - zanim dotrą do produkcji

### Przemówienie Profesora

Najgorsza pułapka, w którą wpadają studenci: "działa na moim komputerze".

CI rozwiązuje ten problem - bo "na moim komputerze" nie istnieje. Jest automatyczne środowisko, które testuje każdą zmianę.

Ale CI to nie jest "magiczna różdżka" - to disciplina. Trzeba:
- Pisać testy (nie jest to "dodatkowa praca", to INWESTYCJA)
- Utrzymywać testy
- Reagować na failures

Bez tego CI jest puste.

## Cel
Zrobić z labu narzędzie inżynierskie, a nie jednorazową demonstrację:
- testy jednostkowe regulatorów,
- testy integracyjne harness,
- regresja na scenariuszach,
- testy deterministyki (tam, gdzie to ma sens).

> TL;DR: Jeśli nie masz scenariuszy regresji i metryk, to każda zmiana jest ruletką.

## Część II: Co automatyzować

## Co testować automatycznie
- matematyka regulatora (PI/PID, anti-windup),
- saturacje i ograniczenia (rampy, jerk),
- reakcja na skok zakłócenia,
- reakcja na dropout i opóźnienie,
- stabilność "w sensie metryk" (nie rośnie błąd i nie pojawiają się oscylacje).

### Co testować automatycznie - szczegóły

**Testy jednostkowe:**
- Czy PI/PID daje poprawny wynik?
- Czy anti-windup działa?
- Czy filtry nie wprowadzają błędów?

**Testy integracyjne:**
- Czy sterowanie działa z modelem?
- Czy komunikacja nie blokuje?
- Czy watchdog reaguje?

**Testy regresji:**
- Czy zmiana nie pogorszyła metryk?
- Czy błąd regulacji jest podobny?
- Czy jitter nie wzrósł?

### Przemówienie Profesora

Najważniejsze pytanie: **co jest dla ciebie "pass/fail"?**

Nie "czy test się wykonał", ale "czy wynik jest akceptowalny".

Przykład: zmieniłeś parametry regulatora. Czy:
- Błąd regulacji wzrósł o > 10%? → FAIL
- Jitter wzrósł o > 20%? → FAIL
- Wszystko OK? → PASS

To jest metryka. Bez metryki - nie ma regresji.

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

### Checklist szczegółowy

**CI Pipeline:**
- [ ] Uruchomienie testów przy każdym PR
- [ ] Generowanie raportu z metrykami
- [ ] Porównanie z baseline
- [ ] Powiadomienie o failures

**Metryki:**
- [ ] Definicja pass/fail dla każdej metryki
- [ ] Zapis baseline
- [ ] Porównanie wyników

**Testy:**
- [ ] Scenariusze regresji (skok, zakłócenie, saturacja)
- [ ] Seed dla powtarzalności
- [ ] Replay capability

**Bezpieczeństwo:**
- [ ] Testy fault-injection w CI
- [ ] Gate na safety-critical changes

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
