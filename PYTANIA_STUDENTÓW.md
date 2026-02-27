# Pytania studentow do wykladow

## Wyklad: Percepcja i wizja

1. Dlaczego pojedynczy sensor zwykle nie wystarcza w robotyce autonomicznej?
2. Jakie sa praktyczne roznice miedzy LiDAR, ToF i stereo vision?
3. W jakich sytuacjach kamera eventowa ma przewage nad klasyczna kamera RGB?
4. Co jest najtrudniejsze w fuzji danych z kamery i IMU?
5. Kiedy klasyczne podejscia typu HOG + SVM sa lepsze od sieci neuronowych?
6. Jakie ograniczenia sprzetowe najbardziej blokuja wdrozenie YOLO na robocie?
7. Jak mozna ocenic wiarygodnosc detekcji obiektu bez ground truth?

## Wyklad: LLM w robotyce

1. Co w praktyce oznacza, ze LLM "rozumie" polecenie?
2. Jak ograniczyc halucynacje LLM w systemie sterowania robotem?
3. Ktore elementy pipeline (parsowanie, mapowanie, walidacja) sa krytyczne dla bezpieczenstwa?
4. Jakie sa zalety i ryzyka generowania kodu przez LLM dla robotow?
5. Jak walidowac plan dzialania wygenerowany przez LLM w czasie rzeczywistym?
6. Kiedy lepiej uzyc klasycznego planowania niz LLM?
7. Jak polaczyc LLM z RTOS bez utraty deterministycznosci?

## Wyklad: Cyberbezpieczenstwo robotow

1. Ktora warstwa ataku na robota jest najczesciej zaniedbywana i dlaczego?
2. Jak rozpoznac, ze robot jest celem ataku typu man-in-the-middle?
3. Jakie sa minimalne wymagania bezpieczenstwa dla ROS2 w produkcji?
4. W jaki sposob ataki na sensory (np. spoofing LiDAR) wplywaja na decyzje robota?
5. Jak budowac whitelisty i ograniczenia bez zablokowania normalnej pracy?
6. Jakie sa skutki uboczne silnego szyfrowania w systemach czasu rzeczywistego?
7. Jak testowac odpornosc robota na ataki bez ryzyka dla sprzetu?

## Wyklad: Robotyka roju

1. Co to jest zachowanie emergentne i jak je mierzyc w praktyce?
2. Jakie reguly lokalne najczesciej prowadza do stabilnej globalnej organizacji?
3. Jak unikac kolizji w roju bez centralnego koordynatora?
4. Jakie protokoly komunikacji najlepiej sprawdzaja sie w sieci mesh roju?
5. Jakie zadania w roju musza byc twardo deterministyczne?
6. Jak kontrolowac zuzycie energii, gdy liczba agentow jest duza?
7. Jak testowac algorytmy roju, zanim trafi sie na realny sprzet?


---

# 🧠 Percepcja i wizja — nowe pytania studentów

### Sensory i przetwarzanie
- Dlaczego roboty wciąż mają problem z przezroczystymi obiektami, skoro ludzie widzą je bez trudu?
- Czy istnieje sensowny sposób na wykrywanie przeszkód przy pełnym oślepieniu kamery (np. słońce prosto w obiektyw)?
- Jakie są praktyczne limity rozdzielczości LiDAR — czy „więcej punktów” zawsze oznacza lepszą percepcję?

### Algorytmy
- Czy klasyczne metody (SIFT/ORB) mają jeszcze przewagę nad CNN w warunkach ekstremalnych?
- Jak ocenić jakość segmentacji, jeśli nie mamy ground truth w czasie rzeczywistym?

### Fuzja danych
- Czy można zrobić stabilny SLAM tylko z IMU + kamera eventowa?
- Jakie są najczęstsze błędy przy synchronizacji czasowej sensorów?

---

# 🤖 LLM w robotyce — nowe pytania studentów

### Rozumienie i planowanie
- Skąd wiemy, że LLM „zrozumiał” polecenie, a nie tylko wygenerował coś statystycznie sensownego?
- Czy LLM może samodzielnie wykryć, że jego plan jest nielogiczny?

### Bezpieczeństwo i niezawodność
- Jak wykrywać halucynacje, jeśli nie mamy referencji ani symulatora?
- Czy LLM może być deterministyczny, jeśli tego wymagamy?

### Integracja z systemem
- Jakie są minimalne wymagania, żeby LLM mógł współpracować z RTOS bez ryzyka opóźnień?
- Czy LLM może zastąpić klasyczny planner (A*, RRT) w robotyce mobilnej?

---

# 🔐 Cyberbezpieczeństwo robotów — nowe pytania studentów

### Ataki i obrona
- Jak wykryć, że ktoś manipuluje danymi z IMU, skoro to sensor o wysokiej częstotliwości?
- Czy robot może sam wykryć, że jego model percepcji został zaatakowany adversarialnie?

### Architektura
- Jak projektować system, który pozostaje bezpieczny nawet przy całkowitym przejęciu jednego z modułów?
- Czy ROS2 DDS Security jest wystarczający w środowisku przemysłowym?

### Operacje
- Jak testować cyberodporność robotów, jeśli nie możemy ich fizycznie uszkodzić?
- Czy robot powinien mieć „czarną skrzynkę” jak samolot?

---

# 🐜 Robotyka roju — nowe pytania studentów

### Zachowania emergentne
- Jak odróżnić zachowanie emergentne od zwykłego chaosu?
- Czy rój może się „rozsynchronizować” i jak temu zapobiegać?

### Komunikacja
- Jakie są praktyczne limity komunikacji mesh przy setkach agentów?
- Czy rój może działać całkowicie bez komunikacji radiowej?

### Skalowanie i niezawodność
- Jak projektować algorytmy roju, które działają równie dobrze dla 10, jak i dla 10 000 robotów?
- Co zrobić, gdy część roju zaczyna działać niezgodnie z protokołem (np. uszkodzenie, atak)?

---

# ⚙️ RTOS i systemy czasu rzeczywistego — nowe pytania studentów

### Determinizm
- Jak mierzyć jitter w systemie, który ma wiele tasków o różnych priorytetach?
- Czy istnieje „idealny” scheduler dla robotyki?

### Architektura
- Jak projektować system, który nie zawiesi się nawet przy błędach w taskach niskiego priorytetu?
- Czy dynamiczna alokacja pamięci *zawsze* jest zła w RTOS?

### Integracja
- Jak zapewnić, że komunikacja Linux ↔ RTOS nie wprowadzi nieprzewidywalnych opóźnień?
- Czy można zrobić stabilną kontrolę 1 kHz na Linuxie z PREEMPT_RT?

arialne. Który sensor jest najłatwiejszy do oszukania i dlaczego?  
4. Wyjaśnij, dlaczego klasyczne metody (SIFT/ORB) mogą przewyższać CNN w systemach krytycznych bezpieczeństwa.  
5. Zaproponuj metodę oceny jakości segmentacji semantycznej w czasie rzeczywistym bez ground truth. Jakie heurystyki mogą wykryć błędną segmentację?

---

# 🤖 **LLM w robotyce — trudne pytania egzaminacyjne**

1. Wyjaśnij, dlaczego LLM nie może być traktowany jako deterministyczny komponent systemu sterowania. Jakie mechanizmy muszą go otaczać, aby był bezpieczny?  
2. Zaprojektuj architekturę, w której LLM generuje plan działania, ale RTOS gwarantuje bezpieczeństwo. Jakie warstwy walidacji są konieczne?  
3. Podaj przykład polecenia, które LLM może „zrozumieć” błędnie mimo poprawnej składni. Wyjaśnij, jak temu zapobiec.  
4. Jak wykrywać halucynacje LLM w systemie, który nie ma dostępu do symulatora ani ground truth?  
5. Wyjaśnij, dlaczego LLM nie może zastąpić klasycznego planera ruchu w robotyce mobilnej — nawet jeśli generuje poprawne trajektorie.

---

# 🔐 **Cyberbezpieczeństwo robotów — trudne pytania egzaminacyjne**

1. Zaprojektuj atak typu man‑in‑the‑middle na komunikację RTOS ↔ Linux w robocie mobilnym. Jakie dane są najbardziej krytyczne?  
2. Wyjaśnij, dlaczego spoofing sensorów (np. LiDAR) jest trudniejszy do wykrycia niż spoofing komunikacji.  
3. Podaj przykład, w którym silne szyfrowanie *obniża* bezpieczeństwo robota.  
4. Zaprojektuj mechanizm wykrywania anomalii, który odróżnia awarię sensora od ataku na sensor.  
5. Wyjaśnij, dlaczego ROS2 DDS Security nie jest wystarczający w środowisku przemysłowym o wysokiej krytyczności.

---

# 🐜 **Robotyka roju — trudne pytania egzaminacyjne**

1. Wyjaśnij, jak odróżnić zachowanie emergentne od niestabilności systemu. Podaj formalne kryteria.  
2. Zaprojektuj algorytm unikania kolizji w roju, który nie wymaga komunikacji radiowej. Jakie są jego ograniczenia?  
3. Wyjaśnij, dlaczego rój może działać poprawnie mimo awarii 30% agentów, ale całkowicie zawodzi przy awarii 5% *konkretnych* agentów.  
4. Zaproponuj metodę synchronizacji czasu w roju bez centralnego zegara i bez komunikacji globalnej.  
5. Wyjaśnij, dlaczego skalowanie algorytmów roju z 50 do 5000 agentów wymaga zmiany architektury komunikacji.

---

# ⚙️ **RTOS i systemy czasu rzeczywistego — trudne pytania egzaminacyjne**

1. Wyjaśnij, dlaczego analiza WCET jest trudniejsza na nowoczesnych MCU niż na starszych architekturach.  
2. Zaprojektuj system, w którym task niskiego priorytetu *nie może* spowodować opóźnienia tasku wysokiego priorytetu — nawet pośrednio.  
3. Podaj przykład, w którym priority inversion nie występuje mimo użycia wielu mutexów.  
4. Wyjaśnij, dlaczego dynamiczna alokacja pamięci może być bezpieczna w RTOS — pod warunkiem spełnienia określonych zasad.  
5. Zaprojektuj mechanizm degradacji kontrolowanej w robocie, który traci 40% mocy obliczeniowej w trakcie działania.

---

