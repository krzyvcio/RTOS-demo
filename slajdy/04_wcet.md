# WCET - Worst-Case Execution Time

## Definicja

Pesymistyczny, gwarantowany górny limit czasu wykonania zadania. WCET jest fundamentalnym parametrem w analizie czasu rzeczywistego - bez niego nie można zagwarantować terminowości systemu.

______________________________________________________________________

## Wizualizacja WCET

```
┌───────────────────────────────────────────────────────────────┐
│                    WCET vs Średni czas                        │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  CZASY WYKONANIA (ms):                                        │
│  ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤              │
│  2   3   4   5   6   7   8   9  10  11  12                    │
│       ↑                   ↑                       ↑          │
│     min=2              avg=7                    max=12        │
│                                                               │
│  WCET:                                                        │
│  ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤ 15ms     │
│                                          ↑                    │
│                                    WCET = 15ms                │
│                                    (z marginesem)             │
│                                                               │
│  WCET musi być GWARANTOWANY - nie obserwowany!               │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Typy czasów wykonania

| Typ | Opis | Zastosowanie |
|-----|------|--------------|
| **BCET** | Best-Case Execution Time | Teoretyczny, rzadko używany |
| **ACET** | Average-Case Execution Time | Analiza wydajności |
| **WCET** | Worst-Case Execution Time | Gwarancja deadline |
| **Observed Max** | Maksimum z pomiarów | NIE jest gwarancją! |

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Avionics (DO-178C)

| System | WCET | Deadline | Margines |
|--------|------|----------|----------|
| Flight Control | 5ms | 10ms | 50% |
| Engine Control | 2ms | 5ms | 60% |
| Display System | 16ms | 33ms | 50% |
| Autopilot | 10ms | 20ms | 50% |

**Wymaganie DO-178C:** WCET musi być udowodniony, nie tylko zmierzony

### Automotive (ISO 26262)

- **ECU Engine:** WCET < 5ms dla cyklu wtrysku
- **Brake-by-Wire:** WCET < 10ms dla pełnej sekwencji
- **ADAS:** WCET < 100ms dla detekcji przeszkód

### Medical Devices

- **Respiratory:** WCET < 50ms dla cyklu oddechu
- **Infusion Pump:** WCET < 1s dla dawkowania
- **Pacemaker:** WCET < 10ms dla detekcji arytmii

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Konsekwencje |
|------------|------|--------------|
| **Niedoszacowanie** | WCET za mały | Deadline miss → awaria |
| **Przeszacowanie** | WCET za duży | Niewykorzystane zasoby |
| **Niekompletna analiza** | Pominięcie ścieżek | Ukryte problemy |
| **Hardware variability** | Cache, pipeline | Nieprzewidywalność |
| **Zmiany kodu** | Nowa wersja = nowy WCET | Konieczność re-analizy |

______________________________________________________________________

## Metody analizy WCET

### 1. Analiza statyczna

```
┌─────────────────────────────────────────────────────────────┐
│                 ANALIZA STATYCZNA                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  WEJŚCIE:                                                   │
│  ├── Kod źródłowy lub binarny                              │
│  ├── Informacje o sprzęcie (timing procesora)              │
│  └── Ograniczenia pętli (loop bounds)                      │
│                                                             │
│  PRZEMIATALNOŚĆ:                                            │
│  ├── Control Flow Graph (CFG)                              │
│  ├── Analiza pętli                                         │
│  ├── Analiza cache                                         │
│  └── ILP (Integer Linear Programming)                      │
│                                                             │
│  WYJŚCIE:                                                   │
│  └── Gwarantowany WCET                                     │
│                                                             │
│  ZALETY:                                                    │
│  ├── Gwarancja matematyczna                                │
│  └── Bezpieczne dla certyfikacji                           │
│                                                             │
│  WADY:                                                      │
│  ├── Może dać pesymistyczny wynik                          │
│  └── Wymaga adnotacji dla pętli                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 2. Pomiar (Measurement-based)

```
ZALETY:
├── Dokładne dla testowanych ścieżek
├── Łatwe w zastosowaniu
└── Realistyczne wyniki

WADY:
├── NIE gwarantuje pokrycia wszystkich ścieżek
├── Może pominąć najgorszy przypadek
└── NIE akceptowane dla najwyższych poziomów certyfikacji
```

### 3. Hybrydowa

- Połączenie statycznej analizy z pomiarami
- Pomiary dostarczają danych dla analizy
- Analiza dostarcza gwarancji

______________________________________________________________________

## Przykład analizy WCET

```c
// Przykład: Oblicz WCET dla funkcji

int process_data(int* data, int n) {
    int sum = 0;
    // Pętla: max n = 100 iteracji (z adnotacją)
    for (int i = 0; i < n; i++) {      // Loop bound: 100
        if (data[i] > 0) {              // Branch
            sum += data[i];             // Path 1: add
        } else {
            sum -= data[i];             // Path 2: sub
        }
    }
    return sum;
}

// ANALIZA:
// - Instrukcje w pętli: min 5, max 6
// - Loop bound: 100 (adnotacja)
// - Cache miss penalty: +20 cykli (pesymistycznie)
// - WCET = 100 * (6 + cache_penalty) + overhead
// - WCET = 100 * 26 + 10 = 2610 cykli
// - Przy 100MHz = 26.1 μs
```

______________________________________________________________________

## Adnotacje dla analizy

```c
// AiT / Bound-T style annotations

/**
 * @loopbound 100
 * @wcet 2610 cycles
 */
int process_data(int* data, int n) {
    // ...
}

// RapiTime annotations
// __RTA_LOOP_BOUND(100)
for (int i = 0; i < n; i++) {
    // ...
}
```

______________________________________________________________________

## Kierunki rozwoju

### 1. AI-assisted WCET Analysis

- ML do przewidywania ścieżek wykonania
- Automatyczne wykrywanie loop bounds

### 2. Hardware-assisted WCET

- Procesory z deterministycznym timing
- Scratchpad memory zamiast cache
- Predykcyjne pipeline

### 3. Probabilystyczny WCET (pWCET)

- Prawdopodobieństwo przekroczenia
- Stosowane w systemach less-critical
- Eksperymentalne

______________________________________________________________________

## Narzędzia analizy WCET

| Narzędzie | Metoda | Certyfikacja | Cena |
|-----------|--------|--------------|------|
| **aiT (AbsInt)** | Statyczna | DO-178C, ISO 26262 | Komercyjne |
| **Bound-T** | Statyczna | DO-178C | Komercyjne |
| **RapiTime** | Hybrydowa | ISO 26262 | Komercyjne |
| **Heptane** | Statyczna | Badawcze | Open source |
| **Chronos** | Statyczna | Badawcze | Open source |
| **SWEET** | Statyczna | Badawcze | Open source |

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Analizuj WCET funkcji:

int compute(int* arr, int n, int threshold) {
    int result = 0;
    for (int i = 0; i < n; i++) {        // Loop bound: ?
        if (arr[i] > threshold) {
            result += complex_calc(arr[i]);  // WCET: 50 cykli
        } else {
            result += simple_calc(arr[i]);   // WCET: 10 cykli
        }
    }
    return result;
}

DANE:
- complex_calc: WCET = 50 cykli
- simple_calc: WCET = 10 cykli
- Loop overhead: 5 cykli na iterację
- Overhead funkcji: 20 cykli
- n max = 1000 (z adnotacji)

PYTANIA:
1. Oblicz WCET dla przypadku pesymistycznego
2. Jaki jest BCET?
3. Jakie dane wejściowe wywołają WCET?

ROZWIĄZANIE:
1. WCET = 1000 * (5 + 50) + 20 = 55,020 cykli
2. BCET = 1000 * (5 + 10) + 20 = 15,020 cykli
3. Wszystkie arr[i] > threshold
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego pomiar nie jest wystarczający do określenia WCET?
1. Jakie informacje są potrzebne do statycznej analizy WCET?
1. Dlaczego WCET jest ważniejszy niż ACET w systemach czasu rzeczywistego?
1. Jak cache wpływa na analizę WCET?

______________________________________________________________________

## Literatura

1. Wilhelm et al., "The Worst-Case Execution Time Problem" (2008)
1. AbsInt, "aiT WCET Analyzer Manual"
1. DO-178C, Section 6.3.4 - Timing Analysis
1. ISO 26262-6, Annex D - Worst-Case Execution Time
