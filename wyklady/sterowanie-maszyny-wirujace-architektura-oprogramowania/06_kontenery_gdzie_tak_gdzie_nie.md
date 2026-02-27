# Wyklad 6: Kontenery (Docker) — gdzie tak, gdzie nie

## Cel
Ustalic bezpieczna polityke deploymentu:
- kontenery dla HMI i narzedzi,
- brak kontenerow w sciezce hard RT,
- izolacja i powtarzalnosc bez psucia determinizmu.

> TL;DR: Kontener nie powinien owijac watku hard-RT. Mozesz konteneryzowac nadzor i tooling.

## Polityka per warstwa
- MCU/RTOS: nie
- RT master: nie dla petli RT; tak dla narzedzi obok (build/monitoring) jesli nie ingeruja w timing
- HMI/nadzor: tak (docker-compose)
- modelowanie: tak (powtarzalnosc)

## Najczestsze bledy
- uruchamianie petli RT w kontenerze z domyslnymi cgroups
- brak limitow na telemetrie i logi
- mieszanie ruchu IT i RT w tej samej sciezce

## Checklisty
- Petla RT dziala na hoście, na izolowanym rdzeniu.
- Usługi HMI/logowania sa odseparowane i rate-limited.

## Kontekst autolabow (2035)
Autonomiczne laboratoria i linie produkcyjne zwykle maja:
- wiele uslug (workflow, LIMS, bazy, dashboardy),
- wymog powtarzalnosci srodowiska,
- audyt i zgodnosc.

Kontenery sa idealne dla warstwy nadzoru, ale:
- musisz pilnowac, by nie ingerowaly w RT (oddzielne hosty/rdzenie/priorytety).

## Pytania do studentow
1. Jakie ryzyko wnosi uruchomienie petli RT w kontenerze z domyslnymi cgroups?
2. Jak zaprojektujesz deployment tak, aby HMI/DB bylo powtarzalne, ale RT bylo stabilne?
3. Jak ograniczysz wplyw telemetrii (sieciowo i CPU) na host z petla RT?
4. Jak odtworzysz srodowisko (versions) bez zmiany charakterystyk czasu rzeczywistego?

## Projekty studenckie
- "Compose stack": docker-compose dla HMI/DB/API + dokumentacja interfejsu do W2.
- "Rate limiter": serwis ograniczajacy strumien telemetryczny i mierzacy dropy.
- "Deployment split": propozycja podzialu na host RT i host IT z kontraktami danych.

## BONUS
- Kontenery to narzedzie do powtarzalnosci. Jezeli ich uzycie pogarsza timing, to znaczy, ze probujesz konteneryzowac niewlasciwa warstwe.
