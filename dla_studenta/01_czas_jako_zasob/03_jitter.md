# Jitter

## Definicja

**Jitter** to wahania (zmienność) czasu wykonania lub latencji. To różnica między czasem oczekiwanym a rzeczywistym, lub między kolejnymi wykonaniami tej samej operacji.

> Jitter to "nerwowość" systemu. Jeśli latencja to średni czas reakcji, to jitter to jak bardzo system jest nieprzewidywalny.

```
Idealny system (brak jitter):
│   │   │   │   │   │   │   │
│   │   │   │   │   │   │   │
└───┴───┴───┴───┴───┴───┴───┴───► czas
    każdy cykl identyczny

System z jitter:
│  │   │ │    │ │   │    │ │
│  │   │ │    │ │   │    │ │
└──┴───┴─┴────┴─┴───┴────┴─┴──► czas
   wahania w każdym cyklu
```

---

## Analogia do przyrody

### 🎵 Muzyka i orkiestra

Idealny metronom: TIK-tak-TIK-tak-TIK-tak (stały rytm)

Orkiestra z jitter: TIK..tak..TIK.tak...TIK..tak (wahania)

**Mały jitter = rubato, artyzm**
**Duży jitter = chaos, niezgranie**

W muzyce barokowej używa się *tempo rubato* - celowego, drobnego jitter dla ekspresji. Ale za dużo = amatorszczyzna.

### 🌊 Fale na morzu

Regularne fale: latanie żaglem jest przewidywalne.
Nieregularne fale (jitter): trudniej nawigować, możliwe przewrócenie.

Tsunami ma bardzo mały jitter - dlatego jest tak niszczycielskie (cała energia uderza naraz).

### 🧬 Bicie serca

Zdrowe serce: regularny rytm (mały jitter)
Chore serce: arytmia (duży jitter)

**Heart Rate Variability (HRV)** to po prostu jitter rytmu serca. Paradoksalnie - pewien poziom jitter jest zdrowy (adaptacja), ale za duży = arytymia.

---

## Podobieństwo do systemów informatycznych

### Network Jitter

```
Pakiet 1: przychodzi po 10ms
Pakiet 2: przychodzi po 15ms  (jitter = 5ms)
Pakiet 3: przychodzi po 8ms   (jitter = 7ms)
Pakiet 4: przychodzi po 12ms  (jitter = 4ms)
```

**Problem w VoIP/Video**: Jitter powoduje przycięcia, zacinanie.

**Rozwiązanie**: Jitter buffer - buforowanie pakietów przed odtworzeniem.

```python
# Symulacja jitter buffer
jitter_buffer = Queue(maxsize=10)

def receiver():
    while True:
        packet = network.receive()  # Nieregularne przychodzenie
        jitter_buffer.put(packet)

def player():
    while True:
        packet = jitter_buffer.get(timeout=expected_interval)
        play(packet)  # Regularne odtwarzanie
```

### Frame Time w grach

Idealnie: 60 FPS = 16.67ms na klatkę

Rzeczywistość:
```
Klatka 1: 15ms
Klatka 2: 18ms  (jitter = 1.33ms)
Klatka 3: 12ms  (jitter = 4.67ms)
Klatka 4: 20ms  (jitter = 3.33ms) → micro-stutter
```

### Database Query Time

```sql
-- To samo zapytanie, różne czasy:
Query 1: 50ms   (cache hit)
Query 2: 200ms  (cache miss, disk read)
Query 3: 55ms   (cache hit)
Query 4: 180ms  (cache miss)
```

Jitter = 150ms. To dlaczego bazy danych mają "percentyle" w SLA.

---

## Rodzaje jitter w RTOS

### 1. Timing Jitter

Wahania czasu między kolejnymi tickami lub przerwaniami.

```
Tick 1: 10.00ms
Tick 2: 10.02ms   (jitter = +0.02ms)
Tick 3:  9.98ms   (jitter = -0.04ms)
Tick 4: 10.05ms   (jitter = +0.07ms)
```

**Przyczyny**:
- Niestabilny zegar (crystal drift)
- Przerwania wyłączane przez inny kod
- Thermal throttling

### 2. Execution Time Jitter

Wahania czasu wykonania tego samego kodu.

```c
void task_periodic(void) {
    // To samo zadanie, różny czas:
    process_data();  // 1-5ms zależnie od danych
    send_result();   // 0.5-2ms zależnie od sieci
}
```

**Przyczyny**:
- Branch prediction (różne ścieżki kodu)
- Cache hit/miss
- Dane wejściowe różnej wielkości

### 3. Response Time Jitter

Wahania czasu odpowiedzi na to samo zdarzenie.

```
Zdarzenie: przycisk wciśnięty
Odpowiedź 1: 5ms   (CPU idle)
Odpowiedź 2: 15ms  (CPU busy z innym taskiem)
Odpowiedź 3: 8ms   (cache miss)
```

---

## Dlaczego jitter jest problemem?

### Problem 1: Deadline miss

```
Deadline = 10ms

│  │ │   │  │
│  │ │   │  │     ← Jitter powoduje, że
└──┴─┴───┴──┴─────► niektóre cykle przekraczają deadline
         ↑
      MISS!
```

Nawet jeśli średnia jest OK, pojedynczy jitter może spowodować miss.

### Problem 2: Kaskada problemów

```
Task A ma jitter → Task B się opóźnia → Task C miss deadline
     ↓
System control loop destabilizuje się
     ↓
Oscylacje, błędy, awaria
```

### Problem 3: Trudność debugowania

```
"U mnie działa" - bo jitter zależy od:
- Obciążenia CPU
- Stanu cache
- Temperatury
- Innych uruchomionych tasków
- Fazy księżyca (prawie)
```

---

## Jak mierzyć jitter?

### Statystycznie

```
Jitter = Max - Min
Jitter RMS = sqrt(Σ(x - mean)² / n)

Percentyle:
P50 (mediana): typowa wartość
P99: najgorsze 1%
P99.9: najgorsze 0.1% - krytyczne dla RT!
```

### Histogram

```
     │
  50 │              ██
  40 │            ████
  30 │          ██████
  20 │        ████████
  10 │      ██████████
   0 │    ████████████
     └────┴────┴────┴────►
        8ms  10ms 12ms  14ms
              ↑
         Jitter = rozkład
```

Dobry system RTOS ma wąski, wysoki "pik". Zły system ma szeroki, niski rozkład z "ogonem" (tail latency).

---

## Jak sobie radzić z jitter?

### Hardware solutions:

1. **Cache locking** - zablokuj krytyczny kod w cache
2. **Tightly Coupled Memory** - pamięć bez cache, deterministyczna
3. **CPU frequency locking** - wyłącz dynamic scaling
4. **Dedicated interrupts** - oddzielne linie dla krytycznych źródeł

### Software solutions:

```c
// ZŁE: Branch-dependent execution
if (data > threshold) {
    process_large();   // czas nieokreślony
} else {
    process_small();   // inny czas
}

// DOBRE: Deterministic execution
process_deterministic(data);  // zawsze ta sama ścieżka
// nawet jeśli less efficient, ale predictable
```

```c
// ZŁE: Dynamic memory
void process() {
    char* buffer = malloc(size);  // czas nieokreślony!
    // ...
    free(buffer);
}

// DOBRE: Static allocation
void process() {
    static char buffer[MAX_SIZE];  // zawsze gotowy
    // ...
}
```

### Architectural solutions:

```
┌─────────────────────────────────────────────────────────┐
│                  TIME-TRIGGERED ARCHITECTURE            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Tick ──► [Task A: 2ms max] ──► [Task B: 1ms max]      │
│             │                        │                  │
│             └───── Buffer overflow? ─┘                  │
│                                                         │
│  Jeśli Task A przekracza 2ms → system error             │
│  Brak "elastyczności" = brak jitter                     │
└─────────────────────────────────────────────────────────┘
```

---

## Jak świat radzi sobie z jitter?

### Audio/Video Streaming

**Jitter buffer** - celowe opóźnienie dla stabilności:

```
Network (jitter) ──► Buffer ──► Player (smooth)
                        │
                        └── 50-200ms opóźnienia
                            za akceptowalny kompromis
```

### Financial Trading

Hedge fundy używają **Field-Programmable Gate Arrays (FPGA)** zamiast CPU, żeby wyeliminować jitter. FPGA jest deterministyczne - zawsze ten sam czas.

### Aerospace

Systemy lotnicze używają **triple modular redundancy**:

```
        ┌─────────┐
    ───►│ System A│───┐
    │   └─────────┘   │
    │   ┌─────────┐   │   ┌─────────┐
Input──►│ System B│───┼──►│ Voter   │──►Output
    │   └─────────┘   │   └─────────┘
    │   ┌─────────┐   │
    ───►│ System C│───┘
        └─────────┘
```

Jitter w jednym systemie nie psuje całości - voter wybiera większość.

---

## Pytania do przemyślenia

1. Jaki jest P99.9 jitter w Twoim systemie? Jak to mierzysz?
2. Czy Twój system ma "tail latency" - rzadkie ale ogromne wahania?
3. Jak jitter wpływa na stabilność Twojej pętli sterowania?

---

## Quiz

**Pytanie**: System ma średni czas odpowiedzi 5ms z jitter ±2ms. Deadline wynosi 7ms. Czy system jest bezpieczny?

**Odpowiedź**: Formalnie tak, ale... to zależy od rozkładu jitter. Jeśli ±2ms to wartości skrajne (min/max), to OK. Jeśli to odchylenie standardowe, to 99.7% przypadków (3σ) będzie w zakresie 5±6ms = -1ms do 11ms. Wtedy deadline 7ms będzie przekraczany w ~15% przypadków. **Zawsze pytaj o percentyle!**

---

## Wskazówka zapamiętywania

> **Jitter = Nerwowość systemu**
>
> Wyobraź sobie perkusistę:
> - TIK-tak-TIK-tak = brak jitter (metronom)
> - TIK..tak.TIK...tak = jitter (człowiek)
>
> W systemach RTOS: perkusista musi być jak metronom. Żadnych rubato.