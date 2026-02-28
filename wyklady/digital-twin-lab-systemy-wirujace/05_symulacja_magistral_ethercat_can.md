# Wykład 5: Symulacja magistral przemysłowych (EtherCAT, CAN/CAN-FD)

## Część I: Wstęp teoretyczny - Komunikacja jako nerw systemu

### Geneza: dlaczego komunikacja jest krytyczna

W systemach sterowania wirujących maszynami komunikacja to nie "dodatek" - to **nerw systemu**. To przez magistrale płyną:
- Zadania prędkości/momentu od kontrolera do napędu
- Pomiary prędkości, pozycji, temperatury z powrotem do kontrolera
- Komendy bezpieczeństwa (stop, safe torque off)
- Dane diagnostyczne i telemetria

Gdy komunikacja zawodzi, cały system staje. I nie chodzi tu tylko o "brak danych" - chodzi o **opóźnienia, jitter i dropouty**, które powodują, że sterowanie działa na przestarzałych informacjach.

### Dlaczego symulacja magistrali jest potrzebna

W idealnym świecie komunikacja jest natychmiastowa i niezawodna. W realnym świecie:
- **Opóźnienie**: dane potrzebują czasu na przejście przez magistrale
- **Jitter**: czas przejścia nie jest stały
- **Dropout**: dane giną (szum, kolizje, błędy CRC)
- **Kolizje**: w shared-medium (CAN) - wiele nadajników naraz

Każde z tych zjawisk wpływa na jakość sterowania. I każde musi być przetestowane - zanim pojawi się prawdziwy sprzęt.

### EtherCAT vs CAN - dwie filozofie

**EtherCAT** (Ethernet for Control Automation Technology):
- Sieć deterministyczna oparta na Ethernet
- Master wysyła ramkę, która przechodzi przez wszystkie slave'y
- Każdy slave "wyciąga" swoje dane i "wstawia" odpowiedź w locie
- Czas cyklu: typowo 100 μs - 1 ms
- Wymaga dedykowanego hardware (EtherCAT PHY)

**CAN (Controller Area Network)**:
- Magistrala shared-medium z arbitrażem
- Niższy priorytet = dłuższe oczekiwanie
- Czas cyklu: zależny od obciążenia, typowo 1-10 ms
- Prostszy hardware, niższy koszt

Wybór zależy od wymagań. EtherCAT - tam, gdzie potrzeba szybkości i determinizmu. CAN - tam, gdzie wystarczy "dostatecznie szybko" i liczy się koszt.

### Przemówienie Profesora

Pamiętam projekt, gdzie zespół zbudował piękne sterowanie. Regulator działał idealnie - w symulacji. Pętla na Linux PREEMPT_RT miała jitter poniżej 10 μs - super.

Ale system jako całość nie działał. Dlaczego? Bo nikt nie pomyślał o komunikacji.

EtherCAT w tym projekcie miał jitter rzędu 500 μs - 50x więcej niż pętla sterowania! Regulator dostawał przestarzałe dane, sterowanie było niestabilne.

Rozwiązanie: przeprojektowanie komunikacji, buforowanie, predykcja. Ale to kosztowało 3 miesiące.

Rada: komunikacja to nie "ostatni etap". Myślcie o niej od początku.

## Cel
Umieć przetestować architekturę komunikacji zanim kupisz sprzęt:
- wymiana danych cyklicznych,
- opóźnienia i jitter,
- dropouty,
- watchdog i reakcje awaryjne.

> TL;DR: Najpierw definiujesz "co jest cykliczne", potem testujesz co się dzieje, gdy cykl nie jest dotrzymany.

## Część II: Symulacja w praktyce

## EtherCAT: co warto symulować
- wymianę PDO (dane sterujące/pomiarowe w cyklu),
- wpływ obciążenia CPU na jitter,
- wpływ "dodatkowych danych" (telemetria) na deterministykę,
- zachowanie watchdogów.

### EtherCAT - co musisz wiedzieć

EtherCAT to technologia, która wygląda prosto, ale ma wiele subtelności:

**Cykl EtherCAT**:
- Master wysyła jedną ramkę
- Ramka przechodzi przez wszystkie slave'y (daisy-chain lub tree)
- Każdy slave "wyciąga" swoje dane wejściowe i "wstawia" dane wyjściowe
- Całość wraca do mastera

Czas cyklu = czas propagacji przez wszystkie slave'y + overhead. Typowo 100 μs - 1 ms.

**Źródła jitteru w EtherCAT**:
- Czas generacji ramki przez mastera
- Czas przetwarzania na slave'ach
- Czas propagacji (zależy od długości kabli)
- Obciążenie CPU mastera (gdy musi przetwarzać wiele PDO)

**Teletria vs determinizm**:
- Dane telemetrii (logi, statystyki) NIE powinny być w cyklu sterowania
- Oddzielny kanał (TCP/IP, acykliczny)
- Cykl sterowania = tylko dane krytyczne

### Przemówienie Profesora

Najczęstszy błąd, jaki widzę: "dodajmy telemetrię do cyklu EtherCAT, to przecież tylko parę bajtów".

NIE. Każdy dodatkowy bajt = dłuższa ramka = dłuższy cykl = większe opóźnienie.

A opóźnienie = niestabilność sterowania.

Rada: EtherCAT dla sterowania (PDO). Telemetria - osobną drogą. Zawsze.

## CAN/CAN-FD: co warto symulować
- opóźnienia wynikające z arbitrażu,
- straty/kolizje (w modelu obciążenia),
- wpływ priorytetów ramek na czas dostarczenia sygnału krytycznego.

### CAN - co musisz wiedzieć

CAN to "demokratyczna" magistrala - każdy węzeł może nadawać, ale gdy dwie osoby nadają naraz, wygrywa ten z wyższym priorytetem (niższy ID).

**Arbitraż CAN**:
- Wszystkie węzły nadają bit po bicie
- Gdy węzeł nadaje "0" (recesywny), a słyszy "1" (dominujący) - wie, że przegrał
- Wygrywa ten, kto nadaje najniższy ID (najwyższy priorytet)

**Konsekwencje**:
- Niskie priorytety mogą czekać bardzo długo przy dużym obciążeniu
- "Broadcast storm" od jednego węzła blokuje wszystko
- Brak gwarancji czasowej (tylko probabilistyczne)

**Kiedy CAN ma sens**:
- Gdy opóźnienie < 10 ms jest akceptowalne
- Gdy koszt jest kluczowy
- Gdy potrzebujesz niezawodności (CRC, acknowledgment)

**Kiedy NIE CAN**:
- Gdy potrzebujesz determinizmu < 1 ms
- Gdy masz wiele węzłów z wysokimi wymaganiami

## Zasada projektowa: budżet czasu end-to-end
Sygnał krytyczny ma:
- deadline,
- budżet opóźnienia,
- reakcję w razie przekroczenia.

### Budżet czasu - jak to policzyć

End-to-end latency = czas od pomiaru do aktuacji. Składowe:
- Czas próbkowania (gdy pomiar jest dostępny)
- Czas przetwarzania (algorytm)
- Czas komunikacji do aktuatora
- Czas aktuacji (np. generacja PWM)

Każda składowa musi być oszacowana i zsumowana. Deadline musi być mniejszy niż czas cyklu sterowania.

Przykład:
- Pomiar: 100 μs opóźnienia
- Algorytm: 50 μs
- Komunikacja: 200 μs
- Aktuacja: 50 μs
- Suma: 400 μs

Przy pętli 1 kHz (1000 μs) - mamy zapas. Ale przy pętli 500 Hz (2000 μs) - zapas mniejszy.

### Przemówienie Profesora

Projektanci często "zapominają" o czasie komunikacji w budżecie. "Nasz regulator działa w 100 μs" - super. Ale zapomnieli, że pomiar przychodzi z 200 μs opóźnieniem, a komenda idzie kolejne 200 μs.

W rezultacie: regulator myśli, że działa w 100 μs, a faktycznie całość to 500 μs.

Rada: zawsze mierzcie end-to-end, nie tylko "algorytm".

## Checklisty
- Masz listę sygnałów cyklicznych i acyklicznych.
- Masz test dropout + test przeciążeniowy.
- Watchdog ma zdefiniowane zachowanie i warunki powrotu.

### Checklist szczegółowy

**Klasyfikacja sygnałów:**
- [ ] Lista sygnałów krytycznych (cykliczne, z deadline)
- [ ] Lista sygnałów nie-krytycznych (acykliczne)
- [ ] Dla każdego sygnału: deadline, priorytet

**Symulacja:**
- [ ] Model opóźnienia (stałe + jitter)
- [ ] Model dropout (losowe Utrata pakietów)
- [ ] Test przeciążeniowy (max obciążenie magistrali)

**Watchdog:**
- [ ] Timeout dla każdego sygnału krytycznego
- [ ] Zachowanie po timeout (safe state)
- [ ] Warunki powrotu do normalnej pracy

**Budżet czasu:**
- [ ] End-to-end latency dla każdego sygnału krytycznego
- [ ] Porównanie z wymaganiami (deadline)
- [ ] Rezerwa (safety margin)

## Slajdy (tekstowe)
### Slajd 1: Co testujemy w magistrali
- cykl, jitter, dropouty
- watchdog i safe behavior

### Slajd 2: EtherCAT vs CAN
- EtherCAT: cykliczność i synchronizacja
- CAN: arbitraż i priorytety

## Pytania do studentów
1. Które sygnały muszą mieć twardy deadline i jak policzysz budżet end-to-end przez magistralę?
2. Jakie zachowanie watchdogów jest minimalnie wymagane, aby system był bezpieczny przy dropoutach?
3. Jakie są konsekwencje mieszania telemetrii z cyklem sterowania?
4. Jak przetestujesz arbitraż i priorytety w CAN (na poziomie architektury, nie tylko teorii)?

## Projekty studenckie
- „Bus simulator”: symulator opóźnień/jitteru/dropoutów i wpływu na sterowanie.
- „Signal classification”: narzędzie do klasyfikacji sygnałów (cykliczne/acykliczne) + rate limiting.
- „Watchdog harness”: testy fault-injection dla zachowania magistrali i reakcji systemu.

## BONUS
- W projektach integracyjnych najczęściej wygrywa zespół, który najpierw definiuje kontrakt sygnałów i zachowanie na awarię, a dopiero potem „podłącza kabel”.
