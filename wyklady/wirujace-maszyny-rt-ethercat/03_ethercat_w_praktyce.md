# Wykład 3: EtherCAT w praktyce (dla sterowania i synchronizacji)

## Cel
Pokazać, gdzie EtherCAT realnie pomaga, jakie problemy rozwiązuje, oraz na co uważać w implementacji.

> TL;DR: EtherCAT daje deterministykę tylko wtedy, gdy świadomie projektujesz: cykl, budżet CPU, synchronizację i zachowanie na awarię.

## Co EtherCAT wnosi do sterowania
W zastosowaniach ruchowych EtherCAT jest wybierany, bo umożliwia:
- deterministyczną wymianę danych w cyklu,
- synchronizację wielu urządzeń (Distributed Clocks),
- szybkie pętle dla wielu napędów i czujników.

## Typowe elementy architektury
- Master: kontroler czasu rzeczywistego (RTOS lub Linux RT).
- Slave: napędy, moduły I/O, moduły pomiarowe.
- Distributed Clocks: wspólny czas i zsynchronizowane próbkowanie/aktualizacja.

## Dane w praktyce: cykliczne vs acykliczne
W praktycznych wdrożeniach musisz rozdzielić, co ma iść w stałym cyklu, a co może iść „obok”:
- dane cykliczne: sygnały sterowania i pomiary potrzebne w pętli,
- dane acykliczne: konfiguracja, diagnostyka, serwis.

Zasada:
- do danych cyklicznych wrzucasz minimum potrzebne do stabilnego sterowania i safety,
- wszystko inne ograniczasz częstotliwościowo albo przenosisz poza cykl.

## Synchronizacja: po co i jak myśleć
Jeśli masz wiele osi lub wiele punktów pomiaru:
- niesynchronizowany sampling tworzy błąd fazowy,
- błąd fazowy wygląda jak szum i może pobudzać rezonanse.

Dlatego celem jest:
- wspólny czas,
- stałe okno, kiedy dane są „ważne”.

Praktyczny wniosek:
- synchronizacja jest „niewidzialna” dopóki działa, ale jej brak objawia się jako niestabilność i losowe błędy w estymacji/diagnostyce.

## Dobór cyklu: kompromis jakości sterowania i deterministyki
Cykl EtherCAT (i cykl sterowania) dobiera się przez kompromis:
- zbyt wolno: pętla widzi stare dane, spada pasmo i rośnie błąd,
- zbyt szybko: rośnie obciążenie CPU, jitter i ryzyko dropoutów.

Metoda praktyczna:
1) wybierz cykl startowy konserwatywnie,
2) zmierz jitter/WCRT i jakość regulacji,
3) iteruj: skracaj cykl tylko jeśli system utrzymuje deterministykę.

## Co zwykle psuje EtherCAT w praktyce
- zbyt duży cykl (za wolno względem dynamiki obiektu),
- zbyt mały cykl (CPU i sieć nie wyrabiają, rośnie jitter),
- mieszanie telemetrii z krytycznym ruchem w tym samym budżecie,
- brak pomiarów WCRT i jitteru (system działa „na oko”).

## Watchdog i zachowanie przy awarii komunikacji
Założenie inżynierskie: komunikacja czasem przestaje być idealna.
Projektujesz więc zachowanie:
- po stronie slave (napęd/I-O): brak aktualizacji -> stan ograniczony lub safe,
- po stronie master: degradacja albo safe stop, bez flappingu.

To nie jest „opcjonalne”:
- bez watchdogów awaria komunikacji może oznaczać utrzymanie ostatniego sterowania w nieskończoność.

## Integracja z napędami
W praktyce:
- pętla prądu jest w napędzie,
- pętla prędkości bywa w napędzie albo w masterze (zależnie od wymaganej dynamiki i funkcji),
- master koordynuje tryby, rampy, synchronizację.

Zasada bezpieczeństwa:
- warstwa napędu musi umieć bezpiecznie przejść w stan ograniczony nawet bez mastera (watchdog).

## Higiena sieci i izolacja od „IT”
Jeśli na tym samym sprzęcie/połączeniu mieszasz:
- ruch sterujący,
- telemetrię,
- zdalny dostęp,
to zwiększasz ryzyko jitteru.

Najbezpieczniejszy wzorzec:
- sieć ruchu (EtherCAT) traktuj jako osobny świat,
- telemetrię i integracje prowadź kanałem, który nie może zakłócić RT.

## Checklisty
- Wybierz cykl EtherCAT i cykl sterowania świadomie (z pomiarów, nie z „wydaje się”).
- Ustal, które pętle są w napędzie, a które w masterze.
- Zastosuj Distributed Clocks, jeśli potrzebujesz spójnego samplingu.
- Oddziel krytyczne PDO od niekrytycznych danych (telemetria, diagnostyka).

## Zadania (praktyka)
1. Zrób tabelę sygnałów i oznacz: „cykliczne” vs „acykliczne”.
2. Zdefiniuj logikę watchdogów: co robi slave, co robi master, jakie są warunki powrotu.
3. Zaproponuj plan testów: przeciążenie CPU mastera, duża telemetria, dropout komunikacji. Co mierzysz i jakie są kryteria „pass”?

## Pytania do studentów
1. Jakie sygnały muszą być w danych cyklicznych, a które mogą być acykliczne? Podaj uzasadnienie RT.
2. Kiedy synchronizacja (np. DC) realnie poprawia jakość sterowania, a kiedy jest „nice to have”?
3. Jakie są konsekwencje zbyt krótkiego cyklu komunikacji dla jitteru i stabilności pętli?
4. Jak zaprojektujesz zachowanie systemu przy awarii komunikacji, aby uniknąć utrzymania „ostatniej komendy”?

## Projekty studenckie
- „Signal budget”: narzędzie generujące tabelę sygnałów (cykliczne/acykliczne) wraz z estymacją kosztu w cyklu.
- „Watchdog spec”: specyfikacja i prototyp FSM watchdogów (slave + master) z testami fault-injection.
- „Reżim telemetrii”: mechanizm rate limiting i backpressure dla danych niekrytycznych.

## BONUS
- Jeśli integrujesz magistralę ruchu, traktuj telemetrię jak „gościa”: może działać tylko wtedy, gdy nie psuje cyklu; to powinno być twardo wymuszone w architekturze (limity, osobne wątki/procesy).
