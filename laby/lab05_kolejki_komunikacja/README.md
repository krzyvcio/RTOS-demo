# Laboratorium 5: Komunikacja między zadaniami - Kolejki

**Czas:** 2 godziny
**Punkty:** 15 pkt

---

## Cel ćwiczenia

1. Użycie kolejek do komunikacji między zadaniami
2. Implementacja wzorca producer-consumer
3. Obsługa pełnych/pustych kolejek
4. Przesyłanie złożonych struktur danych

---

## Teoria (15 min)

### Kolejki w RTOS

```
Queue = FIFO buffer z synchronizacją

Właściwości:
- Określona pojemność (liczba elementów)
- Określony rozmiar elementu
- Thread-safe (bezpieczna dla wielu tasków)
- Blokująca (czeka gdy pusta/pełna)

Operacje:
- Send: dodaj element (blokuje gdy pełna)
- Receive: pobierz element (blokuje gdy pusta)
```

### Dlaczego kolejki?

```
BEZ KOLEJKI:
Task A → shared_data ← Task B
         (race condition!)

Z KOLEJKĄ:
Task A → [Queue] → Task B
         (bezpieczna komunikacja)
```

---

## Zadanie 1: Podstawowa kolejka (20 min)

### 1.1 Tworzenie i używanie kolejki

```c
#include "FreeRTOS.h"
#include "task.h"
#include "queue.h"
#include <stdio.h>

QueueHandle_t data_queue;

void vSenderTask(void *pvParameters) {
    int counter = 0;

    while (1) {
        counter++;

        // Wyślij dane do kolejki
        if (xQueueSend(data_queue, &counter, pdMS_TO_TICKS(100)) == pdTRUE) {
            printf("Sent: %d\n", counter);
        } else {
            printf("Queue full!\n");
        }

        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vReceiverTask(void *pvParameters) {
    int received_data;

    while (1) {
        // Odbierz dane z kolejki
        if (xQueueReceive(data_queue, &received_data, pdMS_TO_TICKS(500)) == pdTRUE) {
            printf("Received: %d\n", received_data);
        } else {
            printf("Queue empty!\n");
        }
    }
}

int main(void) {
    // Utwórz kolejkę: 10 elementów, każdy po 4 bajty (sizeof(int))
    data_queue = xQueueCreate(10, sizeof(int));

    if (data_queue == NULL) {
        printf("Failed to create queue!\n");
        return 1;
    }

    xTaskCreate(vSenderTask, "Sender", 128, NULL, 1, NULL);
    xTaskCreate(vReceiverTask, "Receiver", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 1.2 Obserwacja

```
Sent: 1
Received: 1
Sent: 2
Sent: 3      ← Receiver wolniejszy
Received: 2
Sent: 4
Received: 3
...
```

---

## Zadanie 2: Multiple Producers, Single Consumer (25 min)

### 2.1 Wiele źródeł danych

```c
typedef struct {
    int source_id;
    int value;
    TickType_t timestamp;
} DataItem;

QueueHandle_t data_queue;

void vProducerTask(void *pvParameters) {
    int source_id = (int)pvParameters;
    int counter = 0;

    while (1) {
        DataItem item;
        item.source_id = source_id;
        item.value = counter++;
        item.timestamp = xTaskGetTickCount();

        xQueueSend(data_queue, &item, portMAX_DELAY);
        printf("[Producer %d] Sent: %d\n", source_id, item.value);

        vTaskDelay(pdMS_TO_TICKS(100 + source_id * 50));
    }
}

void vConsumerTask(void *pvParameters) {
    DataItem received;

    while (1) {
        if (xQueueReceive(data_queue, &received, portMAX_DELAY) == pdTRUE) {
            printf("[Consumer] From %d: value=%d, age=%lu ticks\n",
                   received.source_id,
                   received.value,
                   xTaskGetTickCount() - received.timestamp);
        }
    }
}

int main(void) {
    data_queue = xQueueCreate(20, sizeof(DataItem));

    // 3 producentów
    for (int i = 0; i < 3; i++) {
        xTaskCreate(vProducerTask, "Producer", 128, (void*)i, 1, NULL);
    }

    // 1 konsument
    xTaskCreate(vConsumerTask, "Consumer", 128, NULL, 2, NULL);

    vTaskStartScheduler();
    return 0;
}
```

---

## Zadanie 3: Queue Sets - Multiple Queues (25 min)

### 3.1 Czekanie na wiele kolejek

```c
QueueHandle_t sensor_queue;
QueueHandle_t command_queue;
QueueSetHandle_t queue_set;

typedef enum {
    DATA_TYPE_SENSOR,
    DATA_TYPE_COMMAND
} DataType;

typedef struct {
    DataType type;
    union {
        int sensor_value;
        char command[16];
    };
} DataPacket;

void vSensorTask(void *pvParameters) {
    while (1) {
        DataPacket packet;
        packet.type = DATA_TYPE_SENSOR;
        packet.sensor_value = rand() % 100;

        xQueueSend(sensor_queue, &packet, portMAX_DELAY);
        printf("Sensor sent: %d\n", packet.sensor_value);

        vTaskDelay(pdMS_TO_TICKS(200));
    }
}

void vCommandTask(void *pvParameters) {
    int cmd_count = 0;

    while (1) {
        DataPacket packet;
        packet.type = DATA_TYPE_COMMAND;
        snprintf(packet.command, sizeof(packet.command), "CMD_%d", cmd_count++);

        xQueueSend(command_queue, &packet, portMAX_DELAY);
        printf("Command sent: %s\n", packet.command);

        vTaskDelay(pdMS_TO_TICKS(500));
    }
}

void vProcessorTask(void *pvParameters) {
    while (1) {
        // Czekaj na którąkolwiek kolejkę
        QueueSetMemberHandle_t active_queue = xQueueSelectFromSet(queue_set, portMAX_DELAY);

        DataPacket packet;

        if (active_queue == sensor_queue) {
            xQueueReceive(sensor_queue, &packet, 0);
            printf("Processing sensor: %d\n", packet.sensor_value);
        } else if (active_queue == command_queue) {
            xQueueReceive(command_queue, &packet, 0);
            printf("Processing command: %s\n", packet.command);
        }
    }
}

int main(void) {
    sensor_queue = xQueueCreate(10, sizeof(DataPacket));
    command_queue = xQueueCreate(10, sizeof(DataPacket));

    // Queue set - sum of both queue sizes
    queue_set = xQueueCreateSet(20);
    xQueueAddToSet(sensor_queue, queue_set);
    xQueueAddToSet(command_queue, queue_set);

    xTaskCreate(vSensorTask, "Sensor", 128, NULL, 1, NULL);
    xTaskCreate(vCommandTask, "Command", 128, NULL, 1, NULL);
    xTaskCreate(vProcessorTask, "Processor", 128, NULL, 2, NULL);

    vTaskStartScheduler();
    return 0;
}
```

---

## Zadanie 4: Przepełnienie kolejki (20 min)

### 4.1 Strategie obsługi pełnej kolejki

```c
QueueHandle_t bounded_queue;

// Strategia 1: Block (czekaj)
void send_blocking(int data) {
    xQueueSend(bounded_queue, &data, portMAX_DELAY);  // Czeka aż będzie miejsce
}

// Strategia 2: Timeout
void send_timeout(int data) {
    if (xQueueSend(bounded_queue, &data, pdMS_TO_TICKS(100)) != pdTRUE) {
        printf("Queue full, timeout!\n");
        // Obsłuż przepełnienie
    }
}

// Strategia 3: Overwrite (ostatni element nadpisany)
void send_overwrite(int data) {
    xQueueOverwrite(bounded_queue, &data);  // Zawsze się udaje!
}

// Strategia 4: Peek (podgląd bez pobierania)
void peek_example(void) {
    int data;
    if (xQueuePeek(bounded_queue, &data, 0) == pdTRUE) {
        printf("Peeked: %d (still in queue)\n", data);
    }
}

// Strategia 5: Count i dostępność
void check_queue_status(void) {
    UBaseType_t waiting = uxQueueMessagesWaiting(bounded_queue);
    UBaseType_t spaces = uxQueueSpacesAvailable(bounded_queue);

    printf("Items: %lu, Spaces: %lu\n", waiting, spaces);
}

void vFastProducer(void *pvParameters) {
    int counter = 0;

    while (1) {
        counter++;

        // Sprawdź czy jest miejsce
        if (uxQueueSpacesAvailable(bounded_queue) > 0) {
            xQueueSend(bounded_queue, &counter, 0);
        } else {
            printf("Dropped: %d\n", counter);
        }

        vTaskDelay(pdMS_TO_TICKS(10));  // Szybki producent
    }
}

void vSlowConsumer(void *pvParameters) {
    int data;

    while (1) {
        xQueueReceive(bounded_queue, &data, portMAX_DELAY);
        printf("Consumed: %d\n", data);

        vTaskDelay(pdMS_TO_TICKS(100));  // Wolny konsument
    }
}

int main(void) {
    bounded_queue = xQueueCreate(5, sizeof(int));

    xTaskCreate(vFastProducer, "Producer", 128, NULL, 1, NULL);
    xTaskCreate(vSlowConsumer, "Consumer", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

### 4.2 Obserwacja

```
Consumed: 1
Consumed: 2
Consumed: 3
Dropped: 15   ← Kolejka pełna, dane tracone
Dropped: 16
Dropped: 17
Consumed: 4
...
```

---

## Zadanie 5: Przesyłanie wskaźników (20 min)

### 5.1 Kiedy używać wskaźników?

```
Kiedy NIE:
- Małe dane (< 4-8 bajtów) → kopiuj bezpośrednio

Kiedy TAK:
- Duże struktury (kopiowanie kosztowne)
- Zmienne dane (aktualizowane przez sender)
- Złożone obiekty (z pod-strukturami)
```

### 5.2 Implementacja

```c
typedef struct {
    int id;
    char name[32];
    double values[100];
    size_t value_count;
} SensorData;

QueueHandle_t pointer_queue;

// Pool pre-alokowanych danych
#define POOL_SIZE 10
SensorData data_pool[POOL_SIZE];
bool pool_used[POOL_SIZE] = {false};

SensorData* allocate_data(void) {
    for (int i = 0; i < POOL_SIZE; i++) {
        if (!pool_used[i]) {
            pool_used[i] = true;
            return &data_pool[i];
        }
    }
    return NULL;  // Pool exhausted
}

void free_data(SensorData* data) {
    int index = data - data_pool;
    if (index >= 0 && index < POOL_SIZE) {
        pool_used[index] = false;
    }
}

void vProducerTask(void *pvParameters) {
    while (1) {
        SensorData* data = allocate_data();

        if (data != NULL) {
            // Wypełnij dane
            data->id = rand();
            snprintf(data->name, sizeof(data->name), "Sensor_%d", data->id);
            data->value_count = 10;
            for (int i = 0; i < 10; i++) {
                data->values[i] = (double)rand() / RAND_MAX;
            }

            // Wyślij WSKAŹNIK
            xQueueSend(pointer_queue, &data, portMAX_DELAY);
            printf("Sent data pointer: %p\n", (void*)data);
        } else {
            printf("Pool exhausted!\n");
        }

        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vConsumerTask(void *pvParameters) {
    SensorData* received_data;

    while (1) {
        // Odbierz WSKAŹNIK
        if (xQueueReceive(pointer_queue, &received_data, portMAX_DELAY) == pdTRUE) {
            printf("Received: id=%d, name=%s, values[0]=%.2f\n",
                   received_data->id,
                   received_data->name,
                   received_data->values[0]);

            // Zwolnij do pool
            free_data(received_data);
        }
    }
}

int main(void) {
    // Kolejka przechowuje WSKAŹNIKI do SensorData
    pointer_queue = xQueueCreate(POOL_SIZE, sizeof(SensorData*));

    xTaskCreate(vProducerTask, "Producer", 128, NULL, 1, NULL);
    xTaskCreate(vConsumerTask, "Consumer", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

---

## Zadanie Bonus: Stream Buffer

FreeRTOS ma alternatywne mechanizmy:

```c
#include "stream_buffer.h"

StreamBufferHandle_t stream_buffer;

void vStreamProducer(void *pvParameters) {
    char message[] = "Hello from stream!";

    while (1) {
        xStreamBufferSend(stream_buffer, message, sizeof(message), portMAX_DELAY);
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

void vStreamConsumer(void *pvParameters) {
    char buffer[64];
    size_t received;

    while (1) {
        received = xStreamBufferReceive(stream_buffer, buffer, sizeof(buffer), portMAX_DELAY);
        if (received > 0) {
            buffer[received] = '\0';
            printf("Stream: %s\n", buffer);
        }
    }
}

int main(void) {
    // Stream buffer: variable length, byte-oriented
    stream_buffer = xStreamBufferCreate(256, 1);  // 256 bytes, trigger 1 byte

    xTaskCreate(vStreamProducer, "Producer", 128, NULL, 1, NULL);
    xTaskCreate(vStreamConsumer, "Consumer", 128, NULL, 1, NULL);

    vTaskStartScheduler();
    return 0;
}
```

---

## Tabela API

| Funkcja | Opis |
|---------|------|
| `xQueueCreate(len, size)` | Utwórz kolejkę |
| `xQueueSend(queue, item, timeout)` | Wyślij (dodaj na końcu) |
| `xQueueSendToFront()` | Wyślij na początek |
| `xQueueReceive(queue, item, timeout)` | Odbierz |
| `xQueuePeek()` | Podgląd bez pobierania |
| `xQueueOverwrite()` | Nadpisz gdy pełna |
| `xQueueReset()` | Wyczyść kolejkę |
| `uxQueueMessagesWaiting()` | Liczba elementów |
| `uxQueueSpacesAvailable()` | Wolne miejsca |

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Podstawowa kolejka działa | 3 |
| 2 | Multiple producers | 3 |
| 3 | Queue Sets | 3 |
| 4 | Obsługa przepełnienia | 3 |
| 5 | Przesyłanie wskaźników | 3 |

---

## Sprawozdanie

1. Diagram komunikacji przez kolejki
2. Pomiary throughput (elementy/s)
3. Analiza strategii obsługi pełnej kolejki
4. Porównanie kopiowania danych vs wskaźników

---

## Pytania kontrolne

1. Czym różni się kolejka od semafora?
2. Kiedy użyć `xQueueSendToFront`?
3. Jak obsłużyć przepełnienie kolejki?
4. Dlaczego pre-alokacja jest lepsza niż malloc?
5. Co to jest Queue Set?