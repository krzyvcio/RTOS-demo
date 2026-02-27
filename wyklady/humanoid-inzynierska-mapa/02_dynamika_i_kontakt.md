# Wykład 2: Dynamika, siły, momenty, kontakt

## Po co dynamika w humanoidzie
Kinematyka mówi "gdzie", dynamika mówi "dlaczego i ile trzeba przyłożyć".
W praktyce dynamika jest potrzebna do:
- sterowania momentem (torque control),
- kompensacji grawitacji i bezwładności,
- projektowania stabilnego chodu z kontaktami,
- estymacji sił kontaktu i momentów w przegubach,
- diagnostyki (np. tarcie w przegubie, luzy, niezgodność modelu).

## Obiekt modelowania: układ brył połączonych przegubami
Humanoid to system wielu brył sztywnych połączonych przegubami.
Topologia zwykle jest drzewem, ale kontakt z otoczeniem tworzy dodatkowe więzy, które w danej chwili mogą "zamykać" układ.

Grafy, które pojawiają się w praktyce:
- graf brył i przegubów (nośnik parametrów masy i geometrii),
- drzewo dynamiki (articulated body tree) dla szybkich algorytmów rekursywnych,
- graf kontaktów (aktywne punkty/obszary styku),
- graf przepływu sił (intuicja: którędy płyną reakcje i momenty).

## Równania ruchu: forma standardowa
Dla układu z przegubami (uogólnione współrzędne `q`) najczęściej spotkasz:
```text
M(q) qdd + C(q, qd) qd + g(q) = τ + J_c(q)^T f_c
```
gdzie:
- `M(q)` to macierz mas (symetryczna, dodatnio określona),
- `C(q, qd) qd` to składniki Coriolisa i odśrodkowe,
- `g(q)` to grawitacja,
- `τ` to momenty/siły napędów,
- `J_c` to Jacobian kontaktu,
- `f_c` to siły reakcji (kontakty).

W przypadku więzów (twardy kontakt) dochodzi warunek prędkości/przyspieszeń w kontakcie:
```text
J_c(q) qd = 0
J_c(q) qdd + Jdot_c(q,qd) qd = 0
```
To prowadzi do układu równań z mnożnikami Lagrange’a (reakcje kontaktu).

## Lagrange vs. Newton–Euler: dwie perspektywy
Równania Lagrange’a:
- wygodne do wyprowadzeń teoretycznych,
- naturalne dla `q` i energii,
- prowadzą bezpośrednio do postaci z `M, C, g`.

Newton–Euler:
- bardziej "fizyczny" obraz: siły i momenty na każdej bryle,
- świetny do obliczeń rekursywnych w drzewach (algorytmy typu RNEA).

W implementacjach bibliotek robotyki najczęściej spotyka się Newton–Euler w środku, a na zewnątrz udostępnia się `M(q)`, `g(q)` i funkcje typu `inverseDynamics(q,qd,qdd)`.

## Algorytmy rekursywne: RNEA, ABA, CRBA
Te skróty to klasyka modelowania dynamiki wieloczłonowej:

RNEA (Recursive Newton–Euler Algorithm):
- liczy `τ` dla zadanego `(q, qd, qdd)` (inverse dynamics),
- typowo używany do kompensacji grawitacji i feed-forward w sterowaniu momentem.

CRBA (Composite Rigid Body Algorithm):
- liczy `M(q)` w czasie O(n) dla drzew (praktycznie: szybciej niż naiwne sumowanie),
- potrzebny w MPC/WBC, gdzie rozwiązujesz QP na bazie masy/inercji.

ABA (Articulated Body Algorithm):
- liczy `qdd` dla zadanego `τ` (forward dynamics) również w O(n),
- używany w symulacji i predykcji.

## Kontakty: modele twarde, miękkie i hybrydowość
Kontakt w humanoidzie nie jest "stały". Pojawiają się fazy:
- lotu (brak kontaktu),
- pojedynczego podparcia,
- podwójnego podparcia,
- dodatkowych podpór (ręka o ścianę, kolano o podłoże).

To jest system hybrydowy: struktura równań zmienia się wraz z aktywacją/dezaktywacją kontaktów.
W implementacji oznacza to:
- maszynę stanów kontaktu (kiedy które więzy obowiązują),
- filtrację i histerezę detekcji kontaktu,
- odporność regulatora na błędy klasyfikacji.

## Tarcie: stożek Coulomba i jego aproksymacje
Model tarcia Coulomba daje ograniczenie:
```text
||f_t|| <= μ f_n
```
gdzie `f_t` to składowa styczna, `f_n` normalna, `μ` współczynnik tarcia.

W optymalizacji (QP) często aproksymuje się stożek tarcia wielokątem (kilka półprzestrzeni), żeby dostać ograniczenia liniowe:
- szybciej i stabilniej numerycznie,
- kosztem przybliżenia fizyki.

## Kontakt sprężysto-tłumiący (Kelvin–Voigt)
Gdy kontakt jest modelowany miękko (symulacja, "soft constraints"), często używa się:
- sprężyny (penetracja -> siła),
- tłumika (prędkość penetracji -> siła).

To jest praktyczne w symulacji, ale w sterowaniu rzeczywistym robotem:
- rzeczywisty kontakt jest bliżej "twardego" (choć podłoże i stopa mają ugięcia),
- zbyt miękki model rozmywa fizykę i może maskować niestabilności.

## QP dla rozdziału sił i whole-body control
Kiedy masz wiele kontaktów i wiele zadań, naturalnie pojawia się problem optymalizacji:
- chcesz dobrać `τ` i/lub `f_c`, by śledzić zadania,
- spełnić ograniczenia tarcia i momentów,
- zachować stabilność.

Typowy schemat (intuicyjny, bez wchodzenia w detale implementacyjne):
```text
min    ||zadania_kinematyczne||^2 + ||regularizacja||^2
takie że
       dynamika (równania ruchu)
       ograniczenia kontaktu i tarcia
       limity τ, qd, qdd
```

## LCP/MCP i komplementarność
Jeśli chcesz modelować "albo kontakt, albo brak kontaktu" oraz brak penetracji:
```text
0 <= f_n ⟂ φ(q) >= 0
```
gdzie `φ(q)` to odległość (gap function). To prowadzi do LCP/MCP.
W praktyce:
- to bywa kosztowne numerycznie,
- w sterowaniu online często stosuje się przybliżenia (miękkie więzy, QP z ograniczeniami, heurystyki przełączania).

## Praktyka inżynierska: jak nie zrobić sobie krzywdy
- Nie ufaj ślepo parametrom mas i inercji z CAD: skalibruj lub przynajmniej sprawdź czułość sterowania na ich błąd.
- Oddziel model nominalny od kompensacji: feed-forward z modelu + feedback, który "dokręca" i stabilizuje.
- Kontakty traktuj hybrydowo i z marginesami: w realnym świecie kontakt jest zaszumiony i opóźniony.
- Loguj wielkości kluczowe: `f_n`, margines tarcia, saturacje `τ`, błędy zadania, rozbieżność modelu (residuum).

## Checklisty
- Sprawdzenie `M(q)` (symetria, dodatnia określoność) i `g(q)` na kilku pozach.
- Testy RNEA/ABA/CRBA w porównaniu z symulatorem (sanity check).
- Monitorowanie: saturacje `τ`, naruszenia tarcia, residua dynamiki.
- Walidacja kontaktu: histereza/filtrowanie detekcji, logowanie przełączeń stanów.

## Pytania do studentów
1. Jak odróżnisz błąd modelu dynamiki od błędu estymacji stanu (np. `qd/qdd`)?
2. Kiedy model kontaktu „twardy” jest lepszy od „miękkiego” i dlaczego?
3. Jakie informacje muszą być logowane, żeby debugować poślizg i naruszenia stożka tarcia?
4. Co może pójść źle numerycznie w QP/LCP i jak to wykryjesz w runtime?

## Projekty studenckie
- Porównanie inverse dynamics (RNEA) z prostą symulacją: feed-forward grawitacji i ocena jakości śledzenia.
- QP do rozdziału sił kontaktu (aproksymowany stożek tarcia) + metryki marginesu tarcia.
- „Kontakt hybrydowy”: FSM kontaktu + histereza detekcji + logowanie przełączeń i stabilności.

## BONUS
- Loguj residuum równania ruchu (różnica LHS-RHS) jako metrykę zdrowia modelu; to szybciej niż „czucie”, czy kontroler jedzie na fizyce czy na przypadkowych korektach.
