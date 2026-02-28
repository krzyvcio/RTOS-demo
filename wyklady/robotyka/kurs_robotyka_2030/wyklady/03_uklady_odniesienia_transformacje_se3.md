# Wykład 03

### 1. Tytuł wykładu
Układy odniesienia i transformacje jednorodne w \(SE(3)\)

### 2. Cele nauczania (5–7 punktów)
- Student zrozumie pojęcie układu odniesienia (ramy) i potrafi poprawnie zapisywać wektory w różnych ramach.
- Student będzie potrafił budować transformacje jednorodne \(\mathbf{T}\in SE(3)\) oraz interpretować ich część obrotową i translacyjną.
- Student będzie potrafił składać transformacje (łańcuchy kinematyczne) oraz wykonywać transformacje punktów i wektorów.
- Student zrozumie różnicę między transformacją punktu a transformacją wektora (kierunku) oraz znaczenie współrzędnych jednorodnych.
- Student będzie potrafił wyznaczyć transformację odwrotną \(\mathbf{T}^{-1}\) i zinterpretować jej sens fizyczny.
- Student pozna, jak formalizm \(SE(3)\) przenosi się na praktykę: kalibrację czujników, systemy wizyjne, TF w ROS2 oraz cyfrowe bliźniaki do 2030.

### 3. Wstęp teoretyczny
#### 3.1. Dlaczego układy odniesienia są „ukrytym szkieletem” całej robotyki
W robotyce większość błędów, które na początku wyglądają jak „problem algorytmu”, w rzeczywistości okazuje się problemem **niejednoznaczności układów odniesienia**: kamera patrzy w inną stronę niż zakładaliśmy, oś \(z\) w symulatorze jest „do góry”, a w dokumentacji „do przodu”, a czujnik IMU raportuje orientację w swojej własnej ramie. Dlatego formalizm \(SE(3)\) i transformacji jednorodnych jest czymś więcej niż notacją — jest sposobem, aby utrzymać spójność opisu świata w całym „stosie” systemu robotycznego: od mechaniki, przez sterowanie, po percepcję i integrację w ROS2 (Lynch i Park, 2017; Corke, 2017).

W skrócie: **ramy (układy odniesienia) są obiektami pierwszej klasy**. Jeśli student opanuje spójny zapis i operacje na transformacjach, to:
- kinematyka manipulatora (Wykłady 4–6) staje się składaniem \(\mathbf{T}\),
- SLAM i fuzja czujników (Wykłady 10–11) stają się estymacją \(\mathbf{T}\) i jej niepewności,
- kalibracja (kamera–robot, LiDAR–IMU) jest wyznaczaniem stałych transformacji między ramami,
- cyfrowy bliźniak (Część 2) jest utrzymywaniem spójności modeli i transformacji w czasie.

#### 3.2. Notacja: „w jakiej ramie jest ten wektor?”
Ustalmy konwencję, której będziemy konsekwentnie używać:
- \(\{A\}\) — rama (układ odniesienia) A,  
- \({}^A\mathbf{p}\in\mathbb{R}^3\) — współrzędne punktu \(\mathbf{p}\) zapisane w ramie \(\{A\}\),  
- \({}^A\mathbf{R}_B\in SO(3)\) — macierz obrotu, która mapuje współrzędne z \(\{B\}\) do \(\{A\}\),  
- \({}^A\mathbf{p}_B\in\mathbb{R}^3\) — wektor położenia początku \(\{B\}\) względem \(\{A\}\), wyrażony w \(\{A\}\).

To prowadzi do kluczowego zapisu transformacji sztywnej:
\[
{}^A\mathbf{p} = {}^A\mathbf{R}_B\,{}^B\mathbf{p} + {}^A\mathbf{p}_B.
\]
Interpretacja jest geometrycznie prosta: najpierw obracamy współrzędne z \(\{B\}\) do \(\{A\}\), a potem dodajemy przesunięcie.

Warto odróżnić dwie sytuacje, które w praktyce są często mylone:
1) **zmiana ramy opisu tego samego punktu** (ten sam punkt w świecie, inne współrzędne),  
2) **fizyczny ruch punktu/robota w świecie** (zmienia się położenie i orientacja w czasie).

Formalizm \(SE(3)\) obsługuje oba przypadki, ale znaczenie symboli zależy od kontekstu. W kolejnych wykładach będziemy zawsze zadawać pytanie kontrolne: „czy to jest *zmiana opisu*, czy *ruch*?”.

#### 3.3. Współrzędne jednorodne i macierz \(\mathbf{T}\in SE(3)\)
Zapis \({}^A\mathbf{p} = {}^A\mathbf{R}_B\,{}^B\mathbf{p} + {}^A\mathbf{p}_B\) jest poprawny, ale uciążliwy przy składaniu wielu transformacji. Dlatego wprowadza się **współrzędne jednorodne**:
\[
\tilde{\mathbf{p}} =
\begin{bmatrix}
\mathbf{p}\\
1
\end{bmatrix}\in\mathbb{R}^4,
\]
oraz **transformację jednorodną**:
\[
{}^A\mathbf{T}_B=
\begin{bmatrix}
{}^A\mathbf{R}_B & {}^A\mathbf{p}_B\\
\mathbf{0}^\top & 1
\end{bmatrix}\in SE(3).
\]
Wtedy transformacja punktu jest jedną operacją macierzową:
\[
{}^A\tilde{\mathbf{p}} = {}^A\mathbf{T}_B\,{}^B\tilde{\mathbf{p}}.
\]

Zauważ, że \(SE(3)\) jest grupą: składanie transformacji odpowiada mnożeniu macierzy. To jest powód, dla którego formalizm jest tak potężny: „złożona geometria” redukuje się do konsekwentnego rachunku macierzy.

#### 3.4. Składanie transformacji: łańcuchy kinematyczne i „graf ramek”
Jeśli znasz \({}^A\mathbf{T}_B\) i \({}^B\mathbf{T}_C\), to transformacja z \(\{C\}\) do \(\{A\}\) jest dana przez:
\[
{}^A\mathbf{T}_C = {}^A\mathbf{T}_B\,{}^B\mathbf{T}_C.
\]
To równanie jest sednem kinematyki: manipulator o wielu ogniwach jest niczym innym jak łańcuchem takich transformacji (Craig, 2004; Lynch i Park, 2017). W robotyce mobilnej analogicznie składa się transformacje odometrii, IMU i mapy.

W praktycznych systemach (zwłaszcza w ROS2) ramy tworzą **graf** (często drzewo TF). Typowe ramy to:

| Rama | Znaczenie | Przykład w ROS2 | Uwagi |
|---|---|---|---|
| \(\{W\}\) | świat / mapa | `map` | zwykle „globalna” |
| \(\{O\}\) | odometria | `odom` | płynna, ale dryfuje |
| \(\{B\}\) | baza robota | `base_link` | centralna rama platformy |
| \(\{C\}\) | kamera | `camera_link` | wymaga kalibracji względem \(\{B\}\) |
| \(\{I\}\) | IMU | `imu_link` | konwencje osi krytyczne |
| \(\{T\}\) | narzędzie | `tool0` | istotne w manipulacji |

Wykorzystując składanie \({}^W\mathbf{T}_T = {}^W\mathbf{T}_B\,{}^B\mathbf{T}_T\) można obliczać pozycję narzędzia w mapie, ale tylko wtedy, gdy wszystkie transformacje są spójne i poprawnie zorientowane.

#### 3.5. Transformacja odwrotna: „odwrócenie perspektywy”
Transformacja \({}^A\mathbf{T}_B\) mapuje współrzędne z \(\{B\}\) do \(\{A\}\). Często potrzebujemy odwrotności: \({}^B\mathbf{T}_A\). Korzystając z własności macierzy obrotu \(\mathbf{R}^{-1}=\mathbf{R}^\top\), otrzymujemy:
\[
({}^A\mathbf{T}_B)^{-1} =
{}^B\mathbf{T}_A =
\begin{bmatrix}
{}^A\mathbf{R}_B^\top & -{}^A\mathbf{R}_B^\top\,{}^A\mathbf{p}_B\\
\mathbf{0}^\top & 1
\end{bmatrix}.
\]
Interpretacja jest ważna: odwrócenie transformacji nie jest „odwróceniem znaku przesunięcia”, tylko przesunięciem wyrażonym w innej ramie i skorygowanym obrotem. W praktyce to częste źródło błędów w kalibracji i w kodzie.

#### 3.6. Punkt a wektor (kierunek): translacja działa tylko na punkty
Transformacje sztywne działają inaczej na:
- **punkty** (mają położenie),  
- **wektory kierunku** (np. prędkość, normalna płaszczyzny, oś).

Jeśli \(\mathbf{d}\) jest wektorem kierunku, to nie ma „położenia”, więc translacja nie powinna go zmieniać:
\[
{}^A\mathbf{d} = {}^A\mathbf{R}_B\,{}^B\mathbf{d}.
\]
Natomiast dla punktu \(\mathbf{p}\) translacja ma znaczenie:
\[
{}^A\mathbf{p} = {}^A\mathbf{R}_B\,{}^B\mathbf{p} + {}^A\mathbf{p}_B.
\]
Współrzędne jednorodne kodują to w prosty sposób: punkt ma ostatnią składową \(1\), a wektor kierunku ma \(0\):
\[
\tilde{\mathbf{p}}=\begin{bmatrix}\mathbf{p}\\1\end{bmatrix},\qquad
\tilde{\mathbf{d}}=\begin{bmatrix}\mathbf{d}\\0\end{bmatrix}.
\]
Wtedy \(\mathbf{T}\tilde{\mathbf{d}}\) automatycznie „ignoruje” translację.

Ta różnica jest krytyczna m.in. w:
- transformacji normalnych w geometrii 3D (w uogólnieniu: dla transformacji afinicznych potrzebna jest macierz odwrotna transponowana),
- przenoszeniu prędkości i sił między ramami,
- interpretacji osi czujnika (np. IMU).

#### 3.7. \(SE(3)\) jako grupa: mnożenie, nie dodawanie
Wykład 2 podkreślał, że \(SO(3)\) nie jest przestrzenią euklidesową. To samo (z jeszcze większą siłą) dotyczy \(SE(3)\). Nie ma sensu „dodawać” transformacji, jeśli chcemy zachować strukturę ruchu. Poprawnym sposobem składania jest mnożenie:
\[
\mathbf{T}_{AC}=\mathbf{T}_{AB}\mathbf{T}_{BC}.
\]

Jeśli potrzebujesz „małej poprawki” transformacji, używasz formalizmu \(\exp\) w \(\mathfrak{se}(3)\):
\[
\mathbf{T} \leftarrow \mathbf{T}\exp([\delta\boldsymbol{\xi}]_\wedge),\qquad \delta\boldsymbol{\xi}\in\mathbb{R}^6,
\]
czyli analogicznie do perturbacji na \(SO(3)\). Ta idea wróci w SLAM i w optymalizacji trajektorii (Barfoot, 2017).

#### 3.8. Adiunkcja: jak prędkości i „skręty” zmieniają ramę
Transformacja punktów jest prosta, ale w robotyce równie ważne są **prędkości** i **skręty**. Jeśli \(\boldsymbol{\xi}=[\boldsymbol{\omega};\mathbf{v}]\) jest skrętem w ramie \(\{B\}\), to w ramie \(\{A\}\) jest on związany przez macierz adiunkcji:
\[
{}^A\boldsymbol{\xi} = \mathrm{Ad}({}^A\mathbf{T}_B)\,{}^B\boldsymbol{\xi},
\]
gdzie:
\[
\mathrm{Ad}(\mathbf{T})=
\begin{bmatrix}
\mathbf{R} & \mathbf{0}\\
[\mathbf{p}]_\times\mathbf{R} & \mathbf{R}
\end{bmatrix}.
\]
Nie musisz jeszcze pamiętać tego wzoru na pamięć; ważne jest zrozumienie, że „zmiana ramy” dla prędkości liniowej zależy od położenia \(\mathbf{p}\) (pojawia się składnik wynikający z obrotu).

To narzędzie będzie niezbędne, gdy przejdziemy do Jacobianu manipulatora (Wykład 6) i do dynamiki (Wykład 7).

#### 3.9. Kalibracja jako wyznaczanie stałej transformacji
Jednym z najczęstszych zadań inżynierskich jest kalibracja: wyznaczenie \({}^B\mathbf{T}_C\) między bazą robota \(\{B\}\) a czujnikiem \(\{C\}\) (kamera, LiDAR). W idealnym świecie jest to stała transformacja. W praktyce:
- \({}^B\mathbf{T}_C\) jest stała tylko w granicach tolerancji mechanicznej,
- dochodzą błędy montażu, ugięcia, temperatura,
- a w humanoidach i układach miękkich „stałość” bywa przybliżeniem.

Już na poziomie podstaw warto zrozumieć, że kalibracja to „dopasowanie” w \(SE(3)\), a więc problem nieliniowy, w którym błędy powinny być mierzone geometrycznie (na \(SO(3)\) i \(\mathbb{R}^3\)), a poprawki wykonywane przez \(\exp([\delta\boldsymbol{\xi}]_\wedge)\), a nie przez dodawanie do macierzy.

#### 3.10. Prognoza do 2030: \(SE(3)\) jako wspólny język cyfrowych bliźniaków i Physical AI
Trend 2024–2025 jest jasny: roboty coraz częściej działają w środowiskach, gdzie nie ma „jednej prawdy” o układach odniesienia — jest wiele źródeł informacji (wzrok, LiDAR, IMU, modele mapy) i wiele warstw modelu (symulacja, bliźniak, system rzeczywisty). Aby to spiąć, potrzebny jest precyzyjny język transformacji.

Do 2030 formalizm \(SE(3)\) będzie coraz częściej:
- wbudowany w moduły percepcji (szacowanie pozy 6D obiektów),  
- używany w symulacji i cyfrowych bliźniakach do utrzymywania spójności geometrii,  
- łączony z uczeniem (np. modele uczące się działań muszą respektować ramy i metryki na \(SO(3)\)),  
- audytowany (spójność ramek i transformacji jako element bezpieczeństwa i walidacji; NIST AI RMF, 2023).

W praktyce oznacza to, że umiejętność „czytania” grafu ramek oraz poprawnego składania/inwertowania \(\mathbf{T}\) będzie równie podstawowa jak umiejętność rozpisania równania ruchu. To jest też punkt, w którym klasyczna robotyka spotyka się z robotyką 2030: modele mogą się zmieniać, ale geometria i spójność układów odniesienia pozostają niezmienne.

#### 3.11. Aktywna vs pasywna transformacja: dwa opisy tego samego faktu
W literaturze spotkasz dwa sposoby myślenia o transformacjach:
- **pasywny**: zmieniamy *opis* tego samego obiektu w innej ramie (zmiana współrzędnych),  
- **aktywny**: fizycznie obracamy/przesuwamy obiekt w przestrzeni.

Matematycznie te opisy mogą prowadzić do podobnych równań, ale różnią się interpretacją symboli oraz kolejnością mnożenia. W praktyce inżynierskiej nie chodzi o to, by „wygrać spór terminologiczny”, lecz by konsekwentnie trzymać się jednej konwencji w całym projekcie. Dlatego w kursie przyjmujemy konwencję roboczą:
\[
{}^A\tilde{\mathbf{p}} = {}^A\mathbf{T}_B\,{}^B\tilde{\mathbf{p}},
\]
czyli \({}^A\mathbf{T}_B\) jest zawsze „mapą współrzędnych z B do A”. Jeśli student pamięta ten jeden wzór i konsekwentnie sprawdza, „co jest po lewej, a co po prawej”, to unika większości błędów.

Typowe symptomy pomyłki konwencji:
- trajektoria „idzie w złą stronę”, ale tylko po obrocie robota,
- kamera „widzi” obiekty poprawnie w jednym ustawieniu, a błędnie w innym,
- odometria i IMU „walczą” ze sobą (inne osie, inne znaki).

#### 3.12. Konwencje osi i jednostek: ENU/NED, prawoskrętność i „ukryte założenia”
W systemach robotycznych istnieją równoległe tradycje:
- robotyka mobilna i ROS zwykle używają układu prawoskrętnego z \(x\) do przodu, \(y\) w lewo, \(z\) do góry (por. REP 103),  
- lotnictwo często używa NED (north–east–down),  
- w wizji komputerowej kamera bywa opisana tak, że \(z\) „patrzy” wzdłuż osi optycznej.

To oznacza, że sama macierz \(\mathbf{R}\) jest poprawna matematycznie, ale może odpowiadać innej intuicji osi. Dydaktyczna praktyka: zawsze rysuj osie i testuj transformacje na prostych punktach kontrolnych, np.:
\[
{}^B\mathbf{p}=\begin{bmatrix}1\\0\\0\end{bmatrix}\ \Rightarrow\ \text{„punkt 1 m przed robotem”}.
\]
Jeżeli po transformacji punkt ląduje „z tyłu”, to problemem jest konwencja, nie algebra.

Analogicznie, jednostki (metry vs milimetry, radiany vs stopnie) są najprostszą drogą do katastrofy. W formalizmie \(SE(3)\) translacje i obroty są w jednej macierzy, więc błąd jednostek bywa mniej widoczny na pierwszy rzut oka, a ujawnia się dopiero w dynamice lub w planowaniu ruchu.

#### 3.13. Niepewność pozy i propagacja przez \(SE(3)\): dlaczego do 2030 będzie to standard
W realnym systemie \({}^A\mathbf{T}_B\) nie jest „dokładne”. Jest oszacowaniem, które ma niepewność wynikającą z szumu czujników, błędu kalibracji i modelu. W robotyce praktycznej niepewność pozy opisuje się często w przestrzeni stycznej (algebrze) przez wektor \(\delta\boldsymbol{\xi}\in\mathbb{R}^6\) i macierz kowariancji \(\mathbf{\Sigma}\in\mathbb{R}^{6\times 6}\).

Jeżeli zmieniamy ramę opisu skrętu (lub perturbacji) przez \(\mathrm{Ad}(\mathbf{T})\), to kowariancja transformuje się jako:
\[
\mathbf{\Sigma}_A = \mathrm{Ad}(\mathbf{T})\,\mathbf{\Sigma}_B\,\mathrm{Ad}(\mathbf{T})^\top.
\]
To równanie jest kluczowe w fuzji czujników i w SLAM: nawet jeśli algorytm „działa”, błędna propagacja niepewności prowadzi do złych decyzji (np. zbyt agresywne zaufanie do mapy lub zbyt częste zatrzymania robota).

W perspektywie 2030 to nabiera dodatkowego znaczenia: systemy uczące się będą dostarczać oszacowań (np. pozy obiektu 6D) razem z niepewnością lub miarą wiarygodności. Aby taki sygnał włączyć do sterowania i bezpieczeństwa, trzeba mieć spójny aparat do przenoszenia i łączenia niepewności w \(SE(3)\). Innymi słowy: geometria + niepewność stanie się „wspólnym językiem” między percepcją a ruchem.

#### 3.14. Kontrole zdrowego rozsądku: jak szybko wykryć błąd w łańcuchu transformacji
Zanim zaczniesz „debugować algorytm”, warto wykonać kilka testów, które często natychmiast ujawniają błąd ramek:
1) **Test jednostkowy ortonormalności:** \(\| \mathbf{R}^\top\mathbf{R}-\mathbf{I}\|\) ma być bliskie zera, a \(\det(\mathbf{R})\) bliskie 1.  
2) **Test tożsamości:** \({}^A\mathbf{T}_A = \mathbf{I}\) i \({}^A\mathbf{T}_B\,{}^B\mathbf{T}_A=\mathbf{I}\).  
3) **Test punktów kontrolnych:** wybierz 2–3 punkty o łatwej interpretacji (np. „1 m do przodu”, „1 m w lewo”) i sprawdź, czy po transformacji lądują tam, gdzie intuicja podpowiada.  
4) **Test osi:** przekształć wektory jednostkowe osi ramy \(\{B\}\): \(\mathbf{e}_x,\mathbf{e}_y,\mathbf{e}_z\). Jeśli osie „nie są prostopadłe” po transformacji, to coś jest nie tak.

W dydaktyce szczególnie ważny jest test nr 2: jeśli student potrafi bezbłędnie wyprowadzić \(\mathbf{T}^{-1}\) i sprawdzić, że \(\mathbf{T}\mathbf{T}^{-1}=\mathbf{I}\), to zyskuje nawyk weryfikacji, który chroni przed wielodniowym błądzeniem w kodzie.

#### 3.15. TF w ROS2: dlaczego „map–odom–base_link” to nie fanaberia, tylko model błędu
W ROS2 (i w ekosystemie narzędzi) popularna jest struktura ramek `map -> odom -> base_link`. To nie jest przypadkowa tradycja, ale sposób na rozdzielenie dwóch rodzajów informacji:
- `odom` jest lokalnie gładna i użyteczna do sterowania (mało „skoków”), ale może dryfować w czasie,
- `map` jest globalna (może korygować dryf), ale aktualizacje mogą być skokowe (np. po domknięciu pętli w SLAM).

W tym modelu transformacja \({}^{map}\mathbf{T}_{odom}\) „absorbuje” korekty globalne, a \({}^{odom}\mathbf{T}_{base}\) niesie płynny ruch. Jeśli student zrozumie, że to jest model błędu i jego kompensacji, to łatwiej mu potem zrozumieć, dlaczego robot czasem „przeskakuje na mapie”, ale sterowanie lokalne wciąż jest stabilne.

Analogiczny problem pojawia się w manipulacji z kamerą: jeśli kamera „koryguje” pozycję obiektu skokowo, to układ sterowania chwytaka musi to obsłużyć (np. filtrując, ograniczając prędkości, uwzględniając opóźnienia). Z punktu widzenia ramek: skok w \({}^B\mathbf{T}_{obj}\) to nie tylko „nowa liczba”, ale zdarzenie, które wpływa na bezpieczeństwo ruchu.

#### 3.16. Kalibracja ręka–oko jako równanie w \(SE(3)\): \(A X = X B\)
Klasyczny problem kalibracji ręka–oko (kamera na narzędziu) można zapisać jako:
\[
\mathbf{A}_i \mathbf{X} = \mathbf{X}\mathbf{B}_i,
\]
gdzie:
- \(\mathbf{A}_i\) to ruch końcówki roboczej między dwoma pozycjami (w bazie),
- \(\mathbf{B}_i\) to odpowiadający mu ruch kamery (w jej ramie),
- \(\mathbf{X}\) to nieznana stała transformacja między kamerą a narzędziem.

To równanie pokazuje, dlaczego potrzebujemy \(SE(3)\): zarówno \(\mathbf{A}_i\), jak i \(\mathbf{B}_i\) są transformacjami sztywnymi, a rozwiązanie polega na dopasowaniu ich w sensie geometrycznym. W praktyce (zwłaszcza w systemach 2024–2025 opartych o percepcję 3D) kalibracja jest warunkiem koniecznym, aby „ucieleśnione” modele działały dobrze: model może rozpoznać obiekt, ale jeśli \(\mathbf{X}\) jest błędne, robot nie trafi w obiekt mimo „doskonałej” percepcji.

W perspektywie 2030 kalibracja nie zniknie — przeciwnie, stanie się bardziej dynamiczna (samokalibracja, kompensacja ugięć, utrzymanie w czasie), a więc formalizm ramek i transformacji będzie stale obecny w systemie, a nie tylko na etapie uruchomienia.

#### 3.17. Implementacja w kodzie: przechowuj to, co minimalne, ale licz to, co jednoznaczne
W praktycznych bibliotekach spotkasz kilka sposobów reprezentacji pozy:
- jako macierz \(\mathbf{T}\in\mathbb{R}^{4\times 4}\),
- jako para \((\mathbf{R},\mathbf{p})\),
- jako \((q,\mathbf{p})\) z kwaternionem jednostkowym.

W systemach czasu rzeczywistego często przechowuje się \((q,\mathbf{p})\), bo jest to minimalne pamięciowo i wydajne, a macierz \(\mathbf{T}\) tworzy się „na żądanie” do mnożenia lub wizualizacji. Kluczowe jest jednak, aby niezależnie od formatu:
- normalizować kwaternion \(q\) (lub rzutować \(\mathbf{R}\) na \(SO(3)\)),
- nie mieszać konwencji (kolejności mnożenia, ramy prędkości),
- jasno nazywać transformacje (np. `T_A_B` jako „z B do A”).

Doświadczenie inżynierskie pokazuje, że nazewnictwo jest elementem bezpieczeństwa: błędnie nazwana transformacja jest trudniejsza do wykrycia niż błąd kompilacji. Dlatego w projektach (zwłaszcza zespołowych) przyjmuje się sztywne standardy: jednoznaczne nazwy ramek, jednoznaczna konwencja kierunku transformacji i testy niezmienników (por. §3.14). To jest też most do cyfrowych bliźniaków: jeśli „świat w symulacji” i „świat w robocie” mają identyczny graf ramek, łatwiej jest przenosić modele, walidować zachowanie i diagnozować odchylenia.

#### 3.18. Podsumowanie
Transformacje jednorodne w \(SE(3)\) są językiem, który spina całą robotykę: opisuje geometrię, umożliwia składanie łańcuchów kinematycznych, porządkuje percepcję i kalibrację, a w praktyce staje się „kontraktem” między modułami systemu. W kolejnych wykładach wykorzystamy ten język do budowy kinematyki manipulatorów, a w części o percepcji — do spójnej integracji czujników i map.

Warto potraktować ten wykład jako inwestycję: przez chwilę pracujemy z notacją, która może wydawać się formalna, ale dzięki niej unikamy niejawnych założeń. Jeśli w projekcie potrafisz w jednym miejscu odpowiedzieć na pytania „w jakiej ramie jest ten wektor?”, „z jakiej ramy do jakiej mapuje ta macierz?” oraz „czy to jest punkt czy kierunek?”, to większość błędów integracyjnych znika. Do 2030, wraz z rosnącą złożonością systemów (floty robotów, cyfrowe bliźniaki, współdzielona autonomia), ta umiejętność stanie się jednym z kluczowych kompetencyjnych wyróżników.

W następnych częściach kursu będziemy też ćwiczyć „czytanie” zapisu \({}^A\mathbf{T}_B\) tak, aby stał się on nawykiem, a nie obciążeniem.

Na koniec: jeżeli w jakimkolwiek miejscu projektu masz wątpliwość, narysuj osie i przeprowadź punkt testowy — geometria jest bezlitosna, ale bardzo uczciwa.

To jedna z najpewniejszych metod szybkiej diagnozy błędów w robotyce w praktyce.

**Odniesienia w tekście:** Lynch i Park (2017), Corke (2017), Craig (2004), Barfoot (2017), dokumentacja TF/ROS2 (w tym konwencje ramek), NIST AI RMF (2023) oraz raporty trendów (IFR *World Robotics* 2023–2025 jako tło wdrożeń).

### 4. Struktura prezentacji slajdów (PowerPoint / Google Slides)
**Założenie stylu:** Assertion–Evidence — tytuł slajdu jest tezą; treść to dowód (rysunek/animacja/wykres); tekst minimalny.

1. **Slajd nr 1 | 80% błędów integracji to błędy ramek odniesienia**  
   - Treść:  
     - „W jakiej ramie jest ten wektor?”  
     - „Z jakiej do jakiej mapuje ta transformacja?”  
     - „Punkt czy kierunek?”  
   - Sugestie wizualne: kolaż 3 mini-przypadków: (a) kamera obrócona o 90°, (b) IMU z osią \(z\) w dół, (c) `map->odom->base_link` z przeskokiem SLAM; na każdym czerwony znak „frame mismatch”.  
   - Notatki dla prowadzącego: 2–3 min; zaznaczyć, że to wykład „anty-błędowy”.

2. **Slajd nr 2 | Notacja \({}^A\mathbf{p}\) natychmiast mówi, co jest w jakiej ramie**  
   - Treść:  
     - \({}^A\mathbf{p}\): współrzędne punktu w \(\{A\}\)  
     - \({}^A\mathbf{R}_B\): obrót z \(\{B\}\) do \(\{A\}\)  
     - \({}^A\mathbf{T}_B\): transformacja z \(\{B\}\) do \(\{A\}\)  
   - Sugestie wizualne: rysunek dwóch ramek A i B + wektor punktu; podpisy „opis” vs „obiekt”.  
   - Notatki: 3 min; poprosić studentów o głośne czytanie \({}^A\mathbf{T}_B\) („z B do A”).

3. **Slajd nr 3 | Transformacja sztywna to obrót + przesunięcie**  
   - Treść:  
     - \({}^A\mathbf{p} = {}^A\mathbf{R}_B\,{}^B\mathbf{p}+{}^A\mathbf{p}_B\)  
   - Sugestie wizualne: animacja: punkt w ramie B → obrót do A → dodanie przesunięcia; strzałki w dwóch kolorach (obrót, translacja).  
   - Notatki: 2 min; podkreślić kolejność: najpierw obrót, potem przesunięcie.

4. **Slajd nr 4 | Współrzędne jednorodne zamieniają „+” na mnożenie macierzy**  
   - Treść:  
     - \({}^A\tilde{\mathbf{p}} = {}^A\mathbf{T}_B\,{}^B\tilde{\mathbf{p}}\)  
     - \(\mathbf{T}=\begin{bmatrix}\mathbf{R}&\mathbf{p}\\0&1\end{bmatrix}\)  
   - Sugestie wizualne: duża macierz \(\mathbf{T}\) i wektor 4×1; obok „łańcuch” mnożeń jako „klocki Lego”.  
   - Notatki: 3 min; zaznaczyć, że to umożliwia kinematykę łańcuchów.

5. **Slajd nr 5 | \(SE(3)\) jest grupą: ruchy składa się przez mnożenie, nie dodawanie**  
   - Treść:  
     - \({}^A\mathbf{T}_C = {}^A\mathbf{T}_B\,{}^B\mathbf{T}_C\)  
     - „Dodawanie” \(\mathbf{T}\) nie ma sensu geometrycznego  
   - Sugestie wizualne: animacja 3 ramek A–B–C; strzałki jako krawędzie grafu; mnożenie jako „złożenie krawędzi”.  
   - Notatki: 2 min; podać intuicję: „najpierw C→B, potem B→A”.

6. **Slajd nr 6 | Odwrotność transformacji to obrót transponowany i przesunięcie w innej ramie**  
   - Treść:  
     - \(\mathbf{T}^{-1}=\begin{bmatrix}\mathbf{R}^\top&-\mathbf{R}^\top\mathbf{p}\\0&1\end{bmatrix}\)  
   - Sugestie wizualne: dwa panele: „intuicja błędna” (tylko minus na \(\mathbf{p}\)) vs „poprawnie” (minus + obrót); przykład geometryczny na osi 2D.  
   - Notatki: 3 min; pytanie: „dlaczego trzeba \(\mathbf{R}^\top\) przy \(\mathbf{p}\)?”.

7. **Slajd nr 7 | Punkt ma ostatnią składową 1, a wektor kierunku 0 — to usuwa błędy**  
   - Treść:  
     - \(\tilde{\mathbf{p}}=[\mathbf{p};1]\)  
     - \(\tilde{\mathbf{d}}=[\mathbf{d};0]\)  
   - Sugestie wizualne: demonstracja: ta sama \(\mathbf{T}\) zastosowana do punktu i do wektora; translacja działa tylko na punkt.  
   - Notatki: 2 min; połączyć z normalnymi i osiami czujników.

8. **Slajd nr 8 | Graf ramek (TF) jest modelem systemu, nie tylko wizualizacją**  
   - Treść:  
     - Węzły = ramy, krawędzie = \(\mathbf{T}\)  
     - Zapytanie: „jaka jest \({}^W\mathbf{T}_C\)?”  
   - Sugestie wizualne: zrzut stylizowany `rqt_tf_tree`: `map -> odom -> base_link -> camera_link -> optical_frame` z wyróżnieniem ścieżki.  
   - Notatki: 3 min; wyjaśnić, że graf wymusza spójność.

9. **Slajd nr 9 | `map->odom->base_link` rozdziela płynność sterowania od korekt globalnych**  
   - Treść:  
     - `odom`: gładna, dryfuje  
     - `map`: globalna, może skakać  
   - Sugestie wizualne: dwa wykresy pozy w czasie: linia `odom` płynna, `map` z korektą skokową; obok schemat trzech ramek.  
   - Notatki: 3 min; powiązać z SLAM (Wykład 11).

10. **Slajd nr 10 | Adiunkcja opisuje zmianę ramy dla skrętów i prędkości w \(\mathbb{R}^6\)**  
   - Treść:  
     - \({}^A\boldsymbol{\xi}=\mathrm{Ad}({}^A\mathbf{T}_B){}^B\boldsymbol{\xi}\)  
   - Sugestie wizualne: manipulator z dwiema ramami i strzałkami \(\boldsymbol{\omega}\) i \(\mathbf{v}\); overlay „przenoszenie prędkości zależy od \(\mathbf{p}\)”.  
   - Notatki: 2 min; podkreślić: to wróci przy Jacobianie.

11. **Slajd nr 11 | Niepewność pozy też „żyje” w \(SE(3)\): kowariancję przenosi adiunkcja**  
   - Treść:  
     - \(\Sigma_A=\mathrm{Ad}(\mathbf{T})\Sigma_B\mathrm{Ad}(\mathbf{T})^\top\)  
     - Bez tego fuzja czujników jest błędna  
   - Sugestie wizualne: elipsoida niepewności dla pozy i orientacji; strzałka „zmiana ramy” i elipsoida po transformacji.  
   - Notatki: 3 min; intuicja: „niepewność też ma ramę”.

12. **Slajd nr 12 | Kalibracja to znalezienie stałej transformacji między ramami**  
   - Treść:  
     - Szukamy \({}^B\mathbf{T}_C\)  
     - Błędy montażu → błąd w całym łańcuchu  
   - Sugestie wizualne: robot + kamera na uchwycie; overlay wektora błędu „kilka mm/stopni” i skutek: „kilka cm” na końcówce.  
   - Notatki: 2 min; pokaz „wzmacniania błędów” przez długi łańcuch.

13. **Slajd nr 13 | Ręka–oko: równanie \(A X = X B\) ujawnia, że kalibracja jest problemem w \(SE(3)\)**  
   - Treść:  
     - \(\mathbf{A}_i\mathbf{X}=\mathbf{X}\mathbf{B}_i\)  
   - Sugestie wizualne: diagram dwóch ruchów: ruch narzędzia i odpowiadający ruch kamery; \(\mathbf{X}\) jako stała krawędź łącząca grafy.  
   - Notatki: 3 min; zaznaczyć, że to fundament „kamera na narzędziu”.

14. **Slajd nr 14 | Pułapki: konwencje osi (ENU/NED) i „kamera optyczna”**  
   - Treść:  
     - Różne tradycje osi  
     - Zawsze rysuj osie + punkt testowy  
   - Sugestie wizualne: trzy układy osi obok siebie: robot (x forward), lotniczy NED, kamera (z optical); czerwone strzałki pokazujące, gdzie zwykle myli się osie.  
   - Notatki: 3 min; przypomnieć o REP 103 i `*_optical_frame`.

15. **Slajd nr 15 | Kontrole zdrowego rozsądku łapią większość błędów w minutę**  
   - Treść:  
     - \(\det(\mathbf{R})\approx 1\)  
     - \(\mathbf{R}^\top\mathbf{R}\approx \mathbf{I}\)  
     - \(\mathbf{T}\mathbf{T}^{-1}=\mathbf{I}\)  
   - Sugestie wizualne: „panel diagnostyczny” z 3 wskaźnikami (zielony/czerwony) + przykład, jak wykres nagle robi się czerwony po błędzie znaku.  
   - Notatki: 2 min; zachęcić do pisania testów.

16. **Slajd nr 16 | Przykład: transformacja punktu między ramami to 2 linie kodu (jeśli notacja jest poprawna)**  
   - Treść:  
     - \({}^A\tilde{\mathbf{p}} = {}^A\mathbf{T}_B\,{}^B\tilde{\mathbf{p}}\)  
   - Sugestie wizualne: zrzut kodu (2–3 linie) + obok rysunek dwóch ramek; podświetlenie, że „1” i „0” w jednorodnych robi różnicę.  
   - Notatki: 2 min; przygotowanie do przykładów rachunkowych.

17. **Slajd nr 17 | Cyfrowy bliźniak wymaga identycznego grafu ramek w symulacji i rzeczywistości**  
   - Treść:  
     - Spójna geometria = ta sama semantyka  
     - Walidacja i diagnostyka łatwiejsza  
   - Sugestie wizualne: dwa panele: „symulacja” i „robot” z identycznym drzewem ramek; strzałka „porównanie transformacji w czasie”.  
   - Notatki: 3 min; odwołać się do Części 2 (cyfrowe bliźniaki).

18. **Slajd nr 18 | Do 2030 uczenie i percepcja będą zwracać pozy 6D — musisz je poprawnie włączyć do \(SE(3)\)**  
   - Treść:  
     - Estymaty pozy + niepewność  
     - Metryki na \(SO(3)\), nie na składowych  
   - Sugestie wizualne: obiekt na stole + overlay estymowanej ramy obiektu; obok „pasek wiarygodności” i elipsoida niepewności.  
   - Notatki: 2 min; połączyć z walidacją i bezpieczeństwem.

19. **Slajd nr 19 | Podsumowanie: jedna konwencja + testy = spójny system robotyczny**  
   - Treść:  
     - \({}^A\mathbf{T}_B\) konsekwentnie  
     - Punkty vs kierunki  
     - TF jako model systemu  
   - Sugestie wizualne: „ściąga” 3 reguł na jednej planszy + mały graf ramek.  
   - Notatki: 2 min; przejście: „to będzie baza pod kinematykę DH”.

20. **Slajd nr 20 | Most do kolejnych wykładów: \(SE(3)\) → kinematyka łańcuchów i Jacobian**  
   - Treść:  
     - Składanie \(\mathbf{T}\) w manipulatorze  
     - Różniczkowanie \(\rightarrow\) Jacobian  
   - Sugestie wizualne: rysunek manipulatora z kolejnymi ramami na ogniwach; obok schemat „\(\mathbf{T}\) mnożenie” → „\(\dot{\mathbf{T}}\)” → „Jacobian”.  
   - Notatki: 2 min; zamknąć wykład i zapowiedzieć Wykład 04.

### 5. Przykłady i studia przypadku (minimum 5 szczegółowych)
#### Przykład 1: Transformacja punktu z ramy kamery do ramy bazy robota
**Opis problemu.** Kamera \(\{C\}\) jest zamocowana na robocie. Znana jest transformacja \({}^B\mathbf{T}_C\), gdzie:
\[
{}^B\mathbf{R}_C = \mathbf{R}_z(90^\circ),\qquad {}^B\mathbf{p}_C=\begin{bmatrix}0.20\\0.00\\0.50\end{bmatrix}\ \text{m}.
\]
Punkt wykryty przez kamerę ma współrzędne \({}^C\mathbf{p}=\begin{bmatrix}0.10\\0.00\\1.00\end{bmatrix}\ \text{m}\). Oblicz \({}^B\mathbf{p}\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Definicja:
\[
{}^B\mathbf{p} = {}^B\mathbf{R}_C\,{}^C\mathbf{p} + {}^B\mathbf{p}_C.
\]
2) Dla \(\mathbf{R}_z(90^\circ)\) mamy:
\[
\mathbf{R}_z(90^\circ)=
\begin{bmatrix}
0 & -1 & 0\\
1 & 0 & 0\\
0 & 0 & 1
\end{bmatrix}.
\]
3) Mnożenie:
\[
{}^B\mathbf{R}_C\,{}^C\mathbf{p}=
\begin{bmatrix}
0 & -1 & 0\\
1 & 0 & 0\\
0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
0.10\\0\\1.00
\end{bmatrix}
=
\begin{bmatrix}
0\\0.10\\1.00
\end{bmatrix}.
\]
4) Dodajemy przesunięcie kamery:
\[
{}^B\mathbf{p}=
\begin{bmatrix}
0\\0.10\\1.00
\end{bmatrix}
+
\begin{bmatrix}
0.20\\0\\0.50
\end{bmatrix}
=
\begin{bmatrix}
0.20\\0.10\\1.50
\end{bmatrix}\ \text{m}.
\]

**Kod (Python 3.11, NumPy).**
```python
import numpy as np

def Rz(deg):
    a = np.deg2rad(deg)
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, -s, 0],
                     [s,  c, 0],
                     [0,  0, 1]], float)

R_BC = Rz(90.0)
p_BC = np.array([0.20, 0.00, 0.50])
p_C = np.array([0.10, 0.00, 1.00])

p_B = R_BC @ p_C + p_BC
print(p_B)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Rysunek 3D: rama bazy \(\{B\}\), rama kamery \(\{C\}\) przesunięta o (0.2,0,0.5) i obrócona o 90° wokół \(z\); punkt \({}^C\mathbf{p}\) jako strzałka w osi optycznej; wynikowy punkt w \(\{B\}\) zaznaczony w przestrzeni.

---

#### Przykład 2: Składanie transformacji — obliczenie \({}^W\mathbf{T}_C\) i transformacja punktu
**Opis problemu.** Znane są:
\[
{}^W\mathbf{T}_B=
\begin{bmatrix}
\mathbf{R}_z(30^\circ) & \begin{bmatrix}1\\2\\0\end{bmatrix}\\
0 & 1
\end{bmatrix},
\qquad
{}^B\mathbf{T}_C=
\begin{bmatrix}
\mathbf{R}_y(20^\circ) & \begin{bmatrix}0.2\\0\\0.5\end{bmatrix}\\
0 & 1
\end{bmatrix}.
\]
Wyznacz \({}^W\mathbf{T}_C\) oraz położenie w świecie punktu \({}^C\mathbf{p}=[0,0,1]^\top\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Składanie:
\[
{}^W\mathbf{T}_C = {}^W\mathbf{T}_B\,{}^B\mathbf{T}_C.
\]
2) Części składowe:
\[
{}^W\mathbf{R}_C = {}^W\mathbf{R}_B\,{}^B\mathbf{R}_C,\qquad
{}^W\mathbf{p}_C = {}^W\mathbf{R}_B\,{}^B\mathbf{p}_C + {}^W\mathbf{p}_B.
\]
3) Punkt:
\[
{}^W\mathbf{p} = {}^W\mathbf{R}_C\,{}^C\mathbf{p} + {}^W\mathbf{p}_C.
\]

**Kod (Python 3.11).**
```python
import numpy as np

def Rx(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[1,0,0],[0,c,-s],[0,s,c]], float)
def Ry(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c,0,s],[0,1,0],[-s,0,c]], float)
def Rz(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c,-s,0],[s,c,0],[0,0,1]], float)

R_WB = Rz(np.deg2rad(30.0))
p_WB = np.array([1.0, 2.0, 0.0])
R_BC = Ry(np.deg2rad(20.0))
p_BC = np.array([0.2, 0.0, 0.5])

R_WC = R_WB @ R_BC
p_WC = R_WB @ p_BC + p_WB

p_C = np.array([0.0, 0.0, 1.0])
p_W = R_WC @ p_C + p_WC

print("R_WC=\n", R_WC)
print("p_WC=", p_WC)
print("p_W =", p_W)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Trzy ramy: \(\{W\}\), \(\{B\}\), \(\{C\}\) z pokazaniem kolejności składania. Dodatkowo strzałka punktu w ramie \(\{C\}\) (o długości 1 m) oraz jego obraz w świecie.

---

#### Przykład 3: Odwrotność transformacji i test \(\mathbf{T}\mathbf{T}^{-1}=\mathbf{I}\)
**Opis problemu.** Dana jest transformacja:
\[
\mathbf{T}=
\begin{bmatrix}
\mathbf{R}_x(90^\circ) & \begin{bmatrix}0\\1\\0\end{bmatrix}\\
0 & 1
\end{bmatrix}.
\]
Wyznacz \(\mathbf{T}^{-1}\) i sprawdź, że \(\mathbf{T}\mathbf{T}^{-1}=\mathbf{I}\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Dla \(\mathbf{T}=\begin{bmatrix}\mathbf{R}&\mathbf{p}\\0&1\end{bmatrix}\):
\[
\mathbf{T}^{-1}=
\begin{bmatrix}
\mathbf{R}^\top & -\mathbf{R}^\top\mathbf{p}\\
0 & 1
\end{bmatrix}.
\]
2) Dla \(\mathbf{R}=\mathbf{R}_x(90^\circ)\):
\[
\mathbf{R}^\top=\mathbf{R}_x(-90^\circ).
\]
3) Przesunięcie:
\[
-\mathbf{R}^\top\mathbf{p}
=
-\mathbf{R}_x(-90^\circ)\begin{bmatrix}0\\1\\0\end{bmatrix}.
\]

**Kod (Python 3.11).**
```python
import numpy as np

def Rx(deg):
    a = np.deg2rad(deg)
    c, s = np.cos(a), np.sin(a)
    return np.array([[1,0,0],[0,c,-s],[0,s,c]], float)

R = Rx(90.0)
p = np.array([0.0, 1.0, 0.0])

T = np.eye(4)
T[:3,:3] = R
T[:3,3] = p

T_inv = np.eye(4)
T_inv[:3,:3] = R.T
T_inv[:3,3] = -R.T @ p

print("T_inv=\n", T_inv)
print("T*T_inv=\n", T @ T_inv)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Rysunek 3D: rama po transformacji \(\mathbf{T}\) oraz rama po \(\mathbf{T}^{-1}\); obok „macierz tożsamości” jako wynik mnożenia i wskaźnik błędu numerycznego \(\|\mathbf{T}\mathbf{T}^{-1}-\mathbf{I}\|_F\).

---

#### Przykład 4: `map–odom–base_link` — obliczenie pozy bazy i efekt korekty SLAM
**Opis problemu.** W pewnej chwili system ma:
- \({}^{map}\mathbf{T}_{odom}\): korekta globalna z SLAM,  
- \({}^{odom}\mathbf{T}_{base}\): płynna odometria.

Przyjmij:
\[
{}^{map}\mathbf{T}_{odom}=
\begin{bmatrix}
\mathbf{R}_z(5^\circ) & \begin{bmatrix}0.50\\-0.20\\0\end{bmatrix}\\
0 & 1
\end{bmatrix},
\quad
{}^{odom}\mathbf{T}_{base}=
\begin{bmatrix}
\mathbf{R}_z(30^\circ) & \begin{bmatrix}2.0\\1.0\\0\end{bmatrix}\\
0 & 1
\end{bmatrix}.
\]
Wyznacz \({}^{map}\mathbf{T}_{base}\). Następnie załóż, że SLAM „domknął pętlę” i zmienił \({}^{map}\mathbf{T}_{odom}\) na przesunięcie \([0.60,-0.10,0]^\top\) (reszta bez zmian). Jak zmienia się \({}^{map}\mathbf{p}_{base}\)?

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Składanie:
\[
{}^{map}\mathbf{T}_{base} = {}^{map}\mathbf{T}_{odom}\,{}^{odom}\mathbf{T}_{base}.
\]
2) Skutek korekty SLAM: zmienia się tylko \({}^{map}\mathbf{T}_{odom}\), więc cała globalna pozycja bazy przesuwa się zgodnie z tą korektą (to jest zamierzony efekt).

**Kod (Python 3.11).**
```python
import numpy as np

def Rz(deg):
    a = np.deg2rad(deg)
    c, s = np.cos(a), np.sin(a)
    return np.array([[c,-s,0],[s,c,0],[0,0,1]], float)

def T(R, p):
    out = np.eye(4)
    out[:3,:3] = R
    out[:3,3] = p
    return out

T_map_odom = T(Rz(5.0), np.array([0.50, -0.20, 0.0]))
T_odom_base = T(Rz(30.0), np.array([2.0, 1.0, 0.0]))

T_map_base = T_map_odom @ T_odom_base
print("p_map_base =", T_map_base[:3,3])

T_map_odom2 = T(Rz(5.0), np.array([0.60, -0.10, 0.0]))
T_map_base2 = T_map_odom2 @ T_odom_base
print("p_map_base (after loop closure) =", T_map_base2[:3,3])
print("delta =", T_map_base2[:3,3] - T_map_base[:3,3])
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Wykres 2D trajektorii w `odom` (gładna) oraz trajektorii w `map` (z nagłym przesunięciem w chwili domknięcia pętli). Obok graf ramek `map->odom->base_link` z podświetleniem krawędzi, która się zmienia.

---

#### Przykład 5: Propagacja niepewności pozy przez adiunkcję \(\Sigma_A=\mathrm{Ad}(\mathbf{T})\Sigma_B\mathrm{Ad}(\mathbf{T})^\top\)
**Opis problemu.** Czujnik w ramie \(\{C\}\) dostarcza perturbacji \(\delta\boldsymbol{\xi}=[\delta\boldsymbol{\omega};\delta\mathbf{v}]\) z kowariancją:
\[
\Sigma_C=\mathrm{diag}([0.01,0.01,0.02,\ 0.04,0.04,0.09]).
\]
Chcemy wyrazić tę niepewność w ramie bazy \(\{B\}\). Dana jest transformacja \({}^B\mathbf{T}_C\) z:
\[
{}^B\mathbf{R}_C=\mathbf{I},\qquad {}^B\mathbf{p}_C=\begin{bmatrix}0.2\\0\\0.5\end{bmatrix}.
\]
Oblicz \(\Sigma_B\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Adiunkcja:
\[
\mathrm{Ad}(\mathbf{T})=
\begin{bmatrix}
\mathbf{R} & 0\\
[\mathbf{p}]_\times\mathbf{R} & \mathbf{R}
\end{bmatrix}.
\]
2) Ponieważ \(\mathbf{R}=\mathbf{I}\), upraszcza się do:
\[
\mathrm{Ad}(\mathbf{T})=
\begin{bmatrix}
\mathbf{I} & 0\\
[\mathbf{p}]_\times & \mathbf{I}
\end{bmatrix}.
\]
3) Propagacja:
\[
\Sigma_B=\mathrm{Ad}(\mathbf{T})\Sigma_C\mathrm{Ad}(\mathbf{T})^\top.
\]

**Kod (Python 3.11, NumPy).**
```python
import numpy as np

def hat(p):
    px, py, pz = p
    return np.array([[0, -pz, py],
                     [pz, 0, -px],
                     [-py, px, 0]], float)

p = np.array([0.2, 0.0, 0.5])
Ad = np.block([
    [np.eye(3), np.zeros((3,3))],
    [hat(p),    np.eye(3)]
])

Sigma_C = np.diag([0.01,0.01,0.02, 0.04,0.04,0.09])
Sigma_B = Ad @ Sigma_C @ Ad.T

print("Sigma_B=\n", Sigma_B)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Dwie elipsoidy niepewności: w ramie czujnika \(\{C\}\) i w ramie bazy \(\{B\}\). Zaznaczyć, że część translacyjna miesza się z obrotową przez \([\mathbf{p}]_\times\) (pojawiają się wyrazy poza przekątną).

### 6. Materiały dla studentów
#### 6 pytań teoretycznych (z oczekiwanymi odpowiedziami)
1) **Pytanie:** Co oznacza zapis \({}^A\mathbf{T}_B\)?  
   **Odpowiedź:** Transformację jednorodną mapującą współrzędne z ramy \(\{B\}\) do ramy \(\{A\}\).
2) **Pytanie:** Podaj wzór na transformację odwrotną \(\mathbf{T}^{-1}\) i wyjaśnij, skąd bierze się składnik \(-\mathbf{R}^\top\mathbf{p}\).  
   **Odpowiedź:** \(\mathbf{T}^{-1}=\begin{bmatrix}\mathbf{R}^\top&-\mathbf{R}^\top\mathbf{p}\\0&1\end{bmatrix}\); przesunięcie musi być wyrażone w ramie po odwróceniu obrotu.
3) **Pytanie:** Jaka jest różnica między transformacją punktu a transformacją wektora kierunku?  
   **Odpowiedź:** Punkt podlega obrotowi i translacji; wektor kierunku tylko obrotowi. W jednorodnych: punkt ma ostatnią składową 1, kierunek 0.
4) **Pytanie:** Dlaczego w ROS2 stosuje się strukturę `map->odom->base_link`?  
   **Odpowiedź:** Aby rozdzielić płynną odometrię (dobrą do sterowania, ale dryfującą) od globalnych korekt (mapa/SLAM), które mogą być skokowe.
5) **Pytanie:** Co opisuje macierz adiunkcji \(\mathrm{Ad}(\mathbf{T})\)?  
   **Odpowiedź:** Zmianę ramy dla skrętów/prędkości w \(\mathbb{R}^6\) (i w konsekwencji dla ich kowariancji).
6) **Pytanie:** Podaj dwa typowe symptomy pomylenia konwencji osi (np. ENU vs NED).  
   **Odpowiedź:** Ruch „w złą stronę” po obrocie robota; grawitacja z IMU o złym znaku; błędne pochylenie w wizualizacji; niezgodność między czujnikami.

#### 4 zadania obliczeniowe/programistyczne (z poziomem trudności)
1) **(Łatwe)** Dla zadanych \(\mathbf{R}\) i \(\mathbf{p}\) zbuduj \(\mathbf{T}\), policz \(\mathbf{T}^{-1}\) i sprawdź numerycznie \(\|\mathbf{T}\mathbf{T}^{-1}-\mathbf{I}\|_F\).  
2) **(Średnie)** Zaimplementuj funkcję `transform_point(T, p)` oraz `transform_direction(T, d)` i pokaż na przykładach, że translacja nie wpływa na `direction`.  
3) **(Średnie)** Zbuduj prosty graf ramek (4–6 ramek) i napisz funkcję wyszukującą ścieżkę oraz składającą transformację między dowolnymi dwiema ramami.  
4) **(Trudne)** Zasymuluj `map->odom->base_link` dla robota 2D: generuj odometrię z dryfem i co pewien czas dodawaj korektę `map->odom`; narysuj trajektorie w `map` i `odom`.

#### 1 projekt laboratoryjny / projekt domowy (z kryteriami oceny)
**Projekt:** „Kalibracja i graf ramek dla robota z kamerą”.  
**Wymagania:**  
- Zdefiniuj ramy: `map`, `odom`, `base_link`, `camera_link`, `camera_optical_frame`, `tool0` (jeśli dotyczy).  
- Zbuduj spójny graf i opisz wszystkie transformacje (co oznaczają, w jakich jednostkach, jak mierzone).  
- Zaproponuj procedurę kalibracji \({}^B\mathbf{T}_C\) (metoda, dane, walidacja punktami kontrolnymi).  
- Zaproponuj zestaw testów automatycznych (niezmienniki \(\mathbf{R}\), test punktów kontrolnych, test spójności odwrotności).  
**Kryteria oceny (100 pkt):** spójność notacji i ramek (35), realizm procedury kalibracji (25), jakość testów i walidacji (25), klarowność dokumentacji (15).

### 7. Quiz sprawdzający (15 pytań: 10 wyboru wielokrotnego + 5 otwartych + klucz odpowiedzi)
#### 10 pytań wyboru wielokrotnego
1) Zapis \({}^A\mathbf{T}_B\) oznacza transformację:  
   A) z A do B, B) z B do A, C) z A do A, D) z B do B  
2) Dla \(\mathbf{T}=\begin{bmatrix}\mathbf{R}&\mathbf{p}\\0&1\end{bmatrix}\) odwrotność ma postać:  
   A) \(\begin{bmatrix}\mathbf{R}&-\mathbf{p}\\0&1\end{bmatrix}\), B) \(\begin{bmatrix}\mathbf{R}^\top&-\mathbf{R}^\top\mathbf{p}\\0&1\end{bmatrix}\), C) \(\begin{bmatrix}\mathbf{R}^{-1}&-\mathbf{p}\\0&1\end{bmatrix}\), D) \(\mathbf{T}\) nie ma odwrotności  
3) Składanie transformacji \({}^A\mathbf{T}_C = {}^A\mathbf{T}_B\,{}^B\mathbf{T}_C\) odpowiada:  
   A) dodawaniu przesunięć, B) mnożeniu macierzy, C) transpozycji, D) wyznacznikowi  
4) Wektor kierunku \(\mathbf{d}\) transformuje się między ramami jako:  
   A) \(\mathbf{R}\mathbf{d}+\mathbf{p}\), B) \(\mathbf{R}\mathbf{d}\), C) \(\mathbf{d}+\mathbf{p}\), D) \(\mathbf{R}^\top\mathbf{d}+\mathbf{p}\)  
5) Współrzędne jednorodne punktu mają ostatnią składową:  
   A) 0, B) 1, C) \(\det(\mathbf{R})\), D) \(\mathrm{tr}(\mathbf{R})\)  
6) Graf TF w ROS2 reprezentuje:  
   A) tylko wizualizację, B) spójny model zależności ramek i transformacji, C) wyłącznie mapę 2D, D) sterowanie PID  
7) W strukturze `map->odom->base_link` rama `odom` jest zwykle:  
   A) globalna bez skoków, B) lokalnie gładka, ale może dryfować, C) nieruchoma, D) zawsze identyczna z `map`  
8) Adiunkcja \(\mathrm{Ad}(\mathbf{T})\) służy do:  
   A) zmiany ramy punktów w \(\mathbb{R}^3\), B) zmiany ramy skrętów/prędkości w \(\mathbb{R}^6\), C) liczenia wyznacznika, D) filtrowania szumu  
9) Jeśli \(\mathbf{R}\) „uciekła” z \(SO(3)\), to najprostszy test to:  
   A) \(\mathbf{R}+\mathbf{R}^\top\), B) \(\mathbf{R}^\top\mathbf{R}\approx\mathbf{I}\), C) \(\|\mathbf{p}\|\), D) \(\mathbf{R}^{-1}=\mathbf{R}\)  
10) Typowy błąd konwencji osi ujawnia się, gdy:  
   A) robot stoi w miejscu, B) robot wykonuje obrót i nagle „odwraca się” interpretacja kierunków, C) rośnie MTBF, D) zmniejsza się \(T_c\)

#### 5 pytań otwartych
11) Wyjaśnij różnicę między aktywną i pasywną interpretacją transformacji.  
12) Podaj przykład, w którym pomylenie punktu i wektora kierunku powoduje błąd (i jak go wykryć).  
13) Opisz procedurę „punktów kontrolnych” do weryfikacji transformacji kamery względem bazy.  
14) Dlaczego korekta SLAM może powodować skoki w `map`, ale nie powinna destabilizować sterowania lokalnego?  
15) Jak i po co propaguje się kowariancję przez \(\mathrm{Ad}(\mathbf{T})\)?

#### Klucz odpowiedzi
1) B, 2) B, 3) B, 4) B, 5) B, 6) B, 7) B, 8) B, 9) B, 10) B.  
11–15) Oceniane opisowo: poprawność + przykłady i spójna notacja.

### 8. Bibliografia i materiały dodatkowe
1) K. M. Lynch, F. C. Park, *Modern Robotics: Mechanics, Planning, and Control*, Cambridge University Press, 2017.  
2) P. Corke, *Robotics, Vision and Control*, 2nd ed., Springer, 2017.  
3) J. J. Craig, *Introduction to Robotics: Mechanics and Control*, 3rd ed., Pearson, 2004.  
4) T. D. Barfoot, *State Estimation for Robotics*, Cambridge University Press, 2017 (ramy, grupy Liego i niepewność).  
5) R. M. Murray, Z. Li, S. S. Sastry, *A Mathematical Introduction to Robotic Manipulation*, CRC Press, 1994 (formalizm \(SE(3)\), skręty).  
6) B. Siciliano, O. Khatib (red.), *Springer Handbook of Robotics*, 2nd ed., Springer, 2016 (kontekst systemowy).  
7) ROS 2 / TF2 Documentation — transformacje ramek, graf TF, dobre praktyki (w tym konwencje ramek i `*_optical_frame`).  
8) ROS Enhancement Proposal REP 103 — konwencje układów odniesienia i jednostek w ROS (ramy prawoskrętne, ENU).  
9) ISO 10218-1/10218-2 — bezpieczeństwo robotów przemysłowych (spójność modeli i walidacja stanowiska).  
10) NIST, *AI Risk Management Framework (AI RMF 1.0)*, 2023 (walidacja i zarządzanie ryzykiem dla systemów adaptacyjnych).  
11) International Federation of Robotics (IFR), *World Robotics* — raporty 2023–2025 (tło wdrożeń i trendów do 2030).  
12) Stanford University, *AI Index Report* — wydania 2024–2025 (kontekst rozwoju AI istotny dla robotyki i bliźniaków).  
13) NVIDIA Isaac Sim / Omniverse Documentation — cyfrowe bliźniaki i spójność ramek w symulacji (wydania 2023–2025).  
14) Materiały dydaktyczne: zestawy ćwiczeń z \(SE(3)\) (np. zasoby towarzyszące książce *Modern Robotics*).  
15) Materiały wideo: wykłady o kinematyce i transformacjach (K. Lynch) oraz seminaria o estymacji na grupach Liego (na bazie Barfoot).
