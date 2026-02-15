# ISR (Interrupt Service Routine)

## Definicja

**ISR** to procedura obsługi przerwania - funkcja wywoływana przez hardware w odpowiedzi na zdarzenie zewnętrzne. ISR "przejmuje" kontrolę nad CPU, zatrzymując aktualnie wykonywany kod.

> ISR to "niezapowiedziany gość" - może przyjść w dowolnym momencie i domagać się natychmiastowej uwagi. To najszybszy sposób reakcji na zdarzenia, ale też najbardziej niebezpieczny.

```
┌─────────────────────────────────────────────────────────┐
│                  INTERRUPT FLOW                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task running:  ████████████████████████████████       │
│                                    │                    │
│  Interrupt signal ──────────────────┼─────────────────►│
│                                    │                    │
│                                    ▼                    │
│  ISR:                     [████████]                    │
│                                    │                    │
│                                    ▼                    │
│  Task continues:           ████████████████████████    │
│                                                         │
│  Task został "przerwany" w trakcie wykonywania.        │
│  ISR wykonuje się, potem task kontynuuje.              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Analogia do przyrody

### 🚨 Reakcja ucieczki

Zwierzę pasie się spokojnie. Nagle słyszy hałas:

```
Normalna aktywność: jedzenie trawy (task)
Sygnał: hałas (interrupt)
Reakcja: Ucieczka (ISR)
Po reakcji: powrót do jedzenia (task resume)

To jest ISR w naturze - przerwij wszystko, obsłuż zagrożenie,
wróć do normalnej aktywności.
```

### 💓 Reakcja odruchowa

Dotykasz gorącego garnka:

```
Normalna aktywność: gotowanie (task)
Sygnał: ból (interrupt - z nerwów)
Reakcja: cofnięcie ręki (ISR)
Po reakcji: kontynuacja gotowania (task resume)

Odruch jest szybszy niż świadoma decyzja.
ISR to "odruch" systemu.
```

### 🧠 Reakcja na nagły bodziec

Siedzisz i czytasz. Nagle ktoś krzyczy Twoje imię:

```
Normalna aktywność: czytanie (task)
Sygnał: słyszysz swoje imię (interrupt)
Reakcja: podniesienie głowy, szukanie źródła (ISR)
Po reakcji: kontynuacja czytania (task resume)

Twój mózg ma "hardware interrupt" na własne imię.
```

---

## Podobieństwo do systemów informatycznych

### Web Server Request

```javascript
// "Interrupt-like" behavior
app.get('/urgent', (req, res) => {
    // To jest jak ISR - natychmiastowa reakcja
    handleUrgentRequest();
    res.send('Handled');
});

// Główna aplikacja kontynuuje
app.listen(3000);
```

### Signal Handling

```c
// Linux signal - podobne do ISR
void signal_handler(int sig) {
    // To jest jak ISR
    if (sig == SIGINT) {
        handle_ctrl_c();
    }
}

int main() {
    signal(SIGINT, signal_handler);
    // Main program...
}
```

### Event Loop

```javascript
// JavaScript event loop - cooperative "interrupts"
button.addEventListener('click', (event) => {
    // To jest jak ISR - event handler
    handleClick(event);
});

// Main loop kontynuuje
```

---

## Budowa ISR

### Szkielet ISR

```c
void UART_IRQHandler(void) {
    // 1. Zapisz kontekst (automatycznie przez hardware)
    // 2. Sprawdź źródło przerwania
    uint32_t status = UART->STATUS;

    if (status & RX_DATA_AVAILABLE) {
        // 3. Obsłuż przerwanie
        uint8_t data = UART->DATA;
        buffer[buffer_index++] = data;

        // 4. Wyczyść flagę przerwania
        UART->STATUS_CLEAR = RX_DATA_AVAILABLE;
    }

    // 5. Przywróć kontekst (automatycznie przez hardware)
}
```

### Ważne zasady ISR

```
1. KRÓTKI CZAS WYKONANIA
   ISR powinien być jak najkrótszy.
   Mikrosekundy, nie milisekundy.

2. BRAK BLOKOWANIA
   Nie czekaj na mutex, semafor, I/O.
   ISR nie może być blokowany.

3. BRAK ALOKACJI PAMIĘCI
   malloc() w ISR = recipe for disaster.
   Używaj pre-allocated buffers.

4. BEZPIECZNE FUNKCJE
   Tylko funkcje reentrant-safe.
   printf() w ISR = zły pomysł.

5. CZYSZCZENIE FLAG
   Zawsze czyść flagę przerwania!
   Inaczej ISR wywoła się ponownie.
```

---

## Priorytety przerwań

### NVIC (Nested Vector Interrupt Controller)

```
┌─────────────────────────────────────────────────────────┐
│                  INTERRUPT PRIORITIES                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Priority 0: Reset (highest)                           │
│  Priority 1: NMI (Non-Maskable Interrupt)              │
│  Priority 2: HardFault                                  │
│  Priority 3-15: System exceptions                       │
│  Priority 16-255: External interrupts                   │
│                                                         │
│  Wyższy priorytet może przerwać niższy.                │
│  Niższy priorytet musi czekać.                          │
│                                                         │
│  ISR_A (prio 10) running:                               │
│  ISR_B (prio 5) arrives:                                │
│  → ISR_A preempted, ISR_B runs                          │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Konfiguracja priorytetów

```c
// ARM Cortex-M NVIC
void configure_interrupts(void) {
    // Ustaw priorytety
    NVIC_SetPriority(UART_IRQn, 5);     // Średni priorytet
    NVIC_SetPriority(Timer_IRQn, 10);   // Niski priorytet
    NVIC_SetPriority(ADC_IRQn, 3);      // Wysoki priorytet

    // Włącz przerwania
    NVIC_EnableIRQ(UART_IRQn);
    NVIC_EnableIRQ(Timer_IRQn);
    NVIC_EnableIRQ(ADC_IRQn);
}
```

---

## ISR vs Task

| Cecha | ISR | Task |
|-------|-----|------|
| Wyzwalacz | Hardware signal | Scheduler |
| Kontekst | Special (IRQ mode) | Normal |
| Priorytet | Hardware-defined | Software-defined |
| Blokowanie | Niedozwolone | Dozwolone |
| Czas wykonania | Mikrosekundy | Milisekundy+ |
| Preemption | Wyższy priorytet ISR | Wyższy priorytet task |
| API | Ograniczone | Pełne |

---

## Bottom Half Processing

ISR powinien być krótki. Długa operacja powinna być w tasku.

### Pattern: ISR + Task

```c
// Zmienne współdzielone
volatile uint8_t uart_buffer[256];
volatile uint16_t uart_index = 0;
volatile bool data_ready = false;

// ISR - tylko zbierz dane
void UART_IRQHandler(void) {
    uint8_t data = UART->DATA;
    uart_buffer[uart_index++] = data;

    if (data == '\n') {
        data_ready = true;
        // Zasygnalizuj task
        xTaskNotifyFromISR(uart_task_handle, 0, eSetBits, NULL);
    }
}

// Task - przetwórz dane
void uart_task(void* pvParameters) {
    while (1) {
        // Czekaj na sygnał od ISR
        ulTaskNotifyTake(pdTRUE, portMAX_DELAY);

        if (data_ready) {
            // Długa operacja - bezpieczna w tasku
            process_uart_data(uart_buffer, uart_index);
            uart_index = 0;
            data_ready = false;
        }
    }
}
```

### Deferred Interrupt Handling

```c
// ISR: Top Half (szybki)
void GPIO_IRQHandler(void) {
    // Minimalna obsługa
    uint32_t status = GPIO->STATUS;
    GPIO->STATUS_CLEAR = status;

    // Zdeleguj do bottom half
    schedule_bottom_half(gpio_handler, status);
}

// Bottom Half (wolniejszy)
void gpio_handler(uint32_t status) {
    // Dłuższa obsługa
    if (status & BUTTON_PRESSED) {
        handle_button_press();
    }
    if (status & SENSOR_DATA_READY) {
        read_sensor();
    }
}
```

---

## Typowe pułapki ISR

### Pułapka 1: Długi ISR

```c
// ŹLE: Długa operacja w ISR
void UART_IRQHandler(void) {
    char* line = read_line();  // Może trwać milisekundy!
    parse_command(line);       // Może trwać milisekundy!
    execute_command(line);     // Może trwać milisekundy!
    respond(line);             // Może trwać milisekundy!
}

// DOBRZE: Krótki ISR + task
void UART_IRQHandler(void) {
    char c = UART->DATA;
    buffer[index++] = c;
    if (c == '\n') {
        notify_task();  // Szybkie!
    }
}
```

### Pułapka 2: Blokowanie w ISR

```c
// ŹLE: Mutex w ISR
void UART_IRQHandler(void) {
    mutex_lock(&uart_mutex);  // MOŻE ZABLOKOWAĆ!
    // ...
    mutex_unlock(&uart_mutex);
}

// DOBRZE: Użyj task notification
void UART_IRQHandler(void) {
    // Żadnych mutexów w ISR!
    data = UART->DATA;
    xTaskNotifyFromISR(uart_task, data, eSetValueWithOverwrite, NULL);
}
```

### Pułapka 3: Race Condition

```c
// ŹLE: Race condition
volatile uint32_t counter = 0;

void Timer_IRQHandler(void) {
    counter++;  // Nieatomicowe! Może być przerwane
}

// DOBRZE: Atomowe operacje
void Timer_IRQHandler(void) {
    __atomic_add_fetch(&counter, 1, __ATOMIC_RELAXED);
}

// ALBO: Wyłącz przerwania
void Timer_IRQHandler(void) {
    __disable_irq();
    counter++;
    __enable_irq();
}
```

### Pułapka 4: Niebezpieczne funkcje

```c
// ŹLE: printf w ISR
void UART_IRQHandler(void) {
    printf("Received: %c\n", UART->DATA);  // NIEBEZPIECZNE!
    // printf może blokować, używa malloc, nie jest reentrant
}

// DOBRZE: Prosty bufor
void UART_IRQHandler(void) {
    uint8_t data = UART->DATA;
    buffer[index] = data;
    index = (index + 1) % BUFFER_SIZE;
}
```

---

## Latencja przerwania

### Składniki latencji

```
┌─────────────────────────────────────────────────────────┐
│              INTERRUPT LATENCY                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Sygnał przerwania                                      │
│       │                                                 │
│       ▼                                                 │
│  ┌─────────────────┐                                   │
│  │ Hardware Delay  │ ~1-10 cycles                      │
│  │ (signal detect) │                                   │
│  └────────┬────────┘                                   │
│           ▼                                             │
│  ┌─────────────────┐                                   │
│  │ Complete        │ Variable (depends on instruction) │
│  │ Current Instr   │                                   │
│  └────────┬────────┘                                   │
│           ▼                                             │
│  ┌─────────────────┐                                   │
│  │ Context Save    │ ~10-20 cycles                     │
│  │ (push registers)│                                   │
│  └────────┬────────┘                                   │
│           ▼                                             │
│  ┌─────────────────┐                                   │
│  │ ISR Entry       │ ~5-10 cycles                      │
│  │ (vector fetch)  │                                   │
│  └────────┬────────┘                                   │
│           ▼                                             │
│        ISR starts                                       │
│                                                         │
│  Total latency: ~20-50 cycles (typical)                │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Minimalizacja latencji

```c
// 1. Używaj NVIC priority grouping
NVIC_SetPriorityGrouping(3);  // 4 bits preemption, 0 bits subpriority

// 2. Priorytetyzuj krytyczne przerwania
NVIC_SetPriority(CRITICAL_IRQ, 0);  // Najwyższy priorytet

// 3. Minimalizuj czas z wyłączonymi przerwaniami
void critical_section(void) {
    __disable_irq();
    // Minimalny kod
    __enable_irq();
}

// 4. Używaj fast interrupts (FIQ) jeśli dostępne
void FIQ_Handler(void) {
    // FIQ ma niższą latencję niż IRQ
}
```

---

## Nesting przerwań

```c
// Konfiguracja nesting przerwań
void setup_nested_interrupts(void) {
    // Grupa priorytetów
    // Priority 0-3: Preemption disabled (nie mogą przerwać się nawzajem)
    // Priority 4-15: Preemption enabled

    NVIC_SetPriorityGrouping(4);  // 4 bits preemption priority

    // Timer: może przerwać UART
    NVIC_SetPriority(TIMER_IRQn, 2);   // Wyższy priorytet
    NVIC_SetPriority(UART_IRQn, 5);    // Niższy priorytet

    // UART ISR może być przerwany przez Timer ISR
}
```

---

## ISR w praktyce

### Przykład kompletny

```c
// Buffer cykliczny dla UART
#define UART_BUFFER_SIZE 256

typedef struct {
    uint8_t data[UART_BUFFER_SIZE];
    volatile uint16_t head;
    volatile uint16_t tail;
} RingBuffer;

RingBuffer uart_rx_buffer;
TaskHandle_t uart_task_handle;

// Inicjalizacja
void uart_init(void) {
    // Konfiguracja hardware
    UART->BAUDRATE = 115200;
    UART->CTRL = UART_CTRL_RX_ENABLE | UART_CTRL_RX_INT_ENABLE;

    // Konfiguracja NVIC
    NVIC_SetPriority(UART_IRQn, 5);
    NVIC_EnableIRQ(UART_IRQn);

    // Buffer
    uart_rx_buffer.head = 0;
    uart_rx_buffer.tail = 0;
}

// ISR
void UART_IRQHandler(void) {
    uint32_t status = UART->STATUS;

    if (status & UART_STATUS_RX_NOT_EMPTY) {
        // Czytaj dane
        uint8_t data = UART->DATA;

        // Zapisz do bufora
        uint16_t next_head = (uart_rx_buffer.head + 1) % UART_BUFFER_SIZE;

        if (next_head != uart_rx_buffer.tail) {
            // Buffer nie jest pełny
            uart_rx_buffer.data[uart_rx_buffer.head] = data;
            uart_rx_buffer.head = next_head;
        }
        // Jeśli buffer pełny - dane tracone

        // Wyczyść flagę
        UART->STATUS_CLEAR = UART_STATUS_RX_NOT_EMPTY;
    }

    // Zasygnalizuj task (tylko jeśli są dane)
    if (uart_rx_buffer.head != uart_rx_buffer.tail) {
        BaseType_t higher_priority_task_woken = pdFALSE;
        vTaskNotifyGiveFromISR(uart_task_handle, &higher_priority_task_woken);
        portYIELD_FROM_ISR(higher_priority_task_woken);
    }
}

// Task przetwarzający dane
void uart_task(void* pvParameters) {
    while (1) {
        // Czekaj na dane
        ulTaskNotifyTake(pdTRUE, portMAX_DELAY);

        // Przetwórz wszystkie dostępne dane
        while (uart_rx_buffer.tail != uart_rx_buffer.head) {
            uint8_t data = uart_rx_buffer.data[uart_rx_buffer.tail];
            uart_rx_buffer.tail = (uart_rx_buffer.tail + 1) % UART_BUFFER_SIZE;

            process_byte(data);
        }
    }
}
```

---

## Pytania do przemyślenia

1. Jakie przerwania ma Twój system? Jakie mają priorytety?
2. Jak długo trwają Twoje ISR? Czy są krótkie?
3. Czy używasz bottom-half processing dla długich operacji?

---

## Quiz

**Pytanie**: Masz ISR, który musi przetworzyć dane z ADC i wysłać wynik przez UART. Jak to zrobić bezpiecznie?

**Odpowiedź**:

```c
// ŹLE: Wszystko w ISR
void ADC_IRQHandler(void) {
    uint16_t data = ADC->DATA;
    process_data(data);     // Długa operacja!
    UART->DATA = result;    // Może blokować!
}

// DOBRZE: ISR + task
volatile uint16_t adc_data;
TaskHandle_t adc_task_handle;

void ADC_IRQHandler(void) {
    adc_data = ADC->DATA;    // Szybkie!
    xTaskNotifyFromISR(adc_task_handle, adc_data, eSetValueWithOverwrite, NULL);
}

void adc_task(void* pvParameters) {
    while (1) {
        uint32_t data = ulTaskNotifyTake(pdTRUE, portMAX_DELAY);
        process_data(data);    // Bezpieczne w tasku
        send_uart(result);     // Bezpieczne w tasku
    }
}
```

---

## Wskazówka zapamiętywania

> **ISR = Odruch bezwarunkowy**
>
> Kiedy dotykasz gorącego garnka:
> 1. Nie myślisz (to nie jest task)
> 2. Reagujesz natychmiast (to jest ISR)
> 3. Robisz minimum (odsuwasz rękę)
> 4. Potem analizujesz (to jest task)
>
> ISR powinien być jak odruch:
> - Szybki
> - Automatyczny
> - Minimalny
> - Bez myślenia (bez skomplikowanej logiki)