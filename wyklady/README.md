# Wykłady RTOS - Plan semestralny

## Przedmiot: Systemy Operacyjne Czasu Rzeczywistego

**Liczba godzin:** 30h (15 wykładów × 2h)
**ECTS:** 6

______________________________________________________________________

## Plan wykładów

| Nr | Temat | Laboratorium | Tydzień |
|----|-------|--------------|---------|
| 1 | Wprowadzenie do RTOS | Lab 1 | 1 |
| 2 | Architektura RTOS | Lab 1 | 2 |
| 3 | Zadania i stany | Lab 2 | 3 |
| 4 | Scheduler i priorytety | Lab 2 | 4 |
| 5 | Synchronizacja - Mutex | Lab 3 | 5 |
| 6 | Synchronizacja - Semafor | Lab 3 | 6 |
| 7 | Przerwania i ISR | Lab 4 | 7 |
| 8 | Komunikacja - kolejki | Lab 5 | 8 |
| 9 | Timing i timery | Lab 6 | 9 |
| 10 | Scheduling algorithms (RMS/EDF) | Lab 6 | 10 |
| 11 | Priority Inversion | Lab 7 | 11 |
| 12 | Memory management | Lab 8 | 12 |
| 13 | Debugging i tracing | Lab 8 | 13 |
| 14 | Case Studies (Lotnictwo, Automotive) | Lab 9 | 14 |
| 15 | Projekt końcowy - prezentacje | Lab 10 | 15 |

______________________________________________________________________

## Cele kształcenia

Po ukończeniu przedmiotu student potrafi:

1. **Projektować** systemy czasu rzeczywistego z określonymi wymaganiami timing
1. **Implementować** wielozadaniowe systemy na RTOS
1. **Analizować** schedulability i determinizm systemu
1. **Rozwiązywać** problemy synchronizacji i komunikacji
1. **Debugować** i optymalizować systemy RT
1. **Dowodzić** poprawności timing (WCET, schedulability)

______________________________________________________________________

## Metody oceny

| Składnik | Waga |
|----------|------|
| Egzamin pisemny | 40% |
| Laboratoria | 35% |
| Projekt końcowy | 25% |

______________________________________________________________________

## Literatura obowiązkowa

1. **Mastering the FreeRTOS Real Time Kernel** - Richard Barry
1. **Real-Time Systems** - Jane W. S. Liu
1. **Hard Real-Time Computing Systems** - Giorgio Buttazzo

## Literatura uzupełniająca

1. **Making Embedded Systems** - Elecia White
1. **The Art of Multiprocessor Programming** - Herlihy & Shavit
1. **DO-178C/ISO 26262 Documentation**

______________________________________________________________________

## Zasady zaliczenia wykładów

### Obecność

- Obowiązkowa obecność na wykładach
- Max. 3 nieobecności usprawiedliwione

### Egzamin

- Część teoretyczna: definicje, algorytmy
- Część praktyczna: analiza schedulability, kody
- Część problemowa: case studies

### Przykładowe pytania egzaminacyjne

- Wyjaśnij różnicę między preemptive a cooperative scheduling
- Oblicz schedulability dla systemu z podanymi parametrami
- Opisz problem priority inversion i sposoby jego rozwiązania
- Porównaj RMS i EDF - zalety i wady
- Co to jest WCET i jak się je oblicza?
