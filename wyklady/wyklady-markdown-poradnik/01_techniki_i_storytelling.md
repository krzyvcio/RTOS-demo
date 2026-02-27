# 1) Jak tworzyć wykłady w Markdown, żeby były angażujące

## Zasada nadrzędna
Markdown ma robić dwie rzeczy:
- prowadzić wzrok (hierarchia nagłówków),
- budować tempo (krótkie sekcje przeplatane przykładami).

## Techniki, które działają „od ręki”
### 1. „Hak” w pierwszych 10 linijkach
Zacznij od:
- problemu z życia (symptom),
- konsekwencji (co się stanie, jeśli tego nie zrozumiesz),
- obietnicy (co umiesz po wykładzie).

Przykład:
```md
## Dlaczego to ma znaczenie
Jeśli jitter w pętli sterowania rośnie, układ zaczyna oscylować.
Po tym wykładzie będziesz umiał zmierzyć jitter i policzyć budżet czasu end-to-end.
```

### 2. Narracja „objaw -> model -> narzędzie -> wniosek”
Powtarzalny rytm ułatwia czytanie:
- objaw (co widzę),
- model (jak to opisać),
- narzędzie (jak to policzyć/zmierzyć),
- wniosek (co zmieniam w systemie).

### 3. Bloki „TL;DR” i „Checklisty”
Czytelnik może:
- przeskanować wykład,
- wrócić do szczegółów później.

Przykład:
```md
> TL;DR: Zawsze mierz WCRT i jitter, bo średnia nie wystarcza.
```

### 4. Minimalne równania, maksymalna intuicja
Jedno równanie + interpretacja.

Przykład:
```md
J * domega/dt = T_motor - T_load
```
I od razu:
- co jest mierzalne,
- co jest sterowalne,
- co jest zakłóceniem.

### 5. „Bezpieczne skróty”
Nie wszystko trzeba opisać do końca.
Wystarczy:
- wskazać granice modelu,
- podać, co w praktyce się psuje.

## Checklisty
- Każdy rozdział kończ „co robię w systemie”.
- Wstawiaj przykład co 20–40 linijek.
- Nie dawaj długich bloków tekstu bez list/sekcji.

