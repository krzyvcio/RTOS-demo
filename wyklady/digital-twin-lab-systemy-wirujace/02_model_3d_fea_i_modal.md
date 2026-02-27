# Wykład 2: Model 3D, FEA i analiza modalna (rezonanse)

## Cel
Zrozumieć, po co i kiedy wprowadzać narzędzia FEA/modal, oraz jak przenieść wyniki do modelu użytecznego w sterowaniu i diagnostyce.

> TL;DR: FEA daje częstotliwości własne i mody. Sterowanie potrzebuje z tego: listy rezonansów, ich czułości na warunki i sygnałów, na których to widać.

## Co daje analiza modalna
Wyniki, które są praktycznie użyteczne:
- częstotliwości własne (gdzie spodziewać się pików),
- postacie drgań (co „pracuje” i gdzie mierzyć),
- wpływ zmian sztywności (np. mocowanie, łożyska) na przesunięcie rezonansu.

## Jak używać wyników w labie
Minimalny transfer do świata sterowania:
- lista potencjalnych pików do monitorowania w FFT,
- kandydaci do notch (z ostrożnością),
- scenariusze testowe: przejścia przez zakresy prędkości, gdzie mody są pobudzane.

## Pułapki
- FEA jest „idealne”, a realna konstrukcja ma luzy, tolerancje i zmienność.
- Rezonanse potrafią pływać z temperaturą i obciążeniem.
- Model zbyt szczegółowy nie nadaje się do szybkiej symulacji sterowania.

## Checklisty
- Wyniki FEA są przetłumaczone na: „co monitorujemy” i „jak reagujemy”.
- Masz plan walidacji: FFT na sygnałach + korelacja z trybem i obciążeniem.

## Slajdy (tekstowe)
### Slajd 1: Co daje FEA
- Częstotliwości własne
- Postacie drgań

### Slajd 2: Jak to wykorzystać
- Monitoring pików w FFT
- Kandydaci do notch

## Pytania do studentów
1. Jak przetłumaczysz wynik FEA (mody, częstotliwości) na konkretne decyzje w sterowaniu i diagnostyce?
2. Jakie są 3 typowe różnice między FEA a rzeczywistością i jak je uwzględnisz w labie?
3. Jak sprawdzisz, czy pik w FFT to rezonans konstrukcji, a nie artefakt pomiaru (aliasing/filtr)?
4. Jak zaprojektujesz plan walidacji „FEA -> pomiary -> reakcja”?

## Projekty studenckie
- „Modal-to-monitor”: generator listy częstotliwości do monitorowania + progi trendów.
- „Notch candidates”: raport kandydatów do notcha z oceną ryzyka (zmienność z temperaturą/obciążeniem).
- „Placement study”: propozycja miejsc montażu czujników (na podstawie postaci drgań).

## BONUS
- Traktuj FEA jako generator hipotez, nie jako wyrocznię: największa wartość jest w tym, że mówi „gdzie patrzeć” i „czego się spodziewać” w danych.
