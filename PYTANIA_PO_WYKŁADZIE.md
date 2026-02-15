## 1️⃣ Dlaczego **przemysłowo‑inżynierska** sekcja będzie bardziej atrakcyjna niż czysto akademicka?

| Aspekt | Wykład akademicki | Wykład przemysłowy |
|--------|-------------------|--------------------|
| **Cel** | Zrozumienie podstaw teorii planowania, dowody, formalne modele | Szybkie wdrożenie, rozwiązywanie real‑world problemów, minimalizacja czasu wprowadzenia |
| **Materiał** | Wzory, dowody, zadania teoretyczne | Przykłady z konkretnych urządzeń (UAV, samochód, robot przemysłowy) |
| **Narzędzia** | Papier, symulator matematyczny, TLA⁺/UPPAAL | Tracealyzer, SystemViewer, QEMU, profilery, CI/CD pipelines |
| **Rezultat** | “Wiem, jak to działa” | “Potrafię to zaprojektować, wdrożyć i przetestować” |

**Dlatego** w sekcji przemysłowej **więcej czasu** poświęcimy na:

- praktyczne konfiguracje RTOS (Zephyr, FreeRTOS, ThreadX, QNX)
- przykłady synchronizacji i obsługi ISR w urządzeniach wielordzeniowych
- narzędzia do pomiaru jittera i latencji w czasie rzeczywistym
- case‑study z branż (automotive, aerospace, robotyka, IoT)

______________________________________________________________________

## 2️⃣ Kilka **interesujących przykładów** do wplecenia w wykład

| # | Przykład (branża) | Co warto pokazać w 10‑15 min? |
|---|-------------------|-----------------------------|
| 1 | **UAV – lot bezzałogowy** (kontrola lotu + sensor fusion) | <ul><li>Zadania o różnych periodach (250 µs – IMU, 1 ms – regulator, 5 ms – telemetryka)</li><li>Zarządzanie priorytetami i ISR dla DMA</li><li>Tracealyzer – pomiar jittera i wizualizacja timeline’u</li></ul> |
| 2 | **ECU samochodowy** (CAN‑FD + Ethernet) | <ul><li>Hierarchiczny harmonogram: zadania czasu rzeczywistego (ASIL‑D) vs. zadania diagnostyczne (ASIL‑A)</li><li>Priority‑inherited mutexy chroniące bufor CAN</li><li>Wymagania ISO 26262 (FMEA, SIL‑rating) w kontekście RTOS</li></ul> |
| 3 | **Robot przemysłowy** (sterowanie serwonapędów + wizja) | <ul><li>Wielordzeniowy procesor (Cortex‑A53) – podział na domeny: RT‑core vs. Linux‑core (RT‑Linux)</li><li>Wykorzystanie ARINC 653 (partitioning) w systemie bezpieczeństwa</li><li>Integracja z CI/CD – automatyczne testy jednostkowe + testy czasowe</li></ul> |
| 4 | **Smart‑meter / IoT gateway** (protokół Thread + MQTT) | <ul><li>Zadania o niskim obciążeniu (co sekundę) + zadania wysokiej krytyczności (alarm)</li><li>Lock‑free queue do wymiany danych między ISR a taskiem</li><li>Profilowanie latencji na ARM Cortex‑M33 w środowisku QEMU</li></ul> |
| 5 | **Avionika – Flight Management System** (ARINC 653) | <ul><li>Podział na partycje – każda ma własny scheduler</li><li>Wymóg deterministyczności ≤ 1 ms i jitter ≤ 5 µs</li><li>Analiza worst‑case execution time (WCET) w Sym‑TA/S</li></ul> |

Każdy z tych case‑study ma gotowy zestaw slajdów, przykładowy kod (przeważnie w Zephyr/FreeRTOS) oraz **zestaw testowy** (symulacja w QEMU + Tracealyzer). Po prezentacji każdy uczestnik może od razu uruchomić kod i zobaczyć, jak wygląda timeline na ekranie.

______________________________________________________________________

## 3️⃣ “Wymysły na koniec” – **Zaawansowany problem** dla najlepszych mózgów

Poniżej znajdziesz kilka **kompleksowych, praktycznych wyzwań**, które można zadać jako *bonus‑challenge* po wykładzie. Każde z nich łączy w sobie:

- **Projekt** – architektura RTOS i alokacja zasobów
- **Analiza** – formalna weryfikacja gwarancji czasowych i deterministyczności
- **Testowanie** – pomiar jittera, latencji i odporności na awarie
- **Weryfikacja** – wykorzystanie narzędzi (Tracealyzer, SystemC, TLA⁺/UPPAAL)

______________________________________________________________________

### 🛩️ **Problem A – Autonomiczny dron (multi‑core + mieszana krytyczność)**

| Element | Specyfikacja |
|---------|--------------|
| **Platforma** | Dual‑core ARM Cortex‑M7 (160 MHz) + 32 kB SRAM, 2 MB Flash |
| **Zadania** | <ul><li>**Sensor‑Fusion (SF)** – 250 µs, WCET = 120 µs, jitter ≤ 30 µs</li><li>**Control‑Loop (CL)** – 1 ms, WCET = 300 µs, jitter ≤ 100 µs</li><li>**Safety‑Monitor (SM)** – 5 ms, WCET = 1 ms, jitter ≤ 200 µs</li><li>**Telemetry (TL)** – 2 ms, WCET = 500 µs</li><li>**House‑keeping (HK)** – 10 ms, WCET = 1 ms</li></ul> |
| **Zasoby** | 1 × UART (ISR), 1 × SPI (DMA), 1 × CAN‑FD (ISR), 1 × PWM‑Timer, 2 × ADC |
| **Krytyczność** | SF, CL – ASIL‑D (silna preempcja); SM – ASIL‑C; TL, HK – ASIL‑A |
| **Ograniczenia** | <ul><li>CPU‑util ≤ 70 % przy pełnym obciążeniu</li><li>Brak priorytetowych inwersji (priority‑inheritance lub priority‑ceiling)</li><li>Latencja od ISR‑>Task ≤ 5 µs</li></ul> |
| **Cel** | Zaprojektuj harmonogram i alokację zadań, wybierz algorytm planowania (RM, EDF, MUF, DM), zapewnij **worst‑case deadline guarantee** oraz **minimum jitter**. Następnie zweryfikuj za pomocą **response‑time analysis** i symulacji w **Tracealyzer**. |
| **Deliverables** | <ul><li>Schemat alokacji zadań na rdzeni (czy zadania powinny być sticky czy migracyjne?)</li><li>Konfiguracja priorytetów i wykorzystanie **Priority Inheritance Protocol (PIP)** / **Priority Ceiling Protocol (PCP)**</li><li>Analiza formalna – obliczenie reakcji (worst‑case response) dla każdego zadania</li><li>Wyniki pomiarów (jitter, latencja, CPU‑load) z symulacji</li><li>Propozycję procedury **graceful degradation** (np. ograniczenie częstotliwości aktualizacji sensora przy przeciążeniu)</li></ul> |
| **Ocenianie** | <ul><li>Prawidłowość analizy (≤ 5 % błąd vs. symulacja)</li><li>Wykorzystanie narzędzi (Tracealyzer, QEMU, formal verification)</li><li>Jakość rozwiązania w kontekście wymagań ASIL (safety case)</li></ul> |

> **Dlaczego to jest “zabójczo”?**\
> Łączy wielordzeniowość, mieszaną krytyczność, priorytety i synchronizację w jednym systemie. Dodatkowo wymaga umiejętności pracy z narzędziami analitycznymi i tworzenia **dowodu** gwarancji czasowych – dokładnie to, czego potrzebują w branży lotniczej/robotycznej.

______________________________________________________________________

### 🚗 **Problem B – ECU samochodowy (mixed‑criticality, CAN‑FD + Ethernet, ISO 26262)**

| Element | Specyfikacja |
|---------|--------------|
| **Platforma** | 4‑core ARM Cortex‑R52 (300 MHz) + 256 kB SRAM |
| **Domeny** | <ul><li>ASILD (SIL 4) – sterownik hamulców (TC), sterownik prędkości (VC)</li><li>ASILB (SIL 3) – czujniki temperatury, diagnostyka OBD</li><li>ASILA (SIL 2) – logger, OTA</li></ul> |
| **Komunikacja** | <ul><li>CAN‑FD (125 kbps‑2 Mbps) – priorytety wiadomości</li><li>BroadR‑Reach (Ethernet 100 Mbps) – streaming danych</li></ul> |
| **Zadania** | <ul><li>TC: 1 ms, WCET = 300 µs (ASIL‑D)</li><li>VC: 2 ms, WCET = 250 µs (ASIL‑D)</li><li>Temp‑Monitor: 5 ms, WCET = 150 µs (ASIL‑B)</li><li>OBD‑diag: 10 ms, WCET = 800 µs (ASIL‑B)</li><li>Logger: 20 ms, WCET = 500 µs (ASIL‑A)</li></ul> |
| **Wymagania** | <ul><li>Zadania ASIL‑D **nie mogą być wyprzedzone** przez zadania niższej krytyczności (preempcja tylko w obrębie tego samego poziomu krytyczności).</li><li>Zapewnij **bounded jitter** dla TC (≤ 20 µs).</li><li>Wymóg **temporal isolation** pomiędzy domenami (ARINC 653‑style partitions).</li><li>Obsługa **timeoutów** i **fallback** (np. przełączenie na lokalny regulator przy utracie komunikacji Ethernet).</li></ul> |
| **Cel** | Zaprojektuj **hierarchiczny scheduler**: <br> 1️⃣ *Level‑0* – partycjonowanie domen (każda ma własny **rate‑monotonic** plan). <br> 2️⃣ *Level‑1* – **fixed‑priority** w ramach partycji (priorytety ASIL‑D najwyższe). <br> 3️⃣ **Budget‑monitoring** (CPU‑time) dla partycji ASIL‑B/A. <br> Następnie: <br> • wykonaj **schedulability analysis** (Liu‑Layland, Bini‑Baruah, Choudhury). <br> • przeprowadź **simulation** w **SystemC** + **Tracealyzer** i zmierz **worst‑case response**. <br> • stwórz **failure‑mode analysis** (FMEA) i zaproponuj mechanizm **graceful degradation** (np. przejście na lokalny regulator przy utracie danych z czujników). |
| **Deliverables** | <ul><li>Model architektury (partitioning) + diagram priorytetów</li><li>Analiza formalna – wykazanie spełnienia deadline’ów</li><li>Konfiguracja RTOS (np. QNX, Zephyr) – plik konfiguracyjny (device‑tree, Kconfig)</li><li>Skrypt symulacji (Python + Tracealyzer API) + wyniki (latencja, jitter, CPU‑load)</li><li>Plan testów funkcjonalnych (ISO 26262 – HARA, Safety‑Case)</li></ul> |

> **Dlaczego “zabójczo”?**\
> Połączenie **mixed‑criticality**, **sieci real‑time** (CAN‑FD, Ethernet) i **normatywnych wymagań** (ISO 26262, ASIL) wymaga zarówno wiedzy o planowaniu, jak i o certyfikacji. To jest typowy problem w **automotive**.

______________________________________________________________________

### 🤖 **Problem C – Robot przemysłowy (heterogeniczny SoC, RT‑Linux + RT‑Core, ARINC 653‑style partitions)**

| Element | Specyfikacja |
|---------|--------------|
| **Platforma** | **Xilinx Zynq‑UltraScale+** – 4‑core Cortex‑A53 (Linux) + 2‑core Cortex‑R5 (RT‑Core) |
| **Zadania (RT‑Core)** | <ul><li>Serwo‑Drive (SD) – 250 µs, WCET = 150 µs (SIL‑4)</li><li>Force‑Feedback (FF) – 500 µs, WCET = 300 µs (SIL‑4)</li><li>Safety‑Watchdog (SW) – 2 ms, WCET = 1 ms (SIL‑3)</li></ul> |
| **Zadania (Linux)** | <ul><li>Vision‑Processing (VP) – 10 ms, WCET = 5 ms (SIL‑2)</li><li>User‑Interface (UI) – 30 ms, WCET = 10 ms (SIL‑1)</li></ul> |
| **Komunikacja** | <ul><li>AXI‑Lite/AXI‑Stream między RT‑Core i Linux</li><li>Ethernet‑based ROS2 – wymagania deterministyczne (latencja ≤ 2 ms, jitter ≤ 500 µs)</li></ul> |
| **Wymagania** | <ul><li>RT‑Core musi być **hard‑real‑time** – żadne zadania Linux nie mogą go blokować dłużej niż 5 µs.</li><li>Zapewnij **temporal isolation** (partition) dla RT‑Core i Linux.</li><li>Implementuj **cross‑core lock‑free queue** (single‑producer, single‑consumer) – analiza latencji i brak contention.</li></ul> |
| **Cel** | Zaprojektuj **dual‑core architecture**: <br> 1️⃣ **RT‑Core** – scheduler RM/EDF, **hard‑real‑time tasks** (SD, FF, SW). <br> 2️⃣ **Linux** – **SCHED_FIFO** dla VP i UI, ograniczony przez **cgroups**/**rt‑cgroup** żeby nie przekroczyć 5 % CPU w trakcie RT‑Core. <br> 3️⃣ **IPC** – lock‑free ring buffer (single‑producer, single‑consumer) + DMA (AXI‑DMA) – wykonać **latency budget analysis**. <br> 4️⃣ **Verification** – wykorzystaj **UPPAAL** do modelowania komunikacji i sprawdzenia bounded‑delay. <br> 5️⃣ **Test** – uruchom **QEMU** z symulacją SoC, zmierz jitter i latency w Tracealyzer. |
| **Deliverables** | <ul><li>Schemat rozdziału CPU (core‑affinity, IRQ‑affinity, SMP‑migration)</li><li>Konfiguracja Zephyr/FreeRTOS + Linux (device‑tree overlay, kernel config)</li><li>Model w UPPAAL (czasowe własności, dowód ograniczonej latencji)</li><li>Wyniki pomiarów (latencja cross‑core, jitter SD, FF, CPU‑load)</li><li>Plan integracji z CI/CD (testy CI z symulacją i testy hardware‑in‑the‑loop)</li></ul> |

> **Dlaczego “zabójczo”?**\
> To **heterogeniczny SoC** – mieszanka twardego RTOS i systemu Linux – a jednocześnie **ARINC 653‑style partitions** oraz **lock‑free IPC**. W praktyce takie rozwiązania spotyka się w **robotyce wysokiej klasy** (np. roboty medyczne, automatyka produkcyjna).

______________________________________________________________________

### 🛰️ **Problem D – Avionics Flight Management System (ARINC 653, time‑partitioned scheduler, mixed‑criticality, formal verification)**

| Element | Specyfikacja |
|---------|--------------|
| **Platforma** | 2‑core POWERPC e500 (250 MHz) + 1 × FPGA (Hardware‑accelerated avionics) |
| **Partitioned OS** | ARINC 653 (np. VxWorks 653, LynxOS‑178) – **time‑partitioning** (major frame = 10 ms) |
| **Zadania** | <ul><li>**Navigation (NAV)** – 5 ms, WCET = 2 ms (SIL‑4)</li><li>**Guidance (GUID)** – 2 ms, WCET = 800 µs (SIL‑4)</li><li>**Health‑Monitor (HM)** – 10 ms, WCET = 1 ms (SIL‑3)</li><li>**Data‑Logger (DL)** – 100 ms, WCET = 20 ms (SIL‑2)</li></ul> |
| **Komunikacja** | <ul><li>**ARINC 429** (1553‑B) – deterministyczna, latency ≤ 1 ms</li><li>**Ethernet (AFDX)** – wymagania jitter ≤ 20 µs</li></ul> |
| **Wymagania** | <ul><li>Każda **partition** ma swoje **fixed‑time slot** w major frame (np. NAV 2 ms, GUID 1 ms, HM 3 ms, DL 4 ms). </li><li>Zadania w tej samej partition **preemptive** – RM lub EDF (wewnętrznie). </li><li>**Temporal firewall**: brak przenikania między partitions (no‑preempt, no‑shared resource).</li></ul> |
| **Cel** | Zaprojektuj **major‑frame schedule**: <br> 1️⃣ Ustal długości slotów tak, żeby sumaryczny czas ≤ 10 ms i każde zadanie mieści się w swoim **worst‑case execution time**. <br> 2️⃣ Wybierz **EDF** lub **RM** wewnątrz partition (uzasadnij). <br> 3️⃣ Zbuduj **model czasowy** w **UPPAAL** (major frame, slot, task deadline). <br> 4️⃣ Wykonaj **formal verification**: *deadline‑misses = 0*, *no‑overlap*, *bounded jitter*. <br> 5️⃣ Przeprowadź **simulation** w **Simulink + RTOS‑in‑the‑Loop** (MATLAB/Simulink) – weryfikacja latencji w AFDX. |
| **Deliverables** | <ul><li>Grafik major‑frame z slotami (tabela)</li><li>Konfiguracja ARINC 653 (plik .cfg)</li><li>Model UPPAAL + wyniki weryfikacji (logi, trace)</li><li>Skrypt Simulink (model + test‑vectors) + wyniki (jitter, latencja)</li><li>Opis **Safety Case** – dowód, że system spełnia DO‑178C / DO‑254 wymagania dla SIL‑4</li></ul> |

> **Dlaczego “zabójczo”?**\
> To **real‑world system lotniczy** z wymogami **certyfikacji**. Wymaga zrozumienia **ARINC 653**, **formal verification** i **time‑partitioned scheduling** – nie jest to czysto akademickie zadanie, ale realny problem w branży lotniczej.

______________________________________________________________________

### 📡 **Problem E – Real‑Time Sensor Network (Zigbee/Thread, Low‑Power, Deterministic Sleep/Wake‑Up)**

| Element | Specyfikacja |
|---------|--------------|
| **Platforma** | ARM Cortex‑M0+ (48 MHz) + 64 KB RAM, 256 KB Flash |
| **Zadania** | <ul><li>**Sensor‑Acquisition (SA)** – 200 µs (ADC), WCET = 80 µs</li><li>**Packet‑Transmission (PT)** – 2 ms (IEEE 802.15.4), WCET = 800 µs</li><li>**Power‑Management (PM)** – 50 µs (wake‑up/timer), WCET = 30 µs</li></ul> |
| **Wymagania** | <ul><li>Cykl pracy: 10 ms **active** → 990 ms **sleep** (wake‑up na timer).</li><li>Zapewnić **deterministyczny wake‑up latency** ≤ 5 µs.</li><li>Brak **priority inversion** między ISR (timer) a PT.</li><li>Użycie **tickless** FreeRTOS (idle‑tick) i **low‑power timer** (LPTIM).</li></ul> |
| **Cel** | Zaprojektuj **low‑power RTOS** z **tickless idle** i **dynamic tick frequency** (przełączanie między 1 kHz a 100 kHz). <br> 1️⃣ Wykorzystaj **FreeRTOS‑Tickless** i **low‑power timer** (LPTIM). <br> 2️⃣ Zaimplementuj **queue‑based ISR** – ISR wypełnia ring buffer, a task PT odbiera z niego. <br> 3️⃣ Wykonaj **energy‑profile**: pomiar prądu (µA) w trybie sleep vs. active, wyliczenie **average power consumption**. <br> 4️⃣ Przeprowadź **latency analysis** w Tracealyzer (wake‑up → SA → PT). |
| **Deliverables** | <ul><li>Konfiguracja FreeRTOS (tickless, LPTIM, idle hook)</li><li>Diagram ISR → task (priority inheritance, lock‑free)</li><li>Wyniki pomiarów (jitter, wake‑up latency, średni prąd)</li><li>Wnioski dotyczące trade‑off’ów (częstotliwość timera vs. energia vs. deterministyczność)</li></ul> |

> **Dlaczego “zabójczo”?**\
> Łączy **low‑power design** i **hard‑real‑time** w jednym, często spotykanym w IoT. Wymaga dogłębnej znajomości mechanizmów RTOS (tickless, idle) oraz umiejętności **pomiarów energii**.

______________________________________________________________________

## 4️⃣ Jak wykorzystać te projekty w praktyce (po wykładzie)

1. **Dostarczenie materiałów**

   - repozytorium Git z gotowymi szablonami (FreeRTOS/Zephyr, konfiguracja Kconfig, pliki .c/.h).
   - gotowe skrypty uruchamiające symulację (Docker + QEMU + Tracealyzer).
   - **README** z krok‑po‑kroku instrukcją uruchomienia (zajmuje \< 5 min).

1. **Ocena rozwiązań**

   - **Automatyczne testy CI**: `ci.yml` – build, unit‑tests, performance‑tests (jitter measurement).

   - **Rubryka** (przykładowa):

     | Kryterium | 0 % | 50 % | 100 % |
     |-----------|-----|------|-------|
     | Analiza teoretyczna (response‑time, bounded jitter) | Niepoprawna | Częściowo poprawna | Pełna, dowód poprawny |
     | Implementacja RTOS (konfiguracja, ISR, mutex) | Brak | Częściowa | Poprawna, zgodna z best‑practices |
     | Pomiary (Tracealyzer, energia) | Brak | Częściowe | Komplet, wraz z interpretacją |
     | Dokumentacja (Safety‑Case, HARA) | Brak | Szkic | Kompletny case z referencjami |
     | Kreatywność (graceful degradation, extra features) | Brak | Prosty pomysł | Zaawansowany, praktyczny pomysł |

1. **Wydarzenie “hackathon”**

   - **Zespół** 3‑4 osoby (mix – hardware, firmware, test).
   - **Czas**: 4‑6 h (dwie sesje po 2‑3 h).
   - **Nagrody**: licencje na **Tracealyzer Pro**, zestawy **STM32 Nucleo**, dostęp do **cloud CI/CD**.
   - **Prezentacja końcowa** (10 min) + **live demo** (symulacja + pomiar na hardware).

1. **Po‑wykładzie** – **prace naukowe**

   - Uczestnicy mogą rozbudować rozwiązanie do **publikacji konferencyjnej** (np. **IEEE Real‑Time Systems Symposium**, **Embedded Systems Week**).\
     -导师 może prowadzić **seminarium** na temat **formal verification** (UPPAAL, TLA⁺) w kontekście RTOS.

______________________________________________________________________

## 5️⃣ Szybka check‑lista „co trzeba mieć przed rozpoczęciem”

| Co | Dlaczego |
|----|----------|
| **Środowisko Docker** (z QEMU, FreeRTOS/Zephyr, Tracealyzer, UPPAAL) | Jedno polecenie `docker run …` – szybki start, brak instalacji lokalnych. |
| **Pliki konfiguracyjne** (`.config`, device‑tree, Kconfig) | Gotowe szablony pozwalają skupić się na analizie, a nie na „klikaniu”. |
| **Test‑vectors** (np. zestaw danych dla sensor‑fusion, obciążenie CAN) | Łatwiejsze odtworzenie warunków granicznych. |
| **Instrukcja krok‑po‑kroku** (README + FAQ) | Minimalizuje czas „szukania” i pozwala skupić się na problemie. |
| **Rubryka ocen** (z góry znana) | Uczestnicy wiedzą, co jest ważne i mogą planować swoją pracę. |
| **Materiały dodatkowe** (PDF z wykładem + slajdy, krótkie video o RTOS) | Dla osób, które chcą pogłębić teorię. |

______________________________________________________________________

## 6️⃣ Podsumowanie

- **Sekcja przemysłowo‑inżynierska** powinna opierać się na **konkretnych case‑study** (UAV, ECU, robot, smart‑meter, avionics) oraz **narzędziach debugowania** (Tracealyzer, SystemViewer).
- **Zaawansowane problemy** (A‑E) łączą **planowanie, synchronizację, mixed‑criticality, sieci real‑time, niską energię i formalną weryfikację**. Każdy jest **realny** i **dostosowany do konkretnej branży**.
- **Dostarczenie gotowych szablonów i testów** w Docker, **automatyczna ocena CI** oraz **rubryka** zapewnią, że najlepsi uczestnicy będą mogli skoncentrować się na **innowacjach** i **dowodach poprawności** – a nie na konfiguracji środowiska.

**Powodzenia przygotowaniu wykładu!** 🎓🚀 Jeśli potrzebujesz gotowych slajdów, repozytorium lub szczegółowej instrukcji do jednego z powyższych problemów, daj znać – przygotujemy kompletny pakiet w ciągu 24 h.
