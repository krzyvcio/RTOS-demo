# Wykład 3: Symulacja sterowania (PI/PID, notch, LQR/MPC) i dyskretyzacja

## Część I: Wstęp teoretyczny - Dlaczego symulacja sterowania jest fundamentem

### Geneza: od "działa w teorii" do "działa w praktyce"

Teoria sterowania jest piękna - daje wam narzędzia do analizy stabilności, projektowania regulatorów, optymalizacji odpowiedzi. Równania są eleganckie, wykresy Bode'a są przejrzyste, kryterium Nyquista jest rozstrzygające. Ale jest jeden problem: **teoria zakłada czas ciągły, a rzeczywistość jest dyskretna**.

W świecie ciągłym:
- Wyjście regulatora zmienia się w każdej chwili
- Pomiar jest dostępny natychmiast
- Obliczenia są "za free"

W świecie dyskretnym (cyfrowym):
- Regulator oblicza nowe wyjście co T sekund (np. co 1 ms)
- Pomiar jest próbkowany - mamy wartość tylko w dyskretnych chwilach
- Obliczenia kosztują czas, który może przekroczyć period próbkowania

I tu pojawia się "przepaść implementacyjna" - teoria mówi "stabilny", ale implementacja jest "niestabilna", bo nie uwzględniłeś dyskretyzacji, opóźnień, kwantyzacji.

### Dlaczego zaczynamy od PI/PID, a nie od "mądrzejszych" metod

PI/PID to nie jest "prymitywne sterowanie" - to fundament, na którym buduje się wszystko inne. Powody:
1. **Prostota implementacji** - proste równanie, prosta analiza
2. **Zrozumiałość** - każdy parametr ma fizyczny sens
3. **Odporność** - działa "wystarczająco dobrze" dla większości przypadków
4. **Debugowalność** - gdy coś nie działa, wiadomo gdzie szukać

LQR (Linear Quadratic Regulator) i MPC (Model Predictive Control) to "broń ciężka" - używamy ich, gdy:
- Mamy wiele współzależnych zmiennych stanu
- Mamy ograniczenia, które musimy respektować
- Mamy model wystarczająco dobry dla optymalizacji

Ale nawet LQR/MPC potrzebują "higieny" dyskretnej - anti-windup, saturacje, filtry. Bez tego żadna "zaawansowana" metoda nie zadziała.

### Przemówienie Profesora

Siedziałem kiedyś na review projektu, gdzie zespół chwalił się " MPC controllerem nowej generacji". Piękny algorytm, nagrody za publikacje, wszystko idealne.

Pytanie: "Jak radzi sobie z saturacjami?"

Cisza.

" A z opóźnieniem? Z jitterem?"

Cisza.

"Co się dzieje, gdy obliczenia nie zmieszczą się w czasie?"

No cóż... algorytm zakładał nieskończoną moc obliczeniową.

Projekt skończył się niepowodzeniem - nie dlatego, że MPC było złe, ale dlatego, że zespół pominął "nudne" rzeczy: saturacje, opóźnienia, ograniczenia czasowe.

Rada: jeśli nie umiecie zrobić działającego PI/PID, nie róbcie MPC. Bo MPC to PI/PID na sterydach - jeśli fundament jest kruchy, całość się zawali.

## Cel
Przetestować sterowanie zanim pojawi się hardware:
- regulatory PI/PID i anti-windup,
- filtrację (notch, low-pass),
- scenariusze saturacji i zakłóceń,
- wpływ opóźnień i jitteru.

> TL;DR: Najpierw stabilne PI/PID z ograniczeniami i anti-windup. Dopiero potem "mądre" metody (LQR/MPC).

## Część II: Symulacja pętli sterowania

## Symulacja pętli sterowania: co musi być w modelu
- dyskretny czas próbkowania,
- opóźnienie w torze pomiaru i aktuacji,
- saturacje i ograniczenia ramp/jerk,
- szumy pomiarowe,
- zakłócenia `T_load`.

### Dlaczego te elementy są niezbędne

**Dyskretny czas próbkowania** to fundamentalna cecha systemów cyfrowych. Wasz regulator działa "raz na krok" - nie ciągle. To ma konsekwencje:
- Wzmacniacz różniczkowy w PID staje się "wzmacniaczem różnicowym" - reaguje na różnicę między próbkami, nie na chwilową pochodną
- Pasmo regulatora jest ograniczone przez częstotliwość próbkowania (reguła: Fs > 20 * pasmo zamknięte)
- Pojawia się "aliasing" - szybkie zjawiska są "składane" w wolniejsze

**Opóźnienie w torze** jest WSZĘDZIE:
- Czas konwersji ADC
- Czas obliczeń regulatora
- Czas przesłania danych
- Czas konwersji DAC / generacji PWM

Każde z tych opóźnień jest małe, ale sumują się - i mogą spowodować niestabilność.

**Saturacje i rampy/jerk** to "mechanika" sterowania:
- Silnik ma limit momentu
- Prąd ma limit
- Przyspieszenie ma limit (bezpieczeństwo)
- Zmiana momentu ma limit (jerk - "szarpnięcie")

Bez symulacji tych ograniczeń wasz regulator będzie "idealny" tylko w symulacji.

### Przemówienie Profesora

Kiedy widzę studentów, którzy projektują regulator w MATLAB/Simulink i mówią "działa", zawsze pytam: "A jakie jest opóźnienie w twoim modelu? A saturacja? A szum?"

Często słyszę: "To jest drobny szczegół..."

Nie, to nie jest drobny szczegół. To jest różnica między "działa w symulacji" a "działa na prawdziwym sprzęcie".

W tym laboratorium będziemy systematycznie dodawać te "drobne szczegóły" - i będziecie widzieć, jak zachowanie regulatora się zmienia. To jest najważniejsza lekcja tego kursu: szczegóły mają znaczenie.

## Anti-windup: obowiązkowe przy saturacjach
Jeśli w sterowaniu jest człon całkujący i saturacja, to bez anti-windup dostaniesz:
- przeregulowania,
- długie "dochody",
- w skrajnym przypadku oscylacje.

### Dlaczego anti-windup jest kluczowy

Wyobraźcie sobie sytuację:
1. Zadacie dużą zmianę prędkości
2. Regulator widzi duży błąd i zwiększa wyjście
3. Wyjście osiąga limit (saturacja)
4. Błąd nadal jest duży (bo wyjście jest clamped)
5. Człon całkujący nadal integruje błąd
6. ...i nadal integruje... i nadal...

Kiedy wreszcie błąd zaczyna spadać, człon całkujący jest "napompowany" do ogromnej wartości. Ta wartość musi być "wyleczona" przez człon proporcjonalny - co powoduje wielkie przeregulowanie.

Rozwiązanie: **anti-windup**. Zasada prosta: gdy wyjście jest nasycone, przestań integrować. Albo w innej wersji: "cofnij" integrację o różnicę między wyjściem regulatora a wyjściem nasyconym.

### Implementacje anti-windup

1. **Back-calculation** (najpopularniejsza):
   - Oblicz różnicę między wyjściem regulatora a wyjściem nasyconym
   - Pomnóż przez współczynnik i odejmij od integrala
   - Prosta, skuteczna

2. **Conditional integration**:
   - Integruj tylko gdy wyjście NIE jest nasycone
   - Albo: integruj tylko gdy błąd ma "dobry" znak
   - Bardziej konservatywna

3. **Velocity form** (forma prędkościowa):
   - Regulator oblicza zmianę wyjścia, nie wartość bezwzględną
   - Saturacja na zmianie, nie na wartości
   - Bardziej naturalna dla niektórych aplikacji

### Przemówienie Profesora

Anti-windup to jedna z tych rzeczy, które "każdy wie, że trzeba zrobić", ale wielu ludzi zapomina albo robi źle.

Pamiętam debugowanie systemu, gdzie wirnik po każdym "gwałtownym" rozruchu miał ogromne przeregulowanie - wirnik "przeskakiwał" zadaną prędkość o 20-30%.

Problem? Regulator PI bez anti-windup. Gdy tylko pojawiało się nasycenie, integral rósł w nieskończoność.

Rozwiązanie: dodanie back-calculation. Przeregulowanie spadło do 2-3%.

Rada: anti-windup to nie "opcjonalny dodatek" - to obowiązkowy element każdego regulatora z członem całkującym. Zawsze.

## Notch: stabilność vs wycinanie rezonansu
Notch to narzędzie diagnostyczne i stabilizacyjne, ale:
- dodaje fazę/opóźnienie,
- jest wrażliwy na przesunięcie częstotliwości rezonansu.

## LQR/MPC: kiedy warto
LQR:
- gdy masz sensowny model state-space i chcesz wielowymiarowy kompromis.

MPC:
- gdy ograniczenia są kluczowe i chcesz predykcji na horyzoncie,
- kosztem obliczeń i wrażliwości na model.

## Checklisty
- W symulacji są saturacje i anti-windup.
- W symulacji jest opóźnienie i jitter (scenariusze).
- Każda zmiana filtru/regulatora ma test regresji (te same przypadki).

### Checklist szczegółowy

**Podstawy dyskretnego sterowania:**
- [ ] Regulator działa w pętli dyskretnej (nie ciągłej)
- [ ] Częstotliwość próbkowania dobrana (Fs > 20 * pasmo zamknięte)
- [ ] Dyskretyzacja integratora poprawna (Tustin lub forward Euler)

**Ograniczenia:**
- [ ] Saturacja momentu/prądu zaimplementowana
- [ ] Anti-windup zaimplementowany i przetestowany
- [ ] Ograniczenie jerk/rampy (jeśli wymagane)

**Scenariusze testowe:**
- [ ] Skok zadania - odpowiedź bez oscylacji
- [ ] Skok obciążenia - szybkość reakcji
- [ ] Saturacja - zachowanie przy limicie
- [ ] Opóźnienie - jak system znosi zwiększone opóźnienie
- [ ] Jitter - jak system znosi losowe wahania timing

**Filtry:**
- [ ] Low-pass na pomiarze (redukcja szumu)
- [ ] Notch na rezonansach (jeśli potrzebne)
- [ ] Weryfikacja: filtry nie degradują stabilności

### Przemówienie Profesora

Te checklisty to wasza lista "must-have" przed uruchomieniem na prawdziwym sprzęcie. Jeśli cokolwiek z tej listy jest niezrobione - możecie mieć problemy.

Pamiętajcie: regulator, który działa w symulacji bez tych elementów, może być niebezpieczny na prawdziwym sprzęcie. Obiecuje wam, że oscylacje na 30 000 RPM to nie jest doświadczenie, które chcecie przeżyć.

## Slajdy (tekstowe)
### Slajd 1: Co musi mieć symulacja sterowania
- Dyskretyzacja, opóźnienia, saturacje
- Zakłócenia i szum

### Slajd 2: Anti-windup
- Bez tego PI/PID „puchnie” na saturacji

### Slajd 3: LQR/MPC
- Tak, ale dopiero po „higienie” PI/PID

## Pytania do studentów
1. Jakie są minimalne testy regresji dla regulatora prędkości (skok zadania, skok obciążenia, saturacja)?
2. Jak zaprojektujesz anti-windup i jak zweryfikujesz jego działanie w scenariuszu saturacji?
3. Kiedy notch jest właściwą reakcją, a kiedy lepsze jest ograniczenie jerk lub pasma?
4. Jakie ryzyko wnosi MPC w systemie RT i jak je ograniczysz (limit czasu, fallback)?

## Projekty studenckie
- „Discrete PI kit”: regulator PI w dyskrecie + anti-windup + testy jednostkowe.
- „Filter lab”: porównanie wpływu notcha i ramp/jerk na widmo i błąd regulacji.
- „MPC sandbox”: prosty MPC na krótkim horyzoncie z limitem czasu i mechanizmem fallback.

## BONUS
- Ustal „kontrakt czasowy” dla algorytmu: jeśli obliczenia nie zmieszczą się w budżecie, algorytm ma oddać wynik awaryjny, a nie „spóźniony”.
