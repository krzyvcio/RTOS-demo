# Wykład 7: Digital Twin bez wirnika (PC symuluje plant, kontroler myśli że steruje hardware)

## Część I: Wstęp teoretyczny - Czym jest harness i dlaczego jest kluczowy

### Geneza: od symulacji do powtarzalnego testowania

Do tej pory zbudowaliśmy:
- Model obiektu (ODE/MDOF)
- Sterowanie (PI/PID, filtry)
- Środowisko RT (Linux PREEMPT_RT lub RTOS)
- Komunikację (EtherCAT lub CAN)
- HIL (MCU + RTOS)

Ale brakuje czegoś kluczowego: **sposobu na systematyczne testowanie całości**. Setupy, które do tej pory budowaliśmy, są jednorazowe - uruchamiamy, patrzymy, zmieniamy, uruchamiamy znowu.

Problem: nie da się w ten sposób:
- Porównywać wyników między sesjami
- Automatyzować testów
- Regresować zmian
- Powtarzać dokładnie ten sam scenariusz

Harness rozwiązuje te problemy. To szkielet/szkielet, który:
- Standaryzuje interfejsy między komponentami
- Umożliwia automatyczne uruchamianie scenariuszy
- Loguje wszystko w ustrukturyzowany sposób
- Pozwala na replay i porównania

### Przemówienie Profesora

Największy problem, jaki widzę w projektach studenckich: "okej, teraz działa, zapiszmy te ustawienia".

I po miesiącu nikt nie wie, jakie były te ustawienia, jakie były wyniki, co się zmieniło.

Harness to nie jest "dodatkowa praca" - to inwestycja w powtarzalność. W przyszłości będziecie wdzięczni sobie za dzisiejszy czas poświęcony na harness.

## Cel
Zbudować "harness" do testów:
- PC symuluje obiekt (ODE/MDOF),
- kontroler (Linux RT lub MCU) liczy sterowanie,
- wymiana danych przypomina realny system (opóźnienia, dropouty, timestampy).

> TL;DR: Kontroler nie powinien wiedzieć, czy steruje prawdziwym obiektem czy modelem.

## Część II: Architektura harnessu

## Wzorzec architektury
W najprostszej formie:
- kontroler wysyła `torque_cmd`,
- plant odsyła `omega_meas` (z szumem i opóźnieniem),
- dodatkowo: flagi saturacji, wibracje (syntetyczne), temperatury (model).

### Interfejs plant-kontroler

Minimalny interfejs:
```
Kontroler -> Plant: torque_cmd (float)
Plant -> Kontroler: omega_meas (float), status (flags)
```

Rozszerzony interfejs (dla diagnostyki):
```
Kontroler -> Plant: 
  - torque_cmd (float)
  - mode (enum: speed/torque/position)
  - parameters (np. Kp, Ki)

Plant -> Kontroler:
  - omega_meas (float)
  - torque_meas (float)
  - temperature (float)
  - status flags (saturated, error, etc.)
  - diagnostic (vibrations, etc.)
```

Kluczowe: **timestampy**. Każdy pomiar ma czas. To pozwala korelować dane i mierzyć opóźnienia.

## Co musi umieć harness
- wstrzykiwanie opóźnień i jitteru,
- wstrzykiwanie dropoutów,
- przełączanie trybów pracy (FSM),
- logowanie spójne dla online/offline,
- deterministyczny replay scenariuszy (regresja).

### Kluczowe cechy harnessu

**1. Wstrzykiwanie opóźnień i jitteru**
Harness musi symulować realny świat:
- Stałe opóźnienie (np. 1ms)
- Jitter (np. ±100 μs)
- Możliwość scenariuszy "worst case"

**2. Wstrzykiwanie dropoutów**
Symulacja utraty pakietów:
- Losowe dropouty (np. 0.1%)
- Burst dropouty (seria błędów)
- Możliwość testowania watchdogów

**3. Przełączanie trybów (FSM)**
Plant musi obsługiwać różne tryby:
- Normalna praca
- Rozruch
- Zatrzymanie
- Awaria

**4. Logowanie**
Każdy test musi być zalogowany:
- Format: JSON, CSV, protobuf
- Zawiera: timestampy, dane wejście-wyjście, metryki
- Pozwala na offline analysis

**5. Replay**
Możliwość odtworzenia:
- Tego samego scenariusza
- Z tymi samymi parametrami
- Porównanie "before/after"

### Przemówienie Profesora

Najlepsze harnessy, które widziałem, miały jedną cechę: **pozwalały wygenerować problem, którego normalnie byś nie zobaczył**.

Np. "a co jeśli EtherCAT straci 5 pakietów z rzędu?" - normalnie rzadki scenariusz, ale jak się zdarzy, system musi przeżyć.

Harness "trybu okrutnego" - generuje rzadkie scenariusze na żądanie. To pozwala przetestować ścieżki awaryjne, zanim awaria naprawdę nastąpi.

## Checklisty
- Jest jeden format logu używany w całym labie.
- Jest zestaw scenariuszy regresji (skok zakłócenia, saturacja, dropout, opóźnienie).
- Jest mechanizm "replay" i porównywania metryk.

### Checklist szczegółowy

**Format logów:**
- [ ] Zdefiniowany schemat (JSON/protobuf)
- [ ] Timestamp w każdym wpisie
- [ ] Identyfikator scenariusza
- [ ] Identyfikator wersji oprogramowania

**Scenariusze regresji:**
- [ ] Skok zadania prędkości
- [ ] Skok obciążenia
- [ ] Saturacja momentu
- [ ] Dropout komunikacji (single)
- [ ] Burst dropout
- [ ] Zwiększone opóźnienie
- [ ] Zwiększony jitter

**Replay:**
- [ ] Możliwość zapisu scenariusza
- [ ] Możliwość odtworzenia scenariusza
- [ ] Porównanie metryk "before/after"

**Metryki:**
- [ ] Błąd regulacji (max, RMS)
- [ ] Czas ustalania
- [ ] Przeregulowanie
- [ ] Ilość saturacji

## Slajdy (tekstowe)
### Slajd 1: Idea harness
- kontroler steruje „jakby” hardware
- PC odgrywa plant

### Slajd 2: Co wstrzykujemy
- opóźnienie, jitter, dropouty
- szum i saturacje

## Pytania do studentów
1. Jak zaprojektujesz harness tak, aby kontroler „nie wiedział”, że steruje modelem?
2. Jakie scenariusze powinny być częścią regresji (zawsze), a jakie tylko testami eksploracyjnymi?
3. Jak wymusisz deterministyczny replay (seed, wersje schematu, snapshoty parametrów)?
4. Jak rozdzielisz role: sterowanie (RT) vs diagnostyka vs logowanie, aby nie mieszać odpowiedzialności?

## Projekty studenckie
- „Scenario runner”: uruchamianie scenariuszy z plików (opóźnienia, dropouty, zakłócenia) + raport metryk.
- „Replay engine”: odtwarzanie logów w trybie offline i porównanie metryk „przed/po” zmianie regulatora.
- „Contract validator”: walidator ramek danych (wymagane pola, wersja schematu, spójność timestampów).

## BONUS
- Najlepsze harnessy mają „tryb okrutny”: potrafią generować rzadkie, ale realistyczne zdarzenia (piki opóźnień, dropouty) i wymuszać odporność systemu.
