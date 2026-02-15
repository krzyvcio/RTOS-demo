# Laboratorium 10: Projekt Końcowy

**Czas:** 4 godziny (2 zajęcia + praca własna)
**Punkty:** 50 pkt

---

## Cel projektu

Zaprojektować i zaimplementować kompletny system RTOS realizujący konkretne zadanie sterowania lub monitoringu.

---

## Tematy projektów do wyboru

### Projekt A: Sterownik silnika krokowego

**Opis:** System sterujący silnikiem krokowym z precyzyjną kontrolą prędkości i pozycji.

**Wymagania:**
- Generowanie impulsów sterujących
- Kontrola prędkości (profil ramp)
- Pozycjonowanie absolutne i względne
- Obsługa limit switches (przerwania)
- Komenda przez UART: `MOVE 1000`, `SPEED 500`, `STOP`

**Architektura sugerowana:**
```
ISR (limit switch) ──┐
                     ├──► [Control Task] ──► [Step Generator Task]
UART Task ───────────┘
```

---

### Projekt B: System monitorowania środowiska

**Opis:** System zbierający dane z czujników i wysyłający raporty.

**Wymagania:**
- Odczyt 3 czujników (temperatura, wilgotność, ciśnienie)
- Przetwarzanie i filtrowanie danych
- Wykrywanie anomalii
- Wyświetlanie na LCD (lub terminal)
- Zapis na SD card (lub log w pamięci)
- Komunikacja przez UART/I2C

**Architektura sugerowana:**
```
[Sensor Tasks] ──► [Processing Task] ──► [Display Task]
                            │
                            └──► [Logging Task]
```

---

### Projekt C: Robot mobilny - unikanie przeszkód

**Opis:** Prosty robot mobilny reagujący na przeszkody.

**Wymagania:**
- Odczyt czujników odległości (ultradźwiękowe/IR)
- Sterowanie silnikami DC
- Algorytm unikania przeszkód
- Tryby: auto, manual (UART)
- Telemetria przez UART

**Architektura sugerowana:**
```
[Sensor Task] ──► [Decision Task] ──► [Motor Control Task]
       │                                         │
       └─────────────────────────────────────────┘
                       (feedback)
```

---

### Projekt D: System akwizycji danych

**Opis:** System zbierający dane z ADC i przetwarzający w czasie rzeczywistym.

**Wymagania:**
- Próbkowanie ADC z określoną częstotliwością
- Filtrowanie cyfrowe (np. średnia krocząca)
- Wykrywanie progów (alarmy)
- Buforowanie danych
- Eksport danych przez UART

**Architektura sugerowana:**
```
ISR (ADC) ──► [Buffer] ──► [Processing Task] ──► [Output Task]
                               │
                               └──► [Alarm Task]
```

---

### Projekt E: Termostat cyfrowy

**Opis:** Inteligentny termostat z PID control.

**Wymagania:**
- Pomiar temperatury
- Sterowanie grzałką/chłodzeniem (PWM)
- Algorytm PID
- Harmonogramy (program czasowy)
- Interfejs użytkownika (przyciski + wyświetlacz)

**Architektura sugerowana:**
```
[Sensor Task] ──► [PID Task] ──► [PWM Task]
                            │
[Schedule Task] ────────────┘
[UI Task] ──────────────────┘
```

---

### Projekt F: Własny pomysł

Student może zaproponować własny temat.

**Wymagania:**
- Min. 3 zadania współpracujące
- Synchronizacja (mutex/semafor)
- Komunikacja (kolejki)
- Obsługa przerwań (lub symulacja)
- Timing (periodyczne zadania)

---

## Struktura projektu

```
projekt/
├── src/
│   ├── main.c           # Inicjalizacja, tworzenie zadań
│   ├── config.h         # Konfiguracja systemu
│   ├── tasks/
│   │   ├── task_a.c
│   │   ├── task_b.c
│   │   └── ...
│   ├── drivers/
│   │   ├── uart.c
│   │   ├── sensor.c
│   │   └── ...
│   └── FreeRTOSConfig.h
├── docs/
│   ├── architecture.md  # Diagram architektury
│   ├── api.md           # Dokumentacja API
│   └── testing.md       # Plan testów
├── tests/
│   └── test_*.c         # Testy jednostkowe
├── Makefile
└── README.md
```

---

## Wymagania techniczne

### Obowiązkowe

1. **Min. 3 zadania** współpracujące ze sobą
2. **Synchronizacja** - użycie mutex lub semafor
3. **Komunikacja** - użycie kolejek lub task notifications
4. **Timing** - min. 1 zadanie periodyczne
5. **Obsługa błędów** - timeout, sprawdzanie wyników
6. **Dokumentacja** - komentarze w kodzie

### Opcjonalne (+punkty)

1. **ISR** - obsługa przerwań (lub symulacja)
2. **Monitoring** - task statystyk systemu
3. **Watchdog** - wykrywanie zawieszeń
4. **CLI** - prosty interfejs komendowy
5. **Logging** - system logów

---

## Dokumentacja projektu

### 1. Dokumentacja architektury (docs/architecture.md)

```markdown
# Architektura systemu [Nazwa projektu]

## Opis ogólny
[Krótki opis co system robi]

## Diagram zadań
[Diagram lub ASCII art]

## Przepływ danych
[Opis jak dane płyną przez system]

## Synchronizacja
[Jakie zasoby są chronione, jakimi mechanizmami]

## Timing
[Okresy zadań, deadline]
```

### 2. Dokumentacja API (docs/api.md)

```markdown
# API Systemu

## Zadania

### TaskA - Nazwa zadania
- **Cel:** [opis]
- **Priorytet:** [wartość]
- **Okres:** [wartość]
- **Input:** [źródła danych]
- **Output:** [dokąd wysyła]

### Funkcje

#### `void function_name(params)`
- **Opis:** [co robi]
- **Parametry:** [lista]
- **Zwraca:** [wartość]
```

### 3. Plan testów (docs/testing.md)

```markdown
# Plan testów

## Testy jednostkowe
- [ ] Test TaskA w izolacji
- [ ] Test synchronizacji
- [ ] Test kolejek

## Testy integracyjne
- [ ] Test TaskA + TaskB
- [ ] Test całego systemu

## Testy obciążeniowe
- [ ] Test przy maksymalnym obciążeniu
- [ ] Test długotrwały (24h)

## Scenariusze awarii
- [ ] Co gdy task się zawiesi?
- [ ] Co gdy kolejka przepełni się?
```

---

## Prezentacja projektu

### Struktura (10-15 min)

1. **Wprowadzenie** (2 min)
   - Problem do rozwiązania
   - Cele projektu

2. **Architektura** (3 min)
   - Diagram zadań
   - Przepływ danych
   - Synchronizacja

3. **Implementacja** (3 min)
   - Kluczowe fragmenty kodu
   - Wyzwania napotkane

4. **Demonstracja** (4 min)
   - Działający system
   - Testy

5. **Wnioski** (2 min)
   - Co działa
   - Co można poprawić
   - Wnioski dla RTOS

---

## Kryteria oceny

| Kryterium | Punkty |
|-----------|--------|
| **Funkcjonalność** - system działa zgodnie z wymaganiami | 10 |
| **Architektura** - poprawny podział na zadania | 8 |
| **Synchronizacja** - poprawne użycie mutex/semafor | 8 |
| **Timing** - periodyczne zadania, deadline dotrzymane | 6 |
| **Kod** - czytelność, komentarze, obsługa błędów | 6 |
| **Dokumentacja** - kompletna i jasna | 6 |
| **Prezentacja** - jasność wywodu, demonstracja | 4 |
| **Bonus** - funkcje dodatkowe | 2 |

**Razem:** 50 pkt

---

## Harmonogram

| Tydzień | Zadanie |
|---------|---------|
| 1 | Wybór tematu, projekt architektury |
| 2 | Implementacja podstawowa (szkielet) |
| 3 | Implementacja funkcjonalna |
| 4 | Testy, debugging |
| 5 | Dokumentacja, przygotowanie prezentacji |
| 6 | Prezentacja, oddanie projektu |

---

## Wskazówki

### Planowanie

1. Zacznij od prostego prototypu
2. Dodawaj funkcje stopniowo
3. Testuj po każdej zmianie
4. Dokumentuj na bieżąco

### Debugging

1. Używaj printf do śledzenia przepływu
2. Monitoruj stack usage
3. Sprawdzaj timing (tick count)
4. Testuj edge cases

### Optymalizacja

1. Nie optymalizuj za wcześnie
2. Mierz przed optymalizacją
3. Skup się na ścieżkach krytycznych

---

## Checklist przed oddaniem

- [ ] Kod się kompiluje bez warningów
- [ ] System działa zgodnie z wymaganiami
- [ ] Synchronizacja jest poprawna (brak race condition)
- [ ] Obsługa błędów jest zaimplementowana
- [ ] Dokumentacja jest kompletna
- [ ] Prezentacja jest gotowa
- [ ] README.md zawiera instrukcję uruchomienia
- [ ] Kod jest sformatowany i skomentowany

---

## Pytania kontrolne (obrona)

1. Dlaczego ta architektura? Jakie alternatywy rozważałeś?
2. Jakie mechanizmy synchronizacji użyłeś i dlaczego?
3. Jakie są czasy wykonania zadań? Czy deadline są dotrzymane?
4. Co się stanie gdy... (scenariusz awarii)?
5. Co byś zmienił, gdybyś miał więcej czasu?