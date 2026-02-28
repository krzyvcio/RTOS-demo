# Wykład 0: Mapa systemu i architektura wirtualnego laboratorium

## Część I: Wstęp teoretyczny - Geneza problemu

### Dlaczego potrzebujemy wirtualnego laboratorium?

Współczesne systemy mechatroniczne, szczególnie te związane z wirującymi maszynami, osiągają poziom złożoności, który czyni tradycyjne podejście "najpierw prototyp, potem testy" nie tylko nieefektywnym, ale wręcz niebezpiecznym. Wyobraźmy sobie następujący scenariusz: zespół inżynierów przez sześć miesięcy buduje sterowanie dla systemu wirującego, który ma pracować z prędkością 30 000 RPM. Pierwsze uruchomienie na prawdziwym sprzęcie ujawnia rezonans, o którym nikt nie pomyślał - wirnik wchodzi w niebezpieczne oscylacje i uszkadza łożyska. Koszt: nie tylko naprawa mechaniki, ale również utrata zaufania do całego cyklu rozwojowego.

To nie jest hipotetyczny problem - to codzienność w przemyśle. Statystyki pokazują, że około 40% projektów mechatronicznych przekracza budżet z powodu problemów odkrytych zbyt późno w cyklu rozwojowym. Kolejne 25% doświadcza opóźnień wynikających z nieprzewidywalnych interakcji między sterowaniem, mechaniką i warstwą komunikacyjną.

Wirtualne laboratorium (Digital Twin Lab) to odpowiedź na te wyzwania. Pojęcie "cyfrowego bliźniaka" zostało wprowadzone przez NASA w latach 60. XX wieku jako koncepcja symulacji fizycznych obiektów w czasie rzeczywistym. W kontekście systemów wirujących oznacza to możliwość przetestowania całego łańcucha - od modelu fizycznego, przez algorytmy sterowania, po zachowanie w warunkach awaryjnych - zanim jakikolwiek fizyczny element zostanie uruchomiony.

### Przemówienie Profesora

Siedzę w tym zawodzie od ponad trzydziestu lat i widziałem wiele przejść od "naprawdę wiem, co robię" do "kurczę, dlaczego to nie działa?". Najgorsze jest to, że te drugie momenty zwykle przychodzą najdrożej - na stole monterskim, z gotowym sprzętem, pod presją terminu.

Pamiętam projekt z końca lat 90., gdzie budowaliśmy sterowanie dla wirówki przemysłowej. Zespół był świetny - siedmiu doktorantów, wszystkie symulacje wyglądały idealnie. Pierwsze uruchomienie: wirnik osiągnął 8000 RPM i zaczął zachowywać się jakby miał własną wolę. Okazało się, że nikt nie uwzględnił sprzężenia zwrotnego od podatności łożyskowania w modelu. Prosty błąd, ale kosztował nas trzy miesiące opóźnienia i nieprzespane noce.

Gdybyśmy wtedy mieli dzisiejsze narzędzia - możliwość wirtualnego laboratorium, gdzie testujesz nie tylko sterowanie, ale całą architekturę - oszczędzilibyśmy mnóstwo czasu i nerwów. Dlatego właśnie ten wykład jest taki ważny. Bo zanim zaczniemy mówić o równaniach i kodzie, musimy zrozumieć, DLACZEGO budujemy to laboratorium i JAKIE problemy ma rozwiązywać.

Nie chodzi o to, żeby zastąpić myślenie inżynierskie symulacją. Chodzi o to, żeby symulacja była pierwszą linią obrony przed głupimi błędami, a prawdziwy sprzęt - ostatnią.

## Cel
Ustawić wspólny obraz całości:
- jakie bloki składają się na system wirujący,
- gdzie są granice odpowiedzialności (model, sterowanie, RT, magistrala),
- jak wygląda architektura „Digital Twin Lab” krok po kroku.

> TL;DR: Najpierw powstaje model i harness, potem RT i pomiary, dopiero na końcu integracja z magistralą i HIL.

## Mapa bloków systemu
System wirujący w ujęciu inżynierskim to zwykle:
- obiekt (wirnik + łożyskowanie + konstrukcja),
- napęd (falownik + silnik + sensory),
- sterowanie (prąd/moment, prędkość, tryby),
- diagnostyka (wibracje, termika, saturacje),
- bezpieczeństwo (watchdog, limity, safe stop),
- komunikacja (magistrala, synchronizacja czasu).

### Perspektywa systemowa - dlaczego ta mapa jest kluczowa

Każdy z tych bloków reprezentuje odrębną domenę inżynierską, która rozwinęła się jako samodzielna specjalność. Mechanik konstruktor myśli w kategoriach sztywności, masy i rezonansów. Specjalista od napędów operuje pojęciami momentu, prądu i charakterystyk falownika. Inżynier sterowania mówi językiem pętli sprzężenia zwrotnego, całkowania i odpowiedzi czasowej. Każdy z nich ma własne narzędzia, własne metryki sukcesu i własne frustracje.

Problem zaczyna się w momencie, gdy te światy muszą ze sobą rozmawiać. Bo okazuje się, że rezonans mechaniczny, który mechanik przewidział w FEA, ujawnia się zupełnie inaczej w obecności opóźnień komunikacyjnych, o których wie tylko specjalista od sieci. Albo że charakterystyka termiczna silnika, którą napędowiec zmierzył w laboratorium, zmienia się diametralnie gdy przesunie się punkt pracy z powodu nieliniowości regulatora prędkości, o którym wie tylko sterownicowiec.

Wirtualne laboratorium jest właśnie po to, żeby te interakcje odkrywać wcześniej, taniej i bezpieczniej.

### Przemówienie Profesora

Kiedy mówię studentom o mapie bloków, zawsze proszę ich o jedno: wyjdźcie z swojej strefy komfortu. Jeśli jesteś świetny w modelowaniu mechanicznym, porozmawiaj z kimś, kto siedzi w sterowaniu. Jeśli znasz się na komunikacji przemysłowej, zapytaj mechanika, co oznacza "krytyczna prędkość wirnika".

Najlepsze zespoły projektowe, które widziałem, miały jedną cechę wspólną: ludzie rozumieli nie tylko swój kawałek puzzli, ale potrafili powiedzieć, jak ten kawałek współgra z sąsiednimi. I wiedzieli, gdzie są granice - czyli gdzie ich kompetencja się kończy i gdzie potrzebują wsparcia.

To laboratorium jest świetnym treningiem tej umiejętności. Bo tutaj będziecie musieli połączyć wszystkie te światy w działający system - najpierw wirtualnie, potem może na prawdziwym sprzęcie.

## Architektura wirtualnego laboratorium (wariant bazowy)
Schemat, który działa w praktyce:
```text
[Model ODE/MDOF na PC]
        ↓ sprzężenie zwrotne (symulowane pomiary)
[Kontroler RT (Linux PREEMPT_RT albo RTOS)]
        ↓ magistrala (EtherCAT/CAN/UDP/UART)
[Warstwa I/O / MCU (opcjonalnie w HIL)]
        ↓
[Model obiektu / „plant"]
```

### Dlaczego ta architektura ma sens

Ta pozornie prosta struktura kryje w sobie lata doświadczeń i setki projektów, które skończyły się źle. Pozwólcie, że wyjaśnię, dlaczego elementy są ułożone właśnie w tej kolejności.

Model na PC to "prawa fizyki" - najprostsze równania, które opisują zachowanie obiektu. Zaczynamy od najprostszego modelu, bo chcemy szybko mieć coś, co działa i na czym możemy testować sterowanie. Model nie musi być perfect - musi być wystarczająco dobry, żeby wychwycić podstawowe problemy.

Kontroler RT to "inteligencja" - tutaj dzieje się sterowanie. Linux PREEMPT_RT lub RTOS to nie są wymienialne warstwy, to fundamentalnie różne filozofie. PREEMPT_RT daje nam elastyczność Linuxa z przewidywalnością czasową. RTOS (FreeRTOS, Zephyr) daje determinizm kosztem bogactwa bibliotek. Wybór zależy od projektu - ale o tym powiemy więcej w późniejszych wykładach.

Magistrala to "nerw" systemu - miejsce, gdzie pojawiają się opóźnienia, jitter i dropouty. To najtrudniejsza do zamodelowania warstwa, bo zależy od hardware'u, obciążenia sieci i dziesiątek innych czynników. Dlatego zaczynamy od symulacji, a dopiero potem przechodzimy do realnej magistrali.

Warstwa I/O/MCU to "zmysły" - interfejs między światem cyfrowym a analogowym. Przetwarzanie ADC, enkodery, PWM - to wszystko żyje w tej warstwie i ma swoje ograniczenia, które trzeba rozumieć.

Interpretacja:
- "plant" może być tylko procesem na PC (SIL), albo osobnym modułem (HIL),
- kontroler może być procesem RT na PC lub firmware na MCU,
- magistrala może być realna (hardware) albo symulowana.

## Typowe etapy budowy (w kolejności, która minimalizuje ryzyko)
1. Model minimalny ODE (prędkość + zakłócenia) + testy jednostkowe.
2. Symulacja pętli sterowania (PI/PID, saturacje, anti-windup) w czasie dyskretnym.
3. Pomiar deterministyki (jitter/WCRT) w pętli RT (Linux PREEMPT_RT lub RTOS).
4. Dodanie diagnostyki (FFT, piki, trendy, baseline).
5. Integracja magistrali (EtherCAT/CAN) i testy obciążeniowe.
6. HIL: MCU w pętli (PC symuluje plant, MCU myśli, że steruje realnym obiektem).

### Dlaczego ta kolejność ma znaczenie

Każdy etap buduje na poprzednim i jednocześnie weryfikuje założenia. Zaczynamy od modelu, bo to najtańszy sposób na weryfikację, czy nasze rozumienie fizyki jest poprawne. Jeśli sterowanie nie działa na prostym modelu ODE, nie ma sensu iść dalej.

Etap drugi dodaje "higienę" cyfrowego sterowania - dyskretyzację, saturacje, anti-windup. To są problemy, które zawsze ujawniają się na prawdziwym sprzęcie, więc lepiej je zobaczyć wcześniej.

Etap trzeci to "testosterone check" - sprawdzenie, czy nasze środowisko czasu rzeczywistego jest naprawdę deterministyczne. Bo jeśli jitter jest większy niż założenia w etapie drugim, to cała nasza analiza stabilności jest bezwartościowa.

Diagnostyka to etap, który zwykle jest odkładany "na później" - błąd. Bo jeśli nie mierzysz tego, co się dzieje, to nie wiesz, czy system działa poprawnie. A jeśli nie wiesz, to nie możesz debugować.

Integracja magistrali to moment, gdy "papierowe" założenia spotykają rzeczywistość. I zwykle okazuje się, że rzeczywistość jest bardziej skomplikowana.

HIL to ostateczny test - ale uwaga, nie jest to "ostateczny boss" przed prawdziwym sprzętem. HIL ma swoje ograniczenia i nie zastępuje testów na realnym systemie. Ale pozwala wychwycić większość problemów firmware'u i integracji.

### Przemówienie Profesora

Ta kolejność jest wypadkową tysięcy projektów, które widziałem. Oczywiście, zawsze znajdzie się ktoś, kto powie "u mnie to było inaczej" - i pewnie miał rację, dla swojego specyficznego przypadku. Ale jeśli jesteście na początku drogi, zaufajcie tej sekwencji. Jest zbudowana na błędach pokoleń inżynierów.

Pamiętajcie: każdy etap może się "wywalić". I to jest OK - to jest właśnie cel tego laboratorium. Lepiej, żeby model się wywalił, niż żeby wirnik się wywalił.

Jedna uwaga na koniec: wielu studentów chce zacząć od "fajnych" rzeczy - od HIL, od EtherCAT, od wizualizacji. Rozumiem tę pokusę. Ale proszę was: zacznijcie od podstaw. Od prostego modelu ODE, od PI/PID, od logowania. Bo jak tego nie umiecie, to cała reszta jest tylko dekoracją.

## Kompetencje, które muszą powstać w zespole
- modelowanie ODE i podstawy MDOF,
- analiza częstotliwości (FFT, widma, piki),
- sterowanie dyskretne i filtracja,
- pomiary czasu rzeczywistego (WCRT, jitter),
- synchronizacja czasu i timestamping,
- praktyka integracji (magistrala + watchdog + degradacja).

### Dlaczego te kompetencje są kluczowe

Te sześć obszarów to nie jest przypadkowa lista - to minimalny zestaw umiejętności, żeby nie kaleczyć systemu wirującego. Pozwólcie, że krótko wyjaśnię każdy:

Modelowanie ODE/MDOF to podstawa - bez tego nie wiesz, czym sterujesz. Nie mówię tu o doktoracie z mechaniki, ale o umiejętności zapisania równania ruchu i rozwiązania go numerycznie. To pozwala też rozmawiać z mechanikami w ich języku.

Analiza częstotliwości to oczy - bez FFT jesteś ślepy na to, co dzieje się w systemie. Rezonanse, zakłócenia, problemy z filteracją - wszystko to widać w dziedzinie częstotliwości. I zanim powiecie "to robi dział R&D" - powiem wam, że w laboratorium chcę, żebyście widzieli, co się dzieje, a nie żebyście wierzyli w raporty.

Sterowanie dyskretne to trzon - PI/PID, anti-windup, filtry - to są narzędzia, bez których ani rusz. I nie chodzi o to, żeby wymyślać nowe algorytmy, ale żeby rozumieć, dlaczego te istniejące czasem nie działają.

Pomiary RT to sprawdzenie, czy obietnice są dotrzymane. Bo możesz mieć najlepszy algorytm na świecie, ale jeśli czas wykonania przekracza deadline, to cała analiza stabilności jest bezwartościowa.

Synchronizacja czasu to nerw systemu rozproszonego - współczesne systemy wirujące to zwykle wiele węzłów, które muszą ze sobą rozmawiać w określonym czasie. Timestampy, PTP, synchronizacja - to nie jest "tylko dla sieciowców".

Integracja to final boss - bo wszystko, co działa oddzielnie, musi razem zadziałać. I tu pojawiają się problemy, których nikt nie przewidział - bo nikt nie myślał o całości.

### Przemówienie Profesora

Kiedy rekrutuję nowych inżynierów, pytam ich zwykle o jeden problem: co zrobić, gdy regulator zaczyna oscylować? Większość odpowiada "dostroić PID" - i to jest poprawna odpowiedź, ale niepełna. Bo prawdziwe pytanie brzmi: skąd wiesz, że to regulator, a nie rezonans mechaniczny? Skąd wiesz, że to nie jitter komunikacyjny? Skąd wiesz, że to nie szum pomiarowy?

I wtedy rozumiem, czy ktoś ma te kompetencje, czy tylko jedną z nich.

W tym laboratorium będziecie budować system, który wymaga wszystkich tych umiejętności. Nie musicie być ekspertami w każdej dziedzinie - ale musicie rozumieć, jak te dziedziny się przeplatają. Bo inżynieria to nie zbiór wysp, to archipelag, gdzie mosty są tak samo ważne jak wyspy.

## Checklisty
- Masz jeden diagram architektury z podpisanymi opóźnieniami.
- Masz zdefiniowane metryki: błąd regulacji, jitter, saturacje, wibracje, temperatura.
- Masz plan degradacji: co robisz, gdy RT/magistrala nie wyrabia.

### Szczegółowa lista kontrolna dla pierwszego spotkania

Zanim wyjdziecie z tego laboratorium po raz pierwszy, upewnijcie się, że macie:

1. **Diagram architektury** - nie konceptualny szkic, ale dokument, który pokazuje:
   - Jakie procesy/wątki tworzą system
   - Jakie są interfejsy między nimi
   - Jakie są oczekiwane opóźnienia na każdym połączeniu
   - Kto jest "single point of truth" dla czasu

2. **Metryki jakości** - zdefiniowane i mierzalne:
   - Błąd regulacji (steady-state, przejściowy, maksymalny)
   - Jitter pętli (p50, p95, p99)
   - Saturacje (ile razy, jak długo, jak system reaguje)
   - Wibracje (RMS, piki widma)
   - Temperatura (model termiczny, progi alarmowe)

3. **Plan degradacji** - co robi system, gdy coś nie działa:
   - Timeout komunikacji → tryb bezpieczny
   - Przekroczenie jittera → fallback do wolniejszego sterowania
   - Awaria sensora → estymacja lub stop
   - Każdy scenariusz ma zdefiniowany stan bezpieczny

### Przemówienie Profesora

Checklisty to nie biurokracja - to narzędzie przetrwania. Widziałem projekty, które "mialy wszystko w głowie" - i widziałem, jak te projekty umierały w momencie, gdy kluczowa osoba zachorowała albo odeszła.

Proszę was: dokumentujcie. Nie dla mnie, nie dla audytorów - dla siebie. Za trzy miesiące nie będziecie pamiętać, dlaczego ten parametr ma taką wartość. Za rok nie będziecie pamiętać, dlaczego ta architektura była lepsza od innej. Dokumentacja to wasza dźwigara pamięci.

A plan degradacji to wasza polisa ubezpieczeniowa. Bo prędzej czy później coś pójdzie nie tak. Pytanie nie jest "czy", tylko "kiedy" i "jak szybko będziecie z powrotem". Jeśli macie plan degradacji, to "kiedy" jest tylko inconvenience. Bez planu - to katastrofa.

## Slajdy (tekstowe)
### Slajd 1: Co budujemy
- Wirtualne laboratorium do testów systemu wirującego przed hardware
- Neutralny kontekst, praktyka RT i sterowania

### Slajd 2: Bloki systemu
- Plant (wirnik, konstrukcja)
- Drive (moment/prąd)
- Kontroler (pętle i tryby)
- Diagnostyka i safety
- Magistrala i czas

### Slajd 3: Kolejność prac
- Model -> sterowanie -> RT pomiary -> magistrala -> HIL

## Pytania do studentów
1. Który blok w architekturze labu jest „single point of truth” dla czasu i dlaczego?
2. Jakie 5 metryk uznasz za obowiązkowe, żeby lab był narzędziem inżynierskim, a nie demo?
3. Jak rozdzielisz odpowiedzialności między plant, kontroler, magistralę i diagnostykę, aby łatwo debugować?
4. Jakie scenariusze awaryjne muszą być testowane od początku (nie „na końcu projektu”)?

## Projekty studenckie
- „Architektura referencyjna labu”: repo z diagramem, kontraktami danych i minimalnym harness.
- „Scenario pack”: zestaw scenariuszy (zakłócenie, saturacja, opóźnienie, dropout) + raport metryk.
- „Metrics spec”: specyfikacja logów i walidator kompletności danych (wersjonowanie schematu).

## BONUS
- Największy zwrot daje wczesne wprowadzenie „replay”: jeśli potrafisz odtwarzać ten sam scenariusz 1:1, to każda zmiana algorytmu jest mierzalna.
