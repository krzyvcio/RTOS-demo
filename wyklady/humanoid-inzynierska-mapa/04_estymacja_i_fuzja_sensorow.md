# Wykład 4: Estymacja stanu, fuzja sensorów

## Po co estymacja w humanoidzie
Sterowanie i planowanie potrzebują stanu, którego nie da się wprost zmierzyć:
- pozycja i orientacja bazy (tułów/miednica) w świecie,
- prędkości i przyspieszenia,
- biasy IMU (dryfty żyroskopu i akcelerometru),
- siły kontaktu (częściowo z czujników, częściowo z estymacji),
- stan otoczenia (pozycja obiektów, mapy, krawędzie podłoża).

Fuzja sensorów jest odpowiedzią na fakt, że każdy czujnik ma inne wady:
- IMU jest szybkie, ale dryfuje,
- wizja jest wolniejsza i bywa zawodna, ale daje informację absolutną,
- enkodery są dokładne lokalnie, ale nie mówią nic o poślizgu stóp.

## Grafy i struktury probabilistyczne
W robotyce spotkasz kilka równoważnych języków opisu tego samego problemu.

### Bayes i modele stanu (state-space)
Najbardziej klasyczna forma:
```text
x_{k+1} = f(x_k, u_k) + w_k
z_k     = h(x_k)      + v_k
```
gdzie:
- `x_k` to stan (np. pozycja, orientacja, prędkość, biasy),
- `u_k` to sterowania (np. komendy napędów),
- `z_k` to pomiary (IMU, enkodery, kamera),
- `w_k`, `v_k` to szumy procesu i pomiaru.

### Hidden Markov Model (HMM)
Gdy stan jest dyskretny lub ma silny komponent dyskretny:
- tryb kontaktu (kontakt / brak kontaktu),
- tryb awarii (OK / degraded / fail),
- klasyfikacje środowiska (np. typ podłoża).

### Factor Graph
Factor graph to w praktyce "system równań" opisany jako graf:
- wierzchołki to zmienne (pozy w czasie, landmarki, biasy),
- czynniki (faktory) to pomiary i więzy (IMU preintegration, odometria, wizja).

To podejście dominuje w nowoczesnych systemach SLAM i estymacji pozy, bo:
- jest modularne (łatwo dodać nowy czujnik jako nowy faktor),
- naturalnie rozwiązuje problem jako MAP (maximum a posteriori) przez optymalizację.

## Kalman Filter (KF): rdzeń estymacji liniowej
KF zakłada liniowy model z szumami Gaussa:
```text
x_{k+1} = A x_k + B u_k + w_k
z_k     = H x_k         + v_k
```

Kroki:
1) Predykcja:
```text
x^- = A x + B u
P^- = A P A^T + Q
```
2) Aktualizacja:
```text
S   = H P^- H^T + R
K   = P^- H^T S^{-1}
x   = x^- + K (z - H x^-)
P   = (I - K H) P^-
```

Interpretacja praktyczna:
- `Q` mówi, jak bardzo nie ufasz modelowi,
- `R` mówi, jak bardzo nie ufasz pomiarom,
- `P` to niepewność stanu, która rośnie bez pomiarów absolutnych.

## EKF i UKF: nieliniowość w praktyce
W humanoidzie większość modeli jest nieliniowa (rotacje, kontakty).

EKF (Extended KF):
- linearyzuje `f` i `h` w punkcie pracy (Jacobianami),
- jest szybki, ale bywa wrażliwy na złe linearyzacje i outliery.

UKF (Unscented KF):
- zamiast Jacobianów używa punktów sigma,
- często lepiej znosi nieliniowość przy podobnym koszcie,
- nadal zakłada "mniej więcej" Gaussowskość rozkładów.

Praktyczny wybór:
- EKF wygrywa prostotą w systemach wbudowanych,
- UKF wygrywa, gdy trudno o poprawne pochodne albo nieliniowość jest duża,
- factor graph wygrywa, gdy masz dużo czujników i zależy Ci na globalnej spójności.

## Particle Filter: gdy Gauss nie wystarcza
Particle filter reprezentuje rozkład jako chmurę próbek.
To sensowne, gdy:
- rozkład jest wielomodalny (np. lokalizacja z symetriami),
- obserwacje są bardzo nieliniowe,
- chcesz naturalnie kodować ograniczenia nieliniowe.

Koszt:
- większe obliczenia,
- ryzyko degeneracji próbek (wymaga resamplingu i dobrego modelu).

## Least Squares, WLS i MAP jako optymalizacja
Bardzo dużo problemów estymacji sprowadza się do postaci:
```text
min_x  Σ_i || r_i(x) ||_{W_i}^2
```
gdzie `r_i(x)` to residuum pomiaru (różnica "pomiary - model").
Dla szumów Gaussa jest to równoważne MAP.

Gauss–Newton:
- iteracyjnie rozwiązuje liniowe przybliżenie problemu LS,
- jest szybki, gdy residua są "w miarę" liniowe w pobliżu optimum.

Levenberg–Marquardt:
- dodaje tłumienie (regularizację) i stabilizuje w trudnych przypadkach,
- działa podobnie jak DLS w IK, tylko w przestrzeni estymacji.

## SLAM: pose graph i bundle adjustment
Pose graph optimization:
- zmienne to pozy w kolejnych chwilach,
- faktory to odometria/IMU/wizja (relacje między pozami).
To daje spójność globalną i korektę dryfu.

Bundle adjustment:
- do pozy dochodzą landmarki/parametry kamery,
- rozwiązuje się duży problem nieliniowej optymalizacji.

W humanoidzie nie zawsze robisz "pełny SLAM":
- czasem wystarczy "localization in a map",
- czasem estymacja bazy + orientacji i wysokości podłoża.

## Kluczowe aspekty wdrożeniowe
### Synchronizacja czasu i opóźnienia
Fuzja jest tak dobra, jak dobra jest oś czasu.
Typowe problemy:
- różne zegary czujników,
- opóźnienia przesyłu i przetwarzania (kamera),
- jitter w RT pętli estymacji.

Standardowa praktyka:
- stempluj pomiary możliwie blisko sprzętu,
- w estymatorze jawnie uwzględnij opóźnienie (np. buffer stanów i update w przeszłości),
- monitoruj statystyki opóźnień jako metrykę jakości.

### Outliery i odporność (robustness)
Wizja i czujniki kontaktu generują outliery.
Niezbędne elementy:
- gating (odrzucanie pomiarów o zbyt dużej innowacji),
- funkcje kosztu odporne (Huber/Cauchy) w factor graph,
- fallback na inne sensory w razie utraty jednego strumienia.

### Obserwowalność i dryf
Jeśli nie masz absolutnych pomiarów (np. globalnej pozycji), pewne składowe stanu będą dryfować.
Przykład:
- IMU + enkodery bez wzrokowego/absolutnego odniesienia dają dryf pozycji bazy.

Dlatego w architekturze często rozdziela się:
- szybką estymację lokalną (IMU/enkodery, wysoka częstotliwość),
- wolną korektę globalną (wizja/mapa, niższa częstotliwość).

## Checklisty
- Zdefiniuj stan `x`: co jest estymowane (pozy, prędkości, biasy, kontakty).
- Zdefiniuj ramki i transformacje sensorów (kalibracja extrinsic).
- Loguj innowację i jej statystyki (wykrywanie rozjazdów).
- Dodaj mechanizm resetu/rekonwergencji (po upadku, po dużym outlierze).
- Mierz koszty obliczeń i pilnuj deterministycznego czasu iteracji.

## Pytania do studentów
1. Dlaczego timestamping i synchronizacja czasu są krytyczne dla fuzji sensorów?
2. Kiedy EKF przegrywa z factor graph i odwrotnie (modularność vs koszt online)?
3. Jak zaprojektujesz gating/outlier rejection, żeby nie „zatruć” estymatora?
4. Jak rozpoznasz problem obserwowalności (dryf) w danych?

## Projekty studenckie
- EKF dla prostego modelu (IMU + enkodery) z biasem żyroskopu + logowanie innowacji.
- Factor graph dla kilku poz (pose graph) z odporną funkcją kosztu (Huber) + porównanie z EKF.
- System synchronizacji: bufor stanów i update „w przeszłości” dla opóźnionych pomiarów (kamera).

## BONUS
- Najszybciej wykryjesz problemy estymatora po statystykach innowacji: jeśli innowacja rośnie lub ma nielogiczny rozkład, to zwykle problem jest w czasie, kalibracji albo outlierach.
