# Laboratorium RTOS - Plan zajęć

## Przedmiot: Systemy Operacyjne Czasu Rzeczywistego

**Cel przedmiotu:** Zdobycie praktycznych umiejętności projektowania, implementacji i debugowania systemów czasu rzeczywistego z wykorzystaniem RTOS.

______________________________________________________________________

## Plan laboratoriów

| Lab | Temat | Czas | Punkty |
|-----|-------|------|--------|
| 1 | Wprowadzenie do środowiska i symulatora | 2h | 10 |
| 2 | Zadania i scheduler - priorytety, stany | 2h | 15 |
| 3 | Synchronizacja - mutexy, semafory | 2h | 20 |
| 4 | Przerwania i obsługa ISR | 2h | 15 |
| 5 | Komunikacja między zadaniami - kolejki | 2h | 15 |
| 6 | Timing, timery, delays | 2h | 10 |
| 7 | Priority Inversion - problem i rozwiązania | 2h | 15 |
| 8 | Debugging i tracing | 2h | 10 |
| 9 | Case Study - system sterowania | 2h | 20 |
| 10 | Projekt końcowy | 4h | 50 |

**Razem:** 20 godzin zajęć, 180 punktów

______________________________________________________________________

## Wymagania

### Software

- **FreeRTOS** (symulator Linux lub STM32)
- **SimAVR** lub **QEMU** (dla AVR/ARM)
- **STM32CubeIDE** (opcjonalnie, dla hardware)
- **GDB** z Python extensions
- **VS Code** z C/C++ extension

### Hardware (opcjonalnie)

- STM32 Nucleo lub Discovery board
- STM32F4 lub STM32L4
- Logic analyzer (Saleae)
- Oscilloscope (opcjonalnie)

### Wiedza wstępna

- Język C (wskaźniki, struktury)
- Podstawy architektury komputerów
- Podstawy systemów operacyjnych

______________________________________________________________________

## Zasady zaliczenia

### Punkty

- Laboratoria: 180 pkt
- Projekt końcowy: 50 pkt
- **Razem:** 230 pkt

### Oceny

| Punkty | Ocena |
|--------|-------|
| 200-230 | 5.0 |
| 180-199 | 4.5 |
| 160-179 | 4.0 |
| 140-159 | 3.5 |
| 120-139 | 3.0 |
| \<120 | 2.0 |

### Warunki zaliczenia

1. Obecność na wszystkich laboratoriach
1. Oddanie sprawozdania z każdego labu (deadline: +7 dni)
1. Zaliczenie projektu końcowego
1. Min. 120 punktów

______________________________________________________________________

## Struktura sprawozdania

```markdown
# Sprawozdanie Lab X

## Imię i nazwisko
## Data
## Grupa

## 1. Cel ćwiczenia
## 2. Zadania wykonane
## 3. Kod źródłowy (kluczowe fragmenty)
## 4. Wyniki (zrzuty ekranu, wykresy)
## 5. Wnioski
## 6. Problemy napotkane i rozwiązania
```

______________________________________________________________________

## Literatura

1. **FreeRTOS Documentation** - https://www.freertos.org/Documentation/
1. **Mastering the FreeRTOS Real Time Kernel** - R. Barry
1. **Real-Time Systems** - J. W. S. Liu
1. **Hard Real-Time Computing Systems** - G. Buttazzo
1. **Making Embedded Systems** - E. White

______________________________________________________________________

## Kontakty

**Prowadzący:** [imię nazwisko]
**Email:** [email]
**Konsultacje:** [dzień godzina]

**Repository:** [git repo URL]

______________________________________________________________________

## Harmonogram

```
Tydzień 1:  Lab 1 - Wprowadzenie
Tydzień 2:  Lab 2 - Zadania i scheduler
Tydzień 3:  Lab 3 - Synchronizacja (mutexy)
Tydzień 4:  Lab 4 - Przerwania
Tydzień 5:  Lab 5 - Komunikacja (kolejki)
Tydzień 6:  Lab 6 - Timing
Tydzień 7:  Lab 7 - Priority Inversion
Tydzień 8:  Lab 8 - Debugging
Tydzień 9:  Lab 9 - Case Study
Tydzień 10: Lab 10 - Prezentacja projektów
```

______________________________________________________________________

## Skróty używane w instrukcjach

| Skrót | Opis |
|-------|------|
| **TODO** | Zadanie do wykonania |
| **HINT** | Wskazówka pomocnicza |
| **WARNING** | Uwaga - potencjalny problem |
| **BONUS** | Zadanie dodatkowe za extra punkty |
| **CHECK** | Punkt kontrolny - pokaż prowadzącemu |
