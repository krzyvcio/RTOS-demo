# 5) Przykłady fragmentów wykładu w Markdown (format + narracja)

## Fragment 1: TL;DR + cel
```md
> TL;DR: Jeśli nie mierzysz WCRT, nie wiesz czy system jest deterministyczny.

## Cel
Po tej części umiesz policzyć budżet czasu end-to-end i wskazać wąskie gardło.
```

## Fragment 2: model + interpretacja
```md
J * domega/dt = T_motor - T_load

- `J`: bezwładność (nie zmienisz jej softwarem)
- `T_motor`: sterujesz (przez prąd/moment)
- `T_load`: zakłócenie (proces)
```

## Fragment 3: pułapka wdrożeniowa
```md
### Najczęstszy błąd
Dodanie filtrów „na ślepo” zwiększa opóźnienie i psuje margines fazy.
Najpierw zmierz widmo i jitter, potem dobierz filtr.
```

## Fragment 4: mini-lab
```md
### Zadanie (10 min)
Zbierz 5 s danych błędu, policz FFT, zaznacz piki i zaproponuj notch.
```

