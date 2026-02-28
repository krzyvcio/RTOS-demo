# Sterowanie Maszynami Wirujacymi — Architektura Oprogramowania (Wyklady)

Ten folder zawiera rozbudowane wyklady "od software" (warstwy, watki RT, IPC, magistrale, deployment, testy). Kazdy wyklad zawiera:
- Wstep teoretyczny z wyjasnieniami
- Przemowienia "profesorskie" (dlaczego to jest wazne)
- Szczegolowe wyjasnienia pojeciowe
- Przyklady kodu
- Zadania praktyczne

Kontekst jest neutralny aplikacyjnie: system wirujacy jako klasa obiektu przemyslowego.

## Spis wykladow

| # | Plik | Temat |
|---|------|-------|
| 0 | `00_pojecia_i_zasady.md` | Pojecia podstawowe: cykl, deadline, latency, jitter, WCET/WCRT, zlota zasada krytycznej sciezki |
| 1 | `01_architektura_warstwowa.md` | Architektura warstwowa (W1-W4): od sprzetu do HMI, FSM, integracja robot+modul |
| 2 | `02_ethercat_master_i_timing.md` | EtherCAT: dobór cyklu, Distributed Clocks, watchdog, sygnaly cykliczne/acykliczne |
| 3 | `03_rtos_vs_linux_rt.md` | RTOS vs Linux PREEMPT_RT: wzorzec watku RT, izolacja, AI/ML obok RT |
| 4 | `04_jezyki_i_interfejsy.md` | Wielojezycznosc: C/Rust dla RT, Python/Node dla nadzoru, kontrakty danych |
| 5 | `05_ipc_lock_free_shared_memory.md` | IPC: SPSC ring buffer, polityka przepelnienia (drop/overwrite) |
| 6 | `06_kontenery_gdzie_tak_gdzie_nie.md` | Docker: tak dla HMI/nadzoru, nie dla hard-RT |
| 7 | `07_roadmapa_implementacji.md` | Roadmap: model -> RT -> magistrala -> HMI -> safety |
| 8 | `08_safety_watchdog_i_degradacja.md` | Safety: watchdog wielopoziomowy, FSM bezpieczenstwa, fault injection |
| 9 | `09_slajdy_tekstowe_full.md` | Slajdy tekstowe - podsumowanie wszystkich wykladow |
| 10 | `10_wyklad_2035_robotyka_wirowki_rtos.md` | Zintegrowany obraz 2035: roboty + wirowki + RTOS, trendy, etyka |

## Jak czytac (sciezki)

### Sciezka A: Budujesz system RT
`00 → 02 → 03 → 05 → 08`

Wyklad 0 (pojecia) -> Wyklad 2 (EtherCAT) -> Wyklad 3 (RTOS) -> Wyklad 5 (IPC) -> Wyklad 8 (Safety)

### Sciezka B: Budujesz cala platforme (RT + nadzor + CI)
`00 → 01 → 03 → 04 → 05 → 06 → 07 → 08`

Wyklad 0 -> Wyklad 1 (architektura) -> Wyklad 3 (RTOS) -> Wyklad 4 (jezyki) -> Wyklad 5 (IPC) -> Wyklad 6 (kontenery) -> Wyklad 7 (roadmap) -> Wyklad 8 (safety)

### Sciezka C: Chcesz "zintegrowany obraz" (robotyka + wirowki + trendy)
`10 → 00 → 01 → 02 → 08`

Zacznij od Wykladu 10 (kontekst 2035), potem wroc do podstaw (00), architektury (01), komunikacji (02) i safety (08).

## Struktura kazdego wykladu

Kazdy wyklad ma nastepujaca strukture:

1. **Wstep teoretyczny** — geneza problemu, wyjasnienie "dlaczego to wazne"
2. **Wyklad glowny** — pojecia, definicje, wzorce
3. **Przyklady praktyczne** — kod, konfiguracje
4. **Podsumowanie i checklisty** — co zapamietac
5. **Pytania do dyskusji** — pytania kontrolne
6. **Zadania praktyczne** — cwiczenia do samodzielnego wykonania
7. **BONUS** — dodatkowe uwagi i triki

## Zasady architektoniczne (podsumowanie)

| Zasada | Wyklad |
|--------|--------|
| Hard RT nie blokuje sie na niczym poza wlasnym zegarem | 00, 03 |
| Krytyczna sciezka ma zdefiniowany budzet czasu | 00, 02 |
| Jedna warstwa = jedna odpowiedzialnosc | 01 |
| Kontrakty sa wersjonowane | 01, 04 |
| IPC = lock-free ring buffer | 05 |
| Kontenery dla IT, nie dla RT | 06 |
| Safety jako osobny kanal decyzyjny | 08 |
| Fault injection jako standard testow | 08 |

## Wymagania dla systemu RT

- Pomiar `rt_loop_us` i histogram
- Reakcja na missed deadline (degradacja/safe stop)
- Oddzielone watki: RT vs logowanie/HMI
- Telemetria od pierwszego dnia

---

*Wyklady zostaly rozbudowane o wstepy teoretyczne, przemowienia "profesorskie" i szczegolowe wyjasnienia pojeć.*
