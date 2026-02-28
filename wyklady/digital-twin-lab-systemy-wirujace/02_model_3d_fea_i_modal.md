# Wykład 2: Model 3D, FEA i analiza modalna (rezonanse)

## Część I: Wstęp teoretyczny - Dlaczego musimy rozumieć rezonanse

### Geneza problemu rezonansowego

Kiedy wirnik kręci się ze stałą prędkością, energia jest przekazywana z silnika do łożyskowania i konstrukcji. Ale gdy prędkość wirnika zbliża się do jednej z częstotliwości własnych konstrukcji - następuje zjawisko rezonansu. Amplituda drgań gwałtownie rośnie, sometimes nawet dziesięciokrotnie lub stukrotnie. Dla wirującej maszyny to może być katastrofalne: uszkodzenie łożyskowanie, pęknięcie wału, a w skrajnych przypadkach - zniszczenie całego urządzenia.

Historia inżynierii zna wiele katastrof spowodowanych rezonansem. Najsłynniejszy przypadek to most Tacoma Narrows w 1940 roku - choć to był most, nie wirująca maszyna, zasada jest ta sama. Częstotliwość wiatru pasowała do częstotliwości własnej mostu i amplitud drgań wzrosły do tego stopnia, że most się zawalił.

W systemach wirujących rezonanse są jeszcze bardziej niebezpieczne, bo:
- Częstotliwości własne mogą być wysokie (setki Hz, kHz)
- Wirnik przechodzi przez te częstotliwości podczas rozruchu/zatrzymania
- Masa wirnika i siły odśrodkowe modyfikują sztywność

### Dlaczego FEA i analiza modalna

Metoda Elementów Skończonych (FEA) to narzędzie, które pozwala numerycznie rozwiązać problem drgań dla skomplikowanych konstrukcji. Zamiast rozwiązywać równania analitycznie (co jest możliwe tylko dla prostych kształtów), dzielimy konstrukcję na miliony małych elementów i rozwiązujemy numerycznie.

Analiza modalna to szczególny rodzaj FEA, który wyznacza:
- Częstotliwości własne (gdzie są rezonanse)
- Postacie modalne (jak konstrukcja drga w każdym rezonansie)
- Współczynniki tłumienia (jak szybko drgania zanikają)

To jest informacja, którą **przenosimy do świata sterowania**. Sterowanie nie musi znać pełnego modelu MES - musi wiedzieć, gdzie są rezonanse i jak je "obejść" (lub wykorzystać).

### Przemówienie Profesora

Moi studenci czasem pytają: "Po co nam FEA, skoro mamy pomiary? Po prostu zmierzymy charakterystykę na stole probierczym".

Odpowiedź jest prosta: bo pomiary na stole to za mało i za późno.

Na stole probierczym mierzycie to, co macie - z wszystkimi imperfekcjami, luzami, błędami montażu. To jest cenne, ale nie mówi wam, dlaczego te rezonanse istnieją i jak zmienią się, gdy zmienicie konstrukcję.

FEA daje wam **zrozumienie**. Wiecie, że dany rezonans pochodzi od podatności wału, a inny - od rezonansu stojana. Wiecie, jak zmieni się częstotliwość, gdy zwiększycie średnicę łożyska albo zmienicie materiał.

I jeszcze jedno: FEA chroni was przed **drogimi błędami**. Bo zmienić model w FEA jest tanie - zmienić prawdziwą konstrukcję jest drogie. Używajcie FEA do eksperymentowania "w głowie", a pomiary do weryfikacji.
Zrozumieć, po co i kiedy wprowadzać narzędzia FEA/modal, oraz jak przenieść wyniki do modelu użytecznego w sterowaniu i diagnostyce.

> TL;DR: FEA daje częstotliwości własne i mody. Sterowanie potrzebuje z tego: listy rezonansów, ich czułości na warunki i sygnałów, na których to widać.

## Część II: Analiza modalna w praktyce

## Co daje analiza modalna
Wyniki, które są praktycznie użyteczne:
- częstotliwości własne (gdzie spodziewać się pików),
- postacie drgań (co "pracuje" i gdzie mierzyć),
- wpływ zmian sztywności (np. mocowanie, łożyska) na przesunięcie rezonansu.

### Częstotliwości własne - mapa zagrożeń

Częstotliwości własne to "prędkości krytyczne" - punkty, przez które wirnik MUSI przejść podczas rozruchu i zatrzymania. Kluczowe pytanie: **czy system sterowania jest w stanie przejść przez te prędkości bez destabilizacji?**

W idealnym przypadku przechodzimy przez rezonans "szybko" - z przyspieszeniem wystarczającym, by nie wzbudzić drgań. W rzeczywistości:
- Przyspieszenie jest ograniczone przez moment silnika
- Każdy rezonans ma "pasmo" - zakres częstotliwości, gdzie drgania rosną
- Tłumienie jest zwykle małe (małe straty = duże rezonanse)

Postacie modalne pokazują, **gdzie mierzyć** drgania, żeby je widzieć, i **gdzie nie montować czujników**, bo pomiar byłby niereprezentatywny.

### Przemówienie Profesora

Kiedy projektowałem sterowanie dla dużej wirówki przemysłowej, pierwszą rzeczą, którą zrobiłem, było wykreślenie na wykresie częstotliwości własnych. "Tutaj mamy rezonans 120 Hz, tutaj 340 Hz, tutaj 890 Hz" - powiedział mi konstruktor.

Moja odpowiedź: "OK. A przez którą z tych częstotliwości będziemy przechodzić podczas rozruchu?"

Okazało się, że przez wszystkie. I że przewidywany czas przejścia przez rezonans 340 Hz jest taki sam jak czas narastania drgań - czyli mogliśmy mieć problem.

To jest rola inżyniera sterowania: rozumieć mechanikę i wiedzieć, jak sterowanie zachowa się w obecności rezonansów. Nie ignorować rezonanse, nie liczyć na to, że "jakoś będzie" - ale zaprojektować świadome przejście.

## Jak używać wyników w labie
Minimalny transfer do świata sterowania:
- lista potencjalnych pików do monitorowania w FFT,
- kandydaci do notch (z ostrożnością),
- scenariusze testowe: przejścia przez zakresy prędkości, gdzie mody są pobudzane.

### Od FEA do sterowania - praktyczny przepływ

FEA daje wam "mapę" częstotliwości własnych. Ale mapa to nie terytorium - musicie tę mapę przenieść do świata sterowania. Jak to zrobić?

**Krok 1: Lista pików do FFT**
Bierzcie częstotliwości z FEA i tworzycie "listę obserwacyjną" - częstotliwości, na które patrzycie w FFT. Nie wszystkie rezonanse będą widoczne w sygnale prędkości - ale musicie wiedzieć, gdzie szukać.

**Krok 2: Kandydaci do notch**
Notch filter to narzędzie do "wycięcia" rezonansu z odpowiedzi systemu. Ale notch ma cenę: dodaje opóźnienie fazowe, co może destabilizować sterowanie. Dlatego:
- Najpierw weryfikujcie, czy rezonans jest rzeczywiście problemem
- Potem próbujcie "obejść" sterowaniem (np. szybsze przejście)
- Dopiero na końcu - notch, i to z zapasem (częstotliwość notcha ≠ częstotliwość rezonansu)

**Krok 3: Scenariusze testowe**
Przejście przez rezonans to "test stresowy" dla sterowania. Scenariusze:
- Rozruch z różnymi przyspieszeniami
- Zmiana kierunku (przez zero)
- Nagłe odciążenie

### Przemówienie Profesora

Notch to "nóż chirurgiczny" - używany ostrożnie może uratować system, ale używany na oślep może zabić.

Widziałem systemy, gdzie ktoś "dodał notch" na każdy pik w FFT, widząc że piki znikają. Problem: każdy notch dodaje opóźnienie, a opóźnienia się sumują. Pod koniec system miał tyle notchs, że sterowanie było niestabilne - nie przez rezonanse, ale przez opóźnienia.

Rada: notch to ostateczność. Najpierw spróbujcie:
1. Zmienić ścieżkę przejścia (przyspieszenie)
2. Zmienić punkt pracy (unikać rezonansu)
3. Zwiększyć tłumienie (jeśli można mechanicznie)

Dopiero gdy to nie działa - notch.

## Pułapki
- FEA jest "idealne", a realna konstrukcja ma luzy, tolerancje i zmienność.
- Rezonanse potrafią pływać z temperaturą i obciążeniem.
- Model zbyt szczegółowy nie nadaje się do szybkiej symulacji sterowania.

### Dlaczego model i rzeczywistość się różnią

FEA zakłada idealne warunki:
- Idealne połączenia między elementami
- Liniowe materiały
- Precyzyjnie zdefiniowane warunki brzegowe

Rzeczywistość:
- Luzy w połączeniach śrubowych
- Nieliniowości materiałowe (duże odkształcenia = zmiana sztywności)
- Tarcie w połączeniach = tłumienie niepasujące do modelu
- Tolerancje produkcyjne = różnice między egzemplarzami

To jest **fundamentalne ograniczenie** - musicie je zaakceptować i zaprojektować system tak, by był odporny na te różnice.

### Rezonanse pływają

Częstotliwości własne nie są stałe:
- Temperatura: sztywność zmienia się z temperaturą (szczególnie w metalach)
- Naprężenia: siła odśrodkowa "dociska" konstrukcję, zmieniając sztywność
- Obciążenie: masa obciążenia zmienia częstotliwości

W praktyce: model daje wam "nominalną" częstotliwość, ale rzeczywista może być ±10% różnicy. Wasze filtry i strategie sterowania muszą to uwzględniać.

### Przemówienie Profesora

Najgorsze, co możecie zrobić, to traktować wyniki FEA jako "prawdę objawioną". FEA to narzędzie, które pomaga zrozumieć - ale weryfikacja zawsze musi być empiryczna.

Kiedyś pracowałem z zespołem, który wykonał piękną analizę FEA wirnika. Wyniki: rezonans 450 Hz. Pomiar na stole: rezonans 410 Hz. Różnica: prawie 10%.

Gdzie była przyczyna? Okazało się, że model nie uwzględniał podatności kołnierza mocującego wirnik do wału. Mały element, wielki wpływ.

Wniosek: zawsze weryfikujcie FEA pomiarami. I zawsze zakładajcie tolerancję - wasze filtry muszą działać w zakresie, nie na pojedynczej częstotliwości.

## Checklisty
- Wyniki FEA są przetłumaczone na: "co monitorujemy" i "jak reagujemy".
- Masz plan walidacji: FFT na sygnałach + korelacja z trybem i obciążeniem.

### Checklist szczegółowy

**Od FEA do laboratorium:**
- [ ] Lista częstotliwości własnych z FEA
- [ ] Dla każdej częstotliwości: postaci modalne (co drga)
- [ ] Oszacowanie tłumienia (dla rezonansów krytycznych)
- [ ] Mapowanie na "co widzimy w prędkości" vs "co widzimy w wibracjach"

**Monitorowanie:**
- [ ] Lista częstotliwości do FFT (dla każdego rezonansu)
- [ ] Progi alarmowe (kiedy pik = problem)
- [ ] Korelacja z trybem pracy (rozruch, praca, zatrzymanie)

**Reakcja:**
- [ ] Strategia przejścia przez rezonans (sterowanie)
- [ ] Kandydaci do notch (z uzasadnieniem)
- [ ] Fallback: co robimy, gdy rezonans = problem

**Walidacja:**
- [ ] Pomiar FFT na rzeczywistym systemie
- [ ] Porównanie z FEA: zgodność częstotliwości?
- [ ] Test przejścia przez rezonans w różnych warunkach

### Przemówienie Profesora

Na zakończenie tego wykładu chcę, żebyście zapamiętali jedną rzecz: FEA to generator hipotez, nie wyrocznia.

FEA mówi wam: "tu może być rezonans", "tu konstrukcja jest podatna". Ale czy to rzeczywiście problem - to musicie zweryfikować.

Największa wartość FEA jest w tym, że mówi "gdzie patrzeć" i "czego się spodziewać" w danych. Bez FEA szukacie igły w stogu siana. Z FEA wiecie, że igła jest w lewym rogu stogu - ale i tak musicie ją znaleźć.

Używajcie FEA mądrze.

## Slajdy (tekstowe)
### Slajd 1: Co daje FEA
- Częstotliwości własne
- Postacie drgań

### Slajd 2: Jak to wykorzystać
- Monitoring pików w FFT
- Kandydaci do notch

## Pytania do studentów
1. Jak przetłumaczysz wynik FEA (mody, częstotliwości) na konkretne decyzje w sterowaniu i diagnostyce?
2. Jakie są 3 typowe różnice między FEA a rzeczywistością i jak je uwzględnisz w labie?
3. Jak sprawdzisz, czy pik w FFT to rezonans konstrukcji, a nie artefakt pomiaru (aliasing/filtr)?
4. Jak zaprojektujesz plan walidacji „FEA -> pomiary -> reakcja”?

## Projekty studenckie
- „Modal-to-monitor”: generator listy częstotliwości do monitorowania + progi trendów.
- „Notch candidates”: raport kandydatów do notcha z oceną ryzyka (zmienność z temperaturą/obciążeniem).
- „Placement study”: propozycja miejsc montażu czujników (na podstawie postaci drgań).

## BONUS
- Traktuj FEA jako generator hipotez, nie jako wyrocznię: największa wartość jest w tym, że mówi „gdzie patrzeć” i „czego się spodziewać” w danych.
