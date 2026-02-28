# Wykład 02

### 1. Tytuł wykładu
Matematyka ruchu: wektory, macierze, rotacje, kwaterniony i podstawy algebry Liego

### 2. Cele nauczania (5–7 punktów)
- Student będzie potrafił zapisywać i przekształcać wektory oraz macierze opisujące położenie i orientację w 3D.
- Student zrozumie własności macierzy obrotu i warunek przynależności do grupy \(SO(3)\) (ortonormalność i wyznacznik \(+1\)).
- Student pozna równoważne reprezentacje orientacji: oś–kąt, macierz obrotu, kwaternion oraz ich zalety i ograniczenia numeryczne.
- Student będzie potrafił przechodzić między reprezentacjami (R ↔ kwaternion ↔ oś–kąt) i świadomie dobierać metrykę błędu orientacji.
- Student zrozumie ideę algebry Liego \(\mathfrak{so}(3)\) i mapy wykładniczej/logarytmicznej jako narzędzi do „liniaryzacji” ruchu obrotowego.
- Student będzie potrafił zapisać i interpretować prędkość kątową \(\boldsymbol{\omega}\) oraz związek \(\dot{\mathbf{R}} = \mathbf{R}[\boldsymbol{\omega}]_\times\).
- Student pozna, dlaczego geometria \(SO(3)\)/\(SE(3)\) będzie kluczowa dla robotyki 2030 (modele uczone z priorytetami geometrycznymi, cyfrowe bliźniaki, weryfikowalne ograniczenia).

### 3. Wstęp teoretyczny
#### 3.1. Dlaczego „matematyka ruchu” jest sercem robotyki
Robot porusza się w przestrzeni trójwymiarowej, ale opis tego ruchu nie jest trywialny: położenie żyje w \(\mathbb{R}^3\), natomiast orientacja nie jest wektorem w \(\mathbb{R}^3\), tylko elementem zbioru o strukturze grupy — \(SO(3)\). To rozróżnienie jest praktyczne, a nie akademickie: błędny zapis orientacji prowadzi do osobliwości (np. „zablokowanie” kątów Eulera), niestabilności numerycznej i błędów sterowania. W kolejnych wykładach będziemy budować kinematykę i dynamikę (manipulatory, roboty mobilne), a wszędzie tam pojawią się rotacje, transformacje i ich pochodne (Craig, 2004; Siciliano i Khatib, 2016; Lynch i Park, 2017).

W tym wykładzie budujemy wspólny język, który będzie spójny w całym kursie:
- wektory i macierze jako obiekty geometryczne (nie tylko „tablice liczb”),
- macierze obrotu \(\mathbf{R}\in SO(3)\) i ich własności,
- kwaterniony jednostkowe jako praktyczna reprezentacja orientacji,
- podstawy algebry Liego jako narzędzie do różniczkowania i „liniaryzacji” na rozmaitościach.

#### 3.2. Wektory, normy i iloczyny: narzędzia do opisu geometrii
Niech \(\mathbf{x}\in\mathbb{R}^n\) będzie wektorem. W robotyce dominują normy:
\[
\|\mathbf{x}\|_2 = \sqrt{\mathbf{x}^\top \mathbf{x}}, \qquad
\|\mathbf{x}\|_1 = \sum_i |x_i|, \qquad
\|\mathbf{x}\|_\infty = \max_i |x_i|.
\]
Norma \(\ell_2\) jest naturalna geometrycznie (długość), ale w zadaniach optymalizacji i odporności na wartości odstające pojawiają się też normy \(\ell_1\) i \(\ell_\infty\). W praktyce wybór normy wpływa na to, czy rozwiązanie „wyrównuje średnio”, czy „chroni najgorszy przypadek”.

Iloczyn skalarny \(\mathbf{a}^\top\mathbf{b} = \|\mathbf{a}\|\,\|\mathbf{b}\|\cos\theta\) daje kąt między wektorami, a iloczyn wektorowy w \(\mathbb{R}^3\):
\[
\mathbf{a}\times\mathbf{b} =
\begin{bmatrix}
a_2b_3-a_3b_2\\
a_3b_1-a_1b_3\\
a_1b_2-a_2b_1
\end{bmatrix}
\]
jest kluczowy dla opisu momentów, prędkości kątowej i kinematyki różniczkowej. Wygodnie wprowadzić operator „daszka” (macierz antysymetryczną), który koduje iloczyn wektorowy jako mnożenie macierzy:
\[
[\mathbf{a}]_\times =
\begin{bmatrix}
0 & -a_3 & a_2\\
a_3 & 0 & -a_1\\
-a_2 & a_1 & 0
\end{bmatrix},\qquad
[\mathbf{a}]_\times \mathbf{b} = \mathbf{a}\times\mathbf{b}.
\]
To pozornie drobny trik jest fundamentem algebry \(\mathfrak{so}(3)\) i w praktyce upraszcza wyprowadzenia (Barfoot, 2017).

#### 3.3. Macierze jako przekształcenia liniowe i własności ortonormalności
W robotyce macierze traktujemy jako przekształcenia: \(\mathbf{y}=\mathbf{A}\mathbf{x}\). Szczególną rolę grają macierze ortogonalne:
\[
\mathbf{Q}^\top \mathbf{Q} = \mathbf{I}.
\]
Takie macierze zachowują długości i kąty (są „sztywne” w sensie geometrii). Macierze obrotu są ortogonalne oraz mają wyznacznik \(+1\) (eliminujemy odbicia lustrzane):
\[
SO(3)=\{\mathbf{R}\in\mathbb{R}^{3\times 3}\mid \mathbf{R}^\top\mathbf{R}=\mathbf{I},\ \det(\mathbf{R})=+1\}.
\]
To definicja, która od razu daje praktyczne testy poprawności: jeśli w obliczeniach numerycznych \(\mathbf{R}^\top\mathbf{R}\neq\mathbf{I}\), to „uciekliśmy” z \(SO(3)\) i powinniśmy zastosować renormalizację (np. przez rzutowanie na najbliższą macierz ortogonalną metodą SVD).

Warto pamiętać o subtelności: przestrzeń orientacji nie jest euklidesowa. Dodawanie dwóch macierzy obrotu i dzielenie przez 2 nie daje obrotu. To jest jedna z przyczyn, dla których w estymacji i sterowaniu stosuje się narzędzia z teorii grup Liego (Barfoot, 2017).

#### 3.4. Rotacje w 3D: macierze obrotu, kąty Eulera i ich ograniczenia
Najbardziej bezpośrednią reprezentacją orientacji jest macierz \(\mathbf{R}\). Jej kolumny są osiami układu lokalnego wyrażonymi w układzie globalnym. Z punktu widzenia geometrii jest to opis jednoznaczny i pozbawiony osobliwości. Wadą jest redundancja (9 liczb, 3 stopnie swobody) i koszt obliczeń.

W praktyce często spotkasz **kąty Eulera** (np. yaw–pitch–roll). Dla ustalonej konwencji (np. \(Z\!Y\!X\)) można zapisać:
\[
\mathbf{R} = \mathbf{R}_z(\psi)\mathbf{R}_y(\theta)\mathbf{R}_x(\phi).
\]
Kąty Eulera są intuicyjne, ale mają zasadniczą wadę: istnieją konfiguracje, w których reprezentacja staje się osobliwa (*gimbal lock*), a mała zmiana orientacji może odpowiadać dużej zmianie kątów. W sterowaniu i estymacji to prosta droga do problemów numerycznych. Dlatego w systemach robotycznych (zwłaszcza 3D: drony, humanoidy, manipulacja w przestrzeni) standardem są macierze obrotu i kwaterniony (Corke, 2017; Lynch i Park, 2017).

Poniższa tabela streszcza podstawowe reprezentacje orientacji.

| Reprezentacja | Liczba parametrów | Osobliwości | Typowe zalety | Typowe wady |
|---|---:|---|---|---|
| Kąty Eulera | 3 | tak | intuicyjne dla człowieka | osobliwości, trudna interpolacja |
| Macierz \(\mathbf{R}\) | 9 (z więzami) | nie | stabilna, jednoznaczna | redundancja, koszt |
| Oś–kąt \((\mathbf{u},\theta)\) | 3 (wektor w praktyce) | nie (poza \(\theta\approx\pi\)) | blisko geometrii | trudniejsza obsługa numeryczna |
| Kwaternion \(q\) | 4 (z więzem) | nie | dobra interpolacja, wydajność | podwójne pokrycie \(q\sim -q\) |

#### 3.5. Oś–kąt i wzór Rodriguesa: „najkrótszy” opis obrotu
Każdy obrót w 3D (z wyjątkiem szczególnych przypadków) można opisać przez oś jednostkową \(\mathbf{u}\) i kąt \(\theta\): obrót o \(\theta\) wokół \(\mathbf{u}\). Wygodnie używa się wektora \(\boldsymbol{\phi}=\theta\mathbf{u}\in\mathbb{R}^3\), nazywanego czasem „wektorem obrotu”. Związek z macierzą obrotu wyraża wzór Rodriguesa:
\[
\mathbf{R}(\mathbf{u},\theta)=\mathbf{I}+\sin\theta\,[\mathbf{u}]_\times + (1-\cos\theta)\,[\mathbf{u}]_\times^2.
\]
To równanie jest praktyczne: pozwala konstruować \(\mathbf{R}\) z 3 parametrów i daje intuicję, jak obrót zależy od kąta. Dodatkowo, dla małych kątów \(\theta\) mamy przybliżenia:
\[
\sin\theta \approx \theta,\qquad 1-\cos\theta\approx \frac{\theta^2}{2},
\]
co prowadzi do pierwszego przybliżenia:
\[
\mathbf{R}\approx \mathbf{I}+[\boldsymbol{\phi}]_\times,
\]
czyli „mały obrót” zachowuje się prawie liniowo w \(\boldsymbol{\phi}\). To jest punkt wyjścia do liniaryzacji w filtrach i regulatorach.

#### 3.6. Algebra Liego \(\mathfrak{so}(3)\): skąd bierze się \(\dot{\mathbf{R}} = \mathbf{R}[\boldsymbol{\omega}]_\times\)
Orientacje tworzą grupę \(SO(3)\), a jej „przestrzeń styczna w jedności” jest algebrą Liego \(\mathfrak{so}(3)\), czyli zbiorem macierzy antysymetrycznych. Operator \([\cdot]_\times\) mapuje wektor \(\boldsymbol{\omega}\in\mathbb{R}^3\) na element \(\mathfrak{so}(3)\).

Jeśli \(\mathbf{R}(t)\) opisuje orientację w czasie, to jej pochodna spełnia:
\[
\dot{\mathbf{R}} = \mathbf{R}[\boldsymbol{\omega}]_\times,
\]
gdzie \(\boldsymbol{\omega}\) jest prędkością kątową wyrażoną w układzie ciała (tzw. *body rate*). Alternatywnie spotyka się zapis:
\[
\dot{\mathbf{R}} = [\boldsymbol{\Omega}]_\times \mathbf{R},
\]
gdzie \(\boldsymbol{\Omega}\) jest prędkością kątową w układzie świata (tzw. *space rate*). Różnica nie jest kosmetyczna: wpływa na znaki i na to, gdzie w równaniach pojawia się \(\mathbf{R}\). W robotyce często operuje się obiema wersjami, dlatego w kursie będziemy zawsze doprecyzowywać, w jakim układzie jest wyrażony wektor prędkości.

W praktyce często potrzebujemy „odwrócić” Rodriguesa: z danej \(\mathbf{R}\) uzyskać \(\boldsymbol{\phi}\) (mapa logarytmiczna). W najprostszym ujęciu:
\[
\theta = \arccos\left(\frac{\mathrm{tr}(\mathbf{R})-1}{2}\right),
\]
a oś można wydobyć z części antysymetrycznej \(\mathbf{R}-\mathbf{R}^\top\). W implementacjach trzeba jednak uważać na przypadki \(\theta\approx 0\) i \(\theta\approx \pi\) (problemy numeryczne i niejednoznaczność osi). To właśnie w tych miejscach „porządna matematyka” ratuje praktykę.

#### 3.7. Kwaterniony: reprezentacja praktyczna i stabilna numerycznie
Kwaternion \(q\) zapisujemy jako:
\[
q = \begin{bmatrix} q_w \\ \mathbf{q}_v \end{bmatrix} =
\begin{bmatrix} q_w \\ q_x \\ q_y \\ q_z \end{bmatrix},
\qquad \|q\|_2=1.
\]
Orientacji odpowiadają kwaterniony jednostkowe (sfera \(S^3\)), przy czym zachodzi podwójne pokrycie: \(q\) i \(-q\) opisują ten sam obrót. Związek oś–kąt ↔ kwaternion:
\[
q_w = \cos\frac{\theta}{2},\qquad \mathbf{q}_v = \mathbf{u}\sin\frac{\theta}{2}.
\]

Składanie obrotów realizuje się przez iloczyn Hamiltona (mnożenie kwaternionów). Dla \(q_1=[w_1,\mathbf{v}_1]\) i \(q_2=[w_2,\mathbf{v}_2]\):
\[
q_1\otimes q_2 =
\begin{bmatrix}
w_1w_2 - \mathbf{v}_1^\top\mathbf{v}_2\\
w_1\mathbf{v}_2 + w_2\mathbf{v}_1 + \mathbf{v}_1\times \mathbf{v}_2
\end{bmatrix}.
\]
To równanie pokazuje, dlaczego operator \(\times\) jest wszędzie w robotyce 3D.

Największą przewagą kwaternionów w zastosowaniach jest **interpolacja**. Jeśli chcesz płynnie przejść między orientacjami \(q_a\) i \(q_b\), standardem jest interpolacja sferyczna (SLERP):
\[
\mathrm{slerp}(q_a,q_b;\alpha) = \frac{\sin((1-\alpha)\Theta)}{\sin\Theta}q_a + \frac{\sin(\alpha\Theta)}{\sin\Theta}q_b,
\]
gdzie \(\Theta=\arccos(q_a^\top q_b)\) i \(\alpha\in[0,1]\). W praktyce trzeba jeszcze zadbać o to, by wybrać krótszą drogę na \(S^3\) (jeśli \(q_a^\top q_b<0\), to zastępujemy \(q_b\leftarrow -q_b\)).

#### 3.8. Metryki błędu orientacji: co znaczy „mały błąd” na \(SO(3)\)
W \(\mathbb{R}^3\) naturalna jest różnica \(\mathbf{x}-\mathbf{x}^\*\). Dla orientacji analogiem jest błąd względny:
\[
\mathbf{R}_{\text{err}} = \mathbf{R}^{\*\top}\mathbf{R},
\]
czyli obrót, który „domyka” orientację zadaną do aktualnej. Następnie ten obrót sprowadza się do wektora w \(\mathbb{R}^3\) przez logarytm:
\[
\boldsymbol{\phi} = \log\left(\mathbf{R}_{\text{err}}\right)^\vee,
\]
gdzie \((\cdot)^\vee\) jest operatorem odwrotnym do \([\cdot]_\times\) (z macierzy antysymetrycznej na wektor). Wtedy \(\|\boldsymbol{\phi}\|\) ma sens „kątowy” i można ją używać w regulatorach i filtrach. Dla małych błędów \(\boldsymbol{\phi}\) zachowuje się jak klasyczny błąd w \(\mathbb{R}^3\).

W praktyce spotyka się też metrykę śladu:
\[
d(\mathbf{R}_1,\mathbf{R}_2)=\arccos\left(\frac{\mathrm{tr}(\mathbf{R}_1^\top\mathbf{R}_2)-1}{2}\right),
\]
która jest po prostu kątem obrotu między orientacjami. Jest użyteczna w ocenie jakości algorytmów (np. w estymacji).

#### 3.9. Grupy Liego i „liniaryzacja na rozmaitości”: ogólna idea
W robotyce często mamy dwa, pozornie sprzeczne, wymagania:
1) chcemy opisywać ruch poprawnie geometrycznie (na \(SO(3)\) i \(SE(3)\)),  
2) chcemy korzystać z narzędzi liniowych (optymalizacja kwadratowa, filtry liniowe, regulator LQR).

Właśnie do tego służy formalizm grup Liego. W skrócie: elementy grupy (np. \(\mathbf{R}\in SO(3)\)) są „nieliniowe”, ale ich przyrosty w pobliżu punktu można opisywać w algebrze Liego (np. \([\boldsymbol{\phi}]_\times\in\mathfrak{so}(3)\)), która jest przestrzenią liniową. Mapę między nimi daje wykładnicza:
\[
\mathbf{R} = \exp([\boldsymbol{\phi}]_\times),\qquad [\boldsymbol{\phi}]_\times = \log(\mathbf{R}).
\]
W praktyce to oznacza, że:
- estymację i sterowanie projektujemy „w małym” w \(\boldsymbol{\phi}\in\mathbb{R}^3\),
- a stan właściwy przechowujemy jako \(\mathbf{R}\) lub \(q\), dbając, by nie łamać więzów (ortonormalność/jednostkowość).

To podejście jest standardem we współczesnej estymacji stanu w robotyce (np. w wariantach filtrów Kalmana na grupach Liego) oraz w optymalizacji trajektorii i SLAM (Barfoot, 2017).

#### 3.10. Zapowiedź \(SE(3)\): położenie i orientacja w jednej strukturze
W następnym wykładzie przejdziemy do transformacji jednorodnych, ale warto już teraz zobaczyć ideę. Położenie \(\mathbf{p}\in\mathbb{R}^3\) i orientację \(\mathbf{R}\in SO(3)\) łączy się w element \(SE(3)\):
\[
\mathbf{T}=
\begin{bmatrix}
\mathbf{R} & \mathbf{p}\\
\mathbf{0}^\top & 1
\end{bmatrix}.
\]
Podobnie jak dla \(SO(3)\), istnieje algebra \(\mathfrak{se}(3)\), w której naturalnym obiektem jest „skręt” (twist) łączący prędkość kątową i liniową:
\[
\boldsymbol{\xi}=
\begin{bmatrix}
\boldsymbol{\omega}\\
\mathbf{v}
\end{bmatrix}\in\mathbb{R}^6,\qquad
[\boldsymbol{\xi}]_\wedge=
\begin{bmatrix}
[\boldsymbol{\omega}]_\times & \mathbf{v}\\
\mathbf{0}^\top & 0
\end{bmatrix}.
\]
Wtedy można pisać (analogicznie do \(\dot{\mathbf{R}}\)):
\[
\dot{\mathbf{T}} = \mathbf{T}[\boldsymbol{\xi}]_\wedge,
\]
co stanie się „językiem roboczym” przy kinematyce i w SLAM. Na tym etapie nie musisz jeszcze pamiętać wszystkich symboli; ważne jest zrozumienie zasady: **ruch jest poprawnie opisywany przez działanie grupowe, a różniczkowanie odbywa się w algebrze**.

#### 3.11. Stabilność numeryczna: normalizacja, rzutowanie i „małe błędy, duże konsekwencje”
W obliczeniach na komputerze:
- kwaternion może przestać być jednostkowy (\(\|q\|\neq 1\)) wskutek błędów zaokrągleń,
- macierz obrotu może utracić ortonormalność (\(\mathbf{R}^\top\mathbf{R}\neq \mathbf{I}\)).

Praktyczna zasada inżynierska brzmi: **po każdej operacji, która może wprowadzać błąd, normalizuj reprezentację**. Dla kwaternionu to proste:
\[
q \leftarrow \frac{q}{\|q\|}.
\]
Dla macierzy obrotu często stosuje się rzutowanie SVD: jeśli \(\mathbf{R}\approx \mathbf{U}\mathbf{\Sigma}\mathbf{V}^\top\), to najbliższa macierz w \(SO(3)\) (w sensie normy Frobeniusa) to \(\mathbf{R}_{\text{proj}}=\mathbf{U}\mathbf{V}^\top\), z ewentualną korektą znaku, by wymusić \(\det=+1\).

Druga praktyczna zasada: **unikaj bezpośredniej integracji kątów Eulera w 3D**, jeśli system wykonuje duże obroty. Integracja prędkości kątowej jest naturalna w formalizmie \(\exp(\cdot)\) lub w kwaternionach. Już w prostych układach (dron, głowica kamery) prowadzi to do wyraźnie stabilniejszego zachowania.

#### 3.12. Dlaczego to będzie ważniejsze do 2030: geometria jako „twarde ograniczenie” dla uczenia
W latach 2023–2025 w robotyce wzrosło znaczenie podejść, które łączą uczenie z priorytetami geometrycznymi: sieci równoważne względem obrotów i przesunięć (równoważność \(SE(3)\)), modele 3D operujące na chmurach punktów, oraz sterowanie oparte o reprezentacje, które nie „łamią” geometrii. W praktyce oznacza to, że:
- dane uczące i symulacje muszą zachować spójność układów odniesienia,
- metryki błędu orientacji muszą być definiowane na \(SO(3)\), a nie „na składowych”,
- elementy takie jak \(\exp/\log\) na \(SO(3)\) i \(SE(3)\) coraz częściej pojawiają się wewnątrz pętli uczenia i optymalizacji.

Prognoza do 2030 jest następująca: podstawy geometrii ruchu (dzisiejszy wykład) staną się dla inżyniera robotyki tym, czym dla inżyniera oprogramowania są typy i testy — minimalnym mechanizmem gwarancji. Modele uczone (w tym modele fundamentalne dla robotów) będą coraz lepsze w generalizacji, ale ich integracja z fizyką i bezpieczeństwem wymaga precyzyjnego aparatu matematycznego: aby odróżniać błąd modelu od błędu reprezentacji, aby projektować ograniczenia i aby interpretować wyniki w sposób audytowalny (por. dyskusje o ryzyku i walidacji w NIST AI RMF, 2023).

Jako reprezentatywne przykłady nurtu „ucieleśnionych modeli” z lat 2023–2025 warto śledzić prace, które łączą percepcję wielomodalną i sterowanie oraz pokazują, jak wiedza ogólna przenosi się do działań robota (np. PaLM-E, RT-2) oraz podejścia uczące polityki ruchu o wysokiej jakości trajektorii (np. Diffusion Policy). Wspólnym mianownikiem tych kierunków jest to, że sukces zależy od poprawnej geometrii: model może „wymyślić” cel, ale to \(\exp/\log\), metryki na \(SO(3)\) i spójne ramy odniesienia decydują, czy robot wykona ruch bezpiecznie i powtarzalnie.

**Odniesienia w tekście:** pozycje z sekcji 8 (pełna bibliografia).

#### 3.13. Przejścia między reprezentacjami: wzory, które trzeba umieć „z grubsza”
W praktyce inżynierskiej nie chodzi o to, by recytować wszystkie wzory z pamięci, lecz by:
1) rozumieć, *które przejścia są potrzebne w danej warstwie systemu*,  
2) wiedzieć, *gdzie są pułapki numeryczne*,  
3) umieć zweryfikować wynik prostymi testami (ortonormalność, jednostkowość).

Najczęściej używane przejścia to:

**(a) Kwaternion \(\rightarrow\) macierz obrotu.** Dla \(q=[q_w,q_x,q_y,q_z]^\top\) (jednostkowego) standardowa postać:
\[
\mathbf{R}(q)=
\begin{bmatrix}
1-2(q_y^2+q_z^2) & 2(q_xq_y-q_zq_w) & 2(q_xq_z+q_yq_w)\\
2(q_xq_y+q_zq_w) & 1-2(q_x^2+q_z^2) & 2(q_yq_z-q_xq_w)\\
2(q_xq_z-q_yq_w) & 2(q_yq_z+q_xq_w) & 1-2(q_x^2+q_y^2)
\end{bmatrix}.
\]
Kontrola poprawności: \(\mathbf{R}^\top\mathbf{R}\approx \mathbf{I}\) i \(\det(\mathbf{R})\approx 1\).

**(b) Macierz obrotu \(\rightarrow\) kwaternion.** Istnieje kilka algorytmów; typowo wykorzystuje się ślad \(\mathrm{tr}(\mathbf{R})\) i wybiera najstabilniejszą gałąź obliczeń. Pułapka: gdy \(\theta\approx\pi\), ślad jest bliski \(-1\) i proste wzory są wrażliwe na szum.

**(c) Macierz obrotu \(\leftrightarrow\) oś–kąt (wektor obrotu).** Jak wspomniano wcześniej:
\[
\theta=\arccos\left(\frac{\mathrm{tr}(\mathbf{R})-1}{2}\right),
\]
natomiast wektor obrotu \(\boldsymbol{\phi}\) w pobliżu zera można aproksymować z części antysymetrycznej:
\[
[\boldsymbol{\phi}]_\times \approx \frac{\mathbf{R}-\mathbf{R}^\top}{2}.
\]
W praktyce implementacje używają rozwinięć w szereg dla \(\theta\to 0\), aby uniknąć dzielenia przez małe liczby.

Warto też zapamiętać jedną zasadę „inżynierskiej higieny”: jeśli otrzymasz kwaternion z estymatora lub z biblioteki, zawsze doprowadź go do postaci o dodatniej części skalarnej (np. wymuś \(q_w\ge 0\)). Nie zmienia to orientacji (bo \(q\sim -q\)), a ułatwia ciągłość w czasie i interpretację.

#### 3.14. Różniczkowanie orientacji: prędkość kątowa, pochodne i więzy
W sterowaniu i estymacji rzadko wystarcza sama orientacja; potrzebujemy jej pochodnej i związku z prędkością kątową. Dla macierzy obrotu pokazaliśmy relację:
\[
\dot{\mathbf{R}} = \mathbf{R}[\boldsymbol{\omega}]_\times.
\]
Zauważ, że \(\dot{\mathbf{R}}\) nie jest „dowolną” macierzą: wynika z więzów \(SO(3)\). To ważne np. przy liniaryzacji — nie wolno traktować 9 elementów \(\mathbf{R}\) jako niezależnych.

Dla kwaternionu często używa się równania kinematyki:
\[
\dot q = \frac{1}{2}\,\mathbf{Q}(\boldsymbol{\omega})\,q,
\]
gdzie \(\mathbf{Q}(\boldsymbol{\omega})\) jest macierzą liniową w \(\boldsymbol{\omega}\). Jedna z popularnych postaci (dla \(\boldsymbol{\omega}\) w układzie ciała) to:
\[
\mathbf{Q}(\boldsymbol{\omega})=
\begin{bmatrix}
0 & -\omega_x & -\omega_y & -\omega_z\\
\omega_x & 0 & \omega_z & -\omega_y\\
\omega_y & -\omega_z & 0 & \omega_x\\
\omega_z & \omega_y & -\omega_x & 0
\end{bmatrix}.
\]
To równanie jest praktyczne w symulacji i filtracji: integrujesz \(\dot q\), a następnie normalizujesz \(q\leftarrow q/\|q\|\). Warto jednak pamiętać, że „zwykły” całkownik Eulera może wprowadzać istotny błąd przy dużych prędkościach kątowych; dlatego w krytycznych zastosowaniach stosuje się integratory geometryczne lub krokowanie przez \(\exp\) na \(SO(3)\).

Z punktu widzenia robotyki „pełnego stosu” to miejsce, gdzie geometria styka się z implementacją: poprawny model matematyczny musi być sparowany z poprawną metodą numeryczną. Do 2030, wraz z rosnącą rolą cyfrowych bliźniaków i uczenia w symulacji, umiejętność oceny błędu numerycznego (a nie tylko błędu modelu) będzie jednym z czynników odróżniających inżyniera od „użytkownika bibliotek”.

#### 3.15. Perturbacje na \(SO(3)\): „mały błąd” jako wektor w \(\mathbb{R}^3\)
W wielu algorytmach (kalibracja, estymacja, optymalizacja trajektorii) potrzebujemy zapisać „małą poprawkę” orientacji. W przestrzeni euklidesowej robimy to przez dodanie: \(\mathbf{x}\leftarrow \mathbf{x}+\delta\mathbf{x}\). Na \(SO(3)\) naturalnym odpowiednikiem jest mnożenie przez mały obrót:
\[
\mathbf{R} \leftarrow \mathbf{R}\exp([\delta\boldsymbol{\phi}]_\times),
\]
albo (w innej konwencji) \(\mathbf{R}\leftarrow \exp([\delta\boldsymbol{\phi}]_\times)\mathbf{R}\). Wektor \(\delta\boldsymbol{\phi}\in\mathbb{R}^3\) jest wtedy „parametrem perturbacji” i można go traktować jak zwykłą zmienną w metodach gradientowych. To jest dokładnie ten mechanizm, dzięki któremu można robić optymalizację „jak w \(\mathbb{R}^n\)”, a jednocześnie nie łamać geometrii \(SO(3)\).

Analogicznie w \(SE(3)\) perturbacje realizuje się przez:
\[
\mathbf{T} \leftarrow \mathbf{T}\exp([\delta\boldsymbol{\xi}]_\wedge),\qquad \delta\boldsymbol{\xi}\in\mathbb{R}^6,
\]
co będzie kluczowe w wykładach o SLAM i planowaniu. Już na tym etapie warto zapamiętać: **to nie jest „sztuczka matematyczna”, tylko narzędzie, które pozwala pisać poprawne algorytmy numeryczne**. W praktyce, jeśli kiedykolwiek zobaczysz, że ktoś „uśrednia” macierze obrotu przez zwykłą średnią arytmetyczną, to jest to sygnał ostrzegawczy — uśrednianie musi respektować strukturę grupy (np. przez średnią na rozmaitości).

W kolejnych wykładach będziemy wielokrotnie wracać do tej idei w przebraniu „Jacobiana” i liniaryzacji modeli. Gdy będziemy liczyć wpływ małej zmiany przegubu na położenie i orientację końcówki roboczej, tak naprawdę będziemy przeliczać perturbacje w algebrze na perturbacje w przestrzeni zadaniowej. Jeśli ta warstwa jest opanowana, to nagle wiele „trudnych” fragmentów robotyki staje się konsekwentnym rachunkiem, a nie zbiorem sztuczek.

Na poziomie praktycznym warto też pamiętać o kulturze weryfikacji: każdą implementację konwersji i błędu orientacji testuje się na przypadkach brzegowych (\(\theta\to 0\), \(\theta\to \pi\)), na losowych obrotach oraz na niezmiennikach (\(\det(\mathbf{R})=1\), \(\|q\|=1\), symetrie \(q\sim -q\)). Takie testy są „tańsze” niż debugowanie regulatora, który zachowuje się źle wyłącznie dlatego, że gdzieś po drodze pomylono konwencję osi.

W tym kursie będziemy konsekwentnie stosować te same konwencje i testy.

### 4. Struktura prezentacji slajdów (PowerPoint / Google Slides)
**Założenie stylu:** Assertion–Evidence — tytuł slajdu jest tezą; treść to dowód (rysunek/animacja/wykres); tekst maksymalnie oszczędny.

1. **Slajd nr 1 | Orientacja nie jest wektorem — żyje na \(SO(3)\)**  
   - Treść:  
     - Położenie: \(\mathbb{R}^3\)  
     - Orientacja: \(SO(3)\)  
     - Skutek: inne metryki i pochodne  
   - Sugestie wizualne: grafika „mapa przestrzeni”: sześcian \(\mathbb{R}^3\) obok zakrzywionej rozmaitości \(SO(3)\) (symbolicznie jako kula z osiami), z podpisem „nie dodawaj orientacji jak wektorów”.  
   - Notatki dla prowadzącego: 2 min; podać przykład błędu: uśrednianie macierzy obrotu przez średnią arytmetyczną.

2. **Slajd nr 2 | Iloczyn wektorowy można zapisać jako mnożenie macierzy**  
   - Treść:  
     - \([\mathbf{a}]_\times \mathbf{b} = \mathbf{a}\times\mathbf{b}\)  
     - \([\mathbf{a}]_\times^\top = -[\mathbf{a}]_\times\)  
   - Sugestie wizualne: animacja: wektory \(\mathbf{a},\mathbf{b}\) w 3D oraz wynik \(\mathbf{a}\times\mathbf{b}\); obok pojawia się macierz \([\mathbf{a}]_\times\) i to samo wyjście.  
   - Notatki: 2 min; podkreślić: „to brama do \(\mathfrak{so}(3)\)”.

3. **Slajd nr 3 | Macierz obrotu to macierz ortogonalna o wyznaczniku \(+1\)**  
   - Treść:  
     - \(\mathbf{R}^\top\mathbf{R}=\mathbf{I}\)  
     - \(\det(\mathbf{R})=1\)  
     - Kolumny \(\mathbf{R}\) = osie układu lokalnego  
   - Sugestie wizualne: trzy wektory osi lokalnych (czerwony/zielony/niebieski) osadzone w świecie; obok dwa „testy poprawności” jako check-lista.  
   - Notatki: 3 min; pytanie: „Jak wykryć, że błąd numeryczny zepsuł obrót?”

4. **Slajd nr 4 | Kąty Eulera są intuicyjne, ale mają osobliwości**  
   - Treść:  
     - \(\mathbf{R}=\mathbf{R}_z(\psi)\mathbf{R}_y(\theta)\mathbf{R}_x(\phi)\)  
     - „gimbal lock” dla pewnych \(\theta\)  
   - Sugestie wizualne: animacja kardana (3 pierścienie) i utrata stopnia swobody; obok wykres „skoków” kątów Eulera przy płynnym obrocie.  
   - Notatki: 3 min; podkreślić: „nie zakazujemy Eulera, ale rozumiemy ograniczenia”.

5. **Slajd nr 5 | Oś–kąt daje minimalny opis obrotu w 3 parametrach**  
   - Treść:  
     - \(\boldsymbol{\phi}=\theta\mathbf{u}\)  
     - Intuicja: „wektor obrotu”  
   - Sugestie wizualne: strzałka \(\mathbf{u}\) jako oś, pierścień pokazujący kąt \(\theta\); podpis „ten sam obrót = ta sama oś i kąt”.  
   - Notatki: 2 min; zapowiedzieć Rodriguesa.

6. **Slajd nr 6 | Wzór Rodriguesa implementuje mapę wykładniczą na \(SO(3)\)**  
   - Treść:  
     - \(\mathbf{R}=\mathbf{I}+\sin\theta[\mathbf{u}]_\times+(1-\cos\theta)[\mathbf{u}]_\times^2\)  
   - Sugestie wizualne: na ekranie jedno duże równanie; w tle animacja: obrót osi lokalnych o \(\theta\) wokół \(\mathbf{u}\), zsynchronizowana ze zmianą \(\sin\theta\) i \(1-\cos\theta\).  
   - Notatki: 3 min; zaznaczyć, że to „most” między geometrią a rachunkiem.

7. **Slajd nr 7 | Mały obrót jest prawie liniowy: \(\mathbf{R}\approx \mathbf{I}+[\boldsymbol{\phi}]_\times\)**  
   - Treść:  
     - \(\sin\theta\approx\theta\)  
     - \(1-\cos\theta\approx \theta^2/2\)  
     - Zastosowanie: liniaryzacja filtrów/regulatorów  
   - Sugestie wizualne: wykres błędu przybliżenia vs \(\theta\) (0–30°); obok mini-animacja „mały obrót” i strzałka „liniowe”.  
   - Notatki: 2 min; połączyć z Jacobianem w kolejnych wykładach.

8. **Slajd nr 8 | Pochodna orientacji ma strukturę: \(\dot{\mathbf{R}}=\mathbf{R}[\boldsymbol{\omega}]_\times\)**  
   - Treść:  
     - \(\boldsymbol{\omega}\): prędkość kątowa  
     - Układ ciała vs układ świata (konwencja!)  
   - Sugestie wizualne: krótka animacja: obracający się układ osi + wektor \(\boldsymbol{\omega}\); obok dwie ramki „body rate” i „space rate”.  
   - Notatki: 3 min; ostrzec przed typową pomyłką znaków i kolejności mnożenia.

9. **Slajd nr 9 | Kwaterniony unikają osobliwości i ułatwiają interpolację**  
   - Treść:  
     - \(\|q\|=1\), \(q\sim -q\)  
     - Oś–kąt: \(q_w=\cos(\theta/2)\), \(\mathbf{q}_v=\mathbf{u}\sin(\theta/2)\)  
   - Sugestie wizualne: wizualizacja \(S^3\) jako „hiperkuli” przez analogię: okrąg \(S^1\) i kula \(S^2\); dopisek „orientacje = punkty na \(S^3\)”.  
   - Notatki: 3 min; podkreślić podwójne pokrycie i konsekwencje.

10. **Slajd nr 10 | SLERP daje stałą prędkość kątową w interpolacji**  
   - Treść:  
     - \(\mathrm{slerp}(q_a,q_b;\alpha)\)  
     - Zawsze wybieramy krótszą drogę (\(q_b\leftarrow -q_b\) gdy potrzeba)  
   - Sugestie wizualne: porównanie dwóch trajektorii interpolacji: liniowa w \(\mathbb{R}^4\) (zniekształcona) vs SLERP na sferze; podpis „stała prędkość”.  
   - Notatki: 2 min; zastosowanie: planowanie orientacji chwytaka/kamery.

11. **Slajd nr 11 | Błąd orientacji liczymy jako obrót względny, nie różnicę składowych**  
   - Treść:  
     - \(\mathbf{R}_{err}=\mathbf{R}^{*\top}\mathbf{R}\)  
     - \(d=\arccos((\mathrm{tr}(\mathbf{R}_{err})-1)/2)\)  
   - Sugestie wizualne: dwie orientacje osi (zadana i aktualna) + trzeci układ pokazujący \(\mathbf{R}_{err}\); obok liczba \(d\) jako „kąt błędu”.  
   - Notatki: 3 min; to będzie „metryka” w ocenie algorytmów i w regulatorach.

12. **Slajd nr 12 | Logarytm na \(SO(3)\) daje wektor błędu w \(\mathbb{R}^3\)**  
   - Treść:  
     - \(\boldsymbol{\phi}=\log(\mathbf{R}_{err})^\vee\)  
     - \(\|\boldsymbol{\phi}\|\approx d\) dla małych błędów  
   - Sugestie wizualne: „lej” z \(SO(3)\) do \(\mathbb{R}^3\): obraz rozmaitości → strzałka → wektor \(\boldsymbol{\phi}\) w przestrzeni; podpis „liniaryzacja”.  
   - Notatki: 3 min; intuicja: „w algebrze można dodawać”.

13. **Slajd nr 13 | Perturbacja na \(SO(3)\) to mnożenie przez mały obrót**  
   - Treść:  
     - \(\mathbf{R}\leftarrow \mathbf{R}\exp([\delta\boldsymbol{\phi}]_\times)\)  
     - Nigdy: \(\mathbf{R}\leftarrow \mathbf{R}+\delta\mathbf{R}\)  
   - Sugestie wizualne: dwa panele „dobrze” vs „źle”: z lewej mnożenie przez \(\exp\) utrzymuje ortonormalność, z prawej dodawanie niszczy \(\mathbf{R}^\top\mathbf{R}\).  
   - Notatki: 2 min; to klucz do kalibracji/SLAM.

14. **Slajd nr 14 | \(SE(3)\) łączy położenie i orientację w jednej macierzy**  
   - Treść:  
     - \(\mathbf{T}=\begin{bmatrix}\mathbf{R}&\mathbf{p}\\0&1\end{bmatrix}\)  
     - Składanie ruchów = mnożenie \(\mathbf{T}\)  
   - Sugestie wizualne: animacja „łańcucha” transformacji między układami (świat → baza → ogniwo → chwytak); obok jedna macierz \(\mathbf{T}\) jako „opakowanie”.  
   - Notatki: 3 min; przejście do Wykładu 03.

15. **Slajd nr 15 | Skręt (twist) to prędkość w \(\mathbb{R}^6\) dla \(SE(3)\)**  
   - Treść:  
     - \(\boldsymbol{\xi}=[\boldsymbol{\omega};\mathbf{v}]\)  
     - \(\dot{\mathbf{T}}=\mathbf{T}[\boldsymbol{\xi}]_\wedge\)  
   - Sugestie wizualne: humanoid/manipulator z wektorem \(\boldsymbol{\omega}\) i \(\mathbf{v}\) przy końcówce; overlay „6 składowych prędkości”.  
   - Notatki: 2 min; zaznaczyć, że to fundament Jacobianu w Wykładzie 06.

16. **Slajd nr 16 | Normalizacja to nie „hack”, tylko konieczność numeryczna**  
   - Treść:  
     - \(q\leftarrow q/\|q\|\)  
     - \(\mathbf{R}\leftarrow \mathrm{proj}_{SO(3)}(\mathbf{R})\)  
   - Sugestie wizualne: wykres: „dryf” \(\|q\|\) w czasie bez normalizacji vs z normalizacją; obok „heatmapa” \(\|\mathbf{R}^\top\mathbf{R}-\mathbf{I}\|\).  
   - Notatki: 3 min; praktyka implementacyjna w filtrach i symulacji.

17. **Slajd nr 17 | Testy niezmienników szybko wykrywają błędy w geometrii**  
   - Treść:  
     - \(\det(\mathbf{R})=1\)  
     - \(\mathbf{R}^\top\mathbf{R}=\mathbf{I}\)  
     - \(q\sim -q\) (ciągłość w czasie)  
   - Sugestie wizualne: „checklist” testów jednostkowych + przykład wykresu z CI, który łapie błąd konwencji osi.  
   - Notatki: 2 min; zachęcić do automatycznych testów w projektach.

18. **Slajd nr 18 | Błędy konwencji (układ świata vs ciała) są najczęstszą przyczyną awarii**  
   - Treść:  
     - Gdzie jest wyrażone \(\boldsymbol{\omega}\)?  
     - Jaka jest kolejność mnożenia?  
     - Jaka konwencja osi?  
   - Sugestie wizualne: trzy mini-przypadki „pomyłka znaku” z rysunkiem; czerwone ostrzeżenia „zamiana kolejności = inny obrót”.  
   - Notatki: 3 min; podać anegdotę: „działało, dopóki nie obróciliśmy robota o 180°”.

19. **Slajd nr 19 | Geometria staje się składnikiem uczenia: równoważność \(SE(3)\)**  
   - Treść:  
     - Nie uczymy się „od zera”: narzucamy symetrie  
     - Lepsza generalizacja + mniej danych  
   - Sugestie wizualne: przykład chmury punktów obiektu obracanej w 3D; sieć, która daje to samo rozpoznanie/akcję po obrocie (overlay strzałek).  
   - Notatki: 3 min; połączyć z trendami Physical AI.

20. **Slajd nr 20 | Do 2030 formalizm grup Liego będzie językiem walidacji i audytu**  
   - Treść:  
     - Metryki na \(SO(3)\) zamiast „na składowych”  
     - Ograniczenia geometryczne w algorytmach  
   - Sugestie wizualne: schemat „model uczony” + „warstwa geometryczna” + „warstwa bezpieczeństwa”; obok przykład raportu z metryką \(d(\mathbf{R}_1,\mathbf{R}_2)\).  
   - Notatki: 2 min; odwołać się do wątku NIST AI RMF i walidacji.

21. **Slajd nr 21 | Mini-studium: planowanie orientacji chwytaka bez osobliwości**  
   - Treść:  
     - Orientacje kluczowe: \(q_0, q_1, q_2\)  
     - Interpolacja: SLERP + ograniczenie prędkości kątowej  
   - Sugestie wizualne: animacja chwytaka obracającego się do 3 pozycji; wykres prędkości kątowej w czasie (płynny przebieg).  
   - Notatki: 3 min; podkreślić, że „ładny ruch” = poprawna geometria + ograniczenia.

22. **Slajd nr 22 | Podsumowanie: wybór reprezentacji to decyzja inżynierska**  
   - Treść:  
     - Sterowanie/estymacja: \(\mathbf{R}\) lub \(q\) + \(\log/\exp\)  
     - Wizualizacja: kąty Eulera (ostrożnie)  
     - Testy: niezmienniki zawsze  
   - Sugestie wizualne: tabela „zastosowanie → reprezentacja” (sterowanie, estymacja, interpolacja, UI) z zaznaczeniem rekomendacji.  
   - Notatki: 2 min; przejście: „Wykład 03: \(SE(3)\) i transformacje jednorodne”.

### 5. Przykłady i studia przypadku (minimum 5 szczegółowych)
#### Przykład 1: Obrót oś–kąt \(\rightarrow\) macierz \(\mathbf{R}\) (Rodrigues) i test przynależności do \(SO(3)\)
**Opis problemu.** Dany jest obrót o kąt \(\theta=30^\circ\) wokół osi \(\mathbf{u}=\frac{1}{\sqrt{3}}[1,1,1]^\top\).  
(a) Wyznacz macierz obrotu \(\mathbf{R}\) wzorem Rodriguesa.  
(b) Sprawdź, czy \(\mathbf{R}\in SO(3)\) (ortonormalność i \(\det=1\)).  
(c) Oblicz obraz wektora \(\mathbf{v}=[1,0,0]^\top\) po obrocie.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Oś:
\[
\mathbf{u}=\frac{1}{\sqrt{3}}
\begin{bmatrix}1\\1\\1\end{bmatrix},\quad
\theta=30^\circ=\frac{\pi}{6}.
\]
2) Macierz \([\mathbf{u}]_\times\):
\[
[\mathbf{u}]_\times=\frac{1}{\sqrt{3}}
\begin{bmatrix}
0 & -1 & 1\\
1 & 0 & -1\\
-1 & 1 & 0
\end{bmatrix}.
\]
3) Rodrigues:
\[
\mathbf{R}=\mathbf{I}+\sin\theta[\mathbf{u}]_\times+(1-\cos\theta)[\mathbf{u}]_\times^2.
\]
Wartości: \(\sin(\pi/6)=1/2\), \(\cos(\pi/6)=\sqrt{3}/2\). Wstawienie do wzoru daje \(\mathbf{R}\) (w praktyce liczone numerycznie).
4) Test \(SO(3)\): sprawdzamy \(\|\mathbf{R}^\top\mathbf{R}-\mathbf{I}\|\) oraz \(\det(\mathbf{R})\).
5) Obrót wektora:
\[
\mathbf{v}'=\mathbf{R}\mathbf{v}.
\]

**Kod (Python 3.11, NumPy).**
```python
import numpy as np

def hat(w: np.ndarray) -> np.ndarray:
    wx, wy, wz = w
    return np.array([[0, -wz, wy],
                     [wz, 0, -wx],
                     [-wy, wx, 0]], dtype=float)

def so3_exp(phi: np.ndarray) -> np.ndarray:
    theta = np.linalg.norm(phi)
    if theta < 1e-12:
        return np.eye(3) + hat(phi)
    u = phi / theta
    U = hat(u)
    return np.eye(3) + np.sin(theta) * U + (1 - np.cos(theta)) * (U @ U)

u = np.array([1.0, 1.0, 1.0])
u = u / np.linalg.norm(u)
theta = np.deg2rad(30.0)
phi = theta * u

R = so3_exp(phi)
orth_err = np.linalg.norm(R.T @ R - np.eye(3), ord="fro")
detR = np.linalg.det(R)

v = np.array([1.0, 0.0, 0.0])
v_rot = R @ v

print("R=\n", R)
print("||R^T R - I||_F =", orth_err)
print("det(R) =", detR)
print("v_rot =", v_rot)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Wykres 3D: oś \(\mathbf{u}\) jako strzałka, wektor \(\mathbf{v}\) i \(\mathbf{v}'\) jako dwie strzałki, oraz łuk zaznaczający kąt \(30^\circ\); obok dwa wskaźniki: \(\det(\mathbf{R})\) i błąd ortonormalności.

---

#### Przykład 2: Kąty Eulera \(Z\!Y\!X\) \(\rightarrow\) kwaternion i demonstracja osobliwości
**Opis problemu.** Dane są kąty yaw–pitch–roll w konwencji \(Z\!Y\!X\): \(\psi=40^\circ\), \(\theta=89^\circ\), \(\phi=10^\circ\).  
(a) Wyznacz macierz obrotu \(\mathbf{R}\).  
(b) Wyznacz kwaternion \(q\) odpowiadający \(\mathbf{R}\).  
(c) Powtórz obliczenia dla \(\theta=90^\circ\) i pokaż, dlaczego kąty Eulera stają się problematyczne.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Składowe:
\[
\mathbf{R}=\mathbf{R}_z(\psi)\mathbf{R}_y(\theta)\mathbf{R}_x(\phi).
\]
2) Dla \(\theta\to 90^\circ\) dwie osie obrotu stają się współliniowe, przez co małe zaburzenie orientacji może wywoływać duże zmiany \(\psi\) i \(\phi\) — to właśnie osobliwość.

**Kod (Python 3.11).**
```python
import numpy as np

def Rx(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[1, 0, 0],
                     [0, c, -s],
                     [0, s, c]], float)

def Ry(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, 0, s],
                     [0, 1, 0],
                     [-s, 0, c]], float)

def Rz(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, -s, 0],
                     [s, c, 0],
                     [0, 0, 1]], float)

def rot_to_quat(R):
    # stabilny wariant oparty o ślad
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
    if q[0] < 0:  # wymuszamy qw>=0 dla ciągłości
        q = -q
    return q

def compute(psi_deg, theta_deg, phi_deg):
    psi, theta, phi = np.deg2rad([psi_deg, theta_deg, phi_deg])
    R = Rz(psi) @ Ry(theta) @ Rx(phi)
    q = rot_to_quat(R)
    return R, q

for theta_deg in (89.0, 90.0):
    R, q = compute(40.0, theta_deg, 10.0)
    print("theta=", theta_deg)
    print("trace(R)=", np.trace(R))
    print("q=", q)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Dwa kadry: \(\theta=89^\circ\) i \(\theta=90^\circ\); na każdym animacja kardana (3 osie) z zaznaczeniem współliniowości osi przy \(90^\circ\); obok wykres „wrażliwość”: małe \(\Delta\mathbf{R}\) powoduje duże \(\Delta(\psi,\phi)\) w pobliżu osobliwości.

---

#### Przykład 3: Błąd orientacji jako obrót względny i wektor w \(\mathbb{R}^3\)
**Opis problemu.** Dana jest orientacja zadana \(\mathbf{R}^\*\) oraz aktualna \(\mathbf{R}\). Przyjmij:
\[
\mathbf{R}^\* = \mathbf{R}_z(20^\circ),\qquad
\mathbf{R} = \mathbf{R}_z(20^\circ)\mathbf{R}_y(5^\circ).
\]
(a) Wyznacz \(\mathbf{R}_{err}=\mathbf{R}^{*\top}\mathbf{R}\).  
(b) Wyznacz kąt błędu \(d=\arccos((\mathrm{tr}(\mathbf{R}_{err})-1)/2)\).  
(c) Wyznacz wektor błędu \(\boldsymbol{\phi}\) w przybliżeniu małego kąta.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Ponieważ \(\mathbf{R}=\mathbf{R}^\*\mathbf{R}_y(5^\circ)\), to:
\[
\mathbf{R}_{err}=\mathbf{R}^{*\top}\mathbf{R}=\mathbf{R}_y(5^\circ).
\]
2) Dla czystego obrotu wokół osi \(y\) o \(\theta=5^\circ\) mamy:
\[
d = 5^\circ \approx 0.0873\ \text{rad}.
\]
3) Dla małego kąta:
\[
\boldsymbol{\phi}\approx \theta\,\mathbf{e}_y = [0,\ 0.0873,\ 0]^\top.
\]

**Kod (Python 3.11).**
```python
import numpy as np

def Ry(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, 0, s],
                     [0, 1, 0],
                     [-s, 0, c]], float)

def Rz(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, -s, 0],
                     [s, c, 0],
                     [0, 0, 1]], float)

R_star = Rz(np.deg2rad(20.0))
R = R_star @ Ry(np.deg2rad(5.0))
R_err = R_star.T @ R

tr = np.trace(R_err)
d = np.arccos(np.clip((tr - 1) / 2.0, -1.0, 1.0))

phi_small = np.array([0.0, d, 0.0])  # bo tu wiemy, że to czyste Ry

print("R_err=\n", R_err)
print("d [rad]=", d, " d [deg]=", np.rad2deg(d))
print("phi_small=", phi_small)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Dwa układy osi: zadany i aktualny; trzeci układ pokazuje obrót względny \(\mathbf{R}_{err}\) jako strzałka wokół osi \(y\); obok liczba \(d\) i wektor \(\boldsymbol{\phi}\) jako trójka składowych.

---

#### Przykład 4: Integracja prędkości kątowej — Euler vs \(\exp\) na \(SO(3)\) oraz normalizacja kwaternionu
**Opis problemu.** Robot (lub kamera) obraca się ze stałą prędkością kątową \(\boldsymbol{\omega}=[0,0,1]^\top\ \text{rad/s}\) przez \(T=2\ \text{s}\).  
(a) Oblicz orientację końcową jako obrót o \(\theta=\|\boldsymbol{\omega}\|T\).  
(b) Zasymuluj integrację \(\dot{\mathbf{R}}=\mathbf{R}[\boldsymbol{\omega}]_\times\) metodą Eulera dla kroku \(\Delta t=0.01\) i pokaż dryf ortonormalności.  
(c) Zasymuluj integrację przez krokowanie \(\mathbf{R}\leftarrow \mathbf{R}\exp([\boldsymbol{\omega}\Delta t]_\times)\) i porównaj błędy.

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Kąt całkowity:
\[
\theta=\|\boldsymbol{\omega}\|T = 1\cdot 2 = 2\ \text{rad}.
\]
Orientacja końcowa to \(\mathbf{R}_z(2)\).
2) Metoda Eulera:
\[
\mathbf{R}_{k+1}=\mathbf{R}_k + \Delta t\,\mathbf{R}_k[\boldsymbol{\omega}]_\times,
\]
nie zachowuje dokładnie ortonormalności.
3) Krokowanie przez \(\exp\):
\[
\mathbf{R}_{k+1}=\mathbf{R}_k\exp([\boldsymbol{\omega}\Delta t]_\times),
\]
zachowuje \(\mathbf{R}\in SO(3)\) (w idealnej arytmetyce) i jest znacznie stabilniejsze numerycznie.

**Kod (Python 3.11, NumPy).**
```python
import numpy as np

def hat(w):
    wx, wy, wz = w
    return np.array([[0, -wz, wy],
                     [wz, 0, -wx],
                     [-wy, wx, 0]], float)

def so3_exp(phi):
    theta = np.linalg.norm(phi)
    if theta < 1e-12:
        return np.eye(3) + hat(phi)
    u = phi / theta
    U = hat(u)
    return np.eye(3) + np.sin(theta) * U + (1 - np.cos(theta)) * (U @ U)

omega = np.array([0.0, 0.0, 1.0])
dt = 0.01
T = 2.0
N = int(T / dt)

R_euler = np.eye(3)
R_exp = np.eye(3)
W = hat(omega)

orth_err_euler = []
orth_err_exp = []

for _ in range(N):
    R_euler = R_euler + dt * (R_euler @ W)
    R_exp = R_exp @ so3_exp(omega * dt)
    orth_err_euler.append(np.linalg.norm(R_euler.T @ R_euler - np.eye(3), ord="fro"))
    orth_err_exp.append(np.linalg.norm(R_exp.T @ R_exp - np.eye(3), ord="fro"))

R_true = so3_exp(omega * T)

def rot_dist(R1, R2):
    tr = np.trace(R1.T @ R2)
    return np.arccos(np.clip((tr - 1) / 2.0, -1.0, 1.0))

print("d_euler [rad] =", rot_dist(R_true, R_euler))
print("d_exp   [rad] =", rot_dist(R_true, R_exp))
print("orth_euler_end =", orth_err_euler[-1])
print("orth_exp_end   =", orth_err_exp[-1])
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Dwa wykresy w czasie:  
(1) \(\|\mathbf{R}^\top\mathbf{R}-\mathbf{I}\|_F\) dla Eulera i dla \(\exp\) (krzywa Eulera rośnie).  
(2) dystans kątowy do rozwiązania prawdziwego \(d(\mathbf{R}_{true},\mathbf{R}_k)\).  
Opcjonalnie animacja: oś \(z\) i obrót układu w czasie.

---

#### Przykład 5: \(\exp\) na \(SE(3)\) — ruch śrubowy z danym skrętem \(\boldsymbol{\xi}=[\boldsymbol{\omega};\mathbf{v}]\)
**Opis problemu.** Dany jest skręt:
\[
\boldsymbol{\omega}=\begin{bmatrix}0\\0\\1\end{bmatrix}\ \text{rad/s},\qquad
\mathbf{v}=\begin{bmatrix}1\\0\\0\end{bmatrix}\ \text{m/s},
\qquad T=1\ \text{s}.
\]
Zinterpretuj ruch i oblicz transformację \(\mathbf{T}=\exp([\boldsymbol{\xi}T]_\wedge)\). Następnie przekształć punkt \(\mathbf{p}=[1,1,0]^\top\).

**Rozwiązanie krok po kroku (z obliczeniami).**
1) Kąt obrotu: \(\theta=\|\boldsymbol{\omega}\|T=1\ \text{rad}\).  
2) Część obrotowa: \(\mathbf{R}=\exp([\boldsymbol{\omega}T]_\times)=\mathbf{R}_z(1)\).  
3) Dla \(\boldsymbol{\omega}\neq \mathbf{0}\) przesunięcie liczymy przez macierz \(\mathbf{V}\):
\[
\mathbf{V} = \mathbf{I} + \frac{1-\cos\theta}{\theta^2}[\boldsymbol{\omega}]_\times + \frac{\theta-\sin\theta}{\theta^3}[\boldsymbol{\omega}]_\times^2,
\qquad
\mathbf{p} = \mathbf{V}(\mathbf{v}T).
\]
4) Składamy:
\[
\mathbf{T}=
\begin{bmatrix}
\mathbf{R} & \mathbf{p}\\
0 & 1
\end{bmatrix}.
\]
5) Transformacja punktu: \(\mathbf{p}'=\mathbf{R}\mathbf{p}_{in}+\mathbf{p}\).

**Kod (Python 3.11, NumPy).**
```python
import numpy as np

def hat(w):
    wx, wy, wz = w
    return np.array([[0, -wz, wy],
                     [wz, 0, -wx],
                     [-wy, wx, 0]], float)

def so3_exp(phi):
    theta = np.linalg.norm(phi)
    if theta < 1e-12:
        return np.eye(3) + hat(phi)
    u = phi / theta
    U = hat(u)
    return np.eye(3) + np.sin(theta) * U + (1 - np.cos(theta)) * (U @ U)

def se3_exp(omega, v, T):
    theta = np.linalg.norm(omega) * T
    if theta < 1e-12:
        R = np.eye(3)
        p = v * T
        return R, p
    w = omega / np.linalg.norm(omega)
    W = hat(w)
    R = so3_exp(w * theta)
    V = (np.eye(3)
         + (1 - np.cos(theta)) / (theta**2) * W
         + (theta - np.sin(theta)) / (theta**3) * (W @ W))
    p = V @ (v * T)
    return R, p

omega = np.array([0.0, 0.0, 1.0])
v = np.array([1.0, 0.0, 0.0])
T = 1.0

R, p = se3_exp(omega, v, T)
pin = np.array([1.0, 1.0, 0.0])
pout = R @ pin + p

print("R=\n", R)
print("p=", p)
print("pout=", pout)
```

**Wizualizacja wyniku (opis diagramu/wykresu).** Widok z góry: tor punktu \(\mathbf{p}\) jako łuk wynikający z połączenia obrotu wokół osi \(z\) i przesunięcia; overlay pokazujący początkową i końcową ramkę oraz wektor przesunięcia \(\mathbf{p}\).  

### 6. Materiały dla studentów
#### 6 pytań teoretycznych (z oczekiwanymi odpowiedziami)
1) **Pytanie:** Podaj warunki, które musi spełniać \(\mathbf{R}\), aby należeć do \(SO(3)\).  
   **Odpowiedź:** \(\mathbf{R}^\top\mathbf{R}=\mathbf{I}\) oraz \(\det(\mathbf{R})=+1\).
2) **Pytanie:** Dlaczego nie wolno „uśredniać” orientacji przez zwykłe uśrednianie macierzy obrotu?  
   **Odpowiedź:** Ponieważ zbiór \(SO(3)\) nie jest domknięty na takie działanie; średnia arytmetyczna może nie być ortogonalna i nie będzie poprawnym obrotem.
3) **Pytanie:** Wyjaśnij, czym jest osobliwość kątów Eulera (gimbal lock) i jaki ma skutek w sterowaniu.  
   **Odpowiedź:** Dla pewnych konfiguracji osie obrotu stają się współliniowe, tracimy stopień swobody, a małe zmiany orientacji powodują duże skoki kątów; regulator może stać się niestabilny numerycznie.
4) **Pytanie:** Zinterpretuj równanie \(\dot{\mathbf{R}}=\mathbf{R}[\boldsymbol{\omega}]_\times\).  
   **Odpowiedź:** Pochodna macierzy obrotu jest wyznaczona przez prędkość kątową; struktura \([\cdot]_\times\) zapewnia zgodność z geometrią \(SO(3)\).
5) **Pytanie:** Co oznacza podwójne pokrycie w kwaternionach (\(q\sim -q\)) i dlaczego to ważne w implementacji?  
   **Odpowiedź:** Dwa przeciwne kwaterniony opisują ten sam obrót; bez uwzględnienia tego można uzyskać skoki znaku i nieciągłość trajektorii.
6) **Pytanie:** Po co stosuje się mapy \(\exp/\log\) na \(SO(3)\) w filtracji i optymalizacji?  
   **Odpowiedź:** Aby pracować z małymi przyrostami w przestrzeni liniowej \(\mathbb{R}^3\) (algebra Liego), a jednocześnie utrzymywać stan na rozmaitości \(SO(3)\).

#### 4 zadania obliczeniowe/programistyczne (z poziomem trudności)
1) **(Łatwe)** Dla losowego wektora \(\mathbf{a}\) i \(\mathbf{b}\) pokaż numerycznie, że \([\mathbf{a}]_\times\mathbf{b}=\mathbf{a}\times\mathbf{b}\).  
2) **(Średnie)** Zaimplementuj `so3_log(R)` zwracające \(\boldsymbol{\phi}\) i przetestuj na \(\theta\in\{1^\circ,30^\circ,179^\circ\}\). Zwróć uwagę na stabilność przy \(\theta\to 0\) i \(\theta\to \pi\).  
3) **(Średnie)** Porównaj interpolację orientacji: (a) liniową na kątach Eulera, (b) SLERP na kwaternionach. Zmierz maksymalną prędkość kątową w obu przypadkach.  
4) **(Trudne)** Zaimplementuj `se3_exp(xi)` i użyj jej do generacji trajektorii ruchu kamery; zwizualizuj tor punktu w świecie i oceń błąd ortonormalności \(\mathbf{R}\) bez i z rzutowaniem na \(SO(3)\).

#### 1 projekt laboratoryjny / projekt domowy (z kryteriami oceny)
**Projekt:** „Mini-biblioteka geometrii \(SO(3)\)/\(SE(3)\) z testami i zastosowaniem”.  
**Wymagania minimalne:**  
- Implementacje: `hat/vee`, `so3_exp`, `so3_log`, `quat_normalize`, `quat_mul`, `slerp`, `se3_exp` (wystarczy \(\boldsymbol{\omega}\neq 0\) + przypadek \(\boldsymbol{\omega}\to 0\)).  
- Zestaw testów niezmienników: \(\det(\mathbf{R})\), \(\mathbf{R}^\top\mathbf{R}\), \(\|q\|\), symetria \(q\sim -q\).  
- Krótka demonstracja: planowanie orientacji chwytaka/kamery przez 3 orientacje kluczowe z ograniczeniem prędkości kątowej (wykres).  
**Kryteria oceny (100 pkt):** poprawność matematyczna (35), stabilność numeryczna i przypadki brzegowe (25), testy i czytelność kodu (25), jakość demonstracji/wizualizacji (15).

### 7. Quiz sprawdzający (15 pytań: 10 wyboru wielokrotnego + 5 otwartych + klucz odpowiedzi)
#### 10 pytań wyboru wielokrotnego
1) Warunek \(\mathbf{R}^\top\mathbf{R}=\mathbf{I}\) oznacza, że \(\mathbf{R}\) jest:  
   A) symetryczna, B) ortogonalna, C) diagonalna, D) dodatnio określona  
2) Do \(SO(3)\) należą macierze ortogonalne o wyznaczniku:  
   A) \(-1\), B) \(0\), C) \(+1\), D) dowolnym  
3) Najczęstszy problem kątów Eulera w sterowaniu 3D to:  
   A) redundancja parametrów, B) osobliwości, C) brak interpretacji, D) brak mnożenia  
4) Operator \([\mathbf{a}]_\times\) jest macierzą:  
   A) symetryczną, B) antysymetryczną, C) diagonalną, D) trójkątną  
5) Związek \([\mathbf{a}]_\times\mathbf{b}\) równa się:  
   A) \(\mathbf{a}^\top\mathbf{b}\), B) \(\mathbf{a}\times\mathbf{b}\), C) \(\mathbf{b}\times\mathbf{a}\), D) \(\mathbf{a}+\mathbf{b}\)  
6) W kwaternionach \(q\) i \(-q\):  
   A) opisują przeciwne obroty, B) opisują ten sam obrót, C) nie mają znaczenia fizycznego, D) są zawsze różne w \(SO(3)\)  
7) SLERP jest używany głównie do:  
   A) uśredniania macierzy, B) interpolacji orientacji ze stałą prędkością kątową, C) filtracji Kalmana, D) liczenia wyznacznika  
8) Dystans kątowy między \(\mathbf{R}_1\) i \(\mathbf{R}_2\) można policzyć z:  
   A) \(\mathrm{tr}(\mathbf{R}_1^\top\mathbf{R}_2)\), B) \(\det(\mathbf{R}_1+\mathbf{R}_2)\), C) \(\|\mathbf{R}_1-\mathbf{R}_2\|_1\), D) \(\mathrm{rank}(\mathbf{R}_1)\)  
9) Wzór \(\dot{\mathbf{R}}=\mathbf{R}[\boldsymbol{\omega}]_\times\) sugeruje, że \(\boldsymbol{\omega}\) jest:  
   A) przyspieszeniem, B) prędkością kątową, C) momentem, D) położeniem  
10) „Normalizacja” kwaternionu w implementacji służy do:  
   A) zmiany orientacji, B) utrzymania więzu \(\|q\|=1\), C) zwiększenia liczby stopni swobody, D) uniknięcia wyznacznika

#### 5 pytań otwartych
11) Podaj przykład sytuacji, w której interpolacja kątów Eulera daje niepożądany ruch, a SLERP go eliminuje.  
12) Wyjaśnij różnicę między błędem orientacji liczonym jako \(\mathbf{R}-\mathbf{R}^\*\) i jako \(\mathbf{R}^{*\top}\mathbf{R}\).  
13) Opisz, jak sprawdzisz automatycznie (testami), czy Twoja funkcja `rot_to_quat` działa poprawnie dla przypadków bliskich \(\theta\approx\pi\).  
14) Dlaczego w estymacji/optimizacji stosuje się perturbacje \(\mathbf{R}\leftarrow \mathbf{R}\exp([\delta\boldsymbol{\phi}]_\times)\)?  
15) Podaj dwa powody, dla których do 2030 geometria \(SO(3)\)/\(SE(3)\) będzie jeszcze ważniejsza w robotyce opartej o uczenie.

#### Klucz odpowiedzi
1) B, 2) C, 3) B, 4) B, 5) B, 6) B, 7) B, 8) A, 9) B, 10) B.  
11–15) Oceniane opisowo: poprawność merytoryczna + argumentacja i przykłady.

### 8. Bibliografia i materiały dodatkowe
1) B. Siciliano, O. Khatib (red.), *Springer Handbook of Robotics*, 2nd ed., Springer, 2016.  
2) K. M. Lynch, F. C. Park, *Modern Robotics: Mechanics, Planning, and Control*, Cambridge University Press, 2017 (+ materiały online).  
3) J. J. Craig, *Introduction to Robotics: Mechanics and Control*, 3rd ed., Pearson, 2004.  
4) T. D. Barfoot, *State Estimation for Robotics*, Cambridge University Press, 2017.  
5) P. Corke, *Robotics, Vision and Control*, 2nd ed., Springer, 2017.  
6) R. M. Murray, Z. Li, S. S. Sastry, *A Mathematical Introduction to Robotic Manipulation*, CRC Press, 1994 (formalizm grup Liego w robotyce).  
7) J. Solà, *Quaternion kinematics for the error-state Kalman filter*, 2018 (notatki/opracowanie, szeroko cytowane w robotyce).  
8) NIST, *AI Risk Management Framework (AI RMF 1.0)*, 2023.  
9) V. Driess i in., *PaLM-E: An Embodied Multimodal Language Model*, 2023.  
10) A. Brohan i in., *RT-2: Vision-Language-Action Models Transfer Web Knowledge to Robotic Control*, 2023.  
11) Y. Chi i in., *Diffusion Policy: Visuomotor Policy Learning via Action Diffusion*, 2023.  
12) International Federation of Robotics (IFR), *World Robotics* — raporty branżowe (wydania 2023–2025) jako tło trendów do 2030.  
13) Stanford University, *AI Index Report* — wydania 2024–2025 (kontekst trendów „AI w gospodarce” istotnych dla robotyki).  
14) Dokumentacja ROS2: konwencje ramek odniesienia i orientacji (REP/TF2) — jako praktyczne uzupełnienie implementacyjne.  
15) Materiały wideo/online: kursy i seminaria o \(SO(3)\)/\(SE(3)\) (np. wykłady K. Lynch — *Modern Robotics*; wykłady z estymacji na grupach Liego oparte o Barfoot).
