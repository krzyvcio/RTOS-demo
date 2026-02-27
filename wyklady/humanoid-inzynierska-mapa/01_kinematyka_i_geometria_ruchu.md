# Wykład 1: Kinematyka, geometria ruchu, manipulatory

## Po co ta warstwa
Kinematyka odpowiada na pytanie: "gdzie jest dana część robota" i "jak zmieni się jej pozycja/orientacja, gdy poruszę przegubami".
W humanoidzie jest to fundament dla:
- śledzenia trajektorii rąk, stóp, głowy,
- kontroli postawy i stabilizacji,
- unikania kolizji,
- estymacji (np. pozycja IMU względem świata),
- planowania kontaktów (stopa na podłożu, dłoń na obiekcie).

## Notacja i obiekty geometryczne
W praktyce wszystko sprowadza się do spójnej definicji układów współrzędnych (ramek) i transformacji między nimi.

Transformacja sztywna w 3D to element grupy Liego SE(3):
```text
T = [ R  p ]
    [ 0  1 ] ,  R ∈ SO(3), p ∈ R^3
```
Składanie transformacji (łańcuch kinematyczny) to zwykłe mnożenie macierzy:
```text
T_A^C = T_A^B * T_B^C
```

Konwencje, które warto ustalić na starcie projektu:
- które osie są "do przodu", "w lewo", "w górę" w ramce świata,
- czy opisujesz transformacje jako `T_A^B` (z B do A) czy `T_B^A`,
- jednostki (metry, radiany) i kierunki dodatnie przegubów.

## Grafy i struktury kinematyczne
W humanoidzie te same dane można widzieć jako kilka powiązanych grafów, zależnie od zadania.

### Drzewo kinematyczne (kinematic tree)
Najczęstsza reprezentacja: każdy link ma dokładnie jednego rodzica, a przeguby tworzą drzewo od bazy (miednica/tułów) do końcówek (stopy, dłonie).
Taka struktura umożliwia szybkie:
- forward kinematics (FK): oblicz `T_world^link(q)` dla wielu linków,
- propagowanie prędkości i przyspieszeń wzdłuż gałęzi.

### Graf przegubów (joint graph)
To bardziej ogólna forma (graf połączeń). Przydaje się, gdy:
- masz mechaniczne sprzężenia (np. przekładnie różnicowe),
- pojawiają się zamknięte łańcuchy (zamknięty chwyt, kontakt obu stóp i dodatkowe podpory).
Wtedy drzewo jest tylko przybliżeniem, a pełny graf wymaga równań więzów.

### Graf łańcucha kinematycznego
To "wycięty" fragment drzewa/grafu: baza -> wybrana końcówka (end-effector).
Wiele algorytmów IK pracuje właśnie na takim łańcuchu, nawet jeśli cały robot jest większy.

### Graf kontaktów
W humanoidzie kontakty są dynamiczne w czasie.
Wersja praktyczna to zbiór aktywnych kontaktów i ich modeli:
- punkt/obszar kontaktu,
- normalna kontaktu,
- tarcie (stożek tarcia, ograniczenia poślizgu),
- informacja, czy kontakt jest "twardy" (constraint) czy "miękki" (sprężyna-tłumik).

### Graf brył sztywnych (rigid body graph)
To ujęcie, gdzie wierzchołki to bryły, krawędzie to przeguby/wiązania.
W kinematyce jest to tylko opis topologii, ale później (dynamika) ten graf jest nośnikiem zależności sił i momentów.

## Reprezentacja orientacji i pułapki
Konwencjonalnie spotkasz:
- macierze obrotu `R ∈ SO(3)` (bez osobliwości, ale 9 liczb),
- kwaterniony jednostkowe (4 liczby, wymagają normalizacji i uwagi na znak `q` i `-q`),
- kąty Eulera/RPY (intuicyjne, ale mają osobliwości).

Zasada praktyczna:
- do obliczeń i propagacji używaj `SO(3)`/kwaternionów,
- do interfejsów i wizualizacji możesz użyć RPY, ale nie mieszaj ich w pętli sterowania bez świadomości osobliwości.

## SO(3), SE(3), algebry Liego: po co w robotyce
Grupy Liego są naturalnym językiem dla ruchu brył sztywnych.
Wprowadza to:
- spójny "błąd" w przestrzeni konfiguracji (np. różnica pozy w SE(3)),
- stabilne mapy log/exp do małych przyrostów.

W IK często definiujesz błąd pozy (task error) jako:
```text
E = log( T_des^{-1} * T(q) )  ∈ se(3)
```
gdzie `log` mapuje transformację z SE(3) do wektora skrętu (twist) w `se(3)`.

## FK i Jacobian: łącznik między przegubami a ruchem końcówki
Forward kinematics:
- dostaje `q` i zwraca `T(q)` dla wybranych linków (np. stopy).

Jacobian geometryczny `J(q)` łączy prędkości przegubów z prędkością skrętu końcówki:
```text
V = J(q) * qd
```
W praktyce potrzebujesz:
- Jacobian dla końcówki (dłoń, stopa),
- Jacobian dla środka masy (CoM),
- Jacobian dla orientacji tułowia.

## Odwrotna kinematyka (IK): sensowna definicja problemu
Najprostszy model:
```text
znajdź q, aby T(q) ≈ T_des
```
W praktyce IK to optymalizacja z ograniczeniami:
- błędy zadania (pozycja/orientacja),
- limity przegubów `q_min <= q <= q_max`,
- limity prędkości i przyspieszeń,
- unikanie kolizji i ograniczenia kontaktu.

Typowy wariant iteracyjny:
```text
q_{k+1} = q_k + Δq
```
gdzie `Δq` wynika z przybliżenia liniowego błędu w otoczeniu `q_k`.

## Least Squares, pseudoodwrotność, DLS (Levenberg–Marquardt)
Jeśli masz błąd zadania `e` i zlinearyzowanie `e ≈ J Δq`, to LS daje:
```text
Δq = J^+ e
```
Pseudoodwrotność przez SVD jest stabilna numerycznie, ale w pobliżu osobliwości problem robi się źle uwarunkowany.

Damped Least Squares (DLS) dodaje tłumienie:
```text
Δq = J^T (J J^T + λ^2 I)^{-1} e
```
Intuicja:
- gdy `J J^T` ma małe wartości własne (osobliwość), składnik `λ^2 I` stabilizuje odwracanie.

Levenberg–Marquardt można widzieć jako DLS z adaptacyjnym `λ`.

## Newton-Raphson i rozwiązywanie nieliniowe
Gdy model błędu jest mocno nieliniowy (np. duże rotacje, zamknięte łańcuchy), używa się metod Newtona lub quasi-Newtona.
Praktyczna uwaga:
- Newton jest szybki, ale wrażliwy na zły start i niedokładne pochodne,
- w robotyce często wygrywa "bezpieczna" metoda iteracyjna z tłumieniem i ograniczeniami.

## Ograniczenia jako QP/NLP: IK jako problem optymalizacji
Wersja często spotykana w humanoidach (szczególnie w whole-body control) to QP:
```text
min_{Δq}  ||J Δq - e||_W^2 + ||Δq||_R^2
takie że  A Δq <= b
```
gdzie nierówności kodują limity, kolizje i zasady kontaktu.

Jeśli ograniczenia są nieliniowe (np. dokładne modele kolizji), wchodzi NLP/SQP.

## Osobliwości i analiza SVD
Osobliwość to sytuacja, w której tracisz stopień swobody w przestrzeni zadania.
W praktyce wykrywasz ją przez:
- wartości osobliwe `σ_i` Jacobianu,
- uwarunkowanie `κ = σ_max / σ_min`,
- miary manipulowalności, np. `sqrt(det(J J^T))` dla pełnego rzędu.

Co robisz w systemie:
- zwiększasz damping `λ`,
- zmieniasz priorytety zadań,
- przełączasz parametrystykę zadania (np. śledzenie tylko pozycji bez orientacji).

## Mini-przykład: planar 2-DOF jako intuicja
Dla ramienia 2-przegubowego w 2D:
- FK to proste funkcje trygonometryczne,
- Jacobian pokazuje, że gdy ramię jest wyprostowane, sterowanie w pewnych kierunkach robi się trudne (osobliwość).
To jest ta sama historia w 3D, tylko w większym wymiarze.

## Praktyka inżynierska: co zwykle psuje projekty
- Niespójne ramki i konwencje znaków (najczęstszy problem).
- Brak ograniczeń: IK "działa", ale wysyła przeguby poza limity albo generuje skoki `Δq`.
- Brak tłumienia i filtracji: oscylacje w pobliżu osobliwości.
- Orientacja liczona w RPY w środku pętli: gimbal lock w najmniej oczekiwanym momencie.

## Checklisty
- testy jednostkowe FK dla kilku losowych `q` i porównanie z symulatorem,
- testy Jacobianu przez różniczkowanie numeryczne (sprawdzenie pochodnych),
- logowanie `σ_min(J)` i `||Δq||` jako metryk "zdrowia" IK,
- twarde ograniczenia na prędkość/akcelerację w warstwie wykonawczej.

## Pytania do studentów
1. Jak sprawdzisz poprawność FK i Jacobianu bez „zaufania” do jednej biblioteki?
2. Dlaczego DLS/Levenberg–Marquardt pomaga przy osobliwościach i jak dobrać damping `λ`?
3. Jakie ograniczenia (limity, kolizje) muszą być w IK, żeby wynik był użyteczny w realnym robocie?
4. Kiedy lepiej śledzić tylko pozycję końcówki, a kiedy pozycję i orientację?

## Projekty studenckie
- Implementacja FK + testy regresji (porównanie z symulatorem lub różniczkowaniem numerycznym).
- IK z DLS i limitami przegubów (QP lub iteracyjnie) + wykrywanie osobliwości przez SVD.
- Wizualizacja manipulowalności i „mapa osobliwości” dla wybranego łańcucha (np. ramię).

## BONUS
- Najszybszy test sanity dla Jacobianu: różniczkowanie numeryczne FK i porównanie z `J(q)` w kilku losowych punktach oraz blisko osobliwości.
