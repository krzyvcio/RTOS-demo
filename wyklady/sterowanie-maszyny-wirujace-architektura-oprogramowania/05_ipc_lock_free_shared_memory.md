# Wyklad 5: IPC miedzy warstwa 2 i 3 (shared memory + lock-free ring buffer)

## Cel
Zrobic komunikacje RT->nonRT tak, aby:
- RT nie blokowalo sie na niczym,
- nonRT moglo czytac dane z opoznieniem,
- logowanie i HMI nie wplywalo na deterministyke.

> TL;DR: Mutex w sciezce RT to proszenie sie o priority inversion. Ring buffer (SPSC) jest zwykle wystarczajacy.

## Wzorzec: SPSC ring buffer
SPSC = single-producer single-consumer.
- producer: watek RT
- consumer: logger/HMI feeder

Minimalna struktura (koncept):
```text
struct Sample { t_mono_ns, omega_set, omega_meas, u_cmd, rt_loop_us, flags }
ring[ N ]
write_idx (atomic)
read_idx  (atomic)
```

Zasady:
- producer tylko zapisuje i inkrementuje write_idx
- consumer tylko odczytuje i inkrementuje read_idx
- w razie przepełnienia: overwrite najstarsze albo drop (jawna polityka)

## Polityka przepełnienia
W praktyce musisz zdecydowac:
- drop: gubisz probki, ale RT nie cierpi
- overwrite: tracisz historie, ale masz "najnowsze"

W systemach sterowania zwykle preferujesz:
- overwrite dla telemetrii "live"
- drop + licznik dropow dla logow "do analizy"

## Checklisty
- Watek RT nie czeka na consumer.
- Masz licznik drop/overwrite i go logujesz.
- Format probki jest staly i wersjonowany.

## Kontekst: dane sensoryczne i "kaskady buforow"
W robotyce dochodza strumienie danych (IMU, czujniki sily, czasem wizja).
W praktyce robi sie:
- osobny ring buffer per strumien (z roznym N),
- priorytety odczytu (sterowanie bierze tylko to, co musi),
- agregacja w non-RT (logi, dashboard, analiza).

Zasada: RT konsumuje tylko "esencje", reszta jest opcjonalna.

## Pytania do studentow
1. Kiedy SPSC ring buffer wystarcza, a kiedy potrzebujesz MPSC (i dlaczego to trudniejsze)?
2. Co wybierasz: drop czy overwrite? Jak to uzasadniasz dla telemetrii live i logow do analizy?
3. Jak zapewnisz, ze consumer non-RT nie wywola backpressure na RT?
4. Jak mierzysz i raportujesz gubienie probek (drop counters)?

## Projekty studenckie
- "Ring buffer lib": implementacja SPSC ring buffer + testy obciazeniowe + liczniki dropow.
- "Shared memory demo": RT producer + nonRT consumer + eksport CSV.
- "Backpressure study": eksperyment pokazujacy wplyw wolnego consumer'a na rozne polityki przepełnienia.

## BONUS
- Dla wielu zespolow przełomem jest moment, gdy przestaja uzywac mutexow do telemetrii. Lock-free to nie "fancy", to najprostsza droga do przewidywalnosci.
