# Wykład 3: Sterowanie, FOC, impedancja, stabilność

## Cel i architektura warstw
Sterowanie humanoida to stos warstw o bardzo różnych pasmach:
- pętla prądu (kHz): steruje momentem w silniku,
- pętla prędkości/pozycji (setki Hz): stabilizuje przegub,
- warstwa zadaniowa (dziesiątki Hz): śledzi trajektorie dłoni/stóp, postawę, CoM,
- warstwa planowania (Hz): decyduje o krokach, kontaktach, strategii.

Z punktu widzenia stabilności kluczowe jest, by każda warstwa widziała "w miarę idealny" obiekt sterowania.
Jeśli pętla prądu jest słaba, wyższe warstwy dostają nieliniowy, opóźniony i zaszumiony napęd.

## Grafy i narzędzia projektowe
W praktyce sterowanie opisujesz trzema językami równolegle:
- schematy blokowe (intuicja przepływu sygnału i miejsc, gdzie powstaje opóźnienie),
- transmitancje i charakterystyki częstotliwościowe (Bode/Nyquist) dla analizy pasma i marginesów,
- state-space (A,B,C,D) dla nowoczesnych metod (LQR/LQG/MPC).

## Modele: od ciągłych do dyskretnych
W teorii często startujesz od modelu ciągłego:
```text
xdot = A x + B u
y    = C x + D u
```
W implementacji wszystko jest dyskretne, więc realnie pracujesz z:
```text
x_{k+1} = A_d x_k + B_d u_k
y_k     = C x_k + D u_k
```
Trzy rzeczy, które najczęściej psują zgodność z teorią:
- opóźnienie obliczeń i komunikacji (zawsze jest),
- kwantyzacja i saturacje (PWM, prądy, momenty),
- aliasing i filtracja (niewłaściwe filtry, brak antyaliasingu).

## Regulatory klasyczne: PID/PI/PD
PID jest wciąż podstawą w napędach, bo jest:
- prosty,
- przewidywalny,
- łatwy do strojenia na obiekcie.

Pułapki implementacyjne:
- anti-windup (inaczej całkujący człon "napompuje" się przy saturacji),
- filtracja członu D (pochodna wzmacnia szum),
- kompensacja grawitacji i tarcia (PID bez feed-forward często przegrywa z dynamiką humanoida).

## LQR/LQG i MPC: gdy potrzebujesz wielowymiarowości
LQR:
- dobiera sprzężenie `u = -K x` minimalizujące koszt kwadratowy,
- daje sensowne kompromisy między energią sterowania a błędem.

LQG:
- LQR + estymator (Kalman), gdy nie mierzysz całego stanu.

MPC:
- rozwiązuje problem optymalizacji na horyzoncie,
- naturalnie obsługuje ograniczenia (np. limity momentów, prędkości, tarcie kontaktu),
- kosztem obliczeń i wrażliwości na model.

## Impedancja i admisja: interakcja z otoczeniem
W humanoidzie "sztywne" śledzenie pozycji zwykle przegrywa w kontaktach.
Sterowanie impedancyjne:
- narzuca relację siła <-> odchyłka (jak sprężyna-tłumik w przestrzeni zadania),
- jest intuicyjne i bezpieczniejsze przy niepewnym kontakcie.

Sterowanie admisyjne:
- przekształca mierzoną siłę w ruch (odwrotna relacja),
- bywa wygodne, gdy masz dobre czujniki siły i chcesz "podążać" za człowiekiem/otoczeniem.

## FOC (Field-Oriented Control): co realnie robisz w napędzie
FOC sprowadza sterowanie silnikiem do regulacji prądów w osiach `d/q`.
Najczęściej spotkasz:
- Clarke: fazy -> (alpha, beta),
- Park: (alpha, beta) -> (d, q) w ramce wirującej.

W osi `q` prąd odpowiada za moment (dla wielu maszyn), więc pętla prądu w FOC to de facto pętla momentu.
Typowe elementy implementacji:
- regulatory PI dla `i_d` i `i_q`,
- odsprzęganie członów zależnych od prędkości elektrycznej,
- ograniczenia napięcia (saturacja) i prądu,
- obserwator/estymacja kąta elektrycznego (czujnik lub sensorless).

Praktyczna zasada pasm:
- pętla prądu ma być istotnie szybsza niż mechanika i pętle wyższego poziomu,
- inaczej cała struktura kaskadowa traci sens.

## Stabilność: jak to diagnozować i stroić
Narzędzia klasyczne:
- Nyquist: ocena stabilności w pętli,
- Routh–Hurwitz: stabilność wielomianu charakterystycznego,
- Bode: pasmo, margines fazy i wzmocnienia.

W praktyce inżynierskiej:
- najpierw budujesz zapas stabilności (marginesy),
- dopiero potem gonisz wydajność (szybkość odpowiedzi).

## Filtry: notch, biquad, Butterworth, Kalman
Notch:
- bardzo użyteczny w humanoidach do tłumienia rezonansów przekładni i konstrukcji,
- może jednak wprowadzić opóźnienie i pogorszyć margines fazy, jeśli jest nadużywany.

Biquad/Butterworth:
- typowe filtry w torze pomiarowym i wyznaczania prędkości.

Kalman:
- jednocześnie filtr i estymator,
- daje formalny sposób ważenia "model vs. pomiar", ale wymaga sensownego strojenia kowariancji.

## Dyskretyzacja, opóźnienie i jitter
W RT sterowaniu krytyczne są:
- stały czas próbkowania (jitter degraduje stabilność),
- znane opóźnienie (można je kompensować lepiej niż losowe),
- priorytety w systemie (żeby pętla prądu/pozycji zawsze wygrała z telemetrią).

## Checklisty
- Strojenie kaskadowe: najpierw prąd, potem prędkość, potem pozycja, a na końcu zadania w przestrzeni (dłoń/stopa).
- Anti-windup w każdym PI/PID z saturacją.
- Ograniczenie jerk i filtracja zadawania (setpoint shaping), bo mechanika humanoida źle znosi skoki.
- Logowanie: saturacje, rezonanse (FFT sygnałów błędu i sterowania), statystyki czasu iteracji.

## Pytania do studentów
1. Dlaczego strojenie „od góry” (najpierw pozycja) często kończy się niestabilnością?
2. Jak zmierzysz wpływ opóźnienia i jitteru na margines fazy w pętli?
3. Kiedy notch jest dobrym pomysłem, a kiedy lepiej obniżyć pasmo regulatora?
4. Jakie są symptomy braku anti-windup w danych logów?

## Projekty studenckie
- Implementacja dyskretnego PI/PID z anti-windup + testy saturacji i step response.
- Projekt filtru notch na podstawie FFT i walidacja „przed/po” (bez pogorszenia stabilności).
- Mini-MPC dla jednego przegubu z ograniczeniami momentu/prędkości + limit czasu obliczeń.

## BONUS
- Najczęstszy błąd projektowy: „dodaj filtr” bez policzenia opóźnienia; w praktyce filtr to koszt fazy, więc zawsze waliduj wpływ na stabilność, nie tylko na wygląd sygnału.
