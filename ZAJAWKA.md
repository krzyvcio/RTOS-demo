Super temat — RTOS to dokładnie to miejsce, gdzie informatyka spotyka się z „twardą” inżynierią czasu ⏱️⚙️ Zrobię Ci **klarowny, logiczny plan wykładu**, pod Twoją zajawkę: scheduler, walka o CPU, synchronizacja, i porównanie z Linuksem (z naciskiem: *czy Linux może być RTOS-em*).

______________________________________________________________________

# 🧠 Tytuł zajawki - wykład w skrócie

**Systemy czasu rzeczywistego (RTOS) w praktyce informatycznej — od schedulerów po Linux PREEMPT_RT**

______________________________________________________________________

# 1️⃣ Wprowadzenie: Po co nam RTOS?

**Cele:**

- Czym jest *czas rzeczywisty*?

- Czym RTOS różni się od zwykłego systemu operacyjnego?

- Przykłady zastosowań:

  - sterowniki silników, robotyka, automotive
  - audio, wideo, przemysł, medycyna
  - systemy wbudowane vs PC

**Kluczowe pojęcia:**

- deadline
- deterministyczność
- latencja
- jitter

**Podział:**

- Hard RTOS – spóźnienie = katastrofa
- Soft RTOS – spóźnienie = spadek jakości
- Firm RTOS – spóźnienie = wynik bezużyteczny

______________________________________________________________________

# 2️⃣ Procesy, wątki i konkurencyjność (Concurrency)

**Cel:** Zrozumieć *dlaczego w ogóle mamy problem synchronizacji*

**Tematy:**

- Proces vs wątek
- Co to jest **konkurencyjność**?
- Wiele tasków „naraz” na jednym CPU
- Przełączanie kontekstu (context switch)
- Sekcja krytyczna – co to i dlaczego jest groźna

**Przykład:**

- Dwa taski inkrementują tę samą zmienną → błąd wyścigu (race condition)

______________________________________________________________________

# 3️⃣ Scheduler — kilka tasków walczy o CPU

**Twoja zajawka w punkt 👇**

**Rodzaje schedulerów:**

- Round-robin
- Priority-based
- Preemptive vs Non-preemptive
- Fixed priority (np. RTOS)
- Dynamic priority (np. Linux CFS)

**W RTOS:**

- task o wyższym priorytecie **zawsze wygrywa**
- przejęcie CPU w deterministycznym czasie
- pojęcie: *worst-case response time*

**Problem:**

- starvation (zagłodzenie)
- priority inversion (odwrócenie priorytetów)

______________________________________________________________________

# 4️⃣ Synchronizacja: mutexy, semafory, kolejki

## 🔒 Mutex

- Chroni **sekcję krytyczną**
- Tylko jeden task naraz
- Problem: deadlock, priority inversion

## 🚦 Semafor

- Licznik zasobów

- Binary semaphore vs counting semaphore

- Do:

  - synchronizacji tasków
  - sygnalizacji zdarzeń
  - ograniczania dostępu do zasobu

## 📬 Kolejki (Message Queues)

- Komunikacja między taskami
- Producent–konsument
- FIFO
- Bardzo popularne w RTOS-ach (FreeRTOS, Zephyr, QNX)

______________________________________________________________________

# 5️⃣ Zakleszczenia (Deadlock) — czyli jak system sam się zabija 💀

**Warunki deadlocka:**

1. Wzajemne wykluczanie (mutex)
1. Przetrzymywanie zasobu i czekanie na kolejny
1. Brak wywłaszczenia
1. Cykl oczekiwania

**Przykład:**

- Task A trzyma Mutex 1 i czeka na Mutex 2
- Task B trzyma Mutex 2 i czeka na Mutex 1
  → system stoi

**Jak RTOS-y walczą z deadlockiem:**

- kolejność blokad
- timeouty
- priority inheritance / priority ceiling
- analiza statyczna

______________________________________________________________________

# 6️⃣ Kolejkowanie i komunikacja w RTOS

**Modele:**

- Task → Task (queue, mailbox)
- ISR → Task (kolejki, semafory)
- Event flags
- Ring buffer

**Dlaczego to ważne w RTOS?**

- Minimalna latencja
- Deterministyczne czasy reakcji
- Brak aktywnego czekania (busy wait)

______________________________________________________________________

# 7️⃣ RTOS vs Linux — determinism vs best-effort

| Cecha | RTOS | Linux |
| ------------- | ---------------------- | ----------------------- |
| Determinizm | ✅ Tak | ❌ Best-effort |
| Latencja | Niska i przewidywalna | Zmienna |
| Scheduler | Priorytety, preemptive | CFS + RT |
| Deadline | Kluczowy | Opcjonalny |
| Przeznaczenie | Embedded, sterowanie | Desktop/Server/Embedded |

**Ale…**

______________________________________________________________________

# 8️⃣ Czy Linux może być RTOS-em? 🐧⚡

**Odpowiedź:**
➡️ *Prawie tak* — dzięki **PREEMPT_RT**

**Co daje PREEMPT_RT:**

- Preemptible kernel
- Mutexy zamiast spinlocków
- Mniejsza latencja
- Lepsza przewidywalność

**Co dalej nie jest idealne:**

- Nadal większa złożoność
- Nadal nie 100% hard RTOS
- Większy narzut systemowy

**Gdzie się sprawdza:**

- audio real-time
- robotyka
- soft PLC
- systemy sterowania na PC

______________________________________________________________________

# 9️⃣ Studium przypadku (do pokazania na wykładzie)

**Przykład systemu:**

- Task 1: odczyt czujnika (co 1 ms, wysoki priorytet)
- Task 2: przetwarzanie danych
- Task 3: logowanie na dysk
- Komunikacja przez kolejkę
- Mutex na zasób SPI / I2C

**Pytania do studentów:**

- Gdzie dać mutex?
- Gdzie kolejkę?
- Który task ma najwyższy priorytet i dlaczego?
- Co się stanie przy złej konfiguracji?

______________________________________________________________________

# 🔟 Podsumowanie

- RTOS = **przewidywalność, nie szybkość**
- Scheduler z priorytetami to serce systemu
- Mutex, semafor, kolejka = podstawowe narzędzia synchronizacji
- Deadlock i priority inversion to realne problemy
- Linux + PREEMPT_RT = „prawie RTOS”, ale z kompromisami

______________________________________________________________________

# Jeśli chcesz, mogę:

- przerobić to na **slajdy (rozdział po rozdziale)**
- dorzucić **przykłady w C / FreeRTOS / pthreads / Linux RT**
- zrobić **wersję pod prezentację na studia / szkolenie / YouTube**
- albo dopasować poziom: bardziej „akademicki” albo bardziej „praktyka embedded” 😉

Powiedz tylko: **dla kogo ma być ten wykład? (studenci, technikum, programiści, embedded?)**
