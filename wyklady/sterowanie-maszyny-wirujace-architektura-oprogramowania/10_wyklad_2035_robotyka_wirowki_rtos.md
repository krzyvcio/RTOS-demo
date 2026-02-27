# Wyklad 10 (2035): Robotyka + wirowki + RTOS — zintegrowany obraz systemow autonomicznych

## Cel
Przedstawic obraz nowoczesnego (rok 2035) systemu autonomicznego jako jednej calosci:
- architektura robotow (percepcja -> planowanie -> kontrola),
- sensoryka i fuzja danych,
- sterowanie w czasie rzeczywistym (harmonogramowanie, deterministyka),
- wirowki laboratoryjne i przemyslowe jako przyklad maszyn wirujacych z ekstremalnymi wymaganiami stabilnosci,
- integracja robota z modulami maszyn (autonomiczne laboratoria, linie produkcyjne),
- trendy 2035 + etyka, bezpieczenstwo i odpowiedzialnosc inzynierska.

> TL;DR: W 2035 nie wygrywa "najmocniejszy algorytm", tylko system, ktory jest deterministyczny, bezpieczny, diagnozowalny i odporny na bledy.

## 1) Architektura robota: od sensorow do aktuatorow
Typowa architektura autonomicznego robota (mobilnego lub manipulatora) to pipeline:
```text
sensory -> timestamping -> percepcja -> fuzja stanu -> plan -> kontrola -> napedy -> swiat
```

### Sensoryka (co jest typowe w 2035)
- IMU + enkodery (szybkie, lokalne),
- wizja (kamery) i/lub lidar (globalne wskazowki, wykrywanie obiektow),
- czujniki sily/momentu (interakcja i kontrola kontaktu),
- czujniki bezpieczenstwa (strefy, skanery, kurtyny, e-stop).

Najwazniejsze dla software:
- kazdy sensor ma inny czas, opoznienie i zawodnosc,
- dlatego timestamping i synchronizacja czasu sa "pierwsza klasa" problemu.

## 2) Real-time w robotyce: dlaczego to nie jest tylko "szybko"
Robot ma wiele petli o roznych wymaganiach czasowych:
- naped (prad/moment): kHz i wiecej,
- przegub (predkosc/pozycja): setki Hz,
- kontrola w przestrzeni (trajektorie, kontakt): dziesiatki-100 Hz,
- percepcja i planowanie: od kilku Hz do kilkudziesieciu Hz (zalezy od zadania).

RTOS i Linux PREEMPT_RT sa potrzebne, bo:
- jitter niszczy stabilnosc i jakosc,
- miss deadline w zlym miejscu to problem bezpieczenstwa, nie "gorszy wykres".

### Harmonogramowanie i izolacja
W 2035 standardem jest "budgeted RT":
- watki RT maja zdefiniowane deadliny,
- non-RT ma byc izolowane (osobne rdzenie/procesy),
- telemetria i ML inference nie moga zabierac budzetu petli sterowania.

## 3) Wirowki jako maszyny wirujace: co jest specjalne (software-first)
Wirowki laboratoryjne i przemyslowe (neutralnie: jako klasa maszyn wirujacych) wyrozniaja sie:
- wysoka energia kinetyczna,
- bardzo wysoka czulosc na drgania i rezonanse,
- silna zaleznosc zachowania od stanu mechaniki (lozyska, niewywazenie, mocowanie),
- wymaganie bezpiecznego zatrzymania (safe stop) i nadzoru.

### Co sterujesz w praktyce
- profil predkosci (rampy, jerk),
- ograniczenia momentu/pradu,
- tlumienie rezonansow (filtry, ograniczenie pasma, polityka przejsc przez zakresy),
- wykrywanie anomalii (widmo, trendy).

### Predykcyjna diagnostyka (to sie spina z robotyka)
W 2035 "condition monitoring" jest integralne:
- system zbiera metryki (wibracje, temperatura, saturacje, jitter),
- buduje baseline dla trybow pracy,
- wykrywa odchylenia i eskaluje: WARNING -> DEGRADED -> SAFE_STOP.

Klucz: diagnostyka musi byc tak zaprojektowana, by nie psula RT (asynchroniczna, rate-limited).

## 4) MCU + RTOS w urzadzeniach wirujacych (i dlaczego to sie nie zmieni)
W urzadzeniach z twardymi wymaganiami:
- szybkie petle (np. naped) siedza blisko sprzetu,
- RTOS daje przewidywalnosc i prostote reakcji awaryjnych,
- watchdog i limity sprzetowe sa niezalezne od "smart" warstwy wyzej.

To jest wspolne dla:
- robotyki (serwonapedy),
- maszyn wirujacych (napedy, stabilizacja),
- automatyki krytycznej (safety).

## 5) Integracja: robot + modul wirowki (autonomiczne laboratoria, produkcja)
W 2035 typowy wzorzec to "komorka autonomiczna":
- robot (manipulator) wykonuje transfer probek/elementow,
- modul procesu (np. wirowka, mieszalnik, analizator) wykonuje operacje,
- system nadrzedny planuje zadania, kontroluje bezpieczenstwo i audyt.

### Interfejsy (praktyczne, software)
Wazniejsze od "jakiego protokolu" jest:
- jawny kontrakt danych (wersjonowany),
- stan maszyny jako FSM (READY/RUNNING/FAULT/SAFE_STOP),
- zdarzenia i alarmy jako strumien (nie polling w petli RT).

### Odporność na bledy (co musi umiec system)
Przyklady bledow:
- robot upuszcza probke,
- modul procesu zglasza FAULT,
- komunikacja ma dropout,
- petla RT ma missed deadline.

System musi miec:
- deterministyczne zachowanie (nie "dziwnie sie zachowal"),
- logi i audit trail,
- mechanizmy recovery, ktore nie naruszaja bezpieczenstwa.

## 6) Trendy 2035 (bez marketingu, inzyniersko)
### Roboty kolaboracyjne
- wiecej pracy obok czlowieka,
- wiecej czujnikow bezpieczenstwa i ograniczen energii,
- wieksza rola formalizacji zachowan bezpiecznych.

### Autonomiczne laboratoria
- automatyzacja przeplywu pracy (workflow),
- integracja wielu modulow procesu,
- nacisk na powtarzalnosc, audyt i zgodnosc (compliance).

### Inteligentna diagnostyka
- detekcja anomalii i predykcja awarii (modele + dane),
- ale z twarda zasada: ML nie moze byc w krytycznej sciezce RT bez barier bezpieczenstwa.

## 7) Etyka, bezpieczenstwo i odpowiedzialnosc inzynierska
W systemach krytycznych odpowiedzialnosc jest praktyczna:
- projektujesz tak, by awarie byly przewidywalne i bezpieczne,
- unikasz "czarnych skrzynek" w kanale bezpieczenstwa,
- dbasz o prywatnosc i integralnosc danych (logi, audit),
- masz procedury: testy awaryjne, fault injection, regresja.

Minimalne zasady:
- safety jest oddzielnym kanalem decyzyjnym,
- kazdy tryb awarii ma zdefiniowana reakcje i warunki powrotu,
- metryki deterministyki i stabilnosci sa mierzone stale, nie tylko w labie.

## Checklisty
- Czy pipeline sens->control ma spojnosc czasu (timestamping, synchronizacja)?
- Czy watki RT sa izolowane od telemetrii/ML/HMI?
- Czy modul wirowki ma watchdog i safe behavior bez systemu nadrzednego?
- Czy integracja robot+modul ma FSM i deterministyczne zachowanie na bledy?
- Czy masz audytowalne logi i testy fault injection jako regresje?

## Slajdy (tekstowe)
### Slajd 1: 2035 w jednym zdaniu
- Systemy autonomiczne wygrywaja deterministyka + safety + diagnozowalnosc

### Slajd 2: Robot: sensory -> plan -> kontrola
- timestamping jest krytyczny

### Slajd 3: Wirowka: stabilnosc i diagnostyka
- drgania, rezonanse, safe stop

### Slajd 4: Integracja w autolabie
- FSM, zdarzenia, audyt, recovery

### Slajd 5: Etyka i odpowiedzialnosc
- safety jako osobny kanal
- ML poza krytyczna sciezka RT

## Pytania do studentow
1. Ktore elementy "autonomii" powinny byc ograniczone przez twarde bariery safety i dlaczego?
2. Jak zaprojektujesz integracje robota z modulem procesu, by awarie byly przewidywalne (FSM, events, audit)?
3. Jakie metryki deterministyki i diagnostyki uznasz za obowiazkowe w komorce autonomicznej?
4. Jakie decyzje (np. ML) moga byc pomocnicze, ale nie moga byc krytyczne dla safety?

## Projekty studenckie
- "Autonomous cell": projekt architektury komorki (robot + modul procesu) z kontraktami i FSM.
- "Predictive monitoring": pipeline trendow (wibracje/termika/jitter) z eskalacja i raportem.
- "Audit trail": format logow i zdarzen z wersjonowaniem, zeby dało sie odtworzyc decyzje systemu.

## BONUS
- W 2035 przewaga inzynierska to nie "wiecej AI", tylko umiejetnosc projektowania granic: co jest deterministyczne, co jest probabilistyczne, i jak system zachowuje sie bezpiecznie, gdy probabilistyczne zawiedzie.
