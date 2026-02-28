# Wykład 9: Bezpieczeństwo funkcjonalne, logika, nadzór

## Czesc I: Wstep teoretyczny — bezpieczenstwo ponad wszystko

### 1.1 Geneza — niezaleznosci od inteligencji

Bezpieczeństwo funkcjonalne to warstwa która:
- Nie optymalizuje jakości
- Nie "walczy" o cel
- Pilnuje granic dopuszczalnego ryzyka!

### 1.2 W humanoidzie

- Ograniczanie energii/prędkości przy człowieku
- Bezpieczne zatrzymanie przy awarii
- Kontrola stanów kontaktu
- Nadzór: temp, prąd, napięcie, czas

---

## Czesc II: FSM bezpieczenstwa

### 2.1 Stany

```
NORMAL → WARNING → DEGRADED → SAFE_STOP → FAULT → RECOVERY
```

### 2.2 Zasady

- Przejścia do bezpieczniejszych = natychmiastowe
- Wyjście = po warunkach + histereza

---

## Czesc III: Watchdogi

### 3.1 Wielopoziomowe

- Lokalny w napędzie
- W kontrolerze RT
- Na poziomie OS

### 3.2 Zasada

Brak heartbeat → safe state!

---

## Czesc IV: Modele formalne

### 4.1 LTL/CTL

Logika temporalna do weryfikacji własności:
- "Zawsze gdy X, w końcu Y"
- "Nigdy jednocześnie A i B"

### 4.2 Model checking

Automatyczne sprawdzanie własności na modelu FSM

---

## Czesc V: Praktyka inzynierska

### 5.1 Safe state

- Odcięcie momentu (UWAGA: może być niebezpieczne!)
- Kontrolowane hamowanie (wymaga sprawnych napędów)
- Postawa minimalnej energii

### 5.2 Checklisty

- [ ] Każdy błąd ma określony safe response
- [ ] Przejścia z histerezą (brak flapping)
- [ ] Watchdogi wielopoziomowe i testowane

---

## Czesc VI: Pytania do dyskusji

1. Jakie są minimalne stany FSM bezpieczeństwa?
2. Co zyskujesz przez rozdział kanałów?

---

## BONUS: Safety powinno byc nudne

Proste reguły, deterministyczne przejścia, twarde limity!

Każda "sprytna" logika w safety = ryzyko luki!

---

*(Koniec wykladu 9)*
