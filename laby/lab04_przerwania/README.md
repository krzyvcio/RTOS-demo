# Laboratorium 4: Przerwania i Obsługa ISR

**Czas:** 2 godziny
**Punkty:** 15 pkt

---

## Cel ćwiczenia

1. Zrozumienie mechanizmu przerwań
2. Implementacja procedur ISR
3. Komunikacja ISR ↔ Task
4. Minimalizacja czasu ISR

---

## Teoria (20 min)

### Czym jest przerwanie?

```
Przerwanie = Sygnał hardware'owy przerywający normalne wykonanie

Flow:
1. Hardware signal (UART, Timer, GPIO)
2. CPU kończy bieżącą instrukcję
3. CPU zapisuje kontekst
4. Skok do ISR (Interrupt Service Routine)
5. ISR wykonuje się
6. Przywrócenie kontekstu
7. Powrót do przerwanej instrukcji
```

### Zasady ISR

```
1. KRÓTKIE - mikrosekundy, nie milisekundy
2. BEZ BLOKOWANIA - żadnych mutexów, delay
3. BEZ ALOKACJI - brak malloc
4. BEZPIECZNE FUNKCJE - tylko reentrant
5. CZYSZCZENIE FLAG - zawsze czyść źródło przerwania
```

---

## Zadanie 1: Podstawowe przerwanie (25 min)

### 1.1 Symulacja przerwania timerem

W symulatorze Linux używamy sygnały jako "przerwania":

```c
#include <signal.h>
#include "FreeRTOS.h"
#include "task.h"
#include "semphr.h"
#include <stdio.h>

volatile uint32_t interrupt_count = 0;

// Handler symulowanego przerwania
void timer_signal_handler(int sig) {
    interrupt_count++;

    // W rzeczywistym systemie:
    // 1. Zapisz dane
    // 2. Zasygnalizuj task
    // 3. Wyczyść flagę
}

void setup_timer_interrupt(void) {
    struct sigaction sa;
    sa.sa_handler = timer_signal_handler;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    sigaction(SIGALRM, &sa, NULL);

    // Ustaw timer
    struct itimerval timer;
    timer.it_value.tv_sec = 0;
    timer.it_value.tv_usec = 100000;  // 100ms
    timer.it_interval = timer.it_value;
    setitimer(ITIMER_REAL, &timer, NULL);
}

void vMonitorTask(void *pvParameters) {
    while (1) {
        printf("Interrupts: %lu\n", interrupt_count);
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}

int main(void) {
    setup_timer_interrupt();

    xTaskCreate(vMonitorTask, "Monitor", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 1.2 Uruchom i obserwuj

```
Interrupts: 0
Interrupts: 10   ← 10 przerwań na sekundę (100ms period)
Interrupts: 20
Interrupts: 30
...
```

---

## Zadanie 2: ISR → Task komunikacja (30 min)

### 2.1 Problem

```
ISR nie może:
- Blokować (czekać na mutex)
- Wywoływać funkcji blokujących
- Wykonywać długich operacji

Rozwiązanie:
ISR daje sygnał → Task przetwarza
```

### 2.2 Implementacja z semaforem

```c
#include "FreeRTOS.h"
#include "task.h"
#include "semphr.h"

SemaphoreHandle_t data_ready_sem;
volatile uint8_t uart_data;
volatile bool data_available = false;

// ISR - tylko zbiera dane
void UART_IRQHandler(void) {
    // Odczytaj dane (szybkie!)
    uart_data = UART->DATA;  // Symulacja

    // Zasygnalizuj task
    BaseType_t higher_priority_woken = pdFALSE;
    xSemaphoreGiveFromISR(data_ready_sem, &higher_priority_woken);

    // Jeśli wyższy priorytet gotowy - request switch
    portYIELD_FROM_ISR(higher_priority_woken);
}

// Task - przetwarza dane
void vUARTTask(void *pvParameters) {
    while (1) {
        // Czekaj na sygnał od ISR
        if (xSemaphoreTake(data_ready_sem, portMAX_DELAY) == pdTRUE) {
            // Teraz bezpiecznie przetwarzaj
            printf("Received: 0x%02X\n", uart_data);

            // Długie operacje OK w task
            process_data(uart_data);
        }
    }
}

int main(void) {
    data_ready_sem = xSemaphoreCreateBinary();

    xTaskCreate(vUARTTask, "UART", 128, NULL, 2, NULL);

    // W rzeczywistym systemie: NVIC_EnableIRQ(UART_IRQn)

    vTaskStartScheduler();
    return 0;
}
```

### 2.3 Symulacja ISR w task

```c
// Symulujemy ISR jako high-priority task
void vSimulatedISR(void *pvParameters) {
    while (1) {
        vTaskDelay(pdMS_TO_TICKS(100));

        // Symuluj przyjście danych
        uart_data = rand() % 256;

        // To jest "ISR"
        BaseType_t higher_priority_woken = pdFALSE;
        xSemaphoreGiveFromISR(data_ready_sem, &higher_priority_woken);
        portYIELD_FROM_ISR(higher_priority_woken);
    }
}
```

---

## Zadanie 3: Queue z ISR (25 min)

### 3.1 Kolejka z ISR

```c
QueueHandle_t data_queue;

#define QUEUE_SIZE 16

void UART_IRQHandler(void) {
    uint8_t data = UART->DATA;

    // Wyślij do kolejki (nie blokuje!)
    BaseType_t higher_priority_woken = pdFALSE;
    xQueueSendFromISR(data_queue, &data, &higher_priority_woken);
    portYIELD_FROM_ISR(higher_priority_woken);
}

void vConsumerTask(void *pvParameters) {
    uint8_t received_data;

    while (1) {
        // Czekaj na dane z kolejki
        if (xQueueReceive(data_queue, &received_data, portMAX_DELAY) == pdTRUE) {
            process_data(received_data);
        }
    }
}

int main(void) {
    data_queue = xQueueCreate(QUEUE_SIZE, sizeof(uint8_t));

    xTaskCreate(vConsumerTask, "Consumer", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 3.2 TODO - Multiple data w ISR

Zaimplementuj ISR, który odczytuje całą paczkę danych:

```c
void UART_IRQHandler(void) {
    // Symuluj paczkę 5 bajtów
    uint8_t packet[5];

    for (int i = 0; i < 5; i++) {
        packet[i] = UART->DATA;  // Szybkie!
    }

    // Wyślij całą paczkę do taska
    BaseType_t higher_priority_woken = pdFALSE;
    xQueueSendFromISR(packet_queue, packet, &higher_priority_woken);
    portYIELD_FROM_ISR(higher_priority_woken);
}
```

---

## Zadanie 4: ISR Timing (20 min)

### 4.1 Mierzenie czasu ISR

```c
volatile uint32_t isr_entry_time;
volatile uint32_t isr_exit_time;
volatile uint32_t isr_max_time = 0;

void TIM_IRQHandler(void) {
    isr_entry_time = get_cycle_count();  // ARM: DWT->CYCCNT

    // Operacje ISR
    process_interrupt();

    isr_exit_time = get_cycle_count();
    uint32_t elapsed = isr_exit_time - isr_entry_time;

    if (elapsed > isr_max_time) {
        isr_max_time = elapsed;
    }

    // Clear flag
    TIM->SR &= ~TIM_SR_UIF;
}

void vStatsTask(void *pvParameters) {
    while (1) {
        printf("ISR max time: %lu cycles (%.2f us)\n",
               isr_max_time,
               (float)isr_max_time / (SystemCoreClock / 1000000));
        vTaskDelay(pdMS_TO_TICKS(5000));
    }
}
```

### 4.2 Inicjalizacja cycle counter (ARM Cortex-M)

```c
void enable_cycle_counter(void) {
    CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;
    DWT->CYCCNT = 0;
    DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;
}

uint32_t get_cycle_count(void) {
    return DWT->CYCCNT;
}
```

---

## Zadanie 5: Pułapki ISR (20 min)

### 5.1 Pułapka 1: Długi ISR

```c
// ŹLE: Długa operacja w ISR
void UART_IRQHandler(void) {
    char buffer[256];
    int index = 0;

    // Czytaj całą linię - MOŻE TRWAĆ!
    while (UART->DATA != '\n') {
        buffer[index++] = UART->DATA;
    }
    buffer[index] = '\0';

    // Parsuj - TEŻ TRWA!
    parse_command(buffer);

    // Wykonaj komendę - NAJGORZEJ!
    execute_command(buffer);
}

// DOBRZE: Minimalny ISR
void UART_IRQHandler(void) {
    uint8_t data = UART->DATA;

    if (data == '\n') {
        // Tylko sygnał
        xSemaphoreGiveFromISR(line_ready_sem, &higher_priority_woken);
    } else {
        // Tylko zbierz
        buffer[buffer_index++] = data;
    }

    portYIELD_FROM_ISR(higher_priority_woken);
}
```

### 5.2 Pułapka 2: Mutex w ISR

```c
// ŹLE: Mutex w ISR = DEADLOCK RISK!
void UART_IRQHandler(void) {
    xSemaphoreTake(uart_mutex, portMAX_DELAY);  // NIEBEZPIECZNE!
    // ISR może się zablokować!
    xSemaphoreGive(uart_mutex);
}

// DOBRZE: Brak mutexów w ISR
void UART_IRQHandler(void) {
    // Tylko sygnał, bez mutexów
    xQueueSendFromISR(data_queue, &data, &higher_priority_woken);
    portYIELD_FROM_ISR(higher_priority_woken);
}
```

### 5.3 Pułapka 3: Printf w ISR

```c
// ŹLE: Printf w ISR
void UART_IRQHandler(void) {
    printf("Received: %d\n", UART->DATA);  // printf może:
    // - Blokować
    // - Używać malloc
    // - Wywoływać inne funkcje
    // - Zająć ms do s!
}

// DOBRZE: Tylko zbierz dane
void UART_IRQHandler(void) {
    uint8_t data = UART->DATA;
    xQueueSendFromISR(data_queue, &data, NULL);
}

// Task robi printf
void vUARTTask(void *pvParameters) {
    uint8_t data;
    while (1) {
        xQueueReceive(data_queue, &data, portMAX_DELAY);
        printf("Received: %d\n", data);  // Bezpiecznie w task
    }
}
```

---

## Zadanie Bonus: Deferred Interrupt Handling

### Pattern: Top Half / Bottom Half

```c
// Top Half (ISR) - natychmiast
void ADC_IRQHandler(void) {
    uint16_t value = ADC->DR;

    // Tylko dodaj do bufora
    adc_buffer[adc_head] = value;
    adc_head = (adc_head + 1) % ADC_BUFFER_SIZE;
    adc_count++;

    // Zasygnalizuj bottom half
    xSemaphoreGiveFromISR(adc_ready_sem, &higher_priority_woken);
    portYIELD_FROM_ISR(higher_priority_woken);
}

// Bottom Half (Task) - odroczone
void vADCProcessTask(void *pvParameters) {
    while (1) {
        xSemaphoreTake(adc_ready_sem, portMAX_DELAY);

        // Przetwórz wszystkie próbki
        while (adc_count > 0) {
            uint16_t value = adc_buffer[adc_tail];
            adc_tail = (adc_tail + 1) % ADC_BUFFER_SIZE;
            adc_count--;

            // Długie operacje OK
            process_sample(value);
            update_statistics(value);
            check_thresholds(value);
        }
    }
}
```

---

## Tabela API ISR-safe

| Funkcja | ISR-safe | Opis |
|---------|----------|------|
| `xQueueSendFromISR()` | ✓ | Wyślij do kolejki |
| `xQueueReceiveFromISR()` | ✓ | Odbierz z kolejki |
| `xSemaphoreGiveFromISR()` | ✓ | Signal semafor |
| `xSemaphoreTakeFromISR()` | ✓ | Try-take semafor |
| `xTaskNotifyFromISR()` | ✓ | Notyfikuj task |
| `vTaskNotifyGiveFromISR()` | ✓ | Give notyfikację |
| `xQueueSend()` | ✗ | Blokuje! |
| `xSemaphoreTake()` | ✗ | Blokuje! |
| `vTaskDelay()` | ✗ | Blokuje! |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Przerwanie symulowane | 2 |
| 2 | ISR → Task komunikacja | 4 |
| 3 | Queue z ISR | 4 |
| 4 | ISR timing zmierzone | 3 |
| 5 | Pułapki zrozumiane | 2 |

---

## Sprawozdanie

1. Diagram przepływu ISR → Task
2. Pomiary czasu ISR (max, avg)
3. Wykaz funkcji ISR-safe vs niebezpiecznych
4. Wnioski: dlaczego ISR musi być krótki

---

## Pytania kontrolne

1. Dlaczego ISR nie może używać mutexów?
2. Co to jest `portYIELD_FROM_ISR`?
3. Jak mierzyć czas wykonania ISR?
4. Czym różni się `xQueueSend` od `xQueueSendFromISR`?
5. Co to jest deferred interrupt handling?