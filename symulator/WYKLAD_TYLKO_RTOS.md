# 03 — Architektura RTOS zamiast łatania (wykład / notatki)

> Cel: zamiast dopisywać „jeszcze jednego ifa” i „jeszcze jednej flagi” aż system się rozsypie, budujemy **architekturę**, która:
> - skaluje się (funkcje rosną, a nie robi się spaghetti),
> - jest deterministyczna (RTOS ≠ „magia”, tylko kontrolowane czasy),
> - jest testowalna (da się uruchomić logikę na PC lub w symulatorze),
> - jest odporna na błędy (obsługa wyjątków, watchdog, recovery),
> - jest czytelna dla kolejnej osoby (i dla Ciebie za 3 miesiące).

---

## 1) RTOS w pigułce — po co i kiedy

### 1.1. RTOS to nie „szybszy system”
RTOS (Real-Time OS) jest po to, żeby **przewidywać** zachowanie w czasie:
- **deterministyczne opóźnienia** (latencja),
- **gwarantowane terminy** (deadline),
- kontrola nad **priorytetami** i **zasobami**.

RTOS przydaje się, gdy masz:
- wiele równoległych „rzeczy do roboty” (komunikacja, czujniki, UI, logika),
- wymagania czasowe,
- sporo przerwań i zależności,
- potrzebę izolacji modułów.

Jeśli masz prosty projekt z jedną pętlą i 2–3 timerami — **bare metal + scheduler kooperacyjny** może być lepszy.

---

## 2) Źródło chaosu: „łatanie” zamiast projektu

Typowe objawy łatania:
- globalne flagi: `volatile bool rx_done;`
- ISR robi pół projektu („bo szybciej”)
- losowe opóźnienia: `HAL_Delay(10);` „żeby działało”
- brak kontraktów modułów (kto i kiedy woła API)
- kolejność inicjalizacji „magiczna”
- priorytety ustawione „na czuja”
- deadlocki / race condition „czasem się zdarza”

Lekarstwo: **architektura**.

---

## 3) Model wykonania w RTOS — podstawowe elementy

### 3.1. Task/Thread
Wątek (task) = funkcja z własnym stosem i priorytetem.

Ważne parametry:
- **priorytet**
- **stack size** (i monitoring)
- **czas CPU** (profilowanie)
- **stan**: Running/Ready/Blocked/Suspended

### 3.2. Scheduler
Najczęściej: **preemptive priority-based**:
- zawsze wygrywa najwyższy gotowy priorytet,
- tick (np. 1 ms) albo tickless.

Konsekwencja: jeśli dasz za wysoki priorytet i task nie blokuje się nigdy → zagłodzisz resztę.

### 3.3. ISR (przerwania)
Przerwanie ma być krótkie:
- złap zdarzenie,
- zapisz dane,
- obudź task (semafor/queue/notify),
- wyjdź.

**Zasada architektury:** ISR ≠ logika biznesowa.

### 3.4. Synchronizacja i komunikacja
- **Semaphore** (binary/counting) – sygnał „coś się stało”
- **Mutex** – ochrona zasobu (zwykle z *priority inheritance*)
- **Queue** – przesył danych (kopiowanych)
- **Event Group / Flags** – wiele bitów zdarzeń
- **Task notification** – szybki „mini-semafor/mini-queue” (w niektórych RTOS)

---

## 4) „Architektura” — czyli jak podzielić system, żeby nie bolało

### 4.1. Warstwy (layering)
Najprostszy i najbardziej praktyczny podział:

1. **BSP / HAL (sprzęt)**  
   GPIO, UART, SPI, I2C, DMA, timery — surowe sterowniki.
2. **Drivers (urządzenia)**  
   np. driver do czujnika IMU, driver do modemu, driver do wyświetlacza.
3. **Services (usługi systemowe)**  
   logowanie, storage, komunikacja, time, diagnostyka.
4. **Application (logika)**  
   stany, reguły, algorytmy, scenariusze.

**Reguła zależności:** wyższe warstwy nie powinny wchodzić w szczegóły sprzętu.  
Najlepiej: Application nie widzi HAL.

### 4.2. Moduły z kontraktem
Każdy moduł ma:
- `init()`
- API publiczne (`start()`, `set_mode()`, `send()`…)
- jasny model wątków: kto wywołuje funkcje? czy są thread-safe?
- opis: **kto jest właścicielem danych**

---

## 5) Topologie architektury RTOS (wybór stylu)

### 5.1. „One task per feature” (częsty błąd)
Dużo tasków „bo RTOS to multitasking”. Skutki:
- większe zużycie RAM (stacki),
- trudniej ogarnąć priorytety,
- więcej synchronizacji (więcej bugów),
- jitter rośnie.

**Nie znaczy, że to zawsze złe**, ale zwykle warto ograniczać liczbę tasków.

### 5.2. „Pipeline / producer-consumer”
ISR/driver produkuje dane → kolejka → task przetwarza → kolejka → dalej.

Plusy:
- izolacja,
- łatwy backpressure (kolejka pełna → wiesz gdzie problem),
- testowalność.

### 5.3. „Event-driven / active object”
Jeden (lub kilka) tasków działa jako „event loop”:
- odbiera zdarzenia z kolejki,
- wykonuje krótką logikę,
- deleguje ciężkie rzeczy do workerów.

To jest **antidotum na 100 flag**.

### 5.4. „State machine”
Dla logiki urządzenia (tryby, stany awarii) — must have.
Zamiast:
- `if (a && b && !c && timer>...) ...`
robisz:
- stany: INIT → IDLE → RUN → ERROR → RECOVERY
- zdarzenia: `EV_START`, `EV_TIMEOUT`, `EV_SENSOR_FAIL`

---

## 6) Złota zasada: *ISR łapie, task robi*

### 6.1. Przykład: UART RX + DMA
- ISR od DMA/IDLE: tylko oznacza „ramka gotowa” i wysyła wskaźnik/rozmiar do kolejki.
- Task `comm_task`: parsuje protokół, waliduje CRC, generuje eventy.

**Pseudo-C (schemat):**
```c
typedef struct {
  uint8_t *buf;
  size_t len;
} rx_frame_t;

QueueHandle_t q_rx;

void USARTx_IRQHandler(void) {
  BaseType_t woken = pdFALSE;

  if (uart_idle_detected()) {
    size_t len = dma_get_rx_len();
    rx_frame_t f = { .buf = rx_buf, .len = len };
    xQueueSendFromISR(q_rx, &f, &woken);
    dma_restart_rx();
  }

  portYIELD_FROM_ISR(woken);
}

void comm_task(void *arg) {
  rx_frame_t f;
  for (;;) {
    if (xQueueReceive(q_rx, &f, portMAX_DELAY) == pdTRUE) {
      parse_frame(f.buf, f.len);
    }
  }
}
```plaintext

**Zysk architektoniczny:** komunikacja jest przewidywalna, ISR nie „puchnie”.

---

## 7) Priorytety: jak ustawiać, żeby nie zgadywać

### 7.1. Reguła „deadline-first” w praktyce
Nadaj priorytety wg wymagań czasowych:
- najwyżej: bardzo krótkie, krytyczne w czasie wątki (np. sterowanie silnikiem)
- średnio: komunikacja realtime
- niżej: UI, logowanie
- najniżej: housekeeping, statystyki

### 7.2. Uważaj na priorytetowe „korki”
Jeśli task o wysokim priorytecie:
- bierze mutex,
- robi dużo roboty,
to zablokuje inne.

**Zasada:** wysoko-priorytetowe taski powinny:
- robić mało,
- często się blokować,
- delegować.

---

## 8) Priority inversion (odwrócenie priorytetów) i mutexy

### 8.1. Co to jest
Task Low trzyma mutex.
Task High chce mutex → czeka.
Task Medium nie potrzebuje mutexa → działa i zagłodzi Low.
High stoi, bo Low nie dostaje CPU, by zwolnić mutex.

### 8.2. Leczenie
- mutexy z **priority inheritance** (większość RTOS ma),
- krótkie sekcje krytyczne,
- unikanie blokad w taskach high-priority,
- rozdział zasobów (zamiast jednego globalnego locka).

---

## 9) Pamięć: stacki, heap, alokacje

### 9.1. Stack per task
Błędy stosu są podstępne. Rób:
- watermark / stack high water mark,
- overflow hook,
- sensowny zapas.

### 9.2. Heap i alokacje dynamiczne
Dynamiczna alokacja w embedded jest ryzykowna:
- fragmentacja,
- niedeterministyczne czasy.

**Architektura zamiast łatania:**
- prealokuj bufory (pool),
- statyczne kolejki/bufory,
- „zero malloc w runtime” (często dobra polityka).

---

## 10) Time: tick, timery, opóźnienia

### 10.1. `delay()` jako antywzorzec
Jeśli w tasku robisz `delay(50ms)` „żeby coś poczekało”, to:
- ukrywasz zależności,
- wprowadzasz jitter,
- psujesz responsywność.

Zamiast tego:
- czekaj na event (semafor/queue),
- używaj timerów RTOS do timeoutów,
- stosuj „deadline scheduling” w pętli.

### 10.2. Timery RTOS
Timer callback działa często w kontekście **timer task**, więc:
- callback ma być krótki,
- nie blokować,
- zwykle tylko wysłać event.

---

## 11) Observability: logi, trace, metryki (żeby nie zgadywać)

Architektura bez obserwowalności kończy się „łataną telepatią”.

Minimum:
- logger z poziomami (INFO/WARN/ERR),
- timestamp,
- możliwość wyłączenia logów w release,
- zrzut diagnostyki przy błędzie (assert, hardfault, watchdog reset cause).

Dodatkowo:
- statystyki CPU per task,
- kolejki: max fill level,
- stack watermark.

---

## 12) Wzorce, które realnie porządkują system

### 12.1. „Command queue” (kolejka poleceń)
Każdy moduł ma własną kolejkę komend:
- `CMD_START`
- `CMD_STOP`
- `CMD_SET_PARAM`

Task modułu jest jedynym miejscem modyfikacji stanu.

Plusy:
- brak wyścigów,
- brak potrzeby locków na „stan”.

### 12.2. Pub/Sub (publish-subscribe)
Masz bus zdarzeń (np. `EV_TEMP_UPDATE`), a różne moduły subskrybują.
Uwaga: łatwo przesadzić i zrobić „magiczny global event bus”.

Dobra praktyka:
- ogranicz liczbę eventów,
- dokumentuj źródła i konsumentów,
- pilnuj kosztów kopiowania.

### 12.3. „Deferred work” / worker pool
Cięższe rzeczy (np. parsowanie, kompresja, zapis flash) idą do workerów.
Główna logika tylko zleca.

---

## 13) Integracja driverów i aplikacji: granice odpowiedzialności

### 13.1. Driver nie powinien decydować o logice
Driver:
- inicjalizuje hardware,
- wysyła/odbiera dane,
- zgłasza zdarzenia.

Aplikacja:
- decyduje co to znaczy „błąd czujnika”,
- jak zareagować,
- czy retry, fallback, degrade.

### 13.2. Kontrakty czasowe
Dobrze jest mieć spis:
- maksymalny czas obsługi eventu,
- maksymalną latencję dla krytycznych ścieżek,
- priorytety i ich uzasadnienie.

To jest „architektura”, nie „magia”.

---

## 14) Zarządzanie błędami: jak nie zamienić systemu w „zawieszacza”

### 14.1. Kategorie błędów
- *recoverable*: timeout, chwilowy brak danych → retry/backoff
- *degraded*: czujnik padł → tryb awaryjny
- *fatal*: naruszenie integralności → reset/bezpieczny stan

### 14.2. Watchdog + strategia restartu
Watchdog jest OK, ale:
- reset bez diagnostyki = łatanie objawów
- loguj powód resetu, stan modułów, licznik restartów

Przykład polityki:
- 1 reset → normalnie
- 3 reset w 5 min → wejdź w safe mode (np. ogranicz funkcje)

---

## 15) Testowalność: architektura „pod testy”

### 15.1. Separacja logiki od RTOS
Jeśli logika biznesowa wymaga RTOS do testów, to boli.

Lepiej:
- logika jako funkcje „pure” + struktura stanu,
- warstwa RTOS tylko dostarcza eventy i timery.

### 15.2. HAL jako interfejs
Zamiast `HAL_UART_Transmit()` w logice:
- daj interfejs `ICommTransport.send()`
- w testach podstawiasz mock.

---

## 16) Minimalny „szkielet” projektu RTOS (spójny i skalowalny)

Propozycja (praktyczna, typowa):

- `main()`:
  - init clock, basic HAL
  - init logger
  - init services/drivers
  - create tasks + queues
  - start scheduler

- Taski:
  1. **app_task (event loop)** – trzyma stan aplikacji + state machine
  2. **comm_task** – I/O i protokoły
  3. **io_task / sensor_task** – cykliczne odczyty, filtracja
  4. **worker_task** – ciężkie operacje (flash, kompresja)
  5. **idle** – standard

Komunikacja:
- driver → queue → comm_task → event → app_task
- app_task → command queue → moduły

**Efekt:** mało locków, mało globali, jasne przepływy.

---

## 17) Checklista: „architektura zamiast łatania”

**Jeśli to spełniasz, jesteś po dobrej stronie mocy:**
- [ ] ISR tylko sygnalizuje, nie „robi projekt”
- [ ] każdy moduł ma jedną odpowiedzialność i kontrakt
- [ ] zdefiniowane priorytety + uzasadnienie
- [ ] brak `delay()` jako „fix”
- [ ] mało globali, mało flag, dużo komunikacji przez kolejki/eventy
- [ ] obserwowalność: logi + metryki + zrzuty przy błędach
- [ ] strategia błędów + watchdog z diagnostyką
- [ ] polityka pamięci (stack/heap) i pomiary watermarków
- [ ] testowalność: logika odseparowana od HAL/RTOS

---

## 18) Mini-zadanie (żeby to weszło w krew)
Weź dowolny fragment systemu (np. „obsługa czujnika + reakcja na błąd”) i przepisz go na:
1. driver czujnika (tylko I/O + event),
2. task czujnika (kolejka komend, cykliczny odczyt),
3. app state machine (decyduje co robić przy błędzie).

Zapisz:
- jakie eventy lecą,
- jakie są timeouty,
- jaki priorytet i dlaczego.

---

Jeśli chcesz, dopasuję to do Twojego środowiska: **FreeRTOS / Zephyr / ThreadX / CMSIS-RTOS2** i do przykładu (UART, BLE, CAN, silniki, UI). Podeślij tylko:
1) jakiego RTOS używasz, 2) MCU, 3) jakie moduły ma projekt i które dziś są „połatane”.