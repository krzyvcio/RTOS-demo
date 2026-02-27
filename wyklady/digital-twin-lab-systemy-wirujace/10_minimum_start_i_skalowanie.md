# Wykład 10: Minimalny zestaw startowy i skalowanie do poziomu przemysłowego

## Cel
Wybrać zestaw narzędzi, który pozwala zacząć szybko, a potem skalować bez przebudowy wszystkiego od zera.

> TL;DR: Startuj „budżetowo” (Linux RT + Python + MCU), ale od początku miej format logów, scenariusze regresji i pomiary RT.

## Minimalny zestaw startowy (software)
- Linux + PREEMPT_RT (albo przynajmniej środowisko pod RT),
- Python (NumPy/SciPy) + notebooki do analizy,
- narzędzia do analizy sygnałów (FFT),
- narzędzia do testów i CI (unit/integration).

## Minimalny zestaw startowy (hardware)
- MCU dev board,
- interfejs komunikacji (UART/CAN/UDP),
- debug probe,
- logic analyzer,
- oscyloskop (jeśli masz).

## Skalowanie do poziomu przemysłowego (co zwykle dochodzi)
- bardziej złożona magistrala ruchu (jeśli potrzebna),
- izolacja i EMC,
- lepsza akwizycja danych,
- narzędzia FEA/modal do diagnostyki rezonansów,
- polityki bezpieczeństwa i formalizacja procedur testów.

## Checklisty
- Minimalny zestaw ma: harness + log format + scenariusze + metryki.
- Każde „dokładanie narzędzia” ma uzasadnienie w metrykach, nie w modzie.

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
