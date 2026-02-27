# Wykład 9: Bezpieczeństwo funkcjonalne, logika, nadzór

## Cel: wymusić bezpieczne zachowanie niezależnie od "inteligencji" systemu
Bezpieczeństwo funkcjonalne to warstwa, która:
- nie optymalizuje jakości ruchu,
- nie "walczy" o cel,
- tylko pilnuje, by system pozostał w granicach dopuszczalnego ryzyka.

W humanoidzie to oznacza m.in.:
- ograniczanie energii i prędkości w pobliżu człowieka,
- zapewnienie bezpiecznego zatrzymania w razie awarii czujników/napędów,
- kontrolę stanów kontaktu i stabilności,
- nadzór nad temperaturą, prądem, napięciem, czasem rzeczywistym.

## Grafy i struktury nadzoru
### FSM: graf stanów bezpieczeństwa
FSM (finite state machine) to najbardziej praktyczna forma:
- stany: NORMAL, WARNING, DEGRADED, SAFE_STOP, E_STOP, RECOVERY,
- przejścia: warunki na telemetryce i sygnałach bezpieczeństwa.

Zasada:
- przejścia do stanów "bardziej bezpiecznych" muszą być łatwe,
- wyjście ze stanu bezpiecznego wymaga warunków powrotu (histereza, potwierdzenie operatora, testy).

### Drzewo decyzyjne i reguły
W prostszych systemach:
- reguły progowe i drzewa decyzyjne (np. jeśli temperatura > X -> ogranicz moment).
To jest szybkie, ale łatwo o "reguły konfliktujące", więc FSM jest lepszy do globalnej spójności.

### Graf ścieżek awaryjnych
To opis: jakie akcje awaryjne są możliwe w danym stanie i dla danej awarii:
- natychmiastowe odcięcie momentu,
- kontrolowane hamowanie,
- przejście do postawy stabilnej,
- odciążenie kończyny.

Ważne:
- awaryjna ścieżka musi być przewidywalna i testowalna,
- nie może wymagać działania modułów, które mogą być właśnie uszkodzone.

### Graf watchdogów i nadzorców
Watchdogi pilnują "życia" modułów:
- pętla sterowania musi "karmić" watchdog,
- brak heartbeat -> przejście do stanu bezpiecznego.

W praktyce masz wiele watchdogów:
- lokalny w mikrokontrolerze napędu,
- w nadrzędnym kontrolerze czasu rzeczywistego,
- na poziomie systemu operacyjnego.

## Modele formalne: po co i kiedy
Formalne metody nie są po to, by zastąpić testy, tylko by:
- dowieść własności, których testami nie da się pokryć (bo przestrzeń stanów jest ogromna),
- wykryć luki logiczne w przejściach i warunkach bezpieczeństwa.

### LTL / CTL (logika temporalna)
Pozwala wyrazić własności typu:
- "zawsze, gdy wykryto błąd X, w końcu nastąpi SAFE_STOP",
- "nigdy nie będzie tak, że jednocześnie ACTIVE i E_STOP".

### Osiągalność stanów (reachability)
To analiza: czy istnieje ścieżka w grafie stanów, która prowadzi do niepożądanego stanu.
Przykład praktyczny:
- czy da się przejść do "NORMAL" bez przejścia przez "RECOVERY" po utracie IMU.

### Model checking
Automatyczne sprawdzenie własności (LTL/CTL) na modelu FSM.
Największa wartość jest wtedy, gdy:
- FSM jest duży,
- warunki przejść są złożone,
- chcesz uniknąć błędów typu "martwy stan" i "pętla bez wyjścia".

### Sieci Petriego
Przydatne do modelowania współbieżności:
- wiele zdarzeń i zasobów (np. dwa niezależne nadzory),
- unikanie zakleszczeń i warunków wyścigu w logice bezpieczeństwa.

## Projektowanie bezpiecznego zachowania: zasady konwencjonalne
### Safe state i definicja energii systemu
Najpierw definiujesz, co znaczy "bezpiecznie":
- odcięcie momentu (ale uwaga: robot może upaść),
- kontrolowane zatrzymanie (wymaga sprawnych napędów i estymacji),
- postawa podparcia i minimalizacja energii potencjalnej.

Praktyka w humanoidzie:
- SAFE_STOP bywa wielofazowy: najpierw kontrolowane hamowanie, potem odcięcie.

### Separacja kanałów: funkcjonalne vs. bezpieczeństwa
To kluczowa zasada:
- kanał bezpieczeństwa musi działać nawet, gdy kanał funkcjonalny ma błąd (software crash, błąd estymacji).

Oznacza to:
- niezależne limity (np. sprzętowe ograniczenie prądu),
- niezależne watchdogi,
- minimalne zależności od "inteligentnych" modułów.

### Klasy błędów i przejścia
Dla każdej klasy błędu definiujesz:
- detekcję (co obserwujesz),
- reakcję (co robisz),
- warunki powrotu (kiedy wolno wrócić).

Typowe klasy:
- błąd czasu rzeczywistego (missed deadline),
- błąd sensora (brak danych, outlier, dryf),
- błąd napędu (saturacja, przegrzanie, brak odpowiedzi),
- błąd kontaktu/stabilności (poślizg, utrata podparcia),
- błąd zasilania (spadek napięcia).

## Walidacja: testy + formalna weryfikacja
Praktyka:
- testy scenariuszowe (upadek, utrata sensora, przerwa w komunikacji),
- testy chaosowe (wstrzykiwanie opóźnień i utrat pakietów),
- model checking FSM dla własności krytycznych,
- audyt ścieżek awaryjnych i ich zależności.

## Checklisty
- każdy błąd ma określony safe response,
- przejścia w FSM mają histerezę (brak "flapping"),
- watchdogi są wielopoziomowe i testowane,
- logika bezpieczeństwa jest prosta i deterministyczna.

## Pytania do studentów
1. Jakie są minimalne stany FSM bezpieczeństwa dla robota i jakie warunki przejść są krytyczne?
2. Co zyskujesz przez rozdział kanału funkcjonalnego i kanału bezpieczeństwa?
3. Jakie własności warto weryfikować formalnie (LTL/CTL), a jakie wystarczą testami scenariuszowymi?
4. Jak zaprojektujesz recovery po fault tak, aby nie wrócić do niebezpiecznego stanu?

## Projekty studenckie
- Implementacja FSM bezpieczeństwa + logowanie przyczyn przejść + testy fault-injection.
- Specyfikacja własności (LTL/CTL) dla wybranych stanów i przejść + weryfikacja na modelu FSM.
- „Watchdog map”: mapa watchdogów (napęd, kontroler, OS) + plan testów awaryjnych.

## BONUS
- Safety powinno być „nudne”: proste reguły, deterministyczne przejścia, twarde limity; każda „sprytna” logika w safety zwiększa ryzyko luki.
