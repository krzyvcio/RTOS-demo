# Wykład 8: Analiza sygnałów (FFT, widma, piki, trendy)

## Cel
Umieć wykrywać problemy „z danych”:
- rezonanse,
- zakłócenia periodyczne,
- narastanie wibracji,
- różnicę między mechaniką a problemem sterowania/RT.

> TL;DR: FFT + kontekst (tryb/prędkość/obciążenie) daje szybkie hipotezy, które potem weryfikujesz testem.

## Co analizować
- błąd prędkości,
- sygnał sterowania (moment/prąd),
- wibracje (RMS + widmo),
- czas iteracji pętli (czy oscylacje korelują z jitterem).

## Widmo a wniosek
W praktyce pytasz:
- czy pik jest stały (częstotliwość własna),
- czy skaluje się z prędkością (zależność od RPM),
- czy pojawia się po zmianie software (podejrzenie filtru/RT).

## Checklisty
- Zawsze logujesz też kontekst: tryb, prędkość, temperatura.
- Masz zdefiniowane „piki do monitorowania” i progi trendów.

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
