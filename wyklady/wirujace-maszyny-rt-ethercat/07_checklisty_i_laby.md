# Wykład 7: Checklisty wdrożeniowe i mini-laby (praktyka)

## Cel
Dać gotowe ćwiczenia i checklisty do zastosowania w projekcie lub na zajęciach.

> TL;DR: Każdy lab ma mieć: (1) dane, (2) metrykę, (3) kryterium pass/fail, (4) wniosek projektowy.

## Artefakty: minimalne formaty logów
Żeby laby miały sens, potrzebujesz powtarzalnego formatu danych.
Minimalne pola (jedna próbka):
- `t_wall_ms`: czas ścienny (dla człowieka/logów),
- `t_mono_ns`: monotoniczny czas (do opóźnień),
- `mode`: tryb pracy (NORMAL/START/STOP/DEGRADED),
- `omega_set`, `omega_meas`, `omega_err`,
- `torque_cmd` (lub odpowiednik sterowania),
- `sat_flags` (czy były saturacje),
- `temp_drive`, `temp_bearing` (jeśli dostępne),
- `vib_rms` (jeśli dostępne),
- `rt_loop_us` (czas iteracji pętli),
- `miss_deadline` (0/1),
- `comm_drop` (0/1).

Przykładowa linia (CSV, uproszczona):
```text
t_wall_ms,t_mono_ns,mode,omega_set,omega_meas,torque_cmd,rt_loop_us,miss_deadline
12345,9876543210,NORMAL,1000,998,0.12,450,0
```

## Mini-lab 1: Pomiar jitteru pętli
Cel:
- zmierzyć histogram czasu iteracji pętli sterowania,
- wykryć ogon opóźnień.

Wynik:
- wykres/CSV z `czas_iteracji_ms`,
- procent missed deadline.

Kroki:
1. Dodaj timestamp na wejściu i wyjściu pętli (monotoniczny).
2. Zapisuj `rt_loop_us = t_end - t_start` do bufora (bez IO w pętli RT).
3. Raz na N iteracji lub w wątku nie-RT zrzucaj bufor do pliku.
4. Policz histogram i percentyle (p95/p99/p99.9).

Kryteria pass/fail (przykład logiczny):
- pass: brak missed deadlines w typowym obciążeniu i stabilne p99,
- fail: seria missed deadlines lub „długi ogon” korelujący z telemetrią/IO.

Wniosek projektowy:
- jeśli ogon rośnie przy telemetrii, izolujesz wątki, ograniczasz logowanie, porządkujesz priorytety.

## Mini-lab 2: FFT błędu prędkości
Cel:
- zebrać sygnał błędu prędkości,
- policzyć FFT i znaleźć piki.

Wynik:
- lista częstotliwości dominujących,
- hipotezy: rezonans mechaniczny vs zakłócenie.

Kroki:
1. Zbierz okno danych `omega_err` (i najlepiej też `torque_cmd` oraz `vib_rms`).
2. Policz FFT, zidentyfikuj 3–5 największych pików.
3. Sprawdź, czy piki są stałe w trybie i czy przesuwają się z prędkością/obciążeniem.
4. Zaproponuj reakcję: notch / ograniczenie pasma / zmiana ramp/jerk / inspekcja mechaniki.

Kryteria pass/fail:
- pass: po zmianie (np. notch lub rampy) piki spadają bez wzrostu jitteru i bez nowych oscylacji,
- fail: piki spadają, ale rośnie błąd regulacji albo pojawiają się nowe oscylacje (filtr psuje stabilność).

## Mini-lab 3: Test „dropout komunikacji”
Cel:
- zasymulować przerwę danych (np. brak aktualizacji),
- sprawdzić zachowanie watchdogów i safe stop.

Wynik:
- czas reakcji,
- poprawność przejścia do stanu bezpiecznego.

Kroki:
1. Zasymuluj dropout (np. zatrzymaj wysyłkę komend lub wstrzymaj odbiór).
2. Zmierz czas do wykrycia (watchdog) po stronie slave i master.
3. Sprawdź, w jaki stan przechodzi system (DEGRADED/SAFE_STOP).
4. Sprawdź warunki powrotu: brak flappingu, jawny recovery.

Kryteria pass/fail:
- pass: deterministiczna reakcja w zdefiniowanym czasie i kontrolowany recovery,
- fail: utrzymanie ostatniej komendy, brak reakcji lub „flapping” stanów.

## Mini-lab 4: Test „telemetria kontra RT”
Cel:
- udowodnić, że telemetria i logowanie nie psują deterministyki.

Kroki:
1. Uruchom system w baseline (minimalna telemetria), zbierz p99/p99.9 `rt_loop_us`.
2. Włącz telemetrię (zwiększ częstotliwość / ilość danych), zbierz ponownie.
3. Porównaj ogon opóźnień i liczbę missed deadlines.
4. Wprowadź poprawkę: rate limiting, osobny wątek/proces, backpressure.

Wniosek projektowy:
- telemetria ma być „najpierw nieszkodliwa”, dopiero potem „bogata”.

## Checklisty (wdrożeniowe)
- Zmierz i zapisz budżet czasu end-to-end.
- Zmierz jitter i WCRT w warunkach worst-case.
- Wydziel wątki RT i odseparuj logowanie/telemetrię.
- Zidentyfikuj rezonanse (FFT/sweep) i zaplanuj tłumienie.
- Przetestuj awarie: brak czujnika, przegrzanie, dropout komunikacji, missed deadline.

## Checklisty (review architektury)
- Czy pętla krytyczna ma zdefiniowany okres, deadline i reakcję na missed deadline?
- Czy dane cykliczne są minimalne, a reszta jest poza cyklem i rate-limited?
- Czy drive ma watchdog i safe behavior bez mastera?
- Czy logowanie jest asynchroniczne i nie blokuje RT?
- Czy masz baseline metryk (przed/po zmianie) i porównujesz ogony rozkładów?

## Pytania do studentów
1. Jak zdefiniujesz kryterium „pass/fail” dla jitteru, jeśli nie znasz jeszcze docelowej mechaniki?
2. Która polityka bufora telemetrii jest lepsza: drop czy overwrite? Kiedy i dlaczego?
3. Jak pokażesz, że telemetria „nie szkodzi” RT, a nie tylko „nie widzisz problemu”?
4. Jakie metryki uznasz za obowiązkowe w raporcie z labów?

## Projekty studenckie
- „Lab harness”: zestaw skryptów uruchamiających laby i generujących raport (CSV + wykresy + wnioski).
- „Regression kit”: scenariusze regresji (dropout, opóźnienie, saturacja, telemetria) uruchamiane automatycznie.
- „Metrics spec”: specyfikacja formatu logu + walidator, który sprawdza kompletność pól i wersję schematu.

## BONUS
- Traktuj laby jak testy regresji: jeśli po zmianie systemu nie potrafisz łatwo porównać wyników „przed/po”, to lab nie jest jeszcze narzędziem inżynierskim.
