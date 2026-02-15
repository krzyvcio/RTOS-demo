# Latencja

## Definicja

**Latencja** to czas od momentu wystąpienia zdarzenia do momentu rozpoczęcia jego obsługi przez system.

> Latencja to "czas reakcji" - jak szybko system "zauważa", że coś się stało i zaczyna coś z tym robić.

```
Zdarzenie ────► Latencja ────► Rozpoczęcie obsługi ────► Wykonanie ────► Wynik
   │                               │
   │◄────────── Latencja ─────────►│
   │                               │
   (przerwanie)              (task start)
```

---

## Analogia do przyrody

### ⚡ Piorun i grzmot

Piorun jest natychmiastowy, ale grzmot słyszysz z opóźnieniem (latencją) - około 3 sekundy na każdy kilometr odległości.

**Co to nas uczy?**
- Latencja zależy od "medium" (powietrze dla dźwięku, sieć dla danych)
- Latencję można obliczyć i przewidzieć

### 🧠 Reakcja człowieka

Kiedy zobaczysz coś i naciśniesz przycisk:
- Czas reakcji wzrokowej: ~200ms
- Czas reakcji słuchowej: ~170ms

To jest Twoja "latencja systemowa". Kierowca wyścigowy ma latencję ~100ms. Systemy RTOS muszą mieć latencję w mikrosekundach.

### 🌊 Fala tsunami

Trzęsienie ziemi następuje natychmiast, ale fala przychodzi z latencją. Latencja = odległość / prędkość. Im dalej od źródła, tym większa latencja - ale też więcej czasu na reakcję!

---

## Podobieństwo do systemów informatycznych

### HTTP Request

```
Klik ──► DNS lookup ──► TCP handshake ──► SSL ──► HTTP request ──► Response
 │                                                   │
 │◄──────────── Latencja (TTFB) ───────────────────►│
```

Time To First Byte (TTFB) to latencja serwera. Użytkownik czeka, ale nic jeszcze nie "robi się" po stronie klienta.

### Kafka / Message Queue

```
Producer publikuje ──► [Queue] ──► Consumer odbiera
                            │
                     │◄─────┼─────►│
                       Latencja
```

Wiadomość w kolejce "czeka" na przetworzenie. To jest latencja queuing.

### SSD vs HDD

| Operacja | HDD | SSD |
|----------|-----|-----|
| Random read 4KB | ~10ms | ~0.1ms |
| Latencja | 100x gorsza | 100x lepsza |

HDD ma mechaniczną latencję (głowica musi się przesunąć). SSD jest elektroniczne - latencja jest znacznie mniejsza.

---

## Rodzaje latencji w RTOS

### 1. Interrupt Latency

Czas od sygnału przerwania do startu ISR (Interrupt Service Routine).

```
┌─────────────────────────────────────────────────────────┐
│                    INTERRUPT LATENCY                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Sygnał ──► Hardware detect ──► CPU finish current ──►  │
│                                 instruction             │
│                                                         │
│           ──► Context save ──► ISR start                │
│                                                         │
│  │◄──────────────────────────────────────────────►│     │
│                      Interrupt Latency                  │
└─────────────────────────────────────────────────────────┘
```

**Przyczyny zwiększonej latencji przerwań:**
- Wyłączone przerwania (critical section)
- Dłuższy ISR o wyższym priorytecie
- Cache miss przy wejściu do ISR

### 2. Scheduling Latency

Czas od "task gotowy" do "task uruchomiony".

```
Task staje się gotowy (np. semafor released)
              │
              ▼
        Scheduler decision
              │
              ▼
        Context switch
              │
              ▼
        Task starts executing

│◄────────────────────────────────►│
         Scheduling Latency
```

### 3. End-to-End Latency

Całkowity czas od zdarzenia do wyniku.

```
Przerwanie ──► ISR ──► Queue ──► Task ──► Output
    │                                        │
    │◄──────────── End-to-End ──────────────►│
```

---

## Dlaczego latencja jest problemem?

### Problem 1: Latencja się sumuje

```
Sensor ──(1ms)──► ADC ──(0.5ms)──► ISR ──(0.2ms)──► Queue
                                                      │
                          Queue ──(1ms)──► Task ──(2ms)──► Actuator

Total = 1 + 0.5 + 0.2 + wait + 1 + 2 = ?
```

Każdy element dodaje latencję. W systemie RT musisz policzyć **cały łańcuch**.

### Problem 2: Latencja nie jest stała (jitter)

```
Moment 1: Latencja = 5μs
Moment 2: Latencja = 50μs  (cache miss)
Moment 3: Latencja = 5μs
Moment 4: Latencja = 200μs (inny ISR)
```

To się nazywa **jitter** - wahania latencji. Więcej w osobnym pliku.

### Problem 3: Priority Inversion

Niskopriorytetowy task blokuje wysokopriorytetowy - latencja rośnie nieprzewidywalnie.

---

## Jak mierzyć latencję?

### Hardware: Logic Analyzer / Oscilloscope

```
GPIO toggle przy przerwaniu ──┐
                              │
              ┌───────────────┼───────────────┐
              │               │               │
           Ch1: Trigger    Ch2: ISR entry    Ch3: Task start
              │               │               │
              │◄──── Δt ─────►│               │
              │                               │
              │◄──────────── Δt ─────────────►│
```

### Software: Timestamping

```c
void ISR_Handler(void) {
    uint32_t entry_time = get_cycle_count();
    // ... obsługa ...
    uint32_t exit_time = get_cycle_count();

    log_latency(entry_time - trigger_time);
}
```

### Tools: Cyclictest (Linux)

```bash
# Mierzy scheduling latency w Linux
cyclictest -l100000 -m -Sp90 -i200 -h400 -q
```

---

## Jak sobie radzić z problemami latencji?

### Hardware solutions:

1. **Dedykowane przerwania** - oddzielne linie dla krytycznych sygnałów
2. **NVIC configuration** - konfiguracja kontrolera przerwań
3. **DMA** - Direct Memory Access omija CPU, zmniejsza latencję

### Software solutions:

```c
// ZŁE: Długa sekcja krytyczna
void bad_isr(void) {
    ENTER_CRITICAL();
    process_all_data();  // Długo!
    send_to_network();   // Bardzo długo!
    EXIT_CRITICAL();
}

// DOBRE: Minimalny ISR
void good_isr(void) {
    ENTER_CRITICAL();
    flag = true;         // Szybko!
    EXIT_CRITICAL();
    // Task zajmie się resztą
}
```

### Architectural solutions:

```
┌─────────────────────────────────────────────────────────┐
│                  ZERO-COPY ARCHITECTURE                 │
├─────────────────────────────────────────────────────────┤
│                                                         │
│   ISR ──► Wskaźnik do bufora ──► Task ──► Output       │
│           (brak kopiowania danych!)                     │
│                                                         │
│   Latencja = minimalna, bo brak memcpy()               │
└─────────────────────────────────────────────────────────┘
```

---

## Jak świat radzi sobie z latencją?

### Trading high-frequency (HFT)

Mikrosekundy decydują o milionach dolarów. Firmy lokują serwery w tym samym data center co giełda (co-location), żeby zminimalizować latencję sieciową.

**Ciekawostka**: Mikrofalówki są szybsze niż światłowody (mniejszy współczynnik załamania). Firmy budują własne mikrofalowe łącza.

### Gaming (e-sport)

Gracze profesjonalni używają:
- Myszki z 1000Hz polling rate (1ms latencja)
- Monitory 240Hz+ (4ms per frame)
- Klawiatury mechaniczne (szybsza reakcja)

Ludzkie oko+ mózg ma latencję ~13ms. Cały system: input → processing → display → human reaction ≈ 50-100ms.

### Automotive: Airbag

Latencja musi być < 30ms od wykrycia zderzenia do napełnienia poduszki. System jest całkowicie deterministyczny - osobny czujnik, osobny mikrokontroler, bezpośrednie połączenie z detonatorem.

---

## Pytania do przemyślenia

1. Jaka jest maksymalna latencja przerwania w Twoim systemie? Jak to mierzysz?
2. Który element w Twoim systemie ma największą latencję? Dlaczego?
3. Czy latencja Twojego systemu jest deterministyczna, czy ma jitter?

---

## Quiz

**Pytanie**: Masz system z latencją "średnio 10μs, max 500μs". Czy nadaje się do sterowania silnikiem pracującym na 20kHz (cykl 50μs)?

**Odpowiedź**: Nie. Maksymalna latencja 500μs > cykl 50μs. System może nie zdążyć obsłużyć przerwania zanim nadejdzie następne. Potrzebujesz gwarantowanej latencji < 50μs.

---

## Wskazówka zapamiętywania

> **Latencja = Czekanie na reakcję**
>
> Kiedy pukasz do drzwi, latencja to czas do momentu, gdy ktoś zaczyna otwierać.
>
> Nie liczy się, jak długo otwieranie trwa (execution time) - liczy się, jak długo czekasz na reakcję.