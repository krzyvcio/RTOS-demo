# Wykład 1: Modelowanie fizyczne i symulacja dynamiki (ODE/MDOF)

## Część I: Wstęp teoretyczny - Dlaczego model fizyczny jest fundamentem

### Geneza: od równań do symulacji

Modelowanie fizyczne systemów dynamicznych ma swoje korzenie w rewolucji przemysłowej XVIII i XIX wieku, gdy inżynierowie próbowali zrozumieć i przewidzieć zachowanie maszyn parowych, turbin i mechanizmów. Leonhard Euler, Joseph-Louis Lagrange i później Isaac Newton stworzyli matematyczny język, którym do dziś opisujemy ruch - równania różniczkowe zwyczajne (ODE).

W kontekście systemów wirujących, model fizyczny jest tym, co łączy świat fizyki ze światem sterowania. Bez modelu nie wiemy, czym sterujemy. Z modelem możemy przewidywać, symulować i projektować regulatory - zanim cokolwiek zbudujemy.

### Dlaczego zaczynamy od ODE, a nie od razu od skomplikowanych modeli

Prostota ma tu głęboki sens inżynierski. Model ODE dla prędkości wirnika:
```
J * dω/dt = T_motor - T_load - T_losses
```

To równanie mówi wszystko, co potrzebujemy na start: mamy bezwładność (J), mamy wejście (moment silnika), mamy zakłócenie (moment obciążenia) i mamy straty. Z tego równania możemy wyprowadzić odpowiedź czasową, wyznaczyć stałe czasowe, zaprojektować regulator.

Gdybyśmy zaczęli od pełnego modelu MDOF z dziesiątkami stopni swobody,:
- Symulacja byłaby wolna
- Nie wiedzielibyśmy, które efekty są istotne, a które są szczegółami
- Debugging byłby koszmarem

Zasada Pareto: 20% modelu ODE daje 80% użyteczności dla projektowania sterowania. Reszta to doprecyzowanie.

### Przemówienie Profesora

Moi studenci czasem pytają: "Dlaczego uczymy się tych prostych modeli, skoro mamy FEA, symulacje CFD, modele MES?".

Odpowiadam: bo macie mózg, a nie superkomputer. I dlatego, że najlepsze projekty sterowania, jakie widziałem, zaczynały się od prostego modelu.

Znałem inżyniera, który spędził rok na budowaniu szczegółowego modelu MES wirnika - setki tysięcy elementów, nieskończoność parametrów. Pod koniec roku miał model, który nie zgadzał się z pomiarami. Dlaczego? Bo nie wiedział, które parametry są krytyczne, a które - szum.

W tym samym czasie inny zespół wziął prosty model ODE, dopasował trzy parametry do danych z wirnika, i miał działający regulator w trzy miesiące.

Nie chodzi o to, żeby być prostym. Chodzi o to, żeby zaczynać od prostego i dodawać złożoność tylko tam, gdzie jest potrzebna. Ta dyscyplina odróżnia inżyniera od osoby, która nurkuje w szczegółach.

## Cel
Zbudować minimalny, a potem rozszerzalny model obiektu, który:
- pozwala testować regulator,
- pozwala symulować zakłócenia,
- ma parametry, które można później identyfikować.

> TL;DR: Zacznij od jednego równania prędkości, potem dołóż straty i zakłócenia, a MDOF i drgania przenieś do osobnej warstwy.

## Rdzeń: ODE dla prędkości (minimum)
Najprostszy model:
```text
J * domega/dt = T_motor - T_load - T_losses
```
Co to daje:
- `T_motor` jest wejściem sterowania,
- `T_load` jest zakłóceniem od procesu,
- `T_losses` to miejsce na tarcie i straty.

### Głębsze zrozumienie: co mówi nam to równanie

To pozornie proste równanie kryje w sobie całą filozofię sterowania systemami dynamicznymi. Lewa strona to "akumulacja energii kinetycznej" - bezwładność J razy przyspieszenie kątowe. Prawa strona to "bilans sił" - co wkładamy (moment silnika), co nam przeszkadza (obciążenie), co tracimy (tarcie).

Z tego równania możemy wyczytać fundamentalne właściwości:
- **Stała czasowa τ = J/b** - gdzie b to współczynnik tłumienia. To determinuje, jak szybko system reaguje na zmiany.
- **Wzmocnienie statyczne** - przy stałym obciążeniu, błąd steady-state zależy od współczynnika całkowania w regulatorze.
- **Limit przyspieszenia** - moment potrzebny do osiągnięcia zadanego przyspieszenia.

Dla was, jako przyszłych inżynierów, to równanie jest punktem wyjścia do **wszystkiego**. Od prostego PI/PID po zaawansowane sterowanie predykcyjne - wszystko zaczyna się od zrozumienia, że "coś" (bezwładność) jest sterowane przez "coś" (moment) i reaguje w określony sposób.

### Przemówienie Profesora

Kiedy mówię studentom o tym równaniu, zawsze proszę ich o jedno: wyobraźcie sobie, że stoicie przed prawdziwym wirnikiem. Macie regulator prędkości, macie zadać 5000 RPM, a wirnik ledwo się kręci. Co może być nie tak?

Odpowiedzi są różne - za mały moment (problem napędu), za duże obciążenie (problem mechaniki), za duże tarcie (problem łożyskowania). Ale wszystkie te odpowiedzi wynikają z JEDNEGO równania. Bo to równanie mówi wam, gdzie szukać.

I to jest esencja modelowania: nie po to, żeby zastąpić myślenie, ale żeby ukierunkować myślenie. Model to mapa, nie terytorium - ale mapa, która pokazuje, gdzie iść.

## Rozszerzenia, które zwykle są potrzebne
### Straty i tarcie
Praktycznie modelujesz je jako funkcję prędkości i temperatury, ale w labie startujesz od:
- składowa lepka (proporcjonalna do prędkości),
- składowa stała (offset),
- opcjonalnie nieliniowość (jeśli widać w danych).

### Saturacje i ograniczenia
Musisz zasymulować:
- limit momentu/prądu,
- limit przyspieszenia (rampy),
- ograniczenia napięciowe napędu (jeśli mają znaczenie dla dynamiki).

### Szumy pomiarowe i opóźnienia
W labie dodajesz:
- szum pomiaru prędkości,
- opóźnienie próbkowania,
- jitter próbkowania (w scenariuszach worst-case).

### Dlaczego te rozszerzenia są kluczowe

Model bez strat jest jak samochód bez tarcia -理论上的nieskończony poślizg, ale nie ma czegoś takiego w rzeczywistości. Tarcie jest WSZĘDZIE:
- Tarcie w łożyskach (Coulomb + wiskotyczne)
- Tarcie aerodynamiczne (proporcjonalne do ω²)
- Tarcie w przekładniach

I co ważne - tarcie jest zwykle **nieliniowe** i **zależne od temperatury**. W laboratorium chcemy zamodelować przynajmniej podstawowe efekty, bo inaczej sterowanie zaprojektowane na "idealnym" modelu będzie oscylować na rzeczywistym.

Saturacje to drugi fundamentalny element. W idealnym świecie regulator może wygenerować dowolny moment. W rzeczywistości:
- Falownik ma limit prądowy
- Silnik ma limit termiczny
- Mechanika ma limit siłowy

I tu pojawia się zjawisko "windup" - gdy regulator "widzi", że nie może osiągnąć celu, zaczyna akumulować błąd w członie całkującym. Potem, gdy ograniczenie ustępuje, regulator "wypluwa" tę akumulację - i dostajemy ogromne przeregulowanie.

### Przemówienie Profesora

Najgorsze rzeczy w sterowaniu zwykle dzieją się "na granicach". Nie w nominalnym punkcie pracy, ale właśnie przy saturacjach, przy przejściach, przy zakłóceniach.

Pamiętam jeden projekt, gdzie regulator prędkości świetnie działał w laboratorium - do momentu, gdy klient zaczął używać wirnika przy pełnym obciążeniu. Okazało się, że przy dużym obciążeniu moment tarcia przekraczał limit napędu. Regulator widział "uciekającą" prędkość i zwiększał wyjście - ale wyjście było clamped na maximum. Windup powodował, że po ustąpieniu obciążenia wirnik "wystrzeliwał" do prędkości znacznie powyżej zadanej.

Rozwiązanie było proste - anti-windup. Ale żeby je zaprojektować, trzeba było najpierw **zamodelować saturację**. A żeby zamodelować saturację, trzeba było zrozumieć, gdzie są granice systemu.

Rada: od początku zakładajcie, że wszystko ma limity. I testujcie zachowanie przy tych limitach.

## MDOF: po co i kiedy
Gdy chcesz modelować drgania i rezonanse, przechodzisz do postaci:
```text
M * qdd + C * qd + K * q = F(t)
```
Zastosowanie w labie:
- generowanie "wibracji" jako sygnału diagnostycznego,
- testowanie filtrów (notch),
- testowanie odporności sterowania na rezonanse.

### Kiedy MDOF ma sens

MDOF (Multiple Degrees of Freedom) to przejście od jednego równania do układu równań. To nie jest "lepszy" model - to model, który odpowiada na inne pytania.

Pytanie, które powinno was nakłonić do MDOF:
- Czy interesują mnie drgania w konkretnych punktach konstrukcji?
- Czy mam rezonanse, które muszę rozumieć?
- Czy muszę przewidywać zachowanie przy przejściu przez "krytyczne" prędkości?

Jeśli odpowiedź brzmi "nie" - zostańcie przy ODE. MDOF jest złożony i kosztowny obliczeniowo.

### Przemówienie Profesora

W swojej karierze widziałem dwa skrajne podejścia do MDOF:
1. "Mamy FEA, mamy wszystko" - i potem model nie pasuje do rzeczywistości, bo parametry są nieznane
2. "ODE wystarczy" - i potem system wchodzi w rezonans, którego nikt nie przewidział

Prawda jest pośrodku. MDOF ma sens, gdy:
- Macie dane z pomiarów, które wskazują na problem rezonansowy
- Pracujecie z konstrukcją, gdzie rezonanse są krytyczne (długie wały, podatne mocowania)
- Chcecie testować filtry notch, które muszą "trafić" w konkretną częstotliwość

W labie będziemy używać MDOF jako narzędzia do generowania "wibracji syntetycznych" - żeby testować diagnostykę i filtry. Nie będziemy budować pełnego modelu MES - zamiast tego użyjemy uproszczonego modelu modalnego, który jest wystarczający dla celów sterowania.

## ODE solver i „sztywność” układu
W praktyce układy z kontaktami/filtrami/dużymi różnicami czasów bywają sztywne.
Wybór solvera i kroku czasowego wpływa na wiarygodność wniosków:
- jeśli krok jest za duży, symulacja ukrywa niestabilności,
- jeśli krok jest za mały, symulacja jest wolna i utrudnia iteracje.

## Walidacja modelu w labie (zanim masz hardware)
Zasada:
- walidujesz nie "prawdę absolutną", tylko zgodność zachowań klasowych.

Przykłady testów:
- odpowiedź na skok zadania (rampa),
- odpowiedź na skok zakłócenia `T_load`,
- saturacja momentu i reakcja anti-windup,
- odporność na opóźnienie i jitter.

### Filozofia walidacji modelu

Model nie musi być "prawdziwy" - musi być **użyteczny**. To jest fundamentalna różnica w podejściu. Model jest narzędziem do odpowiadania na pytania, nie symulacją rzeczywistości.

Pytanie, które powinno was prowadzić: **"Co chcę sprawdzić/ zaprojektować?"**

- Jeśli chcę dobrać parametry PI - wystarczy ODE z tarciem
- Jeśli chcę sprawdzić, czy regulator przejdzie przez rezonans - potrzebuję MDOF z częstotliwościami własnymi
- Jeśli chcę sprawdzić, czy system przeżyje opóźnienie - potrzebuję wstrzykiwać opóźnienie w pętli

Każdy test walidacyjny powinien odpowiadać na konkretne pytanie. Nie testujcie "wszystkiego" - testujcie to, co jest istotne dla waszego celu.

### Przemówienie Profesora

Najczęstszy błąd, jaki widzę u studentów: próbują sprawdzić, czy model jest "prawdziwy" - czy wyniki dokładnie odpowiadają rzeczywistości.

To jest pułapka. Model NIGDY nie będzie dokładnie odpowiadać rzeczywistości - zawsze są pominięte zjawiska, uproszczenia, niepewność parametrów.

Zamiast tego pytajcie: "Czy model zachowuje się **podobnie** do rzeczywistości **w zakresie, który mnie interesuje**?"

Różnica jest subtelna, ale kluczowa. Odpowiedź na skok zadania może być inna w szczegółach - ale czy **kształt** odpowiedzi jest podobny? Czy **trendy** są prawidłowe? Czy **zachowanie jakościowe** się zgadza?

To są pytania, na które warto odpowiadać.

## Checklisty
- Masz model minimalny + testy jednostkowe (sanity).
- Masz symulowane ograniczenia (saturacje, rampy).
- Masz szum i opóźnienie w torze pomiarowym.

### Checklist szczegółowy

**Model podstawowy:**
- [ ] Równanie ODE zapisane i rozwiązywalne numerycznie
- [ ] Parametry J (bezwładność), b (tłumienie) zdefiniowane
- [ ] Test jednostkowy: czy przy stałym wejściu prędkość dąży do stanu ustalonego?
- [ ] Test jednostkowy: czy odpowiedź na skok ma oczekiwany kształt?

**Rozszerzenia:**
- [ ] Model tarcia: lepka składowa + offset (Coulomb)
- [ ] Saturacja momentu: zaimplementowana i testowana
- [ ] Rampa prędkości: ograniczenie jerk i przyspieszenia
- [ ] Szum pomiarowy: Gaussian noise na prędkości
- [ ] Opóźnienie: stałe (transport delay)
- [ ] Jitter: losowy component w opóźnieniu

**Walidacja:**
- [ ] Test skoku zadania: odpowiedź zbliżona do oczekiwanej
- [ ] Test zakłócenia: jak system radzi sobie ze skokiem obciążenia
- [ ] Test saturacji: czy anti-windup działa (jeśli zaimplementowany)
- [ ] Test odporności: jak system zachowuje się z opóźnieniem/jitterem

### Przemówienie Profesora

Te checklisty to nie są "rzeczy do odhaczenia". To są punkty, gdzie **możecie się potknąć**. I lepiej, żebyście się potknęli teraz, na etapie modelu, niż potem, na etapie sterowania albo - nie daj Boże - na prawdziwym sprzęcie.

Każdy punkt na tej liście kosztował kogoś godziny albo dni debugowania. Nauczcie się na cudzych błędach.

## Slajdy (tekstowe)
### Slajd 1: Minimum modelu
- Jedno ODE: `J * domega/dt = T_motor - T_load - T_losses`
- Wejście: moment, zakłócenie: load

### Slajd 2: Co trzeba dodać, żeby było „realnie”
- Saturacje
- Opóźnienie i jitter
- Szum pomiaru

### Slajd 3: Kiedy MDOF
- Gdy testujesz rezonanse i notch

## Pytania do studentów
1. Które elementy modelu ODE są niezbędne do testów sterowania, a które można odłożyć na później?
2. Jak zasymulujesz saturacje i anti-windup, aby testy były wiarygodne?
3. Jak ocenisz, czy solver i krok czasowy nie maskują niestabilności?
4. Jakie parametry modelu planujesz później identyfikować na danych z hardware?

## Projekty studenckie
- „Minimal plant”: implementacja modelu `J*dω/dt` z tarciem i zakłóceniami + testy scenariuszy.
- „Noise+delay injector”: moduł wstrzykujący szum, opóźnienie i jitter w pomiarze.
- „Parameter fit”: prosta identyfikacja parametrów tarcia na podstawie danych z symulacji lub logów.

## BONUS
- Rozdziel model na: „prawa fizyki” (plant) i „artefakty systemu” (opóźnienia, jitter, kwantyzacja). To przyspiesza debug, bo wiesz, co jest winą modelu, a co architektury.
