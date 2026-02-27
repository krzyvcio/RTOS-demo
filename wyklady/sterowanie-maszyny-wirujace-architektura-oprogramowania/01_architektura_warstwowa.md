# Wyklad 1: Architektura warstwowa (od sprzetu do HMI)

## Cel
Zaprojektowac warstwy tak, by:
- krytyczne petle byly deterministyczne,
- reszta byla elastyczna i latwa do rozwoju,
- interfejsy miedzy warstwami byly minimalne i testowalne.

## Warstwy (referencyjny podzial)
```text
Warstwa 4: modelowanie/symulacja (offline)
Warstwa 3: nadzor/HMI (soft-RT)
Warstwa 2: kontroler RT / master (firm-RT)
Warstwa 1: drive/MCU (hard-RT)
Warstwa 0: sprzet (opcjonalnie FPGA)
```

## Regula architektoniczna
Jedna warstwa ma jedna glowna odpowiedzialnosc:
- W1: "trzymam prad/moment i reaguje bezpiecznie"
- W2: "koordynuje cykl, licze regulacje, trzymam timing"
- W3: "wizualizuje, loguje, alarmuje, nie psuje RT"
- W4: "projektuje i waliduje algorytmy offline"

## Kontrakty miedzy warstwami (software-first)
Zdefiniuj jawne API (najlepiej jako struktury danych + wersjonowanie):
- komendy: setpointy, tryby, rampy
- pomiary: omega, statusy, temperatury, wibracje
- zdarzenia: alarmy, degradacja, safe stop

Minimalny kontrakt W2<->W3:
- W2 publikuje probki telemetryczne do ring buffera,
- W3 czyta i wysyla do bazy/dash.

## Tryby pracy (FSM) jako element "klejacy"
Maszyny wirujace maja trudne przejscia:
- start, rampy, przejscie przez rezonanse
- praca ustalona
- stop kontrolowany
- stop awaryjny

FSM powinien byc po stronie warstwy, ktora ma najtwardsze wymagania na reakcje (zwykle W1/W2).

## Integracja robot + modul procesu (np. urzadzenie wirujace)
W praktyce masz dwa "subsystemy" o roznych rytmach:
- robot: percepcja + plan + kontrola, tryby bezpieczenstwa, ruch w przestrzeni,
- modul procesu: wlasny kontroler RT (naped) + nadzor + safe stop.

Wzorzec integracji software:
- robot i modul maja osobne FSM,
- jest nadrzedny orkiestrator (workflow) ktory spina stany i warunki,
- krytyczne reakcje (safe stop) sa lokalne w kazdym module, nie centralne.

Minimalny kontrakt (przyklad):
- `module_state`: READY/RUNNING/FAULT/SAFE_STOP
- `robot_state`: IDLE/MOVING/INTERLOCK/FAULT
- `events`: ALARM, INTERLOCK, E_STOP

## Trend 2035: komorki autonomiczne
W 2035 typowe jest, ze:
- modul procesu ma wbudowana predykcyjna diagnostyke,
- robot ma ograniczenia energii i aktywne monitorowanie kolizji,
- cala komorka ma audit trail i powtarzalnosc (compliance).

## Checklisty
- Kazda warstwa ma osobny budzet czasu.
- Watek RT jest oddzielony od logowania i sieci IT.
- Interfejsy sa wersjonowane i testowane.

## Zadania (praktyka)
1. Zaprojektuj FSM dla modulu procesu i FSM dla robota, a potem opisz ich synchronizacje.
2. Wypisz 5 zdarzen, ktore musza skutkowac natychmiastowa reakcja lokalna (watchdog/safe stop), bez czekania na orkiestratora.

## Pytania do studentow
1. Jakie sa granice odpowiedzialnosci kazdej warstwy i co sie stanie, gdy warstwa wyzsza przestanie dzialac?
2. Jakie kontrakty danych sa minimalne, zeby robot i modul procesu byly integrowalne (FSM + events)?
3. Co jest single point of failure w tej architekturze i jak je ograniczasz?
4. Jak zapewnisz, ze nadzor/HMI nigdy nie wplynie na timing petli RT?

## Projekty studenckie
- "Two FSM integration": implementacja dwoch FSM (robot+modul) + orkiestrator workflow w soft-RT.
- "Contract versioning": wersjonowanie kontraktu danych i testy kompatybilnosci wstecznej.
- "Interlock demo": prosty interlock (strefy/warunki) wymuszajacy bezpieczne przejscia stanow.

## BONUS
- Jezeli interfejs miedzy warstwami ma wiecej niz kilka kluczowych sygnalow w cyklu, to zwykle jest przerośniety; redukcja kontraktu czesto poprawia deterministyke i niezawodnosc.
