# Wyklad 4: Jezyki i interfejsy (wielojezycznosc bez chaosu)

## Czesc I: Wstep teoretyczny — dlaczego wielojezycznosc jest wyzwaniem

### 1.1 Problem "wielu jezykow"

W nowoczesnym systemie sterowania uzywamy wielu jezykow programowania:

- **C** — sterowniki, niskopoziomowe operacje
- **C++** — logika biznesowa, algorytmy
- **Rust** — bezpieczenstwo pamieci, nowe projekty
- **Python** — prototypowanie, analiza danych, ML
- **JavaScript** — GUI webowe

Kazdy z nich ma inne:
- Model pamieci (stack, heap, garbage collection)
- Szybkosc wykonania (kompilowany vs interpretowany)
- Narzedzia (debugger, profiler, IDE)
- Ekosystem (biblioteki)

### 1.2 Geneza problemu GC

Prosze wyobrazic sobie taki kod:

```python
# Python - wyglada niewinnie
def control_loop():
    data = process(sensor_data)  # Alokuje obiekt
    result = compute(data)        # Alokuje wynik
    return result                 # Obiekty "opuszczaja" funkcje
```

Po kilku sekundach:
```
Python runtime: "Mam za duzo obiektow, musze posprzatac"
GC (Garbage Collector): zatrzymuje caly watek na 50ms
```

**Efekt**: petla sterowania, ktora miala 1ms, nagle ma 51ms. Deadline przekroczony.

To nie jest hipotetyczny scenariusz — to prawdziwy problem, z ktorym spotykaja sie inzynierowie.

### 1.3 Model pamieci — wizualizacja

```
C/C++/Rust (bez GC):
    
    Stack:          Heap:
    +-----+         +------+
    | a=5 | ----->  | dane |
    +-----+         +------+
    | b=10| 
    +-----+
    
    - Alokacja: stala (compile-time)
    - Czas: deterministyczny
    - Ryzyko: wycieki pamieci (w C), ale nie GC pause


Python/Java/JavaScript (z GC):

    Stack:          Heap (z fragmentacja!):
    +-----+         +------+  +------+  +------+
    | ref | ----->  | obj1 |  | obj2 |  | obj3 |
    +-----+         +------+  +------+  +------+
                          \      |      /
                           +-----+------+
                           | GC tracking |
                           +-------------+
                           
    - Alokacja: dynamiczna
    - Czas: NIEdeterministyczny (GC)
    - Ryzyko: GC pause (i katastrofy przy wyciekach)
```

---

## Czesc II: Zasada — jezyk wynika z warstwy

### 2.1 Mapowanie warstw na jezyki

| Warstwa | Jezyk | Uzasadnienie |
|---------|-------|--------------|
| W1 (MCU/drive) | C lub Rust `no_std` | Bez GC, minimalny overhead |
| W2 (RT master) | C/C++ lub Rust | Wydajnosc, kontrola pamieci |
| W3 (nadzor) | Python/Node/Go | Szybki rozwoj, ekosystem |
| W4 (model) | Python/MATLAB/Julia | NLP, symulacja |

### 2.2 Warstwa 1: C lub Rust (no_std)

```rust
// Rust no_std dla MCU
#![no_std]

// Bez alokacji sterty
use core::alloc::GlobalAlloc;

// Prosty kod bez GC
fn compute_pid(error: f32, kp: f32, ki: f32, dt: f32) -> f32 {
    // Tylko zmienne na stosie - deterministyczne
    let p = kp * error;
    let i = ki * error * dt;
    p + i
}
```

Zalety Rust:
- Bezpieczenstwo pamieci (bez wyciekow)
- Brak GC
- Nowoczesny jezyk

Wady:
- Wyzszy próg wejscia
- Mniejszy ekosystem bibliotek niskopoziomowych

### 2.3 Warstwa 2: C/C++ lub Rust

```cpp
// C++ dla mastera RT
class Controller {
private:
    float kp_ = 1.0f;
    float ki_ = 0.1f;
    float error_integral_ = 0.0f;
    
public:
    // Zero alokacji w petli!
    float compute(float setpoint, float measurement, float dt) {
        float error = setpoint - measurement;
        
        // Calkowanie z anti-windup
        error_integral_ += error * dt;
        error_integral_ = clamp(error_integral_, -1.0f, 1.0f);
        
        return kp_ * error + ki_ * error_integral_;
    }
};
```

Zasady:
- Statyczna alokacja pamieci
- Brak wywolan do bibliotek z GC
- Proste struktury danych (tablice, nie wektory)

### 2.4 Warstwa 3: Python/Node/Go

```python
# Python dla nadzoru - tutaj GC jest OK
import sqlite3
import json

class Supervisor:
    def __init__(self):
        # Alokacje w konstruktorze - nie w petli!
        self.db = sqlite3.connect('telemetry.db')
        
    def log_sample(self, sample):
        # Wolne operacje - nie wplywaja na RT
        json_str = json.dumps(sample.to_dict())
        self.db.execute("INSERT INTO samples VALUES (?)", (json_str,))
        
    def update_gui(self):
        # GUI update - moze byc powolne
        self.window.set_speed(sample.omega)
```

Zasady:
- GC jest dozwolony (wolniejsza warstwa)
- Operacje I/O dowolne
- GUI, bazy danych, siec — wszystko tu

---

## Czesc III: Kontrakty danych — wersjonuj i stabilizuj

### 3.1 Problem "wymiany structow"

Bardzo czesty błąd:

```c
// W2 (C)
struct Sample {
    float omega;
    float u;
};

// W3 (Python) - "wiemy, ze to float, nie ma problemu"

...po 6 miesiacach...

// W2 (C) - zmieniono na double
struct Sample {
    double omega;  // Zmiana typu!
    float u;
};

// W3 (Python) - czyta jako float
omega = struct.unpack('f', data[0:4])[0]  // BŁĄD! Odczytuje bity double jako float
```

**Efekt**: błędne odczyty, trudne do zdebugowania.

### 3.2 Rozwiazanie: wersjonowanie schematu

```c
// W2 (C) - wersjonowany schemat
#define SCHEMA_VERSION 2

struct SampleV2 {
    uint32_t    schema_version;  // = 2
    uint64_t    timestamp_ns;
    float       omega_set;
    float       omega_meas;
    float       u_cmd;
    uint8_t     state;
    uint8_t     fault_code;
};

// W3 (Python)
SAMPLE_V2_FORMAT = 'I Q f f f B B'  # schema, timestamp, omega_set, omega_meas, u, state, fault
SAMPLE_V2_SIZE = struct.calcsize(SAMPLE_V2_FORMAT)  # = 24 bytes

def parse_sample(data):
    if len(data) < 4:
        return None
    
    version = struct.unpack('I', data[0:4])[0]
    
    if version == 1:
        return parse_sample_v1(data)
    elif version == 2:
        return parse_sample_v2(data)
    else:
        log_warning(f"Unknown version {version}")
        return None
```

### 3.3 Zasady wersjonowania

1. **Kazdy komunikat ma numer wersji**
2. **Nowe wersje sa kompatybilne wstecz** (dodawane pola na koniec)
3. **Stare pola nie sa usuwane** (lub sa jawnie oznaczone jako deprecated)
4. **Rozmiar ramki jest staly** (lub jawnie z komunikatem dlugosci)

```c
// DOBRA praktyka: stale rozmairy
struct __attribute__((packed)) SampleV2 {
    uint32_t    schema_version;  // v2 = 2
    uint64_t    timestamp_ns;
    float       omega_set;       // zadana predkosc
    float       omega_meas;      // zmierzona predkosc
    float       u_cmd;           // sterowanie
    uint8_t     state;           // stan FSM
    uint8_t     fault_code;      // kod bledu
};  // Rozmiar: 4+8+4+4+4+1+1 = 26 bytes (pad do 28)

// NIEZLA praktyka: zmienne rozmiary z naglowkiem
struct VariableMessage {
    uint32_t    schema_version;
    uint32_t    payload_length;  // rozmiar payload
    uint8_t     payload[];       // dane
};
```

---

## Czesc IV: Bindingi i biblioteki

### 4.1 Problem FFI

FFI (Foreign Function Interface) to mechanizm wywolywania kodu z jednego jezyka w innym:

```c
// C - biblioteka EtherCAT
int ec_send_processdata(uint16_t index, void* data);
int ec_receive_processdata(uint16_t index, void* data);
```

```python
# Python - chcemy uzyc biblioteki C
# Rozwiazanie: ctypes albo Cython
```

### 4.2 Bezpieczna granica FFI

Kluczowa zasada:

> Watek RT jest w C/C++/Rust. Warstwa W3 dostaje dane przez IPC, nie przez wywolania do watku RT.

```c
// NIEBEZPIECZNE:
// Python wywoluje C w petli RT
void rt_loop() {
    // Python callback - NIGDY w petli RT!
    python_callback(data);  // GC moze sie uruchomic!
}

// BEZPIECZNE:
// Komunikacja przez shared memory / ring buffer
void rt_loop() {
    // Zapis do ring buffer - nieblokujacy
    ringbuffer_write(&rb, sample);
}

// W3 - osobny watek, czyta kiedy chce
void logger_loop() {
    Sample* s = ringbuffer_read(&rb);
    if (s) {
        python_process(s);  // Tu GC jest OK
    }
}
```

### 4.3 Bindingi — przyklad praktyczny

```python
# Python - binding do biblioteki C

import ctypes
import mmap
import struct

# 1. Ladowanie biblioteki
lib = ctypes.CDLL('./librt_controller.so')

# 2. Mapowanie pamieci dzielonej
shm = mmap.mmap(0, 4096, "rt_shm", mmap.ACCESS_READ)
ringbuffer_ptr = ctypes.c_void_p.from_buffer(shm)

# 3. Odczyt danych (bez wywolywania C!)
def read_sample():
    # Samodzielne parsowanie - Python poza petla RT
    data = shm.read(28)  # rozmiar SampleV2
    return struct.unpack('I Q f f f B B', data)

# 4. Tylko "glosowanie" - RT watek nigdy nie wywoluje Python!
while True:
    sample = read_sample()  # Czyta z dzielonej pamieci
    if sample:
        update_display(sample)
    sleep(0.1)  # 10 Hz - wystarczy dla GUI
```

---

## Czesc V: Middleware i integracja

### 5.1 Middleware w systemach robotycznych

W systemach robotycznych czesto uzywa sie middleware:
- ROS/ROS2 (robot operating system)
- DDS (Data Distribution Service)
- ZeroMQ, nanomsg

### 5.2 Zasada "middleware nie wchodzi do RT"

```python
# ŹLE: Middleware w petli RT
def rt_loop():
    # Petla RT
    data = collect_data()
    
    # Wysylka przez ROS2 - NIE!
    publisher.publish(data)  # Moze blokowac!
```

```python
# DOBRE: Middleware poza petla RT
def rt_loop():
    data = collect_data()
    ringbuffer_write(&rb, data)  # Non-blocking

def publisher_loop():
    while True:
        data = ringbuffer_read(&rb)
        if data:
            publisher.publish(data)  # Middleware tu - OK
        sleep(0.001)  # 1 kHz publish
```

### 5.3 Praktyczny wniosek

> Nawet jesli "wszystko jest w jednym frameworku" (np. ROS2), petla RT musi miec pierwszenstwo i izolacje.

---

## Czesc VI: Podsumowanie i checklisty

### Zasady wielojezycznosci:

| Zasada | Wyjasnienie |
|--------|-------------|
| GC tylko w wyzszych warstwach | W1/W2 = bez GC |
| Kontrakty sa wersjonowane | Kazda zmiana jest jawna |
| FFI jest asynchroniczny | RT nie wywoluje Python/JS |
| Stale rozmiary ramek | Latwe parsowanie |

### Checklisty:

- [ ] Watek RT nie uzywa jezyka z GC
- [ ] API pomiedzy warstwami ma wersje i testy kompatybilnosci
- [ ] Komunikacja jest asynchroniczna (ring buffer / shared memory)
- [ ] Rozmiar ramki jest staly lub z naglowkiem dlugosci

---

## Czesc VII: Pytania do dyskusji

1. Dlaczego GC w sciezce RT jest ryzykiem i w jakiej warstwie moze byc bezpiecznie uzyty?
2. Jak wersjonujesz kontrakt danych, zeby W3/W4 nie psuly W2 po aktualizacji?
3. Jak zrobisz binding do biblioteki C, zeby nie wpuscic jej w sciezke RT niekontrolowanym sposobem?
4. Jakie dane powinny byc "stale rozmiarowo", a jakie moga byc zmienne (i dlaczego)?

---

## Czesc VIII: Zadania praktyczne

### Zadanie 1: Migracja kontraktu

Masz wersje 1 schematu:
```
struct SampleV1 {
    uint32_t version;
    float omega;
    float u;
};
```

Zaprojektuj wersje 2, ktora dodaje temperature i kod bledu, zachowujac kompatybilnosc wsteczna.

### Zadanie 2: Granica FFI

Zaimplementuj prototyp:
- C/C++: watek RT zapisujacy do shared memory
- Python: watek czytajacy z shared memory
- Demonstracja, ze Python nie blokuje RT

### Zadanie 3: Protokol IPC

Zaprojektuj prosty protokol komunikacji:
- Stale rozmiary ramek
- Wersjonowanie
- Walidacja (checksum)
- Liczniki dropow

---

## BONUS: Twarde granice

> Najlepsza wielojezycznosc to taka, ktora ma twarde granice: watek RT nie zna JSON, HTTP ani DB; on zna tylko strukture probki i licznik czasu.

---

*(Koniec wykladu 4)*
