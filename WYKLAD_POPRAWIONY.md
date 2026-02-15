# Wykład: Systemy Czasu Rzeczywistego i Bezpieczeństwo Funkcjonalne

## Spis treści

1. [Inwersja priorytetów](./slajdy/01_inwersja_priorytetow.md)
2. [Protokół pułapu priorytetu (PCP)](./slajdy/02_protokol_pułapu_priorytetu.md)
3. [Bounding Jitter](./slajdy/03_bounding_jitter.md)
4. [WCET - Worst-Case Execution Time](./slajdy/04_wcet.md)
5. [Partycjonowanie czasowe](./slajdy/05_partycjonowanie_czasowe.md)
6. [Harmonogramowanie mieszanej krytyczności](./slajdy/06_harmonogramowanie_mieszanej_krytycznosci.md)
7. [Struktury bez blokad](./slajdy/07_struktury_bez_blokad.md)
8. [Tickless Idle](./slajdy/08_tickless_idle.md)
9. [ARINC 653](./slajdy/09_arinc_653.md)
10. [ISO 26262 (ASIL)](./slajdy/10_iso_26262.md)
11. [DO-178C/DO-254](./slajdy/11_do_178c.md)
12. [AFDX](./slajdy/12_afdx.md)
13. [Izolacja rdzeni (AMP/Big-Little)](./slajdy/13_izolacja_rdzeni.md)
14. [Safety Monitor / Watchdog](./slajdy/14_safety_monitor.md)
15. [NMI - Non-Maskable Interrupt](./slajdy/15_nmi.md)
16. [MPU - Memory Protection Unit](./slajdy/16_mpu.md)
17. [Deterministyczne DMA](./slajdy/17_dma_deterministyczne.md)
18. [Harmonogramowanie Time-Triggered](./slajdy/18_harmonogramowanie_tt.md)
19. [Safety Case](./slajdy/19_safety_case.md)

---

## Przegląd tematów

| Temat | Branża | Kluczowe pojęcie |
|-------|--------|------------------|
| Inwersja priorytetów | Automotive, Robotyka | Priorytety zadań |
| PCP | Automatyka przemysłowa | Synchronizacja |
| Bounding Jitter | Aerospace | Determinizm |
| WCET | Avionics, Automotive | Czas wykonania |
| Partycjonowanie czasowe | Avionics | Izolacja |
| Mieszana krytyczność | Automotive, Medical | Harmonogramowanie |
| Struktury bez blokad | IoT, Automotive | Lock-free |
| Tickless Idle | IoT, Wearables | Energooszczędność |
| ARINC 653 | Aerospace | Standardy |
| ISO 26262 | Automotive | Certyfikacja |
| DO-178C | Aerospace | Certyfikacja |
| AFDX | Aerospace | Komunikacja |
| Izolacja rdzeni | Embedded | Architektura |
| Safety Monitor | Robotics, Automotive | Bezpieczeństwo |
| NMI | Automotive, Medical | Przerwania |
| MPU | IoT, Automotive | Ochrona pamięci |
| DMA | Data Acquisition | Transfer danych |
| Time-Triggered | Automotive | Harmonogramowanie |
| Safety Case | Aerospace, Medical | Dokumentacja |

---

## Zalecana kolejność przerabiania

### Część I: Podstawy synchronizacji (1-3)
Inwersja priorytetów → PCP → Jitter

### Część II: Analiza czasowa (4)
WCET jako fundament gwarancji czasowych

### Część III: Izolacja i partycjonowanie (5-6, 9, 13, 16)
Partycjonowanie czasowe → Mieszana krytyczność → ARINC 653 → Izolacja rdzeni → MPU

### Część IV: Mechanizmy bezpieczeństwa (7-8, 14-15, 17-18)
Struktury bez blokad → Tickless → Watchdog → NMI → DMA → Time-Triggered

### Część V: Certyfikacja i standardy (10-12, 19)
ISO 26262 → DO-178C → AFDX → Safety Case