# Wykład 0: Mapa systemu i architektura wirtualnego laboratorium

## Cel
Ustawić wspólny obraz całości:
- jakie bloki składają się na system wirujący,
- gdzie są granice odpowiedzialności (model, sterowanie, RT, magistrala),
- jak wygląda architektura „Digital Twin Lab” krok po kroku.

> TL;DR: Najpierw powstaje model i harness, potem RT i pomiary, dopiero na końcu integracja z magistralą i HIL.

## Mapa bloków systemu
System wirujący w ujęciu inżynierskim to zwykle:
- obiekt (wirnik + łożyskowanie + konstrukcja),
- napęd (falownik + silnik + sensory),
- sterowanie (prąd/moment, prędkość, tryby),
- diagnostyka (wibracje, termika, saturacje),
- bezpieczeństwo (watchdog, limity, safe stop),
- komunikacja (magistrala, synchronizacja czasu).

## Architektura wirtualnego laboratorium (wariant bazowy)
Schemat, który działa w praktyce:
```text
[Model ODE/MDOF na PC]
        ↓ sprzężenie zwrotne (symulowane pomiary)
[Kontroler RT (Linux PREEMPT_RT albo RTOS)]
        ↓ magistrala (EtherCAT/CAN/UDP/UART)
[Warstwa I/O / MCU (opcjonalnie w HIL)]
        ↓
[Model obiektu / „plant”]
```

Interpretacja:
- „plant” może być tylko procesem na PC (SIL), albo osobnym modułem (HIL),
- kontroler może być procesem RT na PC lub firmware na MCU,
- magistrala może być realna (hardware) albo symulowana.

## Typowe etapy budowy (w kolejności, która minimalizuje ryzyko)
1. Model minimalny ODE (prędkość + zakłócenia) + testy jednostkowe.
2. Symulacja pętli sterowania (PI/PID, saturacje, anti-windup) w czasie dyskretnym.
3. Pomiar deterministyki (jitter/WCRT) w pętli RT (Linux PREEMPT_RT lub RTOS).
4. Dodanie diagnostyki (FFT, piki, trendy, baseline).
5. Integracja magistrali (EtherCAT/CAN) i testy obciążeniowe.
6. HIL: MCU w pętli (PC symuluje plant, MCU myśli, że steruje realnym obiektem).

## Kompetencje, które muszą powstać w zespole
- modelowanie ODE i podstawy MDOF,
- analiza częstotliwości (FFT, widma, piki),
- sterowanie dyskretne i filtracja,
- pomiary czasu rzeczywistego (WCRT, jitter),
- synchronizacja czasu i timestamping,
- praktyka integracji (magistrala + watchdog + degradacja).

## Checklisty
- Masz jeden diagram architektury z podpisanymi opóźnieniami.
- Masz zdefiniowane metryki: błąd regulacji, jitter, saturacje, wibracje, temperatura.
- Masz plan degradacji: co robisz, gdy RT/magistrala nie wyrabia.

## Slajdy (tekstowe)
### Slajd 1: Co budujemy
- Wirtualne laboratorium do testów systemu wirującego przed hardware
- Neutralny kontekst, praktyka RT i sterowania

### Slajd 2: Bloki systemu
- Plant (wirnik, konstrukcja)
- Drive (moment/prąd)
- Kontroler (pętle i tryby)
- Diagnostyka i safety
- Magistrala i czas

### Slajd 3: Kolejność prac
- Model -> sterowanie -> RT pomiary -> magistrala -> HIL

## Pytania do studentów
1. Który blok w architekturze labu jest „single point of truth” dla czasu i dlaczego?
2. Jakie 5 metryk uznasz za obowiązkowe, żeby lab był narzędziem inżynierskim, a nie demo?
3. Jak rozdzielisz odpowiedzialności między plant, kontroler, magistralę i diagnostykę, aby łatwo debugować?
4. Jakie scenariusze awaryjne muszą być testowane od początku (nie „na końcu projektu”)?

## Projekty studenckie
- „Architektura referencyjna labu”: repo z diagramem, kontraktami danych i minimalnym harness.
- „Scenario pack”: zestaw scenariuszy (zakłócenie, saturacja, opóźnienie, dropout) + raport metryk.
- „Metrics spec”: specyfikacja logów i walidator kompletności danych (wersjonowanie schematu).

## BONUS
- Największy zwrot daje wczesne wprowadzenie „replay”: jeśli potrafisz odtwarzać ten sam scenariusz 1:1, to każda zmiana algorytmu jest mierzalna.
