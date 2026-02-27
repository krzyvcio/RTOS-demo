# Wykład 2: Czas rzeczywisty, jitter i deterministyczność (to, co realnie psuje sterowanie)

## Cel
Zrozumieć, jak jitter i opóźnienia wpływają na stabilność oraz jak mierzyć i budżetować czas w systemie sterowania.

> TL;DR: „Szybko” nie znaczy „deterministycznie”. Sterowanie przegrywa przez **ogony opóźnień**, nie przez średnią.

## Pojęcia, które musisz mieć „w palcach”
- `T` (okres pętli): co ile ms liczysz sterowanie.
- opóźnienie (latency): ile czasu mija od pomiaru do aktuacji.
- jitter: zmienność `T` lub opóźnienia.
- WCET/WCRT: najgorszy czas wykonania / odpowiedzi.

## Mentalny model: opóźnienie jako „dodatkowa faza”
W pętli sterowania opóźnienie zachowuje się jak dodatkowe przesunięcie fazowe.
Praktycznie:
- im większe opóźnienie i jitter, tym mniejszy margines stabilności,
- nawet jeśli regulator „na papierze” jest poprawny.

## Rodzaje opóźnień (które warto rozdzielić)
- opóźnienie pomiaru (sensor, filtr antyaliasing),
- opóźnienie transportu (sieć, kolejki, DMA),
- opóźnienie obliczeń (estymacja + regulator),
- opóźnienie aktuacji (drive, PWM, próbkowanie).

W debugowaniu pomaga rozdzielenie:
- „latency stałe” (można kompensować),
- „latency losowe” (jitter, zabija przewidywalność).

## Dlaczego jitter jest groźny
W sterowaniu dyskretnym zakładasz stały okres próbkowania.
Jeśli okres pływa:
- efektywne wzmocnienia i faza „pływają”,
- margines stabilności maleje,
- pojawiają się oscylacje sporadyczne (najtrudniejsze do debugowania).

Praktyczna obserwacja:
- średnia wydajność nie ratuje sterowania, jeśli ogony rozkładu opóźnień są duże.

## Jak budżetować czas (prosty szablon)
Zrób tabelę:
- pobranie pomiarów: `t1`,
- estymacja/filtracja: `t2`,
- regulator: `t3`,
- wysyłka na sieć/napęd: `t4`,
- margines: `m`.

Warunek:
```text
t1 + t2 + t3 + t4 + m <= T
```

## Co mierzyć: minimum diagnostyczne RT
Nie „zgaduj” czasu, mierz go.
Minimum, które daje sensowne wnioski:
- czas iteracji pętli: `t_end - t_start`,
- rozkład jitteru (histogram, percentyle p95/p99/p99.9),
- liczba missed deadlines i ich kontekst,
- opóźnienie end-to-end (z timestampów),
- błędy/droputy komunikacji.

Jeżeli możesz mierzyć tylko jedną rzecz: mierz p99/p99.9 czasu iteracji, nie średnią.

## Linux PREEMPT_RT: praktyczne zasady
Jeśli sterujesz na Linux RT:
- ustaw priorytety wątków RT,
- izoluj rdzenie (core isolation) dla pętli,
- ogranicz przerwania na rdzeniu RT,
- ogranicz alokacje i GC (w językach zarządzanych),
- logowanie przenieś do wątków nie-RT (asynchronicznie, rate-limited).

### Dodatkowe praktyki (najczęściej potrzebne)
- zablokuj pamięć procesu RT (`mlockall`), żeby uniknąć page faults,
- unikaj dynamicznych alokacji w pętli (new/malloc),
- unikaj IO blokującego (dysk/sieć) w wątku RT,
- przypnij wątek do rdzenia (`taskset`/CPU affinity),
- uważaj na `irqbalance` i „wędrujące” przerwania.

### Narzędzia pomiarowe (Linux)
Przykładowe klasy narzędzi:
- latencja scheduler’a i timerów (testy typu cyclictest/timerlat),
- śledzenie opóźnień w kernelu (ftrace),
- profilowanie obciążenia (perf).

Nie zakładamy, że są dostępne wszędzie; sens jest taki:
- przed strojem zrób baseline,
- po zmianie sprawdź, czy ogon opóźnień się poprawił.

## EtherCAT a deterministyczność
EtherCAT zwykle jest cykliczny:
- masz „tick” komunikacji,
- dane wchodzą/wychodzą w przewidywalnym oknie.

Jeśli cykl komunikacji i cykl sterowania nie są spójne:
- pojawiają się „beat frequencies” (aliasing czasowy),
- regulator widzi dane z niejednoznacznym opóźnieniem.

Praktyczna zasada:
- jeden „master clock” dla cyklu sterowania i cyklu komunikacji jest mniej ryzykowny niż dwa niezależne „zegary” o podobnych częstotliwościach.

## Pomiary, które trzeba robić (nie zgadywać)
- histogram czasu iteracji pętli (czas start->koniec),
- histogram opóźnień end-to-end,
- liczba missed deadlines,
- jitter zegara i synchronizacji,
- dropouty w komunikacji.

## Typowe przyczyny jitteru (krótka lista do debugowania)
- przerwania i softirq (szczególnie od sieci i dysku),
- walka o CPU (złe priorytety, brak izolacji),
- blokujące wywołania w pętli (IO, lock contention),
- alokacje pamięci i GC,
- przepełnione kolejki (backpressure),
- zbyt agresywny cykl komunikacji (CPU „dusi się” obsługą).

## Strategia degradacji (co robić, gdy RT nie wyrabia)
To musi być zdefiniowane, bo „nic nie robić” zwykle jest najgorszą opcją.
Typowy schemat:
- jeśli missed deadline pojedynczy: log + zwiększ diagnostykę,
- jeśli seria missed deadline: ogranicz osiągi (np. zmniejsz rampy/jerk),
- jeśli trwa dalej: przejście do safe stop (zależnie od ryzyka).

## Checklisty
- Masz twardy limit czasu na iterację pętli i reakcję awaryjną.
- Logowanie nie wpływa na RT (asynchroniczne).
- Sieć i I/O są testowane w warunkach worst-case (obciążenie, zakłócenia).
- W razie missed deadline masz zdefiniowany tryb degradacji.

## Zadania (praktyka)
1. Zdefiniuj budżet czasu dla pętli sterowania (tabela `t1..t4 + m`).
2. Zaproponuj zestaw metryk runtime (min. 6) i gdzie je zbierasz (drive/master).
3. Opisz 3 scenariusze worst-case (telemetria, obciążenie CPU, dropouty) i jak je zasymulujesz/testujesz.

## Pytania do studentów
1. Dlaczego średnia latencja jest niewystarczająca do oceny deterministyki? Co pokażą percentyle p99/p99.9?
2. Jaka jest różnica między WCET a WCRT i która miara jest krytyczna dla stabilności sterowania?
3. Jaką strategię degradacji zastosujesz przy pojedynczym missed deadline, a jaką przy serii missed deadlines?
4. Jak zaprojektujesz logowanie tak, aby nie wpływało na wątek RT?

## Projekty studenckie
- „Jitter recorder”: lekki rejestrator `rt_loop_us` z eksportem CSV i obliczaniem percentyli.
- „Worst-case harness”: skrypt obciążeniowy (CPU/IO/telemetria), który uruchamia testy i porównuje ogony rozkładów.
- „RT dashboard”: dashboard pokazujący p99/p99.9, missed deadlines, dropouty komunikacji i ich korelacje.

## BONUS
- W projektach studenckich najczęściej wygrywa ktoś, kto od pierwszego dnia mierzy czas monotoniczny i loguje percentyle, zamiast debugować „wrażeniami”.
