# Wykład 5: Diagnostyka i condition monitoring (praktycznie)

## Cel
Zbudować praktyczny zestaw metryk i mechanizmów, które wykryją degradację zanim stanie się awarią.

> TL;DR: Diagnostyka bez kontekstu (tryb, prędkość, temperatura, obciążenie, czas) daje fałszywe alarmy. Najpierw standaryzujesz metryki i logi, potem dodajesz „inteligencję”.

## Co monitorujesz w maszynach wirujących
- temperatura (napęd, łożyska, elektronika),
- prąd/moment (obciążenie, tarcie),
- wibracje (piki widma, RMS, trend),
- jakość czasu rzeczywistego (missed deadlines, jitter),
- jakość komunikacji (dropouty, błędy).

## Online vs offline: rozdział odpowiedzialności
Diagnostyka online:
- szybka i konserwatywna,
- ma wspierać bezpieczeństwo i utrzymanie pracy (degradacja),
- nie może destabilizować RT (musi być lekka obliczeniowo).

Diagnostyka offline:
- cięższa analiza (korelacje, modele, porównania),
- służy do znalezienia przyczyny i planowania serwisu,
- korzysta z pełnych logów i danych surowych.

## Anomalie: proste metody, które dają największy zwrot
- progi + histereza + okno czasowe,
- trend (pochodna) i porównanie do baseline,
- analiza FFT i detekcja pików,
- korelacja z trybem pracy (stan procesu).

## Metryki, które zazwyczaj dają największą wartość
### Wibracje
- RMS wibracji w oknie czasowym,
- amplituda 3–5 dominujących pików FFT,
- trend RMS/pików w czasie (np. średnia krocząca).

### Napęd i obciążenie
- średni moment/prąd vs prędkość (profil „normalny”),
- odchylenie od profilu (residuum),
- liczba i czas saturacji momentu/prądu.

### Termika
- temperatura absolutna + tempo narastania,
- czas w wysokiej temperaturze (integral), bo to lepiej przewiduje degradację niż pojedynczy pik.

### Czas rzeczywisty i komunikacja
- p99/p99.9 czasu iteracji pętli,
- liczba missed deadlines w oknie,
- dropouty w komunikacji i ich korelacja z obciążeniem CPU/sieci.

## Dlaczego „kontekst” jest krytyczny
Ten sam prąd może oznaczać:
- normalne obciążenie w trybie A,
- degradację w trybie B.

Dlatego loguj zawsze:
- tryb,
- prędkość zadana/rzeczywista,
- temperaturę,
- warunki pracy.

## Baseline i „profil normalny”
Żeby wykrywać anomalie, potrzebujesz punktu odniesienia:
- baseline dla trybów pracy (osobno dla rozruchu, pracy ustalonej, zatrzymania),
- tolerancje zależne od prędkości/obciążenia,
- wersjonowanie baseline (zmiana mechaniki/serwis zmienia profil).

Prosty, praktyczny wzorzec:
- buduj „profil normalny” jako funkcję (tryb, prędkość) -> oczekiwany moment, wibracje, temperatura,
- alarmuj odchyłkę od profilu, a nie surową wartość.

## Alarmy: progi, trendy, eskalacja
Nie wszystko musi kończyć się stopem.
Praktyczna eskalacja:
- WARNING: trend rośnie, ale margines jest,
- DEGRADED: ogranicz osiągi (np. rampy/jerk, max prędkość),
- SAFE_STOP: gdy ryzyko uszkodzenia/niebezpieczeństwa jest wysokie.

## Typowe pułapki
- alarmy bez histerezy (flapping),
- brak okien czasowych (reakcja na pojedyncze piki),
- brak rozróżnienia trybów pracy (rozruch vs praca ustalona),
- logi, które wpływają na RT (diagnostyka psuje to, co ma mierzyć),
- metryki „ładne w dashboardzie”, ale bez wartości przy debugowaniu (brak kontekstu).

## Checklisty
- Metryki są spójne online i offline (te same definicje).
- Masz alarmy trendowe, nie tylko progowe.
- Logi nie rozwalają RT (asynchroniczne).
- W trybie degradacji system ma ograniczenia (np. redukcja prędkości/momentu).

## Zadania (praktyka)
1. Zdefiniuj 10 metryk (wibracje/napęd/termika/RT/komunikacja) i opisz, czy są online czy offline.
2. Zaproponuj baseline dla 2 trybów pracy i sposób wersjonowania.
3. Zaprojektuj eskalację alertów: progi + trend + okno czasowe + histereza.

## Pytania do studentów
1. Dlaczego „kontekst” (tryb, prędkość, temperatura) jest konieczny, aby uniknąć fałszywych alarmów?
2. Jakie metryki diagnostyczne mogą wpływać na RT i jak temu zapobiec architektonicznie?
3. Jak dobrać okno czasowe i histerezę, aby ograniczyć flapping alertów?
4. Jak zaprojektujesz baseline, aby był odporny na zmiany po serwisie (wymiana elementu, kalibracja)?

## Projekty studenckie
- „Health score”: agregator metryk do jednego wskaźnika stanu podsystemu (napęd, termika, wibracje, RT).
- „Trend detector”: detekcja trendów (RMS, piki FFT, temperatura) z eskalacją WARNING/DEGRADED/SAFE_STOP.
- „Baseline manager”: wersjonowanie profili normalnej pracy i porównanie przed/po zmianach.

## BONUS
- Najbardziej wartościowe alarmy to te, które mówią „dlaczego” i „co dalej”: przy każdym alercie loguj nie tylko wartość, ale też metrykę trendu, tryb i rekomendowaną akcję (degradacja/inspekcja).
