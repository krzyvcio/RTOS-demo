# Wykład 8: EMC, sygnały, drgania, rezonanse

## Cel: utrzymać sterowanie i sensory w "czystym" środowisku sygnałowym
W humanoidzie masz jednocześnie:
- duże prądy i szybkie przełączanie (falowniki, PWM),
- wrażliwe sensory analogowe i cyfrowe (IMU, czujniki siły, kamery),
- długie wiązki przewodów i wiele punktów masy,
- konstrukcję mechaniczną z rezonansami.

Efekt końcowy jest wspólny:
- zakłócenia i rezonanse pojawiają się jako "szum" i opóźnienie,
- a to degraduje estymację i stabilność regulatorów.

## Widma i odpowiedzi częstotliwościowe: wspólny język dla EMC i drgań
Najbardziej użyteczne narzędzie diagnostyczne to przejście do domeny częstotliwości:
- widmo sygnału (FFT),
- odpowiedź częstotliwościowa obiektu (Bode, sweep).

Gdy widzisz pik w widmie:
- stały pik w konkretnej częstotliwości -> podejrzenie rezonansu lub zakłócenia periodycznego,
- szerokopasmowy wzrost szumu -> podejrzenie EMC lub aliasingu.

## Modele RLC jako graf obwodów i graf impedancji
Wiązki, filtry, zasilanie i wejścia analogowe można uprościć do modeli RLC.
Graf obwodu:
- węzły: punkty połączeń (masa, zasilanie, sygnał),
- krawędzie: impedancje (R, L, C).

Graf impedancji jest przydatny, bo w EMI/EMC często pytasz:
- gdzie płynie prąd zakłócający,
- jakie są ścieżki powrotu masy,
- gdzie tworzą się pętle i sprzężenia.

## Laplace i Z-transform: po co w praktyce
Laplace:
- opis transmitancji ciągłej i filtrów analogowych,
- analiza stabilności i odpowiedzi układów liniowych.

Z-transform:
- analogiczne narzędzie dla układów dyskretnych,
- projekt filtrów cyfrowych i ich stabilności.

W pętli sterowania interesuje Cię:
- jak filtr zmienia fazę i opóźnienie,
- czy nie psuje marginesu stabilności.

## FFT/DFT i pułapki pomiarowe
FFT daje widmo, ale jest wrażliwe na:
- długość okna i rozdzielczość częstotliwości,
- okno (Hann/Hamming) i leakage,
- aliasing (brak filtracji przed próbkowaniem),
- nieliniowości (harmoniczne).

Praktyka:
- zawsze patrz na częstotliwość próbkowania,
- porównuj widma w różnych warunkach (obciążenie, prędkość, tryb sterowania),
- łącz analizę widma z korelacją z sygnałem sterowania.

## Filtry: FIR i IIR oraz praktyczne biquady
FIR:
- stabilny z definicji,
- może wymagać dużego rzędu (większe opóźnienie).

IIR:
- efektywny (mniejszy rząd),
- wymaga ostrożności ze stabilnością i kwantyzacją.

W praktyce często:
- biquad (IIR 2 rzędu) jako cegiełka filtrów,
- notch do wycinania rezonansu,
- low-pass do ograniczenia szumu.

## Rezonanse mechaniczne i analiza modalna
Rezonanse konstrukcji i przekładni są naturalne.
Analiza modalna:
- rozkłada układ na mody drgań,
- wartości własne dają częstotliwości własne,
- wektory własne dają kształty modów.

W sterowaniu rezonans często widzisz jako:
- wzmocnienie błędu w wąskim paśmie,
- oscylacje przy pewnych prędkościach lub obciążeniach.

## Rozróżnianie: EMC vs. mechanika
Sygnały wskazujące na EMC:
- korelacja z przełączaniem PWM,
- zakłócenia na wielu kanałach jednocześnie,
- zależność od routingu kabli i uziemienia.

Sygnały wskazujące na mechanikę:
- korelacja z obciążeniem i prędkością,
- dominujące piki odpowiadające częstotliwościom własnym,
- wpływ temperatury i smarowania na widmo (tarcie).

## Praktyka inżynierska: co robić w systemie
- Diagnozuj rezonanse przez sweep częstotliwości i testy pobudzenia (chirp).
- Projekt filtracji i ekranowania opieraj na pomiarach.
- Nie "lecz" EMC filtrami w sterowaniu bez równoległej pracy nad sprzętem (masa, ekranowanie, separacja).

## Checklisty
- loguj widma błędu i sygnału sterowania w problematycznych stanach,
- miej czujniki temperatury i prądu jako kontekst diagnostyczny,
- waliduj filtry pod kątem opóźnienia i marginesu fazy,
- testuj w realistycznym okablowaniu i konfiguracji zasilania.

## Pytania do studentów
1. Jak rozróżnisz zakłócenie EMC od rezonansu mechanicznego na podstawie widma i korelacji z obciążeniem?
2. Dlaczego filtr „poprawiający szum” może pogorszyć stabilność w pętli sterowania?
3. Jakie są typowe źródła aliasingu i jak je wykryjesz w danych?
4. Jak zaplanujesz eksperyment sweep/chirp, aby zidentyfikować częstotliwości problematyczne?

## Projekty studenckie
- Pipeline do FFT + tracking pików + korelacje z trybem i obciążeniem.
- Porównanie filtrów FIR/IIR (opóźnienie vs jakość) w torze pomiarowym regulatora.
- „EMC vs mechanika”: zestaw scenariuszy testowych i heurystyk rozróżniających przyczynę.

## BONUS
- Jeśli filtr działa tylko w jednym „magicznym” ustawieniu, to często maskuje problem; dąż do rozwiązań, które poprawiają metryki w szerokim zakresie warunków.
