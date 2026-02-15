# Harmonogramowanie Mieszanej Krytyczności (Mixed-Criticality Scheduling)

## Definicja

Paradygmat harmonogramowania, w którym zadania o różnych poziomach krytyczności (np. ASIL-A do ASIL-D, DAL-A do DAL-E) współdzielą zasoby obliczeniowe przy zachowaniu gwarancji czasowych dla zadań najbardziej krytycznych.

______________________________________________________________________

## Wizualizacja problemu

```
┌───────────────────────────────────────────────────────────────┐
│              MIXED-CRITICALITY SYSTEM (MCS)                   │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  TRADYCYJNY PODEJŚĆ (rozdział zasobów):                       │
│  ├── CPU 1: Tylko zadania ASIL-D (30% wykorzystania)         │
│  ├── CPU 2: Tylko zadania ASIL-B (40% wykorzystania)         │
│  └── CPU 3: Tylko zadania QM (50% wykorzystania)             │
│  → WASTE: Nieefektywne wykorzystanie zasobów!                 │
│                                                               │
│  MIXED-CRITICALITY (współdzielenie):                          │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ CPU 1:                                                  │ │
│  │ ├── ASIL-D (gwarantowane 30%)                          │ │
│  │ ├── ASIL-B (gwarantowane gdy ASIL-D OK, 40%)           │ │
│  │ └── QM (tylko gdy inni OK, 30%)                        │ │
│  └─────────────────────────────────────────────────────────┘ │
│  → EFEKTYWNOŚĆ: Lepsze wykorzystanie CPU                      │
│  → BEZPIECZEŃSTWO: ASIL-D zawsze gwarantowane                 │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Poziomy krytyczności

### Automotive (ISO 26262 ASIL)

| Poziom | Opis | Przykłady |
|--------|------|-----------|
| **ASIL-D** | Najwyższy ryzyko | Hamulce, kierowanie, airbag |
| **ASIL-C** | Wysokie ryzyko | Światła, wycieraczki |
| **ASIL-B** | Średnie ryzyko | Poduszki (niekrytyczne) |
| **ASIL-A** | Niskie ryzyko | Komfort, wygoda |
| **QM** | Quality Managed | Infotainment, multimedia |

### Aerospace (DO-178C DAL)

| Poziom | Opis | Failure Condition |
|--------|------|-------------------|
| **DAL-A** | Catastrophic | Utrata samolotu, życie |
| **DAL-B** | Hazardous | Ciężkie obrażenia |
| **DAL-C** | Major | Znaczna utrata możliwości |
| **DAL-D** | Minor | Mała utrata możliwości |
| **DAL-E** | No Effect | Bez wpływu |

______________________________________________________________________

## Modele harmonogramowania MCS

### 1. MC-FP (Mixed-Criticality Fixed Priority)

```
ZASADY:
├── Zadania mają przypisane priorytety statyczne
├── Zadania wysokiej krytyczności mają wyższe priorytety
├── Przy przekroczeniu WCET(high) → zadania low są dropped

TRYBY:
├── HI-mode: Wszystkie zadania high wykonują się
├── LO-mode: Zadania low mogą wykonywać się "na gap"
└── Switch: Tryb LO → HI przy overrun high-crit
```

### 2. EDF-VD (EDF with Virtual Deadlines)

```
ZASADY:
├── Virtual Deadline dla zadań HI-crit (krótszy)
├── Jeśli HI-crit przekracza C_LO → przełącz na C_HI
├── Zadania LO-crit mogą być zawieszone

PRZYKŁAD:
Task HI: C_LO = 2, C_HI = 4, Deadline = 10
  → Virtual Deadline = 5 (krótszy niż rzeczywisty)
  → Jeśli wykonanie > 2, użyj rzeczywistego deadline = 10
```

### 3. Dual-Mode Scheduling

```
┌─────────────────────────────────────────────────────────────┐
│                    DUAL-MODE SCHEDULING                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  TRYB NORMALNY (LOW-CRIT):                                  │
│  ├── Wszystkie zadania wykonują się                         │
│  ├── Zadania HI używają pesymistycznego C_LO                │
│  └── Zadania LO mają pełny dostęp                           │
│                                                             │
│  PRZEŁĄCZENIE (HI-CRIT OVERRUN):                            │
│  ├── Wykryto: HI-crit task przekracza C_LO                  │
│  ├── Akcja: Zawieś/zabij zadania LO-crit                    │
│  └── Zadania HI używają C_HI                                │
│                                                             │
│  POWRÓT:                                                    │
│  └── Po zakończeniu HI-crit → przywróć LO-crit              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Automotive (ADAS + Infotainment)

```
System: Samochód autonomiczny Level 3

ZADANIA HI-CRIT (ASIL-D):
├── Perception (lidar, camera): 100ms cykl
├── Planning: 50ms cykl
├── Control: 10ms cykl
└── Brake: 5ms cykl

ZADANIA LO-CRIT (QM):
├── Entertainment: best-effort
├── Navigation (UI): 500ms
└── Diagnostics: 1s

STRATEGIA:
├── Normalnie: Wszystkie działają
├── Przy zagrożeniu: LO-crit suspended
└── Gwarancja: HI-crit zawsze mają zasoby
```

### Aerospace (CNS + IFE)

| System | Krytyczność | Tryb normalny | Tryb degraded |
|--------|-------------|---------------|---------------|
| Flight Control | DAL-A | Pełny | Pełny |
| Navigation | DAL-B | Pełny | Pełny |
| Communication | DAL-C | Pełny | Ograniczony |
| IFE | DAL-E | Pełny | Zawieszony |

### Medical (Pacemaker)

```
HI-CRIT: Sensem i stymulacja serca (ms timing)
LO-CRIT: Telemetria, logowanie (sekundy)
STRATEGIA: LO-crit może być opóźnione, HI-crit nigdy
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Cascading Degradation** | Zawieszenie LO-crit wpływa na UX | Graceful degradation |
| **Mode Switch Latency** | Opóźnienie przełączenia trybów | Szybki mechanizm detekcji |
| **Starvation LO-crit** | LO-crit nigdy nie dostaje czasu | Monitoring, timeout |
| **Over-provisioning** | Zbyt pesymistyczne C_HI | Lepsza analiza WCET |
| **Certification Complexity** | Trudna certyfikacja MCS | Formalne metody |

______________________________________________________________________

## Algorytm Vestal (MC-FP)

```
ALGORYTM PRZYZNAWANIA PRIORYTETÓW:

Dla każdego zadania τ_i:
├── C_i^LO = pesymistyczny WCET w trybie LO
├── C_i^HI = pesymistyczny WCET w trybie HI
├── L_i = poziom krytyczności (HI lub LO)
└── Priorytet = funkcja(L_i, D_i, C_i)

ZASADA:
├── Zadania HI mają wyższe priorytety
├── W trybie LO: używaj C_i^LO
├── W trybie HI: używaj C_i^HI, LO-crit stopped
└── Schedulability: sprawdzane dla obu trybów

WARUNEK SCHEDULABILITY:
Σ(C_i^HI / T_i) ≤ 1 dla HI-crit tasks
Σ(C_i^LO / T_i) ≤ 1 dla wszystkich (w trybie LO)
```

______________________________________________________________________

## Kierunki rozwoju

### 1. Multi-Mode MCS

- Więcej niż dwa tryby (HI/LO)
- Stopniowa degradacja

### 2. Resource Sharing in MCS

- MCS z mutexami i PCP
- Izolacja zasobów między krytycznościami

### 3. Certifiable MCS

- Formalne dowody dla certyfikacji
- DO-178C compliant MCS

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
System MCS z następującymi zadaniami:

| Zadanie | L  | C_LO | C_HI | D  | T  |
|---------|-----|------|------|-----|-----|
| τ1      | HI  | 2    | 4    | 10  | 10  |
| τ2      | HI  | 1    | 2    | 10  | 10  |
| τ3      | LO  | 3    | -    | 20  | 20  |
| τ4      | LO  | 2    | -    | 20  | 20  |

PYTANIA:
1. Czy system jest schedulable w trybie LO?
2. Czy system jest schedulable w trybie HI?
3. Co się stanie jeśli τ1 przekroczy C_LO?

ROZWIĄZANIE:
1. Tryb LO: U = (2+1)/10 + (3+2)/20 = 0.3 + 0.25 = 0.55 < 1 ✓

2. Tryb HI (tylko τ1, τ2): U = (4+2)/10 = 0.6 < 1 ✓

3. τ1 przekracza C_LO:
   - Przełączenie na tryb HI
   - τ3, τ4 zawieszone/dropped
   - τ1, τ2 kontynuują z C_HI
```

______________________________________________________________________

## Pytania kontrolne

1. Dlaczego traditional scheduling nie wystarcza dla MCS?
1. Jakie są zalety i wady MC-FP vs EDF-VD?
1. Jak zapewnić izolację między zadaniami różnych krytyczności?
1. Jakie są wymagania certyfikacyjne dla MCS?

______________________________________________________________________

## Literatura

1. Vestal, "Preemptive Scheduling of Multi-Criticality Systems" (2007)
1. Baruah, "Mixed-Criticality Scheduling Theory" (2015)
1. ISO 26262-6, Annex E - Mixed Criticality
1. Burns, "Mixed Criticality Systems - A Review" (2019)
