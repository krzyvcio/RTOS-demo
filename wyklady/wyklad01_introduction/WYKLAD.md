# Wykład 1: Wprowadzenie do systemów czasu rzeczywistego

**Czas:** 2 godziny (90 min)
**Tydzień:** 1

---

## Plan wykładu (90 min)

1. **Wprowadzenie** (10 min) - Czym jest RTOS?
2. **Historia** (10 min) - Od systemów embedded do RTOS
3. **Klasyfikacja systemów RT** (15 min) - Hard, Firm, Soft
4. **Wymagania RT vs zwykłe OS** (15 min) - Determinizm, predictability
5. **Architektura RTOS** (20 min) - Kernel, tasks, scheduler
6. **FreeRTOS overview** (15 min) - Struktura, API
7. **Podsumowanie** (5 min)

---

## Slajd 1: Tytuł

```
SYSTEMY OPERACYJNE CZASU RZECZYWISTEGO
Wykład 1: Wprowadzenie

Semestr: zimowy 2024/2025
Prowadzący: [imię nazwisko]
```

---

## Slajd 2: Cele wykładu

```
Po tym wykładzie będziecie potrafili:

✓ Zdefiniować system czasu rzeczywistego
✓ Rozróżnić hard, firm i soft real-time
✓ Wyjaśnić różnice między RTOS a GPOS
✓ Opisać podstawową architekturę RTOS
✓ Zrozumieć strukturę FreeRTOS
```

---

## Slajd 3: Czym jest system RT?

### Definicja

> **System czasu rzeczywistego** to system, w którym poprawność działania zależy nie tylko od logicznego wyniku, ale również od czasu jego uzyskania.

```
TRADYCYJNY SYSTEM:
- Pytanie: Czy wynik jest poprawny?
- Odpowiedź: Tak/Nie

SYSTEM RT:
- Pytanie: Czy wynik jest poprawny I został uzyskany w czasie?
- Odpowiedź: Tak/Nie + "W jakim czasie?"
```

---

## Slajd 4: Niespodzianka!

```
RTOS ≠ FAST

RTOS = PREDICTABLE

Szybkość to NIE jest definicja czasu rzeczywistego.
Determinizm JEST definicją.
```

**Przykład:**
```
System A: Średni czas odpowiedzi = 1ms, Max = 100ms
System B: Średni czas odpowiedzi = 5ms, Max = 6ms

System B jest LEPSZY dla RT, mimo że wolniejszy!
Dlaczego? Bo PREDICTABLE.
```

---

## Slajd 5: Klasyfikacja systemów RT

### Hard Real-Time

```
Przekroczenie deadline = KATASTROFA

Przykłady:
- Airbag (30ms deadline)
- Sterownik silnika
- Hamulce ABS
- System lotniczy

Kiedy deadline miss:
→ Uraz/śmierć
→ Zniszczenie sprzętu
→ Straty materialne
```

### Firm Real-Time

```
Przekroczenie deadline = UTRATA JAKOŚCI

Przykłady:
- Streaming video
- Audio processing
- Telecom

Kiedy deadline miss:
→ Przerwy w obrazie/dźwięku
→ Spadek jakości
→ Użytkownik niezadowolony
```

### Soft Real-Time

```
Przekroczenie deadline = SPADeK WYDAJNOŚCI

Przykłady:
- Interaktywne aplikacje
- Logowanie
- Raportowanie

Kiedy deadline miss:
→ Opóźnienia
→ Gorsze UX
→ Ale system działa
```

---

## Slajd 6: RTOS vs GPOS

| Cecha | RTOS | GPOS (Linux/Windows) |
|-------|------|----------------------|
| Cel | Determinizm | Throughput |
| Scheduling | Priority-based | Fairness |
| Preemption | Sterowane | Agresywne |
| Interrupt latency | Gwarantowane | Zmienne |
| Memory | Statyczne | Dynamiczne |
| Boot time | ms-s | s-min |
| Size | KB-MB | GB |

---

## Slajd 7: Dlaczego Linux NIE jest RTOS?

```
Linux (bez patchy RT):

- Preemptive, ale z wieloma sekcjami niepreemptowalnymi
- Tick rate 100-1000 Hz (zmienny)
- Dynamic scheduling (CFS)
- Brak gwarancji latencji
- Swapping, paging
- Network stack w kernel space

A typical "fast" operation może zająć 100ms
gdy system robi coś innego.
```

**Rozwiązanie:** PREEMPT_RT patch + tuning
- Ale to nadal nie jest "hard RT"
- Lepsze response time, ale bez gwarancji

---

## Slajd 8: Architektura RTOS

```
┌─────────────────────────────────────────────────────────┐
│                    APPLICATION                          │
│              (User Tasks)                               │
├─────────────────────────────────────────────────────────┤
│                      RTOS KERNEL                        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │Scheduler │ │  Task    │ │   IPC    │ │  Timer   │  │
│  │          │ │ Manager  │ │  (Queue) │ │ Manager  │  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
├─────────────────────────────────────────────────────────┤
│                  HARDWARE ABSTRACTION                   │
│                    (Portable Layer)                     │
├─────────────────────────────────────────────────────────┤
│                      HARDWARE                           │
│               (CPU, Memory, Peripherals)               │
└─────────────────────────────────────────────────────────┘
```

---

## Slajd 9: Scheduler - Serce RTOS

```
SCHEDULER decyduje:
- Które zadanie wykonuje się TERAZ
- Kiedy przełączyć kontekst
- Jakie zadanie jest następne

Typy:
1. PREEMPTIVE - Kernel może przerwać zadanie
2. COOPERATIVE - Zadanie samo oddaje CPU
3. HYBRID - Kombinacja

RTOS używa PREEMPTIVE + PRIORITIES
```

---

## Slajd 10: FreeRTOS - Overview

```
FreeRTOS:
- Najpopularniejszy RTOS na świecie
- Open source (MIT license)
- Mały footprint: 5-10KB
- Przenośny: 40+ architektur

Struktura:
├── tasks.c       - Task management, scheduler
├── queue.c       - Queues, semaphores, mutexes
├── timers.c      - Software timers
├── list.c        - Linked lists (internal)
├── portable/     - Architecture-specific code
│   ├── GCC/
│   │   └── ARM_CM4/  - Cortex-M4 port
│   └── ...
└── FreeRTOSConfig.h - Configuration
```

---

## Slajd 11: FreeRTOS Configuration

```c
// FreeRTOSConfig.h - Kluczowe parametry

// Scheduling
#define configUSE_PREEMPTION            1
#define configTICK_RATE_HZ             1000

// Memory
#define configMINIMAL_STACK_SIZE       128
#define configTOTAL_HEAP_SIZE         65536

// Priorities
#define configMAX_PRIORITIES            5

// Features
#define configUSE_MUTEXES               1
#define configUSE_COUNTING_SEMAPHORES   1
#define configUSE_QUEUE_SETS            1
```

---

## Slajd 12: Pierwszy program

```c
#include "FreeRTOS.h"
#include "task.h"

void vTask1(void *pvParameters) {
    while (1) {
        // Kod zadania
        do_something();
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

int main(void) {
    xTaskCreate(vTask1, "Task1", 128, NULL, 1, NULL);
    vTaskStartScheduler();

    // Nigdy tu nie dojdziemy
    for (;;);
    return 0;
}
```

---

## Slajd 13: Kluczowe pojęcia

```
TASK - Jednostka wykonawcza z własnym stosem

PRIORITY - Liczba określająca ważność zadania
           Wyższy numer = wyższy priorytet (FreeRTOS)

CONTEXT SWITCH - Zapisanie/odtworzenie stanu zadania

PREEMPTION - Przymusowe przerwanie zadania

TICK - Cykliczny interrupt od timera (scheduler)

BLOCKING - Zadanie czeka na zdarzenie
```

---

## Slajd 14: Zadania domowe

```
1. Przeczytać: FreeRTOS Quick Start Guide
2. Zainstalować: FreeRTOS simulator (Linux)
3. Uruchomić: "Hello World" task
4. Przygotować: Pytania na następny wykład

Na następny wykład:
- Jak scheduler wybiera zadanie?
- Co się dzieje przy tick interrupt?
- Jak działa context switch?
```

---

## Slajd 15: Podsumowanie

```
KLUCZOWE WNIOSKI:

1. RTOS ≠ Szybki, RTOS = Przewidywalny
2. Hard RT > Firm RT > Soft RT (krytyczność)
3. Determinizm jest ważniejszy niż throughput
4. Scheduler to serce RTOS
5. FreeRTOS: prosty, popularny, przenośny

NASTĘPNY WYKŁAD:
Architektura RTOS - Taski, stany, scheduler
```

---

## Notatki dla prowadzącego

### Czasomierz

| Sekcja | Plan | Rzeczywisty |
|--------|------|-------------|
| Wprowadzenie | 10 min | |
| Historia | 10 min | |
| Klasyfikacja | 15 min | |
| RT vs GPOS | 15 min | |
| Architektura | 20 min | |
| FreeRTOS | 15 min | |
| Podsumowanie | 5 min | |
| **RAZEM** | **90 min** | |

### Aktywności

1. **Pytanie otwierające:** "Kto używał Linuxa? Czy jest RT?"
2. **Demonstracja:** Pokazać różnicę response time (Linux vs RTOS sim)
3. **Dyskusja:** Przykłady hard RT w życiu codziennym

### Materiały dodatkowe

- FreeRTOS demo project (projektor)
- Grafika porównawcza RT vs GPOS
- Video: airbag deployment timing

---

## Pytania egzaminacyjne z tego wykładu

1. Zdefiniuj system czasu rzeczywistego.
2. Podaj różnicę między hard, firm i soft real-time.
3. Dlaczego Linux nie jest systemem czasu rzeczywistego?
4. Opisz architekturę RTOS.
5. Czym różni się RTOS od GPOS?