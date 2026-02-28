# Wyklad 7: Roadmapa implementacji (od modelu do systemu wielowarstwowego)

## Czesc I: Wstep teoretyczny — dlaczego potrzebujemy roadmapy

### 1.1 Geneza problemu

Prosze wyobrazic sobie sytuacje: zespól inżynierów dostaje zadanie zbudowania systemu sterowania wirówki laboratoryjnej. Co robią?

**Podejscie "na żywioł":**
- Tydzien 1: Piszą kod sterowania
- Tydzien 2: Dodają komunikację
- Tydzien 3: Dodają GUI
- Tydzien 4: Testują — i wszystko się psuje
- Miesiąc 2: Debugowanie, naprawianie, więcej debugowania
- Miesiąc 3: Kolejne problemy z integracją
- Miesiąc 6: System "działa" ale nikt nie wie dlaczego

**Podejście z roadmapą:**
- Tydzień 1-2: Model + symulacja (offline)
- Tydzień 3-4: Prototyp RT + pomiary
- Tydzień 5-6: Integracja magistrali
- Tydzień 7-8: Nadzor + IPC
- Tydzień 9-10: Digital twin
- Tydzień 11-12: Safety + testy awaryjne

Które podejście jest lepsze? Oczywiście drugie!

### 1.2 Dlaczego iteracje sa wazne

Profesor zawsze powtarza: **"Nie da się zbudować systemu RT w jednym kroku."**

Dlaczego? Bo:

1. **Nie wiemy, czego nie wiemy** — dopiero po uruchomieniu会发现 się problemy
2. **Każda warstwa ma inne wymagania** — trzeba je po kolei zweryfikować
3. **Iteracja jest tańsza niż naprawa** — łatwiej naprawić mały kawałek niż całość
4. **Ryzyko rośnie wykładniczo** — im później znajdziesz błąd, tym droższy

### 1.3 Zasada "definition of done"

Każdy etap musi mieć **mierzalne kryterium sukcesu**:

| Etap | Definition of Done |
|------|-------------------|
| Model | Symulacja stabilna, regulator działa w MATLAB/Python |
| Prototyp RT | P99.9 < 500μs przy cyklu 1ms |
| Magistrala | Komunikacja działa, brak dropout |
| IPC | Dane płyną z W2 do W3 bez strat |
| Digital twin | Błąd model vs rzeczywistość < 5% |
| Safety | Testy fault injection przechodzą |

**Bez definition of done — nie wiadomo, kiedy etap się kończy!**

---

## Czesc II: Kroki implementacji — szczegóły

### 2.1 Krok 1: Model ODE w Python/MATLAB

**Cel:** Zweryfikować algorytm sterowania offline

```python
# Model wirnika
# J * domega/dt = tau - load - B * omega
# 
# J - moment bezwładności
# omega - prędkość kątowa
# tau - moment napędowy
# load - obciążenie
# B - współczynnik tarcia

import numpy as np
from scipy.integrate import odeint

def rotor_model(omega, t, J, tau, load, B):
    domega_dt = (tau - load - B * omega) / J
    return domega_dt

# Parametry
J = 0.01  # kg*m^2
B = 0.001 # Nms/rad

# Symulacja
t = np.linspace(0, 10, 1000)
omega0 = 0
tau = 0.1

omega = odeint(rotor_model, omega0, t, args=(J, tau, 0, B))
```

**Co weryfikujemy:**
- Stabilność regulatora
- Odpowiedź na skok
- Zachowanie przy nasyceniu
- Robustność na zakłócenia

### 2.2 Krok 2: Projekt regulatora

**Cel:** Zaprojektować PI/PID z saturacją i anti-windup

```python
# Regulator PI z anti-windup
class PIController:
    def __init__(self, kp, ki, umin, umax):
        self.kp = kp
        self.ki = ki
        self.umin = umin
        self.umax = umax
        self.integral = 0
        self.u_prev = 0
    
    def compute(self, setpoint, measurement, dt):
        error = setpoint - measurement
        
        # Część proporcjonalna
        p = self.kp * error
        
        # Część całkująca z anti-windup
        self.integral += self.ki * error * dt
        self.integral = np.clip(self.integral, self.umin, self.umax)
        
        # Wyjście
        u = p + self.integral
        
        # Back-calculation anti-windup
        if u > self.umax:
            self.integral -= (u - self.umax)
        elif u < self.umin:
            self.integral -= (u - self.umin)
        
        return np.clip(u, self.umin, self.umax)
```

**Testy scenariuszy:**
- Skok setpointu
- Skok obciążenia
- Przejście przez rezonans
- Nasycenie
- Odcięcie zasilania

### 2.3 Krok 3: Prototyp W2 na Linux PREEMPT_RT

**Cel:** Zweryfikować determinizm w rzeczywistym systemie

```c
// Szablon wątku RT
void* rt_control_thread(void* arg) {
    // 1. Konfiguracja SCHED_FIFO
    struct sched_param param = {.sched_priority = 99};
    pthread_setschedparam(pthread_self(), SCHED_FIFO, &param);
    
    // 2. Pinning do rdzenia
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(0, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);
    
    // 3. Memory locking
    mlockall(MCL_CURRENT | MCL_FUTURE);
    
    // 4. Pętla
    uint64_t period_ns = 1000000; // 1ms
    uint64_t next;
    clock_gettime(CLOCK_MONOTONIC, (struct timespec*)&next);
    
    while (running) {
        next += period_ns;
        clock_nanosleep(CLOCK_MONOTONIC, TIM_ABSTIME, (struct timespec*)&next, NULL);
        
        uint64_t t_start = get_time_ns();
        
        // Kod sterowania
        read_inputs();
        compute_control();
        write_outputs();
        
        uint64_t t_end = get_time_ns();
        record_timing(t_start, t_end);
    }
    
    return NULL;
}
```

**Metryki do zebrania:**
- Histogram czasów wykonania
- P50/P95/P99/P99.9
- Licznik missed deadline
- Jitter

### 2.4 Krok 4: Integracja EtherCAT master

**Cel:** Dodać komunikację z slave'ami

```c
// Cykl EtherCAT
void ethercat_cycle() {
    // 1. Przygotuj dane do wysłania
    prepare_pdo_data();
    
    // 2. Wyślij i odbierz
    ec_send_processdata();
    ec_receive_processdata();
    
    // 3. Sprawdź status
    if (WKC != expected_WKC) {
        handle_bus_fault();
    }
    
    // 4. Przetwórz dane
    process_pdo_data();
}
```

**Testy:**
- Komunikacja z jednym slave
- Komunikacja z wieloma slave'ami
- Dropout komunikacji
- Recovery po błędzie

### 2.5 Krok 5: Dodanie W3 (nadzor) i IPC

**Cel:** Dodać interfejs użytkownika

```python
# W3 - nadzor
class Supervisor:
    def __init__(self):
        self.ring_buffer = SharedMemory("rt_shm")
        self.gui = GUI()
        self.logger = Logger()
    
    def run(self):
        while True:
            # Odczyt z ring buffer
            sample = self.ring_buffer.read()
            
            # Aktualizacja GUI
            self.gui.update(sample)
            
            # Logowanie
            self.logger.log(sample)
            
            sleep(0.01)  # 100 Hz
```

### 2.6 Krok 6: Digital twin

**Cel:** Porównać model z rzeczywistością

```python
# Digital twin - porównanie
class DigitalTwin:
    def __init__(self):
        self.model = RotorModel()
        self.baseline_error = None
    
    def compare(self, real_data):
        # Symulacja modelu
        predicted = self.model.predict(real_data.t)
        
        # Błąd
        error = abs(real_data.omega - predicted.omega)
        
        # Baseline
        if self.baseline_error is None:
            self.baseline_error = error
        else:
            self.baseline_error = 0.9 * self.baseline_error + 0.1 * error
        
        return error, self.baseline_error
```

**Metryki:**
- Błąd model vs rzeczywistość
- Drift parametrów
- Anomalie

### 2.7 Krok 7: Safety

**Cel:** Dodać watchdog i safe stop

```c
// Watchdog
typedef enum {
    STATE_NORMAL,
    STATE_WARNING,
    STATE_DEGRADED,
    STATE_SAFE_STOP,
    STATE_FAULT
} SafetyState;

void watchdog_check() {
    if (missed_deadline_count > 10) {
        transition_to(STATE_SAFE_STOP);
    }
    
    if (communication_timeout > 100) {
        transition_to(STATE_FAULT);
    }
    
    if (overtemperature) {
        transition_to(STATE_DEGRADED);
    }
}
```

**Testy fault injection:**
- Dropout komunikacji
- Opóźnienie i jitter
- Przeciążenie CPU
- Utrata sensora
- Przegrzanie

---

## Czesc III: Artefakty ktore musza powstac

### 3.1 Scenariusze regresji

Kazdy system musi mieć zestaw scenariuszy testowych:

| Scenariusz | Opis | Kryterium |
|------------|------|-----------|
| Zakłócenie | Skok obciążenia w połowie pracy | Stabilność w 100ms |
| Saturacja | Maksymalny moment przez 1s | Bez oscylacji |
| Dropout | Utrata 10% pakietów | Recovery < 1s |
| Opóźnienie | Opóźnienie 5ms | Stabilność |
| Load step | 100% CPU load | Brak missed deadline |

### 3.2 Metryki i progi

| Metryka | Próg akceptacji | Próg ostrzegawczy |
|---------|-----------------|-------------------|
| p99.9 rt_loop | < 500 μs | > 500 μs |
| missed_deadline | < 0.1% | > 0.1% |
| jitter | < 100 μs | > 100 μs |
| dropout komunikacji | < 0.01% | > 0.01% |
| temperatura | < 80°C | > 80°C |

### 3.3 Format logow

```c
struct __attribute__((packed)) LogEntry {
    uint64_t timestamp_ns;
    uint32_t iteration;
    float    omega_set;
    float    omega_meas;
    float    u_cmd;
    uint32_t rt_loop_us;
    uint8_t  state;
    uint8_t  fault_code;
};
```

**Zasady:**
- Stały rozmiar
- Wersjonowany
- Kompatybilny wstecz

---

## Czesc IV: Wersja dla komorki

### 4.1 Workflow robot + modul procesu

```
KROK 1: Robot pobiera próbkę
    |
    v
KROK 2: Potwierdza interlock (bezpieczeństwo)
    |
    v
KROK 3: Moduł procesu przechodzi READY -> RUNNING
    |
    v
KROK 4: Robot odchodzi do strefy bezpiecznej
    |
    v
KROK 5: Moduł procesu publikuje zdarzenia (ALARM/COMPLETE)
    |
    v
KROK 6: Robot odbiera wynik i kontynuuje
```

### 4.2 Orkiestrator

**Zasada:** Orkiestrator to soft-RT, ale reakcje awaryjne są lokalne w modułach.

```python
# Orkiestrator (soft-RT)
class WorkflowOrchestrator:
    def __init__(self):
        self.robot_fsm = RobotFSM()
        self.process_module_fsm = ProcessModuleFSM()
    
    def run_workflow(self, sample_id):
        # Krok 1: Pobierz próbkę
        self.robot_fsm.move_to("pickup")
        
        # Krok 2: Potwierdź interlock
        if not self.check_interlock():
            self.robot_fsm.move_to("safe")
            return ERROR
        
        # Krok 3: Uruchom moduł
        self.process_module_fsm.set_state("RUNNING")
        
        # Krok 4: Robot czeka
        self.robot_fsm.move_to("safe_zone")
        
        # Krok 5: Czekaj na zakończenie
        while self.process_module_fsm.get_state() != "COMPLETE":
            event = self.process_module_fsm.get_event()
            if event == "ALARM":
                self.handle_alarm()
            sleep(0.1)
        
        # Krok 6: Odbierz wynik
        result = self.process_module_fsm.get_result()
        return result
```

---

## Czesc V: Podsumowanie i checklisty

### Zasady roadmapy:

| Zasada | Wyjasnienie |
|--------|-------------|
| Definition of Done | Każdy etap ma mierzalne kryterium |
| Iteracja | Małe kroki, weryfikacja po każdym |
| Regresja | Scenariusze testowe na każdą zmianę |
| Safety od początku | Nie jako "ostatni sprint" |

### Checklisty:

- [ ] Każdy krok ma metrykę sukcesu (nie "działa u mnie")
- [ ] Zmiany wprowadzane są z regresją scenariuszy
- [ ] Safety jest wbudowany od początku

---

## Czesc VI: Pytania do dyskusji

1. Jakie metryki uznasz za "definition of done" dla każdego etapu roadmapy?
2. Jak zaprojektujesz scenariusze regresji, żeby wykryły regresję po zmianie filtru/regulatora?
3. Kiedy przechodzisz z SIL do HIL i co ma być kryterium tej decyzji?
4. Jak zaplanujesz integrację safety tak, by nie była "ostatnim sprintem"?

---

## Czesc VII: Zadania praktyczne

### Zadanie 1: Roadmap Tracker

Stwórz repo z etapami + metrykami + automatycznym raportem regresji.

### Zadanie 2: Scenario Pack

Paczka scenariuszy:
- Delay
- Dropout
- Saturacja
- Load step

Plus porównanie z baseline.

### Zadanie 3: Workflow Orchestrator

Prototyp orkiestratora robot+moduł z FSM i eventami.

---

## BONUS: Kontrakty i scenariusze

Najwięcej czasu traci się na integracji bez kontraktów i bez scenariuszy.

Zrób kontrakty i scenariusze najpierw, a integracja stanie się przewidywalna.

---

*(Koniec wykladu 7)*
