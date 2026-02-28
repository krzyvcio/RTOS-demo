# Wykład 04

### 1. Tytuł wykładu
Kinematyka prosta manipulatorów: konwencja Denavita–Hartenberga i modele geometryczne

### 2. Cele nauczania (5–7 punktów)
- Student zrozumie, czym jest kinematyka prosta (FK) manipulatora i jakie dane są potrzebne do wyznaczenia pozy końcówki roboczej.
- Student będzie potrafił przypisać ramy Denavita–Hartenberga do łańcucha kinematycznego oraz poprawnie wyznaczyć parametry \(a_i,\alpha_i,d_i,\theta_i\).
- Student będzie potrafił zbudować macierze \({}^{i-1}\mathbf{T}_i\) i złożyć je w \({}^0\mathbf{T}_n\) dla manipulatora o wielu przegubach.
- Student zrozumie różnice między klasyczną i zmodyfikowaną konwencją DH oraz typowe źródła błędów (osie, znaki, kolejność obrotów).
- Student będzie potrafił zweryfikować model kinematyczny testami zdrowego rozsądku (punkty kontrolne, ograniczenia, niezmienniki \(SO(3)\)).
- Student pozna, jak modele FK są używane w praktyce (URDF/ROS2, MoveIt2, symulacja i cyfrowe bliźniaki) oraz dlaczego do 2030 rośnie rola kinematyki różniczkowalnej i audytowalnej geometrii.

### 3. Wstęp teoretyczny
#### 3.1. Po co nam kinematyka prosta (FK), skoro „są biblioteki”?
Kinematyka prosta (*forward kinematics*, FK) odpowiada na pytanie: **gdzie w przestrzeni znajduje się końcówka robocza manipulatora**, gdy znamy wartości zmiennych przegubowych (kąty lub przesunięcia). Jest to najbardziej podstawowy model geometryczny robota: bez FK nie da się sensownie sterować w przestrzeni zadaniowej, planować ruchu, analizować kolizji ani interpretować danych z czujników związanych z narzędziem (kamera na chwytaku, czujnik siły/momentu w nadgarstku).

Pokusą początkujących jest poleganie wyłącznie na bibliotekach (URDF, MoveIt2, narzędzia producentów). To jest rozsądne na poziomie wdrożenia, ale **niebezpieczne** na poziomie inżynierii: jeśli model jest błędny o kilka stopni lub milimetrów, to robot może nie trafić w detal, a w skrajnym przypadku zachować się niebezpiecznie. Doświadczenie pokazuje, że źródłem problemu bywa nie algorytm planowania, lecz:
- pomylona oś przegubu (znak lub kierunek),
- niezgodna konwencja DH (klasyczna vs zmodyfikowana),
- mylenie ramek \({}^A\mathbf{T}_B\),
- błędna stała transformacja narzędzia (TCP) lub kamery.

Dlatego w kursie budujemy FK „od zera”, aby student potrafił:
1) skonstruować model,  
2) sprawdzić go testami zdrowego rozsądku,  
3) porównać z URDF/symulatorem i zidentyfikować rozbieżności.

Źródłowo FK i DH są klasycznym materiałem (Craig, 2004; Siciliano i in., 2009; Lynch i Park, 2017; Corke, 2017), ale do 2030 rośnie ich znaczenie w nowych kontekstach: kinematyka różniczkowalna w uczeniu sterowania, cyfrowe bliźniaki, oraz audytowalne warstwy geometrii w systemach adaptacyjnych (NIST AI RMF, 2023; IFR *World Robotics* 2023–2025 jako tło rosnących wdrożeń).

#### 3.2. Łańcuch kinematyczny jako graf ramek: \({}^0\mathbf{T}_n(\mathbf{q})\)
Manipulator szeregowy to sekwencja ogniw połączonych przegubami. Formalnie opisujemy go jako łańcuch transformacji w \(SE(3)\):
\[
{}^0\mathbf{T}_n(\mathbf{q}) = {}^0\mathbf{T}_1(q_1)\,{}^1\mathbf{T}_2(q_2)\cdots {}^{n-1}\mathbf{T}_n(q_n),
\]
gdzie \(\mathbf{q}=[q_1,\dots,q_n]^\top\) jest wektorem zmiennych przegubowych. To jest ta sama idea co w poprzednim wykładzie (składanie transformacji), tylko teraz każda transformacja zależy od jednego przegubu.

Wynik \({}^0\mathbf{T}_n\) ma postać:
\[
{}^0\mathbf{T}_n=
\begin{bmatrix}
{}^0\mathbf{R}_n & {}^0\mathbf{p}_n\\
0 & 1
\end{bmatrix},
\]
gdzie \({}^0\mathbf{p}_n\) jest położeniem końcówki roboczej w bazie, a \({}^0\mathbf{R}_n\) jej orientacją.

FK jest więc funkcją:
\[
f:\ \mathbb{R}^n \to SE(3),\qquad \mathbf{q}\mapsto {}^0\mathbf{T}_n(\mathbf{q}).
\]
Zwróć uwagę: wynik nie jest „wektorem 6D” wprost; jest elementem \(SE(3)\). To przygotowuje grunt pod Jacobian i sterowanie w przestrzeni zadaniowej.

#### 3.3. Konwencja Denavita–Hartenberga: dlaczego działa i co „standaryzuje”
Denavit i Hartenberg zaproponowali w 1955 roku konwencję, która upraszcza zapis transformacji między kolejnymi ogniwami przez cztery parametry na przegub/ogniwo (Denavit i Hartenberg, 1955). Kluczowy pomysł jest taki:
- oś \(z_i\) ramy \(\{i\}\) jest zgodna z osią ruchu przegubu \(i\) (oś obrotu dla przegubu R lub kierunek przesuwu dla przegubu P),
- oś \(x_i\) leży na wspólnej normalnej między \(z_{i-1}\) i \(z_i\) (albo jest dobrana w sposób spójny, gdy osie są równoległe),
- \(y_i\) wynika z prawoskrętności.

W klasycznej konwencji DH parametry \((a_i,\alpha_i,d_i,\theta_i)\) opisują transformację \({}^{i-1}\mathbf{T}_i\) jako sekwencję czterech elementarnych ruchów:
1) obrót wokół \(z_{i-1}\) o \(\theta_i\),  
2) przesunięcie wzdłuż \(z_{i-1}\) o \(d_i\),  
3) przesunięcie wzdłuż \(x_i\) o \(a_i\),  
4) obrót wokół \(x_i\) o \(\alpha_i\).

Macierz ma postać:
\[
{}^{i-1}\mathbf{T}_i=
\mathbf{R}_z(\theta_i)\mathbf{T}_z(d_i)\mathbf{T}_x(a_i)\mathbf{R}_x(\alpha_i).
\]
Po wymnożeniu otrzymuje się standardową postać 4×4:
\[
{}^{i-1}\mathbf{T}_i =
\begin{bmatrix}
\cos\theta_i & -\sin\theta_i\cos\alpha_i & \sin\theta_i\sin\alpha_i & a_i\cos\theta_i\\
\sin\theta_i & \cos\theta_i\cos\alpha_i & -\cos\theta_i\sin\alpha_i & a_i\sin\theta_i\\
0 & \sin\alpha_i & \cos\alpha_i & d_i\\
0 & 0 & 0 & 1
\end{bmatrix}.
\]

Interpretacja parametrów:
- \(a_i\) — długość ogniwa (odległość między osiami \(z_{i-1}\) i \(z_i\) mierzona wzdłuż \(x_i\)),  
- \(\alpha_i\) — skręt (kąt między osiami \(z_{i-1}\) i \(z_i\) wokół \(x_i\)),  
- \(d_i\) — przesunięcie wzdłuż \(z_{i-1}\),  
- \(\theta_i\) — obrót wokół \(z_{i-1}\).

W przegubie obrotowym (R) zmienną jest \(\theta_i\), a \(d_i\) jest stałe; w przegubie pryzmatycznym (P) zmienną jest \(d_i\), a \(\theta_i\) jest stałe.

#### 3.4. Klasyczna vs zmodyfikowana DH: dlaczego to się myli i jak temu zapobiec
W literaturze i w narzędziach spotkasz co najmniej dwie popularne wersje DH:
- **klasyczną (standard DH)** — jak powyżej,
- **zmodyfikowaną (modified DH)** — z inną kolejnością elementarnych transformacji i innym przypisaniem osi.

Obie są poprawne, ale nie można mieszać parametrów między nimi. Typowy błąd praktyczny wygląda tak: ktoś bierze tabelę parametrów z dokumentacji producenta (często modified DH), a następnie stosuje wzór na macierz z klasycznego DH. Wynik bywa „prawie dobry” w pobliżu pozycji zerowej i dramatycznie zły gdzie indziej.

Inżynierska zasada bezpieczeństwa brzmi: **zawsze zapisuj obok tabeli DH jedno równanie definiujące \({}^{i-1}\mathbf{T}_i\)** (kolejność \(\mathbf{R}\) i \(\mathbf{T}\)). To jest „kontrakt” konwencji.

<!-- WSTEP_TEORETYCZNY_CONT -->
#### 3.5. Procedura przypisania ramek DH (praktyczna lista kroków)
W zadaniach domowych i projektach największą trudnością jest nie samo mnożenie macierzy, lecz **poprawne przypisanie osi**. Poniżej procedura, która w praktyce działa dla większości manipulatorów szeregowych:

1) **Wybierz ramę bazową \(\{0\}\)**: ustal, gdzie jest początek i jak skierowane są osie (często zgodnie z dokumentacją/URDF).  
2) **Dla każdego przegubu \(i\)** wyznacz oś ruchu i ustaw ją jako oś \(z_{i-1}\) (w klasycznym DH) lub \(z_i\) (w zależności od konwencji; w tym wykładzie trzymamy klasyczne DH).  
3) **Wyznacz wspólną normalną** między \(z_{i-1}\) i \(z_i\): jest to prosta prostopadła do obu osi. Jej kierunek staje się osią \(x_i\).  
   - jeśli osie się przecinają, wspólna normalna ma długość 0 (wtedy \(a_i=0\)),  
   - jeśli osie są równoległe, wspólna normalna nie jest jednoznaczna — wybierz \(x_i\) tak, aby układ był spójny i wygodny.  
4) **Ustaw początek ramy \(\{i\}\)** na przecięciu osi \(x_i\) z osią \(z_i\) (lub w miejscu naturalnym, jeśli osie są równoległe).  
5) **Zamknij prawoskrętność**: \(y_i = z_i \times x_i\).  
6) **Wyznacz parametry**:  
   - \(a_i\): odległość wzdłuż \(x_i\) między osiami \(z_{i-1}\) i \(z_i\),  
   - \(\alpha_i\): kąt między \(z_{i-1}\) i \(z_i\) mierzony wokół \(x_i\),  
   - \(d_i\): przesunięcie wzdłuż \(z_{i-1}\) do wspólnej normalnej,  
   - \(\theta_i\): obrót wokół \(z_{i-1}\) ustawiający \(x_{i-1}\) na \(x_i\).

Ta procedura jest „mechaniczna”, ale wymaga dyscypliny w rysunku osi. W praktyce warto wykonywać szkice 3D (choćby w prostym narzędziu) i stale pytać: „czy osie są prawoskrętne?”, „czy \(z\) jest osią przegubu?”, „czy \(a_i\) ma sens geometryczny?”.

#### 3.6. Model narzędzia (TCP) i „ostatnia stała transformacja”
W wielu zadaniach końcówka robocza (punkt TCP) nie znajduje się w miejscu, które intuicyjnie wybierzemy jako koniec ostatniego ogniwa. Typowy przykład: chwytak ma długość, kamera ma odsunięcie, czujnik siły jest między kołnierzem a narzędziem. Wtedy FK dla „narzędzia” ma postać:
\[
{}^0\mathbf{T}_{TCP}(\mathbf{q}) = {}^0\mathbf{T}_n(\mathbf{q})\,{}^n\mathbf{T}_{TCP},
\]
gdzie \({}^n\mathbf{T}_{TCP}\) jest **stałą** transformacją narzędzia względem ostatniej ramy DH.

W praktyce błędy TCP są jedną z głównych przyczyn „nietrafiania” w aplikacjach przemysłowych. Co ważne, błąd TCP może wyglądać jak błąd kinematyki, choć nim nie jest. Dlatego w walidacji zawsze rozdzielamy:
- walidację łańcucha DH (bez narzędzia),
- walidację narzędzia (osobno, na prostych testach).

#### 3.7. Weryfikacja FK: testy zdrowego rozsądku i minimalny zestaw przypadków
Zanim przejdziesz do bardziej złożonych zadań (IK, Jacobian, planowanie), FK musi przejść testy. Minimalny, praktyczny zestaw obejmuje:

1) **Pozycja zerowa**: sprawdź, czy dla \(\mathbf{q}=0\) (lub konfiguracji referencyjnej) robot ma orientację i położenie zgodne z rysunkiem/dokumentacją.  
2) **Test jednej osi naraz**: porusz tylko jednym przegubem o mały kąt (np. \(+10^\circ\)) i sprawdź, czy ruch końcówki jest zgodny z intuicją (kierunek obrotu!).  
3) **Niezmienniki \(SO(3)\)**: dla każdej konfiguracji \(\det(\mathbf{R})\approx 1\) i \(\mathbf{R}^\top\mathbf{R}\approx \mathbf{I}\).  
4) **Granice przegubów**: sprawdź kilka konfiguracji blisko ograniczeń; często ujawniają się tam pomyłki znaków i osi.  
5) **Porównanie z niezależnym źródłem**: URDF + narzędzie (np. MoveIt2), symulator (np. Isaac Sim), albo pomiar w świecie (kamera z markerami).

Do 2030 rośnie waga takiej weryfikacji, bo coraz częściej FK staje się elementem „warstwy gwarancji” w systemach uczących się: model może proponować ruch, ale warstwa geometryczna musi sprawdzić zgodność z ograniczeniami i sens fizyczny (NIST AI RMF, 2023).

#### 3.8. DH a URDF: dwa opisy tego samego robota, różne cele
URDF opisuje robota jako drzewo łączy (*link*) i przegubów (*joint*) z osiami w lokalnych ramach, wraz z geometrią wizualną/kolizyjną. DH z kolei jest „spłaszczonym” opisem łańcucha w postaci tabeli czterech parametrów na ogniwo.

Nie ma sprzeczności: to są dwa widoki tego samego obiektu. URDF jest wygodny do integracji i symulacji (ROS2, MoveIt2), DH bywa wygodny do analitycznych wyprowadzeń i szybkich obliczeń (np. w dydaktyce, w prototypach). W praktyce inżynier często:
- bierze URDF jako źródło prawdy o geometrii i osiach,
- generuje z niego model kinematyczny (macierze \(\mathbf{T}\)),
- a jeśli potrzebuje DH, to wyprowadza tabelę DH w sposób spójny z URDF.

#### 3.9. Prognoza 2030: kinematyka różniczkowalna i „geometria jako moduł audytu”
W latach 2023–2025 obserwujemy rosnące znaczenie łączenia geometrii z uczeniem i optymalizacją: modele sterowania uczą się na danych, ale są osadzane w strukturze kinematycznej (np. przez warstwy FK w grafie obliczeń). W praktyce oznacza to:
- FK staje się funkcją różniczkowalną używaną w uczeniu przez całkowanie wsteczne (w obrębie geometrii),
- ważne jest unikanie osobliwości reprezentacji orientacji i utrzymanie \(\mathbf{R}\in SO(3)\),
- modele zbliżają się do „fizyki i geometrii jako priorytetu”, a nie jako dodatku.

Prognoza do 2030: w systemach przemysłowych i usługowych warstwa geometrii (FK + ograniczenia + testy) będzie pełnić funkcję audytowalnego „dowodu spójności”: łatwiej jest certyfikować i walidować to, co wynika z geometrii, niż to, co wynika z czarnej skrzynki. Dlatego podstawy DH nie tracą znaczenia, mimo rozwoju Physical AI i modeli fundamentalnych — przeciwnie, staną się narzędziem integracji tych modeli z rzeczywistym robotem.

**Odniesienia w tekście:** Denavit i Hartenberg (1955), Craig (2004), Siciliano i in. (2009), Lynch i Park (2017), Corke (2017), Barfoot (2017), dokumentacja ROS2/URDF/TF, MoveIt2 (wydania i dokumentacja 2023–2025), NIST AI RMF (2023), IFR *World Robotics* (2023–2025).

#### 3.10. Uporządkowany zapis tabeli DH: co dokładnie zapisujemy i jak uniknąć „przesunięć indeksów”
W praktyce tabela DH jest miejscem, gdzie drobna pomyłka indeksu robi ogromną różnicę. Zalecany format tabeli (dla klasycznej DH) to:

| \(i\) | typ | \(a_i\) | \(\alpha_i\) | \(d_i\) | \(\theta_i\) |
|---:|:---:|---:|---:|---:|---:|
| 1 | R/P | … | … | … | … |
| 2 | R/P | … | … | … | … |
| … | … | … | … | … | … |

Przy czym należy dopisać dwie informacje „meta”:
1) **który parametr jest zmienny** (R: \(\theta_i=q_i+\theta_{i,0}\); P: \(d_i=q_i+d_{i,0}\)),  
2) **jaka jest konwencja zerowej konfiguracji** (czy \(q_i=0\) oznacza pozycję mechaniczną „zero”, czy programowe „home”).

Praktyczny zapis (zalecany w kodzie):
\[
\theta_i(q_i)=\theta_{i,0}+s_i q_i,\qquad d_i(q_i)=d_{i,0}+t_i q_i,
\]
gdzie \(s_i\in\{0,1\}\) i \(t_i\in\{0,1\}\) wybierają typ przegubu (dla R: \(s_i=1,t_i=0\); dla P: \(s_i=0,t_i=1\)). Taki zapis:
- ułatwia implementację jednej funkcji `dh_T(a, alpha, d, theta)`,
- zmniejsza ryzyko, że student „przestawi” parametry w wierszu.

#### 3.11. Przykład „wzmacniania błędu”: dlaczego 1° na przegubie to nie zawsze 1° na narzędziu
W systemach wieloprzegubowych błąd orientacji osi lub offsetu kątowego może narastać. Jeśli masz 6-osiowego robota, a w dwóch przegubach pomylisz znak (albo offset) o 1°, to końcówka robocza w pewnych konfiguracjach może odchylić się o centymetry. Dydaktycznie można to streścić:
- błędy osiowe i offsety są mnożone przez długości ogniw,
- błąd w bazie „przechodzi” przez cały łańcuch,
- w pobliżu osobliwości kinematycznych małe błędy w przegubach dają duże błędy w przestrzeni zadaniowej.

Dlatego nawyk testowania FK (por. §3.7) jest fundamentem dalszych tematów, zwłaszcza Jacobianu i IK. W robotyce 2030 dojdzie dodatkowy powód: jeśli komponent uczony proponuje \(\mathbf{q}\), to warstwa geometryczna musi być na tyle poprawna, by wykrywać i blokować ruchy prowadzące do kolizji lub przekroczeń ograniczeń.

#### 3.12. Podsumowanie
Konwencja DH jest skutecznym „językiem kompresji” geometrii manipulatora: cztery parametry na ogniwo pozwalają zbudować model FK, który jest spójny, obliczalny i weryfikowalny. Kluczem do sukcesu nie jest pamiętanie wzoru na macierz, lecz dyscyplina w przypisaniu osi, jawne zapisanie konwencji oraz testy weryfikujące, że model zachowuje się zgodnie z geometrią i dokumentacją. To będzie baza pod kinematykę odwrotną, Jacobian i sterowanie w przestrzeni zadaniowej.

W kolejnym wykładzie przejdziemy do praktycznej budowy FK dla konkretnych manipulatorów i do konsekwencji wyboru ramek dla dalszych obliczeń.

W ramach przygotowania warto już teraz przećwiczyć: (1) narysowanie osi \(z_i\) na rzeczywistym ramieniu robota (lub w symulatorze), (2) weryfikację znaków przez ruch jednego przegubu, oraz (3) zapis tabeli DH tak, aby była jednoznaczna dla osoby trzeciej. Te trzy nawyki oszczędzają najwięcej czasu w projektach zespołowych.

#### 3.13. Ograniczenia DH i alternatywa: iloczyn wykładniczych (POE) oraz teoria skrętów
Konwencja DH jest bardzo użyteczna dydaktycznie i praktycznie, ale ma ograniczenia:
- przypisanie ramek bywa niejednoznaczne (równoległe osie, osie przecinające się),
- w złożonych konstrukcjach (np. nadgarstek z osiami blisko współliniowymi) łatwo o pomyłkę konwencji,
- DH „ukrywa” pewne własności geometryczne, które są naturalne w teorii skrętów.

Dlatego w nowocześniejszych ujęciach kinematyki manipulatorów często spotyka się formalizm **iloczynu wykładniczych** (POE, *product of exponentials*), znany z teorii skrętów (Murray, Li, Sastry, 1994; Lynch i Park, 2017). W tym podejściu opisujesz każdy przegub przez skręt \(\mathbf{S}_i\in\mathbb{R}^6\) (oś obrotu/przesuwu w przestrzeni) i zapisujesz FK jako:
\[
{}^0\mathbf{T}_n(\mathbf{q}) = \exp([\mathbf{S}_1]_\wedge q_1)\,\exp([\mathbf{S}_2]_\wedge q_2)\cdots \exp([\mathbf{S}_n]_\wedge q_n)\,\mathbf{M},
\]
gdzie \(\mathbf{M}\) jest konfiguracją „home” (stałą transformacją końcówki roboczej przy \(\mathbf{q}=0\)), a \([\cdot]_\wedge\) jest operatorem z \(\mathbb{R}^6\) do \(\mathfrak{se}(3)\) (analogicznie do \([\cdot]_\times\) w \(SO(3)\)).

Zaletą POE jest to, że:
- oś przegubu (skręt) ma bezpośrednią interpretację geometryczną,
- formalizm naturalnie łączy się z Jacobianem (tzw. Jacobian przestrzenny i ciałowy),
- jest dobrze dopasowany do różniczkowania i optymalizacji (ważne w robotyce 2030).

W tym kursie DH pozostaje głównym narzędziem w Części 1 (bo uczy dyscypliny ramek i daje szybkie obliczenia), ale warto wiedzieć, że DH nie jest jedyną drogą. Do 2030, wraz z rosnącą rolą narzędzi optymalizacyjnych i uczenia na rozmaitościach, formalizm POE będzie coraz częściej spotykany w kodzie i publikacjach — i dlatego już teraz sygnalizujemy jego istnienie.

#### 3.14. FK jako „interfejs” między mechaniką a oprogramowaniem (MoveIt2, symulacja, cyfrowy bliźniak)
W praktyce przemysłowej FK jest wszędzie, choć często „niewidoczna”:
- planowanie ruchu i unikanie kolizji: potrzebujesz położenia każdego ogniwa w świecie,
- sterowanie: potrzebujesz pozy TCP i często orientacji w czasie rzeczywistym,
- percepcja: musisz przenieść detekcję z ramy kamery do ramy narzędzia/bazy,
- diagnostyka: porównujesz pozycję modelową z pomiarem (np. markerów).

Narzędzia takie jak URDF i MoveIt2 (oraz ich dokumentacja i wydania z lat 2023–2025) implementują ten „interfejs” w sposób systemowy: model geometryczny jest jednym źródłem prawdy, z którego wynikają transformacje TF, kinematyka i geometria kolizyjna. W cyfrowych bliźniakach dochodzi jeszcze jeden wymiar: spójność modeli między symulacją a robotem w terenie. Jeśli FK w bliźniaku różni się od FK w robocie (np. przez inny TCP lub inne osie przegubów), to wszelkie wnioski o czasie cyklu, zasięgu i bezpieczeństwie będą błędne.

W perspektywie 2030 ten problem się nasili, bo rośnie złożoność systemów: integracja wielu czujników, narzędzi i wariantów chwytaków, a także roboty wielofunkcyjne. W tym kontekście dyscyplina kinematyczna (spójne \(\mathbf{T}\), spójne ramy, testy) jest warunkiem skalowania, a nie „miłym dodatkiem”.

#### 3.15. Kinematyka różniczkowalna: dlaczego FK „musi dać się różniczkować”
W klasycznej robotyce używamy FK, a następnie wyprowadzamy Jacobian (Wykład 6), aby powiązać prędkości przegubowe z prędkościami w przestrzeni zadaniowej. W robotyce opartej o uczenie coraz częściej robi się coś dodatkowego: FK staje się częścią obliczeń, przez które przechodzą gradienty (np. w uczeniu trajektorii, dopasowaniu parametrów, kalibracji, dopasowaniu chwytów).

To oznacza dwie praktyczne konsekwencje:
1) implementacja FK powinna być stabilna numerycznie (utrzymywać \(\mathbf{R}\in SO(3)\)),  
2) zapis powinien być jednoznaczny i „audytowalny” — aby odróżniać błąd modelu od błędu danych.

Prognoza do 2030: w wielu systemach „uczących” warstwa geometrii będzie pełnić rolę twardego ograniczenia (np. zasięg, brak kolizji, ograniczenia przegubów), a jednocześnie będzie źródłem gradientów w optymalizacji. To jeszcze jeden powód, by podstawy DH traktować poważnie: to jest najprostszy sposób, aby nauczyć się myślenia o robocie jako o funkcji \(\mathbf{q}\mapsto \mathbf{T}\) z dobrze zdefiniowaną strukturą.

W praktyce dydaktycznej warto także rozdzielić „model geometryczny” od „modelu wykonawczego”: to pierwszy ma być poprawny i testowalny, a dopiero na nim buduje się sterowanie, planowanie i integrację oprogramowania.

### 4. Struktura prezentacji slajdów (PowerPoint / Google Slides)
**Założenie stylu:** Assertion–Evidence — tytuł slajdu jest tezą; treść to dowód (rysunek/animacja/wykres); tekst minimalny.

1. **Slajd nr 1 | FK to „źródło prawdy” o pozie narzędzia**  
   - Treść:  
     - \({}^0\mathbf{T}_n(\mathbf{q})\) opisuje położenie i orientację  
     - Bez FK nie ma planowania ani sterowania w przestrzeni zadaniowej  
   - Sugestie wizualne: animacja manipulatora z wyświetlaną ramą TCP; obok macierz \(\mathbf{T}\) jako „paszport” narzędzia.  
   - Notatki dla prowadzącego: 2–3 min; podać przykłady błędów: zły TCP, zła oś przegubu.

2. **Slajd nr 2 | Łańcuch kinematyczny to mnożenie transformacji w \(SE(3)\)**  
   - Treść:  
     - \({}^0\mathbf{T}_n = \prod_{i=1}^n {}^{i-1}\mathbf{T}_i\)  
   - Sugestie wizualne: graf ramek na kolejnych ogniwach; „klocki” \(\mathbf{T}_1,\mathbf{T}_2,\dots\) składane w \(\mathbf{T}\).  
   - Notatki: 2 min; nawiązać do Wykładu 03.

3. **Slajd nr 3 | DH standaryzuje przejście między kolejnymi ramami przez 4 parametry**  
   - Treść:  
     - \(a_i,\alpha_i,d_i,\theta_i\)  
     - Jeden wiersz tabeli = jedna macierz \({}^{i-1}\mathbf{T}_i\)  
   - Sugestie wizualne: pojedyncze ogniwo z dwiema osiami \(z_{i-1}\), \(z_i\) i wspólną normalną \(x_i\); podpisy parametrów na rysunku.  
   - Notatki: 3 min; podkreślić geometrię \(a_i,\alpha_i\).

4. **Slajd nr 4 | W klasycznym DH oś \(z\) jest osią przegubu**  
   - Treść:  
     - Przegub R: zmienna \(\theta_i\)  
     - Przegub P: zmienna \(d_i\)  
   - Sugestie wizualne: dwa panele: przegub obrotowy (strzałka obrotu) i pryzmatyczny (strzałka przesuwu); na obu oś \(z\) zaznaczona grubą linią.  
   - Notatki: 2 min; „jeśli \(z\) nie jest osią przegubu, tabela jest zła”.

5. **Slajd nr 5 | Macierz DH to sekwencja: \(R_z(\theta)\,T_z(d)\,T_x(a)\,R_x(\alpha)\)**  
   - Treść:  
     - \({}^{i-1}\mathbf{T}_i=\mathbf{R}_z(\theta_i)\mathbf{T}_z(d_i)\mathbf{T}_x(a_i)\mathbf{R}_x(\alpha_i)\)  
   - Sugestie wizualne: animacja „czterech kroków” — obrót, przesunięcie, przesunięcie, obrót — na dwóch ramach; każdy krok innym kolorem.  
   - Notatki: 3 min; to jest „kontrakt konwencji”.

6. **Slajd nr 6 | Jedno równanie + tabela DH eliminuje 90% nieporozumień**  
   - Treść:  
     - Zawsze dopisz kolejność transformacji  
     - Nie mieszaj standard DH z modified DH  
   - Sugestie wizualne: czerwone ostrzeżenie „standard vs modified”; obok dwa wzory z inną kolejnością i ta sama tabela jako przykład pułapki.  
   - Notatki: 2 min; pokazać, jak wygląda błąd „prawie działa”.

7. **Slajd nr 7 | Procedura przypisania ramek DH jest mechaniczna (i trzeba ją ćwiczyć)**  
   - Treść:  
     - \(z\): oś przegubu  
     - \(x\): wspólna normalna  
     - \(y\): prawoskrętność  
   - Sugestie wizualne: checklista 6 kroków z §3.5 + prosty rysunek dwóch osi równoległych (niejednoznaczność).  
   - Notatki: 3 min; zachęcić do testu „jedna oś naraz”.

8. **Slajd nr 8 | TCP to osobna stała transformacja — nie mieszaj jej z DH**  
   - Treść:  
     - \({}^0\mathbf{T}_{TCP}={}^{0}\mathbf{T}_n\,{}^n\mathbf{T}_{TCP}\)  
     - Błąd TCP = „nietrafianie” mimo dobrej DH  
   - Sugestie wizualne: kołnierz robota + chwytak; dwa punkty: „koniec ogniwa” vs „punkt roboczy”; overlay przesunięcia TCP.  
   - Notatki: 2 min; przykład: kamera na chwytaku.

9. **Slajd nr 9 | Walidacja FK: pozycja zerowa, jedna oś naraz, niezmienniki \(SO(3)\)**  
   - Treść:  
     - \(\det(\mathbf{R})\approx 1\)  
     - \(\mathbf{R}^\top\mathbf{R}\approx \mathbf{I}\)  
     - testy punktów kontrolnych  
   - Sugestie wizualne: panel diagnostyczny (zielone/czerwone wskaźniki) + animacja ruchu jednego przegubu o \(10^\circ\).  
   - Notatki: 3 min; podkreślić „zanim obwinisz planner”.

10. **Slajd nr 10 | FK łączy URDF, TF i planowanie ruchu w jednym modelu**  
   - Treść:  
     - URDF: osie przegubów + geometria  
     - TF: graf ramek w czasie  
     - MoveIt2: FK/IK + kolizje  
   - Sugestie wizualne: trójkąt „URDF–TF–MoveIt2” z FK w środku; zrzut stylizowany grafu ramek dla manipulatora.  
   - Notatki: 3 min; nawiązać do zastosowań laboratoryjnych.

11. **Slajd nr 11 | Przykład 2R pokazuje, że DH to przewidywalna geometria**  
   - Treść:  
     - \({}^0\mathbf{T}_2(\theta_1,\theta_2)\)  
     - \(\mathbf{p}=[x(\theta),y(\theta),0]\)  
   - Sugestie wizualne: animacja manipulatora 2R w płaszczyźnie + tor końcówki; obok prosty wykres \(x(\theta_1)\), \(y(\theta_2)\).  
   - Notatki: 2 min; zapowiedzieć przykład rachunkowy.

12. **Slajd nr 12 | „Modified DH” bywa w dokumentacji producenta — musisz to sprawdzić**  
   - Treść:  
     - Ta sama tabela ≠ ta sama macierz  
     - Konieczny jest „kontrakt” \({}^{i-1}\mathbf{T}_i=\dots\)  
   - Sugestie wizualne: mini-studium: ta sama tabela parametrów + dwa różne wyniki FK (dwie różne pozy TCP).  
   - Notatki: 2 min; podkreślić: sprawdzaj źródło tabeli.

13. **Slajd nr 13 | Błąd 1° w offsetach przegubów potrafi dać centymetry na TCP**  
   - Treść:  
     - błąd osi/offsetu \(\rightarrow\) błąd pozy  
     - rośnie z długością ogniw i konfiguracją  
   - Sugestie wizualne: wykres „błąd TCP [mm]” vs „błąd offsetu [deg]” dla dwóch długości ramienia; pokaz konfiguracji „najgorszej”.  
   - Notatki: 3 min; wprowadzić pojęcie wrażliwości (most do Jacobianu).

14. **Slajd nr 14 | FK jest warstwą bezpieczeństwa: ograniczenia i kolizje są liczone na geometrii**  
   - Treść:  
     - ograniczenia przegubów  
     - kolizje i strefy zakazane  
   - Sugestie wizualne: manipulator w pobliżu przeszkody; overlay „bryły kolizyjne” i dystans; czerwone „stop” przy przekroczeniu.  
   - Notatki: 2 min; zaznaczyć, że bez FK nie ma kolizji.

15. **Slajd nr 15 | POE (iloczyn wykładniczych) to alternatywa DH i dobrze pasuje do optymalizacji**  
   - Treść:  
     - \(\mathbf{T}=\prod \exp([\mathbf{S}_i]_\wedge q_i)\mathbf{M}\)  
   - Sugestie wizualne: wizualizacja osi skrętów w przestrzeni; „klocki” wykładniczych; porównanie DH vs POE jako dwa równoważne języki.  
   - Notatki: 3 min; powiedzieć, że DH zostaje, ale warto znać POE.

16. **Slajd nr 16 | Do 2030 FK będzie częścią uczenia: kinematyka różniczkowalna**  
   - Treść:  
     - FK w grafie obliczeń  
     - gradienty przechodzą przez \(\mathbf{T}(\mathbf{q})\)  
   - Sugestie wizualne: schemat: „dane → polityka → \(\mathbf{q}\) → FK → strata” z zaznaczonymi strzałkami gradientu; podpis „całkowanie wsteczne przez geometrię”.  
   - Notatki: 2 min; połączyć z trendami i audytem.

17. **Slajd nr 17 | Najlepsza praktyka: nazwij transformacje i testuj je automatycznie**  
   - Treść:  
     - `T_0_1`, `T_1_2`, …  
     - testy: \(\mathbf{R}^\top\mathbf{R}\), \(\det\)  
   - Sugestie wizualne: zrzut z testów jednostkowych (CI) sprawdzających niezmienniki; przykład wykrycia pomyłki osi.  
   - Notatki: 2 min; zachęcić do testów w projektach.

18. **Slajd nr 18 | Mini-studium: URDF vs DH — jak porównać i znaleźć błąd**  
   - Treść:  
     - porównanie TCP dla 5 konfiguracji  
     - różnica: \(\|\Delta \mathbf{p}\|\) i dystans kątowy  
   - Sugestie wizualne: tabela 5 wierszy: \(\mathbf{q}\), \(\mathbf{p}_{DH}\), \(\mathbf{p}_{URDF}\), błąd; wykres błędu.  
   - Notatki: 3 min; to jest praktyczne ćwiczenie laboratoryjne.

19. **Slajd nr 19 | Z DH do IK: dlaczego FK musi być perfekcyjny**  
   - Treść:  
     - IK rozwiązuje \(f(\mathbf{q})=\mathbf{T}^\*\)  
     - zły FK → zła IK  
   - Sugestie wizualne: schemat pętli: cel \(\mathbf{T}^\*\) → IK → \(\mathbf{q}\) → FK → błąd; zaznaczyć, że FK jest „modelem”.  
   - Notatki: 2 min; zapowiedzieć Wykład 05.

20. **Slajd nr 20 | Podsumowanie: DH to dyscyplina ramek i spójny model geometrii**  
   - Treść:  
     - osie \(z\) = osie przegubów  
     - jawna konwencja + testy  
     - FK jako interfejs systemu  
   - Sugestie wizualne: jedna plansza „ściąga”: definicja \({}^{i-1}\mathbf{T}_i\), tabela DH i 3 testy walidacyjne.  
   - Notatki: 2 min; domknąć wątek i przejść do przykładów/zadań.

21. **Slajd nr 21 | Pytania kontrolne, które zadajesz przed każdym FK**  
   - Treść:  
     - Czy \(z\) jest osią przegubu?  
     - Czy konwencja jest jawna?  
     - Czy TCP jest osobno?  
     - Czy testy przechodzą?  
   - Sugestie wizualne: lista kontrolna z ikonami + mini-rysunki osi.  
   - Notatki: 2 min; „to jest rytuał inżynierski”.

22. **Slajd nr 22 | Most do kolejnych tematów: Jacobian i sterowanie w przestrzeni zadaniowej**  
   - Treść:  
     - FK \(\rightarrow\) pochodne \(\rightarrow\) Jacobian  
     - wrażliwość i osobliwości  
   - Sugestie wizualne: wykres manipulowalności (elipsoida) jako teaser; animacja zmiany wrażliwości w zależności od konfiguracji.  
   - Notatki: 2 min; zapowiedzieć Wykład 06.

### 5. Przykłady i studia przypadku (minimum 5 szczegółowych)
#### Przykład 1: Manipulator planarny 2R — tabela DH i wzór na \((x,y)\)
**Opis problemu.** Rozważ manipulator planarny z dwoma przegubami obrotowymi (2R) o długościach ogniw \(L_1=0.6\ \text{m}\), \(L_2=0.4\ \text{m}\). Zmiennymi są kąty \(\theta_1,\theta_2\).  
(a) Zbuduj model FK i wyprowadź wzory na \((x(\theta),y(\theta))\).  
(b) Oblicz pozycję końcówki dla \(\theta_1=30^\circ\), \(\theta_2=-45^\circ\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Dobór ramek: osie \(z\) są prostopadłe do płaszczyzny ruchu, więc \(\alpha_1=\alpha_2=0\), a przesunięcia \(d_1=d_2=0\).  
2) Dla klasycznego DH wygodna tabela:

| \(i\) | typ | \(a_i\) | \(\alpha_i\) | \(d_i\) | \(\theta_i\) |
|---:|:---:|---:|---:|---:|---:|
| 1 | R | \(L_1\) | \(0\) | \(0\) | \(\theta_1\) |
| 2 | R | \(L_2\) | \(0\) | \(0\) | \(\theta_2\) |

3) Macierze:
\[
{}^{0}\mathbf{T}_{1} = \mathbf{R}_z(\theta_1)\mathbf{T}_x(L_1),\qquad
{}^{1}\mathbf{T}_{2} = \mathbf{R}_z(\theta_2)\mathbf{T}_x(L_2).
\]
4) Po wymnożeniu (lub z geometrii) otrzymujemy pozycję końcówki:
\[
x = L_1\cos\theta_1 + L_2\cos(\theta_1+\theta_2),\qquad
y = L_1\sin\theta_1 + L_2\sin(\theta_1+\theta_2).
\]
5) Podstawienie liczb:
\[
\theta_1=30^\circ,\ \theta_2=-45^\circ\Rightarrow \theta_1+\theta_2=-15^\circ.
\]
\[
x=0.6\cos30^\circ+0.4\cos(-15^\circ)\approx 0.6\cdot 0.8660+0.4\cdot 0.9659\approx 0.9052\ \text{m}.
\]
\[
y=0.6\sin30^\circ+0.4\sin(-15^\circ)\approx 0.6\cdot 0.5+0.4\cdot(-0.2588)\approx 0.1965\ \text{m}.
\]

**Kod (Python 3.11).**
```python
import numpy as np

L1, L2 = 0.6, 0.4
t1 = np.deg2rad(30.0)
t2 = np.deg2rad(-45.0)

x = L1*np.cos(t1) + L2*np.cos(t1+t2)
y = L1*np.sin(t1) + L2*np.sin(t1+t2)

print(x, y)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Rysunek 2D: dwa odcinki o długości \(L_1, L_2\) z zaznaczonymi kątami \(\theta_1,\theta_2\) oraz punkt TCP w \((x,y)\). Dodatkowo wykres toru TCP dla \(\theta_2\) stałego i \(\theta_1\) zmiennego (łuk).

---

#### Przykład 2: Jedna macierz DH „od zera” — obliczenie \({}^{i-1}\mathbf{T}_i\) i interpretacja parametrów
**Opis problemu.** Dla jednego ogniwa w klasycznym DH dane są: \(a=0.25\ \text{m}\), \(\alpha=90^\circ\), \(d=0.10\ \text{m}\), \(\theta=30^\circ\).  
(a) Wyznacz \({}^{i-1}\mathbf{T}_i\).  
(b) Zinterpretuj fizycznie, co oznacza \(\alpha=90^\circ\) w kontekście osi przegubów.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Wzór:
\[
{}^{i-1}\mathbf{T}_i =
\begin{bmatrix}
c_\theta & -s_\theta c_\alpha & s_\theta s_\alpha & a c_\theta\\
s_\theta & c_\theta c_\alpha & -c_\theta s_\alpha & a s_\theta\\
0 & s_\alpha & c_\alpha & d\\
0 & 0 & 0 & 1
\end{bmatrix},
\]
gdzie \(c_\theta=\cos\theta\), \(s_\theta=\sin\theta\) itd.
2) Podstawienie: \(c_\theta=\cos30^\circ=\sqrt{3}/2\), \(s_\theta=1/2\), \(c_\alpha=\cos90^\circ=0\), \(s_\alpha=1\).
3) Otrzymujemy:
\[
{}^{i-1}\mathbf{T}_i =
\begin{bmatrix}
\sqrt{3}/2 & 0 & 1/2 & 0.25\cdot \sqrt{3}/2\\
1/2 & 0 & -\sqrt{3}/2 & 0.25\cdot 1/2\\
0 & 1 & 0 & 0.10\\
0 & 0 & 0 & 1
\end{bmatrix}.
\]
4) Interpretacja \(\alpha=90^\circ\): osie \(z_{i-1}\) i \(z_i\) są prostopadłe (skręt o 90° wokół osi \(x_i\)).

**Kod (Python 3.11).**
```python
import numpy as np

def dh_T(a, alpha, d, theta):
    ca, sa = np.cos(alpha), np.sin(alpha)
    ct, st = np.cos(theta), np.sin(theta)
    return np.array([
        [ct, -st*ca,  st*sa, a*ct],
        [st,  ct*ca, -ct*sa, a*st],
        [0,      sa,     ca,    d],
        [0,       0,      0,    1],
    ], float)

T = dh_T(0.25, np.deg2rad(90.0), 0.10, np.deg2rad(30.0))
print(T)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Dwie osie przegubów \(z_{i-1}\) i \(z_i\) w 3D jako prostopadłe strzałki; wspólna normalna \(x_i\) i zaznaczony kąt \(\alpha\). Obok tabela z podstawionymi wartościami \(\sin/\cos\).

---

#### Przykład 3: FK 3R w przestrzeni — złożenie \({}^0\mathbf{T}_3\) i testy \(SO(3)\)
**Opis problemu.** Rozważ manipulator 3R o następującej tabeli klasycznego DH (metry, radiany):

| \(i\) | typ | \(a_i\) | \(\alpha_i\) | \(d_i\) | \(\theta_i\) |
|---:|:---:|---:|---:|---:|---:|
| 1 | R | 0.30 | \(+90^\circ\) | 0.20 | \(q_1\) |
| 2 | R | 0.40 | 0 | 0 | \(q_2\) |
| 3 | R | 0.20 | 0 | 0 | \(q_3\) |

Dla \(\mathbf{q}=[20^\circ, -30^\circ, 40^\circ]\) oblicz \({}^0\mathbf{T}_3\) oraz sprawdź, czy \({}^0\mathbf{R}_3\in SO(3)\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Liczymy kolejno \({}^0\mathbf{T}_1\), \({}^1\mathbf{T}_2\), \({}^2\mathbf{T}_3\) z tego samego wzoru DH.  
2) Składamy:
\[
{}^0\mathbf{T}_3 = {}^0\mathbf{T}_1\,{}^1\mathbf{T}_2\,{}^2\mathbf{T}_3.
\]
3) Test \(SO(3)\):
\[
e_{orth}=\| \mathbf{R}^\top\mathbf{R}-\mathbf{I}\|_F,\qquad \det(\mathbf{R}).
\]

**Kod (Python 3.11).**
```python
import numpy as np

def dh_T(a, alpha, d, theta):
    ca, sa = np.cos(alpha), np.sin(alpha)
    ct, st = np.cos(theta), np.sin(theta)
    return np.array([
        [ct, -st*ca,  st*sa, a*ct],
        [st,  ct*ca, -ct*sa, a*st],
        [0,      sa,     ca,    d],
        [0,       0,      0,    1],
    ], float)

deg = np.deg2rad
q1, q2, q3 = deg(20.0), deg(-30.0), deg(40.0)

T01 = dh_T(0.30, deg(90.0), 0.20, q1)
T12 = dh_T(0.40, deg(0.0),  0.00, q2)
T23 = dh_T(0.20, deg(0.0),  0.00, q3)
T03 = T01 @ T12 @ T23

R = T03[:3,:3]
orth = np.linalg.norm(R.T @ R - np.eye(3), ord="fro")
detR = np.linalg.det(R)

print("T03=\n", T03)
print("orth_err=", orth)
print("detR=", detR)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Animacja 3D manipulatora w konfiguracji \(\mathbf{q}\) z osiami na każdym ogniwie; obok „panel testów”: \(\det(\mathbf{R})\) i błąd ortonormalności.  

---

#### Przykład 4: Kalibracja prostego offsetu DH — dopasowanie \(\theta_{0}\) z pomiarów (metoda najmniejszych kwadratów)
**Opis problemu.** Dla manipulatora 2R z Przykładu 1 zakładamy, że w pierwszym przegubie istnieje nieznany offset \(\theta_{1,0}\), więc rzeczywisty kąt to \(\theta_1^{real}=\theta_1+\theta_{1,0}\). Wykonano 5 pomiarów pozy TCP \((x_k,y_k)\) dla znanych komend \((\theta_{1,k},\theta_{2,k})\). Dane (w metrach i stopniach):

| \(k\) | \(\theta_{1,k}\) | \(\theta_{2,k}\) | \(x_k\) | \(y_k\) |
|---:|---:|---:|---:|---:|
| 1 | 0 | 0 | 1.000 | 0.015 |
| 2 | 30 | -45 | 0.900 | 0.210 |
| 3 | 60 | -30 | 0.640 | 0.735 |
| 4 | -20 | 10 | 0.956 | -0.322 |
| 5 | 10 | 40 | 0.735 | 0.456 |

Przyjmij \(L_1=0.6\), \(L_2=0.4\). Oszacuj \(\theta_{1,0}\) metodą najmniejszych kwadratów, minimalizując sumę błędów pozy.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Model:
\[
\hat x_k(\delta)=L_1\cos(\theta_{1,k}+\delta)+L_2\cos(\theta_{1,k}+\delta+\theta_{2,k}),
\]
\[
\hat y_k(\delta)=L_1\sin(\theta_{1,k}+\delta)+L_2\sin(\theta_{1,k}+\delta+\theta_{2,k}),
\]
gdzie \(\delta=\theta_{1,0}\) jest nieznanym offsetem.
2) Funkcja kosztu:
\[
J(\delta)=\sum_{k=1}^{5}\left(\hat x_k(\delta)-x_k\right)^2+\left(\hat y_k(\delta)-y_k\right)^2.
\]
3) Ponieważ mamy jedną zmienną, możemy znaleźć minimum metodą przeszukiwania (siatka) lub prostą metodą gradientową. Dydaktycznie najprostsza jest siatka w przedziale np. \([-5^\circ,5^\circ]\).

**Kod (Python 3.11).**
```python
import numpy as np

L1, L2 = 0.6, 0.4
data = [
    (0,   0,   1.000,  0.015),
    (30, -45,  0.900,  0.210),
    (60, -30,  0.640,  0.735),
    (-20, 10,  0.956, -0.322),
    (10,  40,  0.735,  0.456),
]

def predict(t1_deg, t2_deg, delta_rad):
    t1 = np.deg2rad(t1_deg) + delta_rad
    t2 = np.deg2rad(t2_deg)
    x = L1*np.cos(t1) + L2*np.cos(t1+t2)
    y = L1*np.sin(t1) + L2*np.sin(t1+t2)
    return x, y

def cost(delta_rad):
    J = 0.0
    for t1, t2, x_m, y_m in data:
        x, y = predict(t1, t2, delta_rad)
        J += (x-x_m)**2 + (y-y_m)**2
    return J

grid = np.deg2rad(np.linspace(-5, 5, 2001))
J = np.array([cost(d) for d in grid])
best = grid[np.argmin(J)]

print("theta1_offset [deg] =", np.rad2deg(best))
print("min cost =", float(J.min()))
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Wykres \(J(\delta)\) w funkcji \(\delta\) (stopnie) z zaznaczonym minimum; dodatkowo wykres punktów pomiarowych i punktów modelowych po korekcie offsetu.

---

#### Przykład 5: ROS2 (Python) — węzeł obliczający FK z tabeli DH i publikujący TF TCP
**Opis problemu.** Chcesz zbudować minimalny element integracji: węzeł ROS2 subskrybuje `/joint_states` (kąty \(\mathbf{q}\)) i publikuje transformację TCP jako `TransformStamped` (TF2). Załóż, że masz tabelę DH dla 3R z Przykładu 3 oraz stały TCP \({}^3\mathbf{T}_{TCP}\) jako przesunięcie 0.1 m w osi \(x_3\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Dla odebranych \(q_1,q_2,q_3\) liczysz \({}^0\mathbf{T}_3(q)\).  
2) Doklejasz TCP:
\[
{}^0\mathbf{T}_{TCP} = {}^0\mathbf{T}_3\,{}^3\mathbf{T}_{TCP}.
\]
3) Z \(\mathbf{R}\) wyznaczasz kwaternion i publikujesz TF.

**Kod (Python 3.11 + ROS2 Humble, `rclpy`; szkic produkcyjny).**
```python
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import JointState
from geometry_msgs.msg import TransformStamped
from tf2_ros import TransformBroadcaster

def dh_T(a, alpha, d, theta):
    ca, sa = np.cos(alpha), np.sin(alpha)
    ct, st = np.cos(theta), np.sin(theta)
    return np.array([
        [ct, -st*ca,  st*sa, a*ct],
        [st,  ct*ca, -ct*sa, a*st],
        [0,      sa,     ca,    d],
        [0,       0,      0,    1],
    ], float)

def rot_to_quat(R):
    tr = np.trace(R)
    if tr > 0:
        S = np.sqrt(tr + 1.0) * 2
        qw = 0.25 * S
        qx = (R[2,1] - R[1,2]) / S
        qy = (R[0,2] - R[2,0]) / S
        qz = (R[1,0] - R[0,1]) / S
    else:
        i = int(np.argmax([R[0,0], R[1,1], R[2,2]]))
        if i == 0:
            S = np.sqrt(1.0 + R[0,0] - R[1,1] - R[2,2]) * 2
            qw = (R[2,1] - R[1,2]) / S
            qx = 0.25 * S
            qy = (R[0,1] + R[1,0]) / S
            qz = (R[0,2] + R[2,0]) / S
        elif i == 1:
            S = np.sqrt(1.0 + R[1,1] - R[0,0] - R[2,2]) * 2
            qw = (R[0,2] - R[2,0]) / S
            qx = (R[0,1] + R[1,0]) / S
            qy = 0.25 * S
            qz = (R[1,2] + R[2,1]) / S
        else:
            S = np.sqrt(1.0 + R[2,2] - R[0,0] - R[1,1]) * 2
            qw = (R[1,0] - R[0,1]) / S
            qx = (R[0,2] + R[2,0]) / S
            qy = (R[1,2] + R[2,1]) / S
            qz = 0.25 * S
    q = np.array([qw, qx, qy, qz], float)
    q = q / np.linalg.norm(q)
    if q[0] < 0:
        q = -q
    return q  # [w,x,y,z]

class FkTfPublisher(Node):
    def __init__(self):
        super().__init__("fk_tf_publisher")
        self.br = TransformBroadcaster(self)
        self.sub = self.create_subscription(JointState, "/joint_states", self.cb, 10)

        self.a = [0.30, 0.40, 0.20]
        self.alpha = [np.deg2rad(90.0), 0.0, 0.0]
        self.d = [0.20, 0.0, 0.0]

        self.T3_tcp = np.eye(4)
        self.T3_tcp[0, 3] = 0.10  # 10 cm w osi x3

    def cb(self, msg: JointState):
        name_to_pos = dict(zip(msg.name, msg.position))
        q1 = float(name_to_pos.get("joint1", 0.0))
        q2 = float(name_to_pos.get("joint2", 0.0))
        q3 = float(name_to_pos.get("joint3", 0.0))

        T01 = dh_T(self.a[0], self.alpha[0], self.d[0], q1)
        T12 = dh_T(self.a[1], self.alpha[1], self.d[1], q2)
        T23 = dh_T(self.a[2], self.alpha[2], self.d[2], q3)
        T03 = T01 @ T12 @ T23
        T0tcp = T03 @ self.T3_tcp

        R = T0tcp[:3, :3]
        p = T0tcp[:3, 3]
        qw, qx, qy, qz = rot_to_quat(R)

        t = TransformStamped()
        t.header.stamp = self.get_clock().now().to_msg()
        t.header.frame_id = "base_link"
        t.child_frame_id = "tcp"
        t.transform.translation.x = float(p[0])
        t.transform.translation.y = float(p[1])
        t.transform.translation.z = float(p[2])
        t.transform.rotation.w = float(qw)
        t.transform.rotation.x = float(qx)
        t.transform.rotation.y = float(qy)
        t.transform.rotation.z = float(qz)
        self.br.sendTransform(t)

def main():
    rclpy.init()
    node = FkTfPublisher()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
```

**Wizualizacja wyniku (opis diagramu/wykresu).** W RViz2: wizualizacja `base_link` i `tcp` (TF axes), porównanie z modelem URDF; dodatkowo wykres w czasie z `rqt_plot` pozycji TCP.  

### 6. Materiały dla studentów
#### 6 pytań teoretycznych (z oczekiwanymi odpowiedziami)
1) **Pytanie:** Co jest wejściem i wyjściem kinematyki prostej manipulatora?  
   **Odpowiedź:** Wejście: zmienne przegubowe \(\mathbf{q}\); wyjście: transformacja \({}^0\mathbf{T}_n(\mathbf{q})\in SE(3)\) (położenie i orientacja TCP).
2) **Pytanie:** Podaj postać macierzy DH \({}^{i-1}\mathbf{T}_i\) i opisz znaczenie parametrów \(a_i,\alpha_i,d_i,\theta_i\).  
   **Odpowiedź:** Standardowa macierz 4×4 z \(\cos\theta_i,\sin\theta_i,\cos\alpha_i,\sin\alpha_i\); \(a\) — długość, \(\alpha\) — skręt osi, \(d\) — przesunięcie wzdłuż \(z\), \(\theta\) — obrót wokół \(z\).
3) **Pytanie:** Który parametr jest zmienny dla przegubu obrotowego, a który dla pryzmatycznego?  
   **Odpowiedź:** R: zmienny \(\theta_i\); P: zmienny \(d_i\) (przy pozostałych parametrach stałych).
4) **Pytanie:** Dlaczego nie wolno mieszać standard DH z modified DH bez jawnego „kontraktu” na \({}^{i-1}\mathbf{T}_i\)?  
   **Odpowiedź:** Bo ta sama tabela parametrów odpowiada innej kolejności transformacji i innym ramom; wyniki FK będą błędne.
5) **Pytanie:** Co to jest TCP i jak pojawia się w FK?  
   **Odpowiedź:** Punkt/ramy narzędzia; FK dla TCP ma postać \({}^0\mathbf{T}_{TCP}={}^0\mathbf{T}_n\,{}^n\mathbf{T}_{TCP}\).
6) **Pytanie:** Wymień 3 testy weryfikacji FK, które wykonasz przed użyciem IK lub planowania ruchu.  
   **Odpowiedź:** Pozycja referencyjna, test jednej osi naraz, niezmienniki \(SO(3)\) (\(\det\), ortonormalność), porównanie z URDF/symulatorem.

#### 4 zadania obliczeniowe/programistyczne (z poziomem trudności)
1) **(Łatwe)** Dla manipulatora 2R wyprowadź \((x,y)\) i narysuj obszar osiągalny (workspace) dla zadanych ograniczeń \(\theta_1,\theta_2\).  
2) **(Średnie)** Zbuduj tabelę DH dla manipulatora 3R na podstawie rysunku osi i zweryfikuj FK w 10 losowych konfiguracjach testami \(\det(\mathbf{R})\) i \(\mathbf{R}^\top\mathbf{R}\).  
3) **(Średnie)** Porównaj FK z tabeli DH i z URDF: wygeneruj 20 konfiguracji, policz błąd pozy i błąd orientacji (kąt). Zlokalizuj, w którym przegubie jest największa rozbieżność.  
4) **(Trudne)** Zaimplementuj FK zarówno w DH, jak i w POE dla tego samego robota i pokaż numerycznie, że wyniki są zgodne (różnice poniżej tolerancji).

#### 1 projekt laboratoryjny / projekt domowy (z kryteriami oceny)
**Projekt:** „Model FK + walidacja w ROS2 (DH ↔ URDF)”.  
**Wymagania:**  
- Przygotuj tabelę DH dla wybranego manipulatora (realnego lub z symulacji).  
- Zaimplementuj FK w Pythonie i publikuj TF TCP w ROS2.  
- Porównaj TF z FK wynikającym z URDF (np. przez `tf2_echo`) w co najmniej 15 konfiguracjach.  
- Zaproponuj i uruchom testy automatyczne (niezmienniki \(SO(3)\), odwrotność, testy punktów kontrolnych).  
**Kryteria oceny (100 pkt):** poprawność i jednoznaczność ramek (35), jakość walidacji i testów (30), integracja ROS2/TF (20), klarowność raportu (15).

### 7. Quiz sprawdzający (15 pytań: 10 wyboru wielokrotnego + 5 otwartych + klucz odpowiedzi)
#### 10 pytań wyboru wielokrotnego
1) Kinematyka prosta manipulatora zwraca:  
   A) siły w przegubach, B) \({}^0\mathbf{T}_n(\mathbf{q})\), C) mapę 2D, D) MTBF  
2) W klasycznym DH oś \(z\) ramy jest:  
   A) zawsze osią świata, B) osią przegubu, C) osią wspólnej normalnej, D) dowolna  
3) Dla przegubu obrotowego zmienną w DH jest:  
   A) \(a_i\), B) \(\alpha_i\), C) \(d_i\), D) \(\theta_i\)  
4) Parametr \(\alpha_i\) opisuje:  
   A) długość ogniwa, B) kąt między osiami \(z_{i-1}\) i \(z_i\) wokół \(x_i\), C) przesunięcie wzdłuż \(z\), D) obrót wokół \(y\)  
5) Transformacja TCP jest najczęściej:  
   A) zmienna w czasie losowo, B) stała \({}^n\mathbf{T}_{TCP}\), C) zawsze równa \(\mathbf{I}\), D) to samo co \({}^{n-1}\mathbf{T}_n\)  
6) Najczęstszy błąd w implementacji DH to:  
   A) użycie \(\pi\), B) pomylenie konwencji/osi i znaków, C) użycie NumPy, D) normalizacja kwaternionu  
7) Test \(\mathbf{R}^\top\mathbf{R}\approx\mathbf{I}\) sprawdza:  
   A) czy \(\mathbf{R}\) jest diagonalna, B) czy \(\mathbf{R}\in SO(3)\), C) czy \(\mathbf{p}\) jest poprawne, D) czy \(\mathbf{T}\) ma 4 wiersze  
8) Składanie FK w łańcuchu robi się przez:  
   A) dodawanie macierzy, B) mnożenie macierzy w \(SE(3)\), C) transpozycję, D) wyznacznik  
9) POE zapisuje FK jako:  
   A) \(\sum \mathbf{S}_i q_i\), B) \(\prod \exp([\mathbf{S}_i]_\wedge q_i)\mathbf{M}\), C) \(\mathbf{R}^\top\mathbf{p}\), D) \(\mathbf{T}+\mathbf{I}\)  
10) Jeśli FK z DH i FK z URDF różnią się w 20 konfiguracjach, to najbardziej prawdopodobne jest:  
   A) błąd w grafice, B) błąd osi/offsetów lub TCP, C) błąd w MTTR, D) błąd w temperaturze

#### 5 pytań otwartych
11) Opisz procedurę przypisania ramek DH dla dwóch kolejnych osi przegubów, które są równoległe.  
12) Wymień i uzasadnij trzy testy „zdrowego rozsądku” dla FK.  
13) Wyjaśnij, dlaczego błąd TCP może wyglądać jak błąd kinematyki, i jak je rozdzielić w walidacji.  
14) Porównaj DH i POE: w jakich sytuacjach POE jest wygodniejsze?  
15) Podaj przykład, jak FK jest używana w MoveIt2 lub w symulacji do unikania kolizji.

#### Klucz odpowiedzi
1) B, 2) B, 3) D, 4) B, 5) B, 6) B, 7) B, 8) B, 9) B, 10) B.  
11–15) Oceniane opisowo: spójność notacji + argumentacja.

### 8. Bibliografia i materiały dodatkowe
1) J. Denavit, R. S. Hartenberg, *A Kinematic Notation for Lower-Pair Mechanisms Based on Matrices*, Journal of Applied Mechanics, 1955.  
2) J. J. Craig, *Introduction to Robotics: Mechanics and Control*, 3rd ed., Pearson, 2004.  
3) B. Siciliano, L. Sciavicco, L. Villani, G. Oriolo, *Robotics: Modelling, Planning and Control*, Springer, 2009.  
4) K. M. Lynch, F. C. Park, *Modern Robotics: Mechanics, Planning, and Control*, Cambridge University Press, 2017.  
5) P. Corke, *Robotics, Vision and Control*, 2nd ed., Springer, 2017.  
6) R. M. Murray, Z. Li, S. S. Sastry, *A Mathematical Introduction to Robotic Manipulation*, CRC Press, 1994 (POE, skręty).  
7) T. D. Barfoot, *State Estimation for Robotics*, Cambridge University Press, 2017 (grupy Liego i spójność geometrii).  
8) ROS 2 Documentation (Humble i nowsze wydania 2023–2025) — `tf2`, `robot_state_publisher`, `JointState`, dobre praktyki ramek.  
9) URDF / SDFormat Documentation (wydania 2023–2025) — opis osi przegubów, ramek i geometrii kolizyjnej.  
10) MoveIt 2 Documentation (wydania 2023–2025) — FK/IK, planowanie ruchu, walidacja kolizji, integracja z URDF.  
11) NIST, *AI Risk Management Framework (AI RMF 1.0)*, 2023 (weryfikowalność i zarządzanie ryzykiem w systemach adaptacyjnych).  
12) International Federation of Robotics (IFR), *World Robotics* — raporty 2023–2025 (tło wdrożeń i wymagań do 2030).  
13) NVIDIA Isaac Sim / Omniverse Documentation (wydania 2023–2025) — symulacja manipulatorów, ramy, porównanie modeli.  
14) Repozytoria i narzędzia: przykłady FK/DH/POE w materiałach *Modern Robotics* oraz demonstracje testów niezmienników \(SO(3)\).  
15) Materiały wideo: wykłady o kinematyce manipulatorów (np. kurs *Modern Robotics* oraz seminaria z implementacji FK w ROS2/MoveIt2).
