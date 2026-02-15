# Safety Case - Dowód Bezpieczeństwa

## Definicja

Strukturalny dokument przedstawiający uzasadnione przekonanie, że system jest wystarczająco bezpieczny dla danego zastosowania. Safety Case jest wymagany przez normy (ISO 26262, DO-178C, IEC 61508) jako dowód zgodności.

______________________________________________________________________

## Struktura Safety Case

```
┌───────────────────────────────────────────────────────────────┐
│                    SAFETY CASE STRUKTURA                      │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │                   SAFETY GOAL                            │ │
│  │        "System jest bezpieczny dla zastosowania X"       │ │
│  └─────────────────────────┬───────────────────────────────┘ │
│                            │                                  │
│         ┌──────────────────┼──────────────────┐              │
│         │                  │                  │              │
│         ▼                  ▼                  ▼              │
│  ┌────────────┐     ┌────────────┐     ┌────────────┐       │
│  │  ARGUMENT  │     │  ARGUMENT  │     │  ARGUMENT  │       │
│  │   (G1)     │     │   (G2)     │     │   (G3)     │       │
│  └─────┬──────┘     └─────┬──────┘     └─────┬──────┘       │
│        │                  │                  │              │
│        ▼                  ▼                  ▼              │
│  ┌────────────┐     ┌────────────┐     ┌────────────┐       │
│  │  EVIDENCE  │     │  EVIDENCE  │     │  EVIDENCE  │       │
│  │   (E1)     │     │   (E2)     │     │   (E3)     │       │
│  │ Test Report│     │ Analysis   │     │ Inspection │       │
│  └────────────┘     └────────────┘     └────────────┘       │
│                                                               │
│  ELEMENTY:                                                    │
│  ├── Goals (G): Twierdzenia o bezpieczeństwie               │
│  ├── Arguments: Powiązania między celami                    │
│  ├── Evidence (E): Dowody (testy, analizy)                  │
│  ├── Context: Założenia i warunki                           │
│  └── Strategies: Podejście do dowodzenia                    │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## GSN - Goal Structuring Notation

```
┌─────────────────────────────────────────────────────────────┐
│                    GSN NOTATION                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  SYMBOLE:                                                   │
│                                                             │
│  ┌─────────────────┐                                       │
│  │     GOAL        │  Twierdzenie do udowodnienia         │
│  │  "System is    │  (cel bezpieczeństwa)                 │
│  │   safe"        │                                       │
│  └────────┬────────┘                                       │
│           │                                                 │
│           │ supported by                                    │
│           ▼                                                 │
│  ┌─────────────────┐                                       │
│  │   STRATEGY      │  Podejście do dowodzenia             │
│  │ "Argument over │                                       │
│  │  sub-goals"    │                                       │
│  └────────┬────────┘                                       │
│           │                                                 │
│           │ in context of                                   │
│           ▼                                                 │
│  ┌─────────────────┐                                       │
│  │    CONTEXT      │  Założenia, warunki                  │
│  │ "System design │                                       │
│  │  specification"│                                       │
│  └─────────────────┘                                       │
│                                                             │
│  ┌─────────────────┐                                       │
│  │    EVIDENCE     │  Dowód                               │
│  │  "Test Report  │  (dokument, wynik testu)              │
│  │      #123"     │                                       │
│  └─────────────────┘                                       │
│                                                             │
│  PRZYKŁAD GSN:                                              │
│                                                             │
│  G1: "System hamulcowy jest bezpieczny"                    │
│   │                                                         │
│   ├── S1: "Argument nad hazardami"                         │
│   │    │                                                    │
│   │    ├── G2: "Awaria hamulca jest wykrywalna"            │
│   │    │    └── E1: "Testy wykrywania awarii"              │
│   │    │                                                    │
│   │    ├── G3: "Czas reakcji jest akceptowalny"            │
│   │    │    └── E2: "Analiza WCET"                         │
│   │    │                                                    │
│   │    └── G4: "Fallback działa poprawnie"                 │
│   │         └── E3: "Testy hybrydowego backup"             │
│   │                                                         │
│   └── C1: "System operuje w temperaturach -40°C do +85°C"  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Automotive (ISO 26262)

| Dokument | Zawartość | ASIL |
|----------|-----------|------|
| Hazard Analysis | HARA, ASIL rating | Wszystkie |
| Safety Requirements | FSR, TSR | Wszystkie |
| Safety Architecture | Design, isolation | B-D |
| Verification Report | Testy, analiza | A-D |
| Safety Case | Argument, evidence | B-D |

### Aerospace (DO-178C)

- **Plan for Software Aspects of Certification (PSAC)**
- **Software Accomplishment Summary (SAS)**
- **Safety Assessment Report**
- **Traceability Matrices**

### Medical (IEC 62304)

- **Software Safety Class Determination**
- **Risk Management File**
- **Software Development Plan**
- **Verification and Validation Report**

______________________________________________________________________

## Elementy Safety Case

```
┌─────────────────────────────────────────────────────────────┐
│              ELEMENTY SAFETY CASE (ISO 26262)              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. HAZARD ANALYSIS (HARA)                                 │
│     ├── Identyfikacja hazardów                             │
│     ├── Klasyfikacja ASIL                                  │
│     └── Safety Goals                                       │
│                                                             │
│  2. FUNCTIONAL SAFETY CONCEPT                              │
│     ├── Funkcjonalne wymagania bezpieczeństwa (FSR)        │
│     ├── Safety mechanisms                                  │
│     └── Allocation do elementów systemu                    │
│                                                             │
│  3. TECHNICAL SAFETY CONCEPT                               │
│     ├── Techniczne wymagania bezpieczeństwa (TSR)          │
│     ├── Architektura bezpieczeństwa                       │
│     └── Hardware/Software allocation                       │
│                                                             │
│  4. SAFETY VALIDATION                                      │
│     ├── Testy integracyjne                                 │
│     ├── Testy systemowe                                    │
│     └── Walidacja w pojeździe                              │
│                                                             │
│  5. SAFETY ARGUMENT                                        │
│     ├── Argumentacja (GSN)                                 │
│     ├── Dowody (evidence)                                  │
│     └── Assumptions and limitations                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Przykład Safety Case dla Systemu Hamulcowego

```
┌─────────────────────────────────────────────────────────────┐
│           SAFETY CASE: ELEKTRYCZNY SYSTEM HAMULCOWY        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  G0: "System hamulcowy jest bezpieczny (ASIL-D)"           │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ S1: Argument przez eliminację hazardów              │   │
│  └───────────────────────┬─────────────────────────────┘   │
│                          │                                  │
│     ┌────────────────────┼────────────────────┐            │
│     │                    │                    │            │
│     ▼                    ▼                    ▼            │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐         │
│  │ G1: H1   │      │ G2: H2   │      │ G3: H3   │         │
│  │ Utrata   │      │ Nieumie- │      │ Opóźniona│         │
│  │ hamulca  │      │ jetn.    │      │ reakcja  │         │
│  │ ASIL-D   │      │ hamowania│      │ ASIL-C   │         │
│  └────┬─────┘      └────┬─────┘      └────┬─────┘         │
│       │                 │                 │                │
│       ▼                 ▼                 ▼                │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐         │
│  │ S1.1:    │      │ S2.1:    │      │ S3.1:    │         │
│  │ Redundan-│      │ Monitor- │      │ WCET     │         │
│  │ cja      │      │ ing      │      │ analysis │         │
│  └────┬─────┘      └────┬─────┘      └────┬─────┘         │
│       │                 │                 │                │
│       ▼                 ▼                 ▼                │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐         │
│  │ E1: Test │      │ E2: Test │      │ E3: WCET │         │
│  │ redund.  │      │ monitor. │      │ report   │         │
│  │ #BR-001  │      │ #BR-002  │      │ #WCET-01 │         │
│  └──────────┘      └──────────┘      └──────────┘         │
│                                                             │
│  ZAŁOŻENIA (Assumptions):                                  │
│  ├── Temperatura pracy: -40°C do +85°C                    │
│  ├── Napięcie zasilania: 9V do 16V                        │
│  ├── Czas życia systemu: 15 lat                           │
│  └── Przeglądy co 12 miesięcy                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **Incomplete argument** | Brakujące elementy dowodu | Checklist, review |
| **Invalid assumptions** | Błędne założenia | Assumption review |
| **Weak evidence** | Niewystarczające dowody | Wymagane typy evidence |
| **Insufficient independence** | Brak niezależności | Independent review |
| **Outdated safety case** | Nieaktualny dokument | Configuration management |

______________________________________________________________________

## Kierunki rozwoju

### 1. Automated Safety Case

- Generowanie Safety Case z modeli
- Automatyczna traceability
- Continuous compliance

### 2. Safety Case Patterns

- Reużywalne wzorce argumentów
- Biblioteki evidence
- Domain-specific patterns

### 3. Machine-Readable Safety Case

- Formal representation
- Automated reasoning
- Consistency checking

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Stwórz Safety Case dla systemu EPS (Electric Power Steering):

HAZARD: Utrata wspomagania kierownicy
ASIL: D

PYTANIA:
1. Jakie są główne cele (goals)?
2. Jakie dowody (evidence) potrzebujesz?
3. Jakie założenia musisz udokumentować?

ROZWIĄZANIE:
1. Goals:
   G1: Utrata wspomagania jest wykrywana w <100ms
   G2: Failsafe mode jest aktywowany w <200ms
   G3: Kierownica pozostaje sterowalna bez wspomagania
   G4: System odzyskuje się po transient fault

2. Evidence:
   E1: Testy wykrywania awarii (1000 cykli)
   E2: Analiza WCET dla diagnostic task
   E3: Testy mechaniczne układu kierowniczego
   E4: Fault injection tests
   E5: HARA report

3. Assumptions:
   A1: Kierowca może wywierać siłę do 50N
   A2: Maksymalna prędkość pojazdu: 250 km/h
   A3: Temperatura pracy: -40°C do +85°C
   A4: Czas życia: 15 lat
```

______________________________________________________________________

## Pytania kontrolne

1. Czym jest Safety Case i dlaczego jest wymagany?
1. Jakie są główne elementy GSN?
1. Jaka jest różnica między Safety Plan a Safety Case?
1. Jak zapewnić traceability w Safety Case?

______________________________________________________________________

## Literatura

1. ISO 26262, "Road vehicles – Functional safety"
1. DO-178C, "Software Considerations in Airborne Systems"
1. Kelly, "Arguing Safety – A Systematic Approach"
1. GSN Community, "Goal Structuring Notation Standard"
