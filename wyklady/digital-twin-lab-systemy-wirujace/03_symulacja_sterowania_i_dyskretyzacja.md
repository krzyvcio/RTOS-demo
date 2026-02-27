# Wykład 3: Symulacja sterowania (PI/PID, notch, LQR/MPC) i dyskretyzacja

## Cel
Przetestować sterowanie zanim pojawi się hardware:
- regulatory PI/PID i anti-windup,
- filtrację (notch, low-pass),
- scenariusze saturacji i zakłóceń,
- wpływ opóźnień i jitteru.

> TL;DR: Najpierw stabilne PI/PID z ograniczeniami i anti-windup. Dopiero potem „mądre” metody (LQR/MPC).

## Symulacja pętli sterowania: co musi być w modelu
- dyskretny czas próbkowania,
- opóźnienie w torze pomiaru i aktuacji,
- saturacje i ograniczenia ramp/jerk,
- szumy pomiarowe,
- zakłócenia `T_load`.

## Anti-windup: obowiązkowe przy saturacjach
Jeśli w sterowaniu jest człon całkujący i saturacja, to bez anti-windup dostaniesz:
- przeregulowania,
- długie „dochody”,
- w skrajnym przypadku oscylacje.

## Notch: stabilność vs wycinanie rezonansu
Notch to narzędzie diagnostyczne i stabilizacyjne, ale:
- dodaje fazę/opóźnienie,
- jest wrażliwy na przesunięcie częstotliwości rezonansu.

## LQR/MPC: kiedy warto
LQR:
- gdy masz sensowny model state-space i chcesz wielowymiarowy kompromis.

MPC:
- gdy ograniczenia są kluczowe i chcesz predykcji na horyzoncie,
- kosztem obliczeń i wrażliwości na model.

## Checklisty
- W symulacji są saturacje i anti-windup.
- W symulacji jest opóźnienie i jitter (scenariusze).
- Każda zmiana filtru/regulatora ma test regresji (te same przypadki).

## Slajdy (tekstowe)
### Slajd 1: Co musi mieć symulacja sterowania
- Dyskretyzacja, opóźnienia, saturacje
- Zakłócenia i szum

### Slajd 2: Anti-windup
- Bez tego PI/PID „puchnie” na saturacji

### Slajd 3: LQR/MPC
- Tak, ale dopiero po „higienie” PI/PID

## Pytania do studentów
1. Jakie są minimalne testy regresji dla regulatora prędkości (skok zadania, skok obciążenia, saturacja)?
2. Jak zaprojektujesz anti-windup i jak zweryfikujesz jego działanie w scenariuszu saturacji?
3. Kiedy notch jest właściwą reakcją, a kiedy lepsze jest ograniczenie jerk lub pasma?
4. Jakie ryzyko wnosi MPC w systemie RT i jak je ograniczysz (limit czasu, fallback)?

## Projekty studenckie
- „Discrete PI kit”: regulator PI w dyskrecie + anti-windup + testy jednostkowe.
- „Filter lab”: porównanie wpływu notcha i ramp/jerk na widmo i błąd regulacji.
- „MPC sandbox”: prosty MPC na krótkim horyzoncie z limitem czasu i mechanizmem fallback.

## BONUS
- Ustal „kontrakt czasowy” dla algorytmu: jeśli obliczenia nie zmieszczą się w budżecie, algorytm ma oddać wynik awaryjny, a nie „spóźniony”.
