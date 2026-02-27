# Wyklad 0: Pojecia na start i zasady architektury RT

## Cel
Ustalic jezyk i "twarde" zasady, ktore beda wracac w kazdym wykladzie:
- deterministycznosc vs wydajnosc srednia,
- cykl sterowania i budzet czasu end-to-end,
- podzial odpowiedzialnosci miedzy warstwami.

> TL;DR: W sterowaniu RT przegrywasz przez ogony opoznien i bledy architektury (blokady, GC, IO), a nie przez brak mocy CPU.

## Kontekst robotyki (dlaczego te pojecia wracaja)
Robotyka doklada do klasycznego problemu maszyn wirujacych:
- wiele petli jednoczesnie (naped, przegub, task-space, percepcja),
- duzo sensorow z roznymi opoznieniami (IMU, kamera, czujniki sily),
- wymagania bezpieczenstwa przy pracy obok czlowieka (coboty).

Wniosek: "RT" to nie tylko sterowanie napedu. To takze:
- timestamping i synchronizacja,
- deterministyczne kolejkowanie danych,
- watchdog i reakcje na bledy.

## Kluczowe pojecia
### Cykl (T) i deadline
- `T` to okres petli (co ile wykonujesz iteracje sterowania).
- `deadline` to maksymalny dopuszczalny czas od startu iteracji do gotowego wyjscia.

W praktyce wymagasz:
```text
t_iteracji <= deadline  (dla prawie wszystkich iteracji; liczysz percentyle)
```

### Latency i jitter
- `latency` to opoznienie (np. pomiar->aktuacja).
- `jitter` to zmiennosc okresu/latency.

Wniosek praktyczny:
- latency stale da sie kompensowac,
- jitter losowy rozwala margines fazy i stabilnosc.

### WCET/WCRT
- WCET: worst-case execution time (czas samego kodu).
- WCRT: worst-case response time (czas "od gotowosci" do wykonania, z czekaniem na CPU).

W RT interesuje Cie WCRT.

## Zlota zasada: najpierw narysuj krytyczna sciezke
Krytyczna sciezka end-to-end:
```text
sensor -> timestamp/bufor -> filtr/estymacja -> regulator -> transport -> drive -> plant
```
Jesli nie potrafisz podac budzetu czasu dla tej sciezki, to architektura jest "na wiare".

## "Nie rob tego" w sciezce RT
- brak alokacji dynamicznej w petli (new/malloc),
- brak IO blokujacego (dysk/siec) w watku RT,
- brak mutexow na danych krytycznych (priority inversion),
- brak GC w watku RT (Python/JS/Java w tej sciezce),
- brak logowania synchronicznego.

## Co logowac od pierwszego dnia
Minimum telemetrii do debugowania:
- `t_start`, `t_end`, `rt_loop_us`,
- `miss_deadline` (0/1),
- `omega_set`, `omega_meas`, `omega_err`,
- `u_cmd` (moment/prad zadany) i saturacje,
- status komunikacji (dropouty).

## Checklisty
- Masz definicje: T, deadline, WCRT, jitter.
- Masz diagram sciezki end-to-end i miejsce, gdzie mierzysz czas.
- Masz zasade: RT nie blokuje sie na niczym poza wlasnym zegarem.

## Zadania (praktyka)
1. Narysuj krytyczna sciezke dla robota, ktory pobiera probki i uruchamia modul procesu (np. urzadzenie wirujace).
2. Zaznacz, ktore elementy to hard RT, firm RT i soft RT, i jakie maja budzety.

## Pytania do studentow
1. Dlaczego w RT kluczowe sa ogony opoznien, a nie srednia?
2. Jak odroznic WCRT od WCET w praktyce (na danych z systemu)?
3. Co w Twojej architekturze jest "twarde RT", a co moze byc soft-RT, i dlaczego?
4. Jakie 5 sygnalow musisz logowac, zeby nie debugowac w ciemno?

## Projekty studenckie
- "RT telemetry spec": specyfikacja ramki telemetrycznej + walidator schematu.
- "End-to-end budget": narzedzie liczace opoznienie end-to-end z timestampow.
- "RT rules linter": checklista/linterna zasad (brak IO/alokacji/mutexow w petli RT) na review.

## BONUS
- Zanim dotkniesz konfiguracji kernela, wywal z petli RT wszystko, co moze blokowac (IO/mutex/alokacje). To daje zwykle wiecej niz tuning systemu.
