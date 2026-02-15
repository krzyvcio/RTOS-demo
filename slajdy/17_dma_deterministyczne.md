# Deterministyczne DMA

## Definicja

Bezpośredni dostęp do pamięci z gwarantowaną latencją i przewidywalnym zachowaniem. Kluczowe dla systemów czasu rzeczywistego wymagających deterministycznego transferu danych bez obciążania CPU.

______________________________________________________________________

## Problem z tradycyjnym DMA

```
┌───────────────────────────────────────────────────────────────┐
│                    PROBLEM Z TRADYCYJNYM DMA                  │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  TRADYCYJNE DMA:                                              │
│                                                               │
│  CPU:  ──────┬─────────────┬─────────────┬──────────────     │
│              │             │             │                    │
│              │ Wywłaszczenie│ przez DMA   │                    │
│              │ (bus steal) │             │                    │
│              ▼             ▼             ▼                    │
│  DMA:  ██████████████████████████████████████████████████    │
│        └─────────────────────────────────────────────┘        │
│                     Transfer blokujący CPU                    │
│                                                               │
│  PROBLEMY:                                                    │
│  ├── Nieprzewidywalana latencja dostępu do pamięci          │
│  ├── Cache thrashing (DMA nie koherentne z cache)           │
│  ├── Bus contention (spory o szynę)                          │
│  └── Brak gwarancji bandwidth dla CPU                        │
│                                                               │
│  WYNIK:                                                       │
│  ├── Jitter w pętlach RT                                     │
│  ├── Deadline miss przy dużych transferach                   │
│  └── Trudna analiza WCET                                      │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Rozwiązania dla deterministycznego DMA

```
┌─────────────────────────────────────────────────────────────┐
│                    ROZWIĄZANIA                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. DMA Z PRIORYTETAMI                                      │
│     ┌─────────────────────────────────────────────────────┐│
│     │ Channel │ Priority │ Bandwidth │ Latency           ││
│     ├─────────┼──────────┼───────────┼────────────────────┤│
│     │ CH0     │ High     │ Reserved  │ Guaranteed         ││
│     │ CH1     │ Medium   │ Shared    │ Best effort        ││
│     │ CH2     │ Low      │ Shared    │ Background         ││
│     └─────────────────────────────────────────────────────┘│
│                                                             │
│  2. BANDWIDTH ALLOCATION                                    │
│     ├── Rezerwacja bandwidth per kanał                     │
│     ├── Token bucket algorithm                             │
│     └── QoS guarantees                                      │
│                                                             │
│  3. CACHE COHERENCY                                         │
│     ├── Hardware cache coherency (CCI)                     │
│     ├── Software-managed cache flush                       │
│     └── Non-cacheable buffers dla DMA                      │
│                                                             │
│  4. DEDICATED MEMORY                                        │
│     ├── TCM (Tightly Coupled Memory)                       │
│     ├── SRAM dedicated dla DMA                              │
│     └── Brak contention z główną pamięcią                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Automotive (ADAS)

| Transfer | Typ | Wymagania |
|----------|-----|-----------|
| Camera → Memory | Continuous | 1.5 GB/s, bounded latency |
| Radar → Memory | Burst | 100 MB/s, deterministic |
| Sensor Fusion | Scatter-Gather | Low latency |

### Aerospace (AFDX)

- **Network packets:** DMA z priorytetami
- **Sensor data:** Guaranteed bandwidth
- **Logging:** Best effort

### Industrial (EtherCAT)

- **Cyclic data:** Deterministic DMA
- **Acyclic data:** Background DMA
- **Safety:** Highest priority DMA

______________________________________________________________________

## Implementacja

### DMA z priorytetami (STM32)

```c
// Konfiguracja DMA z priorytetami
typedef struct {
    DMA_Stream_TypeDef *stream;
    uint32_t channel;
    uint32_t priority;      // Very High, High, Medium, Low
    uint32_t bandwidth;     // Reserved bandwidth in bytes/sec
} dma_config_t;

void dma_init_deterministic(void) {
    // Kanał krytyczny (Camera) - najwyższy priorytet
    DMA2_Stream0->CR = DMA_SxCR_CHSEL_0 |      // Channel 0
                       DMA_SxCR_PL_1 |          // Very High priority
                       DMA_SxCR_TCIE |          // Transfer complete interrupt
                       DMA_SxCR_MINC |          // Memory increment
                       DMA_SxCR_PINC;           // Peripheral increment

    // Kanał średni (Radar) - średni priorytet
    DMA2_Stream1->CR = DMA_SxCR_CHSEL_1 |
                       DMA_SxCR_PL_0 |          // High priority
                       DMA_SxCR_TCIE |
                       DMA_SxCR_MINC;

    // Kanał tła (Logging) - niski priorytet
    DMA2_Stream2->CR = DMA_SxCR_CHSEL_2 |
                       0 |                      // Low priority
                       DMA_SxCR_TCIE |
                       DMA_SxCR_MINC;
}

// Konfiguracja bandwidth limiting (jeśli dostępne)
void dma_set_bandwidth(DMA_Stream_TypeDef *stream, uint32_t bytes_per_sec) {
    // Niektóre DMA mają wbudowany bandwidth limiter
    // Dla STM32H7: DMA_BNDTR (Bandwidth Control)
    #ifdef DMA_BNDTR_BNDT
    uint32_t tokens = bytes_per_sec / DMA_CLOCK_HZ;
    stream->BNDTR = DMA_BNDTR_BNDT(tokens);
    #endif
}
```

### Deterministyczny transfer

```c
// Double buffering z DMA
#define BUFFER_SIZE  1024
static uint8_t buffer_a[BUFFER_SIZE] __attribute__((section(".dma_mem")));
static uint8_t buffer_b[BUFFER_SIZE] __attribute__((section(".dma_mem")));
static volatile bool use_buffer_a = true;

void dma_start_transfer(void) {
    uint8_t *active_buffer = use_buffer_a ? buffer_a : buffer_b;

    // Upewnij się, że cache jest spójny
    #ifdef DCACHE_ENABLED
    SCB_CleanDCache_by_Addr((uint32_t*)active_buffer, BUFFER_SIZE);
    #endif

    // Konfiguruj DMA
    DMA1_Stream0->PAR = (uint32_t)&PERIPHERAL->DR;
    DMA1_Stream0->M0AR = (uint32_t)active_buffer;
    DMA1_Stream0->NDTR = BUFFER_SIZE;
    DMA1_Stream0->CR |= DMA_SxCR_EN;
}

void DMA1_Stream0_IRQHandler(void) {
    if (DMA1->LISR & DMA_LISR_TCIF0) {
        DMA1->LIFCR = DMA_LIFCR_CTCIF0;

        // Przełącz bufor
        use_buffer_a = !use_buffer_a;

        // Przetwórz dane (deterministycznie)
        process_buffer(use_buffer_a ? buffer_b : buffer_a);

        // Rozpocznij następny transfer
        dma_start_transfer();
    }
}
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Bus contention** | Spory o szynę | Priorytetyzacja DMA |
| **Cache coherency** | Nieaktualne dane | Cache flush, coherency |
| **Bandwidth starvation** | Brak bandwidth | Rezerwacja, QoS |
| **Interrupt storm** | Zbyt częste przerwania DMA | Double buffering |
| **Alignment** | Niepoprawne wyrównanie | Sprawdzenie alignment |

______________________________________________________________________

## Kierunki rozwoju

### 1. Smart DMA (Intel)

- DMA z wbudowanym procesorem
- Przetwarzanie danych w locie
- Offload z CPU

### 2. Cache-Coherent DMA

- Hardware cache coherency
- CCI (Cache Coherent Interconnect)
- ARM ACE protocol

### 3. QoS-Aware DMA

- Per-channel bandwidth reservation
- Latency guarantees
- Integration z TSN

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj system DMA dla kamery ADAS:

WYMAGANIA:
├── Rozdzielczość: 1920x1080, 30 fps
├── Format: RAW10 (10 bit/pixel)
├── Latencja max: 5 ms od ostatniego piksela
└── CPU dostęp do pamięci: min 50% bandwidth

PYTANIA:
1. Jaki bandwidth wymaga kamera?
2. Jak zapewnić determinizm?
3. Jak zarządzać cache?

ROZWIĄZANIE:
1. Bandwidth:
   - 1920 × 1080 × 10 bits × 30 fps = 622 Mbps ≈ 78 MB/s

2. Determinizm:
   - Priorytet Very High dla DMA kamery
   - Double buffering (2 × 2.6 MB)
   - Rezerwacja 100 MB/s dla kamery
   - Pozostałe 50% dla CPU

3. Cache:
   - Bufory DMA w niecache'owanej pamięci
   - Lub: cache flush przed/przed DMA
   - Cache coherency jeśli dostępne
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego tradycyjne DMA jest problematyczne w systemach RT?
1. Jakie są metody zapewnienia determinizmu DMA?
1. Jak zarządzać cache coherency z DMA?
1. Czym jest bandwidth allocation w DMA?

______________________________________________________________________

## Literatura

1. Xilinx, "AXI DMA Engine Reference Guide"
1. Intel, "Intel Smart DMA Technology"
1. ARM, "AMBA DMA Controller Technical Reference Manual"
1. STMicroelectronics, "STM32H7 DMA Application Note"
