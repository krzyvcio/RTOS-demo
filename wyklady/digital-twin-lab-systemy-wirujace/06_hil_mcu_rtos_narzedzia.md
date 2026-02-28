# Wykład 6: HIL w małej skali (MCU + RTOS) i narzędzia warsztatowe

## Część I: Wstęp teoretyczny - HIL jako "pół-realne" testy

### Geneza: dlaczego potrzebujemy "pół-prawdziwego" środowiska

Do tej pory testowaliśmy:
- Model (ODE) - czysta symulacja
- Sterowanie - też w symulacji
- RT Linux - na prawdziwym sprzęcie, ale bez mechaniki
- Komunikacja - symulowana lub na prawdziwym sprzęcie

Ale brakuje jednego: **testu firmware na MCU w pętli z symulowanym obiektem**. To jest HIL (Hardware-in-the-Loop).

HIL pozwala testować:
- Firmware na prawdziwym MCU (nie symulowanym)
- RTOS (scheduler, mutexy, semafory)
- Obsługę przerwań
- Komunikację z peryferiami
- Watchdogi i safe states
- Timing w rzeczywistym środowisku embedded

### Dlaczego MCU + RTOS jest inne niż Linux

Linux (nawet PREEMPT_RT) to "duży" system:
- MMU, wirtualizacja pamięci
- System plików, sieć
- Wiele procesów

MCU + RTOS to "lekki" system:
- Brak MMU lub uproszczony (M-mode w RISC-V)
- Brak systemu plików (lub uproszczony)
- Wszystko działa w jednym obrazie (firmware)

To ma konsekwencje:
- Inne narzędzia debugowania
- Inny profiling
- Inne podejście do testowania

### Co testujemy w HIL

W HIL testujemy to, czego nie przetestujesz na PC:
- **Timing na prawdziwym MCU** - jak długo naprawdę trwa funkcja?
- **Obsługa przerwań** - czy ISR działa poprawnie?
- **RTOS** - czy zadania działają z właściwymi priorytetami?
- **Watchdog** - czy odlicza i resetuje poprawnie?
- **Komunikacja** - UART, SPI, I2C, CAN - timing fizyczny
- **Safe states** - co robi firmware gdy "coś pójdzie nie tak"?

### Przemówienie Profesora

Kiedy współpracowałem z zespołem embedded, często słyszałem: "Ale na PC działało!"

Tak, na PC działało. Ale na MCU:
- Stack ma 2 KB zamiast 8 MB
- Nie ma malloc (albo jest, ale wolny)
- Przerwania mają inny priorytet
- Timery działają inaczej

I nagle "działa na PC" staje się "nie działa na MCU".

HIL jest po to, żeby te różnice wyłapać WCZEŚNIEJ. Zanim prototyp pojedzie na stół montażowy.

## Cel
Zrobić "pół-realne" testy bez budowania docelowego hardware:
- firmware na MCU (FreeRTOS/Zephyr),
- PC jako "plant" (symulacja obiektu),
- komunikacja (UART/CAN/UDP),
- pomiary na stole (debug, timing, sygnały).

> TL;DR: HIL to sposób na test firmware i RT zachowania, zanim mechanika istnieje.

## Część II: MCU i RTOS

## MCU i RTOS: co testujesz naprawdę
- deterministykę pętli (deadline, jitter),
- obsługę przerwań i komunikacji,
- watchdogi i reakcje awaryjne,
- działanie filtrów i regulatorów w realnym czasie.

### Typowy RTOS dla embedded

**FreeRTOS**:
- Najpopularniejszy RTOS dla MCU
- Prosty, lekki, dobrze udokumentowany
- Wsparcie dla wielu architektur (ARM Cortex-M, ESP32, etc.)
- Społeczność i wiele przykładów

**Zephyr**:
- Nowoczesny RTOS (Linux Foundation)
- Device tree, konfiguracja przez Kconfig
- Wsparcie dla wielu architektur
- Aktywny rozwój

**RT-Thread**:
- Chiński RTOS, popularny w Azji
- Bogata biblioteka
- GUI, filesystem, networking

Wybór zależy od projektu. Dla laboratoriow - FreeRTOS jest najprostszy do startu.

### Co musi umieć firmware w RTOS

**Zadania (tasks/threads)**:
- Periodic task (pętla sterowania)
- Communication task (odbieranie/nadawanie danych)
- Diagnostic task (monitorowanie stanu)

**Synchronizacja**:
- Mutexy do chronienia zasobów
- Semafory do sygnalizacji między zadaniami
- Kolejki do przesyłania danych

**Timing**:
- Timer systick lub hardware timer
- Pomiar czasu wykonania
- Deadline monitoring

### Przemówienie Profesora

Pamiętam debugowanie systemu, gdzie pętla sterowania działała "nieregularnie" - czasem szybko, czasem wolno.

Okazało się, że zadanie komunikacji (UART) miało wyższy priorytet niż pętla sterowania. Gdy przychodziły dane, pętla sterowania była przerywana na setki mikrosekund.

Rozwiązanie: zmiana priorytetów. Ale żeby to znaleźć, trzeba było:
1. Wiedzieć, że problem istnieje
2. Mieć narzędzia do pomiaru timing

Rada: od początku definiujcie priorytety zadań i mierzcie, czy są zachowane.

Rada: od początku definiujcie priorytety zadań i mierzcie, czy są zachowane.

## Narzędzia HW, które przyspieszają debug
- debug probe (J-Link lub podobny),
- logic analyzer (widzisz timing na pinach),
- oscyloskop (widzisz rzeczywiste zakłócenia i sygnały),
- zasilacz laboratoryjny (symulacja spadków/warunków).

### Narzędzia programistyczne

**Debug probe**:
- J-Link (Segger) - najpopularniejszy, szybki
- ST-Link (STMicroelectronics) - dla STM32
- CMSIS-DAP - open source

Pozwala na:
- Step-by-step debugging
- Zmienne na żywo
- Breakpointy
- Flashowanie

**Logic analyzer**:
- Saleae Logic
- OpenScope (Digilent)
- FX2LP CY7C68013A + open source

Pozwala na:
- Timing protokołów (UART, SPI, I2C)
- Korelację zdarzeń
- Long capture (minuty-godziny)

**Oscyloskop**:
- Dla sygnałów analogowych
- Pomiar szumu, zakłóceń
- Debug sprzętu (PWM, enkodery)

### Przemówienie Profesora

Najlepszy inżynier, jakiego znałem, miał na biurku:
- J-Link
- Logic analyzer 8-kanałowy
- Oscyloskop 2-kanałowy
- Zasilacz laboratoryjny

"Po co tyle?" - pytali.

"Mniej czasu tracę na zgadywanie, więcej na rozwiązywanie" - odpowiadał.

Narzędzia nie są "dla leniwych" - są dla efektywnych.

## Checklisty
- Firmware nie ma alokacji w pętli krytycznej.
- Masz pomiar czasu iteracji (GPIO toggle, timestamp).
- Watchdog jest testowany przez fault-injection.

### Checklist szczegółowy

**Firmware:**
- [ ] Brak malloc/new w pętli sterowania
- [ ] Stos o odpowiednim rozmiarze
- [ ] Poprawne priorytety zadań
- [ ] Obsługa błędów (assert, recovery)

**Timing:**
- [ ] Pomiar czasu wykonania pętli (GPIO toggle)
- [ ] Deadline monitoring (czy pętla mieści się w czasie?)
- [ ] Jitter pętli (p95, p99)

**Watchdog:**
- [ ] Watchdog skonfigurowany
- [ ] Petowanie w pętli głównej
- [ ] Test fault-injection (symulacja zawieszenia)

**Narzędzia:**
- [ ] Debug probe działa
- [ ] Flash przez debugger działa
- [ ] Logic analyzer podłączony
- [ ] Możliwość pomiaru timing na GPIO

## Slajdy (tekstowe)
### Slajd 1: Po co HIL
- test firmware i RT bez mechaniki

### Slajd 2: Co mierzyć na MCU
- deadline, jitter, watchdog

## Pytania do studentów
1. Jak zmierzysz jitter pętli na MCU bez wpływu debuggera na timing?
2. Jak zaprojektujesz komunikację MCU<->PC tak, aby nie blokowała pętli RT?
3. Jakie błędy sprzętowe/firmware chcesz wstrzykiwać w HIL i jak je wykryjesz?
4. Co powinno być „twardo” w firmware niezależnie od mastera (limity, watchdog, safe state)?

## Projekty studenckie
- „MCU loop timing”: pomiar `rt_loop_us` na MCU (GPIO toggle + logic analyzer) + raport.
- „RTOS tasks”: zestaw zadań RTOS (control, comm, diagnostics) z priorytetami i testem przeciążeniowym.
- „Watchdog demo”: watchdog + safe state + testy dropoutów komunikacji.

## BONUS
- Jeśli nie potrafisz odtworzyć tego samego problemu timingowego na stole (HIL), to na sprzęcie docelowym będzie tylko gorzej. HIL ma służyć powtarzalności.
