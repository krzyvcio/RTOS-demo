# Wspólne cechy systemów RTOS w różnych branżach

______________________________________________________________________

## 1. Wprowadzenie

Systemy czasu rzeczywistego występują w wielu branżach, ale mimo różnic w zastosowaniach mają wspólne fundamenty. Niezależnie czy to sterowanie lotem, hamulce w samochodzie, czy robot przemysłowy - zasady projektowania są podobne.

______________________________________________________________________

## 2. Wspólne wymagania

### 2.1. Gwarancja czasowa

Każdy system RTOS musi gwarantować czas reakcji. To wspólne dla wszystkich branż:

| Branża | Przykład | Wymaganie czasowe |
|--------|----------|-------------------|
| Avionika | Sterowanie lotem | < 1 ms |
| Automotive | System hamulcowy | < 5 ms |
| Robotyka | Pętla sterowania | < 1 ms |
| Medycyna | Defibrylator | Natychmiast |

### 2.2. Izolacja błędów

Każdy system musi izolować błędy. Awaria jednego modułu nie może zniszczyć całego systemu.

Sposoby izolacji:

- Partycjonowanie pamięci (MPU)
- Partycjonowanie czasowe
- Hyperwizory
- Oddzielne procesy

### 2.3. Mechanizmy bezpieczeństwa

Wspólne mechanizmy watchdog i safe mode:

- Watchdog monitoruje żywotność tasków
- Safe mode przy awarii - ograniczona funkcjonalność, ale bezpieczeństwo
- Graceful degradation - kontrolowana degradacja przy przeciążeniu

______________________________________________________________________

## 3. Wspólne problemy

### 3.1. Synchronizacja

Każdy system RTOS zmaga się z tymi samymi problemami:

**Współdzielone zasoby**

- Dostęp do sprzętu (SPI, CAN, UART)
- Wspólne struktury danych
- Komunikacja między zadaniami

**Rozwiązania**

- Mutexy (z protokołami dziedziczenia priorytetów)
- Kolejki komunikatów
- Semafore
- Struktury lock-free

### 3.2. Priority Inversion

Problem uniwersalny - występuje wszędzie:

```
Zadanie L (niski priorytet) trzyma mutex
Zadanie H (wysoki priorytet) czeka na mutex
Zadanie M (średni priorytet) wykonuje się
→ Zadanie H stoi mimo najwyższego priorytetu
```

Rozwiązania:

- Priority Inheritance
- Priority Ceiling Protocol

______________________________________________________________________

## 4. Wspólne wzorce architektoniczne

### 4.1. Pipeline przetwarzania

Wzorzec wspólny dla robotyki, przetwarzania sygnałów i sterowania:

```
Sensory → Filtracja → Estymacja → Sterowanie → Aktuatory
```

Każdy etap to osobne zadanie, komunikacja przez kolejki.

### 4.2. Warstwy systemu

Każdy system ma podobną strukturę warstw:

| Warstwa | Funkcja |
|---------|---------|
| HAL/BSP | Sterowniki sprzętowe |
| RTOS Core | Scheduler, IPC |
| Warstwa RT | Pętle sterowania |
| Warstwa decyzyjna | Planowanie |
| Warstwa komunikacji | Sieć, interfejsy |

### 4.3. Podział na hard i soft RT

Wspólny wzorzec to podział:

- **Hard RT** (MCU, RTOS) - sterowanie, bezpieczeństwo
- **Soft RT** (SoC, Linux) - percepcja, planowanie, UI

Przykład: robot Boston Dynamics

- MCU = sterowanie silników, balans
- SoC = SLAM, planowanie, wizja

______________________________________________________________________

## 5. Wspólne narzędzia i techniki

### 5.1. Analiza czasowa

W każdej branży stosuje się podobne techniki:

- **WCET Analysis** - najgorszy przypadek czasu wykonania
- **Response Time Analysis** - czasy odpowiedzi
- **Schedulability Analysis** - sprawdzenie planowalności

### 5.2. Debugowanie

Narzędzia są wspólne:

- Tracealyzer - wizualizacja timeline
- Osciloskopy logiczne - timing
- Profilery - obciążenie CPU

### 5.3. Testowanie

Podobne podejście:

- Testy jednostkowe
- Testy integracyjne
- Testy obciążeniowe
- Symulacja awarii

______________________________________________________________________

## 6. Wspólne standardy i wzorce

### 6.1. Standardy bezpieczeństwa

| Standard | Branża | Poziomy |
|----------|--------|---------|
| DO-178C | Lotnictwo | A-E |
| ISO 26262 | Automotive | ASIL A-D |
| IEC 61508 | Przemysł | SIL 1-4 |
| IEC 62304 | Medycyna | A-C |

Wspólna idea: poziomy krytyczności wymagają różnej rigorii weryfikacji.

### 6.2. Wzorce komunikacji

Wspólne wzorce w każdej branży:

- **Publish/Subscribe** - dystrybucja danych
- **Request/Response** - komunikacja synchroniczna
- **Message Queue** - asynchroniczna wymiana danych
- **Shared Memory** - dla wysokiej przepustowości (z synchronizacją)

______________________________________________________________________

## 7. Wspólne zasady projektowania

### 7.1. Złote zasady

1. **Najlepszy mutex to ten, którego nie potrzebujesz**

   - Unikaj współdzielonego stanu
   - Message zamiast shared memory

1. **Izolacja jest kluczowa**

   - Oddziel krytyczne od niekrytycznych
   - Miej jasne interfejsy

1. **Mierz WCET wcześnie**

   - Nie zgaduj, mierz
   - Zapas w budgetcie (50-70%)

1. **Planuj awarie**

   - Watchdog to podstawa
   - Safe mode musi działać

### 7.2. Antywzorce

Wspólne błędy w każdej branży:

- Zbyt wiele mutexów
- Dynamiczna alokacja w ścieżce RT
- Printf w pętli sterowania
- Jeden task robiący wszystko

______________________________________________________________________

## 8. Różnice w implementacji

### 8.1. Hardware

| Cecha | Embedded MCU | SoC |
|------|--------------|-----|
| Zegar | Stały, deterministyczny | Zmienny (governor) |
| Rdzenie | 1-4 | 4+ |
| Pamięć | Ograniczona | Duża |
| Izolacja | MPU | Hyperwizor |

### 8.2. Oprogramowanie

| Cecha | Hard RTOS | Linux RT |
|------|-----------|----------|
| Scheduler | Fixed priority | CFS + PREEMPT |
| Izolacja | Partycje | cgroups |
| Certyfikacja | Łatwiejsza | Trudniejsza |
| Elastyczność | Mniejsza | Większa |

______________________________________________________________________

## 9. Podsumowanie

### Co łączy wszystkie systemy RTOS:

1. **Wymagania czasowe** - deadline musi być dotrzymany
1. **Izolacja błędów** - awaria jednego modułu nie psuje innych
1. **Problemy synchronizacji** - mutex, deadlock, priority inversion
1. **Narzędzia** - WCET, Tracealyzer, profilery
1. **Zasady** - izolacja, message passing, planowanie awarii

### Co różni branże:

1. **Rygorystyczność** - lotnictwo vs przemysł
1. **Standardy** - DO-178C vs ISO 26262
1. **Narzędzia** - specyficzne dla branży
1. **Procesy** - poziom dokumentacji i weryfikacji

Mimo różnic, fundament jest wspólny: **przewidywalność w najgorszym przypadku**. To łączy wszystkie systemy czasu rzeczywistego.
