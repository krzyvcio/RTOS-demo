# Wykład 10: Minimalny zestaw startowy i skalowanie do poziomu przemysłowego

## Część I: Wstęp teoretyczny - Jak zacząć i jak skalować

### Geneza: od zera do produkcji

Przeszliśmy długą drogę:
- Modelowanie (ODE, MDOF)
- Sterowanie (PI/PID, filtry)
- Czas rzeczywisty (Linux PREEMPT_RT)
- Komunikacja (EtherCAT, CAN)
- HIL i harness
- Analiza (FFT)
- CI/CD

Ale pytanie pozostaje: **jak zacząć?** I **jak przejść od laboratorium do produkcji?**

### Minimalny zestaw startowy

Nie potrzbujecie od razu wszystkiego. Potrzebujecie MINIMUM, które pozwoli:
- Testować sterowanie bez hardware
- Mierzyć podstawowe metryki
- Iterować szybko

To minimum to:
- Model ODE na PC
- Regulator w Python/C
- Logowanie do pliku
- Podstawowe scenariusze

### Skalowanie do produkcji

Gdy laboratorium działa, pojawiają się nowe wymagania:
- Większa deterministyka (hard RT)
- Bezpieczeństwo (safety)
- EMC
- Akwizycja danych
- Formalne procedury

### Przemówienie Profesora

Najczęstszy błąd: "zróbmy od razu profesjonalnie".

Nie. Zróbcie minimum, które działa. Potem dodawajcie.

Dlaczego? Bo każda warstwa komplikacji ma koszt:
- Więcej czasu na start
- Więcej rzeczy do zepsucia
- Trudniejsze debugowanie

Ale też: każda warstwa ma zwrot. Dodajcie ją, gdy potrzebujecie.

## Cel
Wybrać zestaw narzędzi, który pozwala zacząć szybko, a potem skalować bez przebudowy wszystkiego od zera.

> TL;DR: Startuj "budżetowo" (Linux RT + Python + MCU), ale od początku miej format logów, scenariusze regresji i pomiary RT.

## Część II: Co wybrać na start

## Minimalny zestaw startowy (software)
- Linux + PREEMPT_RT (albo przynajmniej środowisko pod RT),
- Python (NumPy/SciPy) + notebooki do analizy,
- narzędzia do analizy sygnałów (FFT),
- narzędzia do testów i CI (unit/integration).

### Minimalny zestaw startowy (hardware)
- MCU dev board,
- interfejs komunikacji (UART/CAN/UDP),
- debug probe,
- logic analyzer,
- oscyloskop (jeśli masz).

### Dlaczego te wybory

**Python + NumPy/SciPy**:
- Szybki start (błądzenia)
- Bogata biblioteka do analizy
- Łatwe prototypowanie

**Linux PREEMPT_RT**:
- Wspólna platforma z docelowym systemem
- Łatwiejsze przejście do produkcji

**MCU dev board**:
- Niskie koszty
- Dostępność
- Wsparcie

### Przemówienie Profesora

Pamiętajcie: "budżetowy" nie znaczy "zły". Budżetowy znaczy: zacznij od czegoś, co pozwoli ci iterować szybko.

Potem, gdy masz działający system - wtedy inwestuj w lepszy sprzęt.

Ale nigdy nie inwestuj na początku, gdy jeszcze nie wiesz, co działa.

## Skalowanie do poziomu przemysłowego (co zwykle dochodzi)
- bardziej złożona magistrala ruchu (jeśli potrzebna),
- izolacja i EMC,
- lepsza akwizycja danych,
- narzędzia FEA/modal do diagnostyki rezonansów,
- polityki bezpieczeństwa i formalizacja procedur testów.

### Kiedy skalować

**Magistrala ruchu**:
- Gdy masz wiele osi
- Gdy potrzebujesz synchronizacji
- Gdy wymagasz determinizmu

**Izolacja i EMC**:
- Gdy wchodzisz w środowisko przemysłowe
- Gdy masz zakłócenia

**Lepsza akwizycja**:
- Gdy potrzebujesz większej precyzji
- Gdy potrzebujesz większej częstotliwości

**FEA/modal**:
- Gdy masz problemy z rezonansami
- Gdy potrzebujesz lepszej diagnostyki

**Safety**:
- Gdy wchodzisz w application z wymaganiami bezpieczeństwa
- Gdy potrzebujesz certyfikacji

### Przemówienie Profesora

Skalowanie jest OK. Ale skalowanie bez powodu jest pułapką.

Zadawajcie sobie pytanie: **DLACZEGO dodaję tę warstwę?**

Jeśli odpowiedź to "bo jest modna" albo "bo tak wszyscy robią" - nie dodawajcie.

Jeśli odpowiedź to "bo rozwiązuje konkretny problem" - dodawajcie.

To jest różnica między inżynierem a fanboy'em technologii.

## Checklisty
- Minimalny zestaw ma: harness + log format + scenariusze + metryki.
- Każde "dokładanie narzędzia" ma uzasadnienie w metrykach, nie w modzie.

### Checklist szczegółowy

**Minimum:**
- [ ] Model ODE działa
- [ ] Regulator działa
- [ ] Logowanie działa
- [ ] Scenariusze testowe działają
- [ ] Metryki są mierzone

**Skalowanie:**
- [ ] Uzasadnienie biznesowe dla każdej nowej warstwy
- [ ] Kryteria wejścia (kiedy dodajemy)
- [ ] Kryteria wyjścia (kiedy wystarczy)
- [ ] Plan migracji (z minimum do pełnego)

**Inwestycje:**
- [ ] Priorytetyzacja (co teraz, co później)
- [ ] Budżet (ile to kosztuje)
- [ ] ROI (co zyskujemy)

## Slajdy (tekstowe)
### Slajd 1: Start budżetowy
- Linux RT + Python + MCU
- logi, scenariusze, metryki

### Slajd 2: Skalowanie
- magistrala, safety, EMC, akwizycja

## Pytania do studentów
1. Co jest absolutnym minimum, aby móc wiarygodnie testować sterowanie bez hardware?
2. Jakie elementy muszą być zaprojektowane „od początku” (format logów, scenariusze, metryki), żeby skalowanie nie wymagało przebudowy?
3. Kiedy warto przejść z SIL do HIL i jakie ryzyka to redukuje?
4. Jak rozpoznasz, że dokładasz narzędzie „bo modne”, a nie „bo poprawia metryki”?

## Projekty studenckie
- „Budget starter kit”: komplet minimalny (model + regulator + log format + scenariusze + raporty).
- „Scale plan”: plan skalowania labu (magistrala, HIL, safety, EMC) z kryteriami wejścia/wyjścia.
- „Procurement list”: lista zakupowa pod 2 budżety + uzasadnienie metrykami i ryzykiem.

## BONUS
- Najlepsze projekty studenckie nie zaczynają od sprzętu. Zaczynają od logów, scenariuszy i metryk, bo to pozwala iterować na architekturze bez chaosu.
