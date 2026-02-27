# Jak połączyć LLM z RTOS bez utraty determinizmu?

Łączenie LLM z RTOS bez utraty determinizmu to złożone wyzwanie architektoniczne, ponieważ:

* **LLM** są z natury **niedeterministyczne** (losowe próbkowanie, zmienny czas inferencji),
* **RTOS** wymaga **gwarancji czasowych** i przewidywalnego zachowania (WCET, jitter, deadlines).

Poniżej znajdują się **sprawdzone strategie** oraz **przykładowa architektura produkcyjna**.

---

## 1. Separacja warstw czasowych

```text
┌─────────────────────────────────────┐
│  Hard Real-Time (RTOS kernel)       │  ← Krytyczne zadania, ISR, sterowanie
├─────────────────────────────────────┤
│  Soft Real-Time (middleware)        │  ← Komunikacja, buforowanie
├─────────────────────────────────────┤
│  Best-Effort (LLM inference)        │  ← AI, bez gwarancji czasowych
└─────────────────────────────────────┘
```

* LLM działa w osobnym wątku/rdzeniu o niższym priorytecie.
* Komunikacja przez **lock-free queues** (np. ring buffer).

---

## 2. Deterministyczne dekodowanie

* Użyj **greedy decoding** (`temperature = 0`) zamiast sampling.
* Ustaw `max_tokens` na **stałą wartość**.
* Rozważ **speculative decoding** z budżetem czasowym.

---

## 3. Budżet czasowy (time-boxing)

```c
// Pseudokod RTOS
void llm_task(void *params) {
    while (1) {
        TickType_t start = xTaskGetTickCount();

        // Inferencja z timeoutem
        result = llm_inference_with_timeout(input, MAX_INFERENCE_MS);

        if (result == TIMEOUT) {
            use_fallback_response();  // Deterministyczna odpowiedź zapasowa
        }

        vTaskDelayUntil(&start, FIXED_PERIOD_MS);
    }
}
```

---

## 4. Edge AI / TinyML

* Małe modele: **TinyLlama, Phi-3-mini, Gemma 2B**
* Kwantyzacja: **INT8 / INT4** (GGML, TensorRT-LLM)
* **Pruning + distillation** dla przewidywalnego czasu wykonania

---

## 5. Architektura producent–konsument

```text
RTOS Task (hard RT)     LLM Task (soft RT)
      │                       │
      ▼                       ▼
  ┌───────┐   request    ┌────────┐
  │ Queue │──────────────│ LLM    │
  │ (RT)  │◄─────────────│ Worker │
  └───────┘   response   └────────┘
      │
      ▼
  Deterministyczna decyzja
  (timeout = fallback)
```

---

## 6. Precomputed responses

* Dla krytycznych scenariuszy: **cache offline**
* LLM generuje tylko w trybie **idle**
* Wyniki trafiają do **lookup table**

---

## 7. Dual-core split (np. ESP32, RP2040, nRF5340)

```text
Core 0            Core 1
RTOS + kontrola   LLM inference
Hard real-time    Best-effort
Komunikacja przez shared memory / IPC
```

---

## Podsumowanie kluczowych zasad

| Wymóg               | Rozwiązanie                         |
| ------------------- | ----------------------------------- |
| Determinizm czasowy | Time-boxing + fallback              |
| Determinizm wyjścia | `temperature=0`, `top_k=1`          |
| Izolacja            | Osobny wątek/rdzeń, niski priorytet |
| Wydajność           | Kwantyzacja, małe modele            |

---

# Konkretna implementacja: FreeRTOS + llama.cpp (GGML)

Target: **ESP32-S3 (dual-core, PSRAM)**

## Struktura projektu

```text
llm_rtos_project/
├── main/
│   ├── main.c
│   ├── rtos_core.c
│   ├── llm_worker.c
│   ├── comm_bridge.c
│   └── fallback_engine.c
├── components/
│   └── llama_esp/
└── models/
    └── tinyllama-1.1b-q4.gguf
```

---

## 1. Komunikacja międzywątkowa (`comm_bridge.h`)

```c
#ifndef COMM_BRIDGE_H
#define COMM_BRIDGE_H

#include "freertos/FreeRTOS.h"
#include "freertos/queue.h"
#include "freertos/semphr.h"

#define MAX_PROMPT_LEN    256
#define MAX_RESPONSE_LEN  512
#define LLM_TIMEOUT_MS    2000

typedef enum {
    REQUEST_PENDING,
    REQUEST_PROCESSING,
    RESPONSE_READY,
    RESPONSE_TIMEOUT,
    RESPONSE_FALLBACK
} RequestStatus;

typedef struct {
    uint32_t request_id;
    char prompt[MAX_PROMPT_LEN];
    TickType_t deadline;
    uint8_t priority;
} LLMRequest;

typedef struct {
    uint32_t request_id;
    RequestStatus status;
    char response[MAX_RESPONSE_LEN];
    uint32_t inference_time_ms;
    uint8_t tokens_generated;
} LLMResponse;

#endif
```

---

## 2. Diagram czasowy

```text
Core 0 (Hard RT)          Core 1 (Soft RT)          Czas
────────────────          ────────────────          ────
│ ctrl_loop │             │              │           0ms
│ sensors   │             │  LLM idle    │
│ PID       │────req────► │ tokenize     │          10ms
│ output    │             │ generate     │          ...
├───────────┤             │ token 1..n   │
│ ctrl_loop │ ◄──resp──── │ done         │        ~1500ms
│ apply LLM │             │ LLM idle     │
└───────────┘             └──────────────┘
```

**Gwarancja:** pętla sterowania **zawsze co 10 ms**, niezależnie od LLM.

---

## Kluczowe gwarancje architektury

| Właściwość              | Gwarancja                          |
| ----------------------- | ---------------------------------- |
| Timing control loop     | 10 ms ± jitter RTOS                |
| Determinizm wyjścia LLM | `temp=0`, `top_k=1`                |
| Brak blokowania RT      | Non-blocking IPC + izolacja rdzeni |
| Zawsze działa           | PID fallback przy timeout          |
| WCET LLM                | Limit czasu → ucięcie generacji    |

---

# Implementacja: Zephyr RTOS + LLM (llama.cpp)

Zephyr daje:

* SMP + CPU affinity
* Workqueues
* Kconfig / DeviceTree
* Dobre statystyki czasu rzeczywistego

---

## Struktura projektu

```text
llm_zephyr/
├── CMakeLists.txt
├── prj.conf
├── app.overlay
├── Kconfig
├── src/
│   ├── main.c
│   ├── rt_core.c
│   ├── llm_subsys.c
│   ├── ipc_bridge.c
│   └── fallback.c
├── include/
│   ├── llm_subsys.h
│   ├── ipc_bridge.h
│   └── app_config.h
└── modules/
    └── llama_zephyr/
```

---

## Gwarancje czasowe w Zephyr

| Element             | Gwarancja | Mechanizm Zephyr              |
| ------------------- | --------- | ----------------------------- |
| Control loop jitter | < 50 µs   | `k_timer` + CPU affinity      |
| Izolacja LLM        | 100%      | `CONFIG_SMP` + CPU mask       |
| Deadline monitoring | Tak       | `k_uptime_ticks()`            |
| Determinizm wyjścia | 100%      | `temp=0` (Kconfig)            |
| Memory safety       | Tak       | `CONFIG_HW_STACK_PROTECTION`  |
| Statystyki RT       | Runtime   | `CONFIG_THREAD_RUNTIME_STATS` |

