# Inwersja Priorytetów (Priority Inversion)

## Definicja

Zjawisko, w którym zadanie o niższym priorytecie blokuje wykonanie zadań o wyższych priorytetach poprzez posiadanie zasobu (np. mutexa), na który zadanie wysokiego priorytetu czeka.

______________________________________________________________________

## Diagram problemu

```
CZAS →
┌─────────────────────────────────────────────────────────────────┐
│ PRIORYTET                                                       │
│   ↑                                                             │
│   │  WYSOKI ──────┐          ╔═════════════╗────────────────     │
│   │               │ czeka na │ SEKCJA      │                    │
│   │               │ mutex    │ KRYTYCZNA   │                    │
│   │  ŚREDNI ──────┼──────────│─────────────│─────────────────    │
│   │               │ ← blokuje! (bierze CPU)│                    │
│   │               │          │             │                    │
│   │  NISKI   ──╔══╧══════════╧═════════════╧══╗─────────────     │
│   │            ║ NISKI trzyma mutex           ║                 │
│   └────────────╨──────────────────────────────╨───────────────  │
└─────────────────────────────────────────────────────────────────┘
```

**Sekwencja zdarzeń:**

1. Zadanie NISKIE bierze mutex
1. Zadanie WYSOKIE chce ten sam mutex → blokuje się
1. Zadanie ŚREDNIE (nie potrzebuje mutexa) wywłaszcza NISKIE
1. WYSOKIE czeka na ŚREDNIE → **INWERSJA!**

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Branża motoryzacyjna (ASIL-D)

| System | Wymaganie czasowe | Ryzyko inwersji |
|--------|-------------------|-----------------|
| ABS/ESC | < 5ms reakcja | Blokada hamulców |
| Drive-by-Wire | < 10ms sterowanie | Utrata kontroli |
| Airbag | < 3ms odpalenie | Nieaktywna poduszka |
| EPS ( wspomaganie) | < 10ms odpowiedź | Utrata wspomagania |

### Robotyka przemysłowa

- **Koordynacja ramion robota** - 6+ osi muszą być zsynchronizowane
- **Emergency stop** - musi zadziałać natychmiast, niezależnie od obciążenia
- **Przykłady:** KUKA KR 1000, ABB IRB 7600, FANUC M-2000

### Lotnictwo (Fly-by-Wire)

- **Airbus A320** - pierwszy cywilny samolot z pełnym FBW
- **Boeing 777/787** - triplex redundancy z voting
- **Wymaganie:** Reakcja < 50ms na input pilota

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Konsekwencje |
|------------|------|--------------|
| **Nieograniczona inwersja** | Zadanie średniego priorytetu może wykonywać się nieskończenie długo | Przekroczenie deadline → awaria systemu |
| **Kaskadowa awaria** | Inwersja rozprzestrzenia się na inne zadania | Całkowite zablokowanie systemu |
| **Trudność reprodukcji** | Problem zależy od timing-u | Trudne testowanie i debugowanie |
| **Naruszenie WCET** | Przekroczenie pesymistycznego czasu wykonania | Odrzucenie certyfikacji |
| **Safety violation** | Zadanie krytyczne nie wykonuje się w czasie | Zagrożenie życia (ASIL-D) |

______________________________________________________________________

## Case Study: Mars Pathfinder (1997)

```
┌─────────────────────────────────────────────────────────────┐
│                    MARS PATHFINDER                          │
├─────────────────────────────────────────────────────────────┤
│ PROBLEM:                                                     │
│ • Rover resetował się co kilka dni na Marsie                │
│ • Przyczyna: Priority Inversion w VxWorks                   │
│ • Zadanie HIGH (bus management) czekało na mutex            │
│ • Mutex trzymany przez zadanie LOW (data collection)        │
│ • Zadania MED (communication) blokowały LOW                 │
│                                                              │
│ DIAGNOZA:                                                    │
│ • JPL wykryło problem przez telemetry                       │
│ • Analiza logów wykazała wzorzec inwersji                   │
│                                                              │
│ ROZWIĄZANIE:                                                 │
│ • Włączenie Priority Inheritance w VxWorks                  │
│ • Aktualizacja software → 40 minutowy upload na Marsa       │
│ • Działa do dzisiaj!                                        │
│                                                              │
│ LEKCJA:                                                      │
│ • Nawet NASA popełnia błędy w systemach RT                  │
│ • Testowanie w warunkach race condition jest kluczowe       │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Kierunki rozwoju i rozwiązania

### 1. Priority Inheritance Protocol (PIP)

```
NORMALNIE:
  NISKI trzyma mutex z priorytetem NISKI
  ↓
  ŚREDNI wywłaszcza NISKIEGO
  ↓
  WYSOKI czeka na mutex

Z PIP:
  NISKI trzyma mutex → DZIEDZICZY priorytet WYSOKI
  ↓
  ŚREDNI NIE MOŻE wywłaszczyć (ma niższy priorytet)
  ↓
  NISKI kończy szybko → WYSOKI dostaje mutex
```

**Implementacje:** ThreadX, VxWorks, QNX, FreeRTOS (configurable)

### 2. Priority Ceiling Protocol (PCP)

- Każdy mutex ma przypisany "pułap priorytetu" (ceiling)
- Zadanie wykonujące się na mutexie jest podnoszone do pułapu
- **Zalety:** Zapobiega deadlock i inwersji jednocześnie
- **Wady:** Wymaga statycznej analizy wszystkich zadań

### 3. Immediate Priority Ceiling

- Priorytet podnoszony PRZED zajęciem mutexa
- Jeszcze bardziej deterministyczny
- Stosowany w OSEK/VDX (automotive)

### 4. Struktury Lock-Free

```
Zamiast mutexa:
┌─────────────────────────────────────┐
│  Ring Buffer (atomic indexes)       │
│  - ISR pisze bez blokowania         │
│  - Task czyta bez blokowania        │
│  - Brak możliwości inwersji!        │
└─────────────────────────────────────┘
```

**Techniki:** CAS (Compare-And-Swap), LL/SC (Load-Linked/Store-Conditional)

______________________________________________________________________

## Narzędzia i standardy

| Narzędzie/Standard | Mechanizm | Branża |
|--------------------|-----------|--------|
| ThreadX Mutex | Priority Inheritance | Automotive, IoT |
| QNX Neutrino | PCP + adaptive scheduling | Automotive, Medical |
| VxWorks | Priority Inheritance | Aerospace |
| FreeRTOS | Configurable (configUSE_MUTEX) | IoT, Consumer |
| Zephyr RTOS | Priority Inheritance | IoT, Wearables |
| OSEK/VDX | Immediate Priority Ceiling | Automotive |
| AUTOSAR OS | Priority Ceiling | Automotive |

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego priority inversion jest trudna do wykrycia w standardowych testach?
1. Kiedy lepiej użyć lock-free queue zamiast mutexa z PIP?
1. Jak Priority Ceiling Protocol zapobiega deadlock?
1. Jaki jest związek między priority inversion a WCET?
1. Dlaczego Mars Pathfinder był trudny do zdiagnozowania zdalnie?

______________________________________________________________________

## Zadanie praktyczne: Kod w C

```c
// Zadanie: Znajdź błąd i napraw używając Priority Inheritance
#include <pthread.h>
#include <unistd.h>

pthread_mutex_t zasob = PTHREAD_MUTEX_INITIALIZER;

void* zadanie_niskie(void* arg) {
    pthread_mutex_lock(&zasob);
    sleep(10);  // Długa operacja I/O
    pthread_mutex_unlock(&zasob);
    return NULL;
}

void* zadanie_srednie(void* arg) {
    while(1) {
        // Praca obliczeniowa - nie potrzebuje zasobu!
        obliczenia();
    }
    return NULL;
}

void* zadanie_wysokie(void* arg) {
    pthread_mutex_lock(&zasob);  // Czeka na niskie!
    // Sekcja krytyczna - musi być szybka!
    pthread_mutex_unlock(&zasob);
    return NULL;
}

// ROZWIĄZANIE:
// pthread_mutexattr_t attr;
// pthread_mutexattr_setprotocol(&attr, PTHREAD_PRIO_INHERIT);
// pthread_mutex_init(&zasob, &attr);
```

______________________________________________________________________

## Literatura

1. Butenhof, "Programming with POSIX Threads" - rozdział o priorytetach
1. Sha, Rajkumar, Lehoczky, "Priority Inheritance Protocols" (1990)
1. NASA JPL, "Mars Pathfinder Priority Inversion" - case study
1. ISO 26262-6, Part 6: Software safety requirements
