# Wykład 1: Modelowanie fizyczne i symulacja dynamiki (ODE/MDOF)

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

## MDOF: po co i kiedy
Gdy chcesz modelować drgania i rezonanse, przechodzisz do postaci:
```text
M * qdd + C * qd + K * q = F(t)
```
Zastosowanie w labie:
- generowanie „wibracji” jako sygnału diagnostycznego,
- testowanie filtrów (notch),
- testowanie odporności sterowania na rezonanse.

## ODE solver i „sztywność” układu
W praktyce układy z kontaktami/filtrami/dużymi różnicami czasów bywają sztywne.
Wybór solvera i kroku czasowego wpływa na wiarygodność wniosków:
- jeśli krok jest za duży, symulacja ukrywa niestabilności,
- jeśli krok jest za mały, symulacja jest wolna i utrudnia iteracje.

## Walidacja modelu w labie (zanim masz hardware)
Zasada:
- walidujesz nie „prawdę absolutną”, tylko zgodność zachowań klasowych.

Przykłady testów:
- odpowiedź na skok zadania (rampa),
- odpowiedź na skok zakłócenia `T_load`,
- saturacja momentu i reakcja anti-windup,
- odporność na opóźnienie i jitter.

## Checklisty
- Masz model minimalny + testy jednostkowe (sanity).
- Masz symulowane ograniczenia (saturacje, rampy).
- Masz szum i opóźnienie w torze pomiarowym.

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
