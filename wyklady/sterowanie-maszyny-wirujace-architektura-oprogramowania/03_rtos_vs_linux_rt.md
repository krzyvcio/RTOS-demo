# Wyklad 3: RTOS vs Linux PREEMPT_RT (watki, priorytety, izolacja)

## Cel
Wybrac, gdzie ma byc "hard RT", a gdzie "firm RT", i jak to faktycznie zrobic w kodzie i systemie.

> TL;DR: Linux RT jest ok dla mastera, ale tylko gdy stosujesz dyscypline: SCHED_FIFO, pinning, mlockall, kontrola IRQ, brak IO w petli.

## RTOS - kiedy wygrywa
RTOS (MCU) wygrywa, gdy:
- potrzebujesz jitteru rzedu mikrosekund
- masz petle 10-40 kHz (np. FOC)
- chcesz prosty i weryfikowalny runtime

## Linux PREEMPT_RT - kiedy wygrywa
Linux RT wygrywa, gdy:
- chcesz ekosystem (sieć, narzedzia, debug, sterowniki)
- potrzebujesz RT "wystarczajaco dobrego" (firm RT)
- integrujesz EtherCAT master i diagnostyke

## "Wrzucmy wszystko na Linuxa" - klasyczna pulapka
Bez:
- izolacji rdzeni,
- priorytetow RT,
- kontroli przerwan,
robisz losowy system z jitterem.

## Watek RT - wzorzec implementacyjny
Jeden watek, jedna odpowiedzialnosc:
- czeka na tick
- czyta wejscia
- liczy regulacje
- zapisuje wyjscia
- publikuje telemetrie do lock-free bufora

Co NIE w tym watku:
- formatowanie JSON, HTTP, DB
- alokacje
- mutexy globalne

## Checklisty
- Masz pomiar `rt_loop_us` i histogram.
- Masz reakcje na missed deadline (degradacja/safe stop).
- Masz oddzielone watki: RT vs logowanie/HMI.

## Kontekst 2035: AI/ML obok RT (bezpieczne granice)
W 2035 inference (wizja, detekcja anomalii) bywa kosztowne obliczeniowo.
Zasada architektury:
- ML nie moze wchodzic do krytycznej petli RT bez barier (timeout, fallback, degradacja).

Praktycznie:
- inference dziala w osobnym procesie (non-RT),
- wynik trafia do planowania/nadzoru, nie do petli momentu/pradu.

## Pytania do studentow
1. Jakie sa minimalne wymagania, zeby uznac, ze petla na Linux PREEMPT_RT jest "firm-RT"?
2. Jak zmierzysz i udowodnisz, ze logowanie/telemetria nie wplywa na p99/p99.9 petli?
3. Jak zaprojektujesz fallback, gdy inference ML nie zdazy w czasie?
4. Jakie sa 3 najczestsze przyczyny jitteru w systemach Linux i jak je wykryjesz?

## Projekty studenckie
- "RT thread template": szablon watku SCHED_FIFO + pinning + mlockall + metryki czasu iteracji.
- "Telemetry isolation": pipeline RT->ring->logger z ograniczeniem (rate limit) i metrykami dropow.
- "Jitter hunt": seria eksperymentow identyfikujaca wplyw IRQ/IO/alokacji na ogony opoznien.

## BONUS
- W praktyce lepiej miec wolniejsza, ale przewidywalna petle, niz szybsza z losowym jitterem. Determinizm jest funkcja architektury, nie tylko scheduler'a.
