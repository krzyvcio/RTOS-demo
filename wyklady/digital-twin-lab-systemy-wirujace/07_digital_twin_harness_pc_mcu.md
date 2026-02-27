# Wykład 7: Digital Twin bez wirnika (PC symuluje plant, kontroler myśli że steruje hardware)

## Cel
Zbudować „harness” do testów:
- PC symuluje obiekt (ODE/MDOF),
- kontroler (Linux RT lub MCU) liczy sterowanie,
- wymiana danych przypomina realny system (opóźnienia, dropouty, timestampy).

> TL;DR: Kontroler nie powinien wiedzieć, czy steruje prawdziwym obiektem czy modelem.

## Wzorzec architektury
W najprostszej formie:
- kontroler wysyła `torque_cmd`,
- plant odsyła `omega_meas` (z szumem i opóźnieniem),
- dodatkowo: flagi saturacji, wibracje (syntetyczne), temperatury (model).

## Co musi umieć harness
- wstrzykiwanie opóźnień i jitteru,
- wstrzykiwanie dropoutów,
- przełączanie trybów pracy (FSM),
- logowanie spójne dla online/offline,
- deterministyczny replay scenariuszy (regresja).

## Checklisty
- Jest jeden format logu używany w całym labie.
- Jest zestaw scenariuszy regresji (skok zakłócenia, saturacja, dropout, opóźnienie).
- Jest mechanizm „replay” i porównywania metryk.

## Slajdy (tekstowe)
### Slajd 1: Idea harness
- kontroler steruje „jakby” hardware
- PC odgrywa plant

### Slajd 2: Co wstrzykujemy
- opóźnienie, jitter, dropouty
- szum i saturacje

## Pytania do studentów
1. Jak zaprojektujesz harness tak, aby kontroler „nie wiedział”, że steruje modelem?
2. Jakie scenariusze powinny być częścią regresji (zawsze), a jakie tylko testami eksploracyjnymi?
3. Jak wymusisz deterministyczny replay (seed, wersje schematu, snapshoty parametrów)?
4. Jak rozdzielisz role: sterowanie (RT) vs diagnostyka vs logowanie, aby nie mieszać odpowiedzialności?

## Projekty studenckie
- „Scenario runner”: uruchamianie scenariuszy z plików (opóźnienia, dropouty, zakłócenia) + raport metryk.
- „Replay engine”: odtwarzanie logów w trybie offline i porównanie metryk „przed/po” zmianie regulatora.
- „Contract validator”: walidator ramek danych (wymagane pola, wersja schematu, spójność timestampów).

## BONUS
- Najlepsze harnessy mają „tryb okrutny”: potrafią generować rzadkie, ale realistyczne zdarzenia (piki opóźnień, dropouty) i wymuszać odporność systemu.
