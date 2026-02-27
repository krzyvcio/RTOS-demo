# Wyklad 4: Jezyki i interfejsy (wielojezycznosc bez chaosu)

## Cel
Ulozyc stack jezykow tak, by:
- hard RT bylo bez GC,
- middle layer bylo wydajne i testowalne,
- upper layer bylo szybkie w rozwoju,
- granice byly ostre (API, kontrakty, wersjonowanie).

## Zasada: jezyk wynika z warstwy
- W1 (MCU/drive): C albo Rust `no_std`
- W2 (RT master): C/C++ albo Rust (z rygorem RT)
- W3 (nadzor): Python/Node/Go (dowolnie)
- W4 (model): Python/MATLAB/Julia (offline)

## Kontrakty danych: wersjonuj i stabilizuj
Najwiekszy blad to "wymieniamy structy z pamieci procesu bez wersji".
W praktyce wprowadz:
- `schema_version`
- stale rozmiary ramek telemetrycznych
- kompatybilnosc wsteczna w W3/W4

## Bindingi i biblioteki (praktycznie)
Jesli uzywasz biblioteki w C (np. master):
- watek RT jest w C/C++/Rust
- warstwa W3 dostaje dane przez IPC, nie przez wywolania do watku RT

## Checklisty
- Watek RT nie uzywa jezyka z GC.
- API miedzy warstwami ma wersje i testy kompatybilnosci.

## Kontekst robotyki: middleware i "real-time friendly" integracja
W systemach robotycznych czesto pojawia sie middleware (pub/sub, RPC).
Zasada pozostaje ta sama:
- middleware nie moze wprowadzac niekontrolowanego jitteru w petli RT,
- watki RT publikuja dane do lokalnego bufora, a middleware bierze je asynchronicznie.

Praktyczny wniosek:
- nawet jesli "wszystko jest w jednym frameworku", petla RT musi miec pierwszenstwo i izolacje.

## Pytania do studentow
1. Dlaczego GC w sciezce RT jest ryzykiem i w jakiej warstwie moze byc bezpiecznie uzyty?
2. Jak wersjonujesz kontrakt danych, zeby W3/W4 nie psuly W2 po aktualizacji?
3. Jak zrobisz binding do biblioteki C, zeby nie wpuscic jej w sciezke RT niekontrolowanym sposobem?
4. Jakie dane powinny byc "stale rozmiarowo", a jakie moga byc zmienne (i dlaczego)?

## Projekty studenckie
- "Schema v1/v2": migracja kontraktu danych z zachowaniem kompatybilnosci wstecznej.
- "FFI boundary": prototyp granicy FFI (C<->Rust/C++) + testy stabilnosci i czasu.
- "Telemetry protocol": prosty protokol IPC z walidacja i licznikami dropow.

## BONUS
- Najlepsza wielojezycznosc to taka, ktora ma twarde granice: watek RT nie zna JSON, HTTP ani DB; on zna tylko strukture probki i licznik czasu.
