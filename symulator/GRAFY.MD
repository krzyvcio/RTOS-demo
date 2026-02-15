Świetne kompendium wrzuciłeś — to jest już **pełna inżynierska mapa systemu humanoida**. Zrobię Ci teraz **transfer wiedzy** w jednym miejscu: **jakie grafy, modele i metody matematyczne realnie stoją za tym wszystkim** (sterowanie, sieci, synchronizacja, planowanie, diagnostyka, EMC, RT, bezpieczeństwo).

Podzielę na **bloki zastosowań** + **konkretne narzędzia matematyczne**.

---

# 1️⃣ Kinematyka, geometria ruchu, manipulatory

**Grafy / struktury:**

* 🌳 Drzewo kinematyczne robota (kinematic tree)
* 📈 Graf połączeń przegubów (joint graph)
* 🔗 Graf łańcucha kinematycznego
* 🧭 Graf kontaktów (stopy–podłoże, ręka–obiekt)
* 🕸️ Graf brył sztywnych (rigid body graph)

**Matematyka:**

* Macierze jednorodne 4×4 (SE(3))
* Grupy Liego: **SO(3), SE(3)**
* Algebry Liego: **so(3), se(3)** (log/exp mapy)
* Macierze Jacobiego
* Odwrotna kinematyka:

  * Least Squares
  * Damped Least Squares (Levenberg–Marquardt)
  * Pseudoodwrotność Moore’a–Penrose’a
* Rozwiązywanie układów nieliniowych (Newton-Raphson)
* Ograniczenia nierówności (QP / NLP)
* Singular Value Decomposition (SVD)
* Analiza osobliwości (det(J))

---

# 2️⃣ Dynamika, siły, momenty, kontakt

**Grafy / struktury:**

* 🕸️ Graf brył połączonych przegubami
* 🌳 Drzewo dynamiki (articulated body tree)
* 🦶 Graf kontaktów (contact graph)
* ⚖️ Graf przepływu sił

**Matematyka:**

* Równania Lagrange’a
* Równania Newton–Euler
* Algorytmy:

  * RNEA (Recursive Newton-Euler Algorithm)
  * ABA (Articulated Body Algorithm)
  * CRBA (Composite Rigid Body Algorithm)
* Macierze:

  * Macierz mas M(q)
  * Macierz Coriolisa C(q, q̇)
  * Wektor grawitacji g(q)
* Równania różniczkowe zwyczajne (ODE)
* Modele kontaktu:

  * Stożek tarcia (Coulomb)
  * Modele sprężysto-tłumiące (Kelvin–Voigt)
* Programowanie wypukłe:

  * QP (Quadratic Programming) dla rozkładu sił
  * LCP / MCP (Linear/ Mixed Complementarity Problems)

---

# 3️⃣ Sterowanie (FOC, impedancja, stabilność)

**Grafy / struktury:**

* 📊 Schematy blokowe (block diagrams)
* 🔁 Graf pętli sprzężenia zwrotnego
* 📉 Diagramy Bodego / Nyquista / Root Locus
* 🧠 Graf stanów (state-space graph)

**Matematyka:**

* Równania stanu (state-space):

  * ẋ = Ax + Bu
  * y = Cx + Du
* Regulatory:

  * PID / PI / PD
  * LQR / LQG
  * MPC (Model Predictive Control)
* Sterowanie impedancyjne / admisyjne
* Transformacje:

  * Clarke
  * Park
* FOC:

  * Modele dq
  * Regulatory prądu PI
* Stabilność:

  * Kryterium Nyquista
  * Kryterium Routh–Hurwitza
  * Analiza biegunów i zer
* Filtry:

  * Notch
  * Butterworth / Biquad
  * Filtry Kalmana

---

# 4️⃣ Estymacja stanu, fuzja sensorów

**Grafy / struktury:**

* 🕸️ Graf czynników (Factor Graph)
* 🧩 Graf Bayesowski
* 🧠 Hidden Markov Model
* 🌐 Graf zależności sensorów

**Matematyka:**

* Filtry:

  * Kalman Filter (KF)
  * Extended KF (EKF)
  * Unscented KF (UKF)
  * Particle Filter
* Estymacja:

  * Least Squares
  * Weighted Least Squares
* Rachunek prawdopodobieństwa:

  * Bayes
  * Rozkłady Gaussa
* Optymalizacja nieliniowa:

  * Gauss–Newton
  * Levenberg–Marquardt
* SLAM:

  * Bundle Adjustment
  * Pose graph optimization

---

# 5️⃣ Planowanie ruchu, trajektorie

**Grafy / struktury:**

* 🗺️ Graf stanów (state graph)
* 🌲 Drzewa przeszukiwania:

  * RRT, RRT*
  * PRM
* 🧭 Graf drogowy (roadmap)
* 📦 Graf kolizji / konfiguracji (C-space graph)

**Matematyka:**

* Przestrzeń konfiguracji C-space
* Interpolacja:

  * Splajny (cubic, quintic)
  * B-splines
  * Bezier
* Optymalizacja:

  * Trajectory optimization
  * CHOMP, STOMP, TrajOpt
* Ograniczenia:

  * QP / NLP / SQP
* Kryteria kosztu:

  * Energia
  * Jerk
  * Czas
* A*, D*, Dijkstra

---

# 6️⃣ Sieci czasu rzeczywistego, synchronizacja

**Grafy / struktury:**

* 🌐 Graf sieci (nodes + edges)
* ⏱️ Graf harmonogramu (time-triggered schedule)
* 📡 Graf przepływu danych (dataflow graph)
* 🔄 Graf zależności czasowych

**Matematyka:**

* Teoria kolejek (queueing theory)
* Analiza WCET / WCRT
* Teoria harmonogramowania:

  * Rate Monotonic
  * EDF
  * Deadline Monotonic
* Synchronizacja:

  * Model zegarów
  * Estymacja offsetu i dryftu
* Algebra max-plus (dla sieci czasu rzeczywistego)
* Grafy przepływu z ograniczeniami czasowymi

---

# 7️⃣ Diagnostyka, testy, niezawodność

**Grafy / struktury:**

* 🧩 Drzewo błędów (Fault Tree)
* 🕸️ Graf przyczynowo-skutkowy
* 🔁 Markov Chain (stany: OK / DEGRADED / FAIL)
* 📊 Graf Pareto usterek

**Matematyka:**

* Teoria niezawodności:

  * MTBF, MTTF
* Łańcuchy Markowa
* Statystyka:

  * Testy regresji
  * Analiza wariancji
* Detekcja anomalii:

  * PCA
  * Mahalanobis distance
* Analiza sygnałów:

  * FFT
  * Analiza widmowa
  * Autokorelacja

---

# 8️⃣ EMC, sygnały, drgania, rezonanse

**Grafy / struktury:**

* 📈 Widma częstotliwości
* 📉 Odpowiedzi częstotliwościowe
* 🧱 Modele RLC jako graf obwodów
* 🕸️ Graf impedancji

**Matematyka:**

* FFT / DFT
* Analiza Laplace’a
* Analiza Z-transform
* Modele:

  * RLC
  * Transmitancje
* Filtry:

  * FIR / IIR
* Analiza rezonansów:

  * Wartości własne
  * Postacie drgań (modal analysis)

---

# 9️⃣ Bezpieczeństwo funkcjonalne, logika, nadzór

**Grafy / struktury:**

* 🔐 Graf stanów bezpieczeństwa (FSM)
* 🌳 Drzewo decyzyjne
* 🧯 Graf ścieżek awaryjnych
* 🔄 Graf watchdogów i nadzorców

**Matematyka:**

* Automaty skończone (FSM)
* Logika temporalna (LTL / CTL)
* Analiza osiągalności stanów
* Model checking
* Sieci Petriego (Petri Nets)

---

# 🔟 Optymalizacja globalna systemu

**Grafy / struktury:**

* 📊 Graf kosztów i ograniczeń
* 🧮 Graf zależności zmiennych

**Matematyka:**

* Programowanie:

  * Liniowe (LP)
  * Kwadratowe (QP)
  * Nieliniowe (NLP)
  * Mieszane (MILP / MIQP)
* Metody:

  * Gradient descent
  * Newton / Quasi-Newton
  * Interior-point
* Analiza wypukłości

---

# 🧠 TL;DR — w jednym zdaniu

W tych systemach używasz **grafów kinematycznych, grafów kontaktów, grafów sieci, grafów stanów, grafów czynników, drzew planowania i grafów błędów**, a matematycznie opierasz się na **algebrze liniowej, grupach Liego, równaniach różniczkowych, teorii sterowania, optymalizacji wypukłej i nieliniowej, probabilistyce, teorii grafów, teorii harmonogramowania i analizie sygnałów**.

---

Jeśli chcesz, mogę:

* zrobić **mapę “co do czego”** (np. EtherCAT → jakie modele czasu, jakie równania),
* albo rozpisać **konkretny stack matematyczny** dla: *jednego stawu*, *jednej nogi*, albo *całego humanoida*.
