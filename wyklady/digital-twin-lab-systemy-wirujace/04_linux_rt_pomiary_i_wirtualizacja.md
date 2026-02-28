# Wykład 4: Linux PREEMPT_RT, pomiary deterministyki i wirtualizacja

## Część I: Wstęp teoretyczny - Czas rzeczywisty i jego znaczenie

### Geneza: dlaczego "wystarczająco szybko" nie wystarcza

W świecie sterowania cyfrowego mamy do czynienia z fundamentalnym problemem: **fizyka nie czeka na obliczenia**. Wirnik kręci się z prędkością 30 000 RPM - to 500 obrotów na sekundę, 500 Hz. Jeśli pętla sterowania działa z częstotliwością 1 kHz, to między dwoma wykonaniami pętli wirnik obróci się o pół obrotu. Jeśli pętla działa z opóźnieniem, to sterowanie "goni" rzeczywistość.

Problem staje się krytyczny, gdy:
- Mamy szybkie procesy (wysokie RPM, szybkie zmiany obciążenia)
- Mamy zamknięte pętle (sterowanie zależy od szybkiego sprzężenia zwrotnego)
- Mamy wiele współpracujących pętli (synchronizacja)

W takich systemach "wystarczająco szybko" nie wystarcza - potrzebujemy **przewidywalności czasowej**. Nie chodzi o to, żeby było szybko, ale o to, żeby było **zawsze i wszędzie tak samo szybko**.

### Czym jest czas rzeczywisty (RT)

Czas rzeczywisty (Real-Time, RT) nie oznacza "szybko". Oznacza "**z gwarantowanym deadline**". System RT gwarantuje, że zadanie zostanie wykonane w określonym czasie - lub poinformuje o przekroczeniu.

Wyróżniamy:
- **Soft RT**: Przekroczenie deadline jest niepożądane, ale system może kontynuować (np. streaming wideo)
- **Hard RT**: Przekroczenie deadline oznacza awarię (np. sterowanie lotem, hamulcami)

W systemach wirujących zwykle mamy do czynienia z "mid-soft" RT - przekroczenie deadline nie jest katastrofą, ale powinno być rzadkie i obsłużone.

### PREEMPT_RT vs RTOS vs Linux vanilla

**Linux vanilla**:
- Domyślny scheduler, wiele źródeł opóźnień (preempcja kernel, locki, IRQ)
- Świetny do ogólnych zastosowań
- Nieprzewidywalny dla zastosowań RT

**Linux PREEMPT_RT**:
- Patch do kernela Linux dodający pełną preempcję
- Większość sekcji krytycznych jest przerywalna
- Zachowuje kompatybilność z aplikacjami Linux
- Gwarancje soft-RT, zbliżone do mid-hard

**RTOS (FreeRTOS, Zephyr, RT-Thread)**:
- Minimalny kernel, dedykowany do embedded
- Pełna kontrola nad czasem
- Mniej "wygodny" niż Linux
- Hard-RT possible

Wybór zależy od wymagań projektu. Dla laboratoriow - PREEMPT_RT daje dobry kompromis między możliwościami i przewidywalnością.

### Przemówienie Profesora

Kiedy zaczynałem w latach 90., nie było Linux PREEMPT_RT. Mieliśmy "bare metal" albo bardzo drogie systemy RT. Linux w sterowaniu był nie do pomyślenia.

Dzisiaj sytuacja jest inna - PREEMPT_RT daje wam "prawie-RT" na zwykłym sprzęcie. Ale - i to jest ważne - wciąż potraficie "zepsuć" determinizm zwykłym kodem.

Pamiętajcie: Linux PREEMPT_RT daje wam podstawę, ale to, co wy napiszcie w aplikacji, może tę podstawę zniszczyć. Alokacje pamięci, I/O, mutexy - to wszystko może wprowadzić opóźnienia.

W tym wykładzie nauczymy się mierzyć te opóźnienia - bo co nie jest mierzone, nie jest kontrolowane.

## Cel
Zbudować środowisko, w którym:
- pętla sterowania ma mierzalny jitter i WCRT,
- potrafisz wykonać testy obciążeniowe,
- umiesz odróżnić "spadek jakości" od "brak deterministyki".

> TL;DR: Bez pomiaru p99/p99.9 czasu iteracji nie wiesz, czy architektura jest RT.

## Część II: Pomiary determinizmu

## Pomiary (przykładowe klasy narzędzi)
- testy latencji scheduler/timer,
- narzędzia do wykrywania "szumu" systemu,
- tracing (do znalezienia źródła ogonów opóźnień).

Wniosek praktyczny:
- zrób baseline,
- po każdej zmianie mierz ponownie,
- patrz na ogony rozkładów.

### Co mierzyć - metryki

**Latencja pętli (loop latency)**:
- Czas od przerwania timera do zakończenia pętli sterowania
- Mierzone w mikrosekundach

**Jitter**:
- Wariancja czasu wykonania
- Różnica między najszybszym a najwolniejszym wykonaniem

**WCRT (Worst-Case Response Time)**:
- Najgorszy przypadek czasu od przerwania do reakcji
- Kluczowe dla hard-RT

**Percentyle p50, p95, p99, p99.9**:
- p50 = median
- p95 = 5% przekroczeń
- p99 = 1% przekroczeń
- p99.9 = 0.1% przekroczeń

Dlaczego percentyle, a nie średnia? Bo średnia ukrywa problemy. System może mieć średnią 100us, ale p99.9 = 10ms - co jest katastrofą dla niektórych aplikacji.

### Przemówienie Profesora

Najgorsze, co możecie zrobić, to powiedzieć "średnia jest OK". Bo średnia nic wam nie mówi o ogonach.

Wyobraźcie sobie: macie 1000 wykonań pętli. 999 trwa 100 μs - super. Ale 1 trwa 50 ms. Wasza średnia to 149 μs - wygląda dobrze. Ale ta jedna pętla 50 ms może spowodować utratę kontroli, przeregulowanie, a w skrajnym przypadku - awarię.

Patrzcie na OGONY. Zawsze.

## Wirtualizacja (QEMU/VirtualBox): do czego się nadaje
Wirtualizacja jest dobra do:
- pipeline CI dla logiki i symulacji,
- testów integracyjnych bez sprzętu,
- powtarzalności środowiska.

Wirtualizacja jest słaba do:
- oceny "twardego RT" (host i hypervisor wpływają na timing).

### Dlaczego wirtualizacja nie jest dobra dla hard-RT

Wirtualizacja dodaje dodatkowe warstwy:
- Hypervisor (VirtualBox, QEMU, KVM)
- Scheduler hosta
- Emulacja sprzętu

Każda z tych warstw wprowadza opóźnienia, których nie kontrolujecie. Guest może być "wstrzymany" przez hosta na nieokreślony czas.

Dla testów logiki i symulacji - wirtualizacja jest świetna. Możecie uruchamiać testy automatycznie, w powtarzalnym środowisku, bez dedykowanego sprzętu.

Dla pomiarów determinizmu - NIE używajcie wirtualizacji. Pomiary będą nieprawdziwe.

### Przemówienie Profesora

Kiedyś spotkałem zespół, który chwalił się: "Nasz system ma jitter tylko 50 μs!"

Pytanie: "Jak mierzyliście?"

"O, w VM, na laptopie."

To było bezwartościowe. VM na laptopie ma jitter rzędu milisekund - w zależności od tego, co laptop robi w tle.

Wniosek: pomiary RT zawsze na dedykowanym sprzęcie, z odizolowanym systemem. Jeśli macie wątpliwości - użyjcie GPIO toggle i oscyloskopu/logic analyzer. To daje prawdziwy pomiar.

## Checklisty
- Masz baseline p99/p99.9 czasu iteracji pętli.
- Masz test obciążeniowy (telemetria/CPU/IO) i widzisz jego wpływ.
- Masz strategię degradacji przy missed deadlines.

### Checklist szczegółowy

**Baseline:**
- [ ] Pomiar p50, p95, p99, p99.9 czasu pętli
- [ ] Pomiar przy minimalnym obciążeniu systemu
- [ ] Porównanie z wymaganiami (budżet czasowy)

**Testy obciążeniowe:**
- [ ] Test CPU load (100% jednego core)
- [ ] Test I/O (dysk, sieć)
- [ ] Test telemetria (logowanie)
- [ ] Pomiar wpływu na p99/p99.9

**Strategia degradacji:**
- [ ] Definicja "missed deadline"
- [ ] Reakcja na missed deadline (logowanie, fallback)
- [ ] Watchdog (co robi, gdy pętla się zawiesi)

**Pomiary:**
- [ ] Pomiar na dedykowanym sprzęcie (nie VM)
- [ ] GPIO toggle + logic analyzer dla weryfikacji
- [ ] Histogram/percentyle zapisane jako baseline

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
