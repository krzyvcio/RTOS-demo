# Wykład: Digital Twin — gdy fizyka spotyka cyfrę

______________________________________________________________________

## Wprowadzenie: Po co nam cyfrowy bliźniak?

Wyobraź sobie że możesz:

- Przetestować nowy algorytm na cyfrowym robocie przed wgraniem na prawdziwego
- Symulować awarię bez niszczenia sprzętu
- Przewidzieć kiedy robot się zepsuje zanim do tego dojdzie
- Trenować AI na milionach symulowanych scenariuszy

To właśnie robi **Digital Twin** — wirtualna kopia fizycznego systemu.

W tym wykładzie dowiesz się:

- Czym jest Digital Twin i jak działa
- Jak go zbudować dla systemów robotycznych
- Jakie są pułapki i jak ich unikać
- Jak używać go w praktyce

______________________________________________________________________

## 1. Czym jest Digital Twin?

### Definicja

Digital Twin to **wirtualna kopia** fizycznego systemu, która:

- Działa w czasie rzeczywistym (lub zbliżonym)
- Jest połączona z systemem fizycznym (data flow)
- Może przewidywać zachowanie systemu
- Pozwala na eksperymenty bez ryzyka

### Analogia

```
┌─────────────────┐         ┌─────────────────┐
│   RZECZYWISTY   │ ←─────→ │   DIGITAL TWIN  │
│                 │  dane   │                 │
│  [ Robot #42 ]  │         │  [ Model 3D ]   │
│                 │         │  [ Sim fizyki ] │
│  - temp: 45°C  │         │  - temp: 45°C   │
│  - vib: 0.1g   │         │  - vib: 0.1g   │
│  - wear: 23%   │         │  - wear: 23%   │
└─────────────────┘         └─────────────────┘
        ↓                         ↓
    fizyczne dane          predykcje / symulacje
```

### Typy Digital Twin

| Typ | Opis | Przykład |
|-----|------|----------|
| **Shadow** | Obserwuje, nie steruje | Monitoring stanu |
| **Interactive** | Dwukierunkowa komunikacja | Zdalne sterowanie |
| **Predictive** | Predykuje przyszłość | Predictive maintenance |
| **Autonomous** | Działa autonomicznie | Self-optimizing |

______________________________________________________________________

## 2. Architektura Digital Twin

### Warstwy systemu

```
┌─────────────────────────────────────────────────────────┐
│                  DIGITAL TWIN PLATFORM                   │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────┐  │
│  │  Dashboard   │    │  Analytics   │    │   AI/ML  │  │
│  │  (wizualiz.)│    │   (raporty)  │    │ (predykcje│  │
│  └──────────────┘    └──────────────┘    └──────────┘  │
│           ↓                  ↓                 ↓         │
│  ┌─────────────────────────────────────────────────────┐│
│  │              SIMULATION ENGINE                      ││
│  │  - Physics solver                                   ││
│  │  - FEM/CFD                                         ││
│  │  - Rigid body dynamics                             ││
│  │  - Control simulation                              ││
│  └─────────────────────────────────────────────────────┘│
│                         ↓                                │
│  ┌─────────────────────────────────────────────────────┐│
│  │              DATA LAYER                             ││
│  │  - Time-series DB (InfluxDB)                       ││
│  │  - 3D models (CAD/BIM)                            ││
│  │  - Asset metadata                                  ││
│  └─────────────────────────────────────────────────────┘│
│                                                          │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│                  PHYSICAL SYSTEM                         │
│                                                          │
│  [Sensory] → [Gateway] → [Cloud/Edge]                  │
│       ↓              ↓              ↓                    │
│  temp, vibr.    filtered      aggregated                 │
└─────────────────────────────────────────────────────────┘
```

### Przepływ danych

```python
class DigitalTwin:
    def __init__(self, robot_id):
        self.robot_id = robot_id
        
        # Połącz z fizycznym robotem
        self.robot = RobotGateway(robot_id)
        
        # Modele symulacyjne
        self.physics = PhysicsModel()
        self.control = ControlModel()
        self.degradation = DegradationModel()
        
        # Baza danych
        self.timeseries = InfluxDBClient()
        
    def sync(self):
        """Synchronizacja z fizycznym robotem"""
        
        # 1. Pobierz dane z fizycznego robota
        sensor_data = self.robot.get_current_state()
        
        # 2. Zapisz do bazy
        self.timeseries.write(sensor_data)
        
        # 3. Zaktualizuj model
        self.physics.update(sensor_data)
        
        # 4. Sprawdź różnicę (digital vs real)
        discrepancy = self.check_discrepancy()
        
        if discrepancy > THRESHOLD:
            self.alert(f"Model drift: {discrepancy}%")
            self.recalibrate()
    
    def predict_maintenance(self):
        """Przewidywanie awarii"""
        
        # Pobierz dane historyczne
        history = self.timeseries.query(
            f"SELECT * FROM sensors WHERE robot_id='{self.robot_id}' "
            f"AND time > now() - 30d"
        )
        
        # Model degradacji
        wear = self.degradation.predict(history)
        
        # Predykcja
        remaining_life = self.degradation.estimate_remaining_life(wear)
        
        return {
            "current_wear": wear,
            "predicted_failure": remaining_life < 30 days,
            "days_until_failure": remaining_life
        }
```

______________________________________________________________________

## 3. Budowa Digital Twin dla robota

### 3.1. Model fizyczny

```python
# Model dynamiki robota mobilnego

class RobotPhysicsModel:
    def __init__(self, mass, inertia, wheel_radius):
        self.m = mass          # masa [kg]
        self.I = inertia       # moment bezwładności
        self.r = wheel_radius # promień koła
        
        # Parametry tarcia
        self.viscous_friction = 0.1
        self.coulomb_friction = 0.5
        
    def dynamics(self, state, control):
        """
        Równania dynamiki:
        M(q)q̈ + C(q,q̇)q̇ = τ
        """
        q = state.position      # [x, y, theta]
        q_dot = state.velocity  # [vx, vy, omega]
        tau = control           # [tau_l, tau_r]
        
        # Kinematyka różnicowa
        v = self.r * (tau[0] + tau[1]) / 2
        omega = self.r * (tau[1] - tau[0]) / self.wheel_base
        
        # Siły tarcia
        F_friction = -self.viscous_friction * q_dot
        F_coulomb = -self.coulomb_friction * np.sign(q_dot)
        
        # Przyspieszenie
        acceleration = (tau - F_friction - F_coulomb) / self.m
        
        # Nowy stan
        new_q_dot = q_dot + acceleration * self.dt
        new_q = q + new_q_dot * self.dt
        
        return State(new_q, new_q_dot)
    
    def simulate(self, controls, dt, steps):
        """Symulacja trajektorii"""
        state = initial_state
        
        for _ in range(steps):
            state = self.dynamics(state, controls)
            
        return state
```

### 3.2. Model termiczny

```python
# Model termiczny silnika

class ThermalModel:
    def __init__(self, thermal_resistance, thermal_capacity):
        self.R = thermal_resistance  # K/W
        self.C = thermal_capacity    # J/K
        
        self.ambient_temp = 25  # temperatura otoczenia
        self.current_temp = 25
        
    def update(self, power, dt):
        """
        Równanie ciepła:
        C * dT/dt = P - (T - Ta)/R
        """
        # Strat na obudowie
        heat_loss = (self.current_temp - self.ambient_temp) / self.R
        
        # Zmiana temperatury
        dT = (power * dt - heat_loss * dt) / self.C
        self.current_temp += dT
        
        return self.current_temp
    
    def predict_overheating(self, future_power_profile):
        """Czy przekroczy limity?"""
        temp = self.current_temp
        
        for power in future_power_profile:
            temp = self.update(power, self.dt)
            if temp > MAX_TEMP:
                return True, temp
                
        return False, temp
```

### 3.3. Model degradacji

```python
# Model zużycia (predictive maintenance)

class DegradationModel:
    def __init__(self):
        self.wear_model = load_model("wear_lstm.pt")
        
    def calculate_wear(self, operational_data):
        """
        Oblicz zużycie na podstawie profilu pracy
        """
        # Feature engineering
        features = {
            'total_cycles': sum(d.cycles for d in operational_data),
            'avg_load': mean(d.load for d in operational_data),
            'max_temp': max(d.temperature for d in operational_data),
            'vibration_rms': rms(d.vibration for d in operational_data),
            'thermal_cycles': count_thermal_cycles(operational_data)
        }
        
        # Model LSTM przewiduje RUL (Remaining Useful Life)
        wear = self.wear_model.predict(features)
        
        return wear
    
    def estimate_remaining_life(self, current_wear, usage_profile):
        """Predykcja żywotności"""
        
        # Symuluj przyszłe zużycie
        future_wear = current_wear
        
        for i, day in enumerate(usage_profile):
            daily_wear = self.calculate_wear(day)
            future_wear += daily_wear
            
            if future_wear >= MAX_WEAR:
                return i  # Dni do awarii
        
        return float('inf')  # Nie widać końca
```

______________________________________________________________________

## 4. Przypadki użycia

### 4.1. Virtual Commissioning

```python
# Testowanie algorytmu na Digital Twin przed deploymentem

class VirtualCommissioning:
    def __init__(self, twin):
        self.twin = twin
        
    def test_new_controller(self, new_controller):
        """Testuj nowy kontroler w symulacji"""
        
        results = []
        
        # Scenariusze testowe
        test_scenarios = [
            "normal_operation",
            "obstacle_avoidance", 
            "emergency_stop",
            "low_battery",
            "sensor_failure"
        ]
        
        for scenario in test_scenarios:
            # Ustaw warunki początkowe
            self.twin.reset(scenario)
            
            # Uruchom symulację
            try:
                for step in range(SIMULATION_STEPS):
                    # Pobierz obserwację
                    obs = self.twin.get_observation()
                    
                    # Decyzja nowego kontrolera
                    action = new_controller.compute(obs)
                    
                    # Wykonaj w symulacji
                    self.twin.step(action)
                    
                    # Sprawdź bezpieczeństwo
                    if self.twin.check_safety_violation():
                        results.append({
                            'scenario': scenario,
                            'status': 'FAIL',
                            'violation': self.twin.get_violation()
                        })
                        break
                        
                results.append({
                    'scenario': scenario,
                    'status': 'PASS',
                    'metrics': self.twin.get_metrics()
                })
                
            except SimulationError as e:
                results.append({
                    'scenario': scenario,
                    'status': 'ERROR',
                    'error': str(e)
                })
        
        return results
```

### 4.2. Predictive Maintenance

```python
# Predykcja awarii

class PredictiveMaintenance:
    def __init__(self, fleet_twins):
        self.fleet = fleet_twins
        
    def generate_maintenance_schedule(self):
        """Harmonogram konserwacji dla floty"""
        
        schedule = []
        
        for robot_id, twin in self.fleet.items():
            # Pobierz predykcję
            prediction = twin.predict_maintenance()
            
            if prediction['predicted_failure']:
                schedule.append({
                    'robot_id': robot_id,
                    'priority': 'HIGH',
                    'predicted_day': prediction['days_until_failure'],
                    'reason': 'Predicted failure',
                    'recommended_action': self.suggest_action(prediction)
                })
            elif prediction['current_wear'] > 70:
                schedule.append({
                    'robot_id': robot_id,
                    'priority': 'MEDIUM',
                    'recommended_action': 'Inspection'
                })
        
        # Posortuj po priorytecie
        schedule.sort(key=lambda x: x['priority'])
        
        return schedule
    
    def suggest_action(self, prediction):
        """Sugestia działania na podstawie predykcji"""
        
        if prediction['thermal_stress']:
            return "Check cooling system"
        elif prediction['vibration']:
            return "Check bearings and mounts"
        elif prediction['wear']:
            return "Schedule replacement"
        else:
            return "General inspection"
```

### 4.3. Scenario Testing

```python
# Testowanie scenariuszy awaryjnych

class ScenarioTesting:
    def __init__(self, twin):
        self.twin = twin
        
    def test_sensor_failure(self, sensor):
        """Co się stanie gdy sensor zawodzi?"""
        
        # Wymuś awarię sensora
        self.twin.inject_fault(sensor, "stuck_at_zero")
        
        # Obserwuj zachowanie
        baseline = self.twin.get_estimated_state()
        
        # Sprawdź jak system reaguje
        results = {
            'detection_time': self.twin.fault_detection_time,
            'estimated_state_accuracy': compare(baseline, self.twin.estimated_state),
            'system_behavior': self.twin.get_behavior(),
            'recovery_time': self.twin.recovery_time
        }
        
        return results
    
    def test_emergency_scenario(self, scenario):
        """Symulacja scenariusza awaryjnego"""
        
        scenarios = {
            'sudden_obstacle': {'obstacle': (1, 0.5), 'distance': 0.3},
            'loss_of_communication': {'timeout': 5.0},
            'low_battery': {'battery_level': 5},
            'mechanical_failure': {'wheel': 'front_left', 'failure_mode': 'blocked'}
        }
        
        self.twin.setup_scenario(scenarios[scenario])
        
        return self.twin.run_simulation()
```

______________________________________________________________________

## 5. Integracja z systemami rzeczywistymi

### 5.1. Połączenie z ROS

```python
# Digital Twin <-> ROS bridge

class ROSTwinBridge:
    def __init__(self, twin):
        self.twin = twin
        
        # Subskrypcje ROS
        self.cmd_vel_sub = rospy.Subscriber(
            '/cmd_vel', Twist, self.on_cmd_vel
        )
        self.joint_sub = rospy.Subscriber(
            '/joint_states', JointState, self.on_joint_states
        )
        
        // Publifikacje ROS
        self.predicted_pub = rospy.Publisher(
            '/twin/predicted_state', Odometry, queue_size=1
        )
        self.health_pub = rospy.Publisher(
            '/twin/health', Float32, queue_size=1
        )
        
    def on_cmd_vel(self, msg):
        # Prześlij do twin
        self.twin.simulate_command(msg.linear, msg.angular)
        
    def on_joint_states(self, msg):
        # Zaktualizuj stan fizyczny
        self.twin.update_real_state(msg)
        
        # Porównaj z predykcją
        discrepancy = self.twin.get_discrepancy()
        
        # Opublikuj predykcję
        pred = self.twin.predict_next_state()
        self.predicted_pub.publish(pred)
        
        # Alert przy dużej różnicy
        if discrepancy > 0.1:
            rospy.logwarn(f"Twin discrepancy: {discrepancy}")
```

### 5.2. Synchronizacja czasowa

```python
# Synchronizacja czasu między twin a rzeczywistością

class TimeSync:
    def __init__(self, twin):
        self.twin = twin
        self.offset = None
        
    def calibrate(self):
        """Kalibracja offsetu czasowego"""
        
        # Wyślij timestamp do twin
        send_time = time.time()
        
        # Oblicz RTT (Round Trip Time)
        recv_time = self.twin.ping()
        rtt = recv_time - send_time
        
        # Offset = (RTT / 2) - processing_time
        self.offset = (rtt / 2) - PROCESSING_TIME
        
    def get_synced_state(self):
        """Stan zsynchronizowany czasowo"""
        
        # Pobierz stan z timestampem
        state = self.twin.get_state()
        
        # Skoryguj czas
        state.timestamp += self.offset
        
        return state
```

______________________________________________________________________

## 6. Pułapki i jak ich unikać

### 6.1. Model Drift

```python
# Problem: Model coraz bardziej odbiega od rzeczywistości

class ModelDriftDetector:
    def __init__(self):
        self.threshold = 0.05  # 5%
        
    def detect(self, real_state, predicted_state):
        # Oblicz różnicę
        diff = abs(real_state.position - predicted_state.position)
        
        # Sprawdź czy różnica rośnie
        if diff > self.threshold:
            return True, "Model drift detected"
        
        return False, None
    
    def handle_drift(self, detected):
        if detected:
            # Recalibrate model
            self.twin.recalibrate_from_actual()
            
            # Alert
            alert("Model recalibrated due to drift")
```

### 6.2. Synchronizacja

```python
# Problem: Opóźnienia w synchronizacji

class SyncManager:
    def __init__(self, max_lag_ms=100):
        self.max_lag = max_lag_ms
        
    def validate_data_freshness(self, data):
        age = time.time() - data.timestamp
        
        if age > self.max_lag:
            # Dane za stare - nie używaj do synchronizacji
            return False, "Data too old"
            
        return True, None
```

### 6.3. Walidacja

```python
# Problem: Twin nie odzwierciedla rzeczywistości

class ValidationSuite:
    def validate(self, twin, real_system):
        """Walidacja czy twin jest wiarygodny"""
        
        tests = []
        
        # Test 1: Zgodność stanów
        tests.append(self.test_state_agreement(twin, real_system))
        
        # Test 2: Odpowiedź na znane bodźce
        tests.append(self.test_response(twin, real_system))
        
        # Test 3: Granice fizyczne
        tests.append(self.test_physical_limits(twin))
        
        # Test 4: Awaryjne scenariusze
        tests.append(self.test_failure_modes(twin, real_system))
        
        return all(tests.passed for tests)
```

______________________________________________________________________

## 7. Narzędzia

| Narzędzie | Zastosowanie | Typ |
|-----------|-------------|-----|
| **Gazebo** | Symulacja robotów | Open source |
| **CoppeliaSim** | Symulacja fizyczna | Open source |
| **NVIDIA Isaac Sim** | Robotyka + AI | Komercyjny |
| **MATLAB Simulink** | Modelowanie + symulacja | Komercyjny |
| **AnyLogic** | Digital twin + symulacja | Komercyjny |
| **Azure Digital Twins** | IoT twin platform | Cloud |
| **AWS IoT TwinMaker** | Digital twin AWS | Cloud |

______________________________________________________________________

## 8. Podsumowanie

### Kiedy używać Digital Twin?

| Przypadek | Korzyść |
|-----------|---------|
| **Rozruch nowego systemu** | Bezpieczne testowanie |
| **Optymalizacja** | Eksperymenty bez ryzyka |
| **Predictive maintenance** | Unikanie awarii |
| **Sz szkolenie** | Symulacja scenariuszy |
| **Debugging** | Reprodukcja problemów |

### Kluczowe zasady

1. **Model musi być wierny** — inaczej nie ma sensu
1. **Dane muszą być świeże** — stare dane = zły twin
1. **Walidacja jest kluczowa** — sprawdzaj czy działa
1. **Integracja dwukierunkowa** — obserwacja + sterowanie
1. **Zacznij prosty** — iteruj i rozwijaj

### Ryzyka

- **Model drift** — model odbiega od rzeczywistości
- **Koszt** — budowa i utrzymanie twin jest drogie
- **Złożoność** — wiele komponentów do zsynchronizowania
- **Cyberbezpieczeństwo** — twin to kolejny wektor ataku

______________________________________________________________________

## Pytania do dyskusji

1. Czy Digital Twin może zastąpić testy na prawdziwym sprzęcie?
1. Kto jest odpowiedzialny za błędy w Digital Twin - producent czy operator?
1. Jak często synchronizować Digital Twin z rzeczywistością?

______________________________________________________________________

## Źródła

- Gartner: Digital Twin Best Practices
- AWS IoT TwinMaker Documentation
- NVIDIA Isaac Sim
- "Digital Twin Technology" - MIT Sloan Review
