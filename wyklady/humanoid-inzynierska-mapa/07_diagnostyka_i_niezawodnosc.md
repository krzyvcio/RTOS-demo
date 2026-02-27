# Wykład 7: Diagnostyka, testy, niezawodność

## Cel: wykrywać degradację zanim stanie się awarią
Humanoid ma dużo elementów, które degradują "po cichu":
- łożyska i przekładnie (wzrost tarcia, luzy),
- złącza i wiązki (przerywane połączenia),
- sensory (dryf, utrata kalibracji),
- chłodzenie (przegrzewanie pod obciążeniem),
- zasilanie (spadki napięcia, zakłócenia).

Diagnostyka ma trzy cele:
- bezpieczeństwo: szybko wykryć stan zagrażający,
- dostępność: utrzymać działanie w trybie degradacji,
- serwis: dostarczyć dane, które pozwalają naprawić przyczynę, a nie objaw.

## Grafy i modele przyczynowo-skutkowe
### Fault Tree (drzewo błędów)
To model top-down:
- zaczynasz od zdarzenia krytycznego (np. upadek, brak momentu w nodze),
- rozbijasz na przyczyny logiczne (AND/OR),
- kończysz na elementarnych przyczynach (np. czujnik prądu, wtyczka, driver).

Zastosowania:
- analiza ryzyka,
- identyfikacja punktów, gdzie warto dodać redundancję lub testy.

### Graf przyczynowo-skutkowy
To bardziej ogólny graf zależności:
- węzły: symptomy i przyczyny,
- krawędzie: wpływ (często z wagą lub prawdopodobieństwem).

W praktyce pomaga budować systemy diagnozy online:
- z jednego symptomu (np. wzrost poboru prądu) nie wnioskujesz automatycznie o przyczynie,
- potrzebujesz kombinacji objawów i kontekstu (temperatura, prędkość, obciążenie).

### Łańcuch Markowa (OK/DEGRADED/FAIL)
To prosty model probabilistyczny:
- stany: OK, DEGRADED, FAIL,
- przejścia opisują degradację w czasie.

To jest użyteczne do:
- prognozowania ryzyka (predictive maintenance),
- symulowania wpływu strategii serwisowej na dostępność.

### Pareto usterek
Zasada 80/20:
- mała liczba typów usterek odpowiada za większość przestojów.
Graf Pareto to narzędzie priorytetyzacji:
- najpierw naprawiasz te usterki, które najczęściej występują lub mają największy koszt.

## Metryki niezawodności: MTBF i MTTF
MTBF (Mean Time Between Failures):
- średni czas między awariami (dla elementów naprawialnych).

MTTF (Mean Time To Failure):
- średni czas do awarii (dla elementów nienaprawialnych lub w modelu uproszczonym).

Pułapka:
- sama średnia jest mało informatywna bez rozkładu (ogony decydują o ryzyku).

## Detekcja anomalii: od sygnałów do decyzji
Masz zwykle sygnały:
- prądy, napięcia, temperatury,
- błędy sterowania (tracking error),
- wibracje/akustyka (czasem),
- statystyki sieci i czasu (jitter, straty pakietów),
- wskaźniki estymatora (innowacje, kowariancje).

### PCA i odległość Mahalanobisa
PCA:
- redukuje wymiar i pozwala wyłapać nietypowe kombinacje cech.

Mahalanobis distance:
```text
d(x) = sqrt( (x - μ)^T Σ^{-1} (x - μ) )
```
Jest to naturalna miara "odstępstwa" w przestrzeni wielowymiarowej.

Praktyka:
- ustaw progi na podstawie danych z normalnej pracy,
- stosuj histerezę i okna czasowe, by nie reagować na pojedyncze piki.

### Analiza sygnałów: FFT, widmo, autokorelacja
FFT/widmo:
- wykrywa rezonanse, uszkodzenia mechaniczne, zakłócenia EMC,
- pozwala rozróżnić problem "stałej częstotliwości" od losowego szumu.

Autokorelacja:
- wykrywa periodyczne zaburzenia i powtarzalne wzorce (np. uszkodzony ząb przekładni).

## Testy: regresja i statystyka
W systemach humanoida testy dzielą się na:
- testy jednostkowe (algorytmy, transformacje, filtry),
- testy integracyjne (czujniki -> estymacja -> kontrola),
- testy HIL/SIL (symulacja i hardware-in-the-loop),
- testy obciążeniowe (worst-case timing).

Analiza wariancji i statystyka testów regresji pomagają odpowiedzieć:
- czy zmiana poprawiła system, czy tylko zmieniła rozkład błędu,
- czy poprawa jest stabilna w wielu scenariuszach.

## Praktyka inżynierska: architektura diagnostyki
- Diagnostyka online: szybkie, konserwatywne decyzje bezpieczeństwa.
- Diagnostyka offline: dokładna analiza logów i korelacji, znalezienie przyczyny.

Eskalacja alertów (typowy schemat):
- ostrzeżenie: trend pogorszenia, nadal bezpiecznie,
- degradacja: ogranicz osiągi (prędkość, moment, zakres ruchu),
- zatrzymanie bezpieczne: natychmiastowe przejście do safe state.

## Checklisty
- wspólne metryki i format logów dla online/offline,
- loguj kontekst: stan, obciążenie, temperatura, czas, tryb kontaktu,
- wprowadź "health score" dla podsystemów (napędy, IMU, wizja, sieć),
- regularnie aktualizuj Pareto usterek na podstawie danych z eksploatacji.

## Pytania do studentów
1. Dlaczego bez „kontekstu” (tryb, obciążenie) detekcja anomalii daje dużo false positives?
2. Jak zbudujesz fault tree dla zdarzenia „utrata stabilności” w robocie?
3. Kiedy PCA/Mahalanobis ma sens, a kiedy lepsze są progi i trendy?
4. Jakie dane są niezbędne, żeby diagnoza była możliwa bez dostępu do robota?

## Projekty studenckie
- „Health score”: agregacja metryk do wskaźnika stanu z eskalacją WARNING/DEGRADED/SAFE_STOP.
- Detektor anomalii Mahalanobisa na cechach (prąd, temp, jitter, wibracje) + walidacja na danych.
- Raport Pareto usterek z logów + plan działań „top 3” poprawiających dostępność.

## BONUS
- Najlepsze systemy diagnostyki mają tę samą cechę co dobre sterowanie: są odporne na outliery i nie destabilizują systemu, który obserwują (asynchroniczność i ograniczanie wpływu).
