# Wykład 01

### 1. Tytuł wykładu
Czym jest robotyka dziś? Definicje, klasy robotów, metryki skuteczności

### 2. Cele nauczania (5–7 punktów)
- Student zrozumie, czym różni się „robot” od „automatu” i „systemu mechatronicznego” w sensie funkcjonalnym i regulacyjnym.
- Student pozna podstawowe klasy robotów (manipulatory, roboty mobilne, humanoidy, roboty współpracujące, roje) oraz typowe zastosowania.
- Student będzie potrafił zdefiniować i obliczyć kluczowe metryki skuteczności: dokładność, powtarzalność, czas cyklu, przepustowość, dostępność, niezawodność, zużycie energii.
- Student będzie potrafił dobrać metryki do celu aplikacji (np. produkcja, logistyka, medycyna, badania) i wskazać kompromisy projektowe.
- Student zrozumie rolę bezpieczeństwa (normy, analiza ryzyka) jako ograniczenia projektowego, a nie „dodatek na końcu”.
- Student pozna aktualne (2024–2025) trendy w robotyce (Physical AI, modele fundamentalne, cyfrowe bliźniaki, humanoidy) i logiczną ekstrapolację wyzwań do 2030.

### 3. Wstęp teoretyczny
#### 3.1. Robot: definicja operacyjna i granice pojęcia
W praktyce akademickiej i przemysłowej „robot” bywa rozumiany intuicyjnie: jako „maszyna wykonująca zadania zamiast człowieka”. Taka definicja jest jednak zbyt szeroka (obejmuje np. proste automaty) i zbyt wąska (pomija systemy, które nie „zastępują” człowieka, lecz rozszerzają jego możliwości). Dlatego na potrzeby kursu przyjmijmy **definicję operacyjną**:

> Robot to system mechatroniczny zdolny do postrzegania (pomiaru stanu własnego i/lub otoczenia), podejmowania decyzji (regułowo lub adaptacyjnie) oraz oddziaływania na świat (aktuacja), przy czym działanie to zachodzi w pętli sprzężenia zwrotnego i jest realizowane w warunkach niepewności.

Ta definicja podkreśla trzy filary: **percepcję**, **decyzję** i **aktuację**. W odróżnieniu od klasycznego automatu sekwencyjnego, robot:
1) operuje w środowisku zmiennym (niepewność),  
2) często wymaga planowania w przestrzeni ciągłej (ruch w \(SE(3)\)),  
3) może pracować blisko człowieka (wymogi bezpieczeństwa),  
4) musi integrować wiele podsystemów (mechanika, elektronika, sterowanie, informatyka).

Wymiar regulacyjny jest równie istotny: w przemyśle definicje i klasyfikacje robotów są powiązane z normami bezpieczeństwa (np. ISO 10218) i wymaganiami stanowiska pracy. Z perspektywy inżyniera oznacza to, że „co to jest robot” nie jest wyłącznie pytaniem filozoficznym, ale wpływa na dobór komponentów, certyfikację, procedury odbioru i odpowiedzialność.

Warto też podkreślić praktyczny aspekt „decyzji”: nie musi to być od razu sztuczna inteligencja. Reguły (np. automaty stanów) są w wielu systemach dominujące, ale jeśli robot ma działać w niepewnym środowisku, to i tak w tle pojawia się **estymacja stanu** oraz **sterowanie ze sprzężeniem zwrotnym**. Właśnie ten „zestaw minimalny” odróżnia robotykę od prostego sterowania sekwencyjnego.

#### 3.2. Klasy robotów i typowe zastosowania (mapa terenu)
W pierwszym przybliżeniu wyróżnimy pięć klas, które będą przewijać się przez cały kurs:

1) **Manipulatory stacjonarne** (ramiona 4–7 osi): spawanie, montaż, paletyzacja, obsługa maszyn, laboratoria.  
2) **Roboty mobilne** (AMR/AGV, drony, roboty kołowe/gąsienicowe): logistyka, inspekcja, mapowanie, ratownictwo.  
3) **Roboty współpracujące (coboty)**: praca obok człowieka, wspomaganie montażu, testy, małe serie.  
4) **Roboty humanoidalne i układy whole-body**: lokomocja + manipulacja w środowisku człowieka, potencjał dla logistyki i usług do 2030.  
5) **Systemy wielorobotowe (zespoły/roje)**: rozproszone mapowanie, magazyny, rolnictwo, misje krytyczne.

Powyższy podział jest celowo „funkcjonalny”. Te same elementy matematyczne (np. opis ruchu w \(SE(3)\)) pojawiają się w wielu klasach, ale ich **wagi** w projekcie są różne. Manipulator „wybaczy” brak mapy otoczenia, ale nie wybaczy błędów kinematyki i kalibracji; robot mobilny odwrotnie.

Tabela 1 porządkuje różnice w priorytetach projektowych.

| Klasa robota | Dominująca trudność | Kluczowe metryki | Typowe czujniki | Typowe ryzyka |
|---|---|---|---|---|
| Manipulator | kinematyka/dynamika, kolizje | dokładność, powtarzalność, czas cyklu | enkodery, czujniki siły/momentu, kamera | zgniecenie, kolizje, błąd trajektorii |
| Mobilny AMR | nawigacja, niepewność mapy | bezpieczeństwo, dostępność, autonomia energetyczna | LiDAR, IMU, kamera | potrącenie, błędna lokalizacja |
| Cobot | współpraca człowiek–robot | bezpieczeństwo kontaktu, ergonomia | momenty, wizyjne, strefy bezpieczeństwa | uraz, błędna interpretacja intencji |
| Humanoid | kontakt, równowaga, energia | koszt energii, stabilność, niezawodność | IMU, kamery, siłomierze stóp, LiDAR | upadek, uszkodzenie otoczenia |
| Rój | koordynacja, komunikacja | skalowalność, odporność | zależnie od platformy | utrata łączności, zachowania emergentne |

#### 3.3. Metryki skuteczności: co mierzymy i dlaczego
Robotyka jest dziedziną „mostową”: łączy elegancję modeli matematycznych z brutalnością realnego świata. Dlatego w praktyce ocenia się nie tyle „czy algorytm działa”, ile **jak działa w metrykach, które są istotne dla zadania**. Te metryki są częścią specyfikacji inżynierskiej: stanowią „język porozumienia” między projektantem, użytkownikiem, działem bezpieczeństwa i utrzymania ruchu.

W tym wykładzie wprowadzamy metryki na poziomie ogólnym, ale od razu z intencją, że będą one „przyczepiane” do formalnych modeli w kolejnych wykładach (kinematyka, dynamika, planowanie, percepcja).

##### 3.3.1. Dokładność i powtarzalność

W metrologii robotów rozróżnia się:

- **dokładność** (accuracy): jak blisko robot trafia w cel absolutny,
- **powtarzalność** (repeatability): jak bardzo powtarzalne są wyniki dla powtarzanego zadania.

Niech \(\mathbf{p}^\*\in\mathbb{R}^3\) będzie punktem zadanym, a \(\mathbf{p}_i\) punktami osiągniętymi w \(N\) powtórzeniach. Definiujemy błąd:
\[
\mathbf{e}_i = \mathbf{p}_i - \mathbf{p}^\*,
\qquad
e_i = \|\mathbf{e}_i\|_2.
\]

Jedna z prostych miar dokładności to średnia norma błędu:
\[
\bar e = \frac{1}{N}\sum_{i=1}^N e_i.
\]
Powtarzalność można opisać odchyleniem standardowym błędów:
\[
\sigma_e = \sqrt{\frac{1}{N-1}\sum_{i=1}^N (e_i-\bar e)^2}.
\]
W interpretacji inżynierskiej często używa się też promienia \(3\sigma_e\) jako „typowego” rozrzutu, jeśli rozkład błędu jest zbliżony do normalnego.

Ważna obserwacja: **robot może być bardzo powtarzalny, a jednocześnie niedokładny** (np. stały błąd kalibracji). To prowadzi do dwóch różnych strategii doskonalenia:
- poprawa **kalibracji** i modelu geometrycznego (dla dokładności),
- poprawa **sztywności, sterowania i jakości czujników** (dla powtarzalności).

##### 3.3.2. Czas cyklu i przepustowość
Dla zadań przemysłowych kluczowy jest czas cyklu:
\[
T_c = T_{\text{dojazd}} + T_{\text{chwyt}} + T_{\text{weryfikacja}} + T_{\text{operacja}} + T_{\text{odjazd}} + T_{\text{bufory}}.
\]
W praktyce łatwo wpaść w pułapkę: maksymalna prędkość osi robota (z katalogu) ma mniejsze znaczenie niż „otoczenie procesu”: chwytak, podajnik, jakość części, oświetlenie, czas wizyjnej detekcji, narzucone ograniczenia bezpieczeństwa oraz synchronizacja z innymi maszynami.

Przepustowość (szt./h) w prostym modelu:
\[
Q = \frac{3600}{T_c}\cdot \eta,
\]
gdzie \(\eta\in(0,1]\) uwzględnia straty (mikroprzestoje, odrzuty, oczekiwanie). Już tu widać kluczowe napięcie projektowe: zbyt agresywna optymalizacja czasu cyklu może obniżać jakość (wzrost odrzutów), zwiększać ryzyko (bezpieczeństwo) lub pogarszać zużycie energii.

##### 3.3.3. Dostępność i niezawodność (perspektywa utrzymania ruchu)
W systemach przemysłowych często spotkasz pojęcia MTBF (średni czas między awariami) i MTTR (średni czas naprawy). Prosta aproksymacja dostępności:
\[
A \approx \frac{\text{MTBF}}{\text{MTBF}+\text{MTTR}}.
\]
Z punktu widzenia eksploatacji dostępność jest często bardziej „prawdziwa” niż laboratoryjna dokładność: robot, który w katalogu ma świetne parametry, ale w praktyce wymaga częstych resetów lub serwisu, nie spełni oczekiwań.

W robotyce mobilnej „dostępność” splata się z problemem ładowania, degradacji baterii, kondycji czujników i warunków środowiskowych (kurz, wibracje, oświetlenie). Do 2030 rosnąć będzie znaczenie metryk kosztu całkowitego posiadania (TCO) oraz „dowodu bezpieczeństwa” dla systemów adaptacyjnych.

##### 3.3.4. Zużycie energii jako metryka pierwszego rzędu
W robotach mobilnych i humanoidach energia jest często ograniczeniem dominującym, a w robotach przemysłowych coraz częściej staje się elementem kosztu i polityki środowiskowej. Dla zadania o czasie \(T\) i mocy chwilowej \(P(t)\) energia:
\[
E = \int_0^T P(t)\,dt.
\]
W praktyce porównuje się zużycie energii „na zadanie” (np. J/szt.) lub „na metr” (J/m) w robotach mobilnych. Metryka ta jest krytyczna, gdy:
- robot działa długo bez przerw (magazyn, patrol),
- masa jest duża (humanoidy, ciężkie chwytaki),
- wymagane są szybkie przyspieszenia (profil ruchu),
- liczy się dyskretna infrastruktura ładowania i jej przepustowość.

Ten kurs będzie wracał do energii wielokrotnie: od prostych bilansów (Część 1) po zaawansowane techniki projektowania aktuatorów i odzysku energii (Część 2).

#### 3.4. Bezpieczeństwo: ograniczenie projektowe i język norm
W robotyce bezpieczeństwo nie jest „opcją”, lecz elementem definicji poprawnego działania. Analiza ryzyka obejmuje zwykle:
1) identyfikację zagrożeń (mechaniczne, elektryczne, programowe, środowiskowe),  
2) oszacowanie ryzyka (prawdopodobieństwo \(\times\) skutek),  
3) dobór środków redukcji ryzyka (technicznych i organizacyjnych),  
4) walidację oraz dokumentację.

W dydaktyce użyteczny jest uproszczony model minimalnej odległości bezpieczeństwa (zgodny z intuicją norm bezpieczeństwa maszyn):
\[
S = K\cdot T + C,
\]
gdzie:
- \(K\) — prędkość zbliżania (np. człowieka lub zagrożenia) [mm/s],
- \(T\) — czas reakcji systemu (detekcja + zatrzymanie) [s],
- \(C\) — stała uwzględniająca dodatkowe czynniki (np. penetrację strefy, tolerancje) [mm].

Model jest prosty, ale ujawnia mechanizm kompromisu: skrócenie \(T\) wymaga lepszej detekcji i szybszego hamowania (mechanika + sterowanie), a zmniejszenie \(C\) bywa ograniczone geometrią i tolerancjami. W robotach współpracujących (cobotach) dochodzi ocena bezpieczeństwa kontaktu i ograniczanie sił/momentów. W praktyce inżynier uczy się „tłumaczyć” wymagania norm na parametry sterowania (limity prędkości, momentu), konstrukcji (zaokrąglenia, osłony) i percepcji (strefy bezpieczeństwa).

#### 3.5. Od klasycznej robotyki do Physical AI (trend 2024–2025)
Robotyka klasyczna opiera się na czterech filarach: modelowaniu (kinematyka, dynamika), planowaniu, sterowaniu i estymacji (np. SLAM). Od 2023 roku obserwujemy intensyfikację podejść opartych o **uczenie maszynowe w skali**, w szczególności modeli łączących język, obraz i działanie (VLA — vision-language-action) oraz dużych zbiorów danych z realnych robotów. Pojęcie *Physical AI* akcentuje, że „inteligencja” robota jest bezwzględnie ograniczona prawami fizyki: tarciem, bezwładnością, opóźnieniami, sztywnością chwytaka, kontaktami i niepewnością pomiaru.

Kluczowy wniosek dla kursu jest konserwatywny, ale praktyczny: do 2030 nie znikną podstawy (kinematyka, dynamika, stabilność), lecz staną się **warstwą gwarancji** i „bezpieczników”, na którą będą nakładane komponenty uczone (lepsza percepcja, lepsza generalizacja, lepsze priorytetyzowanie celów). Wykłady Części 2 będą konsekwentnie wracały do pytania: *jak połączyć adaptacyjność z weryfikowalnością?*

#### 3.6. Prognoza do 2030: osie zmian i konsekwencje dla inżyniera
Na podstawie trendów 2024–2025 można wskazać trzy osie rozwoju do 2030:

1) **Skalowanie danych i modeli**: większe zbiory danych ucieleśnionych, lepsze uogólnianie między robotami i zadaniami. Konsekwencja: rośnie rola inżynierii danych, oceny jakości danych i metryk uogólniania.  
2) **Integracja symulacji i rzeczywistości (cyfrowe bliźniaki)**: symulacja przestaje być tylko narzędziem do testów, staje się elementem eksploatacji i utrzymania ruchu. Konsekwencja: rośnie znaczenie identyfikacji, walidacji i zarządzania konfiguracją.  
3) **Regulacje i zaufanie społeczne**: systemy adaptacyjne będą podlegały coraz ostrzejszym wymaganiom audytu, bezpieczeństwa i odpowiedzialności. Konsekwencja: w programie kursu muszą pojawić się ramy ryzyka, dokumentacja, testy i śledzenie decyzji.

Robotyka 2030 jest zatem dziedziną „pełnego stosu”: od śrub i przekładni, przez równania ruchu, aż po ocenę ryzyka i wpływ społeczno-rynkowy.

**Uwaga o źródłach:** w tekście odwołujemy się do fundamentów (podręczniki i normy) oraz do reprezentatywnych publikacji z lat 2023–2025 dotyczących modeli ucieleśnionych i trendów branżowych. Pełna lista znajduje się w sekcji 8.

#### 3.7. Metryki percepcji i estymacji: od „dokładnego czujnika” do „wiarygodnej decyzji”
W potocznym rozumieniu „percepcja” kojarzy się z kamerą i rozpoznawaniem obiektów. W robotyce jest to zbyt wąskie: percepcja to przede wszystkim **model pomiaru** i jego niepewność. Ogólny zapis (w czasie dyskretnym) wygląda następująco:
\[
\mathbf{z}_k = h(\mathbf{x}_k) + \mathbf{v}_k,
\qquad
\mathbf{v}_k \sim \mathcal{N}(\mathbf{0},\mathbf{R}_k),
\]
gdzie \(\mathbf{x}_k\) to stan (np. położenie, prędkość, orientacja), \(\mathbf{z}_k\) to pomiar, \(h(\cdot)\) to model czujnika, a \(\mathbf{R}_k\) opisuje szum i niepewność. Nawet jeśli „obraz jest ostry”, to wciąż musimy odpowiedzieć na pytania: *jak pewna jest detekcja? jak błąd pomiaru wpływa na sterowanie? czy robot potrafi rozpoznać sytuację, w której nie wie?*

Przykładowe metryki percepcji i estymacji, które pojawią się później (Wykłady 10–11), obejmują:
- **metryki detekcji/klasyfikacji** (precyzja, czułość, \(F_1\)) — ważne np. dla detekcji obiektów i przeszkód,
- **metryki estymacji trajektorii** (np. ATE/RPE w SLAM) — ważne dla robotów mobilnych,
- **miary niepewności** (kowariancja, elipsy błędu) — ważne, gdy planowanie i bezpieczeństwo zależą od „pewności lokalizacji”.

W ujęciu probabilistycznym interesuje nas nie tylko „najlepszy punktowy szacunek”, ale całe rozkłady:
\[
p(\mathbf{x}_k\mid \mathbf{z}_{1:k}) \propto p(\mathbf{z}_k\mid \mathbf{x}_k)\,p(\mathbf{x}_k\mid \mathbf{z}_{1:k-1}).
\]
To formalne równanie (w duchu filtracji Bayesowskiej) ma praktyczne konsekwencje: jeśli czujniki zawodzą (oślepienie kamery, degradacja LiDAR, poślizg kół), to *niepewność rośnie* i system powinien przełączać tryby pracy (np. zwolnić, zatrzymać się, poprosić o interwencję). Takie zachowanie jest elementem inżynierskiej definicji „bezpieczeństwa” i „rozsądku” robota (por. Corke, 2017; Siciliano i Khatib, 2016).

#### 3.8. Metryki bezpieczeństwa i ryzyka: od macierzy ryzyka do oczekiwanej straty
Bezpieczeństwo w robotyce wymaga języka, który łączy technikę z oceną skutków. Najprostsza dydaktyczna postać ryzyka to iloczyn:
\[
R = P \cdot S,
\]
gdzie \(P\) jest miarą prawdopodobieństwa zdarzenia niebezpiecznego, a \(S\) — miarą dotkliwości skutków. W praktyce normy i procedury używają kategorii (np. skale 1–5), a nie liczb ciągłych, ale idea jest ta sama: **nie da się projektować robota bez jawnego kompromisu między prawdopodobieństwem a skutkiem** (ISO 10218; ISO/TS 15066).

W systemach adaptacyjnych (uczących się) do 2030 coraz częściej spotkasz podejście, które można zapisać jako minimalizację oczekiwanej straty:
\[
\min_{\pi}\ \mathbb{E}\big[L(\mathbf{x},\mathbf{u},\text{zdarzenia})\big],
\]
gdzie \(\pi\) to polityka sterowania (regułowa lub uczona), \(\mathbf{u}\) to sterowanie, a \(L\) może silnie karać zdarzenia niebezpieczne. Teza dydaktyczna jest prosta: nawet jeśli narzędzia się zmieniają (od PID do polityk uczonych), projektant zawsze musi zdefiniować, **co jest niedopuszczalne** i jak system ma się zachować, gdy nie jest pewien (NIST AI RMF 1.0, 2023).

#### 3.9. Metryki uogólniania w robotyce uczącej się: dlaczego „działa w laboratorium” to za mało
W klasycznej robotyce często buduje się model i udowadnia własności (stabilność, zbieżność) w określonych założeniach (Craig, 2004; Lynch i Park, 2017). W robotyce opartej o uczenie (Physical AI) sukces mierzy się często statystycznie: **odsetkiem udanych prób** w rozkładzie zadań i środowisk.

Niech \(\tau\) oznacza „zadanie/epizod” (konkretna konfiguracja obiektu, oświetlenia, tarcia, celu), losowane z pewnego rozkładu \(\mathcal{D}\). Metryka sukcesu może mieć postać:
\[
\text{SR} = \mathbb{E}_{\tau\sim\mathcal{D}}\left[\mathbb{1}\{\text{zadanie ukończone}\}\right].
\]
Problem praktyczny polega na tym, że \(\mathcal{D}\) w laboratorium i \(\mathcal{D}\) w realnym wdrożeniu są różne (przesunięcie rozkładu). Dlatego do 2030 standardem staną się zestawy testów typu „in-domain” i „out-of-domain”, testy odporności (zakłócenia czujników, inne tarcie, inne obiekty) oraz raportowanie nie tylko średniej SR, ale i **najgorszych przypadków**.

Publikacje z lat 2023–2025 dotyczące modeli ucieleśnionych (np. PaLM-E; RT-2; Diffusion Policy) pokazują, że skala danych pomaga w uogólnianiu, ale bezpieczeństwo i fizyka nie „wynikają automatycznie” z samego skalowania (Driess i in., 2023; Brohan i in., 2023; Chi i in., 2023). Stąd rośnie rola metod hybrydowych: łączenia komponentów uczonych z klasycznymi ograniczeniami, planowaniem, sterowaniem i walidacją.

#### 3.10. Podsumowanie: dlaczego zaczynamy od metryk
Z punktu widzenia dydaktyki ten wykład pełni rolę „ramy”: zanim zaczniemy formalizować ruch w \(SO(3)\) i \(SE(3)\), musimy ustalić, **po czym poznamy, że system działa dobrze**. Metryki są kompasem: determinują wymagania na kinematykę, dynamikę, percepcję, bezpieczeństwo i architekturę oprogramowania (ROS2). W Części 2 metryki zostaną rozszerzone o miary uogólniania, audytu oraz wpływu rynkowo-społecznego, bo to one będą wyznaczały realne wdrożenia robotyki do 2030 (IFR, 2023–2025).

#### 3.11. Metryki kosztowe: od ceny robota do kosztu całkowitego posiadania
W praktyce wdrożeniowej najczęstszym nieporozumieniem jest utożsamianie „kosztu robotyzacji” z ceną samego robota. Tymczasem to, co interesuje przedsiębiorstwo lub instytucję, to **koszt całkowity posiadania** (TCO) w horyzoncie kilku lat, obejmujący m.in. integrację, oprzyrządowanie, szkolenia, energię, serwis, przestoje oraz koszty jakości (odrzuty). Dydaktycznie użyteczny jest rozkład:
\[
\text{TCO} \approx \text{CapEx} + \text{OpEx},
\]
gdzie CapEx (nakłady inwestycyjne) obejmuje zakup robota, chwytaków, osprzętu, infrastruktury bezpieczeństwa, a OpEx (koszty operacyjne) obejmuje energię, serwis, utrzymanie, części zamienne i koszty przestojów. W wielu aplikacjach to właśnie dostępność \(A\) i czas cyklu \(T_c\) „wchodzą” do TCO, bo determinują produkcję i wykorzystanie zasobów.

Z perspektywy 2030 ważne są dwa trendy: (1) wzrost udziału usług (serwis, aktualizacje, zarządzanie flotą) w kosztach oraz (2) silniejsze powiązanie kosztu z wymaganiami bezpieczeństwa i zgodności (audyt). Dlatego w dalszych wykładach będziemy traktować metryki kosztowe jako równorzędne z metrykami technicznymi, a nie jako „sprawę działu zakupów”.

#### 3.12. Metryki środowiskowe i reprodukowalność: dlaczego „wynik” to nie wszystko
Coraz częściej, zwłaszcza w robotyce mobilnej i usługowej, ocenia się system nie tylko po efektywności, ale i po wpływie środowiskowym. Najprostszy most między energetyką a śladem emisji można zapisać jako:
\[
\text{CO}_2 \approx E \cdot \alpha,
\]
gdzie \(E\) to energia zużyta na zadanie (lub w czasie), a \(\alpha\) to współczynnik emisyjności miksu energetycznego. W dydaktyce nie chodzi o dokładność tego wzoru, lecz o uświadomienie studentom, że „energia na zadanie” staje się metryką, którą można optymalizować technicznie (aktuatory, trajektorie, sterowanie), a jej konsekwencje są mierzalne i raportowalne.

Drugim wątkiem, który do 2030 będzie narastał, jest **reprodukowalność** i inżynieria procesu wytwarzania oprogramowania robotycznego: wersjonowanie modeli, danych i konfiguracji, testy scenariuszowe oraz możliwość odtworzenia decyzji systemu po incydencie. Właśnie dlatego w Części 1 pojawia się ROS2 jako narzędzie integracji i śledzenia eksperymentów, a w Części 2 wrócimy do cyfrowych bliźniaków, walidacji i audytu. W ujęciu praktycznym „dobry robot” to nie tylko robot o wysokiej SR, ale system, którego działanie da się zmierzyć, wyjaśnić i bezpiecznie utrzymać.

Warto już teraz przyjąć nawyk „inżynierii dowodów”: każda teza o skuteczności (czas, energia, bezpieczeństwo, uogólnianie) powinna mieć przypisaną procedurę pomiaru oraz zestaw danych testowych. Ten nawyk będzie w kursie konsekwentnie wzmacniany: od prostych metryk w komórce (Wykład 1) po walidację scenariuszową i raportowanie ograniczeń modeli w robotyce 2030 (Wykłady 16, 23–24).

### 4. Struktura prezentacji slajdów (PowerPoint / Google Slides)
**Założenie stylu:** Assertion–Evidence — tytuł slajdu jest tezą, treść to dowód (rysunek/wykres/animacja), tekst minimalny.

1. **Slajd nr 1 | Robotyka to pętla: percepcja–decyzja–aktuacja**  
   - Treść:  
     - Percepcja \(\rightarrow\) estymacja stanu  
     - Decyzja \(\rightarrow\) plan/sterowanie  
     - Aktuacja \(\rightarrow\) ruch/siła  
     - Sprzężenie zwrotne + niepewność  
   - Sugestie wizualne: schemat blokowy pętli sterowania robota z trzema modułami i strzałkami; na każdym module ikony błędów (szum czujników, opóźnienia, tarcie).  
   - Notatki dla prowadzącego: 2–3 min; pytanie retoryczne: „Czy pralka automatyczna jest robotem? Dlaczego (nie)?”.

2. **Slajd nr 2 | „Robot” różni się od automatu skalą niepewności środowiska**  
   - Treść:  
     - Automaty: środowisko przewidywalne  
     - Roboty: świat zmienny i niepełna obserwowalność  
     - Konsekwencja: estymacja + planowanie  
   - Sugestie wizualne: porównanie dwóch fotografii: linia produkcyjna z osłonami vs. magazyn z ludźmi i AMR; overlay „poziom niepewności”.  
   - Notatki: 2 min; podkreślić, że robotyka „ucieka” z klatek bezpieczeństwa.

3. **Slajd nr 3 | Klasy robotów narzucają inne metryki sukcesu**  
   - Treść:  
     - Manipulator: dokładność/powtarzalność  
     - Mobilny: bezpieczeństwo/lokalizacja  
     - Cobot: kontakt/ergonomia  
     - Humanoid: energia/stabilność  
   - Sugestie wizualne: tablica 2×2 z ikonami klas robotów i podpisami „dominująca trudność”.  
   - Notatki: 3 min; wprowadzić mapę wykładów (co kiedy).

4. **Slajd nr 4 | Dokładność i powtarzalność to dwa różne błędy**  
   - Treść:  
     - \(\mathbf{e}_i=\mathbf{p}_i-\mathbf{p}^\*\)  
     - Dokładność: \(\bar e\)  
     - Powtarzalność: \(\sigma_e\) lub \(3\sigma_e\)  
   - Sugestie wizualne: rzut punktów trafień na tarczę: skupione, ale przesunięte vs. rozproszone wokół celu; podpisy „błąd systematyczny” i „błąd losowy”.  
   - Notatki: 3 min; zapowiedzieć kalibrację i modelowanie.

5. **Slajd nr 5 | Czas cyklu jest sumą etapów procesu, nie „prędkością robota”**  
   - Treść:  
     - \(T_c=T_{\text{dojazd}}+T_{\text{chwyt}}+\dots\)  
     - Wąskie gardła: chwyt, wizja, bezpieczeństwo  
   - Sugestie wizualne: wykres Pareto udziału składowych czasu cyklu dla pick-and-place (przykładowe dane).  
   - Notatki: 2 min; pytanie: „Co zwykle jest najwolniejsze w komórce?”

6. **Slajd nr 6 | Przepustowość wynika z \(T_c\) i strat procesu**  
   - Treść:  
     - \(Q=\frac{3600}{T_c}\cdot\eta\)  
     - \(\eta\): mikroprzestoje, odrzuty, synchronizacja  
   - Sugestie wizualne: prosty wykres \(Q\) w funkcji \(T_c\) dla kilku \(\eta\) (np. 0.8, 0.9, 0.95).  
   - Notatki: 2 min; pokazać nieliniowość „1/T”.

7. **Slajd nr 7 | Dostępność to metryka eksploatacyjna: MTBF i MTTR**  
   - Treść:  
     - \(A\approx \frac{\text{MTBF}}{\text{MTBF}+\text{MTTR}}\)  
     - Diagnostyka skraca MTTR  
   - Sugestie wizualne: oś czasu z odcinkami „praca” i „naprawa”; obok proste obliczenie \(A\).  
   - Notatki: 2 min; podkreślić rolę logów i monitoringu.

8. **Slajd nr 8 | Energia staje się metryką pierwszego rzędu (szczególnie do 2030)**  
   - Treść:  
     - \(E=\int_0^T P(t)\,dt\)  
     - J/m, J/szt., Wh/dzień  
   - Sugestie wizualne: wykres słupkowy „energia na zadanie” dla: manipulator w klatce, AMR, humanoid (wartości poglądowe) + prognoza trendu do 2030.  
   - Notatki: 2 min; łącznik do wykładów o aktuatorach.

9. **Slajd nr 9 | Bezpieczeństwo można parametryzować: \(S=K\cdot T + C\)**  
   - Treść:  
     - \(T\): detekcja + zatrzymanie  
     - \(K\): prędkość zbliżania  
     - \(C\): tolerancje i penetracja  
   - Sugestie wizualne: schemat stref bezpieczeństwa z człowiekiem i skanerem; animacja „wejście w strefę → stop”.  
   - Notatki: 3 min; zaznaczyć, że to model uproszczony, ale użyteczny.

10. **Slajd nr 10 | Coboty wymagają projektowania pod kontakt, nie tylko pod trajektorie**  
   - Treść:  
     - Ograniczanie siły/momentu  
     - Detekcja kolizji  
     - Ergonomia stanowiska  
   - Sugestie wizualne: zdjęcie cobota przy stanowisku montażowym + overlay wektorów sił i stref; prosty wykres limitów momentu vs prędkości.  
   - Notatki: 2–3 min; przejście do roli norm i analizy ryzyka.

11. **Slajd nr 11 | Percepcja to model pomiaru + niepewność, nie „ładny obraz”**  
   - Treść:  
     - Czujniki: kamera, LiDAR, IMU, enkodery  
     - Błąd systematyczny vs losowy  
     - Kalibracja i fuzja  
   - Sugestie wizualne: zestaw ikon czujników + miniwykresy szumu; przykładowa macierz kowariancji jako „cień” niepewności.  
   - Notatki: 2 min; zapowiedzieć SLAM i filtrację.

12. **Slajd nr 12 | ROS2 jest standardem integracji systemu, a nie „samym algorytmem”**  
   - Treść:  
     - Węzły, tematy, usługi, akcje  
     - Diagnostyka, logowanie, odtwarzalność  
     - Integracja sterowania i percepcji  
   - Sugestie wizualne: `rqt_graph` dla prostego układu (cmd → robot → odom → logger) z podświetleniem „gdzie mierzymy metryki”.  
   - Notatki: 3 min; podkreślić, że inżynieria systemu jest kluczowa.

13. **Slajd nr 13 | Robotyka 2024–2025: modele łączące język, obraz i działanie**  
   - Treść:  
     - VLA (vision–language–action)  
     - Dane ucieleśnione w skali  
     - Uogólnianie: obiecujące, ale kruche  
   - Sugestie wizualne: diagram „tekst/obraz → model → sekwencja akcji robota” + czerwone „fizyka/bezpieczeństwo” jako ograniczenia.  
   - Notatki: 3 min; zaznaczyć, że to motyw przewodni Części 2.

14. **Slajd nr 14 | Physical AI: fizyka jest nieusuwalnym ograniczeniem uczenia**  
   - Treść:  
     - Tarcie, kontakt, opóźnienie  
     - Ograniczenia momentu i mocy  
     - Problem sim2real  
   - Sugestie wizualne: sekwencja obrazów chwytu obiektu śliskiego: sukces/porażka; overlay „tarcie” i „siła normalna”.  
   - Notatki: 2 min; zapowiedzieć cyfrowe bliźniaki (Wykład 16).

15. **Slajd nr 15 | Do 2030 wzrośnie rola walidacji i audytu systemów adaptacyjnych**  
   - Treść:  
     - Testy scenariuszowe  
     - Śledzenie danych i wersji modeli  
     - Monitoring po wdrożeniu  
   - Sugestie wizualne: „łańcuch dowodowy”: dane → trening → testy → wdrożenie → monitoring; przy każdym etapie pieczątka „artefakt audytu”.  
   - Notatki: 3 min; łącznik do regulacji i etyki (Wykład 23).

16. **Slajd nr 16 | Humanoidy: potencjał jest duży, ale energia i niezawodność to bariery**  
   - Treść:  
     - Lokomocja + manipulacja  
     - Koszt energii na zadanie  
     - Ryzyko upadków  
   - Sugestie wizualne: zdjęcie humanoida w magazynie + overlay: środek masy, punkty kontaktu stóp, „energia na cykl”.  
   - Notatki: 2 min; zapowiedzieć Wykład 17 i 20.

17. **Slajd nr 17 | Wielorobotowość: skala daje korzyści i nowe tryby awarii**  
   - Treść:  
     - Redundancja i odporność  
     - Koordynacja i komunikacja  
     - Zachowania emergentne  
   - Sugestie wizualne: animacja 30 robotów magazynowych z koloryzacją priorytetów i konfliktów; przykład „zatoru”.  
   - Notatki: 2 min; zapowiedzieć Wykład 21.

18. **Slajd nr 18 | Metryki dobieramy do celu: nie istnieje „jedna najlepsza”**  
   - Treść:  
     - Produkcja: \(T_c, Q, A\)  
     - Medycyna: bezpieczeństwo i dokładność  
     - Ratownictwo: autonomia i niezawodność  
   - Sugestie wizualne: macierz zastosowanie → metryka, z podświetleniem różnych pól dla 3 aplikacji.  
   - Notatki: 2 min; pokazać, że metryki są „językiem projektu”.

19. **Slajd nr 19 | Mini-studium: pick-and-place – kompromisy metryk w praktyce**  
   - Treść:  
     - Wąskie gardło: chwyt/wizja/bufor  
     - Poprawa: równoległość, bufor, lepsze dane  
   - Sugestie wizualne: schemat komórki z etapami i czasami; overlay propozycji usprawnień z przewidywanym zyskiem \(Q\).  
   - Notatki: 3 min; przygotowanie do zadań domowych.

20. **Slajd nr 20 | Mapa kursu: od \(SE(3)\) do robotyki 2030**  
   - Treść:  
     - Matematyka ruchu \(\rightarrow\) kinematyka  
     - Dynamika \(\rightarrow\) sterowanie  
     - Percepcja \(\rightarrow\) SLAM  
     - Physical AI \(\rightarrow\) wyzwania 2030  
   - Sugestie wizualne: oś czasu 24 wykładów z blokami tematycznymi i kolorami (modelowanie / sterowanie / percepcja / trendy 2030).  
   - Notatki: 2 min; zakończyć jasnym „co student powinien umieć po Części 1”.

### 5. Przykłady i studia przypadku (minimum 5 szczegółowych)
#### Przykład 1: Metryki przepustowości dla komórki pick-and-place
**Opis problemu.** Komórka ma wykonać przeniesienie elementu z podajnika do pojemnika. Czasy (w sekundach) oszacowano następująco: dojazd 0.9, chwyt 0.4, weryfikacja wizyjna 0.6, odjazd 0.8, dodatkowy bufor bezpieczeństwa 0.3. Straty organizacyjne \(\eta=0.92\). Oblicz czas cyklu i przepustowość.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Czas cyklu:
\[
T_c = 0.9 + 0.4 + 0.6 + 0.8 + 0.3 = 3.0\ \text{s}.
\]
2) Przepustowość idealna:
\[
Q_0 = \frac{3600}{3.0} = 1200\ \text{szt./h}.
\]
3) Przepustowość z uwzględnieniem strat:
\[
Q = Q_0\cdot \eta = 1200\cdot 0.92 = 1104\ \text{szt./h}.
\]

**Kod (Python 3.11 + pseudodane).**
```python
from dataclasses import dataclass

@dataclass
class Cycle:
    t_travel_in: float
    t_grip: float
    t_vision: float
    t_travel_out: float
    t_buffer: float
    eta: float = 1.0

    def cycle_time(self) -> float:
        return self.t_travel_in + self.t_grip + self.t_vision + self.t_travel_out + self.t_buffer

    def throughput_per_hour(self) -> float:
        tc = self.cycle_time()
        return (3600.0 / tc) * self.eta

cycle = Cycle(0.9, 0.4, 0.6, 0.8, 0.3, eta=0.92)
print("Tc [s] =", cycle.cycle_time())
print("Q [pcs/h] =", cycle.throughput_per_hour())
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Wykres Pareto udziału poszczególnych składowych czasu w \(T_c\) (słupki + wartości procentowe) oraz obok liczba \(Q\) w szt./h.

---

#### Przykład 2: Dokładność vs powtarzalność na podstawie pomiarów
**Opis problemu.** Robot ma trafić w punkt \(\mathbf{p}^\*=(0,0)\) w płaszczyźnie. Zarejestrowano 10 trafień (w mm):  
\((2.0, -1.0), (2.2,-1.1), (1.9,-0.9), (2.1,-1.2), (2.0,-1.1), (2.1,-1.0), (2.3,-1.1), (2.0,-1.3), (1.8,-0.8), (2.2,-1.0)\).  
Oblicz (a) średni błąd wektorowy \(\bar{\mathbf{e}}\), (b) średnią normę błędu \(\bar e\), (c) miarę powtarzalności jako \(3\sigma_e\) dla \(e_i=\|\mathbf{e}_i\|\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Średni błąd wektorowy:
\[
\bar{\mathbf{e}}=\left(\frac{1}{N}\sum x_i,\ \frac{1}{N}\sum y_i\right).
\]
Suma \(x\): \(20.6\), suma \(y\): \(-10.5\), zatem:
\[
\bar{\mathbf{e}}=(2.06,\ -1.05)\ \text{mm}.
\]
To wskazuje na istotny **błąd systematyczny** (przesunięcie).

2) Dla każdej próby:
\[
e_i=\sqrt{x_i^2+y_i^2},
\qquad
\bar e=\frac{1}{N}\sum e_i,
\qquad
\sigma_e=\sqrt{\frac{1}{N-1}\sum (e_i-\bar e)^2}.
\]

**Kod (Python 3.11).**
```python
import math
import statistics

points = [
    (2.0, -1.0), (2.2, -1.1), (1.9, -0.9), (2.1, -1.2), (2.0, -1.1),
    (2.1, -1.0), (2.3, -1.1), (2.0, -1.3), (1.8, -0.8), (2.2, -1.0),
]

xs = [p[0] for p in points]
ys = [p[1] for p in points]
e_bar_vec = (sum(xs) / len(xs), sum(ys) / len(ys))
errors = [math.hypot(x, y) for x, y in points]

e_mean = sum(errors) / len(errors)
sigma = statistics.stdev(errors)

print("e_bar vector [mm] =", e_bar_vec)
print("mean norm error [mm] =", e_mean)
print("repeatability ~ 3*sigma_e [mm] =", 3.0 * sigma)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Wykres punktów trafień na płaszczyźnie z zaznaczonym celem w (0,0), środkiem \(\bar{\mathbf{e}}\) i okręgiem promienia \(3\sigma_e\) wokół \(\bar{\mathbf{e}}\) jako miarą rozrzutu.

---

#### Przykład 3: Odległość bezpieczeństwa z uproszczonego modelu \(S=K\cdot T + C\)
**Opis problemu.** Projektujesz stanowisko z robotem, gdzie detekcja wejścia człowieka do strefy realizowana jest skanerem. Przyjmij \(K=1600\ \text{mm/s}\), czas reakcji systemu \(T = 0.18\ \text{s}\) (detekcja) + \(0.22\ \text{s}\) (zatrzymanie) = \(0.40\ \text{s}\), oraz \(C=120\ \text{mm}\). Oblicz minimalną odległość \(S\).

**Rozwiązanie krok po kroku (z obliczeniami).**
\[
S = 1600\cdot 0.40 + 120 = 640 + 120 = 760\ \text{mm}.
\]
Interpretacja: w tym uproszczeniu granica strefy detekcji powinna być co najmniej ~0.76 m od miejsca zagrożenia, aby zatrzymanie nastąpiło „na czas”. W praktyce należy uwzględnić dodatkowe czynniki (geometrię, tolerancje, scenariusze), a obliczenia wykonywać w reżimie właściwych norm i oceny ryzyka.

**Kod (Python 3.11).**
```python
K = 1600  # mm/s
T = 0.18 + 0.22  # s
C = 120  # mm
S = K * T + C
print("S [mm] =", S)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Rzut z góry stanowiska: robot + strefa zagrożenia + pierścień strefy detekcji o promieniu odpowiadającym \(S\); strzałki pokazujące składniki czasu \(T\).

---

#### Przykład 4: Jednokrokowy filtr Kalmana dla estymacji prędkości (enkoder + IMU)
**Opis problemu.** Estymujesz prędkość liniową robota mobilnego \(v\). Model: \(v_k = v_{k-1} + w\), gdzie \(w\sim\mathcal{N}(0,Q)\). Pomiary: enkoder \(z^{(e)}_k\) i IMU \(z^{(i)}_k\) są niezależne i mierzą \(v\) z szumami \(R_e, R_i\). Dla kroku \(k\):  
\(v_{k-1}=0.50\ \text{m/s}\), \(P_{k-1}=0.04\), \(Q=0.01\).  
Pomiary: \(z^{(e)}=0.62\), \(R_e=0.02\) oraz \(z^{(i)}=0.55\), \(R_i=0.05\).  
Wykonaj predykcję i aktualizację sekwencyjną (najpierw enkoder, potem IMU).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Predykcja:
\[
\hat v^- = 0.50,\qquad P^- = P_{k-1}+Q = 0.04+0.01=0.05.
\]
2) Aktualizacja enkoderem:
\[
K_e = \frac{P^-}{P^-+R_e}=\frac{0.05}{0.05+0.02}=\frac{0.05}{0.07}\approx 0.714.
\]
\[
\hat v^{(e)} = \hat v^- + K_e(z^{(e)}-\hat v^-)=0.50+0.714(0.62-0.50)=0.5857.
\]
\[
P^{(e)} = (1-K_e)P^- \approx (1-0.714)\cdot 0.05=0.0143.
\]
3) Aktualizacja IMU:
\[
K_i = \frac{P^{(e)}}{P^{(e)}+R_i}=\frac{0.0143}{0.0143+0.05}\approx 0.222.
\]
\[
\hat v = \hat v^{(e)} + K_i(z^{(i)}-\hat v^{(e)})\approx 0.5857 + 0.222(0.55-0.5857)\approx 0.5778.
\]
\[
P = (1-K_i)P^{(e)}\approx 0.0111.
\]

**Kod (Python 3.11).**
```python
v = 0.50
P = 0.04
Q = 0.01

z_e, R_e = 0.62, 0.02
z_i, R_i = 0.55, 0.05

# predykcja
v_pred = v
P_pred = P + Q

# aktualizacja: enkoder
K_e = P_pred / (P_pred + R_e)
v_e = v_pred + K_e * (z_e - v_pred)
P_e = (1 - K_e) * P_pred

# aktualizacja: IMU
K_i = P_e / (P_e + R_i)
v_upd = v_e + K_i * (z_i - v_e)
P_upd = (1 - K_i) * P_e

print("pred:", v_pred, P_pred)
print("upd:", v_upd, P_upd)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Wykres czasowy: predykcja (linia przerywana), pomiar enkodera i IMU (punkty z „belkami” niepewności), estymata po fuzji (linia ciągła) oraz malejące \(P\) jako wykres w drugim panelu.

---

#### Przykład 5: Minimalny system ROS2 do zbierania metryk (publisher/subscriber)
**Opis problemu.** Chcesz rejestrować czas cyklu i liczbę wykonanych zadań w komórce. Zaimplementuj w ROS2 (Python, `rclpy`) dwa węzły:
1) `cycle_publisher` publikuje na temat `/cycle_time` wartości czasu cyklu (float) co 1 s (symulacja).  
2) `metrics_logger` subskrybuje `/cycle_time` i oblicza średnią kroczącą z ostatnich 30 próbek.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Definiujemy próbkę \(T_c\).  
2) Bufor okna \(N=30\).  
3) Średnia krocząca:
\[
\bar T_c(k)=\frac{1}{N}\sum_{j=k-N+1}^{k} T_c(j).
\]

**Kod (Python 3.11 + ROS2 Humble, `rclpy`).**
```python
# cycle_publisher.py
import random
import rclpy
from rclpy.node import Node
from std_msgs.msg import Float32

class CyclePublisher(Node):
    def __init__(self):
        super().__init__("cycle_publisher")
        self.pub = self.create_publisher(Float32, "/cycle_time", 10)
        self.timer = self.create_timer(1.0, self.tick)

    def tick(self):
        msg = Float32()
        msg.data = float(3.0 + random.uniform(-0.25, 0.35))
        self.pub.publish(msg)

def main():
    rclpy.init()
    node = CyclePublisher()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
```
```python
# metrics_logger.py
from collections import deque
import rclpy
from rclpy.node import Node
from std_msgs.msg import Float32

class MetricsLogger(Node):
    def __init__(self):
        super().__init__("metrics_logger")
        self.window = deque(maxlen=30)
        self.sub = self.create_subscription(Float32, "/cycle_time", self.cb, 10)

    def cb(self, msg: Float32):
        self.window.append(float(msg.data))
        avg = sum(self.window) / len(self.window)
        self.get_logger().info(f"Tc={msg.data:.3f} s | avg30={avg:.3f} s | n={len(self.window)}")

def main():
    rclpy.init()
    node = MetricsLogger()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Zrzut z terminala z logami `avg30`, plus wykres z `rqt_plot` pokazujący `/cycle_time`; w wariancie rozszerzonym drugi temat `/cycle_avg30` dla porównania.

### 6. Materiały dla studentów
#### 6 pytań teoretycznych (z oczekiwanymi odpowiedziami)
1) **Pytanie:** Podaj definicję operacyjną robota używaną w kursie.  
   **Odpowiedź:** System mechatroniczny z percepcją, decyzją i aktuacją w pętli sprzężenia zwrotnego, działający w warunkach niepewności.
2) **Pytanie:** Czym różni się dokładność od powtarzalności?  
   **Odpowiedź:** Dokładność opisuje błąd względem wartości zadanej (absolutnie), a powtarzalność rozrzut wyników przy powtarzaniu zadania (wariancja/odchylenie).
3) **Pytanie:** Dlaczego czas cyklu jest ważniejszy od maksymalnej prędkości osi robota w aplikacji przemysłowej?  
   **Odpowiedź:** Czas cyklu zawiera wszystkie składowe procesu (chwyt, wizja, bufory bezpieczeństwa), które często dominują.
4) **Pytanie:** Zinterpretuj wzór \(A \approx \frac{\text{MTBF}}{\text{MTBF}+\text{MTTR}}\).  
   **Odpowiedź:** Dostępność rośnie, gdy awarie są rzadkie (duże MTBF) i/lub naprawy szybkie (małe MTTR).
5) **Pytanie:** Co oznacza w praktyce, że energia staje się metryką pierwszego rzędu?  
   **Odpowiedź:** Projektuje się robota pod budżet energii na zadanie; ogranicza to prędkości, masy, dobór aktuatorów i algorytmów.
6) **Pytanie:** Co wnosi trend Physical AI względem klasycznej robotyki?  
   **Odpowiedź:** Skalowane uczenie i uogólnianie, ale z koniecznością respektowania fizyki i bezpieczeństwa; rośnie rola danych i walidacji.

#### 4 zadania obliczeniowe/programistyczne (z poziomem trudności)
1) **(Łatwe)** Oblicz przepustowość \(Q\) dla trzech wariantów \(T_c\) i \(\eta\); wskaż, który parametr najbardziej wpływa na wynik.  
2) **(Średnie)** Z danych punktów trafień wyznacz \(\bar{\mathbf{e}}, \bar e, \sigma_e\) i narysuj wykres; zinterpretuj, czy dominuje błąd systematyczny czy losowy.  
3) **(Średnie)** Zaprojektuj prosty arkusz (Python/pandas) do liczenia dostępności z logów awarii (czasy start/stop) i porównaj dwa scenariusze MTTR.  
4) **(Trudne)** Zaimplementuj w ROS2 węzeł, który subskrybuje `/cycle_time`, wykrywa anomalie (np. \(T_c > \bar T_c + 3\sigma\)) i publikuje alarm na `/cycle_alarm`.

#### 1 projekt laboratoryjny / projekt domowy (z kryteriami oceny)
**Projekt:** „Karta metryk” dla wybranej aplikacji robotycznej (np. paletyzacja, AMR w magazynie, robot inspekcyjny).  
**Wymagania:**  
- Opisz aplikację, środowisko i ograniczenia (1–2 strony).  
- Wybierz 6–10 metryk sukcesu (w tym: bezpieczeństwo, energia lub czas).  
- Zdefiniuj sposób pomiaru każdej metryki (czujniki/logi/testy).  
- Zaproponuj plan walidacji (scenariusze testowe + kryteria zaliczenia).  
**Kryteria oceny (100 pkt):** metryki (30), poprawność definicji i pomiaru (25), realizm walidacji (25), klarowność i spójność (20).

### 7. Quiz sprawdzający (15 pytań: 10 wyboru wielokrotnego + 5 otwartych + klucz odpowiedzi)
#### 10 pytań wyboru wielokrotnego
1) Dokładność robota najtrafniej opisuje:  
   A) rozrzut wyników wokół średniej, B) błąd względem celu, C) maksymalną prędkość osi, D) liczbę osi  
2) Powtarzalność robota może być wysoka nawet wtedy, gdy:  
   A) ma duży błąd kalibracji, B) ma idealną kamerę, C) ma małą masę, D) jest humanoidem  
3) Czas cyklu \(T_c\) obejmuje:  
   A) tylko ruch robota, B) tylko chwyt, C) wszystkie składowe procesu, D) tylko sterowanie  
4) Dostępność \(A\) rośnie, gdy:  
   A) rośnie MTTR, B) maleje MTBF, C) maleje MTTR, D) rośnie liczba czujników  
5) Wzór \(E=\int P(t)\,dt\) opisuje:  
   A) energię, B) moc, C) przyspieszenie, D) dokładność  
6) „Physical AI” podkreśla, że:  
   A) robot uczy się bez danych, B) fizyka ogranicza działania, C) nie ma potrzeby sterowania, D) SLAM znika  
7) ROS2 w systemie robotycznym służy głównie do:  
   A) spawania, B) komunikacji i integracji, C) pomiaru siły bez czujników, D) obliczania Jacobianu  
8) Metryka „szt./h” jest najbardziej związana z:  
   A) przepustowością, B) kalibracją, C) kinematyką odwrotną, D) kwaternionami  
9) Najbardziej prawdopodobny wąski gardło w pick-and-place to:  
   A) zawsze prędkość osi 1, B) zawsze moc CPU, C) często chwyt i wizja, D) zawsze liczba osi  
10) W modelu \(S=K\cdot T + C\) parametr \(T\) to:  
   A) temperatura, B) czas reakcji systemu, C) moment, D) tarcie

#### 5 pytań otwartych
11) Podaj przykład systemu, który *nie* jest robotem wg definicji kursu, i uzasadnij.  
12) Zaproponuj zestaw 5 metryk dla robota magazynowego AMR i uzasadnij dobór.  
13) Wyjaśnij, dlaczego „więcej czujników” nie zawsze oznacza „lepszą percepcję”.  
14) Opisz kompromis między bezpieczeństwem a przepustowością w komórce z robotem.  
15) Podaj 2 powody, dla których do 2030 wzrośnie rola audytu i walidacji w robotyce.

#### Klucz odpowiedzi
1) B, 2) A, 3) C, 4) C, 5) A, 6) B, 7) B, 8) A, 9) C, 10) B.  
11–15) Oceniane opisowo: poprawność merytoryczna + argumentacja.

### 8. Bibliografia i materiały dodatkowe
1) B. Siciliano, O. Khatib (red.), *Springer Handbook of Robotics*, 2nd ed., Springer, 2016.  
2) B. Siciliano, L. Sciavicco, L. Villani, G. Oriolo, *Robotics: Modelling, Planning and Control*, Springer, 2009.  
3) K. M. Lynch, F. C. Park, *Modern Robotics: Mechanics, Planning, and Control*, Cambridge University Press, 2017 (+ materiały online).  
4) P. Corke, *Robotics, Vision and Control*, 2nd ed., Springer, 2017.  
5) J. J. Craig, *Introduction to Robotics: Mechanics and Control*, 3rd ed., Pearson, 2004.  
6) ISO 10218-1 / ISO 10218-2, *Robots and robotic devices — Safety requirements for industrial robots*, ISO (wydania bazowe + aktualizacje).  
7) ISO/TS 15066, *Robots and robotic devices — Collaborative robots*, ISO, 2016.  
8) ISO 13855, *Safety of machinery — Positioning of safeguards with respect to the approach speeds of parts of the human body*, ISO, 2010.  
9) ROS 2 Documentation: Humble Hawksbill (LTS, 2022) — `rclpy`, komunikacja, narzędzia diagnostyczne.  
10) NIST, *AI Risk Management Framework (AI RMF 1.0)*, 2023.  
11) V. Driess i in., *PaLM-E: An Embodied Multimodal Language Model*, 2023.  
12) A. Brohan i in., *RT-2: Vision-Language-Action Models Transfer Web Knowledge to Robotic Control*, 2023.  
13) Y. Chi i in., *Diffusion Policy: Visuomotor Policy Learning via Action Diffusion*, 2023.  
14) International Federation of Robotics (IFR), *World Robotics* — raporty branżowe (wydania 2023–2025; część „Industrial Robots” i „Service Robots”).  
15) Materiały wideo/online (do samodzielnego utrwalenia): wykłady *Modern Robotics* (K. Lynch) oraz kanały IEEE RAS / Robotics and Automation (wybór tematyczny).
