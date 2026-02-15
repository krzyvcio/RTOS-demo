# Recenzja wykładu: Systemy Czasu Rzeczywistego i Bezpieczeństwo Funkcjonalne

______________________________________________________________________

## 1. Ocena merytoryczna

### Zalety

**Kompleksowość podejścia**
Wyklad kompleksowo omawia systemy RTOS od podstaw teoretycznych az po praktyczne zastosowania w przemyśle. Materiał obejmuje zarówno aspekty teoretyczne (scheduler, synchronizacja), jak i praktyczne (implementacja w FreeRTOS, Zephyr, QNX).

**Praktyczne przykłady**
Bardzo dobre są przykłady z rzeczywistych branż:

- Robotyka (Boston Dynamics, drony)
- Avionika (ARINC 653, DO-178C)
- Automotive (ISO 26262, ASIL)
- Kosmonautyka (SEU, TMR)

**Podejście inżynierskie**
Wyklad nie jest zbyt teoretyczny - pokazuje realne problemy i ich rozwiazania. Sekcja "co zabija systemy w praktyce" jest bardzo cenna.

### Wady

**Chaotyczna organizacja**
Materiał jest trudny do sledzenia - wyklad przeskakuje pomiedzy tematami bez wyraznej struktury. Brakuje wyraznego podziału na czesci.

**Powtórzenia**
Niektóre tematy sa powtarzane wielokrotnie (np. priority inversion pojawia sie w wielu miejscach).

**Zbyt obszerny zakres**
Chciec objac wszystko - od podstaw po zaawansowane standardy - powoduje, ze kazdy temat jest potraktowany powierzchownie.

______________________________________________________________________

## 2. Ocena dydaktyczna

### Pozytywne elementy

| Element | Ocena |
|---------|-------|
| Przykłady z branż | 5/5 |
| Story-telling (case studies) | 5/5 |
| Checklisty projektowe | 4/5 |
| Kod i konfiguracje | 4/5 |
| Diagramy architektury | 4/5 |

### Problem

Brak progresji trudnosci - wyklad raz jest bardzo podstawowy (mutex, semafor), a raz zaawansowany (formal verification, seL4) bez wyraznych przejsc.

______________________________________________________________________

## 3. Najcenniejsze elementy wykładu

1. **Case study Boston Dynamics** - pokazuje jak teoria przeklada sie na praktyke
1. **Scenariusze awarii** - ucza na bledach innych
1. **Porównanie RTOS vs Linux PREEMPT_RT** - praktyczne wskazowki
1. **Checklista "co zabija systemy"** - bezcenna w praktyce

______________________________________________________________________

## 4. Rekomendacje

Wykład wymaga lepszej organizacji. Proponowana struktura:

1. **Podstawy** (1-2 wyklady)

   - Wprowadzenie do RTOS
   - Scheduler i taski
   - Podstawy synchronizacji

1. **Problemy i rozwiazania** (2-3 wyklady)

   - Deadlock, livelock, starvation
   - Priority inversion i protokoly
   - Struktury lock-free

1. **Projektowanie** (2-3 wyklady)

   - Architektura systemow RT
   - Partycjonowanie
   - Mixed-criticality

1. **Standardy i certyfikacja** (2 wyklady)

   - DO-178C, ARINC 653
   - ISO 26262, ASIL

1. **Praktyka** (1-2 wyklady)

   - Implementacja
   - Narzedzia (Tracealyzer)
   - Case studies

______________________________________________________________________

**Ocena ogólna: 4/5**
Wykład jest warty przejscia, ale wymaga lepszej organizacji materialu. Najwieksza wartosc stanowia praktyczne przyklady i case studies z przemyslu.
