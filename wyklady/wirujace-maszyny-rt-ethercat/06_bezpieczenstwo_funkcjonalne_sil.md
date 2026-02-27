# Wykład 6: Bezpieczeństwo funkcjonalne (SIL) i reakcje awaryjne

## Cel
Zaprojektować warstwę bezpieczeństwa, która jest niezależna od „sprytnego” sterowania i działa w najgorszych warunkach.

> TL;DR: Safety to osobny kanał decyzyjny. Musi działać, gdy: master się zawiesi, komunikacja padnie, sensor oszaleje, a obiekt ma nadal energię kinetyczną.

## Zasada podstawowa
Bezpieczeństwo funkcjonalne odpowiada na pytanie:
„co system zrobi, gdy coś pójdzie źle”.

Typowe zdarzenia:
- przegrzanie,
- przekroczenie prądu,
- utrata czujnika prędkości,
- utrata synchronizacji,
- missed deadline w pętli,
- błąd komunikacji z napędem.

## Co znaczy „SIL” w praktyce (bez wchodzenia w formalizmy)
Niezależnie od poziomu formalnej certyfikacji, praktycznie interesuje Cię:
- identyfikacja zagrożeń,
- funkcje bezpieczeństwa (co ma się zadziać),
- niezależność kanału bezpieczeństwa od kanału sterowania funkcjonalnego,
- testowalność (czy umiesz to zweryfikować w runtime i w testach).

Najważniejsza zasada architektoniczna:
- nie możesz zakładać, że moduł sterowania funkcjonalnego jest „żywy” i „racjonalny”, gdy dochodzi do awarii.

## Warstwy zabezpieczeń (praktyka)
- sprzętowe limity prądu/temperatury w napędzie,
- watchdog w napędzie (brak komunikacji -> safe state),
- watchdog w kontrolerze (brak pętli -> safe state),
- logika awaryjna: kontrolowane hamowanie vs odcięcie momentu (zależnie od ryzyka).

## Safe stop: różne strategie i ich ryzyka
W praktyce spotkasz co najmniej dwa podejścia:
- odcięcie momentu (szybko, prosto, ale obiekt może „pójść własną drogą”),
- kontrolowane hamowanie (bezpieczniejsze dla układu, ale wymaga sprawnych napędów i pomiarów).

Dlatego safe stop musi być:
- zdefiniowany dla konkretnych scenariuszy awarii,
- zaprojektowany jako sekwencja (np. hamowanie -> zatrzymanie -> odcięcie),
- odporny na brak części sygnałów (degradacja sensoryki).

## Co znaczy „safe stop” w praktyce
To musi być jawnie zdefiniowane dla danej maszyny:
- „odcięcie” bywa niebezpieczne, jeśli obiekt ma dużą energię kinetyczną,
- „kontrolowane hamowanie” wymaga sprawnych napędów i sensownych pomiarów.

Wniosek:
- projektujesz tryb awaryjny tak, by minimalizował ryzyko w typowych awariach.

## Separacja kanałów: funkcjonalny vs bezpieczeństwa
To klucz do odporności:
- kanał funkcjonalny: regulacja, optymalizacja, telemetria,
- kanał bezpieczeństwa: proste reguły, watchdogi, limity, safe stop.

W praktyce oznacza to:
- niezależne limity w napędzie (sprzętowo/firmware),
- watchdogi wielopoziomowe (slave + master),
- minimalne zależności (safety nie czeka na „ładne dane” z telemetrii).

## Wstrzykiwanie błędów: jak testować safety
Najbardziej praktyczne testy to testy awaryjne:
- dropout komunikacji,
- opóźnienie w pętli (missed deadline),
- brak czujnika prędkości lub pomiar „zamrożony”,
- przegrzanie (symulowane),
- saturacja prądu/momentu (symulowana).

Klucz: test musi sprawdzać nie tylko, że „zatrzymało”, ale:
- czy reakcja była w czasie,
- czy nie było flappingu,
- czy powrót do normalnej pracy jest kontrolowany (warunki powrotu).

## Checklisty
- Każda klasa awarii ma przypisaną reakcję i warunki powrotu.
- Watchdogi są wielopoziomowe (napęd + kontroler).
- Bezpieczeństwo nie zależy od telemetrii i „miękkich” usług.
- Testujesz awarie przez wstrzykiwanie błędów (dropouty, opóźnienia, brak czujnika).

## Zadania (praktyka)
1. Zdefiniuj 6 klas awarii i dla każdej: detekcja -> reakcja -> warunki powrotu.
2. Narysuj FSM bezpieczeństwa z co najmniej 5 stanami (NORMAL/WARNING/DEGRADED/SAFE_STOP/RECOVERY).
3. Ułóż plan testów fault-injection i kryteria „pass/fail” (czas reakcji, brak flappingu, kontrolowany powrót).

## Pytania do studentów
1. Dlaczego kanał bezpieczeństwa musi działać bez „smart” modułów (telemetria, ML, HMI)?
2. Kiedy „odcięcie momentu” może być gorsze niż kontrolowane hamowanie i dlaczego nie ma jednej odpowiedzi dla wszystkich maszyn?
3. Jak zapewnisz, że powrót z SAFE_STOP do NORMAL nie spowoduje flappingu i niebezpiecznych przejść?
4. Jakie testy fault-injection uznasz za obowiązkowe w każdym wydaniu (release)?

## Projekty studenckie
- „Safety FSM”: implementacja FSM bezpieczeństwa + logowanie przyczyn przejść + testy scenariuszowe.
- „Fault injector”: moduł do wstrzykiwania dropoutów, opóźnień, zamrożonych pomiarów i saturacji.
- „Release gate”: checklista i skrypt, który blokuje release bez przejścia testów awaryjnych.

## BONUS
- Jeśli nie da się łatwo opisać reakcji na awarię w jednym zdaniu i jednej tabeli, logika safety jest prawdopodobnie zbyt złożona i powinna zostać uproszczona.
