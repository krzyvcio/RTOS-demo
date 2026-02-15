# Obrona przed zakleszczeniami w systemach RTOS

______________________________________________________________________

## 1. Czym jest zakleszczenie?

Zakleszczenie (deadlock) to sytuacja, w której dwa lub więcej zadań wzajemnie czekają na zasoby trzymane przez drugie zadanie. System stoi w miejscu, żadne z zadań nie może postąpić.

### Klasyczny przykład

```
Zadanie A:
    pobierz mutex_1
    czekaj na mutex_2
    zwolnij mutex_1

Zadanie B:
    pobierz mutex_2
    czekaj na mutex_1
    zwolnij mutex_2

→ Zakleszczenie! A i B czekają na siebie nawzajem.
```

### Warunki konieczne

Do wystąpienia deadlocka muszą być spełnione wszystkie warunki:

1. **Wzajemne wykluczanie** - zasób może być używany tylko przez jedno zadanie
1. **Przetrzymywanie i czekanie** - zadanie trzyma zasób i czeka na kolejny
1. **Brak wywłaszczenia** - zasoby nie mogą być odebrane siłą
1. **Cykl oczekiwania** - tworzy się zamknięty łańcuch zależności

______________________________________________________________________

## 2. Strategie obrony przed deadlockem

### 2.1. Zapobieganie (Prevention)

Eliminacja jednego z warunków deadlocka.

#### Hierarchia zasobów

Wymuś kolejność pobierania zasobów:

```
DOBRA PRAKTYKA:
mutex_1 musi być pobrany przed mutex_2
mutex_2 musi być pobrany przed mutex_3

ZŁA PRAKTYKA:
dowolna kolejność
```

```c
// Zawsze w tej samej kolejności!
void funkcja_bezpieczna(void) {
    mutex_lock(&mutex_1);  // Zawsze pierwszy
    mutex_lock(&mutex_2);  // Zawsze drugi
    // ... operacje ...
    mutex_unlock(&mutex_2);
    mutex_unlock(&mutex_1);
}
```

#### Jednorazowe pobranie

Pobierz wszystkie potrzebne zasoby na raz:

```c
// Zamiast wielu lock/unlock:
void funkcja(void) {
    mutex_lock(&A);
    // operacje
    mutex_lock(&B);
    // operacje
}

// Pobierz wszystko na raz:
void funkcja(void) {
    lock_multiple(&locks[], 2);  // Pobierz A i B atomowo
    // operacje
    unlock_multiple(&locks[], 2);
}
```

#### Wywłaszczanie zasobów

Jeśli zadanie nie może pobrać zasobu w określonym czasie, oddaje już posiadane:

```c
bool pobierz_z_wywaszczeniem(mutex_t* m, uint32_t timeout_ms) {
    uint32_t start = get_tick_ms();
    
    while (!mutex_try_lock(m)) {
        if (get_tick_ms() - start > timeout_ms) {
            // Timeout - oddaj wszystko i spróbuj ponownie
            return false;
        }
        task_sleep(1);  // Czekaj chwilę
    }
    return true;
}
```

______________________________________________________________________

### 2.2. Unikanie (Avoidance)

Monitorowanie stanu systemu i unikanie deadlocka w runtime.

#### Time-outy

Najprostsza obrona - każde oczekiwanie ma limit czasu:

```c
#define LOCK_TIMEOUT_MS 100

void operacja(void) {
    if (mutex_lock_timeout(&uart_mutex, LOCK_TIMEOUT_MS)) {
        // Sukces - wykonaj operację
        uart_send(data);
        mutex_unlock(&uart_mutex);
    } else {
        // Timeout - loguj błąd, obsłuż sytuację
        log_error("Nie można pobrać mutexa UART");
        system_reset();  // lub safe mode
    }
}
```

#### Hierarchia czasowa

Zadanie może trzymać zasób tylko przez określony czas:

```c
#define MAX_LOCK_TIME_MS 50

void operacja_czasowa(void) {
    uint64_t start = get_tick_us();
    mutex_lock(&resource);
    
    // Sprawdź czas w pętli
    while (get_tick_us() - start < MAX_LOCK_TIME_MS) {
        if (praca_zakończona()) break;
        // wykonaj kawałek pracy
    }
    
    if (get_tick_us() - start >= MAX_LOCK_TIME_MS) {
        // Przekroczony czas - błąd!
        log_error("Przekroczony czas trzymania mutexa");
    }
    
    mutex_unlock(&resource);
}
```

______________________________________________________________________

### 2.3. Wykrywanie i odzyskiwanie (Detection and Recovery)

Pozwól deadlockowi wystąpić, ale wykryj i napraw.

#### Watchdog per zadanie

Każde krytyczne zadanie musi zgłaszać się na czas:

```c
#define CONTROL_WATCHDOG_MS 10

void control_task(void) {
    while (1) {
        // Wykonaj pętlę sterowania
        run_control_loop();
        
        // Zgłoś życie
        watchdog_kick(CONTROL_TASK_ID);
        
        task_sleep(1);  // 1 ms period
    }
}

// W watchdog task:
void watchdog_task(void) {
    while (1) {
        if (watchdog_expired(CONTROL_TASK_ID)) {
            // Zadanie nie zgłosiło się!
            // Może deadlock lub zawieszenie
            enter_safe_mode();
        }
        task_sleep(1);
    }
}
```

#### Detekcja cykli

Sprawdzanie grafów oczekiwania:

```c
// Uproszczony przykład detekcji
typedef struct {
    task_id_t waiting_for;
    task_id_t holding;
} resource_t;

bool detect_deadlock(void) {
    // Sprawdź czy istnieje cykl w grafie oczekiwania
    // Jeśli tak -> deadlock
    
    for (int i = 0; i < num_tasks; i++) {
        if (task[i].state == BLOCKED) {
            resource_t* r = find_resource(task[i].waiting_for);
            if (r && r->holder != NULL) {
                // Sprawdź czy holder czeka na zasób trzymany przez task[i]
                if (forms_cycle(r->holder, task[i])) {
                    return true;  // Deadlock!
                }
            }
        }
    }
    return false;
}
```

______________________________________________________________________

## 3. Protokoły synchronizacji

### 3.1. Priority Inheritance (Dziedziczenie priorytetów)

Gdy zadanie wysokiego priorytetu czeka na mutex trzymany przez zadanie niskie, to zadanie niskie tymczasowo otrzymuje wyższy priorytet.

```c
// W RTOS z priority inheritance:
void priority_inheritance_example(void) {
    // Zadanie L (niski priorytet = 5) pobiera mutex
    task_set_priority(task_L, 5);
    mutex_lock(&shared_mutex);  // L trzyma mutex
    
    // Zadanie H (wysoki priorytet = 20) chce mutex
    // System automatycznie podnosi priorytet L do 20
    task_set_priority(task_L, 20);  // Dziedziczenie!
    
    // Teraz L może szybciej zwolnić mutex
    mutex_unlock(&shared_mutex);
    task_set_priority(task_L, 5);  // Przywróć
}
```

**Zalety:**

- Automatyczne rozwiązanie problemu
- Proste w użyciu

**Wady:**

- Narzut na zarządzanie priorytetami
- Nie rozwiązuje wszystkich przypadków

### 3.2. Priority Ceiling Protocol (Protokół sufitu priorytetów)

Każdy mutex ma przypisany sufit priorytetu - najwyższy priorytet zadania, które może go użyć. Zadanie pobierające mutex działa z sufitowym priorytetem.

```c
// Definicja sufitu priorytetu
#define UART_CEILING 15
#define SPI_CEILING 12

void init_mutexes(void) {
    mutex_set_ceiling(&uart_mutex, UART_CEILING);
    mutex_set_ceiling(&spi_mutex, SPI_CEILING);
}

void uart_task(void) {
    // Zadanie z priorytetem 10 pobiera mutex
    // Automatycznie otrzymuje sufit = 15
    mutex_lock(&uart_mutex);  // sufit = 15
    
    // Jeśli inne zadanie (priorytet 14) chce ten mutex,
    // system blokuje je - bo 14 < 15
    
    mutex_unlock(&uart_mutex);  // Przywróć priorytet
}
```

______________________________________________________________________

## 4. Architektoniczne podejścia

### 4.1. Eliminacja współdzielonego stanu

Najlepsza obrona to brak współdzielonych zasobów.

#### Message Passing

Zamiast współdzielonych zmiennych:

```c
// ZŁE - współdzielony stan:
shared_data_t sensor_data;
mutex_lock(&data_mutex);
sensor_data = read_sensors();
mutex_unlock(&data_mutex);

// DOBRE - message passing:
queue_t sensor_queue;

void sensor_task(void) {
    data_t d = read_sensors();
    queue_push(&sensor_queue, &d);  // Kolejka zamiast mutexa
}

void control_task(void) {
    data_t d;
    if (queue_pop(&sensor_queue, &d, 10)) {  // Z timeoutem
        // Przetworz dane
    }
}
```

#### Własność zasobów (Ownership)

Jedno zadanie jest właścicielem danych:

```c
// Zadanie A - właściciel danych
void sensor_task(void) {
    data_t local_data = read_sensors();
    queue_push(&to_control, &local_data);  // Oddaj własność
}

// Zadanie B - konsument
void control_task(void) {
    data_t d;
    if (queue_pop(&to_control, &d, 10)) {
        // Teraz B jest właścicielem
        process(d);
    }
}
```

### 4.2. Izolacja partycji

Każda partycja ma własne zasoby:

```
Partycja A (sterowanie):
    - własne mutexy
    - własne kolejki
    - brak dostępu do zasobów partycji B

Partycja B (komunikacja):
    - własne mutexy
    - własne kolejki
    - brak dostępu do zasobów partycji A
```

### 4.3. Jednozadaniowy RT path

W ścieżce hard-RT nie ma współbieżności:

```c
// Tylko jeden task w torze RT - brak synchronizacji!
void motor_control_isr(void) {
    // Odczyt encoderów - bez mutex!
    int32_t enc = read_encoder();
    
    // Obliczenie PID - bez mutex!
    int32_t output = pid_calc(&pid, enc, setpoint);
    
    // Wyjście - bez mutex!
    set_pwm(output);
}
```

______________________________________________________________________

## 5. Praktyczne wzorce

### 5.1.Scoped Locking

Zawsze zwalniaj mutex, nawet przy błędzie:

```c
void funkcja(void) {
    mutex_lock(&m);
    
    if (warunek_bledu) {
        // Zwalnianie mutexa! Dzięki goto lub RAII
        mutex_unlock(&m);
        return ERROR;
    }
    
    mutex_unlock(&m);
}

// Lepsze - RAII w C++ lub makra w C:
#define SCOPED_LOCK(m) \
    for (int _locked = (mutex_lock(m), 1); _locked; _locked--, mutex_unlock(m))

void funkcja(void) {
    SCOPED_LOCK(&m) {
        // Operacje - mutex automatycznie zwolniony
    }
}
```

### 5.2. Double-Check Locking

Sprawdź warunek dwa razy:

```c
void get_resource(resource_t** out) {
    if (cache != NULL) {
        // Szybka ścieżka - bez lock
        *out = cache;
        return;
    }
    
    mutex_lock(&cache_mutex);
    if (cache == NULL) {  // Drugie sprawdzenie!
        cache = allocate_resource();
    }
    *out = cache;
    mutex_unlock(&cache_mutex);
}
```

### 5.3. Circuit Breaker

Po wielu błędach - przerwa obwód:

```c
#define MAX_RETRIES 3
#define RESET_TIME_MS 5000

int operation_with_retry(void) {
    static int failures = 0;
    static uint32_t last_failure = 0;
    
    if (failures >= MAX_RETRIES) {
        if (get_tick_ms() - last_failure > RESET_TIME_MS) {
            failures = 0;  // Spróbuj ponownie
        } else {
            return ERROR_CIRCUIT_OPEN;  // Blokada
        }
    }
    
    if (do_operation() != SUCCESS) {
        failures++;
        last_failure = get_tick_ms();
        return ERROR;
    }
    
    failures = 0;
    return SUCCESS;
}
```

______________________________________________________________________

## 6. Checklista anti-deadlock

### Na etapie projektowania:

- [ ] Zdefiniuj hierarchię wszystkich mutexów
- [ ] Sprawdź czy możesz użyć kolejek zamiast mutexów
- [ ] Określ maksymalny czas trzymania każdego mutexa
- [ ] Zaimplementuj watchdog dla krytycznych zadań
- [ ] Zaprojektuj safe mode

### Na etapie implementacji:

- [ ] Stosuj konsekwentną kolejność pobierania zasobów
- [ ] Używaj timeoutów na wszystkich blokadach
- [ ] Minimalizuj czas w sekcjach krytycznych
- [ ] Nie rób blokujących operacji z lockami
- [ ] Dodaj logging przy deadlock

### Na etapie testowania:

- [ ] Testy obciążeniowe
- [ ] Symulacja awarii
- [ ] Testy z timeoutami
- [ ] Testy z priorytetami

______________________________________________________________________

## 7. Podsumowanie

Obrona przed deadlockiem to wielo-poziomowa strategia:

| Poziom | Metoda | Kiedy stosować |
|--------|--------|----------------|
| Architektura | Message passing | Zawsze, gdy możliwe |
| Projektowanie | Hierarchia zasobów | Przy wielu mutexach |
| Implementacja | Time-outy | Zawsze |
| Runtime | Priority inheritance | Automatycznie |
| Awaria | Watchdog | Dla krytycznych systemów |

Najważniejsze zasady:

1. **Unikaj** współdzielonego stanu
1. **Porządkuj** kolejność pobierania zasobów
1. **Limituj** czas oczekiwania
1. **Monitoruj** stan systemu
1. **Planuj** awarie z góry

Nie ma jednego rozwiązania - kombinacja metod daje najlepszą ochronę.
