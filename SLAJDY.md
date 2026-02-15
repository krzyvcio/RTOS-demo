# SLAJDY DO WYKŁADU RTOS

Format: Jeden slajd = jeden nagłówek z treścią (max 5-6 punktów)

______________________________________________________________________

# Tytuł wykładu: Projektowanie systemów RTOS dla robotyki

- Deterministyczność, architektura i walka z chaosem współbieżności
- Jak projektować systemy, żeby robotyka nie umierała w losowych momentach
- Mniej teorii, więcej decyzji projektowych i pułapek
- Dla projektantów przyszłych systemów RTOS

______________________________________________________________________

# Wymagania robotyki wobec RTOS

- Gwarantowane czasy reakcji (nie "żeby działało")
- Ograniczony jitter sterowania
- Przewidywalność w najgorszym przypadku (WCET, WCRT)
- Odporność na przeciążenia
- Degradacja kontrolowana, nie losowa

______________________________________________________________________

# Przykładowe częstotliwości w robotyce

- Pętla sterowania: 1 kHz
- Fuzja sensorów: 200 Hz
- Planowanie ruchu: 10 Hz
- Logowanie / UI: "jak się da"
- Już tu widać: priorytety, izolacja i komunikacja są krytyczne

______________________________________________________________________

# Model wykonania: taski, wątki, ISR

- ISR: minimum pracy (tylko pobierz dane, zasygnalizuj)
- Taski: logika biznesowa i przetwarzanie
- Kolejki/eventy: komunikacja między komponentami
- Brak "globalnych zmiennych do wszystkiego"
- Antywzorzec: "jeden task robi wszystko + mutexy wszędzie"

______________________________________________________________________

# Scheduler jako element architektury systemu

- Scheduler to nie detal implementacyjny - to kontrakt czasowy
- Wybory: Fixed priority preemptive? EDF? Time slicing?
- Rate Monotonic Scheduling (RMS)
- Deadline Monotonic
- Analiza wykonalności (schedulability analysis)

______________________________________________________________________

# Współdzielone zasoby: minimalizować mutexy

- Najlepszy mutex to ten, którego nie potrzebujesz
- Partycjonowanie danych między taskami
- Własność zasobów (ownership)
- Przekazywanie danych przez kolejki zamiast shared memory
- Copy vs zero-copy (trade-off latency vs safety)

______________________________________________________________________

# Mutexy inżyniersko: protokoły i gwarancje

- Priority Inheritance Protocol
- Priority Ceiling Protocol
- Unikanie nieograniczonego blokowania
- Czas trzymania mutexa jako parametr krytyczny
- Pytanie: jaki jest maksymalny czas blokady?

______________________________________________________________________

# Trylogia śmierci systemu: deadlock, livelock, starvation

- Deadlock - wszyscy czekają na siebie nawzajem
- Livelock - wszyscy pracują, ale nikt nie robi postępu
- Starvation - ktoś nigdy nie dostaje CPU
- Zapobieganie: hierarchie zasobów, time-outy, watchdog
- Projekt: "no dynamic allocation in RT path"

______________________________________________________________________

# Kolejki, eventy, pipeline danych

- Sensory → filtr → estymator → regulator → aktuatory
- Każdy etap jako osobny task
- Komunikacja: kolejki, ring buffer, lock-free FIFO
- Decyzja: co robić gdy kolejka pełna? (drop oldest/newest, blokuj)
- Decyzja: co robić gdy konsument nie nadąża?

______________________________________________________________________

# Determinizm vs przepustowość

- RTOS: gwarancje czasowe > maksymalna wydajność
- Linux + PREEMPT_RT: lepsza infrastruktura, gorsze WCET
- Pytanie projektowe: czy robot ma nie spóźnić się nigdy, czy działać szybko średnio?
- Świadome kompromisy, nie "magiczne rozwiązania"

______________________________________________________________________

# Linux jako element architektury robotycznej

- MCU + RTOS: pętle sterowania, safety
- SoC z Linux: percepcja, planowanie, UI, sieć
- Komunikacja: DDS, ROS2, shared memory, SPI, Ethernet
- Linux nie zastępuje RTOS - on go uzupełnia
- Model hybrydowy to standard w nowoczesnej robotyce

______________________________________________________________________

# Case study: architektura robota mobilnego

- Task Motor Control: 1 kHz, najwyższy priorytet
- Task IMU Fusion: 500 Hz
- Task Localization: 50 Hz
- Task Path Planning: 5 Hz
- Task Logging/Telemetry: low priority

______________________________________________________________________

# Pytania do sali o robota mobilnego

- Gdzie może powstać jitter?
- Który task może zostać opóźniony i dlaczego?
- Co się stanie przy przeciążeniu CPU?
- Jakie kolejki mogą się przepełnić?

______________________________________________________________________

# Podsumowanie dla projektantów

- RTOS to narzędzie do kontroli czasu
- Architektura > implementacja
- Synchronizacja to ryzyko, nie wygoda
- Kolejki i ownership wygrywają z mutexami
- Dobry system RTOS jest nudny, przewidywalny i odporny

______________________________________________________________________

# Memory management w RTOS

- Brak dynamicznej alokacji w ścieżce RT (static allocation)
- Memory pool + pre-alokacja
- Stack per task - kontrola rozmiaru
- Fragmentacja = wróg determinizmu
- Malloca() w pętli sterowania = proszenie się o kłopoty

______________________________________________________________________

# Priority inversion: problem i rozwiązanie

- Zadanie HP blokowane przez LP trzymające mutex
- MP "wchodzi pomiędzy" i przedłuża blokadę
- Rozwiązanie: Priority Inheritance / Priority Ceiling
- Mars Pathfinder 1997 - klasyczny przykład
- Nie używaj mutexów bez protokołów chroniących

______________________________________________________________________

# Watchdog i recovery

- Hard watchdog: reset procesora
- Soft watchdog: flaga, task sprawdza
- Supervision task: monitoruje inne taski
- Recovery: restart, degrade, safe state
- Projektuj system tak, żeby watchdog był ostatnią deską ratunku

______________________________________________________________________

# Time-triggered vs event-triggered

- Time-triggered: wszystko na zegarze, przewidywalne
- Event-triggered: reakcja na zdarzenia, trudniejsze do analizy
- TTA (Time-Triggered Architecture) w systemach safety
- Static schedule vs dynamic priority
- HETEROGENIAZCJA: różne podejścia dla różnych części systemu

______________________________________________________________________

# Mixed-criticality systems

- Systemy o różnych poziomach krytyczności na jednym CPU
- Izolacja czasowa i pamięciowa
- Partycjonowanie (ARINC 653 style)
- Przykłady: automotive (ASIL levels), lotnictwo (DAL levels)
- Przyszłość: microkernel + partycje RT + Linux

______________________________________________________________________

# Dlaczego RTOS nie zniknie

- Są systemy gdzie deadline miss = awaria
- Lotnictwo, kosmos, automotive, medycyna, przemysł
- Nie da się ich przenieść na best-effort OS
- RTOS ewoluuje: microkernel, partycjonowanie, formalna weryfikacja
- Następca RTOS to architektura, w której RTOS jest elementem

______________________________________________________________________

# Przyszłość: seL4 i formalnie weryfikowane kernele

- seL4: formalny dowód poprawności kernela
- Zero bugów race/use-after-free w kernelu
- Izolowane komponenty zamiast monolitu
- RTOS jako usługa w architekturze mikrojądra
- Mniej ręcznego dłubania, więcej modelowania czasu i zasobów

______________________________________________________________________

# Pytania do studentów na koniec

- Dlaczego mutexy są "ryzykiem, a nie wygodą"?
- Jak zaprojektować system bez dynamicznych alokacji w RT?
- Co się dzieje, gdy task nie nadąża?
- Jakie są 3 najczęstsze błędy w projektowaniu RTOS?

______________________________________________________________________

# Literatura i zasoby

- "Real-Time Systems" - Jane Liu
- FreeRTOS documentation
- Zephyr Project documentation
- seL4 whitepapers
- AUTOSAR documentation
- Mars Pathfinder case study

______________________________________________________________________

# Dziękuję za uwagę

- Pytania?
- Dyskusja o Waszych projektach?
- Kto planuje użyć RTOS w pracy dyplomowej?

______________________________________________________________________
