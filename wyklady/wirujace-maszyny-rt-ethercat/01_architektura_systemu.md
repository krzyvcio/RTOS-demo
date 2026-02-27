# Wykład 1: Architektura sterowania maszyną wirującą (praktycznie)

## Cel
Zbudować mentalny model systemu:
- co jest sterowane i jakimi pętlami,
- gdzie powstają opóźnienia i jitter,
- gdzie pojawia się ryzyko niestabilności (mechanika, filtracja, czas),
- jaką rolę mają EtherCAT/RTOS/Linux PREEMPT_RT w realnym wdrożeniu.

> TL;DR: Najpierw rysujesz **krytyczną ścieżkę end-to-end**, potem decydujesz co musi być deterministyczne, a dopiero na końcu wybierasz RTOS/Linux RT i sieć.

## Co to jest „maszyna wirująca” w kontekście sterowania
W praktyce masz:
- wirnik (bezwładność `J`),
- napęd (silnik + falownik),
- łożyskowanie i konstrukcję (rezonanse, drgania),
- czujniki (prędkość, pozycja, wibracje, temperatury),
- system sterowania (pętle regulacji),
- system nadzoru i bezpieczeństwa (limity, watchdog, safe stop).

Najprostszy model prędkości (intuicja):
```text
J * domega/dt = T_motor - T_load - T_losses
```
To nie wystarczy do realnego świata, ale wystarczy do zrozumienia:
- czemu opóźnienia i jitter psują stabilność,
- czemu potrzebujesz kaskadowych pętli (prąd -> prędkość -> proces).

## Najważniejszy rysunek: system jako „łańcuch sygnałowy”
W realnym systemie debugujesz najczęściej nie „model”, tylko opóźnienia i zależności między blokami:
```text
(sensor) -> (timestamp/buffer) -> (filter/estimator) -> (controller) -> (transport) -> (drive) -> (plant)
```
Jeśli gdziekolwiek ten łańcuch ma:
- losowe opóźnienie,
- sporadyczne dropouty,
- brak spójnego czasu,
to w sterowaniu zobaczysz to jako „dziwne” oscylacje, spadek jakości i trudną do uchwycenia niestabilność.

## Warstwy systemu (typowa praktyka)
### Warstwa napędowa (kHz)
- sterowanie prądem/momentem (FOC),
- szybkie zabezpieczenia prądowe,
- pomiar prądów, napięć, temperatur.

### Warstwa regulacji ruchu (100 Hz–kHz, zależnie od obiektu)
- regulator prędkości,
- ograniczanie przyspieszeń i jerk,
- tłumienie rezonansów (notch, ograniczenie pasma).

### Warstwa procesu (Hz–100 Hz)
- receptury, sekwencje,
- kontrola parametrów procesu (zależnie od zastosowania),
- logika start/stop, rampy, tryby.

### Warstwa diagnostyki i bezpieczeństwa (ciągle w tle)
- monitoring trendów (wibracje, temperatura, prąd),
- logika degradacji,
- bezpieczne zatrzymanie.

## Granice odpowiedzialności: drive vs master (to decyzja architektoniczna)
Najczęstsze źródło problemów to zły podział pętli.
Praktyczna heurystyka:
- pętla prądu/momentu: **w napędzie** (najbliżej sprzętu, najwyższe pasmo),
- szybkie zabezpieczenia prądowe/temperaturowe: **w napędzie**,
- pętla prędkości: zależnie od wymagań (często w napędzie, czasem w masterze),
- koordynacja, profile ruchu, synchronizacja wielu modułów: **w masterze**.

Jeśli przenosisz pętlę prędkości „na górę” (do mastera), musisz umieć odpowiedzieć:
- jak duże jest opóźnienie i jitter całego toru pomiar->sterowanie->napęd,
- czy cykl komunikacji jest spójny z cyklem sterowania,
- co się dzieje przy dropoutach (watchdog, degradacja).

## Gdzie tu jest EtherCAT
EtherCAT jest najczęściej elementem „kręgosłupa” dla:
- napędów (drive’y),
- I/O (czujniki, moduły analogowe, enkodery),
- synchronizacji (Distributed Clocks),
- deterministycznej wymiany danych w cyklu sterowania.

Klucz: EtherCAT jest atrakcyjny tam, gdzie liczy się:
- stały cykl,
- niski jitter,
- synchronizacja wielu osi/modułów.

## RTOS vs Linux PREEMPT_RT: decyzje praktyczne
RTOS (klasy przemysłowej):
- przewidywalność (mniej „niespodzianek”),
- często łatwiejsza certyfikacja w systemach krytycznych,
- mniejszy komfort w rozwoju aplikacji „bogatej”.

Linux PREEMPT_RT:
- dobry kompromis „RT + ekosystem”,
- wymaga dyscypliny: izolacja rdzeni, priorytety, unikanie niekontrolowanych przerwań,
- integracja z aplikacjami wyższego poziomu jest łatwiejsza.

Reguła: jeśli pętla sterowania jest bardzo krytyczna i twarda czasowo, umieszczasz ją:
- jak najbliżej sprzętu (drive/MCU/RTOS),
- a Linux RT wykorzystujesz na poziomie koordynacji i obliczeń (z rygorem RT).

## „Twardość” wymagań: twarde RT, miękkie RT, brak RT
W praktyce warto rozdzielić:
- twarde RT: missed deadline = błąd bezpieczeństwa (np. aktualizacja napędu),
- miękkie RT: missed deadline = degradacja jakości (np. filtracja diagnostyczna),
- brak RT: funkcje biznesowe/telemetria (nie mogą zakłócić RT).

To pomaga decydować:
- które wątki dostają priorytet RT,
- co idzie osobnym procesem,
- gdzie dajesz watchdog i jaką reakcję.

## „Krytyczna ścieżka” end-to-end (to trzeba narysować)
Najważniejsze w praktyce jest policzenie opóźnienia:
```text
czujnik -> transport -> estymacja -> regulator -> transport -> napęd
```
Każdy element dokłada:
- opóźnienie,
- jitter,
- ryzyko dropoutów.

Zasada praktyczna:
- jeśli nie potrafisz podać (choćby szacunkowo) budżetu czasu end-to-end, to nie wiesz, czy architektura ma prawo działać stabilnie.

## „Tryby pracy” a architektura (start/stop, rampy, przejścia)
Maszyny wirujące mają przejścia stanów, które są trudniejsze niż praca ustalona:
- rozruch i rampy,
- przejścia przez zakresy rezonansów,
- zatrzymanie kontrolowane vs awaryjne.

W architekturze musisz mieć:
- jasno zdefiniowane tryby (FSM),
- warunki przejść (histereza, czas),
- mechanizmy ograniczeń (rampy, jerk, limity momentu/prądu),
- integrację z safety (kiedy tryb awaryjny przejmuje kontrolę).

## Co logować, żeby nie debugować „na ślepo”
Minimalny zestaw sygnałów:
- prędkość zadana i rzeczywista,
- błąd regulacji + sterowanie (np. moment),
- saturacje (prąd/moment/napięcie),
- znacznik czasu iteracji i wykryte missed deadlines,
- wskaźniki wibracji (choćby RMS) i temperatura.

## Pułapki wdrożeniowe
- „Wrzućmy wszystko na Linuxa”: bez izolacji i priorytetów to kończy się jitterem.
- „EtherCAT najszybciej jak się da”: zbyt mały cykl może zwiększyć jitter, bo CPU zaczyna gubić deterministykę.
- „Dodajmy filtr”: filtr zwiększa opóźnienie i może zjeść margines fazy, jeśli nie jest policzony.
- „Telemetria w tej samej pętli”: klasyczny sposób na sporadyczne oscylacje.

## Checklisty
- Zdefiniuj pętle i ich częstotliwości (prąd / prędkość / proces).
- Zmapuj krytyczną ścieżkę end-to-end i jej budżet czasu.
- Oddziel komunikację krytyczną od telemetrii (inny priorytet, inny kanał).
- Zidentyfikuj rezonanse i zaplanuj strategię tłumienia (filtry, ograniczenie pasma, mechanika).

## Zadania (praktyka)
1. Narysuj architekturę dla przykładowej maszyny (blokowo + ścieżka danych).
2. Zdefiniuj 3 pętle (prąd, prędkość, proces) i opisz, gdzie je umieszczasz (drive vs master).
3. Wypisz 5 sygnałów, które będziesz logować w runtime i uzasadnij dlaczego.

## Pytania do studentów
1. Jakie są 3 najważniejsze źródła opóźnień end-to-end w Twojej architekturze i jak je zmierzysz?
2. Które pętle umieścisz w napędzie, a które w masterze, i co jest Twoim kryterium decyzji?
3. Co w Twoim systemie jest hard RT, co firm RT, a co soft RT? Jakie są konsekwencje missed deadline w każdym z tych obszarów?
4. Jak odróżnisz problem rezonansu mechanicznego od problemu jitteru czasowego tylko na podstawie logów?

## Projekty studenckie
- „Mapa opóźnień”: narzędzie, które automatycznie wylicza budżet czasu end-to-end z timestampów i wskazuje wąskie gardła.
- „Architektura referencyjna”: repozytorium z szablonem warstw (RT pętla + logger + HMI) oraz przykładową telemetrią.
- „Tryby pracy”: FSM dla start/stop/degraded/safe stop z logowaniem przejść i przyczyn.

## BONUS
- Jeśli masz tylko jedną metrykę do ciągłego monitorowania, wybierz p99/p99.9 czasu iteracji pętli + licznik missed deadlines; to najszybciej demaskuje „niewidzialne” problemy architektury.
