# Słownik pojęć RTOS — Systemy Czasu Rzeczywistego

______________________________________________________________________

## 1. Podstawowe pojęcia czasu rzeczywistego

### RTOS (Real-Time Operating System)

System operacyjny czasu rzeczywistego — specjalizowany system operacyjny zaprojektowany do przetwarzania danych i reagowania na zdarzenia w ściśle określonych ramach czasowych. W odróżnieniu od zwykłych systemów operacyjnych, RTOS gwarantuje, że zadania zostaną wykonane w określonym czasie, nawet w najgorszym przypadku.

### Deterministyczność

Właściwość systemu oznaczająca, że czas wykonania operacji jest z góry znany i powtarzalny. System deterministyczny zawsze reaguje w przewidywalny sposób — dane wejściowe zawsze dają te same wyniki w tym samym czasie. Jest to fundamentalna cecha systemów hard RTOS.

### Latencja

Czas, jaki upływa od momentu wystąpienia zdarzenia do momentu rozpoczęcia jego obsługi. W kontekście RTOS najczęściej mówi się o **latencji przerwań** (interrupt latency) — czasie od zgłoszenia przerwania do rozpoczęcia wykonania procedury obsługi przerwania (ISR).

### Jitter

Zmienność czasu reakcji systemu. Jeśli zadanie ma być wykonywane co 1 ms, ale czasem wykonuje się po 0,8 ms, a czasem po 1,2 ms, to różnica 0,4 ms to jitter. W systemach sterowania jitter jest krytyczny — małe wahania mogą powodować niestabilność układu regulacji.

### Deadline (termin wykonania)

Ostateczny czas, do którego zadanie musi zostać zakończone. W systemach czasu rzeczywistego rozróżnia się:

- **Twardy (hard deadline)** — niedotrzymanie oznacza katastrofę systemu
- **Miękki (soft deadline)** — niedotrzymanie obniża jakość, ale system dalej działa
- **Firm (firm deadline)** — wynik po terminie jest bezużyteczny

### WCET (Worst-Case Execution Time)

Najgorszy możliwy czas wykonania zadania. Projektant systemu RTOS musi znać WCET każdego krytycznego zadania, aby zagwarantować dotrzymanie deadline'ów. WCET analizuje się statycznie lub mierzy empirycznie.

### WCRT (Worst-Case Response Time)

Najgorszy czas od zgłoszenia zdarzenia do zakończenia obsługi. Obejmuje czas oczekiwania w kolejce, czas wykonania zadania i czas przełączenia kontekstu.

______________________________________________________________________

## 2. Architektura i organizacja zadań

### Task (zadanie)

Podstawowa jednostka pracy w RTOS — autonomiczny wątek wykonania z własnym stosem. Task może być:

- **Okresowy** — wykonywany w regularnych interwałach czasowych
- **Aperiodyczny** — uruchamiany w odpowiedzi na zdarzenie
- **Ciągły** — działa w pętli bez końca

### Wątek (thread)

Lżejsza od procesu jednostka wykonawcza. Wątki w tym samym procesie współdzielą pamięć, ale mają własne rejestry i stos. W RTOS terminy „task" i „wątek" często są używane zamiennie.

### ISR (Interrupt Service Routine)

Procedura obsługi przerwania — kod wykonywany w odpowiedzi na zdarzenie sprzętowe. W RTOS ISR powinna wykonywać minimalną pracę i sygnalizować zadanie do dalszej obsługi przez kolejkę lub semafor.

### Scheduler (planista)

Jądro systemu zarządzające przydziałem czasu CPU do zadań. Decyduje które zadanie, kiedy i na jak długo otrzymuje procesor. Rodzaje schedulerów:

- **Priority-based** — zadania o wyższym priorytecie wyprzedzają niższe
- **Round-robin** — każde zadanie otrzymuje równy kwant czasu
- **EDF (Earliest Deadline First)** — priorytet ma zadanie z najbliższym deadline'em

### Preempcja

Mechanizm pozwalający schedulerowi przerwać wykonywanie zadania i oddać CPU innemu zadaniu. W systemach preemptive zadanie o wyższym priorytecie może w każdej chwili wyprzedzić zadanie niższe.

### Context switch (przełączenie kontekstu)

Zapis stanu jednego zadania i przywrócenie stanu innego. Context switch ma narzut czasowy — im więcej przełączeń, tym mniej czasu CPU na faktyczną pracę.

______________________________________________________________________

## 3. Mechanizmy synchronizacji

### Mutex (Mutual Exclusion)

Mechanizm wzajemnego wykluczania — binarne narzędzie synchronizacji pozwalające na dostęp do współdzielonego zasobu tylko jednemu zadaniu w danym momencie. Chroni sekcję krytyczną. Problemem jest możliwość deadlocka i priority inversion.

### Semafor

Zmienna chroniona z operacjami „wait" (P) i „signal" (V). Wyróżniamy:

- **Semafor binarny** — działa jak mutex, ale bez właściciela
- **Semafor zliczający** — pozwala na określoną liczbę jednoczesnych dostępów

### Kolejka komunikatów (Message Queue)

Struktura danych FIFO służąca do komunikacji między zadaniemi. Producent wpisuje dane, konsument je odczytuje. Kolejki są podstawowym mechanizmem message passing w RTOS.

### Sekcja krytyczna

Fragment kodu, który musi być wykonany atomowo — bez możliwości przerwania przez inne zadanie. Sekcja krytyczna jest chroniona mutexem lub wyłączeniem przerwań.

______________________________________________________________________

## 4. Problemy współbieżności

### Deadlock (zakleszczenie)

Sytuacja, w której dwa lub więcej zadań wzajemnie czekają na zasoby trzymane przez drugie. Klasyczny przykład: zadanie A trzyma mutex 1 i czeka na mutex 2, a zadanie B trzyma mutex 2 i czeka na mutex 1.

### Livelock (kołowacenie)

Stan, w którym zadania ciągle zmieniają stan, ale nie mogą postąpić do przodu. W odróżnieniu od deadlocka, zadania „pracują", ale bez efektu.

### Starvation (zagłodzenie)

Sytuacja, w której zadanie o niskim priorytecie nigdy nie otrzymuje CPU, ponieważ zadania o wyższym priorytecie cały czas je przepraszają.

### Race condition (stan wyścigu)

Błąd wynikający z nieokreślonej kolejności wykonania operacji przez współbieżne zadania. Klasyczny przykład: dwa zadania inkrementują tę samą zmienną — wynik końcowy jest nieprzewidywalny.

### Priority Inversion (odwrócenie priorytetów)

Zjawisko, w którym zadanie o niskim priorytecie blokuje zasób potrzebny zadaniu o wysokim priorytecie. Zadanie średniego priorytetu wyprzedza zadanie niskie, a to z kolei wyprzedza wysokie.

______________________________________________________________________

## 5. Protokoły rozwiązywania problemów synchronizacji

### Priority Inheritance (dziedziczenie priorytetów)

Protokół, w którym zadanie trzymające mutex tymczasowo otrzymuje priorytet zadania, które na ten mutex czeka. Minimalizuje efekt priority inversion.

### Priority Ceiling Protocol (protokół sufitu priorytetów)

Każdy mutex ma przypisany sufit priorytetu (najwyższy priorytet zadania, które może go użyć). Zadanie blokujące mutex działa z tym wyższym priorytetem.

### Time-outy

Mechanizm zabezpieczający przed deadlockem — zadanie rezygnuje po określonym czasie oczekiwania na zasób.

______________________________________________________________________

## 6. Planowanie zadań

### Rate Monotonic Scheduling (RMS)

Algorytm planowania z stałymi priorytetami, gdzie krótszy okres = wyższy priorytet. Formalnie udowodniono, że przy obciążeniu CPU ≤ 69% system jest zawsze planowalny.

### Deadline Monotonic

Wariant RMS, gdzie priorytet jest tym wyższy, im krótszy jest deadline (niekoniecznie równy okresowi).

### Time-Triggered Scheduling

Statyczny harmonogram, w którym każde zadanie ma przydzielony stały przedział czasowy. Eliminuje jitter strukturalny i ułatwia certyfikację.

### Time Slicing

Podział czasu CPU między zadania o tym samym priorytecie. Każde otrzymuje kwant czasu轮转。

### CPU Utilization (wykorzystanie CPU)

Procent czasu CPU zużywany przez zadania. W projektowaniu RTOS planuje się z zapasem — typowo 50-70%, aby mieć rezerwę na przeciążenie.

### Budget czasowy

Maksymalny czas, jaki zadanie lub partycja może używać CPU w danym okresie. Monitorowanie budgetu pozwala wykryć przeciążenie.

______________________________________________________________________

## 7. Architektura systemów

### Partycjonowanie (partitioning)

Izolacja zadań w czasie i/lub pamięci. Każda partycja ma własny harmonogram i nie może zakłócać innych. Stosowane w avionice (ARINC 653).

### Mixed-Criticality

Architektura, w której na tym samym sprzęcie działają systemy o różnym poziomie krytyczności (np. sterowanie bezpieczeństwa + rozrywka). Wymaga silnej izolacji.

### Hypervisor

Warstwa oprogramowania pozwalająca uruchamiać wiele systemów operacyjnych na jednym sprzęcie. Umożliwia izolację partycji o różnej krytyczności.

### Microkernel

Minimalne jądro systemu zawierające tylko podstawowe funkcje (scheduling, IPC). Reszta usług działa jako oddzielne procesy. Zwiększa bezpieczeństwo i izolację.

______________________________________________________________________

## 8. Pojęcia specyficzne dla systemów embedded/RT

### Lock-free Queue

Kolejka nieblokująca — implementowana bez użycia mutexów, na atomowych operacjach. Używana do komunikacji ISR → task.

### Ring Buffer

Bufor cykliczny — struktura danych z określonym początkiem i końcem. Po zapisaniu na końcu wraca do początku. Używana w komunikacji szeregowej.

### Double-buffering

Technika z dwoma buforami — jednen jest wypełniany, drugi przetwarzany. Eliminuje problem jednoczesnego dostępu.

### Message Passing

Komunikacja przez przekazywanie komunikatów, nie przez współdzieloną pamięć. Bezpieczniejsza i bardziej skalowalna.

### Ownership (własność)

Zasada, że dane mają jednego właściciela — jedyne zadanie, które może je modyfikować. Inne zadania mogą tylko odczytywać lub muszą otrzymać kopię.

### Memory Pool

Pulapamięci o stałym rozmiarze — alternatywa dla malloc/free. Eliminuje fragmentację pamięci i jest deterministyczna.

### Zero-copy

Przesyłanie danych bez kopiowania — wskaźnik jest przekazywany między zadaniami. Zmniejsza latencję kosztem złożoności zarządzania.

### Watchdog

Mechanizm monitorujący czy zadanie lub system działa poprawnie. Jeśli zadanie nie zgłasza się w określonym czasie, następuje restart lub przejście do safe mode.

### Safe Mode

Bezpieczny stan systemu po wykryciu awarii — ograniczenie funkcjonalności, ale utrzymanie podstawowej pracy (np. zatrzymanie robota w miejscu).

### Graceful Degradation

Kontrolowana degradacja — system reaguje na przeciążenie zmniejszeniem funkcjonalności, ale nadal działa (np. niższa częstotliwość aktualizacji czujników).

______________________________________________________________________

## 9. Narzędzia i pomiary

### Tracealyzer

Narzędzie do wizualizacji śladu wykonania systemu RTOS. Pokazuje timeline zadań, przerwań, przełączeń kontekstu. Nieocenione do debugowania problemów timingowych.

### Cyclictest

Benchmark do pomiaru latencji Linux PREEMPT_RT. Wielokrotnie uruchamia task i mierzy różnicę między planowanym a rzeczywistym czasem wykonania.

### WCET Analysis

Analiza statyczna lub dynamiczna najgorszego czasu wykonania. Wymagana do certyfikacji systemów safety-critical.

### CPU Affinity

Przypisanie zadania do określonego rdzenia procesora. Używane do izolacji zadań RT od innych.

### Governor

W systemie Linux — regulator częstotliwości CPU. Dla systemów RT wymaga ustawienia na „performance" (stała maksymalna częstotliwość).

### C-States

Stany oszczędzania energii procesora. Dla hard RTOS wyłączane, ponieważ wprowadzają nieprzewidywalne opóźnienia wybudzania.

### Turbo Boost

Technologia Intela pozwalająca na chwilowe przekroczenie nominalnej częstotliwości. W systemach RT często wyłączana dla determinizmu.

______________________________________________________________________

## 10. Standardy i certyfikacja

### DO-178C

Standard FAA/EASA dla oprogramowania lotniczego. Definiuje poziomy od A (katastrofalny) do E (brak efektu). Wyższy poziom = większe wymagania weryfikacyjne.

### ARINC 653

Standard architektury awioniki z partycjonowaniem czasowym i pamięciowym. Wymaga izolacji między systemami (np. Flight Control, Navigation).

### ISO 26262

Standard bezpieczeństwa funkcjonalnego dla motoryzacji. Definiuje poziomy ASIL (A-D) i wymagania dla systemów elektronicznych.

### ASIL (Automotive Safety Integrity Level)

Poziom integralności bezpieczeństwa w motoryzacji (ASIL A najniższy, ASIL D najwyższy). Określa rygorystyczność procesu rozwoju i weryfikacji.

### seL4

Formalnie zweryfikowany microkernel. Pierwszy z formalnym dowodem poprawności — gwarantuje brak określonych klas błędów.

______________________________________________________________________

## 11. Pojęcia specyficzne dla lotnictwa i kosmosu

### ECC (Error-Correcting Code)

Korekcja błędów w pamięci — kod wykrywający i naprawiający bity błędy. W kosmosie stosowany przeciwko promieniowaniu.

### TMR (Triple Modular Redundancy)

Potrójna redundancja — trzy kopie obliczeń głosujące. Awaria jednej nie wpływa na wynik.

### SEU (Single Event Upset)

Zakłócenie pojedynczego zdarzenia — błąd pojedynczego bitu wywołany cząstką promieniowania. W kosmosie częste i nieuchronne.

### SEL (Single Event Latch-up)

Zwarcie w układzie scalonym wywołane promieniowaniem. Może spowodować „zamrożenie" CPU.

### Memory Scrubbing

Okresowe sprawdzanie i naprawianie pamięci ECC. Wykrywa błędy zanim spowodują problemy.

### State Sanity Check

Weryfikacja, czy stan systemu jest fizycznie możliwy (np. czy kąt orientacji mieści się w zakresie).

______________________________________________________________________

## 12. RTOS w praktyce — przykładowe systemy

### FreeRTOS

Najpopularniejszy darmowy RTOS dla mikrokontrolerów. Lekki, prosty, szeroko wspierany.

### Zephyr RTOS

Nowoczesny RTOS Linux Foundation. Modułowy, wspiera wiele architektur.

### QNX

Komercyjny RTOS microkernel. Wykorzystywany w automotive, medycynie, infrastrukturze.

### VxWorks

Komercyjny RTOS, historycznie używany w misjach kosmicznych NASA.

### PREEMPT_RT

Patch do kernela Linux dodający możliwości hard RTOS. Zwiększa determinizm kosztem złożoności.

### Linux PREEMPT_RT

Linux z nałożonym patchem PREEMPT_RT. Nie jest „prawdziwym" RTOS, ale zbliża się do niego.

______________________________________________________________________

## 13. Specyficzne scenariusze awarii

### Memory Leak

„Wyciek pamięci" — alokacja bez zwolnienia. W długotrwałych misjach (satélity) prowadzi do wyczerpania pamięci.

### Stack Overflow

Przepełnienie stosu — zadanie zużywa więcej pamięci stosu niż przydzielono. Może uszkodzić inne zadania lub jądro.

### ISR Storm

„Burza przerwań" — seria szybko po sobie następujących przerwań (np. przez EMI), która paraliżuje system.

### Fragmentacja pamięci

Rozkład alokacji i de-alokacji prowadzący do niemożności przydzielenia większego bloku pomimo wystarczającej sumarycznej pamięci.

### Jitter Burst

Nagły wzrost jittera spowodowany np. przeciążeniem sieci, dużą ilością przerwań, lub problemami z pamięcią.

### Priority Inversion (szczegółowo)

Zadanie wysokiego priorytetu czeka na zadanie niskiego, które trzyma mutex, podczas gdy zadanie średniego priorytetu wykonuje się. Klasyczny Mars Pathfinder.

______________________________________________________________________

## 14. Pojęcia związane z robotyką

### Pętla sterowania (control loop)

Cykliczne zadanie realizujące algorytm regulacji. Typowo 1-10 kHz w robotach.

### Sensor Fusion

Łączenie danych z wielu czujników dla lepszej estymacji stanu (np. fuzja IMU + GPS).

### Estymacja stanu

Określanie aktualnego stanu robota (pozycja, prędkość, orientacja) na podstawie czujników.

### Trajektoria

Zaplanowana ścieżka ruchu robota w czasie. Generowana przez planner, wykonywana przez kontroler.

### Soft Real-Time

System, w którym opóźnienia obniżają jakość, ale nie powodują katastrofy. Np. streaming wideo.

### Hard Real-Time

System, w którym przekroczenie deadline'a to awaria. Np. sterowanie lotem, hamulce w samochodzie.

### RT Gateway

Zadanie mostu między światem RT a systemem Linux. Waliduje i buforuje polecenia z Linuxa.

______________________________________________________________________

## 15. Zagadnienia zaawansowane

### Formal Verification

Matematyczny dowód poprawności systemu. W krytycznych systemach (lotnictwo, jądra seL4) wymagany zamiast tylko testów.

### Response Time Analysis

Formalna analiza czasów odpowiedzi zadań. Oblicza WCRT dla danej konfiguracji priorytetów i obciążenia.

### Schedulability Analysis

Sprawdzenie, czy dla danego zestawu zadań istnieje harmonogram spełniający wszystkie deadline'y.

### Temporal Isolation

Izolacja czasowa — gwarancja, że jedno zadanie nie wpłynie na czasy wykonania innych.

### Heterogeneous SoC

Procesor z różnymi typami rdzeni (np. Cortex-A + Cortex-R + GPU). Wymaga specjalnej architektury RTOS.

### DMA (Direct Memory Access)

Bezpośredni dostęp do pamięci przez peryferia — odciąża CPU. W RTOS wymaga synchronizacji z taskami.

### Tickless Idle

Tryb OS, w którym zegar (tick) jest zatrzymywany, gdy system jest bezczynny. Oszczędza energię, ale wymaga specjalnej obsługi budzenia.

### High-Water Mark

Najwyższy dotychczasowy poziom zużycia zasobu (np. stosu, pamięci). Monitorowanie pozwala wykryć problemy.

### Bounding Jitter

Ograniczanie jittera — techniki minimalizujące zmienność czasu reakcji systemu. Obejmuje:

- Statyczne harmonogramy (time-triggered)
- Izolację zadań RT od innych
- Wyłączanie przerwań w krytycznych sekcjach
- CPU affinity dla zadań czasowych

### NMI (Non-Maskable Interrupt)

Niemaszkowalne przerwanie — przerwanie, którego nie można zablokować. Używane dla krytycznych zdarzeń (np. watchdog, utrata zasilania). W systemach RT NMI ma najwyższy priorytet i musi być obsłużone natychmiast.

### MPU (Memory Protection Unit)

Jednostka ochrony pamięci — sprzętowy mechanizm izolujący pamięć między zadaniami. MPU definiuje regiony pamięci z uprawnieniami (odczyt/zapis/wykonywanie). W systemach embedded chroni przed błędami i atakami.

### AFDX (Avionics Full-Duplex Ethernet)

Avionics Full-Duplex Ethernet — standard sieci lotniczej. Determininistyczny Ethernet z gwarantowaną przepustowością i ograniczonym jitterem. Używany w nowoczesnej avionice do komunikacji między systemami.

### Safety Case

Dokumentacja bezpieczeństwa — strukturalny dowód, że system spełnia wymagania bezpieczeństwa. Zawiera:

- Analizę zagrożeń (HARA)
- Wymagania bezpieczeństwa
- Dowody zgodności (testy, analizy, weryfikacje)
- Zarządzanie ryzykiem

### Izolacja rdzeni (AMP / Big-Little)

Architektura z wieloma rdzeniami o różnej mocy:

- **AMP (Asymmetric Multi-Processing)** — każdy rdzeń ma własny OS
- **Big-Little** — połączenie wydajnych (big) i energooszczędnych (little) rdzeni
  W systemach RT: rdzenie RT izolowane od Linuxa dla determinizmu.

### Deterministic DMA

Deterministyczny DMA — transfer danych bez obciążania CPU z gwarantowanym czasem. Kluczowe:

- Buforowanie danych przed transferem
- Sygnalizacja zakończenia przez event/flagę
- Unikanie blokowania w ISR

______________________________________________________________________

## 17. Dodatkowe akronimy

| Skrót | Znaczenie |
|-------|-----------|
| AFDX | Avionics Full-Duplex Ethernet |
| NMI | Non-Maskable Interrupt |
| MPU | Memory Protection Unit |
| HARA | Hazard Analysis and Risk Assessment |
| DO-254 | Design Assurance Guidance for Electronic Hardware |
| TTA | Time-Triggered Architecture |
| TSN | Time-Sensitive Networking |
| AMP | Asymmetric Multi-Processing |
| SMP | Symmetric Multi-Processing |

______________________________________________________________________

*Ten słownik zawiera pojęcia niezbędne do zrozumienia wykładu o systemach RTOS. Każde pojęcie zostało wyjaśnione w kontekście praktycznym, z uwzględnieniem zastosowań w robotyce, lotnictwie, kosmonautyce i motoryzacji.*
