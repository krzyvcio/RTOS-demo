# Wykład 8: Analiza sygnałów (FFT, widma, piki, trendy)

## Część I: Wstęp teoretyczny - Dlaczego analiza częstotliwości jest kluczowa

### Geneza: co widać w dziedzinie czasu, a co w częstotliwości

Analiza sygnałów w dziedzinie czasu (przebiegi czasowe) mówi nam "kiedy" - kiedy pojawił się skok, jak długo trwało zdarzenie, jaka była sekwencja.

Analiza w dziedzinie częstotliwości (FFT, widma) mówi nam "co" - jakie częstotliwości składają się na sygnał, gdzie są rezonanse, jakie są dominujące składowe.

Dla systemów wirujących, analiza częstotliwości jest szczególnie ważna, bo:
- **Rezonanse** są widoczne jako piki w widmie - częstotliwości, gdzie energia drgań jest skoncentrowana
- **Zakłócenia periodyczne** (np. od przekładni) są widoczne jako charakterystyczne częstotliwości
- **Szum** ma często charakterystyczne widmo (biały, różowy)
- **Nieliniowości** manifestują się jako harmoniczne częstotliwości podstawowej

### Przemówienie Profesora

Kiedy patrzę na przebieg czasowy sygnału prędkości, widzę "ogień" - wszystko miga, szumi, trzeszczy. Nie wiem, gdzie szukać problemu.

Kiedy patrzę na widmo FFT - widzę **mapę**. Tu jest rezonans 120 Hz, tu harmoniczna 60 Hz, tu szum.

To jest różnica między "patrzeniem" a "widzeniem".

FFT to nie jest "tylko dla teoretyków". To jest narzędzie diagnostyczne - pozwala szybko znaleźć problem.

## Cel
Umieć wykrywać problemy "z danych":
- rezonanse,
- zakłócenia periodyczne,
- narastanie wibracji,
- różnicę między mechaniką a problemem sterowania/RT.

> TL;DR: FFT + kontekst (tryb/prędkość/obciążenie) daje szybkie hipotezy, które potem weryfikujesz testem.

## Część II: Analiza w praktyce

## Co analizować
- błąd prędkości,
- sygnał sterowania (moment/prąd),
- wibracje (RMS + widmo),
- czas iteracji pętli (czy oscylacje korelują z jitterem).

### Co analizować - praktyczne wskazówki

**Błąd prędkości** (error = zadanie - pomiar):
- Czy błąd jest stały? → problem offset/kalibracja
- Czy błąd oscyluje? → problem sterowania
- Czy błąd rośnie z prędkością? → problem wzmocnienia

**Sygnał sterowania** (moment, prąd):
- Czy są oscylacje? → problem pętli
- Czy są "szarpnięcia"? → problem dyskretyzacji/kwantyzacji
- Czy jest nasycenie? → sprawdź limity

**Wibracje** (z akcelerometru):
- Piki w widmie → rezonanse
- Zależność od prędkości → problem mechaniczny
- Zależność od obciążenia → problem podatności

**Czas pętli**:
- Czy oscylacje błędu korelują z jitterem? → problem RT

## Widmo a wniosek
W praktyce pytasz:
- czy pik jest stały (częstotliwość własna),
- czy skaluje się z prędkością (zależność od RPM),
- czy pojawia się po zmianie software (podejrzenie filtru/RT).

### Interpretacja widma

**Stały pik** (niezależny od RPM):
- Częstotliwość własna konstrukcji
- Rezonans mechaniczny
- Wymaga: notch, zmiana sterowania, lub unikanie prędkości

**Pik skalujący się z RPM** (np. 1x RPM, 2x RPM):
- Niewyważenie wirnika
- Problem z zamontowaniem
- Wymaga: wyważenie, zmiana mocowania

**Pik pojawiający się po zmianie software**:
- Problem z filterm (np. nieostateczny notch)
- Problem z RT (jitter wprowadza szum)
- Wymaga: weryfikacja zmian

### Przemówienie Profesora

Analiza widma bez kontekstu to jak czytanie mapy bez kompasu. Nie wiesz, gdzie jesteś, nie wiesz, dokąd zmierzasz.

ZAWSZE patrzcie na widmo razem z:
- Trybem pracy (rozruch, praca, zatrzymanie)
- Prędkością wirnika
- Obciążeniem
- Temperaturą

Tylko wtedy widmo staje się narzędziem diagnostycznym, nie dekoracją.

## Checklisty
- Zawsze logujesz też kontekst: tryb, prędkość, temperatura.
- Masz zdefiniowane "piki do monitorowania" i progi trendów.

### Checklist szczegółowy

**Logowanie:**
- [ ] Każdy pomiar ma timestamp
- [ ] Każdy pomiar ma kontekst (tryb, prędkość)
- [ ] Format logu jest ustrukturyzowany

**Analiza:**
- [ ] Lista częstotliwości do monitorowania
- [ ] Progi alarmowe dla pików
- [ ] Progi trendowe (RMS, piki)

**Weryfikacja:**
- [ ] Test potwierdzający źródło piku (zmiana prędkości/obciążenia)
- [ ] Dla każdego piku: plan reakcji

## Slajdy (tekstowe)
### Slajd 1: Co daje FFT
- piki, które wskazują rezonanse i zakłócenia

### Slajd 2: Co musi być w logu
- sygnał + kontekst + timestamp

## Pytania do studentów
1. Jak wybierzesz okno FFT i jak unikniesz wniosków z artefaktów (leakage, aliasing)?
2. Jak rozpoznasz, że pik wynika z rezonansu mechaniki, a nie z jitteru w czasie?
3. Jakie metryki trendu (RMS, piki) będą najbardziej użyteczne dla predykcji degradacji?
4. Jak zaprojektujesz „raport widmowy” tak, aby był porównywalny między wersjami oprogramowania?

## Projekty studenckie
- „Peak tracker”: automatyczne śledzenie 3–5 największych pików FFT w czasie + alarm trendowy.
- „Correlation view”: korelacja pików widma z trybem, temperaturą i obciążeniem.
- „Before/after report”: raport porównujący widma i metryki po zmianie filtru/regulatora.

## BONUS
- Najszybsza droga do dobrych wniosków: zawsze analizuj widmo razem z sygnałem sterowania i metrykami RT. Samo widmo bez kontekstu często myli.
