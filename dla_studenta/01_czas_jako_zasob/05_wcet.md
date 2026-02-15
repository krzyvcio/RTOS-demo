# WCET (Worst Case Execution Time)

## Definicja

**WCET** to najdłuższy możliwy czas wykonania fragmentu kodu. To gwarantowane maksimum, nie średnia czy typowa wartość.

> WCET to pesymistyczna odpowiedź na pytanie: "Ile maksymalnie czasu to może zająć?" W systemach RTOS to jedyne pytanie, które się liczy.

```
Wykonania kodu:
│ Run 1:  50ms
│ Run 2:  45ms
│ Run 3: 120ms  ← WCET! (najgorszy przypadek)
│ Run 4:  48ms
│ Run 5:  52ms
└────────────────
Average: 63ms
WCET:    120ms  ← Tylko to się liczy w RTOS!
```

---

## Analogia do przyrody

### 🌊 Rzeka i powódź

Inżynier buduje tamę. Nie projektuje jej na "przeciętny przepływ rzeki". Projektuje na **najgorszy przypadek** - stuletnią powódź.

```
Przeciętny przepływ: 100 m³/s
Najgorszy przypadek: 1000 m³/s (WCET)
Tama musi wytrzymać: 1000+ m³/s
```

Jeśli zaprojektujesz na średnią - tama runie przy pierwszej powodzi.

### 🚗 Hamowanie samochodu

Kiedy hamujesz przed przeszkodą, nie liczysz na "typowe" warunki. Liczysz na najgorsze:

```
- Mokra nawierzchnia
- Zużyte opony
- Obciążony samochód
- Stare klocki hamulcowe

WCET hamowania = wszystkie te czynniki naraz
```

### 🏔️ Wspinaczka górska

Planujesz wspinaczkę. Nie bierzesz wody na "przeciętny czas wejścia". Bierzesz na najgorszy przypadek: burza, zgubienie szlaku, kontuzja.

**WCET wspinaczki = czas przetrwania w najgorszych warunkach**

---

## Podobieństwo do systemów informatycznych

### Load Testing

```python
# Testy wydajnościowe
def test_api():
    # Średni czas: 50ms
    # P95: 200ms
    # P99: 500ms
    # P99.9: 2000ms ← To jest prawie WCET
    pass
```

W świecie IT mówi się o "tail latency" - najgorsze 1% lub 0.1%. W RTOS to za mało - potrzebujesz absolutnego maximum.

### Database Query Optimization

```sql
-- Query execution time:
-- Fast path (index hit):     1ms
-- Slow path (full scan):   500ms  ← WCET
-- Need index to avoid WCET!
```

Index to sposób na zmniejszenie WCET, nie średniego czasu.

### Network Timeout

```python
# Timeout to "deklarowany WCET"
response = requests.get(url, timeout=5.0)
# Jeśli WCET > 5s → timeout
# Musisz znać WCET, żeby ustawić timeout!
```

---

## Dlaczego WCET jest trudny?

### Problem 1: Branch-dependent paths

```c
void process(int data) {
    if (data > 0) {
        // Ścieżka A: 10 instrukcji
    } else {
        // Ścieżka B: 100 instrukcji
    }
}
```

Która ścieżka jest WCET? B. Ale musisz wiedzieć, że `data <= 0` jest możliwe.

### Problem 2: Loops

```c
void process_array(int arr[], int n) {
    for (int i = 0; i < n; i++) {
        process_element(arr[i]);
    }
}
```

Ile iteracji? WCET = max(n) × WCET(process_element).

Ale co jeśli `n` zależy od danych?

### Problem 3: Cache i memory

```c
void process(void) {
    // Cache hit:   4 cycles
    // Cache miss: 100 cycles
    access_memory(global_data);
}
```

WCET = zakładając cache miss. Ale to pesymistyczne. A jeśli cache locking?

### Problem 4: Interrupts

```c
void process(void) {
    // Ten kod może być przerwany!
    // WCET to nie tylko ten kod
    // Ale też ewentualne przerwania
}
```

---

## Jak obliczyć WCET?

### Metoda 1: Pomiar (Measurement-based)

```c
void measure_wcet(void) {
    uint32_t min_time = UINT32_MAX;
    uint32_t max_time = 0;

    for (int i = 0; i < 10000; i++) {
        uint32_t start = get_cycle_count();
        process_data(test_inputs[i]);
        uint32_t end = get_cycle_count();

        uint32_t elapsed = end - start;
        if (elapsed < min_time) min_time = elapsed;
        if (elapsed > max_time) max_time = elapsed;
    }

    printf("Min: %u, Max: %u\n", min_time, max_time);
}
```

**Problem**: Nigdy nie wiesz, czy znalazłeś najgorszy przypadek!

```
Możliwe, że:
- Nie testowałeś wszystkich danych wejściowych
- Nie trafiłeś na cache miss
- Nie miałeś interferencji od przerwań
```

### Metoda 2: Statyczna analiza (Static Analysis)

```c
// Narzędzia: aiT, RapiTime, OTAWA
void process(int x) {
    // Analizator oblicza:
    // - Liczbę iteracji pętli
    // - Ścieżki wykonania
    // - Cachowanie
    // - Pipeline CPU
}
```

**Zalety**:
- Gwarantowane WCET (bezpieczne)
- Nie wymaga uruchomienia kodu

**Wady**:
- Może być bardzo pesymistyczne (over-estimation)
- Trudne dla skomplikowanego kodu
- Wymaga adnotacji dla pętli

### Metoda 3: Hybrid

```c
// Połącz pomiar z analizą:
// 1. Zmierz typowe wykonanie
// 2. Dodaj margines bezpieczeństwa z analizy
// 3. Waliduj na rzeczywistym hardware

Measured WCET: 50μs
Analysis margin: +20%
Final WCET: 60μs
```

---

## WCET Analysis - przykład

```c
int search(int arr[], int n, int target) {
    for (int i = 0; i < n; i++) {        // Loop: max n iterations
        if (arr[i] == target) {           // Branch
            return i;                      // Early exit
        }
    }
    return -1;                            // Not found
}
```

**Analiza WCET**:

```
Best case: 1 iteracja (target na pozycji 0)
Worst case: n iteracji (target nie istnieje lub na końcu)

WCET = n × (loop_overhead + comparison + branch)
     = n × (5 + 2 + 1) cycles
     = 8n cycles

Dla n = 100: WCET = 800 cycles
```

**Ale wait...**

```c
// Co jeśli n jest nieograniczone?
int search(int arr[], int n, int target) {
    // n może być dowolnie duże!
    // WCET = nieskończoność?!
}
```

**Rozwiązanie: Bounded loops**

```c
#define MAX_ARRAY_SIZE 256

int search(int arr[], int n, int target) {
    // WCET jest teraz policzalne
    int limit = (n > MAX_ARRAY_SIZE) ? MAX_ARRAY_SIZE : n;
    for (int i = 0; i < limit; i++) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

// WCET = 8 × 256 = 2048 cycles
```

---

## WCET vs Real Hardware

| Czynnik | Wpływ na WCET | Jak radzić sobie |
|---------|---------------|------------------|
| Cache miss | +100 cycles | Cache locking, preheating |
| Branch misprediction | +10 cycles | Branchless code |
| Memory access | +50 cycles | TCM, local variables |
| Interrupts | +? cycles | Disable ints, analysis |
| DMA contention | +? cycles | Reserved bandwidth |

---

## Jak pisać kod przyjazny WCET?

### Unikaj nieograniczonych pętli

```c
// ZŁE: WCET nieznane
while (process_next()) {
    // Ile iteracji? Kto wie!
}

// DOBRE: WCET znane
for (int i = 0; i < MAX_ITERATIONS; i++) {
    if (!process_next()) break;
}
```

### Unikaj dynamicznej alokacji

```c
// ZŁE: WCET nieprzewidywalne
char* buffer = malloc(size);

// DOBRE: WCET stałe
static char buffer[MAX_SIZE];
```

### Unikaj złożonych branchy

```c
// ZŁE: Różne ścieżki, różne czasy
if (condition_a) {
    slow_path();
} else if (condition_b) {
    medium_path();
} else {
    fast_path();
}

// DOBRE: Stały czas
result = (condition_a && slow_path_value) |
         (condition_b && medium_path_value) |
         (!condition_a && !condition_b && fast_path_value);
```

### Używaj adnotacji

```c
// Pomóż analizatorowi WCET
void process(void) {
    // LOOP_BOUND: 100
    for (int i = 0; i < 100; i++) {
        // ...
    }
}
```

---

## Jak świat radzi sobie z WCET?

### Automotive (ISO 26262)

WCET musi być udowodnione dla każdego krytycznego tasku. Narzędzia jak aiT, RapiTime są standardem.

```
ASIL-D (najwyższy poziom):
- Każda funkcja musi mieć znane WCET
- Margines bezpieczeństwa: często 2x measured
- Regularna walidacja na target hardware
```

### Aerospace (DO-178C)

```
DAL-A (najwyższy poziom):
- Strukturalne pokrycie kodu (MC/DC)
- WCET analysis dla każdej ścieżki
- Independent verification
```

### Medical (IEC 62304)

```
Klasa C (life-threatening):
- WCET analysis wymagane
- Evidence of timing safety
```

---

## Narzędzia do WCET analysis

| Narzędzie | Typ | Platformy |
|-----------|-----|-----------|
| aiT (AbsInt) | Static analysis | ARM, PowerPC, x86 |
| RapiTime (Rapita) | Hybrid | Multi-platform |
| OTAWA | Open source | ARM, others |
| SWEET (Mälardalen) | Research | Generic |
| Bound-T | Static analysis | ARM, AVR |

---

## Pytania do przemyślenia

1. Czy znasz WCET wszystkich krytycznych funkcji w Twoim systemie?
2. Jak mierzysz WCET? Czy masz pokrycie wszystkich ścieżek?
3. Czy Twój kod jest "WCET-friendly"? Czy ma bounded loops?

---

## Quiz

**Pytanie**: Masz funkcję:

```c
void process(int* data, int count) {
    for (int i = 0; i < count; i++) {
        if (data[i] > 0) {
            complex_operation(data[i]);  // 100 cycles
        } else {
            simple_operation(data[i]);   // 10 cycles
        }
    }
}
```

count max = 1000, wszystkie dane > 0.

Jaki jest WCET?

**Odpowiedź**:

```
WCET = count_max × max(complex_operation, simple_operation)
     = 1000 × 100
     = 100,000 cycles

Uwaga: Założyliśmy, że "wszystkie dane > 0" to pesymistyczny przypadek.
Analizator WCET może tego nie wiedzieć bez adnotacji!
```

---

## Wskazówka zapamiętywania

> **WCET = Pesymistyczny scenariusz**
>
> Wyobraź sobie, że projektujesz ewakuację budynku.
> Nie liczysz na "przeciętną" ilość ludzi.
> Liczysz na:
> - Pełny budynek
> - Zablokowane wyjścia
> - Panikę
> - Osoby z niepełnosprawnościami
>
> To jest WCET ewakuacji. Inżynierieria bezpieczeństwa to pesymizm.