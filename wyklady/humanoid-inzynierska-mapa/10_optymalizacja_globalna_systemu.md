# Wykład 10: Optymalizacja globalna systemu

## Po co "globalna optymalizacja"
W humanoidzie prawie każda trudna decyzja jest kompromisem:
- stabilność vs. szybkość,
- energia vs. dynamika,
- dokładność śledzenia vs. bezpieczeństwo kontaktu,
- komfort mechaniczny (jerk) vs. czas wykonania.

Optymalizacja jest językiem, który pozwala te kompromisy opisać jawnie:
- funkcja kosztu koduje "co jest ważne",
- ograniczenia kodują "czego nie wolno".

W praktyce optymalizacja występuje w:
- IK z ograniczeniami (QP/NLP),
- whole-body control (QP z dynamiką i tarciem),
- MPC (predykcja na horyzoncie),
- estymacji (LS/MAP w factor graph),
- planowaniu trajektorii (TrajOpt/CHOMP/STOMP).

## Grafy: koszty, ograniczenia i zależności
### Graf kosztów i ograniczeń
Węzły to zmienne decyzyjne, a krawędzie/faktory to:
- składniki kosztu,
- ograniczenia równościowe i nierównościowe.

To jest analogiczne do factor graph w estymacji:
- tu jednak optymalizujesz "sterowania/traektorie", a nie "stany".

### Graf zależności zmiennych
To praktyczny sposób, by zrozumieć:
- które zmienne wpływają na które ograniczenia,
- gdzie jest sprzężenie (np. kontakt wpływa na dynamikę i jednocześnie na kolizje),
- jak rozbić problem na podproblemy (dekompozycja).

## Klasy problemów optymalizacji
### LP (Linear Programming)
```text
min    c^T x
takie że  A x <= b,  Aeq x = beq
```
Zalety: szybkie, przewidywalne.
W robotyce LP pojawia się np. w prostych alokacjach zasobów i ograniczeniach liniowych.

### QP (Quadratic Programming)
```text
min    1/2 x^T H x + c^T x
takie że  A x <= b,  Aeq x = beq
```
To jest koń roboczy humanoidów, bo:
- kwadratowy koszt dobrze modeluje "błędy" i regularizacje,
- ograniczenia liniowe łatwo kodują limity i aproksymacje stożka tarcia.

### NLP (Nonlinear Programming)
Gdy masz nieliniowe ograniczenia/cele (dokładna geometria, dokładna dynamika), potrzebujesz NLP.
Koszt: trudniejsze zbieżności i większy czas obliczeń.

### MILP/MIQP (mieszane z całkowitymi)
Pojawiają się, gdy masz decyzje dyskretne:
- wybór kontaktu (stopa lewa/prawa),
- wybór trybu (chodzenie vs. wspinanie),
- planowanie sekwencji zdarzeń.

Zalety: formalnie uchwycisz logikę.
Wada: obliczeniowo ciężkie; często używa się ich offline lub na małych podproblemach.

## Wypukłość: dlaczego wszyscy jej chcą
Jeśli problem jest wypukły:
- optimum jest globalne,
- solvery są stabilne,
- zachowanie jest przewidywalne.

Dlatego praktyczna zasada:
- modeluj tak, by najwięcej było wypukłe (QP), a nieliniowości przybliżaj lokalnie,
- jeśli musisz użyć NLP, pilnuj dobrego startu i regularizacji.

## Metody rozwiązywania: gradient, Newton, quasi-Newton, interior-point
Gradient descent:
- prosty, ale wolny i wrażliwy na skalowanie.

Newton:
- szybki lokalnie, wymaga Hessianu (lub jego przybliżenia) i dobrego startu.

Quasi-Newton (np. BFGS/L-BFGS):
- kompromis: dobra szybkość bez pełnego Hessianu.

Interior-point:
- standard dla LP/QP w wielu solverach,
- dobrze działa dla problemów z ograniczeniami.

W robotyce online liczy się:
- deterministyczny czas,
- odporność na problemy numeryczne,
- degradacja: solver nie może "zawiesić" systemu sterowania.

## Skalowanie i stabilność numeryczna: rzeczy, które bolą najbardziej
Najczęstsze źródła problemów:
- różne jednostki i skale (metry vs. radiany vs. niutony),
- źle dobrane wagi w koszcie (jedna dominuje i rozjeżdża pozostałe),
- słabe uwarunkowanie macierzy (osobliwości, niemal zależne ograniczenia),
- saturacje i sprzeczne ograniczenia (problem staje się niewykonalny).

Praktyczne techniki:
- normalizacja zmiennych,
- regularizacja (np. dodanie `ε I` do `H`),
- slack variables (miękkie naruszenia) z dużą karą zamiast twardej niewykonalności,
- monitorowanie statusu solvera i fallback.

## Online vs. offline
Offline:
- dokładniejsze modele, większe NLP/MILP, dłuższy czas,
- używane do: strojenia, generowania trajektorii bazowych, identyfikacji parametrów.

Online:
- krótkie horyzonty, QP, MPC, szybkie replany,
- nacisk na deterministyczny runtime i bezpieczeństwo.

## Projektowanie funkcji kosztu: inżynieria kompromisów
Typowa struktura:
- błąd celu (task),
- regularizacja postawy (posture),
- gładkość (qd/qdd/jerk),
- kary na sterowania (energia, saturacje),
- kary za bliskość ograniczeń (margines tarcia, margines kolizji).

Ważne:
- bezpieczeństwo i stabilność powinny mieć priorytet nad "ładnym śledzeniem",
- wagi dobierasz na danych i scenariuszach, nie na intuicji.

## Checklisty
- każda zmienna ma sensowną skalę i jednostkę,
- każdy ogranicznik ma uzasadnienie i jest mierzony w runtime,
- solver ma limit czasu i mechanizm fallback,
- logujesz: status, naruszenia, kondycję macierzy i wartości kosztu.

## Pytania do studentów
1. Dlaczego skalowanie zmiennych i jednostek ma krytyczny wpływ na stabilność solvera?
2. Kiedy QP jest wystarczające, a kiedy musisz przejść do NLP/SQP?
3. Jak zaprojektujesz slack variables, żeby uniknąć niewykonalności bez „psucia” bezpieczeństwa?
4. Jakie metryki numeryczne (kondycja, naruszenia) będziesz monitorować w runtime?

## Projekty studenckie
- QP z dwoma zadaniami (task + regularizacja postawy) i ograniczeniami limitów + analiza wpływu wag.
- „Solver guard”: limit czasu + fallback + logowanie statusów solvera.
- Eksperyment skalowania: te same dane w różnych jednostkach i obserwacja wpływu na zbieżność.

## BONUS
- Najlepsza funkcja kosztu to taka, która koduje priorytety bezpieczeństwa zanim zacznie „upiększać” śledzenie celu; w praktyce to często oznacza twarde ograniczenia + miękkie cele.
