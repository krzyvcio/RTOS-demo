# AFDX - Avionics Full-Duplex Switched Ethernet

## Definicja

Deterministyczna sieć Ethernet opracowana dla lotnictwa, zapewniająca gwarantowane opóźnienia i przepustowość. Stosowana w Boeing 787 Dreamliner i Airbus A350.

______________________________________________________________________

## Architektura AFDX

```
┌───────────────────────────────────────────────────────────────┐
│                    AFDX NETWORK                               │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────┐    ┌─────────────┐    ┌─────────┐               │
│  │ End     │    │   AFDX      │    │ End     │               │
│  │ System  │────│   SWITCH    │────│ System  │               │
│  │ (ES)    │    │             │    │ (ES)    │               │
│  └─────────┘    │  - Routing  │    └─────────┘               │
│       │         │  - Policing │          │                    │
│       │         │  - Timing   │          │                    │
│       │         └─────────────┘          │                    │
│       │              │                    │                    │
│       │              │                    │                    │
│       │         ┌─────────────┐          │                    │
│       │         │   AFDX      │          │                    │
│       └─────────│   SWITCH    │──────────┘                    │
│                 │             │                               │
│                 └─────────────┘                               │
│                                                               │
│  WLASCIWOSCI:                                                 │
│  ├── Full-duplex (100 Mbps)                                  │
│  ├── Deterministyczny (gwarantowane opóźnienia)              │
│  ├── Redundancja (dual network)                              │
│  └── Virtual Links (izolacja ruchu)                          │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Virtual Links (VL)

```
┌─────────────────────────────────────────────────────────────┐
│                    VIRTUAL LINKS                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  KONCEPT:                                                   │
│  Każdy kanał komunikacyjny to "Virtual Link" z:            │
│  ├── Unikalnym ID (VL ID)                                  │
│  ├── Gwarantowaną przepustowością (BAG)                    │
│  ├── Maksymalnym rozmiarem ramki (Lmax)                    │
│  └── Gwarantowanym opóźnieniem (max latency)               │
│                                                             │
│  PRZYKŁAD:                                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ VL ID │ BAG (ms) │ Lmax (bytes) │ Przepustowość    │   │
│  ├───────┼──────────┼──────────────┼──────────────────┤   │
│  │ 100   │ 4        │ 1500         │ 3 Mbps           │   │
│  │ 101   │ 8        │ 800          │ 0.8 Mbps         │   │
│  │ 102   │ 2        │ 500          │ 2 Mbps           │   │
│  │ 103   │ 16       │ 2000         │ 1 Mbps           │   │
│  └───────┴──────────┴──────────────┴──────────────────┘   │
│                                                             │
│  BAG (Bandwidth Allocation Gap):                           │
│  ├── Minimalny odstęp między ramkami                      │
│  ├── Wartości: 1, 2, 4, 8, 16, 32, 64, 128 ms             │
│  └── Gwarantuje: Lmax / BAG = max throughput              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Use Cases (Zastosowania praktyczne)

### Boeing 787 Dreamliner

| System | VL ID | BAG | Dane |
|--------|-------|-----|------|
| Flight Control | 1-10 | 1-4 ms | Pozycja, prędkość |
| Avionics | 11-30 | 4-16 ms | Nawigacja, komunikacja |
| Cabin Systems | 31-50 | 16-64 ms | IFE, oświetlenie |
| Maintenance | 51-100 | 64-128 ms | Diagnostyka |

### Airbus A350

- Podobna architektura AFDX
- ~1000 Virtual Links
- Redundantne switche
- Deterministyczna latencja < 150 µs

______________________________________________________________________

## Zagrożenia

| Zagrożenie | Opis | Mitigacja |
|------------|------|-----------|
| **BAG violation** | Nadawca wysyła częściej niż BAG | Policing w switchu |
| **Oversized frame** | Ramka przekracza Lmax | Traffic shaping |
| **Switch congestion** | Przepełnienie bufora | Redundancja, QoS |
| **Redundancy mismatch** | Różnice między sieciami A/B | Integrity checking |
| **Timing drift** | Rozsynchronizacja zegarów | IEEE 1588 PTP |

______________________________________________________________________

## Redundancja w AFDX

```
┌─────────────────────────────────────────────────────────────┐
│                    REDUNDANCJA A/B                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  End System                                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                     │   │
│  │   ┌─────┐     ┌─────────┐     ┌─────────────┐      │   │
│  │   │ App │────▶│ Redund. │────▶│ Network A   │──────┼──┼──▶ Network A
│  │   │     │     │ Mgmt    │     └─────────────┘      │   │
│  │   │     │     │         │     ┌─────────────┐      │   │
│  │   │     │     │         │────▶│ Network B   │──────┼──┼──▶ Network B
│  │   └─────┘     └─────────┘     └─────────────┘      │   │
│  │                                                     │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ZASADY:                                                    │
│  ├── Każda ramka wysyłana na obu sieciach (A i B)         │
│  ├── Sequence Number do identyfikacji duplikatów          │
│  ├── Odbiorca wybiera pierwszą poprawną ramkę             │
│  └── Integrity checking: SN++, timeout                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Timing w AFDX

```
┌─────────────────────────────────────────────────────────────┐
│                    TIMING AFDX                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  MAXIMUM LATENCY:                                           │
│  ├── End System → Switch: ~50 µs                           │
│  ├── Switch processing: ~30 µs                             │
│  ├── Switch → End System: ~50 µs                           │
│  └── Total: ~130-150 µs (max)                              │
│                                                             │
│  JITTER:                                                    │
│  ├── End System jitter: < 500 µs                           │
│  ├── Switch jitter: < 30 µs                                │
│  └── Total jitter: < 1 ms                                  │
│                                                             │
│  TIMELINE:                                                  │
│                                                             │
│  ES_A        Switch         ES_B                            │
│  ┌───┐       ┌─────┐       ┌───┐                           │
│  │Tx │──────▶│Route│──────▶│Rx │                           │
│  └───┘       └─────┘       └───┘                           │
│    │           │             │                              │
│    │◀─50µs────▶│◀─30µs──────▶│                              │
│    │                         │                              │
│    └─────────130µs───────────┘                              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

______________________________________________________________________

## Kierunki rozwoju

### 1. TSN (Time-Sensitive Networking)

- Następca AFDX dla nowych projektów
- IEEE 802.1 standards
- Lepsza integracja z Ethernet standard

### 2. AFDX End System 2.0

- Wyższe przepustowości (1 Gbps)
- Lepsze mechanizmy redundancy

### 3. Hybrid networks

- AFDX + TSN w jednym systemie
- Migracja z AFDX do TSN

______________________________________________________________________

## Zadanie praktyczne

```
ZADANIE:
Zaprojektuj Virtual Links dla systemu avionics:

SYSTEMY:
├── Flight Control: 100 fps, 500 bytes/frame
├── Navigation: 50 fps, 1000 bytes/frame
├── Communication: 25 fps, 1500 bytes/frame
└── Monitoring: 10 fps, 200 bytes/frame

PYTANIA:
1. Oblicz wymagane BAG dla każdego systemu
2. Oblicz całkowitą przepustowość
3. Czy 100 Mbps wystarczy?

ROZWIĄZANIE:
1. BAG (najmniejszy możliwy):
   - Flight Control: BAG = 10 ms (100 fps)
   - Navigation: BAG = 20 ms (50 fps)
   - Communication: BAG = 40 ms (25 fps)
   - Monitoring: BAG = 100 ms (10 fps)

2. Przepustowość:
   - FC: 500 * 8 * 100 = 0.4 Mbps
   - Nav: 1000 * 8 * 50 = 0.4 Mbps
   - Comm: 1500 * 8 * 25 = 0.3 Mbps
   - Mon: 200 * 8 * 10 = 0.016 Mbps
   - SUMA: ~1.1 Mbps

3. Tak, 100 Mbps wystarczy z dużym zapasem (1.1% wykorzystania)
```

______________________________________________________________________

## Pytania kontrolne

1. Czym jest Virtual Link w AFDX?
1. Jak BAG wpływa na przepustowość?
1. Jak działa redundancja A/B w AFDX?
1. Dlaczego AFDX jest deterministyczny?

______________________________________________________________________

## Literatura

1. ARINC 664 Part 7, "Avionics Full-Duplex Switched Ethernet"
1. AFDX Specification, Airbus
1. IEEE 802.1, "Time-Sensitive Networking"
1. Boeing 787 Systems Documentation
