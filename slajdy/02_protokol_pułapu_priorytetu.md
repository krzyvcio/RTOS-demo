# Protokół Pułapu Priorytetu (Priority Ceiling Protocol - PCP)

## Definicja

Mechanizm synchronizacji, który zapobiega zarówno inwersji priorytetów, jak i zakleszczeniom (deadlock) poprzez dynamiczne podnoszenie priorytetu zadania do "pułapu" zdefiniowanego dla każdego zasobu.

______________________________________________________________________

## Diagram działania

```
┌───────────────────────────────────────────────────────────────┐
│                    PRIORITY CEILING PROTOCOL                  │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  MUTEX_A → CEILING = WYSOKI (priorytet 90)                   │
│  MUTEX_B → CEILING = ŚREDNI (priorytet 50)                   │
│  MUTEX_C → CEILING = NISKI (priorytet 10)                    │
│                                                               │
│  Zadanie NISKIE (priorytet 10) bierze MUTEX_A:               │
│  → Priorytet rośnie do 90 (ceiling MUTEX_A)                 │
│  → Żadne zadanie nie może go wywłascić!                      │
│  → Szybko kończy pracę z mutexem                             │
│  → Priorytet wraca do 10                                     │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Porównanie z PIP

| Cecha | Priority Inheritance (PIP) | Priority Ceiling (PCP) |
|-------|---------------------------|------------------------|
| Zapobieganie inwersji | ✅ Tak | ✅ Tak |
| Zapobieganie deadlock | ❌ Nie | ✅ Tak |
| Zapobieganie chain blocking | ❌ Nie | ✅ Tak |
| Wymaga analizy priorytetów | ❌ Nie | ✅ Tak (statyczna) |
| Overhead | Niski | Średni |
| Determinizm | Średni | Wysoki |

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Automatyka przemysłowa

| System | Wymaganie | Zastosowanie PCP |
|--------|-----------|------------------|
| Sterowniki PLC | Cykl < 1ms | Gwarantowany czas sekcji krytycznej |
| Roboty spawalnicze | Koordynacja 8+ osi | Brak deadlock przy wielu zasobach |
| Linie produkcyjne | Downtime < 0.1% | Deterministyczne blokowanie |

**Przykład:** Siemens SIMATIC S7-1500 - PCP dla komunikacji PROFINET

### Systemy motoryzacyjne

- **Sterownik silnika (ECU)** - cykl 10ms, wielokrotne zasoby
- **Skrzynia biegów** - koordynacja momentu obrotowego
- **ADAS** - fuzja danych z czujników

### Systemy medyczne

- **Respiratory** - kontrola przepływu tlenu
- **Pompy insulinowe** - podawanie leków
- **Monitorowanie pacjenta** - akwizycja danych

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Błędny ceiling** | Ustawienie zbyt niskiego pułapu | Analiza wszystkich zadań korzystających z zasobu |
| **Zbyt wysoki ceiling** | Niepotrzebne blokowanie innych zadań | Dokładna analiza priorytetów |
| **Złożoność konfiguracji** | Duża liczba zasobów = dużo konfiguracji | Narzędzia automatyczne (np. SymTA/S) |
| **Zmiana wymagań** | Nowe zadanie wymaga aktualizacji ceiling | Dokumentacja i traceability |

______________________________________________________________________

## Formalna specyfikacja

```
DEFINICJA CEILING:
  ceiling(S) = max{priorytet(τ) : τ korzysta z zasobu S}

ZASADY PCP:
  1. Zadanie τ może zablokować zasób S tylko jeśli:
     priorytet(τ) > current_ceiling

  2. Po zajęciu zasobu S:
     priorytet_aktualny(τ) = ceiling(S)

  3. Priorytet wraca do oryginalnego po zwolnieniu zasobu

WŁASNOŚCI:
  • Brak deadlock - zadania czekają w kolejce wg priorytetów
  • Brak chain blocking - maksymalnie jedno blokowanie
  • Deterministyczny blocking time = Σ(blokujących zadań × czas_sekcji)
```

______________________________________________________________________

## Implementacje

### QNX Neutrino

```c
#include <pthread.h>

pthread_mutexattr_t attr;
pthread_mutex_t mutex;

// Ustawienie Priority Ceiling
pthread_mutexattr_init(&attr);
pthread_mutexattr_setprotocol(&attr, PTHREAD_PRIO_PROTECT);
pthread_mutexattr_setprioceiling(&attr, 90);  // Ceiling = 90

pthread_mutex_init(&mutex, &attr);
```

### VxWorks

```c
#include <semLib.h>

SEM_ID sem = semMCreate(SEM_Q_PRIORITY | SEM_PRIORITY_CEILING);
semPrioritySet(sem, 90);  // Ustawienie ceiling
```

### OSEK/VDX (Automotive standard)

```c
// W pliku OIL (konfiguracja):
RESOURCE zasobA {
  RESOURCEPROPERTY = STANDARD;
  CEILING = 90;
};

// W kodzie C:
GetResource(zasobA);   // Priorytet rośnie do 90
// ... sekcja krytyczna ...
ReleaseResource(zasobA);  // Priorytet wraca
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Adaptive Priority Ceiling

- Dynamiczne dostosowanie ceiling na podstawie runtime
- AI/ML do optymalizacji priorytetów

### 2. Hierarchical Scheduling

- PCP w systemach z partycjonowaniem (ARINC 653)
- Wielopoziomowe priorytety

### 3. Mixed-Criticality PCP

- Różne ceiling dla różnych poziomów krytyczności
- Zapewnienie izolacji między zadaniami ASIL-A i ASIL-D

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Masz system z następującymi zadaniami:

  Zadanie A (priorytet 90) - korzysta z Mutex_X, Mutex_Y
  Zadanie B (priorytet 70) - korzysta z Mutex_Y
  Zadanie C (priorytet 50) - korzysta z Mutex_X, Mutex_Z
  Zadanie D (priorytet 30) - korzysta z Mutex_Z

PYTANIA:
1. Jaki powinien być ceiling dla każdego mutexa?
2. Czy może wystąpić deadlock?
3. Oblicz maksymalny blocking time dla zadania A.

ROZWIĄZANIE:
1. Ceiling:
   - Mutex_X: max(90, 50) = 90
   - Mutex_Y: max(90, 70) = 90
   - Mutex_Z: max(50, 30) = 50

2. Z PCP - NIE, bez PCP - TAK (A→X→C→Z→D→Y→A)
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego PCP zapobiega deadlock a PIP nie?
1. Jak obliczyć ceiling dla danego zasobu?
1. Co się stanie jeśli ceiling jest ustawiony za nisko?
1. Kiedy warto wybrać PIP zamiast PCP?
1. Jak PCP wpływa na analizę WCET?

______________________________________________________________________

## Literatura

1. Sha, Rajkumar, Lehoczky, "Priority Inheritance Protocols: An Approach to Real-Time Synchronization" (1990)
1. Buttazzo, "Hard Real-Time Computing Systems" - rozdział o synchronizacji
1. OSEK/VDX Specification - Priority Ceiling Protocol
1. AUTOSAR OS Specification - Resource Management
