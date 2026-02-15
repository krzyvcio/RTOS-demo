# Wykład: Robotyka roju — miliony umysłów, jedna misja

______________________________________________________________________

## Wprowadzenie: Kiedy mrówki pokonują słonia

Pamiętasz historię o tym, jak mrówki zniszczyły wielki telefon? Albo jak łańcuch pająków może unieść rower?

To nie metafory. To **biologia roju** - najpotężniejszy system na Ziemi, który przetrwał miliony lat.

W tym wykładzie dowiesz się jak:

- Zbudować armię tysięcy robotów współpracujących jak mrówki
- Stworzyć system odporny na awarie pojedynczych jednostek
- Zaprogramować zachowania emergentne - które "same" wynikają z prostych reguł
- I jak to połączyć z RTOS o którym właśnie się uczyliśmy

______________________________________________________________________

## 1. Rewolucja roju: Od mrówek do robotów

### Co to jest robotyka roju?

```
POJEDYNCZY ROBOT:
- Ograniczone możliwości
- Awaria = koniec misji
- Proste zadania

MILIONY ROBOTÓW:
- Emergentne zachowania
- Odporność na awarie
- Złożone zadania emergentnie
```

### Skąd czerpiemy inspirację?

| System biologiczny | Co inspiruje | Zastosowanie |
|-------------------|--------------|--------------|
| **Mrówki** | Podział pracy, stigmergia | Logistyka, optymalizacja |
| **Pszczoły** | Taniec informacyjny | Eksploracja, decyzje grupowe |
| **Ryb schooling** | Unikanie kolizji, flow | Formacje, transport |
| **Termity** | Budowanie bez architekta | Samoorganizacja |
| **Neurony** | Sieć decyzyjna | Rozproszone przetwarzanie |

______________________________________________________________________

## 2. Architektura roju - poziomy organizacji

### Poziom 1: Pojedynczy robot (agent)

```python
# Agent w roju - prosty, ale sprytny

class SwarmAgent:
    def __init__(self, agent_id):
        self.id = agent_id
        self.position = None
        self.velocity = None
        self.state = IDLE
        
        # Lokalne sensory
        self.sensors = {
            'proximity': ProximitySensor(),
            'communication': V2V无线电(),
            'battery': BatteryMonitor()
        }
        
        # Proste reguły zachowania
        self.behaviors = {
            'explore': ExploreBehavior(),
            'follow': FollowLeaderBehavior(),
            'avoid': CollisionAvoidance(),
            'return': ReturnToBaseBehavior()
        }
    
    def decide(self, local_view):
        """
        Każdy agent podejmuje decyzję lokalnie
        na podstawie lokalnych obserwacji
        """
        # Priorytetyzacja zachowań
        if self.sensors['proximity'].detect_obstacle():
            return self.behaviors['avoid'].compute()
        
        if self.battery_low():
            return self.behaviors['return'].compute()
        
        # Stigmergia - ślady feromonowe (wirtualne)
        if local_view.has_pheromones():
            return self.behaviors['follow'].compute()
        
        return self.behaviors['explore'].compute()
```

### Poziom 2: Komunikacja lokalna

```python
# Sieć typu mesh - każdy z każdym w zasięgu

class V2VCommunication:
    def __init__(self):
        self.range = 10  # metrów
        self.protocol = '802.11p'  # DSRC/V2X
        self.neighbors = []
        
    def broadcast(self, message):
        """
        Broadcast do wszystkich w zasięgu
        Ale nie do całego roju - tylko lokalnie!
        """
        for neighbor in self.get_neighbors_in_range():
            self.send(neighbor, message)
    
    def gossip(self, data, ttl=3):
        """
        Gossip protocol - rozprzestrzenianie plotek
        TTL = time to live - ile razy może być przesłane
        """
        if ttl <= 0:
            return
        
        # Wyślij do losowych sąsiadów
        for _ in range(3):
            neighbor = random.choice(self.get_neighbors_in_range())
            self.send(neighbor, {'data': data, 'ttl': ttl - 1})
```

### Poziom 3: Emergentna organizacja

```
                    ROJU
                      │
        ┌─────────────┼─────────────┐
        │             │             │
    CLUSTER A    CLUSTER B    CLUSTER C
        │             │             │
    ┌───┴───┐     ┌───┴───┐     ┌───┴───┐
    │ • • • │     │ • • • │     │ • • • │
    │ • • • │     │ • • • │     │ • • • │
    │ • • • │     │ • • • │     │ • • • │
    └───────┘     └───────┘     └───────┘
    
    Lokalne        Lokalne        Lokalne
    decyzje       decyzje       decyzje
        │             │             │
        └─────────────┼─────────────┘
                      │
              GLOBALNY wzór
           (emergentny!)
```

______________________________________________________________________

## 3. RTOS w robotyce roju - krytyczny element

### Dlaczego RTOS jest niezbędny?

W roju każdy robot musi:

- Reagować w **mikrosekundach** na zagrożenia
- Synchronizować się z sąsiadami
- Gwarantować czas komunikacji
- Być odpornym na zakłócenia

### Architektura RTOS dla agenta roju

```python
# Agent roju z RTOS - FreeRTOS

class SwarmAgentRTOS:
    def __init__(self):
        # Taski wysokiego priorytetu (Hard RT)
        self.tasks = {
            'safety': Task(
                func=self.safety_check,
                priority=10,
                period_ms=1,      # 1 kHz
                stack=512
            ),
            'comm_rx': Task(
                func=self.receive_messages,
                priority=9,
                period_ms=1,
                stack=512
            ),
            'navigation': Task(
                func=self.navigate,
                priority=5,
                period_ms=10,     # 100 Hz
                stack=1024
            ),
            'sensors': Task(
                func=self.read_sensors,
                priority=8,
                period_ms=5,      # 200 Hz
                stack=512
            ),
            'comm_tx': Task(
                func=self.transmit,
                priority=3,
                period_ms=20,     # 50 Hz
                stack=1024
            )
        }
        
        # Kolejki komunikacyjne
        self.queues = {
            'sensor_data': Queue(10),
            'neighbor_data': Queue(20),
            'commands': Queue(5)
        }
        
        # Mutexy dla współdzielonych zasobów
        self.position_mutex = Mutex()
    
    def safety_check(self):
        """
        Najwyższy priorytet - bezpieczeństwo
        """
        dist = self.proximity.read()
        
        if dist < SAFE_DISTANCE:
            # NATYCHMIAST zatrzymaj
            self.motors.emergency_stop()
            # Wyślij alarm do sąsiadów
            self.broadcast({
                'type': 'COLLISION_ALERT',
                'id': self.id,
                'position': self.position
            })
```

### Synchronizacja czasu w roju

```python
# Synchronizacja zegarów - kluczowa dla koordynacji

class TimeSynchronization:
    def __init__(self):
        self.local_time = 0
        self.offset = 0
        self.sync_interval = 100  # ms
        
    def sync_with_neighbors(self):
        """
        Synchronizacja przez wymianę timestampów
        Implementacja PTP-like
        """
        # Wyślij request
        t1 = self.get_hardware_timestamp()
        
        response = self.query_neighbor_time()
        
        t4 = self.get_hardware_timestamp()
        t2, t3 = response.timestamps  # odpowiedź
        
        # Oblicz offset
        delay = (t4 - t1 - (t3 - t2)) / 2
        offset = ((t2 - t1) - (t4 - t3)) / 2
        
        # Zaktualizuj
        self.offset = offset
        self.local_time = t4 + self.offset
```

______________________________________________________________________

## 4. Niesamowite zastosowania - wizja przyszłości

### 4.1. Self-Assembling Machines (maszyny samomontujące się)

```
WIZJA:

     ┌─────┐                    ┌─────────┐
     │ • • │  ──────────────→   │         │
     │ • • │    emergence      │ FORMA   │
     │ • • │                   │ ZDEFINIO-|
     └─────┘                   │ WANA    │
                               └─────────┘

Miliony mikrorobotów łączą się w dowolną formę!

ZASTOSOWANIA:
- Medycyna: nanoboty budujące struktury w ciele
- Kosmos: samomontujące się anteny satelitarne
- Budownictwo: roboty budujące domy bez ludzi
- Ratownictwo: formowanie się w mosty/pochwyty
```

```python
# Prosty algorytm samomontowania

class SelfAssembly:
    def __init__(self):
        self.target_shape = None
        
    def assemble(self, agents, target):
        """
        Zasada: lokalne decyzje → globalny kształt
        """
        # Każdy agent zna swoją rolę w kształcie
        for agent in agents:
            # Znajdź "sąsiada" w docelowym kształcie
            target_pos = target.get_position(agent.id)
            
            # Oblicz wektor do celu
            direction = target_pos - agent.position
            
            # Jeśli blisko - "przyłącz się"
            if distance(agent.position, target_pos) < BINDING_DISTANCE:
                agent.bind(target_pos)
            else:
                agent.move_toward(target_pos)
```

### 4.2. Living Architecture (żywa architektura)

```
WIZJA:

    ┌────────────────────────────────────────────┐
    │                                              │
    │    ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐      │
    │    │  │   │  │   │  │   │  │   │  │      │
    │    └──┘   └──┘   └──┘   └──┘   └──┘      │
    │                                              │
    │   Roboty-budowniczowie tworzący strukturę   │
    │   która ROŚNIE i ZMIENIA SIĘ               │
    │                                              │
    └────────────────────────────────────────────┘

ZASTOSOWANIA:
- Adaptacyjne budynki zmieniające kształt
- Samonaprawiające się mosty
- Struktury na Marsie budowane przez roje
- Ekipy ratunkowe: roboty odbudowujące ruiny
```

### 4.3. Underground Empire (podziemne imperium)

```
WIZJA:

        Powierzchnia
           ════
    ┌──────────────────┐
    │                  │
    │   ◉ ◉ ◉ ◉ ◉ ◉   │  ← Roboty-kret
    │  ◉ ◉ ◉ ◉ ◉ ◉ ◉  │     (termity cyfrowe)
    │  ◉ ◉ ◉ ◉ ◉ ◉ ◉  │
    │                  │
    └──────────────────┘

ZASTOSOWANIA:
- Drążenie tuneli na Marsie/Księżycu
- Układanie kabli pod ziemią
- Wyszukiwanie ocalałych w gruzach
- Kopanie schronów
- Wydobycie surowców
```

### 4.4. Ocean Intelligence (oceaniczna inteligencja)

```
WIZJA:

        Woda
    ═══════════════════
    
         🤖  🤖  🤖
       🤖   🤖  🤖  🤖
      🤖  🤖  🤖  🤖  🤖
     🤖  🤖  🤖  🤖  🤖  🤖
    
    Roje podwodne badające ocean!

ZASTOSOWANIA:
- Mapowanie dna oceanicznego
- Monitorowanie zanieczyszczeń
- Wczesne ostrzeżenia przed tsunami
- Sieć obserwacyjna klimatu
- Wyszukiwanie wraków/zaginionych
- Oczyszczanie oceanu z plastiku
```

### 4.5. Insect Cyborgs (cyborgi-owady)

```
WIZJA:

        ┌─────────┐
        │  OWAD   │  ← naturalny organizm
        │  + chip │
        │ + sensor│  ← cyfrowy interfejs
        │ + komun.│
        └─────────┘

HYBRYDY: Owady sterowane przez chipy!

ZASTOSOWANIA:
- Ratownictwo: znalezienie ocalałych w gruzach
- Wywiad: infiltracja terenów niedostępnych
- Rolnictwo: zapylanie roślin (gdy pszczoły wymrą)
- Ekologia: monitoring owadów zagrożonych
- Medycyna: mikro-roboty w ciele
```

### 4.6. Fog Computing Swarm (obliczeniowy rój)

```
WIZJA:

        ┌─────────┐
        │ Cloud   │  ← ciężkie obliczenia
        └────┬────┘
             │
    ┌────────┼────────┐
    │        │        │
    ▼        ▼        ▼
  ┌───┐   ┌───┐   ┌───┐
  │RT │   │RT │   │RT │  ← edge computing
  └───┘   └───┘   └───┘
    │        │        │
    ▼        ▼        ▼
  🤖🤖🤖   🤖🤖🤖   🤖🤖🤖
  
Rój to też rozproszony komputer!

ZASTOSOWANIA:
- Rozproszone przetwarzanie danych
-backup chmury na krawędzi
- AI inference na milionach urządzeń
- Blockchain/LEDGER rozproszony
```

### 4.7. Space Swarm (kosmiczny rój)

```
WIZJA:

        🌍
      ⚫  ●  ●
      ●  ●  ●
      ●  ●  ⚫

    Satelity-roboty formujące teleskopy/anteny

ZASTOSOWANIA:
- Teleskopy o milionowej rozdzielczości
- Solar sails napędzane przez rój
- Szybkie reagowanie na zagrożenia
- Samonaprawa satelitów
- Wydobycie asteroidowe
- Tarcze radiacyjne
```

______________________________________________________________________

## 5. Algorytmy roju - mózg zbiorowy

### 5.1. Particle Swarm Optimization (PSO)

```python
# Optymalizacja przez rój cząstek

class PSO:
    def __init__(self, n_particles, dimensions):
        self.particles = [
            Particle(dimensions) for _ in range(n_particles)
        ]
        self.global_best = None
        
    def optimize(self, objective_func, iterations):
        for _ in range(iterations):
            for particle in self.particles:
                # Ocena
                fitness = objective_func(particle.position)
                
                # Aktualizuj lokalne najlepsze
                if fitness > particle.best_fitness:
                    particle.best_fitness = fitness
                    particle.best_position = particle.position.copy()
                
                # Aktualizuj globalne najlepsze
                if fitness > (self.global_best or 0):
                    self.global_best = particle.position.copy()
                
                # Ruch - połączenie inercji, kognicji, socjalności
                r1, r2 = random.random(), random.random()
                
                particle.velocity = (
                    INERTIA * particle.velocity +
                    COG * r1 * (particle.best_position - particle.position) +
                    SOC * r2 * (self.global_best - particle.position)
                )
                
                particle.position += particle.velocity
        
        return self.global_best
```

### 5.2. Ant Colony Optimization (ACO)

```python# Optymalizacja mrowiskowa - jak mrówki znajdują najkrótszą drogę

class AntColony:
    def __init__(self, graph):
        self.graph = graph
        self.pheromones = defaultdict(lambda: 1.0)
        
    def run(self, n_ants, iterations):
        for _ in range(iterations):
            paths = []
            
            # Każda mrówka buduje ścieżkę
            for _ in range(n_ants):
                path = self.build_path()
                paths.append(path)
            
            # Oblicz długości
            for path in paths:
                length = self.path_length(path)
                
                # Odkładaj feromony (więcej na krótszych ścieżkach)
                pheromone_deposit = 1.0 / length
                
                for edge in path:
                    self.pheromones[edge] += pheromone_deposit
            
            # Parowanie feromonów
            for edge in self.pheromones:
                self.pheromones[edge] *= 0.5
        
        return self.best_path()
    
    def build_path(self):
        """Buduj ścieżkę probabilistycznie"""
        current = start
        path = [current]
        
        while current != goal:
            # Wybierz następny węzeł na podstawie feromonów
            neighbors = self.graph.get_neighbors(current)
            probs = [self.pheromones[(current, n)] for n in neighbors]
            
            # Roulette wheel selection
            current = random.choices(neighbors, weights=probs)[0]
            path.append(current)
        
        return path
```

______________________________________________________________________

## 6. Wyzwania i rozwiązania

### 6.1. Skalowanie

```
PROBLEM:
100 robotów = OK
1000 robotów = OK
1 000 000 robotów = 💥

Rozwiązania:
- Hierarchiczne klastry
- Agregacja informacji
- Ograniczona komunikacja
```

```python
# Agregacja - zamiast mówić wszystkim, mów liderom

class HierarchicalSwarm:
    def __init__(self, n_agents):
        # Podziel na klastry
        self.cluster_size = 100
        self.n_clusters = n_agents // self.cluster_size
        
        self.clusters = [
            Cluster(i, self.cluster_size) 
            for i in range(self.n_clusters)
        ]
        
    def propagate_decision(self, decision):
        # Liderzy-cluster podejmują decyzję
        for cluster in self.clusters:
            cluster.leader.decide(decision)
        
        # Liderzy rozsyłają do swoich członków
        for cluster in self.clusters:
            cluster.broadcast(decision)
```

### 6.2. Awarie

```
PROBLEM:
1 robot = awaria 1%
1000 robotów = 63% szansa że chociaż 1 się zepsuje

Rozwiązanie: Odporność na awarie!
```

```python
# Każdy robot ma "zastępcę"

class ResilientAgent:
    def __init__(self):
        self.backup = None
        self.state = ACTIVE
        
    def monitor_health(self):
        # Monitoruj sąsiadów
        neighbors = self.comm.get_neighbors()
        
        for neighbor in neighbors:
            if not neighbor.is_alive():
                # Aktywuj backup
                self.activate_backup(neighbor)
                
                # Poinformuj rój
                self.broadcast({
                    'type': 'AGENT_FAILED',
                    'id': neighbor.id,
                    'takeover': self.id
                })
```

______________________________________________________________________

## 7. Przyszłość: Hybrydy człowiek-roj

### Interfejs człowiek-roj

```python
class HumanSwarmInterface:
    def __init__(self):
        self.bci = BCIInterface()  # Z poprzedniego wykładu!
        self.swarm = None
        
    def control_with_thoughts(self):
        """
        Steruj rojem myślami!
        """
        # Odczytaj intencję z BCI
        intent = self.bci.decode_intention()
        
        # Mapuj na komendę roju
        command = {
            'FOCUS': lambda: self.swarm.converge(),
            'RELAX': lambda: self.swarm.disperse(),
            'LEFT': lambda: self.swarm.move(Direction.LEFT),
            'RIGHT': lambda: self.swarm.move(Direction.RIGHT),
            'SEARCH': lambda: self.swarm.search_pattern(),
            'RETURN': lambda: self.swarm.return_to_base()
        }
        
        # Wykonaj
        if intent in command:
            command[intent]()
```

______________________________________________________________________

## 8. Podsumowanie

### Kluczowe zasady robotyki roju

1. **Lokalne decyzje, globalny efekt** - proste reguły → złożone zachowania
1. **Brak centralnego mózgu** - każdy robot myśli lokalnie
1. **Stigmergia** - komunikacja przez środowisko
1. **Emergentność** - więcej niż suma części
1. **Odporność** - awaria pojedynczego = nic się nie dzieje

### RTOS w roju

- **Deterministyczny czas** - krytyczny dla synchronizacji
- **Niska latencja** - bezpieczeństwo w milisekundach
- **Przewidywalność** - gwarancja czasu reakcji

### Przyszłość

Rój to nie przyszłość - to teraźniejszość. Ale prawdziwa rewolucja dopiero nadejdzie gdy:

- Połączymi AI z rójem
- Dodamy interfejsy neuralne
- Zbudujemy miliardy robotów
- Pojedyncze jednostki będą nano-skala

______________________________________________________________________

## Pytania do dyskusji

1. Czy rój może być "świadomy"? Gdzie przebiega granica między emergentnością a inteligencją?
1. Kto odpowiada za błędy roju - twórca algorytmu czy operator?
1. Czy powinniśmy tworzyć miliardy tanich robotów jednorazowych?

______________________________________________________________________

## Źródła

- "Swarm Robotics" - Giovanni
- "Ant Colony Optimization" - Dorigo
- "Emergent Behavior in Artificial Swarm Intelligence"
- MIT's "Self-Assembling Robots" Project
