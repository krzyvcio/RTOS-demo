# Wykład 3: EtherCAT w praktyce (dla sterowania i synchronizacji)

## Czesc I: Wstep teoretyczny — czym jest EtherCAT i dlaczego jest taki popularny

### 1.1 Geneza — dlaczego potrzebujemy specjalnej sieci

Proszę wyobrazić sobie sytuację: mamy wirówkę z trzema osiami (wirnik, pokrywa, chłodzenie) i wieloma czujnikami (temperatura, wibracje, prędkość). Chcemy, żeby wszystko działało z opóźnieniem poniżej 1 milisekundy.

**Co możemy użyć?**

| Opcja | Opóźnienie | Determinizm | Uwagi |
|-------|------------|-------------|-------|
| Ethernet (TCP) | 1-100 ms | Niestabilny | Za wolny |
| Ethernet (UDP) | 0.1-10 ms | Średni | Lepszy, ale ryzykowny |
| CAN | 1-10 ms | Dobry | Za wolny dla ruchu |
| **EtherCAT** | 0.1-1 ms | Bardzo dobry | Idealny dla ruchu |
| EtherNet/IP | 1-10 ms | Średni | Kompromis |

**Dlaczego EtherCAT jest taki szybki?**

Tradycyjny Ethernet:
```
Komputer A → Switch → Komputer B → Switch → Komputer C
    |           |          |           |
    v           v          v           v
Każde urządzenie = osobne połączenie = opóźnienie
```

EtherCAT:
```
Master → [Dane] → Slave1 → [Slave1 wstawia swoje dane] → Slave2 → ... → Master
                    |                                        |
                    v                                        v
            "W locie" przetwarzanie              Wracają wszystkie dane
```

### 1.2 Jak to działa "w locie"

Wyobraźmy sobie strukturę ramki EtherCAT:

```
[ nagłówek ETH ] [ 命令 0x1000 ] [ dane_slave1 ] [ dane_slave2 ] [ ... ] [ CRC ]
                     ^                   ^                  ^
                     |                   |                  |
                Komenda           Slave1 wyciąga       Slave2 wyciąga
                EtherCAT           swoje dane          swoje dane
                                   i wstawia          i wstawia
                                   odpowiedź          odpowiedź
```

To jest "przetwarzanie w locie" — ramka praktycznie nie zatrzymuje się w każdym urządzeniu.

---

## Czesc II: Co EtherCAT wnosi do sterowania

### 2.1 Trzy kluczowe cechy

| Cecha | Co daje | Przykład zastosowania |
|-------|---------|----------------------|
| **Deterministyczna wymiana** | Stały cykl, niski jitter | Sterowanie prędkością |
| **Distributed Clocks** | Synchronizacja czasu | Wiele osi, pomiary fazowe |
| **Wysoka przepustowość** | Wiele danych w jednym cyklu | Telemetria + sterowanie |

### 2.2 Elementy architektury

```
                    +------------------+
                    |    MASTER        |
                    | (Linux RT/RTOS)  |
                    +--------+---------+
                             |
                    +--------v---------+
                    |   EtherCAT       |
                    |   Network        |
                    +--------+---------+
                             |
        +--------------------+--------------------+
        |                    |                    |
+--------v-------+  +--------v-------+  +--------v-------+
|   SLAVE 1     |  |   SLAVE 2     |  |   SLAVE N       |
|  (Napęd 1)   |  |  (Napęd 2)    |  |  (I/O, czujniki)|
+--------------+  +--------------+  +-----------------+
```

**Master:**
- Kontroluje timing
- Inicjuje cykl
- Przetwarza dane

**Slave:**
- Napędy (sterowanie prądem/prędkością)
- Moduły I/O (wejścia/wyjścia analogowe/digitale)
- Enkodery (pomiar pozycji/prędkości)

---

## Czesc III: Dane — cykliczne vs acykliczne

### 3.1 Podział — kluczowa decyzja

**Dane cykliczne (cyclic):**
- Potrzebne do sterowania w każdej iteracji
- Zadane prędkości, zmierzone prędkości, stany
- Wymiana każdy cykl

**Dane acykliczne (acyclic):**
- Konfiguracja, parametry, diagnostyka
- Nie potrzebne "na już"
- Rate-limited

### 3.2 Przykładowa tabela

| Sygnal | Typ | Częstotliwość | Uzasadnienie |
|--------|-----|---------------|--------------|
| omega_set | Cykliczny | Co cykl | Sterowanie |
| omega_meas | Cykliczny | Co cykl | Sprzężenie zwrotne |
| u_cmd | Cykliczny | Co cykl | Aktuacja |
| temperature | Cykliczny | Co 10 cykli | Wystarczy wolniej |
| fault_status | Cykliczny | Co cykl | Bezpieczeństwo |
| config_Kp | Acykliczny | Na żądanie | Zmiana parametrów |
| log_data | Acykliczny | Co 100 cykli | Diagnostyka |

### 3.3 Zasada

> Do danych cyklicznych wrzucasz **minimum potrzebne** do stabilnego sterowania i safety. Wszystko inne ograniczasz częstotliwościowo albo przenosisz poza cykl.

---

## Czesc IV: Synchronizacja — Distributed Clocks (DC)

### 4.1 Problem synchronizacji

Wyobraźmy sobie 3 osie wirówki, każda z własnym enkoderem. Chcemy:
- Zmierzyć prędkość każdej osi
- Porównać fazę (czy są synchroniczne)
- Wykryć drgania skrętne

Problem: każdy enkoder ma własny zegar:

```
Czas mastera:  0.000000 s
Czas slave 1: +1 μs (offset)
Czas slave 2: -2 μs (offset)  
Czas slave 3: +3 μs (offset)
```

Przy cyklu 1 ms i precyzji 1 μs — to jest problem!

### 4.2 Rozwiązanie: Distributed Clocks

EtherCAT ma wbudowany mechanizm synchronizacji:

1. **Jeden zegar referencyjny** (DC Master lub jeden slave)
2. **Offset compensation** — każdy slave zna różnicę między swoim zegarem a DC
3. **Sync pulse** — sygnał "teraz"

```
Bez DC:               Z DC:
                    
Slave 1: ----  ----  ----      Slave 1: |---| |---| |---|
Slave 2: ---  ---  ---  ---     Slave 2: |---| |---| |---|
Slave 3: ----  ----  ----      Slave 3: |---| |---| |---|
                 ^
           rozjechane              zsynchr.
```

### 4.3 Kiedy DC ma sens

**DC ma sens gdy:**
- Masz wiele osi i chcesz spójnego samplingu
- Porównujesz sygnały między slaveami (faza)
- Potrzebujesz precyzyjnej korelacji czasowej

**DC NIE ma sens gdy:**
- Masz jeden slave
- Wystarcza "w przybliżeniu" synchroniczne dane

---

## Czesc V: Dobor cyklu

### 5.1 Kompromis

**Za wolny cykl:**
- Pętla widzi stare dane
- Spada pasmo
- Rośnie błąd regulacji

**Za szybki cykl:**
- Rośnie obciążenie CPU
- Rośnie jitter
- Ryzyko dropoutów

### 5.2 Metoda praktyczna

```
KROK 1: Wybierz cykl konserwatywnie (np. 1 ms)

KROK 2: Zmierz jitter/WCRT i jakość regulacji
   - p99.9 < 80% cyklu? 
   - stabilność regulatora OK?

KROK 3: Iteruj
   - Skracaj cykl tylko jeśli determinizm trzyma
```

### 5.3 Typowe cykle

| Zastosowanie | Typowy cykl | Uzasadnienie |
|--------------|-------------|--------------|
| Serwonapęd | 50-200 μs | FOC wymaga częstego próbkowania |
| Sterowanie wirnika | 0.5-2 ms | Wystarczające dla mechaniki |
| Robot manipulacyjny | 1-4 ms | Pozycja, trajektorie |
| Wizja + sterowanie | 10-100 ms | Wizja jest wolna |

---

## Czesc VI: Co psuje EtherCAT w praktyce

### 6.1 Najczęstsze problemy

| Problem | Skutek | Rozwiązanie |
|---------|--------|-------------|
| Zbyt duży cykl | Sterowanie widzi stare dane | Skróć cykl |
| Zbyt mały cykl | CPU nie wyrabia, jitter rośnie | Wydłuż cykl |
| Telemetria w tym samym cyklu | Obciążenie, jitter | Rate limiting |
| Brak pomiaru WCRT | "Działa na oko" | Dodaj pomiary |

### 6.2 Zasada telemetrii

> Traktuj telemetrię jak "gościa": może działać tylko wtedy, gdy nie psuje cyklu.

---

## Czesc VII: Watchdog i awarie komunikacji

### 7.1 Założenie inżynierskie

**Komunikacja czasem przestaje być idealna.** Projektujemy zachowanie:

### 7.2 Po stronie slave

```c
// Watchdog w napędzie
void watchdog_check() {
    static uint32_t last_heartbeat = 0;
    uint32_t now = get_tick_ms();
    
    if (now - last_heartbeat > WATCHDOG_TIMEOUT_MS) {
        // Brak komunikacji → safe state
        set_pwm(0);           // Wyłącz PWM
        engage_brake();       // Załącz hamulec
        set_fault(FAULT_WD);  // Ustaw flagę błędu
    }
}
```

### 7.3 Po stronie master

```c
// Reakcja mastera na brak odpowiedzi
enum MasterReaction {
    MASTER_IGNORE,      // Ignoruj (niebezpieczne!)
    MASTER_DEGRADE,     // Ogranicz prędkość
    MASTER_SAFE_STOP,  // Natychmiast stop
    MASTER_FAULT       // Przejdź do fault
};
```

### 7.4 Zasada

> Bez watchdogów awaria komunikacji może oznaczać utrzymanie ostatniego sterowania w nieskończoność.

---

## Czesc VIII: Integracja z napedami

### 8.1 Podział pętli

W praktyce:
- **Pętla prądu** — w napędzie (falownik)
- **Pętla prędkości** — w napędzie lub masterze (zależnie)
- **Koordynacja** — w masterze

### 8.2 Zasada bezpieczeństwa

> Warstwa napędu musi umieć bezpiecznie przejść w stan ograniczony nawet bez mastera (watchdog).

---

## Czesc IX: Higiena sieci i izolacja od IT

### 9.1 Problem mieszania ruchu

Jeśli na tym samym sprzęcie mieszasz:
- Ruch sterujący (EtherCAT)
- Telemetrię
- Zdalny dostęp

→ Zwiększasz ryzyko jitteru!

### 9.2 Najbezpieczniejszy wzorzec

```
Sieć ruchu (EtherCAT) ←→ Traktuj jako osobny świat
                                          ↓
Telemetria i integracje ←→ Kanał, który nie może zakłócić RT
```

---

## Czesc X: Podsumowanie i checklisty

### Checklisty:

- [ ] Wybierasz cykl EtherCAT świadomie (z pomiarów, nie "wydaje się")
- [ ] Ustalasz, które pętle są w napędzie, a które w masterze
- [ ] Stosujesz Distributed Clocks, jeśli potrzebujesz spójnego samplingu
- [ ] Oddzielasz krytyczne PDO od niekrytycznych danych

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Cykliczne = minimum | Sterowanie + safety |
| Acykliczne = rate-limited | Konfiguracja, diagnostyka |
| Jeden zegar | Unikaj beat frequencies |
| Watchdog zawsze | Komunikacja bywa zawieść |

---

## Czesc XI: Pytania do dyskusji

1. Jakie sygnały muszą być w danych cyklicznych, a które mogą być acykliczne?
2. Kiedy synchronizacja (DC) realnie poprawia jakość sterowania?
3. Jakie są konsekwencje zbyt krótkiego cyklu komunikacji?
4. Jak zaprojektujesz zachowanie przy awarii komunikacji?

---

## Czesc XII: Zadania praktyczne

### Zadanie 1: Tabela sygnałów

Zrób tabelę sygnałów i oznacz: "cykliczne" vs "acykliczne".

### Zadanie 2: Logika watchdogów

Zdefiniuj co robi slave, co robi master, jakie są warunki powrotu.

### Zadanie 3: Plan testów

Zaproponuj plan testów:
- Przeciążenie CPU mastera
- Duża telemetria
- Dropout komunikacji

---

## BONUS: Telemetria jako gość

Jeśli integrujesz magistralę ruchu, traktuj telemetrię jak "gościa": może działać tylko wtedy, gdy nie psuje cyklu.

To powinno być twardo wymuszone w architekturze — limity, osobne wątki/procesy.

---

*(Koniec wykladu 3)*
