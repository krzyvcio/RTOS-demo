# Wykład 4: Linux PREEMPT_RT, pomiary deterministyki i wirtualizacja

## Cel
Zbudować środowisko, w którym:
- pętla sterowania ma mierzalny jitter i WCRT,
- potrafisz wykonać testy obciążeniowe,
- umiesz odróżnić „spadek jakości” od „brak deterministyki”.

> TL;DR: Bez pomiaru p99/p99.9 czasu iteracji nie wiesz, czy architektura jest RT.

## Linux PREEMPT_RT: co to daje
- preempcja w kernelu jest bardziej „RT-friendly”,
- łatwiej uzyskać przewidywalny scheduler dla wątków krytycznych,
- nadal potrzebujesz dyscypliny (priorytety, izolacja, przerwania).

## Pomiary (przykładowe klasy narzędzi)
- testy latencji scheduler/timer,
- narzędzia do wykrywania „szumu” systemu,
- tracing (do znalezienia źródła ogonów opóźnień).

Wniosek praktyczny:
- zrób baseline,
- po każdej zmianie mierz ponownie,
- patrz na ogony rozkładów.

## Wirtualizacja (QEMU/VirtualBox): do czego się nadaje
Wirtualizacja jest dobra do:
- pipeline CI dla logiki i symulacji,
- testów integracyjnych bez sprzętu,
- powtarzalności środowiska.

Wirtualizacja jest słaba do:
- oceny „twardego RT” (host i hypervisor wpływają na timing).

## Checklisty
- Masz baseline p99/p99.9 czasu iteracji pętli.
- Masz test obciążeniowy (telemetria/CPU/IO) i widzisz jego wpływ.
- Masz strategię degradacji przy missed deadlines.

## Slajdy (tekstowe)
### Slajd 1: Co mierzymy
- jitter, WCRT, missed deadlines
- percentyle, nie średnia

### Slajd 2: Wirtualizacja
- dobra do CI i integracji
- ostrożnie do „twardego RT”

## Pytania do studentów
1. Jakie metryki czasu rzeczywistego uznasz za kluczowe (i dlaczego percentyle, a nie średnia)?
2. Jak zaprojektujesz test obciążeniowy, który realistycznie „psuje” timing (CPU/IO/telemetria)?
3. Kiedy wyniki z VM są użyteczne, a kiedy wprowadzają w błąd?
4. Jaką strategię degradacji zastosujesz, gdy p99/p99.9 przekracza budżet?

## Projekty studenckie
- „RT baseline”: skrypt zbierający histogram/percentyle `rt_loop_us` w kilku konfiguracjach systemu.
- „Load scenarios”: zestaw scenariuszy obciążeniowych + raport wpływu na ogony opóźnień.
- „VM CI”: pipeline CI uruchamiający testy logiki degradacji i watchdogów w środowisku wirtualnym.

## BONUS
- Zanim „tuningujesz” kernel, usuń najpierw 90% problemów architektonicznych: IO w pętli, mutexy, alokacje. Kernel RT nie uratuje złej architektury.
