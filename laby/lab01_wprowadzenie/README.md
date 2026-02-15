# Laboratorium 1: Wprowadzenie do środowiska i symulatora

**Czas:** 2 godziny
**Punkty:** 10 pkt

---

## Cel ćwiczenia

1. Zapoznanie ze środowiskiem deweloperskim
2. Konfiguracja symulatora FreeRTOS
3. Uruchomienie pierwszego programu
4. Zrozumienie struktury projektu RTOS

---

## Wymagania wstępne

- Zainstalowany GCC (lub clang)
- Zainstalowany Make
- Podstawowa znajomość C

---

## Teoria (15 min)

### Czym jest RTOS?

RTOS (Real-Time Operating System) to system operacyjny zapewniający:
- **Determinizm** - gwarantowane czasy odpowiedzi
- **Wielozadaniowość** - wiele zadań wykonywanych "równolegle"
- **Priorytety** - ważniejsze zadania wykonują się pierwsze
- **Synchronizację** - bezpieczna komunikacja między zadaniami

### FreeRTOS

FreeRTOS to darmowy, open-source RTOS dla mikrokontrolerów:
- Mały footprint (5-10KB)
- Przenośny (ARM, AVR, ESP32, RISC-V...)
- Popularny (milliony pobrań rocznie)

---

## Zadanie 1: Konfiguracja środowiska (30 min)

### 1.1 Pobranie symulatora

```bash
# Utwórz katalog projektu
mkdir -p ~/rtos-lab
cd ~/rtos-lab

# Pobierz FreeRTOS
git clone https://github.com/FreeRTOS/FreeRTOS.git --recurse-submodules

# Lub użyj gotowego symulatora Linux:
git clone https://github.com/cjlano/freertos-simulator.git
```

### 1.2 Struktura katalogów

```
freertos-simulator/
├── src/
│   ├── main.c           # Twój kod
│   └── FreeRTOSConfig.h # Konfiguracja RTOS
├── FreeRTOS/
│   ├── tasks.c          # Scheduler
│   ├── queue.c          # Kolejki
│   ├── semphr.c         # Semafor
│   └── ...
├── portable/
│   └── GCC/             # Port dla Linux
└── Makefile
```

### 1.3 Sprawdź konfigurację

Otwórz `FreeRTOSConfig.h`:

```c
// Kluczowe parametry
#define configUSE_PREEMPTION            1   // Preemptive scheduling
#define configTICK_RATE_HZ             1000 // 1000 Hz tick
#define configMINIMAL_STACK_SIZE       128  // Minimalny stos
#define configMAX_PRIORITIES           5    // Liczba priorytetów
#define configUSE_MUTEXES              1    // Włącz mutexy
#define configUSE_COUNTING_SEMAPHORES  1    // Włącz semafory
```

**TODO:** Zmień `configTICK_RATE_HZ` na 100 Hz. Jaki będzie okres ticka?

---

## Zadanie 2: Pierwszy program (30 min)

### 2.1 Uruchomienie demo

```bash
cd freertos-simulator
make
./main
```

Powinieneś zobaczyć:
```
Task 1 is running
Task 2 is running
Task 1 is running
Task 2 is running
...
```

### 2.2 Analiza kodu

Otwórz `main.c`:

```c
#include "FreeRTOS.h"
#include "task.h"
#include <stdio.h>

// Definicja zadania
void vTask1(void *pvParameters) {
    while (1) {
        printf("Task 1 is running\n");
        vTaskDelay(pdMS_TO_TICKS(1000));  // Czekaj 1000ms
    }
}

void vTask2(void *pvParameters) {
    while (1) {
        printf("Task 2 is running\n");
        vTaskDelay(pdMS_TO_TICKS(500));   // Czekaj 500ms
    }
}

int main(void) {
    // Utwórz zadania
    xTaskCreate(vTask1, "Task1", 128, NULL, 1, NULL);
    xTaskCreate(vTask2, "Task2", 128, NULL, 1, NULL);

    // Uruchom scheduler
    vTaskStartScheduler();

    // Nigdy tu nie dojdziemy
    for (;;);
    return 0;
}
```

### 2.3 Pytania do analizy

1. Dlaczego `vTaskDelay()` jest potrzebne?
2. Co się stanie, jeśli usuniesz delay?
3. Co oznacza parametr `1` w `xTaskCreate`?
4. Dlaczego funkcje zadań mają `while(1)`?

**TODO:** Odpowiedz na pytania w sprawozdaniu.

---

## Zadanie 3: Dodanie nowego zadania (20 min)

### 3.1 TODO - Dodaj zadanie Task3

Stwórz nowe zadanie, które:
- Wypisuje "Task 3: running" co 250ms
- Ma priorytet 2 (wyższy niż Task 1 i 2)

```c
// Miejsce na Twój kod
void vTask3(void *pvParameters) {
    // TODO: Implementacja
}

// W main():
// TODO: Utwórz zadanie
```

### 3.2 Obserwuj zachowanie

**Pytania:**
1. Jak często pojawia się "Task 3"?
2. Czy Task 3 przejmuje CPU od innych zadań?
3. Co się stanie, jeśli Task 3 nie ma `vTaskDelay`?

---

## Zadanie 4: Priorytety (20 min)

### 4.1 Eksperyment z priorytetami

Zmień priorytety:
- Task 1: priorytet 1
- Task 2: priorytet 2
- Task 3: priorytet 3 (najwyższy)

```c
xTaskCreate(vTask1, "Task1", 128, NULL, 1, NULL);  // prio 1
xTaskCreate(vTask2, "Task2", 128, NULL, 2, NULL);  // prio 2
xTaskCreate(vTask3, "Task3", 128, NULL, 3, NULL);  // prio 3
```

### 4.2 Obserwacja

Uruchom program i obserwuj:

```
Task 3 is running    ← Najwyższy priorytet
Task 3 is running
Task 3 is running
Task 2 is running    ← Średni (gdy Task 3 delay)
Task 2 is running
Task 1 is running    ← Najniższy (gdy Task 2 i 3 delay)
```

**TODO:** Wyjaśnij w sprawozdaniu dlaczego tak się dzieje.

---

## Zadanie 5: Stack overflow (15 min)

### 5.1 Symulacja stack overflow

```c
void vStackOverflowTask(void *pvParameters) {
    char big_array[10000];  // Za duży dla stosu!
    memset(big_array, 0, sizeof(big_array));

    while (1) {
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

// W main():
xTaskCreate(vStackOverflowTask, "Stack", 128, NULL, 1, NULL);
```

### 5.2 Obserwacja

**HINT:** Program może się zawiesić lub pokazać błąd.

**TODO:** Zwiększ rozmiar stosu do 1024 słów. Czy problem zniknął?

---

## Zadanie Bonus: Task States

Zaimplementuj zadanie, które:
- Przechodzi przez różne stany (Running → Blocked → Ready)
- Wypisuje swój stan

```c
void vStateTask(void *pvParameters) {
    printf("Task started (Ready → Running)\n");

    while (1) {
        printf("Working...\n");
        vTaskDelay(pdMS_TO_TICKS(100));  // Running → Blocked
        // Po delay: Blocked → Ready → Running
    }
}
```

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Środowisko skonfigurowane | 2 |
| 2 | Demo uruchomione | 2 |
| 3 | Task 3 dodany | 2 |
| 4 | Eksperyment priorytetów | 2 |
| 5 | Stack overflow zrozumiany | 2 |

---

## Sprawozdanie

W sprawozdaniu uwzględnij:

1. Odpowiedzi na pytania z zadań
2. Zrzuty ekranu z uruchomionego programu
3. Wyjaśnienie zachowania priorytetów
4. Wnioski o wpływie `vTaskDelay` na działanie systemu
5. Wyjaśnienie problemu stack overflow

---

## Pytania kontrolne

1. Czym różni się RTOS od zwykłego OS?
2. Co to jest preemptive scheduling?
3. Dlaczego zadania w RTOS mają pętle nieskończone?
4. Co się dzieje, gdy zadanie nie oddaje CPU?
5. Jak obliczyć potrzebny rozmiar stosu?

---

## Co na następnym laboratorium?

- Tworzenie i zarządzanie zadaniami
- Stany zadań i przełączanie kontekstu
- Priorytety i scheduling
- Badanie wpływu priorytetów na działanie systemu