# Sterowanie Maszynami Wirujacymi — Architektura Oprogramowania (Wyklady)

Ten folder zamienia Twoj opis na cykl wykladow "od software" (warstwy, watki RT, IPC, magistrale, deployment, testy).
Kontekst jest neutralny aplikacyjnie: system wirujacy jako klasa obiektu przemyslowego.

## Spis wykladow
1. `00_pojecia_i_zasady.md`
2. `01_architektura_warstwowa.md`
3. `02_ethercat_master_i_timing.md`
4. `03_rtos_vs_linux_rt.md`
5. `04_jezyki_i_interfejsy.md`
6. `05_ipc_lock_free_shared_memory.md`
7. `06_kontenery_gdzie_tak_gdzie_nie.md`
8. `07_roadmapa_implementacji.md`
9. `08_safety_watchdog_i_degradacja.md`
10. `09_slajdy_tekstowe_full.md`
11. `10_wyklad_2035_robotyka_wirowki_rtos.md`

## Jak czytac (sciezki)
- Jezeli budujesz system RT: 00 -> 03 -> 05 -> 02 -> 08.
- Jezeli budujesz cala platforme (RT + nadzor + CI): 00 -> 01 -> 05 -> 06 -> 07 -> 08.
- Jezeli chcesz "zintegrowany obraz" (robotyka + wirowki + RTOS + trendy): zacznij od 10, potem wracaj do modulow 00-08.

