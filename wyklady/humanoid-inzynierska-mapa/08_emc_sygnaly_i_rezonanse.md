# Wykład 8: EMC, sygnały, drgania, rezonanse

## Czesc I: Wstep teoretyczny — sygnałowe wyzwania

### 1.1 Geneza — czyste srodowisko sygnalowe

W humanoidzie jednocześnie:
- Duże prądy i szybkie PWM
- Wrażliwe sensory (IMU, siła)
- Długie wiązki kablowe
- Konstrukcja z rezonansami

### 1.2 Efekt

Zakłócenia i rezonanse = szum + opóźnienie = degradacja sterowania!

---

## Czesc II: Widma i odpowiedzi czestotliwosciowe

### 2.1 FFT

Najważniejsze narzędzie diagnostyczne!

- Pik stały = rezonans/zakłócenie periodyczne
- Szerokopasmowy wzrost = szum EMC/aliasing

### 2.2 Odpowiedz czestotliwosciowa

Bode, sweep — do analizy obiektu

---

## Czesc III: Filtry

### 3.1 FIR vs IIR

| Typ | Zalety | Wady |
|-----|--------|------|
| FIR | Stabilny | Duży rząd, opóźnienie |
| IIR | Efektywny | Wrażliwy na kwantyzację |

### 3.2 Biquad

Typowe filtry jako kaskady biquad (IIR 2 rzędu)

---

## Czesc IV: Rezonanse mechaniczne

### 4.1 Analiza modalna

- Wartości własne = częstotliwości własne
- Wektory własne = kształty modów

### 4.2 W sterowaniu

Rezonans = wzmocnienie błędu w wąskim paśmie!

---

## Czesc V: EMC vs mechanika

### 5.1 Sygnały EMC

- Korelacja z PWM
- Zakłócenia na wielu kanałach
- Zależność od routingu

### 5.2 Sygnały mechaniczne

- Korelacja z obciążeniem/prędkością
- Dominujące piki = częstotliwości własne

---

## Czesc VI: Praktyka inzynierska

### 6.1 Checklisty

- [ ] Loguj widma w problematycznych stanach
- [ ] Mierz kontekst: temperatura, prąd
- [ ] Waliduj filtry pod kątem opóźnienia

---

## BONUS: Filtr jako maska

Jeśli filtr działa tylko w jednym "magicznym" ustawieniu → maskuje problem!

Dąż do rozwiązań które poprawiają metryki w szerokim zakresie warunków!

---

*(Koniec wykladu 8)*
