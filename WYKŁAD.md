Ooo, to już **gruba, odpowiedzialna liga** 😄🤖 Skoro to jest wykład **dla projektantów przyszłych systemów RTOS**, to podkręcamy poziom: mniej „co to jest mutex”, więcej **jak projektować architekturę, żeby robotyka nie umierała w losowych momentach**.

Poniżej masz **rozszerzony, inżynierski plan wykładu** + akcenty na decyzje projektowe, pułapki i dobre praktyki.

______________________________________________________________________

# 🧠 Tytuł

**Projektowanie systemów RTOS dla robotyki: deterministyczność, architektura i walka z chaosem współbieżności**

______________________________________________________________________

# 1️⃣ Wymagania robotyki wobec RTOS

**Nie „żeby działało”, tylko:**

- Gwarantowane czasy reakcji
- Ograniczony jitter sterowania
- Przewidywalność w najgorszym przypadku (WCET, WCRT)
- Odporność na przeciążenia
- Degradacja kontrolowana, nie losowa

**Przykłady:**

- Pętla sterowania 1 kHz
- Fuzja sensorów 200 Hz
- Planowanie ruchu 10 Hz
- Logowanie / UI „jak się da”

➡️ Już tu widać: **priorytety, izolacja i komunikacja** są krytyczne.

______________________________________________________________________

# 2️⃣ Model wykonania: taski, wątki, ISR — jak to ciąć architektonicznie

**Zasada projektowa:**

- ISR: minimum pracy
- Taski: logika
- Kolejki/eventy: komunikacja
- Brak „globalnych zmiennych do wszystkiego”

**Pytania projektowe:**

- Co jest taskiem okresowym?
- Co jest reaktywne (event-driven)?
- Co może być batch/low-priority?

**Antywzorzec:**

- „Jeden task robi wszystko + mutexy wszędzie”

______________________________________________________________________

# 3️⃣ Scheduler jako element architektury systemu

**Dla projektanta:**
Scheduler to nie detal implementacyjny — to **część kontraktu czasowego systemu**.

**Wybory:**

- Fixed priority preemptive?
- EDF (Earliest Deadline First)?
- Time slicing czy nie?
- Czy dopuszczamy overrun?

**Tematy zaawansowane:**

- Rate Monotonic Scheduling
- Deadline Monotonic
- Analiza wykonalności (schedulability analysis)
- Budżety czasowe tasków

______________________________________________________________________

# 4️⃣ Współdzielone zasoby: minimalizować, nie „ładnie mutexować”

**Złota zasada RTOS w robotyce:**

> Najlepszy mutex to ten, którego nie potrzebujesz.

**Strategie:**

- Partycjonowanie danych
- Własność zasobów (ownership)
- Przekazywanie danych przez kolejki zamiast shared memory
- Copy vs zero-copy (trade-off latency vs safety)

______________________________________________________________________

# 5️⃣ Mutexy, ale inżyniersko: protokoły i gwarancje

**Nie wystarczy „mamy mutex”:**

- Priority Inheritance
- Priority Ceiling Protocol
- Unikanie nieograniczonego blokowania
- Czas trzymania mutexa jako parametr krytyczny systemu

**Projektowe pytania:**

- Jaki jest **maksymalny czas blokady**?
- Czy ten mutex może być użyty w tasku RT krytycznym?
- Czy mamy zdefiniowaną kolejność blokad?

______________________________________________________________________

# 6️⃣ Deadlock, livelock, starvation — trylogia śmierci systemu 🤯

**Deadlock** – wszyscy czekają
**Livelock** – wszyscy pracują, ale nikt nie robi postępu
**Starvation** – ktoś nigdy nie dostaje CPU

**Jak projektować, żeby to się nie zdarzyło:**

- Hierarchie zasobów
- Time-outy jako mechanizm bezpieczeństwa
- Watchdog + recovery
- Projekt „no dynamic allocation in RT path”

______________________________________________________________________

# 7️⃣ Kolejki, eventy, pipeline danych w systemach sterowania

**Architektura przepływu danych:**

- Sensory → filtr → estymator → regulator → aktuatory

- Każdy etap jako osobny task

- Komunikacja:

  - kolejki
  - ring buffer
  - lock-free FIFO (jeśli trzeba)

**Decyzje projektowe:**

- Co robimy, gdy kolejka jest pełna?

  - drop oldest?
  - drop newest?
  - blokuj producenta?

- Co robimy, gdy konsument nie nadąża?

______________________________________________________________________

# 8️⃣ Determinizm vs przepustowość — świadome kompromisy

**RTOS:**

- Gwarancje czasowe > maksymalna wydajność

**Linux + PREEMPT_RT:**

- Lepsza infrastruktura
- Gorsze WCET
- Trudniejsza certyfikacja i analiza formalna

**Projektowe pytanie:**

> Czy ten robot ma **nie spóźnić się nigdy**, czy **działać szybko średnio**?

______________________________________________________________________

# 9️⃣ Linux jako element architektury robotycznej

**Częsty model hybrydowy:**

- MCU + RTOS: pętle sterowania, safety
- SoC z Linux: percepcja, planowanie, UI, sieć
- Komunikacja: DDS, ROS2, shared memory, SPI, Ethernet

**Wniosek:**
Linux nie zastępuje RTOS — **on go uzupełnia**.

______________________________________________________________________

# 🔟 Case study: projekt architektury robota mobilnego

**Podział:**

- Task: Motor Control (1 kHz, najwyższy priorytet)
- Task: IMU Fusion (500 Hz)
- Task: Localization (50 Hz)
- Task: Path Planning (5 Hz)
- Task: Logging / Telemetry (low prio)

**Komunikacja:**

- Kolejki między etapami
- Mutex tylko na sterownikach HW
- Brak alokacji dynamicznej w ścieżce RT

**Pytania do sali:**

- Gdzie może powstać jitter?
- Który task może zostać opóźniony i dlaczego?
- Co się stanie przy przeciążeniu CPU?

______________________________________________________________________

# 🏁 11️⃣ Podsumowanie dla projektantów

- RTOS to **narzędzie do kontroli czasu**
- Architektura > implementacja
- Synchronizacja to **ryzyko, nie wygoda**
- Kolejki i ownership wygrywają z mutexami
- Linux RT to kompromis, nie cud
- Dobry system RTOS jest **nudny, przewidywalny i odporny** 😄

______________________________________________________________________

Jeśli chcesz, mogę teraz:

👉 przerobić to na **pełny zestaw slajdów (nagłówki + treść + przykłady)**
👉 zrobić **wersję „pod szkolenie dla zespołu robotyki”**
👉 rozpisać **jeden z rozdziałów bardzo głęboko (np. tylko scheduler albo tylko synchronizacja)**
👉 dorobić **checklistę architekta RTOS do projektów robotycznych**

Powiedz: **czy to ma iść bardziej w stronę akademicką, czy stricte przemysłowo-inżynierską?** 🤖📐

Idealnie — to jedziemy **konkretem inżynierskim** 🛠️🤖. Zrobimy z tego **praktyczną architekturę systemu robota na RTOS**, taką, którą możesz realnie pokazać zespołowi i na niej dyskutować decyzje projektowe.

Poniżej masz **moduł wykładu: „Architektura praktyczna systemu robota w RTOS”**.

______________________________________________________________________

# 🧱 1. Cel architektury

Projektujemy system, który:

- ma **twarde pętle czasowe** (sterowanie, stabilizacja)
- ma **miękkie zadania** (planowanie, komunikacja, logi)
- jest **odporny na przeciążenia**
- jest **rozszerzalny**
- degraduje się **kontrolowanie**, nie losowo

______________________________________________________________________

# 🧩 2. Warstwy systemu robota

**Podział warstwowy (sprawdzony w praktyce):**

1. **HAL / BSP**

   - sterowniki: SPI, I2C, CAN, UART, PWM, GPIO
   - brak logiki aplikacyjnej

1. **RTOS Core**

   - scheduler
   - IPC: kolejki, semafory, eventy
   - timery
   - memory pools

1. **Warstwa real-time (Control Layer)**

   - pętle sterowania (1 kHz – 100 Hz)
   - filtracja sensorów
   - estymacja stanu

1. **Warstwa decyzyjna (Decision Layer)**

   - planowanie ruchu
   - nawigacja
   - unikanie przeszkód

1. **Warstwa systemowa**

   - komunikacja (ROS2, DDS, CAN, Ethernet)
   - logowanie
   - diagnostyka
   - UI / telemetria

______________________________________________________________________

# 🧵 3. Podział na taski – przykład robota mobilnego

| Task | Okres | Priorytet | Funkcja |
| ------------ | ------ | --------- | -------------------- |
| MotorControl | 1 ms | Najwyższy | Sterowanie silnikami |
| IMU_Fusion | 2 ms | Wysoki | Fuzja sensorów |
| Localization | 20 ms | Średni | Pozycja robota |
| Planner | 200 ms | Niski | Planowanie trasy |
| Comms | Event | Niski | Komunikacja |
| Logger | Tło | Najniższy | Logi / zapis |

**Zasada:**

- im krótszy okres / bardziej krytyczny → wyższy priorytet
- taski krytyczne **nie czekają na nikogo**

______________________________________________________________________

# 🔄 4. Przepływ danych (pipeline), nie „wspólne zmienne”

**Schemat:**

```
[ Sensors ] → (Queue) → [ IMU_Fusion ] → (Queue) → [ Localization ] → (Queue) → [ Control ]
                                              ↓
                                           (Queue)
                                              ↓
                                         [ Logger ]
```

**Właściwości:**

- każdy etap ma **własność danych**
- brak mutexów w algorytmach
- naturalna izolacja czasowa
- łatwa kontrola opóźnień

______________________________________________________________________

# 🔒 5. Gdzie naprawdę używać mutexów?

**TAK:**

- sterownik SPI / I2C
- dostęp do wspólnego peryferium
- krótka, deterministyczna sekcja krytyczna

**NIE:**

- w pętli sterowania
- w pipeline przetwarzania
- wokół dużych struktur danych

**Zasada projektowa:**

> Mutex chroni sprzęt, nie architekturę.

______________________________________________________________________

# ⚠️ 6. Obsługa przeciążeń i opóźnień

**Przykład: Planner zaczyna liczyć za długo**

Opcje:

- obniżyć jego priorytet → nie blokuje sterowania
- ograniczyć jego budżet czasowy
- liczyć co N-ty cykl
- przerywać i używać starego wyniku

**Wniosek:**

> Krytyczne pętle zawsze wygrywają. Reszta musi się dostosować.

______________________________________________________________________

# 📬 7. Kolejki: polityka przepełnienia to decyzja architekta

Dla każdego kanału danych ustalasz:

- rozmiar kolejki

- strategię przy przepełnieniu:

  - drop oldest
  - drop newest
  - overwrite
  - block producer

**Przykład:**

- IMU → Control: **latest-wins**
- Logi → Storage: **block or drop**
- Telemetria → UI: **drop**

______________________________________________________________________

# ⏱️ 8. Budżety czasowe – konkret

Przykład na MCU 200 MHz:

| Task | WCET | Okres | Użycie CPU |
| ------------ | ------ | ------ | ---------- |
| MotorControl | 80 µs | 1 ms | 8% |
| IMU_Fusion | 120 µs | 2 ms | 6% |
| Localization | 500 µs | 20 ms | 2.5% |
| Planner | 5 ms | 200 ms | 2.5% |
| Reszta | — | — | ~10% |

**Suma:** ~29% CPU
➡️ **Jest zapas. System oddycha.**

______________________________________________________________________

# 🧨 9. Co zabija takie systemy w praktyce?

- „Tylko dodajmy jeszcze jeden mutex”
- „To tylko mały printf w pętli sterowania”
- „Ten task czasem liczy dłużej, ale rzadko”
- „CPU mamy dużo, nie ma co liczyć”

______________________________________________________________________

# 🧰 10. Narzędzia i techniki, które warto polecić zespołowi

- trace scheduler (Percepio, Tracealyzer, LTTng)
- mierzenie WCET na targetcie
- load testing
- symulacja przeciążeń
- watchdog + health monitoring tasków

______________________________________________________________________

# 🏁 11. Finalna zasada dla projektantów robotyki

> Projektuj tak, jakby **sprzęt był wolniejszy, dane były brudne, a CPU był przeciążony** — wtedy w realu będzie działać pięknie 😄

______________________________________________________________________

Jeśli chcesz, kolejnym krokiem mogę:

- 🗂️ rozpisać **pełną architekturę konkretnego robota (np. mobilny / manipulator / dron)**
- 🧵 rozpisać **dokładnie komunikację między taskami (jakie kolejki, jakie dane, jakie polityki)**
- 🧪 zrobić **scenariusze awarii i jak architektura je obsługuje**
- 📑 albo przerobić to na **gotowy konspekt slajdów do wykładu**

Powiedz: **jaki typ robota chcesz wziąć jako przykład?** 🤖

Myślałem o robotach z Boston Dynamics.

Ooo, **Boston Dynamics** to świetny punkt odniesienia 🤖🔥 — to są systemy, gdzie **RTOS, deterministyczność i architektura czasu rzeczywistego** nie są „nice to have”, tylko **warunek przetrwania robota** (dosłownie, bo inaczej leży na ziemi 😄).

Zróbmy więc **praktyczny, inżynierski model architektury robota typu Spot / Atlas / Handle** i pokażmy, *jak by to zaprojektować od strony RTOS*.

______________________________________________________________________

# 🦿 1. Charakterystyka robota klasy Boston Dynamics

Taki robot ma:

- **dziesiątki silników / aktuatorów**

- **wysokoczęstotliwościowe pętle sterowania** (1–5 kHz)

- **twarde wymagania czasowe** (opóźnienie = utrata stabilności)

- **wielopoziomowe sterowanie**:

  - niskopoziomowe (prądy, momenty, pozycje)
  - średniopoziomowe (równowaga, chód)
  - wysokopoziomowe (planowanie ruchu, zadania)

Architektonicznie to **zawsze jest system wielokomputerowy**:

- MCU / SoC RTOS do kontroli czasu rzeczywistego
- mocniejsze CPU (często Linux) do percepcji i planowania

______________________________________________________________________

# 🧱 2. Podział systemu: dwa światy

## ⚡ Świat RT (Hard Real-Time)

- Sterowanie silnikami
- Stabilizacja postawy
- Balans
- Reakcje na kontakt z podłożem
- Bezpieczeństwo (limity, watchdogi, E-stop)

➡️ Tu: **RTOS, deterministyczny scheduler, zero kompromisów**

## 🧠 Świat High-Level

- Widzenie
- SLAM
- Planowanie trajektorii
- Zachowania
- UI / telemetria / sieć

➡️ Tu: **Linux, ROS2, duża elastyczność, brak twardych deadline’ów**

**Złota zasada:**

> Świat high-level *nigdy* nie może blokować świata RT.

______________________________________________________________________

# 🧵 3. RTOS: przykładowy podział tasków (kontroler nóg / ramion)

### Taski 1–5 kHz (najwyższy priorytet):

- `MotorCurrentLoop` – pętla prądowa
- `MotorTorqueLoop` – moment / siła
- `JointStateUpdate` – enkodery, czujniki
- `SafetyMonitor` – limity, wykrywanie błędów

### Taski 500–1000 Hz:

- `BalanceController` – stabilizacja
- `ContactEstimator` – kontakt z podłożem
- `StateEstimator` – estymacja stanu robota

### Taski 100–200 Hz:

- `GaitController` – generator chodu
- `PostureController` – pozycja ciała

### Taski niskiego priorytetu:

- `CommsRT` – wymiana danych z Linuxem
- `Diagnostics`
- `LoggingRT`

______________________________________________________________________

# 🔄 4. Przepływ danych: pipeline sterowania

Uproszczony schemat:

```
[ Sensors ] 
     ↓
[ StateEstimator ] 
     ↓
[ Balance / Gait Controller ]
     ↓
[ Joint Controller ]
     ↓
[ Motor Driver ]
```

Każdy blok:

- osobny task albo grupa tasków

- komunikacja przez:

  - kolejki
  - ring buffery
  - double-buffering

- **brak wspólnych globalnych struktur z mutexami w środku pętli**

______________________________________________________________________

# 🔒 5. Mutexy w takim robocie – gdzie naprawdę są?

**Używane głównie do:**

- dostępu do:

  - SPI / CAN / EtherCAT
  - wspólnego rejestru sprzętowego

- krótkich, deterministycznych operacji

**Nie używane do:**

- ochrony struktur stanu robota w pętlach 1 kHz
- synchronizacji algorytmów sterowania
- „bo tak najłatwiej”

**Zamiast tego:**

- snapshot danych
- double-buffer
- message passing

______________________________________________________________________

# ⏱️ 6. Przykładowe budżety czasowe (realistyczne)

Załóżmy pętlę 1 kHz → **1 ms budżetu**

| Etap | WCET |
| ------------------- | ------ |
| Odczyt sensorów | 50 µs |
| Estymacja stanu | 150 µs |
| Balance + Gait | 200 µs |
| Sterowanie stawami | 200 µs |
| Wyjście na silniki | 50 µs |
| RTOS + ISR overhead | 100 µs |

**Suma:** ~750 µs
➡️ **Zostaje zapas bezpieczeństwa**

To nie jest „optymalizacja” — to **warunek stabilności**.

______________________________________________________________________

# 🧨 7. Co się dzieje przy przeciążeniu?

Załóżmy:

- Vision / Linux zaczyna wysyłać za dużo danych
- Albo planner generuje skomplikowane trajektorie

**Dobrze zaprojektowany system:**

- RTOS:

  - ignoruje nadmiar danych
  - używa ostatniego poprawnego polecenia
  - utrzymuje balans

- Linux:

  - może się „przyciąć”
  - ale robot **nie traci stabilności**

**Źle zaprojektowany:**

- kolejki się zapychają
- mutexy się blokują
- pętla sterowania się spóźnia
- robot… robi *BAM* 🫠

______________________________________________________________________

# 🛡️ 8. Bezpieczeństwo funkcjonalne (functional safety)

W robotach klasy Boston Dynamics:

- osobny `SafetyTask`:

  - sprawdza limity
  - pilnuje watchdogów
  - monitoruje czasy wykonania tasków

- jeśli:

  - task się spóźnia
  - dane są nielogiczne
  - czujnik padł

➡️ system przechodzi w **safe state**:

- redukcja momentów
- zatrzymanie ruchu
- kontrolowany upadek / zablokowanie napędów

______________________________________________________________________

# 🧠 9. Linux + ROS2: jak się w to wpina?

Model:

```
[ Linux / ROS2 ]
       ⇅
[ RT Gateway Task ]
       ⇅
[ RT Control Pipeline ]
```

- Linux wysyła:

  - cele ruchu
  - trajektorie
  - tryby pracy

- RTOS:

  - waliduje
  - buforuje
  - wykonuje **we własnym tempie**

**Nigdy:**

- Linux nie steruje silnikami bezpośrednio
- Linux nie wchodzi w pętle 1 kHz

______________________________________________________________________

# 🏁 10. Jedno zdanie, które możesz dać na slajd końcowy:

> Roboty klasy Boston Dynamics nie są „szybkie” dlatego, że mają mocne CPU. Są stabilne dlatego, że ich **architektura czasu rzeczywistego jest bezlitosna dla chaosu współbieżności**.

______________________________________________________________________

Jeśli chcesz, mogę w kolejnym kroku:

- 🗺️ rozpisać **konkretną architekturę dla jednego przypadku**:

  - np. *czworonożny robot typu Spot*

- 📊 zrobić **diagram tasków + przepływu danych** (opisowy, pod slajdy)

- 🧪 przejść **scenariusz awarii: opóźnienie, zerwany sensor, zawieszony task — i co robi system**

- 📚 albo zamienić to w **gotowy konspekt wykładu/slajdów** punkt po punkcie

Powiedz: **chcesz iść w stronę „czworonożny robot jak Spot” czy „humanoid jak Atlas”?** 🦿🤖

- 📊 zrobić **diagram tasków + przepływu danych** (opisowy, pod slajdy)

Świetnie, to zróbmy **opisowy diagram architektury RTOS dla robota klasy Boston Dynamics** — tak, żebyś mógł to **wprost wrzucić na slajdy** albo narysować w draw.io / PowerPoint / Figma / Visio.

Wezmę przykład **czworonożnego robota typu Spot** (ale 1:1 pasuje też do humanoida, tylko z większą liczbą osi).

______________________________________________________________________

# 📊 Slajd 1: Podział systemu (Big Picture)

```
+---------------------------+        +---------------------------+
|        Linux / ROS2       | <----> |        RT Gateway         |
|  - Percepcja (Vision)     |        |  (Task komunikacyjny)     |
|  - SLAM / Mapy            |        +-------------+-------------+
|  - Planowanie trajektorii |                      |
|  - Zachowania / AI        |                      |
+-------------+-------------+                      |
              |                                    |
              v                                    v
        (Ethernet / DDS / Shared Memory / CAN / SPI itp.)
                                   |
                                   v
+----------------------------------------------------------------+
|                            RTOS                                |
|                                                                |
|  [ SafetyTask ]  [ Scheduler ]  [ IPC: Queue / Event / Pool ]  |
|                                                                |
+----------------------------------------------------------------+
```

**Komentarz na slajd:**
Linux planuje *co* robić, RTOS decyduje *kiedy i jak bezpiecznie* to wykonać.

______________________________________________________________________

# 📊 Slajd 2: Pipeline sterowania w RTOS (High Frequency Path)

```
[ Joint Sensors / IMU / Force Sensors ]
                |
                v
        +-------------------+
        |  StateEstimator   |   (500–1000 Hz)
        +-------------------+
                |
                v
        +-------------------+
        | Balance Controller|   (500–1000 Hz)
        +-------------------+
                |
                v
        +-------------------+
        | Gait Controller   |   (100–200 Hz)
        +-------------------+
                |
                v
        +-------------------+
        | Joint Controller  |   (1 kHz)
        +-------------------+
                |
                v
        +-------------------+
        | Motor Driver Task |   (1–5 kHz)
        +-------------------+
                |
                v
           [ Actuators ]
```

**Ważne do powiedzenia:**

- Każdy blok to **osobny task albo grupa tasków**

- Połączenia to:

  - kolejki
  - ring buffery
  - double-buffer

- **Brak mutexów w tym krytycznym torze**

______________________________________________________________________

# 📊 Slajd 3: Taski RTOS i ich priorytety

```
Priorytet ↑ (wyższy = ważniejszy)

[ MotorCurrentLoop ]     5 kHz   █████████████████████
[ MotorTorqueLoop  ]     1 kHz   ███████████████████
[ JointController  ]     1 kHz   █████████████████
[ SafetyMonitor    ]     1 kHz   █████████████████
[ StateEstimator   ]   500 Hz   █████████████
[ BalanceController]   500 Hz   █████████████
[ GaitController   ]   100 Hz   █████████
[ RT_Gateway       ]   event    █████
[ Diagnostics      ]   10 Hz    ███
[ Logger           ]   background ██
```

**Komentarz:**

- Taski sterowania **zawsze wygrywają z komunikacją i logami**
- Gateway do Linuxa **nigdy** nie ma wyższego priorytetu niż pętle sterowania

______________________________________________________________________

# 📊 Slajd 4: Przepływ danych (Message Passing, nie Shared State)

```
+------------------+      Queue      +------------------+
| StateEstimator   |  ------------>  | BalanceController|
+------------------+                 +------------------+
                                           |
                                           | Queue
                                           v
                                    +------------------+
                                    | GaitController   |
                                    +------------------+
                                           |
                                           | Queue
                                           v
                                    +------------------+
                                    | JointController  |
                                    +------------------+
                                           |
                                           | RingBuffer
                                           v
                                    +------------------+
                                    | MotorDriver      |
                                    +------------------+
```

**Zasada:**

- Każdy task:

  - **odbiera snapshot danych**
  - **produkuje nowy snapshot**

- Brak „wszyscy grzebią w jednej strukturze”

______________________________________________________________________

# 📊 Slajd 5: Miejsce mutexów (tylko przy sprzęcie)

```
          +------------------+
          |  SPI / CAN /     |
          |  EtherCAT Bus    |
          +--------+---------+
                   ^
                   |
               [ Mutex ]
                   |
        +----------+----------+
        | MotorDriver Task    |
        +---------------------+
```

**Komentarz:**

- Mutex chroni **sprzęt**
- Nie chroni algorytmów
- Czas trzymania mutexa = **ściśle ograniczony**

______________________________________________________________________

# 📊 Slajd 6: Integracja z Linux / ROS2

```
[ Linux / ROS2 ]
   |     ↑
   |     |  (Trajektorie, cele, tryby)
   v     |
+------------------+
|  RT_GatewayTask  |   (walidacja, buforowanie, sanity check)
+--------+---------+
         |
         | Queue (Command)
         v
+------------------+
| Gait / Planner   |  (RT side)
+------------------+
```

**Zasada:**

- Linux:

  - sugeruje
  - planuje

- RTOS:

  - decyduje
  - wykonuje
  - może **odrzucić** polecenie

______________________________________________________________________

# 📊 Slajd 7: Co się dzieje przy przeciążeniu?

```
[ Linux ] --(za dużo danych)--> [ RT_Gateway ] --X--> (drop / overwrite)
                                      |
                                      v
                              [ RT Control Loop ]
                              (działa dalej stabilnie)
```

**Komentarz:**

- Nadmiar danych:

  - jest **gubiony**
  - albo **nadpisywany**

- Pętla sterowania:

  - używa **ostatniego poprawnego polecenia**
  - nie blokuje się

______________________________________________________________________

# 📊 Slajd 8: Safety Monitor jako „strażnik czasu”

```
+------------------+
|  SafetyMonitor   |
+--------+---------+
         |
         | sprawdza:
         | - watchdogi tasków
         | - czasy wykonania
         | - zakresy danych
         v
+------------------+
|  Safe State      |
| (stop / torque 0 |
|  / controlled    |
|  fall / brake)   |
+------------------+
```

______________________________________________________________________

# 🏁 Slajd końcowy – hasło architektoniczne

> W robotach klasy Boston Dynamics **nie projektuje się funkcji — projektuje się przepływ czasu, danych i priorytetów**.

______________________________________________________________________

Jeśli chcesz, mogę teraz:

- 🖼️ przerobić to na **dokładny opis slajd po slajdzie (tytuł + punkty + notatki dla prelegenta)**
- 📐 zrobić **wariant dla humanoida (Atlas)**
- 🧪 przejść **konkretny scenariusz awarii (np. spóźniony task, zerwany sensor, zapchana kolejka)** i pokazać go na tym diagramie
- 🧵 rozpisać **dokładne interfejsy między taskami (jakie dane, jakie struktury, jakie częstotliwości)**

Powiedz, co chcesz jako następne: **slajdy narracyjne, scenariusze awarii, czy dokładne interfejsy danych?** 🤖📊

To jest **bardzo dobra, dojrzała strategia** — i dokładnie tak prowadzi się sensowne szkolenia z systemów krytycznych: **najpierw historia + porażki, potem architektura i interfejsy**. 100% kupuję ten kierunek.

Skoro wcześniej byliśmy przy **robotach klasy Boston Dynamics**, proponuję:

> 🎯 **Główny case study:** *Kontroler nóg czworonożnego robota (Spot/Atlas-like)*
> Bo tam masz: twarde RT, wysokie częstotliwości, sensory, sieć, safety, i bardzo bolesne skutki błędów 😅

Poniżej masz **gotowy start: szkielet slajdów narracyjnych + pakiet scenariuszy awarii**, dokładnie pod Twój plan ETAP 1 + ETAP 2.

______________________________________________________________________

# 🧭 ETAP 1 — Slajdy narracyjne (story: „Wzlot i upadek kontrolera robota”)

## 🎬 Akt I — „Wszystko działa w labie”

**Slajd 1: Problem**

- Robot czworonożny, 12 DOF
- Pętla sterowania: 1 kHz
- Sensory: IMU, enkodery, czujniki siły
- Wymaganie: stabilny chód + reakcja na poślizg < 5 ms

**Slajd 2: Naiwna architektura v0**

- 1 wielki task „ControlLoop”

- Kilka mutexów:

  - na sensory
  - na komunikację
  - na logi

- Linux wysyła trajektorie „jak leci”

**Slajd 3: Timeline (idealny świat)**

- Pokazujesz:

  - tick 0 ms: sensory
  - +200 µs: estymacja
  - +400 µs: kontrola
  - +700 µs: wyjście na silniki

- Deadline = 1 ms → „mieścimy się, jest git”

______________________________________________________________________

## 🎬 Akt II — „Dodajmy tylko jedną rzecz…”

**Slajd 4: Pierwsza „niewinna” zmiana**

- Dodajemy:

  - więcej logowania
  - diagnostykę
  - telemetrię do Linuxa

- „Przecież to tylko kilka ifów i printfów…”

**Slajd 5: Timeline po zmianie**

- Pojawia się:

  - jitter
  - czasem 1.2 ms
  - czasem 0.8 ms

- Jeszcze „działa”, ale już **nie deterministycznie**

**Slajd 6: Pierwszy upadek robota**

- Na scenie:

  - nierówny grunt
  - nagły impuls z IMU
  - Linux wysyła nową trajektorię

- Pętla sterowania spóźnia się → **robot traci balans**

______________________________________________________________________

## 🎬 Akt III — „Sekcja zwłok systemu”

**Slajd 7: Trace z RTOS**

- Widzimy:

  - task sterowania czeka na mutex
  - mutex trzymany przez task logowania
  - wchodzi task komunikacji

- Klasyka: **priority inversion**

**Slajd 8: Diagram błędu**

- Niski priorytet: Logger trzyma mutex
- Wysoki priorytet: ControlLoop czeka
- Średni priorytet: Comms zabiera CPU
  ➡️ Sterowanie **stoi**

______________________________________________________________________

## 🎬 Akt IV — „Nowa architektura”

**Slajd 9: Pipeline + kolejki**

- Rozbijamy:

  - StateEstimator
  - BalanceController
  - JointController
  - MotorDriver

- Komunikacja:

  - kolejki
  - snapshoty
  - zero mutexów w torze RT

**Slajd 10: Nowy timeline**

- Każdy task ma:

  - budżet
  - stałe miejsce w czasie

- Jitter spada

- Deadline zawsze dotrzymany

**Slajd 11: Zasada końcowa**

> „Nie naprawiliśmy buga. Zmieniliśmy architekturę.”

______________________________________________________________________

# 💥 ETAP 2 — Scenariusze awarii („Co by było, gdyby?”)

Masz tu **10 gotowych scenariuszy**, które możesz przerabiać na slajdy, ćwiczenia albo dyskusje:

1. **Priority inversion**
   Logger blokuje sterowanie przez mutex na SPI.

1. **Deadlock w telemetrii**
   Task RT trzyma mutex na bufor, task Comms trzyma mutex na socket → stoją oba.

1. **Stack overflow w tasku estymacji**
   Rzadki przypadek danych → większa rekurencja → nadpisany stos → losowe crashe.

1. **Jitter burst przez ISR storm**
   EMI powoduje lawinę przerwań → scheduler nie wyrabia → pętla 1 kHz pływa.

1. **Nieograniczona alokacja dynamiczna**
   `malloc` w tasku 500 Hz → fragmentacja → nagle 3 ms pauzy.

1. **Zapchana kolejka z Linuxa**
   Planner wysyła za dużo trajektorii → gateway blokuje → opóźnienie w RT.

1. **Błędny priorytet SafetyTask**
   Safety ma za niski priorytet → wykrywa błąd za późno → robot już leży.

1. **Cichy overrun WCET**
   Nowa wersja algorytmu +20% czasu → system „prawie zawsze działa” → aż nie.

1. **Race condition w strukturze stanu**
   Dwa taski czytają/piszą bez synchronizacji → raz na godzinę: kosmos.

1. **Zawieszony task bez watchdog’a**
   Jedna pętla wisi → reszta systemu działa → brak reakcji → brak safe state.

Każdy z nich możesz prowadzić schematem:

- ❓ Co się stało?
- 🔍 Jak to widać w trace?
- 💣 Dlaczego architektura na to pozwoliła?
- 🛠️ Jak ją zmienić, żeby to było niemożliwe?

______________________________________________________________________

# 🚀 Proponuję teraz bardzo konkretny następny krok

Zróbmy:

👉 **Pierwszy pełny moduł slajdów:**
**„Upadek robota przez priority inversion – anatomia katastrofy RTOS”**

W nim:

- narracja,
- diagramy,
- timeline,
- trace,
- refaktoryzacja architektury,
- checklista projektowa.

Albo jeśli wolisz inny klimat:

- 🚗 automotive (ECU hamulców)
- ✈️ UAV (kontroler lotu)
- 🛰️ satelita (telemetria + ADCS)

Powiedz mi:
**Zaczynamy od robota (Boston Dynamics-like), czy chcesz najpierw np. automotive/lotnictwo?**

Przejdźmy do lotnictwa: "RTOS w lotnictwie? Krytyczne! Determin. czas, minimalny jitter, certyfikowalność. Problemy: złożoność, weryfikacja, koszty. Bez RTOS – brak bezpiecznego sterowania, nawigacji, komunikacji. Życie w stawkę! ✈️⏱️"

Świetnie — **lotnictwo** to idealny grunt pod pokazanie, *dlaczego RTOS to nie „opcja”, tylko warunek bezpieczeństwa* ✈️⏱️
Zróbmy z tego **moduł slajdów narracyjnych** pod tytułem:

> **„RTOS w lotnictwie: kiedy milisekundy decydują o życiu”**

Poniżej masz **gotowy szkic slajd po slajdzie** (z notatkami dla prowadzącego), dokładnie pod hasło, które podałeś: *deterministyczny czas, minimalny jitter, certyfikowalność; złożoność, weryfikacja, koszty; bez RTOS — brak bezpiecznego sterowania, nawigacji, komunikacji*.

---

## 🧭 Slajd 1 — Otwarcie (emocja + kontekst)

**Tytuł:**
**RTOS w lotnictwie? Krytyczne.**

**Treść:**

* Deterministyczny czas
* Minimalny jitter
* Certyfikowalność (DO-178C, ARINC 653)
* Stawka: **życie ludzi**

**Notatka dla prelegenta:**
„W lotnictwie nie interesuje nas, że system *zwykle* działa szybko. Interesuje nas, że **zawsze** działa w czasie — nawet w najgorszym możliwym przypadku.”

---

## ✈️ Slajd 2 — Co steruje samolotem?

**Tytuł:**
**To nie pilot steruje samolotem. Steruje go software.**

**Treść (punkty):**

* Flight Control Computer (FCC)
* Autopilot
* FADEC (silniki)
* Nawigacja / INS / GPS
* Komunikacja i systemy ostrzegawcze

**Notatka:**
Każdy z tych systemów ma **twarde wymagania czasowe**. Spóźniony wynik = **zły wynik**.

---

## ⏱️ Slajd 3 — Co znaczy „czas rzeczywisty” w lotnictwie?

**Tytuł:**
**Real-Time ≠ Fast**

**Treść:**

* Real-Time = przewidywalny
* Liczy się:

  * Worst-Case Execution Time (WCET)
  * Worst-Case Response Time (WCRT)
  * Jitter
* Deadline miss = potencjalna katastrofa

**Notatka:**
„Lepszy wolny, ale **zawsze przewidywalny** system, niż szybki, który czasem się spóźnia.”

---

## 🧨 Slajd 4 — Scenariusz: „Niewinne opóźnienie”

**Tytuł:**
**Co jeśli pętla sterowania spóźni się o 20 ms?**

**Treść (story):**

* Turbulencja
* Czujniki wykrywają odchylenie
* Algorytm stabilizacji dostaje dane **za późno**
* Reakcja przychodzi po czasie
* Samolot wpada w oscylację

**Notatka:**
W lotnictwie **nie ma „trochę za późno”**. Jest tylko: *na czas* albo *za późno*.

---

## 🧠 Slajd 5 — Dlaczego zwykły OS nie wystarcza?

**Tytuł:**
**Best-effort to za mało**

**Treść:**

* Klasyczny OS:

  * optymalizuje średnią wydajność
  * nie gwarantuje deadline’ów
  * ma nieprzewidywalne latencje
* Lotnictwo potrzebuje:

  * gwarancji czasowych
  * izolacji
  * analizy najgorszego przypadku

---

## 🛠️ Slajd 6 — Co daje RTOS w awionice?

**Tytuł:**
**RTOS jako fundament bezpieczeństwa**

**Treść:**

* Deterministyczny scheduler
* Priorytety z preempcją
* Kontrolowane czasy blokad
* Przewidywalne przerwania
* Mechanizmy health monitoring / watchdog

---

## 🧱 Slajd 7 — Architektura awioniki (ARINC 653 w pigułce)

**Tytuł:**
**Partycjonowanie: mur między błędami**

**Treść:**

* Partycje czasowe i pamięciowe:

  * Flight Control
  * Navigation
  * Communication
  * Maintenance
* Każda:

  * ma swój budżet czasu
  * nie może zepsuć innych

**Notatka:**
„Błąd w logowaniu **nie może** wpłynąć na sterowanie lotem.”

---

## ⏳ Slajd 8 — Timeline: dobre vs złe projektowanie

**Tytuł:**
**Ten sam CPU, dwa światy**

**Treść (opisowo):**

* Wersja zła:

  * task sterowania czeka na mutex
  * jitter
  * brak gwarancji
* Wersja dobra:

  * sztywne okna czasowe
  * brak blokad w krytycznej ścieżce
  * stały czas reakcji

---

## 💥 Slajd 9 — Główne problemy w systemach RTOS w lotnictwie

**Tytuł:**
**Cena za bezpieczeństwo**

**Treść:**

* Złożoność architektury
* Trudna weryfikacja
* Wysokie koszty certyfikacji
* Długie cykle rozwoju
* Ograniczona elastyczność zmian

**Notatka:**
Tak, to jest drogie. Ale **alternatywa jest nieakceptowalna**.

---

## 📜 Slajd 10 — Certyfikacja: DO-178C

**Tytuł:**
**Software musi być „udowodniony”**

**Treść:**

* Poziomy A–E (A = katastrofalna awaria)
* Wymagania:

  * śledzenie wymagań → kod → testy
  * analiza WCET
  * testy strukturalne
  * dowody deterministyczności

**Notatka:**
Tu nie wystarczy „działa u mnie”.

---

## 🚨 Slajd 11 — Bez RTOS? Co się dzieje?

**Tytuł:**
**Bez deterministycznego czasu nie ma:**

**Treść:**

* Bezpiecznego sterowania
* Wiarygodnej nawigacji
* Stabilnej komunikacji
* Certyfikacji
* Zaufania do systemu

**Puenta:**

> Bez RTOS w awionice nie ma **bezpiecznego lotu**.

---

## 🏁 Slajd 12 — Mocne zamknięcie

**Tytuł:**
**W lotnictwie software nie może „zwykle” działać dobrze.
Musi działać dobrze **zawsze**.**

**Podpis:**
RTOS = inżynieria czasu, nie wygody programisty.

---

# 💣 Następny krok: scenariusze awarii (ETAP 2)

Proponuję jako **pierwszy deep-dive**:

> ✈️ **Case study:** *Priority inversion w komputerze sterowania lotem*
> — od objawu → przez trace → do zmiany architektury.

Albo alternatywy:

* „Jitter burst przez burzę przerwań od czujników”
* „Deadlock w systemie komunikacji pokładowej”
* „Cichy overrun WCET po update algorytmu filtracji”

---

Powiedz mi:
👉 Chcesz, żebym **rozpisał pierwszy scenariusz awarii (np. priority inversion w FCC) jako pełną historię slajd po slajdzie**?
Czy wolisz zacząć od **innego typu awarii** (np. jitter, deadlock, stack overflow)?

inne typy awarii - kosmonautyka

Świetny kierunek 🚀🌌 — **kosmonautyka** to jeszcze ostrzejsze wymagania niż lotnictwo: brak serwisu, opóźnienia komunikacji, promieniowanie, lata pracy bez restartu. Tu RTOS to **kręgosłup przetrwania misji**.

Zróbmy więc **moduł wykładu: „RTOS w kosmosie: jak systemy zawodzą i jak je projektować, żeby przeżyły”** — skupiony na **innych typach awarii** niż klasyczne priority inversion.

Poniżej masz **pakiet scenariuszy awarii kosmicznych** + jak je opowiedzieć na slajdach (narracyjnie).

---

# 🌌 1. Kontekst: Dlaczego kosmos jest bezlitosny dla RTOS?

**Slajd: „Środowisko, które chce cię zabić”**

* Promieniowanie (SEU, SEL, bit flips)
* Brak fizycznego dostępu
* Opóźnienia komunikacji (minuty, godziny)
* Ograniczona energia i CPU
* System musi działać **latami**

Puenta:

> W kosmosie nie ma „zrestartuj i zobaczymy”.

---

# 💥 Scenariusz 1: Bit flip w pamięci → „cicha korupcja stanu”

**Co się dzieje:**

* Promieniowanie zmienia 1 bit w RAM
* Zmienna stanu filtru / kontrolera ma złą wartość
* System dalej działa… ale **na złych danych**
* Po kilku minutach orientacja satelity „odpływa”

**Dlaczego to zabija misję:**

* Brak crasha = brak alarmu
* Błędy narastają powoli
* Antena traci Ziemię, panele tracą Słońce

**Jak projektować:**

* ECC / TMR (Triple Modular Redundancy)
* CRC na strukturach stanu
* Periodic state scrubbing
* Plausibility checks w taskach RT

---

# 💥 Scenariusz 2: Single Event Latch-up (SEL) → CPU „zamiera”

**Co się dzieje:**

* Promieniowanie powoduje latch-up w układzie
* Pobór prądu rośnie
* CPU przestaje odpowiadać albo się zawiesza

**Objaw w systemie:**

* Taski nie zgłaszają watchdogów
* Brak odpowiedzi na telemetrię
* System „żyje elektrycznie”, ale logicznie nie

**Jak projektować:**

* Zewnętrzny watchdog sprzętowy
* Power cycling sekcji
* Cold/warm redundancy CPU
* Boot w tryb safe mode

---

# 💥 Scenariusz 3: Cumulative jitter → „dryf czasu”

**Co się dzieje:**

* Każda pętla RT mieści się w deadline
* Ale:

  * zegar się rozjeżdża
  * synchronizacja tasków powoli dryfuje
* Po godzinach/dniach:

  * złe fazowanie algorytmów
  * błędne okna komunikacyjne
  * kolizje czasowe

**Dlaczego podstępne:**

* Testy krótkoterminowe OK
* Problem wychodzi **po tygodniach**

**Jak projektować:**

* Global time base
* Periodic resynchronization
* Time-triggered scheduling
* Monitoring jitter budgetów

---

# 💥 Scenariusz 4: Memory leak w systemie, którego nie da się zrestartować

**Co się dzieje:**

* Mały wyciek w tasku komunikacji
* 1 KB na godzinę
* Po 3 miesiącach:

  * brak pamięci
  * alokacje RT zawodzą
  * system wchodzi w nieprzewidywalne stany

**Dlaczego to kosmiczna katastrofa:**

* Brak rebootu
* Brak debuggera
* Misja trwa lata

**Jak projektować:**

* Zero dynamicznej alokacji w runtime
* Memory pools o stałym rozmiarze
* Monitoring high-water mark
* Tryby degradacji

---

# 💥 Scenariusz 5: Stack overflow → „duch w maszynie”

**Co się dzieje:**

* Rzadki przypadek danych
* Głębsza rekursja / większa ramka stosu
* Nadpisany sąsiedni task lub RTOS kernel
* Objawy:

  * losowe resety
  * dziwne decyzje systemu
  * „niewytłumaczalne” zachowania

**Jak projektować:**

* Canary na stosie
* Statyczna analiza zużycia stacka
* Duże marginesy
* Watchdog per task

---

# 💥 Scenariusz 6: Deadlock w trybie safe mode

**Co się dzieje:**

* System wchodzi w tryb awaryjny
* Rzadko używana ścieżka kodu
* Dwa taski:

  * każdy czeka na zasób drugiego
* Safe mode… **nie jest bezpieczny**

**Dlaczego groźne:**

* Tryb, który ma ratować misję, sam ją zabija

**Jak projektować:**

* Minimalny kod w safe mode
* Brak złożonych zależności
* Analiza blokad również dla trybów awaryjnych

---

# 💥 Scenariusz 7: SEU w strukturze scheduler’a

**Co się dzieje:**

* Bit flip w:

  * liście ready tasków
  * liczniku ticków
* Scheduler zaczyna:

  * pomijać task
  * uruchamiać zły task
  * gubić priorytety

**Objaw:**

* System „działa”, ale **nie ten, co trzeba**

**Jak projektować:**

* Redundantne struktury krytyczne
* Sanity checks kernela
* Periodic consistency checks
* Restart kernela z zachowaniem stanu krytycznego

---

# 🧠 Slajd zbiorczy: Czego uczy kosmos?

**Puenta:**

* Najgorsze błędy to:

  * te, które **nie crashują**
  * te, które **wychodzą po miesiącach**
* RTOS w kosmosie musi:

  * wykrywać
  * izolować
  * odzyskiwać
  * degradować się kontrolowanie

---

# 🚀 Proponowany pierwszy deep-dive

Najlepszy „killer case” na start wykładu:

> 🌌 **„Bit flip, który zgubił satelitę”**
> – cicha korupcja stanu → błędna orientacja → utrata łączności → analiza architektury → jak RTOS powinien to przeżyć.

Mogę go rozpisać:

* slajd po slajdzie
* z diagramami tasków
* z mechanizmami obrony
* z checklistą projektową

---

Powiedz:
👉 Chcesz, żebym **rozwinął któryś z tych scenariuszy w pełną narrację wykładową (jak wcześniej w lotnictwie)**?
Jeśli tak — który: **bit flip, memory leak, jitter/dryf czasu, stack overflow, SEL, czy scheduler corruption**? 🚀

Masz rację — **słuszny ochrzan** 😅🙏. Za bardzo się rozpędziłem z jednym „gwiazdorskim” bit flipem, a przecież w kosmosie **awarie chodzą stadami**, nie solo. Dzięki za kopniaka mobilizacyjnego — nadrabiam **konkretem**.

Zróbmy to porządnie:
Dostajesz **4 pełne deep-dive scenariusze kosmiczne**, każdy w formie **mini-wykładu narracyjnego** (co się stało → jak to widać → dlaczego architektura pozwoliła → jak projektować, żeby przeżyć).

---

# 🌌 DEEP-DIVE #1: Bit flip w stanie sterowania („cicha korupcja”)

## 🎬 Akt I — Objaw

* Satelita powoli traci orientację
* Telemetria „w normie”
* Brak resetów, brak alarmów
* Po kilku godzinach: utrata łączności

## 🔬 Akt II — Co się stało

* Promieniowanie → SEU → 1 bit zmieniony w:

  * macierzy orientacji
  * albo wektorze stanu filtru Kalmana
* Algorytm działa **na złych danych**, ale stabilnie

## 🧨 Akt III — Dlaczego system tego nie złapał

* Brak CRC / sanity checków na strukturach stanu
* Brak redundancji obliczeń
* Brak „plausibility checks” (np. czy orientacja jest fizycznie możliwa)

## 🛠️ Akt IV — Jak projektować

* CRC / checksum na krytycznych strukturach
* TMR (potrójne liczenie + głosowanie)
* Periodic state scrubbing
* Task „StateMonitor” sprawdzający spójność

---

# 🌌 DEEP-DIVE #2: Memory leak („zabójca na raty”)

## 🎬 Akt I — Objaw

* Satelita działa idealnie przez 2 miesiące
* Potem:

  * sporadyczne błędy alokacji
  * potem brak pamięci
  * potem chaos w RTOS

## 🔬 Akt II — Co się stało

* Task komunikacji:

  * alokuje bufor na każdą ramkę
  * czasem nie zwalnia
* 200 bajtów/godzinę → po tygodniach: katastrofa

## 🧨 Akt III — Dlaczego to przeszło testy

* Testy trwały godziny / dni, nie miesiące
* W labie zawsze był „świeży start”
* Nikt nie patrzył na high-water mark RAM

## 🛠️ Akt IV — Jak projektować

* Zero malloc/free w runtime
* Memory pools o stałym rozmiarze
* Monitoring zużycia pamięci
* Tryby degradacji (np. wyłącz telemetrię, ratuj sterowanie)

---

# 🌌 DEEP-DIVE #3: Cumulative jitter („śmierć przez tysiąc mikrosekund”)

## 🎬 Akt I — Objaw

* Każda pętla RT:

  * mieści się w deadline
* Ale po tygodniach:

  * okna komunikacyjne się rozjeżdżają
  * synchronizacja z Ziemią się gubi
  * algorytmy zaczynają wchodzić sobie w drogę

## 🔬 Akt II — Co się stało

* Każdy task ma:

  * +5 µs jitter
* Scheduler nie pilnuje **fazowania**
* Błąd narasta w czasie → dryf systemowy

## 🧨 Akt III — Dlaczego tego nie widać w testach

* Testy krótkie: wszystko OK
* Problem wychodzi po:

  * tysiącach
  * milionach cykli

## 🛠️ Akt IV — Jak projektować

* Global time base
* Time-triggered scheduling
* Periodic resynchronization
* Budżety jitteru, nie tylko WCET

---

# 🌌 DEEP-DIVE #4: Stack overflow („duch w maszynie”)

## 🎬 Akt I — Objaw

* Raz na kilka dni:

  * losowy reset
  * albo dziwne decyzje algorytmu
* Brak powtarzalnego scenariusza

## 🔬 Akt II — Co się stało

* Rzadki przypadek danych → większa głębokość stosu
* Task przekracza limit stacka
* Nadpisuje:

  * sąsiedni task
  * albo struktury RTOS

## 🧨 Akt III — Dlaczego to takie zdradliwe

* 99.999% czasu działa idealnie
* Błąd tylko w ekstremalnych danych
* Objawy **nie wskazują** na stos

## 🛠️ Akt IV — Jak projektować

* Canary / guard pattern na stosie
* Statyczna analiza zużycia stacka
* Duże marginesy
* Watchdog per task + stack watermark

---

# 🌌 BONUS: DEEP-DIVE #5: Uszkodzenie struktur schedulera (SEU w kernelu)

## Objaw:

* Task znika z harmonogramu
* Inny task dostaje CPU „za często”
* System „działa”, ale **nie ten, co trzeba**

## Obrona:

* Redundantne struktury krytyczne
* Sanity checks kernela
* Periodic consistency scan
* Kontrolowany restart kernela + restore stanu krytycznego

---

## 🧠 Slajd zbiorczy — Lekcja z kosmosu

> Najgroźniejsze błędy w systemach kosmicznych to:
>
> - te, które __nie powodują crasha__
> - te, które __narastają powoli__
> - te, które __wychodzą po miesiącach__

RTOS w kosmosie musi:
- wykrywać
- izolować
- korygować
- __i umieć przeżyć własne uszkodzenia__

______________________________________________________________________

Jeśli chcesz, to teraz możemy zrobić __porządny, pełny moduł slajdów__ z jednego z nich, np.:

- 🌌 „Memory leak, który zabił misję po 6 miesiącach”
- 🌌 „Jitter, który rozjechał synchronizację satelity”
- 🌌 „Stack overflow i poltergeist w RTOS”
- 🌌 „Bit flip, który przekręcił satelitę bokiem do Słońca”

Wybierz __jeden__, a ja go rozpiszę __slajd po slajdzie, jak prawdziwy case study do wykładu__.
I obiecuję: __nie będzie już wstydu, tylko mięso inżynierskie__ 😄🚀

RTOS i kto będzie jego następcą?

Świetne pytanie — i bardzo „architektoniczne” 😄
Krótka odpowiedź brzmi: **RTOS nie zniknie**. Ale **zmieni się w coś większego**. Tak jak Unix nie zniknął, tylko ewoluował w Linuxa, Androida, embedded Linux itd.

Zróbmy to na spokojnie i konkretnie.

---

# 🧠 1. Dlaczego RTOS w ogóle istnieje?

Bo są systemy, w których:

* liczy się **najgorszy przypadek**, nie średnia
* **deadline miss = awaria systemu**
* trzeba mieć:

  * deterministyczny scheduler
  * kontrolę nad latencją
  * przewidywalność pamięci
  * certyfikowalność

To:

* lotnictwo ✈️
* kosmos 🚀
* automotive 🚗
* robotyka 🤖
* medycyna 🏥
* przemysł 🏭

Tych domen **nie da się** „przenieść na best-effort OS” bez utraty bezpieczeństwa.

➡️ **Wniosek:** RTOS jako idea **nie umrze**.

---

# 🔮 2. Co się więc zmieni? (Bo klasyczny RTOS też ma swoje limity)

Klasyczne RTOS-y:

* są:

  * niskopoziomowe
  * ręczne w integracji
  * trudne w weryfikacji
* nie skaluje się dobrze na:

  * wielordzeniowość
  * heterogeniczne SoC (CPU + GPU + NPU)
  * mixed-criticality
  * cyberbezpieczeństwo

Świat idzie w stronę:

* **wielu poziomów krytyczności**
* **wielu typów obliczeń**
* **wielu domen w jednym SoC**

---

# 🧱 3. Najbardziej prawdopodobny „następca”:

## 👉 Platformy Mixed-Criticality + Microkernel / Separation Kernel

Zamiast:

> „Jeden RTOS dla wszystkiego”

Będzie:

> **Platforma**, która uruchamia:

* partycję hard-RT (sterowanie)
* partycję safety (monitoring)
* partycję soft-RT (percepcja)
* partycję Linux/AI (planowanie, UI)

Na wspólnym sprzęcie, ale:

* z **twardą izolacją czasową i pamięciową**
* z **formalnie weryfikowanym kernelem**

Przykłady kierunku:

* seL4 (formalnie zweryfikowany microkernel)
* QNX Hypervisor
* PikeOS
* INTEGRITY
* AUTOSAR Adaptive + Classic
* ARINC 653 style partitioning

➡️ To nie jest „nowy RTOS”.
➡️ To jest **RTOS jako jedna z partycji w większym systemie**.

---

# 🕰️ 4. Time-Triggered Systems zamiast „klasycznego schedulera”

Coraz więcej systemów safety idzie w stronę:

* **Time-Triggered Architecture (TTA)**
* **Static schedules**
* Sztywne okna czasowe zamiast „kto ma wyższy priorytet”

Zamiast:

* „task się obudzi, jak scheduler pozwoli”

Masz:

* „task A zawsze w oknie 0–200 µs”
* „task B zawsze w oknie 200–350 µs”
* „komunikacja zawsze w slocie 10”

Efekt:

* zero jitteru strukturalnego
* łatwiejsza certyfikacja
* łatwiejsza analiza WCET/WCRT

To już widać w:

* avionice
* automotive (TSN, TTA, time-aware scheduling)
* systemach kosmicznych

---

# 🛡️ 5. Microkernel + formalna weryfikacja = przyszłość safety

Duży trend:

> Zamiast „ufać” kernelowi, **udowadniamy**, że jest poprawny.

Przykład:

* seL4:

  * formalny dowód poprawności kernela
  * brak klas całych bugów (race, use-after-free w kernelu itd.)
* Krytyczne systemy:

  * sterowanie
  * bezpieczeństwo
  * monitorowanie
    działają jako **oddzielne, izolowane komponenty**

RTOS w tej wizji:

* nie jest monolitem
* jest **jedną z usług w architekturze mikrojądra**

---

# 🧬 6. Języki i narzędzia też zmieniają grę

C/C++ nie zniknie szybko, ale:

* coraz więcej:

  * Rust (safety memory)
  * SPARK/Ada (formal methods)
  * model-based design
* coraz więcej:

  * generowania kodu z modeli czasowych
  * automatycznej analizy schedulingu
  * automatycznych dowodów własności czasowych

Czyli:

> Mniej „ręcznego dłubania w taskach”, więcej **projektowania systemu jako modelu czasu i zasobów**.

---

# 🤖 7. A co z AI, GPU, NPU?

Tu klasyczny RTOS **sam nie wystarczy**.

Nadchodzi era:

* heterogenicznych systemów:

  * CPU (RT)
  * GPU / NPU (percepcja, ML)
* z orkiestracją:

  * kto
  * gdzie
  * kiedy
    może liczyć

Przyszłość to:

* **RTOS jako część orkiestratora czasu i zasobów**
* nie „system operacyjny”, tylko **platforma wykonawcza dla mixed-criticality**

---

# 🏁 8. Odpowiedź w jednym zdaniu

> RTOS nie będzie zastąpiony. Zostanie **wchłonięty** przez większe platformy: microkernel + partycjonowanie + time-triggered scheduling + formalna weryfikacja + mixed-criticality.

Albo jeszcze prościej:

> Następcą RTOS nie jest „nowy OS”.
> Następcą RTOS jest **architektura systemu, w której RTOS jest tylko jednym z elementów**.

---

# 🎯 Jeśli chcesz, możemy pójść w bardzo konkretną stronę:

* 🧱 „Jak wygląda system przyszłości: seL4 + RT partycja + Linux + AI?”
* 🕰️ „Time-triggered vs priority-based — co wygra w safety?”
* 🚗 „Jak automotive już teraz przechodzi z klasycznych RTOS na platformy mixed-criticality?”
* 🚀 „Jak byś dziś zaprojektował system kosmiczny/lotniczy ‘next-gen’?”

Powiedz, który kierunek Cię najbardziej interesuje, a rozrysujemy go **jak prawdziwą architekturę systemu** 🧠📐
