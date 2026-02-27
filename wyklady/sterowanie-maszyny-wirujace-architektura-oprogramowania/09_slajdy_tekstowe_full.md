# Slajdy (tekstowe): Sterowanie maszynami wirujacymi — architektura oprogramowania

## Slajd 1: O co chodzi
- Jeden system, wiele warstw
- Rozne wymagania czasowe
- RT psuje sie przez jitter i blokady

## Slajd 2: Warstwy
- W1: drive/MCU (hard RT)
- W2: master (firm RT)
- W3: nadzor/HMI (soft RT)
- W4: modelowanie (offline)

## Slajd 3: Kontekst 2035
- Roboty + moduly procesu + diagnostyka jako jedna komorka autonomiczna
- Wygrywa deterministyka, safety i diagnozowalnosc

## Slajd 4: EtherCAT w jednym zdaniu
- Cykliczna wymiana danych sterujacych i pomiarowych
- Synchronizacja (DC) gdy liczy sie faza i sampling

## Slajd 5: RTOS vs Linux RT
- RTOS: mikrosekundy, pelna kontrola
- PREEMPT_RT: ekosystem, ale wymaga dyscypliny

## Slajd 6: Watek RT - wzorzec
- tick -> wejscia -> regulator -> wyjscia -> ring buffer
- zero IO, zero alokacji, zero mutexow

## Slajd 7: IPC RT->nonRT
- shared memory + lock-free ring buffer (SPSC)
- polityka: drop/overwrite + licznik

## Slajd 8: Kontenery
- tak: HMI, DB, API, modelowanie
- nie: petla hard-RT

## Slajd 9: Roadmapa
- model -> sterowanie -> RT pomiary -> magistrala -> HMI -> twin -> safety

## Slajd 10: Safety
- watchdog wielopoziomowy
- degradacja i safe stop
- fault injection jako test

## Slajd 11: Integracja robot + modul procesu
- Dwa FSM: robot i modul
- Orkiestrator (workflow) spina stany, ale awarie obsluguja moduly lokalnie
- Kontrakty: READY/RUNNING/FAULT/SAFE_STOP + zdarzenia

## Slajd 12: Diagnostyka predykcyjna
- Baseline per tryb pracy
- Trendy: WARNING -> DEGRADED -> SAFE_STOP
- Diagnostyka asynchroniczna (nie psuje RT)

## Slajd 13: Etyka i odpowiedzialnosc
- Safety jako osobny kanal decyzyjny
- ML poza krytyczna sciezka RT (timeout, fallback)
- Logi i audit trail, testy awaryjne jako standard
