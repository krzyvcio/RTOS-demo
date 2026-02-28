# Wyklad 5: IPC miedzy warstwa 2 i 3 (shared memory + lock-free ring buffer)

## Czesc I: Wstep teoretyczny — problem komunikacji RT -> nonRT

### 1.1 Geneza problemu

Mamy dwa swiaty:

```
Warstwa 2 (RT Master):
- Czas wykonania: < 1ms
- Jitter: < 10 μs
- Deadline: twardy
- Co 1ms: odczyt -> sterowanie -> zapis

Warstwa 3 (Nadzor/GUI):
- Czas wykonania: > 10ms (dowolny)
- Jitter: nieistotny
- Deadline: zadny (lub bardzo duzy)
- Co 100ms: logowanie, GUI, siec
```

Problem: jak dane z W2 (szybka) dostarczyc do W3 (wolna) bez:
- Blokowania W2
- Utraty danych
- Wplywu na determinizm W2

### 1.2 Klasyczne rozwiazania i ich problemy

**Rozwiazanie 1: Kolejka z mutexem**

```c
// W2 (producer)
std::queue<Sample> queue;
std::mutex queue_mutex;

void rt_loop() {
    Sample s = collect_sample();
    
    // PROBLEM: mutex moze blokowac!
    std::lock_guard<std::mutex> lock(queue_mutex);
    queue.push(s);  // W2 czeka na mutex!
}
```

**Problem**: Priority Inversion
- W2 (wysoki priorytet) czeka na mutex
- W3 (niski priorytet) trzyma mutex
- W2 jest blokowany przez W3

**Rozwiazanie 2: Kolejka z lockem**

```python
# W3 (consumer)
while True:
    with lock:
        if queue:
            sample = queue.pop()
    # Cokolwiek
```

**Problem**: ciagle budzenie, ciagle lock/unlock, ryzyko contention.

**Rozwiazanie 3: Shared memory + lock-free**

```
W2 (Producer):                    W3 (Consumer):
                                 
    |---- write idx ---> |             |---- read idx ---> |
    |                    |             |                   |
    v                    v             v                   v
    +------+------+------+------+------+------+------+------+
    | S0   | S1   | S2   | S3   | S4   | S5   | S6   | S7   |
    +------+------+------+------+------+------+------+------+
    
    Napisane: [S0,S1,S2,S3,S4]    Odczytane: [S0,S1,S2]
    write_idx = 5                 read_idx = 3
```

Brak mutexow. Brak blokad. Tylko atomowe operacje.

---

## Czesc II: SPSC Ring Buffer — wzorzec

### 2.1 Definicja

**SPSC** = Single Producer Single Consumer

Dwa watki:
- Producer: watek RT (W2)
- Consumer: logger/GUI (W3)

Kazdy ma swoj wlasny wskaznik (indeks):
- `write_idx` — tylko producer modyfikuje
- `read_idx` — tylko consumer modyfikuje

### 2.2 Struktura danych

```c
// Stala definicja rozmiaru
#define RING_BUFFER_SIZE 1024

// Struktura probki (musi byc stala!)
struct __attribute__((packed)) Sample {
    uint64_t    timestamp_ns;   // 8 bytes
    float       omega_set;      // 4 bytes
    float       omega_meas;      // 4 bytes
    float       u_cmd;          // 4 bytes
    uint32_t    rt_loop_us;     // 4 bytes
    uint8_t     state;          // 1 byte
    uint8_t     fault_code;     // 1 byte
    uint8_t     padding[2];     // 2 bytes (alignment)
};  // Total: 28 bytes

// Struktura ring buffer (w shared memory!)
struct RingBuffer {
    // Atomowe indeksy
    _Atomic uint32_t write_idx;
    _Atomic uint32_t read_idx;
    
    // Dane
    Sample samples[RING_BUFFER_SIZE];
};
```

### 2.3 Operacje producenta (W2)

```c
// Producer: watek RT
void ringbuffer_write(RingBuffer* rb, const Sample* s) {
    uint32_t write_idx = atomic_load_explicit(&rb->write_idx, memory_order_relaxed);
    uint32_t next_idx = (write_idx + 1) % RING_BUFFER_SIZE;
    
    // Sprawdz przepełnienie
    uint32_t read_idx = atomic_load_explicit(&rb->read_idx, memory_order_acquire);
    
    if (next_idx == read_idx) {
        // Przepełnienie! Politika: drop lub overwrite
        // Tutaj: drop (kontrolowana utrata)
        return;  // Sample lost!
    }
    
    // Zapisz dane
    rb->samples[write_idx] = *s;
    
    // Memory barrier - zapis musi byc widoczny przed indeksem
    atomic_store_explicit(&rb->write_idx, next_idx, memory_order_release);
}
```

### 2.4 Operacje konsumenta (W3)

```c
// Consumer: watek non-RT
bool ringbuffer_read(RingBuffer* rb, Sample* s) {
    uint32_t read_idx = atomic_load_explicit(&rb->read_idx, memory_order_relaxed);
    uint32_t write_idx = atomic_load_explicit(&rb->write_idx, memory_order_acquire);
    
    if (read_idx == write_idx) {
        // Pusty buffer
        return false;
    }
    
    // Odczytaj dane
    *s = rb->samples[read_idx];
    
    // Zwieksz indeks
    uint32_t next_idx = (read_idx + 1) % RING_BUFFER_SIZE;
    atomic_store_explicit(&rb->read_idx, next_idx, memory_order_release);
    
    return true;
}
```

---

## Czesc III: Polityka przepełnienia

### 3.1 Dwa podejscia

| Polityka | Opis | Zalety | Wady |
|----------|------|--------|------|
| **Drop** | Gub najnowsza probe | RT nie cierpi | Tracimy dane |
| **Overwrite** | Nadpisz najstarsza probe | Zawsze mamy dane | Tracimy historie |

### 3.2 Kiedy ktora polityka

**Drop** — dla danych krytycznych:
- Dane do analizy (logi)
- Diagnostyka (FFT, trendy)

```c
// Drop z licznikiem
if (next_idx == read_idx) {
    atomic_fetch_add(&rb->drop_count, 1);
    return;
}
```

**Overwrite** — dla danych "live":
- Aktualny stan do GUI
- Telemetria "current value"

```c
// Overwrite - zawsze zapisz
rb->samples[write_idx] = *s;
atomic_store_explicit(&rb->write_idx, next_idx, memory_order_release);
```

### 3.3 Praktyczny wybor

W systemach sterowania zwykle:

- **Overwrite** dla telemetrii "live" (do GUI)
- **Drop + licznik** dla logow "do analizy"

```c
// Dwa ring buffer!
RingBuffer live_rb;     // Overwrite, dla GUI
RingBuffer log_rb;      // Drop + counter, dla pliku
```

---

## Czesc IV: Shared Memory

### 4.1 Pojecie

Shared memory (pamiec dzielona) to mechanizm, gdzie dwa procesy (lub watki) dziela ten sam obszar pamieci:

```
Proces A:              Pamiec dzielona:        Proces B:
                      +----------------+
0x1000: data -------->|                |<------- 0x2000: data
                      +----------------+
```

### 4.2 Implementacja w Linux

```c
// Tworzenie shared memory
int shm_fd = shm_open("/rt_telemetry", O_CREAT | O_RDWR, 0666);
ftruncate(shm_fd, sizeof(RingBuffer));
RingBuffer* rb = mmap(NULL, sizeof(RingBuffer), PROT_READ | PROT_WRITE,
                      MAP_SHARED, shm_fd, 0);

// Inicjalizacja
atomic_init(&rb->write_idx, 0);
atomic_init(&rb->read_idx, 0);

// W2 (producer)
ringbuffer_write(rb, &sample);

// W3 (consumer) - w innym procesie!
RingBuffer* rb = mmap(NULL, sizeof(RingBuffer), PROT_READ,
                      MAP_SHARED, shm_fd, 0);

// Odczyt
Sample s;
if (ringbuffer_read(rb, &s)) {
    process_sample(s);
}
```

### 4.3 Python dostep do shared memory

```python
import mmap
import struct

# Otworz shared memory
shm = mmap.mmap(shm_fd, mmap.PAGESIZE, access=mmap.ACCESS_READ)

# Odczyt
def read_sample():
    # Odczytaj write_idx
    shm.seek(0)
    write_idx = struct.unpack('I', shm.read(4))[0]
    shm.seek(8)
    read_idx = struct.unpack('I', shm.read(4))[0]
    
    if write_idx == read_idx:
        return None
    
    # Odczytaj probke
    offset = 8 + (read_idx * 28)  # 28 = rozmiar Sample
    shm.seek(offset)
    data = shm.read(28)
    
    # Parsuj
    sample = struct.unpack('I Q f f f I B B', data)
    return sample
```

---

## Czesc V: Backpressure — ochrona przedzalewem

### 5.1 Problem

Co jesli consumer (W3) jest za wolny?

```
W2: [1][2][3][4][5][6][7][8][9] -> (ciagle produkuje)
W3: [1][2]       (ciagle czyta)
```

Dwa rozwiazania:

1. **Rate limiting producenta** — W2 zwalnia
2. **Drop** — W2 gubi dane (ale nie blokuje!)

### 5.2 Rate limiting w W3 (bez wplywu na W2)

```python
# Consumer - rate limiting siebie, nie producenta
class RateLimitedConsumer:
    def __init__(self, max_rate_hz=100):
        self.min_interval = 1.0 / max_rate_hz
        self.last_read = 0
    
    def read(self):
        now = time.time()
        elapsed = now - self.last_read
        
        if elapsed < self.min_interval:
            time.sleep(self.min_interval - elapsed)
        
        sample = ringbuffer_read(&rb)
        self.last_read = now
        return sample
```

### 5.3 Jak zapewnic brak backpressure

Kluczowa zasada: **producer nigdy nie czeka na consumera!**

```c
// DOBRE: nieblokujacy write
void write_never_blocks() {
    // Sprawdz przepełnienie
    if (buffer_full()) {
        drop_sample();  // Gubi, ale nie blokuje
        return;
    }
    
    // Zapisz
    buffer[write_idx] = sample;
    write_idx++;
}

// ZŁE: blokujacy write
void write_blocks() {
    while (buffer_full()) {
        // Czekaj na consumer - BLOKUJE PRODUCERA!
        sleep(1);
    }
    buffer[write_idx] = sample;
    write_idx++;
}
```

---

## Czesc VI: Kontekst — dane sensoryczne

### 6.1 Kaskady buforow

W robotyce mamy wiele strumieni danych:

- IMU (akcelerometr + zyroskop): 1 kHz
- Enkodery: 10-100 kHz
- Czujniki sily: 1-10 kHz
- Kamery: 30-120 Hz

W praktyce robi sie:
- Osobny ring buffer per strumien (z roznym rozmiarem)
- Priorytety odczytu (sterowanie bierze tylko to, co musi)
- Agregacja w non-RT (logi, dashboard, analiza)

### 6.2 Zasada

> RT konsumuje tylko "esencje", reszta jest opcjonalna.

```c
// Przyklad: RT potrzebuje tylko predkosci
// Non-RT moze miec pelny zestaw danych

// W2: tylko niezbedne
struct RTData {
    float omega;
    float u_cmd;
    uint8_t fault;
};

// W3: pelne dane
struct FullTelemetry {
    float omega;
    float omega_raw;
    float temperature;
    float vibration;
    float u_cmd;
    uint64_t timestamps[100];  // historia
    // ... wiecej
};
```

---

## Czesc VII: Podsumowanie i checklisty

### Checklisty:

- [ ] Watek RT nie czeka na consumer
- [ ] Masz licznik drop/overwrite i go logujesz
- [ ] Format probki jest staly i wersjonowany
- [ ] Shared memory jest poprawnie skonfigurowana (rozmiar, permissions)

### Zasady SPSC:

| Zasada | Wyjasnienie |
|--------|-------------|
| Atomowe indeksy | Tylko atom_store/load |
| Memory barriers | Wlasciwe memory_order |
| Nie blokuj | Politika drop lub overwrite |
| Liczniki dropow | Monitoring zdrowia systemu |

---

## Czesc VIII: Pytania do dyskusji

1. Kiedy SPSC ring buffer wystarcza, a kiedy potrzebujesz MPSC (i dlaczego to trudniejsze)?
2. Co wybierasz: drop czy overwrite? Jak to uzasadniasz dla telemetrii live i logow do analizy?
3. Jak zapewnisz, ze consumer non-RT nie wywola backpressure na RT?
4. Jak mierzysz i raportujesz gubienie probek (drop counters)?

---

## Czesc IX: Zadania praktyczne

### Zadanie 1: Implementacja SPSC

Zaimplementuj SPSC ring buffer:
- Atomowe indeksy
- Politika drop z licznikiem
- Testy obciazeniowe (producer 10kHz, consumer 10Hz)

### Zadanie 2: Shared memory demo

Zaimplementuj:
- Producer w C (watek RT)
- Consumer w Python
- Komunikacja przez shared memory
- Eksport do CSV

### Zadanie 3: Backpressure study

Przeprowadz eksperyment:
- Producer 1 kHz
- Consumer 10 Hz, 1 Hz, 0.1 Hz
- Mierz drop counter
- Demonstracja roznych polityk przepełnienia

---

## BONUS: Lock-free to nie fancy

Dla wielu zespolow przełomem jest moment, gdy przestaja uzywac mutexow do telemetrii.

Lock-free to nie "fancy" — to najprostsza droga do przewidywalnosci.

---

*(Koniec wykladu 5)*
