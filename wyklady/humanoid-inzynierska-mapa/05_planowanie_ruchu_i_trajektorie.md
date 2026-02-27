# Wykład 5: Planowanie ruchu, trajektorie

## Co znaczy "planowanie" w humanoidzie
Planowanie ruchu to nie jeden algorytm, tylko pipeline:
- wybór celu i strategii (np. podejść, sięgnąć, postawić stopę),
- plan geometryczny (bez kolizji),
- plan dynamiczny (wykonalny z ograniczeniami sił i momentów),
- generacja trajektorii (gładkość, jerk, ograniczenia),
- wykonanie z kontrolą i replanningiem w pętli (bo świat jest niepewny).

W humanoidzie planowanie obejmuje także:
- planowanie kontaktów (kiedy i gdzie stawiam stopę),
- stabilność (nie wywrócić się),
- bezpieczeństwo w pobliżu człowieka.

## Grafy i struktury
### Graf stanów
Stan może być różnie zdefiniowany:
- konfiguracja przegubów `q`,
- pozycja bazy + konfiguracja (dla ruchu całego ciała),
- abstrakcyjny stan chodu (np. lewa stopa w kontakcie, prawa w locie).

W planowaniu często używa się stanu o mniejszym wymiarze niż pełne `q`, bo pełny stan humanoida jest ogromny.

### Graf C-space i kolizji
Przestrzeń konfiguracji C-space to przestrzeń wszystkich `q`.
Przeszkody w świecie mapują się na obszary niedozwolone w C-space.
Praktycznie:
- sprawdzasz kolizje przez collision checking na geometrii (mesh, kapsuły),
- do planowania w czasie rzeczywistym często stosuje się uproszczone bryły (capsule/sphere).

### Drzewa przeszukiwania: RRT, RRT*
RRT:
- buduje drzewo przez losowe próbkowanie,
- dobrze działa w wysokich wymiarach,
- nie gwarantuje optimum, ale szybko znajduje "jakąś" ścieżkę.

RRT*:
- z czasem poprawia rozwiązanie (asymptotycznie optymalne),
- jest wolniejszy, ale daje lepsze trajektorie przy dłuższym czasie planowania.

Klucz praktyczny:
- wydajność zależy głównie od szybkości collision checkera i heurystyk próbkowania.

### PRM i roadmap
PRM:
- precomputuje graf połączeń w C-space,
- świetne, gdy środowisko jest w miarę stałe i masz wiele zapytań.

Roadmap:
- ogólna idea: graf przejezdności, w którym wyszukujesz ścieżkę.

## Planowanie na grafach: A*, D*, Dijkstra
Dijkstra:
- najkrótsza ścieżka bez heurystyki.

A*:
- Dijkstra + heurystyka (np. odległość do celu),
- standard w planowaniu w siatkach i graph roadmaps.

D*:
- warianty do replanningu przy zmianach mapy/otoczenia.

W humanoidzie często:
- globalnie planujesz w 2D/3D (mapa),
- lokalnie rozwiązujesz problem w przestrzeni ruchu kończyny lub bazy.

## Trajektorie: gładkość, ograniczenia i czas
Nawet jeśli ścieżka geometryczna jest OK, nadal musisz:
- wygenerować profil czasowy (time-parameterization),
- ograniczyć prędkości, przyspieszenia i jerk,
- zapewnić ciągłość (zwykle C1 lub C2).

Typowe narzędzia:
- splajny cubic (C2 w segmentach) i quintic (lepsza kontrola warunków brzegowych),
- B-splines: lokalna kontrola kształtu i gładkości,
- Bezier: wygodne warunki brzegowe i intuicyjna geometria.

W praktyce humanoida liczy się jerk:
- duży jerk = uderzenia w przekładnie, poślizgi, pobudzenie rezonansów,
- więc ograniczanie jerk to często "ukryty" warunek stabilności mechanicznej.

## Optymalizacja trajektorii: CHOMP, STOMP, TrajOpt
Te metody traktują trajektorię jako zmienną optymalizacji.

CHOMP:
- gradientowo minimalizuje koszt gładkości + koszt kolizji.

STOMP:
- wariant stochastyczny (sampling), bywa odporniejszy na lokalne minima.

TrajOpt:
- zwykle formułowane jako problem (kolejno) wypukły / SQP,
- popularny w praktycznych systemach, bo integruje ograniczenia i kolizje.

Najważniejsze kompromisy:
- szybkość planowania vs. jakość/bezpieczeństwo,
- globalne optimum vs. szybki replanning,
- dokładna geometria vs. szybkie przybliżenia kolizji.

## Ograniczenia: QP/NLP/SQP w praktyce
W humanoidzie ograniczenia są zawsze:
- limity przegubów i napędów,
- ograniczenia kontaktu (tarcie, brak poślizgu),
- ograniczenia stabilności (np. nie przekroczyć dopuszczalnego regionu podparcia),
- ograniczenia środowiskowe (kolizje, strefy zakazane).

QP:
- szybkie, stabilne numerycznie, dobre do online,
- wymaga kwadratowych kosztów i liniowych (lub liniaryzowanych) ograniczeń.

NLP/SQP:
- bardziej ogólne (nieliniowe ograniczenia i koszty),
- wolniejsze i wrażliwe na start, ale potrafią modelować więcej fizyki.

## Funkcje kosztu: energia, jerk, czas
Typowa konstrukcja kosztu to ważona suma:
- błąd śledzenia celu,
- energia lub norma sterowań,
- gładkość (przyspieszenie/jerk),
- marginesy bezpieczeństwa od kolizji i tarcia,
- kara za "dziwne" postawy (posture regularization).

Dobór wag to inżynieria:
- zbyt duża kara za czas -> agresywne trajektorie,
- zbyt duża gładkość -> robot "nie dojdzie" na czas,
- zbyt mała regularizacja postawy -> ekstremalne ułożenia przegubów.

## Planowanie kroków i kontaktów: specyfika humanoida
Konwencjonalny podział problemu chodu:
- planowanie stóp (footstep planning): gdzie postawić stopę,
- planowanie tułowia/CoM: jak przesuwać masę, by utrzymać stabilność,
- planowanie faz: kiedy stopa ma kontakt, a kiedy swing.

W praktyce te warstwy często są sprzężone przez ograniczenia kontaktu i dynamikę.
Dlatego w systemach czasu rzeczywistego:
- globalny plan bywa prosty (kolejne kroki),
- lokalny kontroler (WBC/MPC) robi "prawdziwą fizykę" na krótkim horyzoncie.

## Praktyka inżynierska: co działa w realnym systemie
- Połącz planowanie globalne z lokalnym replanningiem i kontrolą reaktywną.
- Uwzględniaj ograniczenia dynamiki i kontaktu wcześniej niż później: inaczej plan "bez kolizji" będzie niewykonalny.
- Miej tryby degradacji: gdy planowanie nie zdąży, robot musi przejść w zachowanie bezpieczne (stop, stabilizacja).

## Checklisty
- metryki jakości: margines kolizji, margines tarcia, maks. jerk, saturacje,
- testy regresji na scenariuszach: wąskie przejście, schody, poślizg, zaburzenia,
- monitorowanie czasu planowania i liczby prób collision checkera.

## Pytania do studentów
1. Kiedy plan geometryczny jest bezużyteczny bez uwzględnienia dynamiki i kontaktu?
2. Jakie są zalety i wady RRT/RRT* vs PRM w kontekście replanningu?
3. Dlaczego jerk jest „ukrytym” ograniczeniem stabilności mechanicznej?
4. Jak budujesz funkcję kosztu, żeby nie wymuszała ekstremalnych postaw?

## Projekty studenckie
- Implementacja RRT dla prostego układu (2D/planarny) + porównanie z A* na siatce.
- Generator trajektorii na splajnach (quintic) z ograniczeniem jerk + walidacja czasowa.
- TrajOpt/SQP w wersji „toy” z ograniczeniami i kolizjami (uprościć geometrię do kapsuł).

## BONUS
- W praktycznych systemach wygrywa hybryda: plan globalny + lokalny kontroler reaktywny; projektuj od początku punkty, gdzie można robić replanning bez destabilizacji.
