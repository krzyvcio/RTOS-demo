// ============================================================================
// RTOS HAZARDS - C EXAMPLES (FreeRTOS / Zephyr Style)
// ============================================================================
// Krytyczne przypadki niskopoziomowe z wstawkami Assemblera
// ============================================================================

#ifndef RTOS_HAZARDS_H
#define RTOS_HAZARDS_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// ============================================================================
// INDEKS PRZYPADKÓW ZAGROŻEŃ (C)
// ============================================================================

/*
KATEGORIA: SYNCHRONIZACJA (50 przypadków C)
============================================

#C_SYNC_001   - Priority Inversion z mutex
#C_SYNC_002   - Deadlock z pthread_mutex
#C_SYNC_003   - Race condition na zmiennej globalnej
#C_SYNC_004   - Spinlock w single-core (waste)
#C_SYNC_005   - Barrier deadlock
#C_SYNC_006   - Condition variable bez predykatu
#C_SYNC_007   - Semaphore overflow
#C_SYNC_008   - Recursive mutex misuse
#C_SYNC_009   - Priority ceiling violation
#C_SYNC_010   - Lock order violation

KATEGORIA: ISR (40 przypadków C)
=================================

#C_ISR_001    - ISR reentrancy issue
#C_ISR_002    - ISR blocking on mutex (FATAL!)
#C_ISR_003    - ISR calling blocking API
#C_ISR_004    - ISR stack overflow
#C_ISR_005    - Nested ISR overflow
#C_ISR_006    - ISR latency violation
#C_ISR_007    - ISR priority inversion
#C_ISR_008    - ISR race with main
#C_ISR_009    - ISR missing volatile
#C_ISR_010    - ISR DMA coherency

KATEGORIA: PAMIĘĆ (40 przypadków C)
====================================

#C_MEM_001    - Stack overflow (recursion)
#C_MEM_002    - Heap fragmentation
#C_MEM_003    - Buffer overflow
#C_MEM_004    - Use after free
#C_MEM_005    - Double free
#C_MEM_006    - Memory leak in ISR
#C_MEM_007    - Unaligned access
#C_MEM_008    - Cache coherency issue
#C_MEM_009    - DMA buffer ownership
#C_MEM_010    - Stack canary bypass

KATEGORIA: TIMING (40 przypadków C)
====================================

#C_TIME_001   - Deadline miss detection
#C_TIME_002   - Jitter measurement
#C_TIME_003   - WCET violation
#C_TIME_004   - Tick overflow
#C_TIME_005   - Timer drift
#C_TIME_006   - Watchdog timeout
#C_TIME_007   - Brown-out handling
#C_TIME_008   - Clock glitch
#C_TIME_009   - NMI storm
#C_TIME_010   - Power mode transition

KATEGORIA: HARDWARE (30 przypadków C)
======================================

#C_HW_001     - MPU violation
#C_HW_002     - Bus fault
#C_HW_003     - Hard fault handler
#C_HW_004     - Peripheral lockup
#C_HW_005     - DMA channel conflict
#C_HW_006     - GPIO race condition
#C_HW_007     - UART overrun
#C_HW_008     - SPI CRC error
#C_HW_009     - I2C arbitration lost
#C_HW_010     - CAN bus-off
*/

// ============================================================================
// #C_SYNC_001 - PRIORITY INVERSION z MUTEX (FreeRTOS Style)
// ============================================================================
//
// OPIS: Klasyczny problem inwersji priorytetów
//
// SCENARIUSZ:
// 1. Zadanie NISKIE trzyma mutex
// 2. Zadanie WYSOKIE chce mutex → blokuje się
// 3. Zadanie ŚREDNIE (nie potrzebuje mutex) wywłaszcza NISKIE
// 4. WYSOKIE czeka na ŚREDNIE → INWERSJA!
//
// NORMY: ISO 26262 ASIL-D, #PriorityInversion
// ============================================================================

#ifdef FREERTOS

#include "FreeRTOS.h"
#include "task.h"
#include "semphr.h"

static SemaphoreHandle_t xMutex;

// Zadanie niskiego priorytetu - trzyma mutex długo
void vLowPriorityTask(void *pvParameters) {
    (void)pvParameters;

    for (;;) {
        printf("[LOW] Czekam na mutex...\n");

        if (xSemaphoreTake(xMutex, portMAX_DELAY) == pdTRUE) {
            printf("[LOW] Mam mutex - pracuję 500ms\n");
            vTaskDelay(pdMS_TO_TICKS(500));  // Długa praca
            printf("[LOW] Zwalniam mutex\n");
            xSemaphoreGive(xMutex);
        }

        vTaskDelay(pdMS_TO_TICKS(100));
    }
}

// Zadanie średniego priorytetu - CPU bound
void vMediumPriorityTask(void *pvParameters) {
    (void)pvParameters;

    for (;;) {
        // Heavy computation - nie potrzebuje mutex
        volatile uint32_t i;
        for (i = 0; i < 1000000; i++) {
            __asm__ volatile("nop");
        }
        printf("[MED] Pracuję...\n");
        vTaskDelay(pdMS_TO_TICKS(50));
    }
}

// Zadanie wysokiego priorytetu - potrzebuje mutex
void vHighPriorityTask(void *pvParameters) {
    (void)pvParameters;
    TickType_t xStartTime;

    for (;;) {
        vTaskDelay(pdMS_TO_TICKS(200));

        printf("[HIGH] Chcę mutex!\n");
        xStartTime = xTaskGetTickCount();

        if (xSemaphoreTake(xMutex, portMAX_DELAY) == pdTRUE) {
            TickType_t xWaitTime = xTaskGetTickCount() - xStartTime;
            printf("[HIGH] Czekałem %lu ticks - INWERSJA!\n", xWaitTime);

            // Krytyczna sekcja
            vTaskDelay(pdMS_TO_TICKS(10));

            xSemaphoreGive(xMutex);
        }
    }
}

// Inicjalizacja demonstracji
void vCreatePriorityInversionDemo(void) {
    // Standard mutex - BEZ priority inheritance
    xMutex = xSemaphoreCreateMutex();

    xTaskCreate(vLowPriorityTask,    "LOW",  256, NULL, 1, NULL);
    xTaskCreate(vMediumPriorityTask, "MED",  256, NULL, 2, NULL);
    xTaskCreate(vHighPriorityTask,   "HIGH", 256, NULL, 3, NULL);
}

// ROZWIĄZANIE: Mutex z Priority Inheritance
void vCreatePriorityInversionSolution(void) {
    // Mutex Z Priority Inheritance
    xMutex = xSemaphoreCreateMutex();  // FreeRTOS domyślnie ma PI

    // Lub explicite (zależy od konfiguracji):
    // Ustaw configUSE_MUTEX_PRIORITY_INHERITANCE = 1 w FreeRTOSConfig.h

    xTaskCreate(vLowPriorityTask,    "LOW",  256, NULL, 1, NULL);
    xTaskCreate(vMediumPriorityTask, "MED",  256, NULL, 2, NULL);
    xTaskCreate(vHighPriorityTask,   "HIGH", 256, NULL, 3, NULL);
}

#endif // FREERTOS

// ============================================================================
// #C_ISR_001 - ISR REENTRANCY ISSUE
// ============================================================================
//
// OPIS: Problem reentrancy w ISR - funkcja wywołana z ISR nie może być
// bezpiecznie wywołana ponownie jeśli przerwanie nastąpi podczas jej trwania
//
// OBJAWY:
// - Losowe błędy danych
// - Stack corruption
// - System crash
//
// NORMY: #ISR, IEC 61508 SIL-3
// ============================================================================

// NIEBEZPIECZNA funkcja - nie jest reentrant!
static uint32_t global_buffer[32];  // Globalna zmienna = niebezpieczeństwo
static uint8_t buffer_index = 0;

void non_reentrant_isr_handler(void) {
    // Jeśli to przerwanie nastąpi ponownie przed zakończeniem,
    // global_buffer zostanie uszkodzony!
    for (int i = 0; i < 32; i++) {
        global_buffer[buffer_index++] = read_hardware_register();
    }
}

// ROZWIĄZANIE 1: Użyj zmiennych lokalnych
void reentrant_isr_handler(void) {
    uint32_t local_buffer[32];  // Lokalna na stacku - bezpieczne
    uint8_t local_index = 0;

    for (int i = 0; i < 32; i++) {
        local_buffer[local_index++] = read_hardware_register();
    }

    // Teraz bezpiecznie kopiujemy do globalnej
    memcpy(global_buffer, local_buffer, sizeof(local_buffer));
}

// ROZWIĄZANIE 2: Wyłącz przerwania na czas krytycznej sekcji
void protected_isr_handler(void) {
    uint32_t primask = __get_PRIMASK();
    __disable_irq();  // Wyłącz przerwania

    // Krytyczna sekcja - bezpieczna przed reentrancy
    for (int i = 0; i < 32; i++) {
        global_buffer[buffer_index++] = read_hardware_register();
    }

    __set_PRIMASK(primask);  // Przywróć stan przerwań
}

// ============================================================================
// #C_ISR_002 - ISR BLOKING ON MUTEX (FATAL ERROR!)
// ============================================================================
//
// OPIS: Próba zajęcia mutex w ISR - to jest FATALNY błąd!
//
// OBJAWY:
// - System zawiesza się
// - Hard fault
// - Nieprzewidywalne zachowanie
//
// POWÓD: ISR nie może blokować (nie ma "kontekstu" do przełączenia)
//
// NORMY: #ISR, #ASIL-D, #DAL-A
// ============================================================================

// BŁĘDNA implementacja - NIGDY TAK NIE RÓB!
void bad_isr_with_mutex(void) {
    SemaphoreHandle_t xMutex = get_shared_mutex();

    // TO JEST BŁĄD! xSemaphoreTake może blokować w ISR!
    // if (xSemaphoreTake(xMutex, portMAX_DELAY) == pdTRUE) {  // CRASH!
    // }

    // FreeRTOS ma specialne API "FromISR"
    // Ale mutex NIE MOŻE być użyty w ISR!
}

// POPRAWNA implementacja - użyj "FromISR" APIs
void good_isr_handler(void) {
    BaseType_t xHigherPriorityTaskWoken = pdFALSE;

    // Zamiast mutex - użyj semaphore lub queue
    QueueHandle_t xQueue = get_data_queue();

    uint32_t data = read_hardware_register();

    // Special API dla ISR - nie blokuje!
    xQueueSendFromISR(xQueue, &data, &xHigherPriorityTaskWoken);

    // Jeśli zadanie o wyższym priorytecie zostało obudzone
    portYIELD_FROM_ISR(xHigherPriorityTaskWoken);
}

// ============================================================================
// #C_MEM_001 - STACK OVERFLOW (Recursion)
// ============================================================================
//
// OPIS: Stack overflow przez niekontrolowaną rekurencję
//
// OBJAWY:
// - Losowe crashe
// - Corrupted data
// - Hard fault
//
// DETEKCJA:
// - FreeRTOS stack watermark
// - MPU stack protection
// - Stack canaries
//
// NORMY: #StackOverflow, ISO 26262 ASIL-C
// ============================================================================

// NIEBEZPIECZNA funkcja rekurencyjna
void dangerous_recursion(int depth) {
    volatile char buffer[256];  // Każde wywołanie zużywa 256+ bajtów stacku
    memset(buffer, 0, sizeof(buffer));

    if (depth > 0) {
        dangerous_recursion(depth - 1);  // Rekursja!
    }
}

// Demonstracja stack overflow
void stack_overflow_demo(void) {
    // Przy stacku 1KB, to zawiedzie przy depth ~4
    dangerous_recursion(100);  // CRASH!
}

// ROZWIĄZANIE 1: Iteracja zamiast rekurencji
void safe_iteration(int depth) {
    while (depth > 0) {
        // Logika tutaj
        depth--;
    }
}

// ROZWIĄZANIE 2: Stack canary check
#define STACK_CANARY_VALUE 0xDEADBEEF

typedef struct {
    uint32_t canary;
    uint8_t data[1024];
    uint32_t canary_end;
} protected_stack_t;

bool check_stack_canary(protected_stack_t *stack) {
    if (stack->canary != STACK_CANARY_VALUE ||
        stack->canary_end != STACK_CANARY_VALUE) {
        // Stack overflow detected!
        trigger_safety_shutdown();
        return false;
    }
    return true;
}

// ============================================================================
// #C_MEM_003 - BUFFER OVERFLOW
// ============================================================================
//
// OPIS: Klasyczny buffer overflow - bezpieczeństwo i stabilność
//
// OBJAWY:
// - Corrupted memory
// - Crash
// - Code execution exploit
//
// NORMY: #BufferOverflow, CVE, ISO 21434
// ============================================================================

#include <string.h>
#include <stdio.h>

// PODATNA funkcja
void vulnerable_copy(const char *input) {
    char buffer[64];
    strcpy(buffer, input);  // Niebezpieczne - brak sprawdzenia długości!
    printf("Buffer: %s\n", buffer);
}

// Demonstracja overflow
void buffer_overflow_demo(void) {
    const char *attack = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
                        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
                        "AAAAAAAAAAAAAAAA";  // 120+ znaków
    vulnerable_copy(attack);  // OVERFLOW!
}

// ROZWIĄZANIE 1: strncpy z explicit length
void safe_copy_1(const char *input) {
    char buffer[64];
    strncpy(buffer, input, sizeof(buffer) - 1);
    buffer[sizeof(buffer) - 1] = '\0';  // Zawsze null-terminate!
    printf("Buffer: %s\n", buffer);
}

// ROZWIĄZANIE 2: snprintf
void safe_copy_2(const char *input) {
    char buffer[64];
    snprintf(buffer, sizeof(buffer), "%s", input);
    printf("Buffer: %s\n", buffer);
}

// ROZWIĄZANIE 3: strlcpy (BSD/macOS)
#ifdef __BSD__
void safe_copy_3(const char *input) {
    char buffer[64];
    strlcpy(buffer, input, sizeof(buffer));
    printf("Buffer: %s\n", buffer);
}
#endif

// ============================================================================
// #C_TIME_001 - DEADLINE MISS DETECTION
// ============================================================================
//
// OPIS: Wykrywanie przekroczenia deadline w zadaniu RT
//
// IMPLEMENTACJA:
// - Hardware timer
// - Watchdog per-task
// - Deadline monitoring
//
// NORMY: #DeadlineMiss, ISO 26262 ASIL-D
// ============================================================================

typedef struct {
    const char *name;
    TickType_t period_ms;
    TickType_t deadline_ms;
    TickType_t wcet_us;
    TickType_t last_start;
    uint32_t deadline_misses;
    uint32_t total_runs;
} task_metrics_t;

// Monitor deadline dla zadania
void task_start_timing(task_metrics_t *metrics) {
    metrics->last_start = xTaskGetTickCount();
}

void task_end_timing(task_metrics_t *metrics) {
    TickType_t elapsed = xTaskGetTickCount() - metrics->last_start;

    metrics->total_runs++;

    if (elapsed > pdMS_TO_TICKS(metrics->deadline_ms)) {
        metrics->deadline_misses++;

        // Loguj błąd krytyczny
        printf("[DEADLINE MISS] Task %s: %lu ms > %lu ms\n",
               metrics->name,
               elapsed * portTICK_PERIOD_MS,
               metrics->deadline_ms);

        // ASIL-D: Trigger safety action
        if (metrics->deadline_misses > 3) {
            enter_safe_state();
        }
    }
}

// Przykład użycia
void control_task(void *pvParameters) {
    task_metrics_t metrics = {
        .name = "ControlTask",
        .period_ms = 10,
        .deadline_ms = 8,
        .wcet_us = 5000,
    };

    for (;;) {
        task_start_timing(&metrics);

        // Kontrola silnika...
        read_sensors();
        compute_control();
        apply_actuators();

        task_end_timing(&metrics);

        vTaskDelayUntil(&(metrics.last_start), pdMS_TO_TICKS(metrics.period_ms));
    }
}

// ============================================================================
// #C_HW_003 - HARD FAULT HANDLER
// ============================================================================
//
// OPIS: Obsługa Hard Fault - krytyczne błędy CPU
//
// PRZYCZYNY:
// - Bus fault
// - Memory fault
// - Usage fault
// - MPU violation
//
// NORMY: #HardFault, IEC 61508, Cortex-M
// ============================================================================

// Cortex-M Hard Fault Handler
// Wymaga assembler dla dostępu do stack frame
__attribute__((naked))
void HardFault_Handler(void) {
    __asm__ volatile (
        "tst lr, #4                                    \n"
        "ite eq                                        \n"
        "mrseq r0, msp                                 \n"
        "mrsne r0, psp                                 \n"
        "b hard_fault_handler_c                        \n"
    );
}

// C handler dla Hard Fault
void hard_fault_handler_c(uint32_t *stack_frame) {
    // Stack frame zawiera:
    // stack_frame[0] = R0
    // stack_frame[1] = R1
    // stack_frame[2] = R2
    // stack_frame[3] = R3
    // stack_frame[4] = R12
    // stack_frame[5] = LR
    // stack_frame[6] = PC (adres błędu!)
    // stack_frame[7] = xPSR

    uint32_t pc = stack_frame[6];
    uint32_t lr = stack_frame[5];
    uint32_t cfsr = SCB->CFSR;  // Configurable Fault Status Register
    uint32_t hfsr = SCB->HFSR;  // Hard Fault Status Register
    uint32_t mmfar = SCB->MMFAR;  // Memory Management Fault Address
    uint32_t bfar = SCB->BFAR;  // Bus Fault Address

    // Loguj do backup registers (nieulotnych)
    backup_register_write(BACKUP_HF_PC, pc);
    backup_register_write(BACKUP_HF_LR, lr);
    backup_register_write(BACKUP_HF_CFSR, cfsr);

    // Analiza błędu
    printf("HARD FAULT:\n");
    printf("  PC: 0x%08lX\n", pc);
    printf("  LR: 0x%08lX\n", lr);
    printf("  CFSR: 0x%08lX\n", cfsr);

    if (cfsr & 0x01) {  // IACCVIOL - MPU violation
        printf("  MPU Instruction Access Violation\n");
        printf("  MMFAR: 0x%08lX\n", mmfar);
    }
    if (cfsr & 0x02) {  // DACCVIOL
        printf("  MPU Data Access Violation\n");
        printf("  MMFAR: 0x%08lX\n", mmfar);
    }
    if (cfsr & 0x0100) {  // UNSTKERR
        printf("  Unstacking Error\n");
    }

    // Krytyczne: enter safe state
    enter_safe_state();

    // Reset lub halt
    while (1) {
        __WFI();
    }
}

// ============================================================================
// POMOCNICZE FUNKCJE
// ============================================================================

// Stub functions dla demonstracji
static inline uint32_t read_hardware_register(void) { return 0x12345678; }
static inline SemaphoreHandle_t get_shared_mutex(void) { return NULL; }
static inline QueueHandle_t get_data_queue(void) { return NULL; }
static inline void enter_safe_state(void) { while(1) __WFI(); }
static inline void trigger_safety_shutdown(void) { while(1) __WFI(); }

// ARM Cortex-M helpers
#ifndef __get_PRIMASK
static inline uint32_t __get_PRIMASK(void) {
    uint32_t result;
    __asm__ volatile ("mrs %0, primask" : "=r" (result));
    return result;
}
#endif

#ifndef __set_PRIMASK
static inline void __set_PRIMASK(uint32_t value) {
    __asm__ volatile ("msr primask, %0" : : "r" (value));
}
#endif

#ifndef __disable_irq
static inline void __disable_irq(void) {
    __asm__ volatile ("cpsid i");
}
#endif

#endif // RTOS_HAZARDS_H