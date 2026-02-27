# Wykład 6: Sieci czasu rzeczywistego, synchronizacja

## Dlaczego to jest krytyczne
W humanoidzie czas jest "ukrytą zmienną stanu".
Jeśli opóźnienia i jitter rosną, sterowanie zaczyna widzieć przeszłość, a estymacja i planowanie rozjeżdżają się w fazie.
To prowadzi do:
- oscylacji regulatorów (mniejszy margines fazy),
- pogorszenia estymacji (złe dopasowanie pomiarów do stanu),
- błędnych decyzji o kontakcie (stopa "już" w kontakcie, ale system jeszcze tego nie wie),
- trudnych do debugowania sporadycznych awarii.

## Grafy i struktury: jak patrzeć na system
### Graf sieci (nodes + edges)
Węzły: sensory, kontrolery, napędy, komputery.
Krawędzie: łącza komunikacyjne.
Taki graf jest podstawą do:
- analizy topologii i punktów krytycznych,
- identyfikacji wąskich gardeł przepustowości i opóźnień,
- planowania redundancji.

### Graf przepływu danych (dataflow graph)
To graf zależności obliczeniowych:
- węzły = zadania (estymacja, kontrola, planowanie),
- krawędzie = dane (IMU -> estymacja -> kontrola).
Na nim liczysz opóźnienie end-to-end i szukasz miejsc, gdzie można zrobić pipelining.

### Graf zależności czasowych
To ujęcie, gdzie krawędzie mają ograniczenia typu:
- "zadanie B musi wystartować <= X ms po zadaniu A",
- "aktualizacja napędu musi nastąpić co T ms z jitterem <= J".

### Graf harmonogramu (time-triggered schedule)
W systemach deterministycznych planujesz czas:
- okna transmisji,
- okna obliczeń,
- okresy pętli.
To jest szczególnie ważne, gdy komunikacja jest współdzielona i nie możesz polegać na "best effort".

## Podstawowe pojęcia czasu rzeczywistego
### WCET i WCRT
WCET (Worst-Case Execution Time):
- maksymalny czas wykonania zadania w najgorszym przypadku (po cache misses, przerwaniach).

WCRT (Worst-Case Response Time):
- czas od "gotowości" zadania do zakończenia (uwzględnia oczekiwanie na CPU i preempcje).

Praktyka:
- dla sterowania liczy się WCRT, nie średnia,
- deterministyczność jest ważniejsza niż "wysoka średnia wydajność".

### Jitter
Jitter to zmienność okresu lub opóźnienia.
Dwa kluczowe rodzaje:
- jitter próbkowania (czas między kolejnymi iteracjami pętli),
- jitter opóźnienia komunikacji (czas dostarczenia wiadomości).

## Harmonogramowanie: RM, DM, EDF
Modele klasyczne zakładają zadania okresowe z czasem wykonania `C_i` i okresem `T_i` oraz deadlinem `D_i`.

Rate Monotonic (RM):
- priorytety stałe: krótszy okres -> wyższy priorytet,
- prosty i przewidywalny.

Deadline Monotonic (DM):
- priorytety stałe: krótszy deadline -> wyższy priorytet.

Earliest Deadline First (EDF):
- priorytety dynamiczne: najbliższy deadline wygrywa,
- teoretycznie efektywniejszy, ale wymaga ostrożności w implementacji i analizie.

W praktyce w systemach sterowania:
- wybór polityki jest mniej ważny niż: izolacja krytycznych wątków, pinning rdzeni, ograniczanie zakłóceń,
- nie możesz dopuścić, by telemetria i logowanie zabierały budżet pętli sterowania.

## Teoria kolejek: intuicja dla opóźnień
Gdy masz kolejki (np. bufor wiadomości, ring buffer), pojawia się:
- opóźnienie oczekiwania zależne od obciążenia,
- ryzyko narastania opóźnienia (backpressure).

W inżynierskim skrócie:
- jeśli średnie obciążenie jest blisko 100%, to ogony rozkładu opóźnień eksplodują,
- więc budżetuj margines (headroom).

## Synchronizacja czasu: offset i dryft
Każdy węzeł ma swój zegar:
```text
t_local = a * t_true + b
```
gdzie:
- `b` to offset,
- `a` (blisko 1) to tempo zegara (dryft).

Synchronizacja polega na estymacji `a` i `b` (często wprost lub pośrednio).
W praktyce:
- offset korygujesz częściej,
- dryft śledzisz w dłuższym oknie i korygujesz wolniej.

Jeśli nie masz synchronizacji:
- timestamp z IMU i timestamp z kamery nie są porównywalne,
- a fuzja sensorów staje się losowa.

## Algebra max-plus i grafy czasowe
W systemach deterministycznych opóźnienia end-to-end można analizować przez:
- grafy zdarzeń (event graphs),
- relacje typu "czas wyjścia = max(czas wejść + opóźnienia)".

To jest intuicyjnie max-plus:
- sumowanie opóźnień to dodawanie,
- synchronizacja ścieżek to max.

Nie musisz używać formalizmu na co dzień, ale warto rozumieć:
- krytyczna ścieżka determinuje opóźnienie,
- równoległość pomaga tylko wtedy, gdy ścieżki nie synchronizują się wąskim gardłem.

## Praktyka inżynierska: co mierzyć i jak projektować
- Budżet opóźnień end-to-end: sensor -> estymacja -> kontrola -> napęd.
- Pomiary jitteru i WCRT w runtime (statystyki i alarmy).
- Separacja klas ruchu: krytyczny sterujący vs. diagnostyka vs. telemetria.
- Strategie degradacji: co robisz, gdy pętla nie wyrabia (np. redukcja częstotliwości planowania, priorytet stabilizacji).

## Checklisty
- każda pętla ma zdefiniowany okres, deadline i dopuszczalny jitter,
- logowanie jest asynchroniczne i ograniczone (rate limiting),
- testy obciążeniowe (worst-case) są częścią walidacji, nie dodatkiem.

## Pytania do studentów
1. Jak zmierzysz opóźnienie end-to-end od sensora do aktuatora, a nie tylko „czas wątku”?
2. Kiedy EDF ma sens w praktyce, a kiedy prostszy RM/DM wygrywa przewidywalnością?
3. Jak jitter wpływa na stabilność sterowania i które miejsca w systemie są najgroźniejsze?
4. Jak zaprojektujesz rozdział ruchu krytycznego od telemetrii w architekturze sieci?

## Projekty studenckie
- Monitor jitteru i WCRT w runtime + alarmy trendowe.
- Symulator harmonogramowania (RM/EDF) dla zestawu zadań + wnioski o wykonalności.
- „Budget tool”: narzędzie do budżetowania opóźnień w grafie dataflow (krytyczna ścieżka).

## BONUS
- Najczęściej „tajnym” wąskim gardłem jest kolejka lub lock w miejscu, którego nikt nie uważa za krytyczne; dlatego mierz WCRT i histogramy, a nie tylko średnie czasy.
