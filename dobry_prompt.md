Poniższe rozbicie pokazuje praktyczną architekturę “RTOS + Linux” w robocie humanoidalnym (androidzie), z naciskiem na kończyny (napędy, sterowanie ruchem) i “elektronikę głębszą” (zasilanie, drivery silników, magistrale, EMC, bezpieczeństwo).

Architektura systemowa (warstwy)
Warstwa napędów (Hard Real-Time, 1–10 kHz):
Mikrokontrolery (STM32, ESP32-S3, TI C2000, NXP i.MX-RT) z RTOS (FreeRTOS/Zephyr) per staw lub per grupa stawów.
Zadania: pętle FOC prądu (10–40 kHz), pętla prędkości/pozycji/impedancji (1–4 kHz), odczyt enkoderów, czujniki momentu/siły.
Interfejs do nadrzędnego systemu: EtherCAT, CAN-FD, RS-485 (Half-Duplex), rzadziej UDP raw.
Warstwa koordynacji kończyn (Soft/firm Real-Time, 250–1000 Hz):
Linux z łatką PREEMPT_RT lub Xenomai; alternatywnie CPU izolowane i SCHED_FIFO.
Zadania: kinematyka odwrotna, impedancja na poziomie segmentu, synchronizacja wielu stawów, kontakt z podłożem.
Komunikacja cykliczna czasu deterministycznego do MCU napędów (EtherCAT/CAN-FD).
Warstwa percepcji i planowania (Best-effort na Linux):
Percepcja 2D/3D (kamery, LiDAR), SLAM, rozpoznawanie obiektów, planowanie ruchu całego ciała, dialog.
ROS 2 (DDS) z profilami QoS (deadline, reliability), oddzielona od pętli sterowania twardo-czasowej.
Warstwa bezpieczeństwa:
Nadzorca (safety MCU, SIL/PL) z niezależnym zasilaniem i wejściem E-Stop, odcina zasilanie mocy i/lub wymusza tryb bezpieczny.
Hardware interlocks: wyłączniki krańcowe, czujniki drzwi, watchdog na Linuxie i na MCU.
Linux vs RTOS – role i decyzje
RTOS (MCU):
Gdzie wymagane jest \<100 µs jitteru, brak page-faultów, deterministyczny dostęp do GPIO/PWM, praca przy wysokich częstotliwościach.
Implementuje najniższe pętle regulacji (prąd, prędkość, moment, impedancja stawu).
Linux (SBC/CPU x86/ARM):
Integracja systemowa, percepcja, planowanie, UI, logowanie, komunikacja chmura.
Z PREEMPT_RT + tuning jądra uzyskasz jitter rzędu kilkunastu–kilkudziesięciu µs dla wątków SCHED_FIFO, ale to i tak za mało na FOC – wystarczające na 250–1000 Hz koordynacji kończyn.
Typowe problemy w kończynach (mechanika + sterowanie)
Przekładnie i napędy:
Luz, histereza, tarcie statyczne, compliance (harmonic drive, linki), nieliniowości → drgania, overshoot.
Rozwiązania: obserwatory tarcia (LuGre/Stribeck), feedforward (modele ID), filtry notch na rezonanse, identyfikacja compliance i kompensacja.
Czujniki i kalibracje:
Dryf offsetu enkoderów/momentu, zakłócenia od silników, słaba referencja masy.
Rozwiązania: wielopunktowa kalibracja, enkodery absolutne + indeks, auto-zero czujników momentu przy odciążeniu.
Sterowanie:
Windup integratora, saturacja prądów, niestabilność przy interakcji z człowiekiem (sztywne nastawy).
Rozwiązania: anti-windup, ograniczniki prądu/jerk, przełączanie trybów (pozycja/impedancja/siła), detekcja kolizji z szybkim obniżeniem sztywności.
Kinematyka i dynamika:
Osobliwości (singularities), limit prędkości/zakresu, sprzężenie między osiami.
Rozwiązania: damped least squares IK, ograniczenia w solverze, preview trajektorii, rozdział na toretkę i końcówkę manipulacyjną.
Termika i zasilanie:
Przegrzewanie uzwojeń i mostków MOSFET, spadki napięcia przy szczytach prądowych.
Rozwiązania: modele termiczne w sterowniku, derating mocy, czujniki temperatury w pakiecie, soft-start pętli.
Typowe problemy w “elektronice głębszej”
Drivery silników BLDC/FOC:
Błędy pomiaru prądu (offset, gain), dead-time, shoot-through, ringing na bramkach.
Rozwiązania: kalibracja CSA, kompensacja dead-time, proper gate drive, snubbery RC/RCD, layout o niskiej indukcyjności pętli mocy.
Layout PCB wysokoprądowych:
Pętle prądowe, ground bounce, wspólna masa z analogiem, wtryski zakłóceń do IMU/ToF.
Rozwiązania: gwiazdowa masa, wydzielona “analog ground”, szerokie polygony mocy, separacja sygnałów niskoszumowych, kontrola powrotów prądu.
EMC/EMI i okablowanie:
Przewody silnikowe emitują szerokopasmowe zakłócenia → błędy enkoderów, dropy na magistralach.
Rozwiązania: kable ekranowane, dławiki ferrytowe, filtry LC na zasilaniu, poprawne uziemienie ekranów 360°, separacja galwaniczna (isolated gate drivers, digital isolators).
Magistrale:
CAN-FD: kolizje, błędne terminacje, zbyt długie odnogi.
EtherCAT: jitter zegara, problemy PTP, zasilanie PoE zakłóca FOC.
Rozwiązania: poprawne Rterm, krótkie stuby, kontrola impedancji, synchronizacja czasu (PTP/Distributed Clocks), izolacja galwaniczna portów.
Zasilanie i BMS:
Brownout przy szczytach, tętnienia, grzanie przetwornic.
Rozwiązania: odpowiednia rezerwa mocy, duże low-ESR bulk caps blisko mostków, soft-start, OR-ing, monitor napięć, ścieżki bezpiecznikowe.
Bezpieczeństwo:
Brak jednokanałowego odcięcia mocy, zaufanie do software E-Stop.
Rozwiązania: sprzętowy E-Stop odcinający HV lub bramkowanie driverów, nadzorca z niezależnym zegarem, redundantne ścieżki pomiaru.
Komunikacja i middleware
ROS 2 na Linux:
QoS: Reliability=RELIABLE dla krytycznych stanów, BestEffort dla strumieni wideo; Deadline/Liveliness do detekcji opóźnień.
Wykonawcy czasu rzeczywistego: rclcpp z priority-based callback groups, pinning do izolowanych CPU.
Kanał sterowania napędami:
EtherCAT (najbardziej deterministyczny dla wielu osi, zsynchronizowany 1 kHz).
CAN-FD (tańszy, mniejsza przepustowość; typowo 500–2000 Hz dla pakietów krótkich).
UDP + PTP/TSN tylko gdy interfejsy i switch’e wspierają deterministykę.
Zegary i synchronizacja:
PTP (IEEE 1588) lub EtherCAT Distributed Clocks do synchronizacji Linux↔MCU i między MCU.
Na Linux: chrony/ptp4l, napięcia CPU i governor w trybie performance.
Linux PREEMPT_RT – praktyczny tuning
Jądro: PREEMPT_RT, High-Res Timers, tickless (nohz_full dla izolowanych rdzeni).
CPU shielding: isolcpus, rcu_nocbs, irqbalance off lub przypięcie IRQ do rdzeni nie-RT.
Wątki sterowania: SCHED_FIFO z wysokimi priorytetami, CPU affinity.
Wyłączyć: intel_pstate/turbo (jeśli wymusza zmienny jitter), C-states głębokie.
Narzędzia: cyclictest, rtla osnoise/rtla timerlat, ftrace, perf, hwlatdetect – mierzyć i dokumentować jitter.
Projekt sterowania kończynami (wzorzec)
Na MCU (RTOS):
ISR od PWM/ADC → pętla prądu (20–40 kHz).
Zadanie 1–4 kHz: moment/pozycja/impedancja, filtracja czujników, ochrona termiczna/prądowa, detekcja kolizji, watchdog comms.
Prosty protokół PDO (EtherCAT) lub ramek CAN-FD z czasem i sekwencją; timestampy.
Na Linux (RT):
Wątek 1 kHz: rozsyła wektory referencji (tau/pos/vel), odbiera sprzężenia, solver IK/ID (np. RNEA), kontrola kontaktu (ZMP/CoP).
Wątek 100–200 Hz: planowanie trajektorii i ograniczenia (acc/jerk).
Asynchroniczne: percepcja, mapowanie, UI.
Testy, diagnostyka i logowanie
Logi pierścieniowe RAM (minimalny narzut), marker timestamp z tego samego zegara co sterowanie.
Blackbox joint-level na MCU: snapshot ostatnich 1–2 s próbek przy fault.
Self-testy przy starcie: enkodery, kierunek faz, czujniki momentu, krańcówki.
HIL/SIL: symulacja dynamiki (Gazebo/Ignition, MuJoCo) + loopback magistrali.
BIAS/offset sweep dla CSA i IMU; testy EMI w komorze (przynajmniej pre-compliance).
Typowe awarie i jak je diagnozować
Niestabilność przy kontakcie:
Objawy: “twarde” odbicia, drgania przy dotyku.
Diagnoza: bode joint’a (chirp), identyfikacja rezonansów.
Naprawa: obniżenie Kp, zwiększenie Kd, notch na rezonans, przejście w impedancję.
Drifty czujników:
Objawy: powolne pływanie pozycji/momentu.
Diagnoza: log offsetów vs temperatura, korelacja z prądem.
Naprawa: termokompensacja, auto-zero, poprawa referencji.
Ucieczki czasowe (jitter):
Objawy: sporadyczne strzały momentu, “szarpnięcia”.
Diagnoza na Linux: cyclictest/rtla; na MCU: sprawdzanie deadline miss.
Naprawa: pinning, izolacja IRQ, skrócenie ścieżki danych, zmniejszenie rozmiarów ramek.
Magistrale:
CAN-FD: błędy CRC, bus-off → sprawdź terminacje i długości odgałęzień.
EtherCAT: lost frames → jakości kabli, zegary, prądy upływu ekranów.
Dobre praktyki integracyjne
Oddziel zasilanie mocy od logiki, wspólny punkt masy, prowadzenie ekranów 360° do obudowy.
Redundancja telemetrii krytycznej (np. drugi kanał temperatury).
Fail-silent MCU napędów: w razie utraty ramek przechodzą w napięciowy hamulec lub bezpieczną impedancję.
Aktualizacje OTA/field-upgrade: podwójny obraz firmware (A/B), rollback, podpisy.
Konfiguracja jako dane: kalibracje, PID, limity w plikach/param server, śledzenie wersji.
Dokumentacja czasu: budżety latencji per warstwa, SLA magistrali i zegarów.
Rekomendowana mapa technologiczna
RTOS: FreeRTOS/Zephyr na MCU napędów i czujników siły.
Linux: PREEMPT_RT na SBC (x86/ARM), ROS 2 Humble/Jazzy, CycloneDDS/FastDDS.
Magistrale: EtherCAT dla nóg/ramion (dużo osi, wymagany tight sync), CAN-FD dla dłoni, palców, peryferiów.
Synchronizacja: PTP/ETG Distributed Clocks; chrony/ptp4l.
Estymacja stanu: IMU + kinematyka + foot force sensors (EKF/UKF), kontakt feet detection.
Podsumowanie

Najniższe pętle (prąd/siła/impedancja stawów) trzymaj na RTOS na MCU blisko napędów.
Linux (z RT tuningiem) prowadzi koordynację kończyn 250–1000 Hz i integruje percepcję/planowanie.
Krytyczne są: poprawna magistrala deterministyczna, synchronizacja czasu, EMC/layout mocy, zabezpieczenia sprzętowe oraz procedury testów z pomiarem jitteru i identyfikacją dynamiki. Implementacja według powyższych punktów minimalizuje typowe problemy kończyn i elektroniki w robotach androidach.

oniżej zestawienie pokrewnych technologii i komponentów spotykanych w systemach czasu rzeczywistego, pogrupowane tematycznie.

RTOS-y (mikrokontrolery i SoC)

FreeRTOS, SafeRTOS
Zephyr RTOS
ThreadX (Azure RTOS)
embOS, µC/OS-II / µC/OS-III
VxWorks
QNX Neutrino
RTEMS
NuttX
TI-RTOS, MQX, eCos
Integrity (Green Hills)
PX4/Nuttx (lotnictwo/robotyka)
Linux czasu rzeczywistego i alternatywy na ogólne CPU

Linux PREEMPT_RT
Xenomai / Cobalt
RTAI (historycznie)
Jailhouse (partitioning/hypervisor)
ACRN, Xen z domenami RT
RT-PREEMPT + CPU isolation (SCHED_FIFO, SCHED_DEADLINE)
Hiperwizory/partitioning (mixed-criticality)

PikeOS, INTEGRITY Multivisor
QNX Hypervisor
Xen, KVM z vCPU RT
seL4 (formally verified microkernel)
Middleware i frameworki robotyczne/embedded

ROS 2 (DDS: CycloneDDS, FastDDS, Connext)
OPC UA PubSub z TSN
AUTOSAR Classic/Adaptive
DDS/RTPS (poza ROS 2)
LwM2M/CoAP (IoT z ograniczeniami czasu)
Magistrale i sieci deterministyczne

EtherCAT, PROFINET IRT, SERCOS III, POWERLINK
CAN, CAN-FD, CANopen, J1939
FlexRay (automotive)
Time-Sensitive Networking (TSN: 802.1Qbv/Qbu/Qci/Qcc)
RS-485/RS-422, SPI deterministyczne, I2C z harmonogramem
TTEthernet (lotnictwo)
Synchronizacja czasu

IEEE 1588 PTP (ptp4l, linuxptp), gPTP (802.1AS)
EtherCAT Distributed Clocks
NTP (mniej deterministyczny)
Sterowanie napędami i FOC

Biblioteki FOC: STM32 Motor Control SDK, TI C2000 MotorWare/FOC
HAL/LL producentów MCU (STM32 HAL/LL, NXP SDK, TI DriverLib)
Przetwornice/sterowniki: DRV83xx, TMC, gate driver’y izolowane
Komunikacja niskopoziomowa i protokoły lekkie

UART z DMA i time-slotted protokołami
lwIP (TCP/IP dla MCU)
nanopb/flatbuffers/cap’n proto (serializacja RT)
CANopen, UAVCAN/Cyphal (dla robotyki)
Planowanie i analizy czasu

Algorytmy: Rate Monotonic, Deadline Monotonic, Earliest Deadline First
Analiza WCRT, sporządzanie harmonogramów cyclic executive
Narzędzia: Cheddar, MAST, SymTA/S, Rapita RVS
Języki i biblioteki o właściwościach RT

C/C++ z minimalnym dynamicznym alokowaniem, lock-free, ring-buffers
Rust (no_std, RTIC, embassy, critical sections)
Ada/SPARK (systemy krytyczne), MISRA C/C++ guidelines
Bezpieczeństwo funkcjonalne i standardy

ISO 26262 (automotive), IEC 61508 (przemysł), DO-178C/DO-254 (lotnictwo), IEC 62304 (med)
Certyfikowane RTOS-y: SafeRTOS, QNX, VxWorks Cert, Integrity-178B
Diagnostyka, śledzenie i profilowanie RT

ftrace, perf, LTTng, Trace Compass
ETM/ITM/SWO (Cortex-M), Segger SystemView, Percepio Tracealyzer
cyclictest, rtla (osnoise/timerlat), hwlatdetect
CAN analyzers, EtherCAT Wireshark dissector
Sprzęt i MCU popularne w RT

ARM Cortex-M (M0+/M3/M4/M7/M33), Cortex-R
TI C2000, NXP i.MX RT, ESP32(-S3/C3)
FPGA SoC (Zynq, Cyclone V SoC) do twardych ścieżek czasowych
DSP-y i kontrolery napędów
Wzorce architektoniczne

Split: RTOS na MCU dla pętli \<1 ms, Linux RT dla koordynacji 250–1000 Hz
Double-buffering, lock-free queues, zero-copy IO
Time-triggered scheduling, harmoniczne częstotliwości tasków
Watchdogi wielowarstwowe, fail-silent, circuit breakers mocy
Testy HIL/SIL i symulacja

SIL/HIL: dSPACE, OPAL-RT, Speedgoat
Symulatory: Gazebo/Ignition, MuJoCo, Webots (dla robotyki)
Testy deterministyczne z fault injection i jitter budgets
Jeśli podasz konkretny sektor (robotyka, automotive, medyczny, lotniczy) lub klasę sprzętu (MCU, SBC, SoC z FPGA), przygotuję skondensowaną listę “best choices” z uwagami wdrożeniowymi.
