# EtherCAT / RTOS / Linux RT w Maszynach Wirujących — Wyklady

Ten cykl dotyczy **praktycznej inżynierii czasu rzeczywistego i automatyki** dla maszyn wirujących:
- Wirówek przemysłowych
- Wrzecion CNC
- Kompresorów
- Turbin
- Innych napędów o wysokich prędkościach obrotowych

Uwaga bezpieczeństwa:
- Materiał jest **celowo neutralny technologicznie** i nie opisuje konstrukcji ani parametrów specyficznych dla zastosowań o charakterze proliferacyjnym.
- Skupiamy się na **ogólnych zasadach sterowania, synchronizacji, diagnostyki i bezpieczeństwa funkcjonalnego**.

## Dla kogo

- Dla osób od automatyki/embedded/robotyki, które chcą rozumieć **RT w praktyce** (jitter, WCRT, budżety czasu).
- Dla osób wdrażających sieci ruchu (np. EtherCAT) i chcących uniknąć typowych pułapek integracji (cykl, DC, watchdog).
- Dla osób, które muszą połączyć sterowanie z diagnostyką i safety (SIL) w jednym produkcie.

## Co umiesz po cyklu (outcomes)

- Narysować architekturę sterowania maszyną wirującą i zidentyfikować krytyczną ścieżkę end-to-end.
- Zbudżetować czas, zmierzyć jitter/WCRT i wyciągnąć wnioski projektowe.
- Zaprojektować podział odpowiedzialności: drive vs master, RTOS vs Linux PREEMPT_RT.
- Ustawić EtherCAT pod sterowanie cykliczne (konceptualnie): co ma iść w PDO, co w SDO, kiedy potrzebujesz DC.
- Zdiagnozować "oscylacje, które pojawiają się sporadycznie" jako problem czasu, filtrów lub rezonansów.
- Zaprojektować podstawowy system condition monitoring + degradacja + safe stop.

## Jak korzystać z materiału

- Czytaj w kolejności 1–6, a 7 traktuj jako "warsztat".
- W każdym wykładzie szukaj sekcji:
  - **Wstęp teoretyczny** (dlaczego to jest ważne)
  - **Co mierzyć** (bo bez danych debugujesz na ślepo)
  - **Pułapki** (najczęstsze źródła problemów)
  - **Checklisty** (gotowe punkty do projektu i review)
  - **Zadania praktyczne** (ćwiczenia)

## Założenia i ograniczenia

- Opisujemy wzorce inżynierskie dla maszyn wirujących w przemyśle; nie wchodzimy w wrażliwe detale aplikacyjne.
- Konkretne liczby (częstotliwości, parametry) zależą od obiektu; tu skupiamy się na metodzie doboru na podstawie pomiarów.
- EtherCAT/RTOS/Linux RT traktujemy jako narzędzia: wybór ma wynikać z wymagań czasowych i ryzyka.

## Spis treści

| # | Plik | Temat |
|---|------|-------|
| 1 | [01_architektura_systemu.md](./01_architektura_systemu.md) | Architektura sterowania, krytyczna ścieżka, warstwy |
| 2 | [02_czas_rzeczywisty_i_jitter.md](./02_czas_rzeczywisty_i_jitter.md) | Czas rzeczywisty, jitter, WCET/WCRT, Linux PREEMPT_RT |
| 3 | [03_ethercat_w_praktyce.md](./03_ethercat_w_praktyce.md) | EtherCAT, cykle, Distributed Clocks, watchdog |
| 4 | [04_napedy_i_tlumienie_drgan.md](./04_napedy_i_tlumienie_drgan.md) | Napędy, kaskady pętli, FFT, filtry notch, rampy/jerk |
| 5 | [05_diagnostyka_i_condition_monitoring.md](./05_diagnostyka_i_condition_monitoring.md) | Condition monitoring, baseline, eskalacja alarmów |
| 6 | [06_bezpieczenstwo_funkcjonalne_sil.md](./06_bezpieczenstwo_funkcjonalne_sil.md) | Safety, FSM bezpieczeństwa, watchdog, fault injection |
| 7 | [07_checklisty_i_laby.md](./07_checklisty_i_laby.md) | Checklisty wdrożeniowe, mini-laby, metryki |

## Szybki indeks po problemach

- "Sterowanie czasem się rozjeżdża" → Wykład 2
- "Po dodaniu telemetrii pojawiły się oscylacje" → Wykład 2 + 3
- "Wibracje rosną w jednym zakresie prędkości" → Wykład 4 + 5
- "Czemu DC/synchronizacja ma znaczenie" → Wykład 3
- "Jak zdefiniować safe stop i degradację" → Wykład 6

## Struktura kazdego wykladu

Kazdy wyklad zawiera:
1. **Wstęp teoretyczny** — geneza problemu, wyjaśnienie "dlaczego to ważne"
2. **Część główna** — pojęcia, definicje, wzorce z kodem
3. **Przykłady praktyczne** — fragmenty kodu C/Python
4. **Podsumowanie i checklisty** — co zapamiętać
5. **Pytania do dyskusji** — pytania kontrolne
6. **Zadania praktyczne** — ćwiczenia do samodzielnego wykonania
7. **BONUS** — dodatkowe uwagi i triki

---

*Wykłady zostały rozbudowane o wstępy teoretyczne, wyjaśnienia pojęć i przemówienia "profesorskie".*
