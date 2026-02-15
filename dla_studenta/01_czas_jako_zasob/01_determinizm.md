# Determinizm

## Definicja

**Determinizm** w systemach RTOS oznacza, że dla tych samych wejść i tego samego stanu początkowego, system zawsze zwróci te same wyniki w tym samym czasie. Brak niespodzianek. Brak "czasem działa, czasem nie".

> W systemie deterministycznym możesz policzyć najgorszy możliwy czas wykonania - i on **zawsze** się sprawdzi.

---

## Analogia do przyrody

### 🌊 Pływy morskie

Pływy są deterministyczne - możesz obliczyć dokładną godzinę przypływu za 100 lat. Siła grawitacji Księżyca i Słońca działa zawsze tak samo.

**Co gdyby pływy NIE były deterministyczne?**
- Żeglarze nie mogliby planować tras
- Porty nie wiedziałyby, kiedy mogą przyjmować statki
- Budowniczowie tam nie wiedzieliby, jak wysokie je budować

### 🧬 Bicie serca (w zdrowym organizmie)

Serce bije w rytmie. Każde uderzenie jest przewidywalne. Gdy serce traci determinizm - mamy arytmię, która może być śmiertelna.

### ⚡ Obwód elektryczny

Przycisk włącza żarówkę. Zawsze. To jest determinizm. Gdyby "czasem działał, czasem nie" - nie mielibyśmy cywilizacji opartej na elektryczności.

---

## Podobieństwo do systemów informatycznych

### Baza danych transakcyjna

```sql
BEGIN TRANSACTION;
UPDATE konto SET saldo = saldo - 100 WHERE id = 1;
UPDATE konto SET saldo = saldo + 100 WHERE id = 2;
COMMIT;
```

Transakcja jest deterministyczna: albo obie operacje się udadzą, albo żadna. Nie ma "czasem jedna, czasem druga".

### Testy jednostkowe

```python
def test_dodawanie():
    assert dodaj(2, 3) == 5  # Zawsze prawda
```

Test deterministyczny przechodzi zawsze lub nigdy. Testy "flaky" (czasem zielone, czasem czerwone) są **niedeterministyczne** - i są koszmarem każdego programisty.

### DNS

Gdy wpisujesz `google.com`, DNS zawsze zwraca ten sam IP (dla tej samej konfiguracji). Gdyby DNS był niedeterministyczny - internet by nie działał.

---

## Dlaczego determinizm jest problemem?

### Problem 1: Cache i branch prediction

Nowoczesne CPU są **niedeterministyczne** z punktu widzenia czasu:
- Cache hit: 4 cykle
- Cache miss: 100+ cykli

```c
// To samo zadanie, różny czas!
if (dane[wazny_indeks]) {  // Cache hit czy miss?
    wykonaj_cos();
}
```

**Rozwiązanie**: Cache locking, preheating cache, unikanie branchy.

### Problem 2: Systemy operacyjne ogólnego przeznaczenia

Linux może w każdej chwili:
- Przełączyć kontekst na inny proces
- Obsłużyć przerwanie sieciowe
- Uruchomić garbage collector

**Rozwiązanie**: RT-patch (PREEMPT_RT), CPU isolation, scheduling policies.

### Problem 3: Sieci i I/O

Sieć jest z natury niedeterministyczna:
- Pakiet może przyjść za 1ms lub 100ms
- Może nie przyjść wcale

**Rozwiązanie**: Timeouts, redundancy, local buffering.

---

## Jak sobie radzić z problemami determinizmu?

### W hardware:

1. **Cache locking** - zablokuj krytyczne dane w cache
2. **Tightly Coupled Memory (TCM)** - pamięć bez cache, gwarantowany czas dostępu
3. **Deterministic interconnect** - szyny z gwarantowaną latencją

### W software:

```c
// ZŁE: Niedeterministyczne
void przetworz() {
    if (rand() % 2) {  // Losowość!
        opcja_a();
    } else {
        opcja_b();
    }
}

// DOBRE: Deterministyczne
void przetworz(int tryb) {
    if (tryb == TRYB_A) {
        opcja_a();
    } else {
        opcja_b();
    }
}
```

### W architekturze:

```
┌─────────────────────────────────────────────────────────┐
│                    TIME-TRIGGERED DESIGN                │
│                                                         │
│  Tick 0:   [Task A] ──────────────────────────>│        │
│  Tick 1:   [Task B] ─────────────────>│         │        │
│  Tick 2:   [Task C] ───────────────────────────>│        │
│             │                                    │        │
│             └────────── Deterministic! ─────────┘        │
└─────────────────────────────────────────────────────────┘
```

---

## Jak świat radzi sobie z determinizmem?

### Lotnictwo: DO-178C

Każda linia kodu musi być przewidywalna. Testy pokrywają 100% ścieżek. Żadnych "może działa, może nie".

### Kolej: ETCS (Europejski System Sterowania Pociągami)

System oblicza maksymalny czas hamowania i zawsze zakłada **najgorszy przypadek**. Pociąg nigdy nie przejedzie na czerwonym - bo system deterministycznie wylicza, gdzie musi zacząć hamować.

### Energetyka: Protection relays

Przekaźniki ochronne muszą zadziałać w określonym czasie przy zwarciu. Milisekundy decydują o awarii sieci. Determinizm jest krytyczny.

---

## Pytania do przemyślenia

1. Czy Twój system zawsze reaguje w tym samym czasie na to samo zdarzenie?
2. Jak mierzysz determinizm? Czy masz metryki jitter?
3. Co się dzieje w najgorszym możliwym momencie - gdy cache jest zimny, a CPU obciążone?

---

## Quiz

**Pytanie**: System ma czas odpowiedzi "średnio 5ms, max 50ms". Czy jest deterministyczny?

**Odpowiedź**: Nie w sensie RTOS. Średnia nic nie znaczy. Deterministyczny system ma gwarantowany **górny limit**, który zawsze jest dotrzymywany. Jeśli 50ms to gwarantowane maximum - wtedy tak. Jeśli 50ms to obserwowane maximum, ale teoretycznie może być więcej - nie.

---

## Wskazówka zapamiętywania

> **Determinizm = Brak niespodzianek**
>
> Jeśli możesz odpowiedzieć na pytanie "ile maksymalnie czasu to zajmie?" i być pewien na 100% - masz determinizm.
>
> Jeśli mówisz "zazwyczaj około..." - nie masz determinizmu.