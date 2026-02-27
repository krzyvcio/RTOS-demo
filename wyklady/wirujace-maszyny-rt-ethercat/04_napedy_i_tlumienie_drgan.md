# Wykład 4: Napędy, stabilizacja prędkości i tłumienie drgań

## Cel
Zebrać praktyczne techniki sterowania napędem w maszynach wirujących oraz strategie radzenia sobie z rezonansami, tak aby:
- uzyskać stabilną regulację prędkości,
- nie pobudzać konstrukcji (drgań) agresywnym sterowaniem,
- mieć procedurę diagnozy: „co jest mechaniką, co jest czasem, co jest regulatorem”.

> TL;DR: Najpierw porządkujesz kaskadę pętli i ograniczenia (rampy/jerk), potem mierzysz widma, dopiero wtedy dobierasz notch. Filtry „na oko” często psują stabilność.

## Model minimalny prędkości (intuicja)
```text
J * domega/dt = T_motor - T_load - T_losses
```
Interpretacja:
- `J`: bezwładność wirnika (fizyka, nie software),
- `T_motor`: sterujesz (przez prąd/moment),
- `T_load`: zakłócenie od procesu,
- `T_losses`: tarcie i straty (zwykle zależne od prędkości/temperatury).

W realnym świecie dochodzą:
- rezonanse mechaniczne (konstrukcja, łożyskowanie),
- opóźnienia i kwantyzacja pomiaru prędkości,
- saturacje prądu/momentu i ograniczenia napięciowe napędu.

## Model drgań: minimum, które wystarcza do decyzji
Do praktycznej pracy często wystarcza model masa-tłumik-sprężyna:
```text
M * xdd + D * xd + K * x = F(t)
```
To mówi Ci:
- częstotliwości własne wynikają z `M` i `K`,
- tłumienie `D` decyduje o „ostrości” piku,
- pobudzenie `F(t)` może pochodzić z napędu (sterowanie), niewyważenia, przekładni, otoczenia.

Wniosek praktyczny:
- jeśli regulator ma pasmo w okolicy częstotliwości własnej, to bardzo łatwo o wzbudzenie.

## Kaskada pętli (napęd -> prędkość -> profil)
Typowy schemat w przemysłowych napędach:
- pętla prądu/momentu: w falowniku (kHz),
- pętla prędkości: w napędzie albo w masterze (zależnie od wymagań i architektury),
- profil setpointu (rampy, jerk, limity): w warstwie nadrzędnej.

Najważniejsza zasada:
- pętla wewnętrzna musi być istotnie szybsza i stabilna, inaczej wyższe warstwy sterują „glutem”.

## Setpoint shaping: rampy i ograniczanie jerk (tania stabilność)
Wiele problemów drgań bierze się z agresywnych zmian zadania.
Stosuje się:
- limit przyspieszenia (rampa prędkości),
- limit jerk (rampa przyspieszenia),
- łagodne przejścia stanów (start/stop/tryby).

To często daje większy efekt niż dokładanie złożonych filtrów.

## Widma (FFT): jak robić to praktycznie
Sygnały do FFT, które zwykle są najbardziej informatywne:
- błąd prędkości,
- sygnał sterowania (moment/prąd zadany),
- wibracje (akcelerometr),
- czasem prądy (jeśli widzisz harmoniczne).

Wskazówki:
- analizuj na tych samych odcinkach czasowych i w porównywalnych warunkach (tryb, prędkość, obciążenie),
- loguj także kontekst (temperatura, tryb, obciążenie), bo rezonanse „pływają” z warunkami.

Interpretacja (prosta, ale działa):
- pik w błędzie i w sterowaniu: regulator pobudza rezonans,
- pik tylko w wibracjach: mechanika/łożyskowanie lub pobudzenie zewnętrzne,
- piki rosną z prędkością: możliwe niewyważenie lub element zależny od prędkości.

## Notch: kiedy pomaga i kiedy szkodzi
Notch dobrze wycina wąski rezonans, ale:
- nie jest „za darmo” (zmienia fazę i opóźnienie),
- źle dobrany notch potrafi pogorszyć margines stabilności.

Procedura bezpieczna:
1) zidentyfikuj dominujący pik (FFT),
2) sprawdź, czy pik jest stabilny w warunkach pracy,
3) dodaj pojedynczy notch,
4) zweryfikuj: poprawa widma + brak pogorszenia stabilności,
5) dopiero potem rozważ kolejne filtry.

## Co najczęściej psuje „dobry regulator”
- uśrednianie prędkości i filtry w torze pomiaru (opóźnienie),
- saturacje momentu/prądu bez anti-windup w regulatorze prędkości,
- zbyt duże pasmo „bo ma być sztywno” (pobudzenie konstrukcji),
- brak ograniczeń jerk (skoki pobudzają mody),
- próby „leczenia mechaniki softwarem” bez usunięcia źródła (luzy, niewyważenie, mocowania).

## Checklisty
- Pętla prądu/momentu jest szybsza niż pętla prędkości (kaskada ma sens).
- Setpoint ma rampy i ograniczenie jerk (szczególnie w przejściach).
- Widma (FFT) są częścią diagnostyki: błąd + sterowanie + wibracje.
- Notch jest dobierany z pomiarów i walidowany pod kątem stabilności.
- Logujesz także kontekst (tryb, obciążenie, temperatura), nie tylko sygnały.

## Zadania (praktyka)
1. Wybierz 3 sygnały do FFT i uzasadnij, co z nich wywnioskujesz.
2. Opisz procedurę: wykrycie piku -> weryfikacja -> notch -> walidacja.
3. Zdefiniuj, gdzie implementujesz rampy/jerk (drive vs master) i dlaczego.

## Pytania do studentów
1. Jakie ryzyko niesie dołożenie notcha „na oko” bez walidacji wpływu na fazę i stabilność?
2. W jakim miejscu łańcucha (setpoint, regulator, pomiar) najczęściej powstaje pobudzenie rezonansu i jak to wykryjesz w logach?
3. Kiedy ograniczenie jerk daje większy efekt niż filtracja widmowa?
4. Jak zaprojektujesz procedurę testową przejścia przez zakresy prędkości, w których spodziewasz się rezonansów?

## Projekty studenckie
- „FFT pipeline”: skrypt, który z logu liczy FFT dla kilku sygnałów, znajduje piki i generuje raport.
- „Notch tuner”: prototyp narzędzia do doboru parametru notcha na podstawie dominującego piku i walidacji przed/po.
- „Setpoint shaper”: biblioteka ramp/jerk z testami regresji (brak skoków, brak przekroczeń).

## BONUS
- Najlepszy „filtr” to często mechanika: jeśli widzisz narastające piki w stałym paśmie, równolegle do pracy software zaplanuj inspekcję mocowań, niewyważenia i łożysk.
