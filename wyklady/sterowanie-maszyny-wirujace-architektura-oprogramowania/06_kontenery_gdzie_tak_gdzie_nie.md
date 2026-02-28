# Wyklad 6: Kontenery (Docker) — gdzie tak, gdzie nie

## Czesc I: Wstep teoretyczny — czym sa kontenery i dlaczego sa popularne

### 1.1 Geneza kontenerow

W传统的 świecie IT, aby uruchomic aplikacje, potrzebowales:

1. **Fizyczny serwer** (lata 80-90)
2. **Maszyna wirtualna** (lata 2000+)
3. **Kontener** (lata 2013+)

Kontenery to "lekka wersja" maszyn wirtualnych:

```
Maszyna wirtualna:
+------------------+
|    Guest OS      |
+------------------+
|    Hypervisor    |
+------------------+
|    Host OS       |
+------------------+
|    Hardware      |
+------------------+

Kontener:
+------------------+
|   App + libs     |
+------------------+
|   Container      |
|   Runtime        |
+------------------+
|    Host OS       |
+------------------+
|    Hardware      |
+------------------+
```

Zalety:
- Lżejsze (MB zamiast GB)
- Szybsze uruchamianie (sekundy zamiast minut)
- Latwiejsze zarzadzanie zaleznosciami
- Latwiejsze skalowanie

### 1.2 w Docker praktyce

```dockerfile
# Dockerfile
FROM python:3.11

WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .
CMD ["python", "server.py"]
```

```bash
# Budowanie iie
docker build -t myapp .
docker run -p 800 uruchamian0:8000 myapp
```

### 1.3 Problem z RT

Ale kontenery nie sa stworzone do sterowania w czasie rzeczywistym!

```
Kontener domyslnie:
- Dzieli kernel z hostem
- Ma dostep do wszystkich CPU (bez izolacji)
- Moze byc zaplanowany na dowolnym rdzeniu
- Nie ma gwarancji latency
```

---

## Czesc II: Polityka per warstwa

### 2.1 Gdzie kontenery TAK

| Warstwa | Kontener? | Uzasadnienie |
|---------|-----------|--------------|
| W1 (MCU/RTOS) | NIE | Bezposredni dostep do sprzetu |
| W2 (RT master) | NIE dla petli RT | Wymaga izolacji CPU, determinizmu |
| W2 (narzedzia) | TAK | Build, monitoring (nie wplywa na RT) |
| W3 (HMI/nadzor) | TAK | GUI, API, logowanie |
| W4 (modelowanie) | TAK | Powtarzalnosc srodowiska |

### 2.2 W3: Nadzor w kontenerze

```yaml
# docker-compose.yml
version: '3.8'

services:
  hmi:
    build: ./hmi
    ports:
      - "8080:8080"
    volumes:
      - ./config:/config
    depends_on:
      - database

  database:
    image: postgres:15
    volumes:
      - telemetry_db:/var/lib/postgresql/data

  api:
    build: ./api
    ports:
      - "3000:3000"

volumes:
  telemetry_db:
```

To jest idealne zastosowanie kontenerow!

### 2.3 W4: Modelowanie w kontenerze

```dockerfile
# Python + MATLAB Runtime (julia podobnie)
FROM python:3.11-scipy

WORKDIR /workspace
COPY model.py .
COPY data/ ./data/

CMD ["python", "model.py", "--input", "/data/test.csv"]
```

Kontenery zapewniaja powtarzalnosc — "u mnie dziala, u ciebie tez bedzie".

---

## Czesc III: Gdzie kontenery NIE — hard RT

### 3.1 Problem: cgroups i izolacja

Kontenery domyslnie nie daja izolacji czasowej:

```bash
# Kontener bez limitow
docker run ubuntu:22.04

# Moze byc zaplanowany na dowolnym CPU
# Moze uzywac tyle RAM ile potrzebuje
# Moze byc wyrzucony (OOM killer)
```

Brak gwarancji!

### 3.2 Problem: host kernel

Wspoldzielony kernel = wspoldzielone opoznienia:

```
Host Linux:
+----------------------------------+
|  Kernel (IRQ handling)           |
+----------------------------------+
|  Kontener A  |  Kontener B      |
|  (non-RT)    |  (non-RT)       |
+----------------------------------+
|  Watek RT (poza kontenerem!)   |
+----------------------------------+
```

Watek RT na hoście = kontener moze byc zaplanowany "wokol" niego.

### 3.3 Problem: network I/O

Sieciowanie w kontenerze idzie przez virtualizacje:

```
Kontener:    send() 
                |
                v
    +------------------------+
    |   veth (virtual eth)   |
    +------------------------+
                |
                v
    +------------------------+
    |   bridge/switch        |
    +------------------------+
                |
                v
    +------------------------+
    |   physical eth         |
    +------------------------+
```

Kazda warstwa = opoznienie = jitter!

---

## Czesc IV: Najczestsze bledy

### 4.1 Blad 1: Uruchamianie petli RT w kontenerze

```yaml
# ZŁE:
services:
  rt_controller:
    image: rt_system:latest
    # Brak izolacji!
    # To NIE bedzie dzialac jako hard RT
```

NIGDY nie uruchamiaj hard-RT w kontenerze z domyslnymi cgroups!

### 4.2 Blad 2: Brak limitow na telemetrie

```yaml
# ZŁE:
services:
  telemetry:
    image: telemetry_logger:latest
    # Bez limitow - moze zzalac CPU!
```

Bez limitow — kontener moze zjesc cale CPU i wplynac na hosta.

### 4.3 Blad 3: Mieszanie IT i RT

```yaml
# ZŁE:
services:
  rt_controller:
    image: rt_system:latest
  web_server:
    image: nginx:latest
  # Wszystko na jednej maszynie bez izolacji!
```

Ruch IT moze wplywac na RT!

---

## Czesc V: Poprawna polityka

### 5.1 Podzial hostow

Najlepsza architektura:

```
Host A: RT (izolowany)
+----------------------------------+
| Linux PREEMPT_RT                 |
| Watki RT (SCHED_FIFO, pinning)  |
| Brak kontenerow                  |
| Izolacja IRQ                     |
+----------------------------------+

Host B: IT (kontenery)
+----------------------------------+
| Linux standard                   |
| Docker/Kubernetes                |
| HMI, API, DB, monitoring        |
+----------------------------------+
```

### 5.2 Komunikacja pomiedzy hostami

```
Host A (RT)          Host B (IT)
    |                    |
    | Shared Memory      |
    | (ethereal/fiber)   |
    |                    |
    v                    v
+-----------------+  +-----------------+
| Ring Buffer     |  | Consumer        |
| (non-blocking)  |  | (Python/Go)     |
+-----------------+  +-----------------+
```

### 5.3 Kontenery "obok" RT (jesli musi byc na tym samym hoście)

```yaml
# docker-compose.yml z izolacja
version: '3.8'

services:
  # RT controller - NIE w kontenerze!
  # To jest osobny proces na hoście
  
  # Nadzor - w kontenerze
  hmi:
    build: ./hmi
    # Izolacja CPU: tylko rdzenie 2-7
    cpuset_cpus: "2-7"
    # Limit CPU: 50%
    cpu_quota: 50000
    cpu_period: 100000
    # Limit pamieci
    mem_limit: 512m
    # Priorytet (mniejszy niz RT)
    cpu_shares: 1024
    
  database:
    image: postgres:15
    cpuset_cpus: "4-7"
    mem_limit: 1g
```

---

## Czesc VI: Izolacja i powtarzalnosc

### 6.1 Rate limiting telemetrii

Nawet jesli kontenery sa "obok" RT, trzeba ograniczyc ich wplyw:

```python
# Rate limiter jako sidecar
class RateLimiter:
    def __init__(self, max_rate_hz=100):
        self.max_interval = 1.0 / max_rate_hz
        self.last_sent = 0
    
    def send(self, data):
        now = time.time()
        if now - self.last_sent < self.max_interval:
            return False  # Drop
        self.last_sent = now
        return True
```

### 6.2 Monitoring resource usage

```yaml
# docker-compose z monitoringiem
services:
  hmi:
    image: hmi:latest
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 256M
```

### 6.3 Powtarzalnosc

Kontenery sa idealne dla:
- Wersjonowania srodowisk
- Reprodukcji bugow
- Deploymentu

Ale nie dla: hard-RT!

---

## Czesc VII: Kontekst autolabow (2035)

### 7.1 Architektura 2035

Autonomiczne laboratoria i linie produkcyjne zwykle:

- Wiele uslug (workflow, LIMS, bazy, dashboardy)
- Wymog powtarzalnosci srodowiska
- Audyt i zgodnosc (compliance)

### 7.2 Role kontenerow

Kontenery sa idealne dla:
- Warstwy nadzoru (W3)
- Workflow orchestration
- Bazy danych
- API i GUI

Ale:
- Musisz pilnowac, by nie ingerowaly w RT
- Oddzielne hosty lub rdzenie/priorytety
- Monitoring i rate limiting

---

## Czesc VIII: Podsumowanie i checklisty

### Checklisty:

- [ ] Petla RT dziala na hoście, na izolowanym rdzeniu
- [ ] Usługi HMI/logowania sa odseparowane i rate-limited
- [ ] Kontenery dla narzedzi (build, monitoring) nie wplywaja na RT

### Zasady:

| Zasada | Wyjasnienie |
|--------|-------------|
| RT poza kontenerem | Hard RT na hoscie |
| IT w kontenerze | HMI, DB, API |
| Izolacja | cgroups, cpuset |
| Rate limiting | Ograniczenie wplywu |

---

## Czesc IX: Pytania do dyskusji

1. Jakie ryzyko wnosi uruchomienie petli RT w kontenerze z domyslnymi cgroups?
2. Jak zaprojektujesz deployment tak, aby HMI/DB bylo powtarzalne, ale RT bylo stabilne?
3. Jak ograniczysz wplyw telemetrii (sieciowo i CPU) na host z petla RT?
4. Jak odtworzysz srodowisko (versions) bez zmiany charakterystyk czasu rzeczywistego?

---

## Czesc X: Zadania praktyczne

### Zadanie 1: Docker Compose Stack

Zbuduj docker-compose dla:
- HMI (web app)
- API (REST)
- Baza danych (PostgreSQL)
- Dokumentacja interfejsu do W2

### Zadanie 2: Rate Limiter

Zaimplementuj serwis rate-limitingu:
- Ogranicza strumien telemetryczny
- Mierzy dropy
- Raportuje metryki

### Zadanie 3: Deployment Split

Zaproponuj podzial na:
- Host RT (wydzielone CPU)
- Host IT (kontenery)
- Kontrakty danych pomiedzy nimi

---

## BONUS: Kontenery to narzedzie do powtarzalnosci

Jezeli ich uzycie pogarsza timing, to znaczy, ze probujesz konteneryzowac niewlasciwa warstwe.

---

*(Koniec wykladu 6)*
