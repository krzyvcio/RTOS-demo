# Zajawka

Scheduler z priorytetami — kilka tasków walczących o CPU
Synchronizację — mutexy, semafory, kolejki
Porównanie RTOS vs Linux — deadline determinizm vs best-effort

## Zadania projektowe w Rust dotyczące RTOS i systemów czasu rzeczywistego

1. Minimalny RTOS w Rust z priorytetowym schedulerem
Cel: Zaimplementować prosty RTOS z:
• schedulerem z priorytetami (np. fixed‑priority preemptive),
• kilkoma taskami walczącymi o CPU,
• prostym mechanizmem przełączania kontekstu.
Wymagania:
• Taski reprezentowane jako funkcje lub obiekty z własnym stosem.
• Priorytety: np. 0–3.
• Preempcja na podstawie timera (np. SysTick w symulacji).
• Kolejka ready‑queue (np. tablica list dla każdego priorytetu).
Efekt: Student rozumie, jak działa scheduler i dlaczego priorytety są kluczowe w RTOS.
---
2. Implementacja mutexów i semaforów binarnych
Cel: Zaimplementować podstawowe prymitywy synchronizacji.
Wymagania:
• Mutex z priorytetowym dziedziczeniem (priority inheritance).
• Semafor binarny z możliwością blokowania tasków.
• Test: dwa taski o różnych priorytetach walczą o zasób → obserwacja problemu odwrócenia priorytetów.
Efekt: Student widzi, jak Rust (ownership + borrow checker) pomaga w bezpieczeństwie współbieżności.
---
3. Kolejki komunikatów (message queues)
Cel: Zaimplementować bezpieczną kolejkę FIFO do komunikacji między taskami.
Wymagania:
• Blokujące i nieblokujące operacje send/receive.
• Możliwość timeoutu.
• Test: producent–konsument z różnymi priorytetami.
Efekt: Student rozumie, jak w RTOS realizuje się IPC bez systemu plików i procesów.
---
4. Symulacja systemu czasu rzeczywistego: sterowanie robotem
Cel: Zbudować aplikację RTOS z kilkoma taskami o różnych priorytetach.
Przykładowe taski:
• Task odczytu czujników (wysoki priorytet).
• Task sterowania silnikiem (średni priorytet).
• Task komunikacji UART (niski priorytet).
• Task logowania danych (najniższy priorytet).
Synchronizacja:
• Semafor do sygnalizacji nowych danych z czujników.
• Mutex do dostępu do struktury stanu robota.
• Kolejka do przesyłania logów.
Efekt: Student widzi, jak priorytety wpływają na deterministyczność.
---
5. Porównanie RTOS vs Linux – eksperyment praktyczny
Cel: Zmierzyć różnice między RTOS a Linuxem w kontekście deterministyczności.
Wymagania:
• Napisać ten sam program w Rust dla:
	◦ RTOS (np. własny lub Embassy/RTIC),
	◦ Linux (np. tokio async lub std::thread).
• Program: task generujący sygnał co 1 ms (timer periodic).
• Zmierzyć jitter (odchylenie od idealnego okresu).
Wyniki:
• W RTOS jitter rzędu mikrosekund.
• W Linux jitter rzędu milisekund (best‑effort scheduling).
Efekt: Student rozumie różnicę między deadline determinism a best‑effort.
---
6. Implementacja protokołu komunikacyjnego w RTOS
Cel: Zaimplementować prosty protokół (np. ramki z CRC) w systemie RTOS.
Wymagania:
• Task odbierający dane z UART.
• Task analizujący ramki.
• Kolejka do przekazywania pakietów.
• Mutex do współdzielenia bufora.
Efekt: Realistyczny przykład aplikacji embedded.
---
7. RTOS z dynamiczną alokacją tasków
Cel: Dodać możliwość tworzenia i usuwania tasków w czasie działania.
Wymagania:
• Zarządzanie pamięcią (np. slab allocator).
• Lista tasków dynamiczna.
• Test: task tworzy inne taski i czeka na ich zakończenie.
Efekt: Student rozumie koszty dynamicznej alokacji w systemach czasu rzeczywistego.
---
8. Symulacja problemów klasycznych (producer–consumer, dining philosophers)
Cel: Pokazać, jak RTOS radzi sobie z klasycznymi problemami synchronizacji.
Wymagania:
• Implementacja w Rust na RTOS.
• Wersje z mutexami, semaforami i kolejkami.
• Analiza: które rozwiązanie jest najbardziej deterministyczne.
Efekt: Połączenie teorii z praktyką.

Link: https://pl.hwlibre.com/RTOS/
