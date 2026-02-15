# Wykład: Cyberbezpieczeństwo robotów — gdy roboty się buntują

---

## Wprowadzenie: Roboty to komputery na sterydach

Każdy robot to komputer z dostępem do fizycznego świata. Co się stanie gdy ktoś go zhakuje?

- **Robot przemysłowy** — może zranić lub zabić człowieka
- **Dron** — może być bronią
- **Samochód autonomiczny** — może zamienić się w bombę
- **Robot medyczny** — może zabić pacjenta

Cyberbezpieczeństwo w robotyce to nie jest "fajnie mieć". To **kwestia życia i śmierci**.

---

## 1. Anatomia ataku na robota

### Warstwy ataku

```
┌────────────────────────────────────────────┐
│         Warstwa ataku na robota            │
├────────────────────────────────────────────┤
│ 7. Aplikacja (ROS, algorytmy)             │  ← jailbreak LLM
│ 6. System operacyjny (Linux, RTOS)         │  ← kernel exploit
│ 5. Sieć (WiFi, Ethernet, 5G)              │  ← man-in-the-middle
│ 4. Protokoły (ROS2, DDS, Modbus)          │  ← protocol attack
│ 3. Komunikacja (SPI, I2C, CAN)            │  ← bus sniffing
│ 2. Sterownik (MCU)                        │  ← firmware exploit
│ 1. Hardware                                │  ← side-channel attack
└────────────────────────────────────────────┘
```

### Wektory ataku

| Wektor | Przykład | Skuteczność |
|--------|----------|-------------|
| Sieć | Atak na ROS2 przez DDS | Wysoka |
| Aktualizacje OTA | Złośliwy firmware | Wysoka |
| Sensor | LIDAR spoofing | Średnia |
| Łącze radiowe | Zigbee/Bluetooth jam | Średnia |
| Fizyczny | Podpięcie się do debug | Niska (ale możliwa) |

---

## 2. Specyficzne zagrożenia w robotyce

### 2.1. Ataki na protokoły robotyczne

#### ROS/ROS2

```python
# Atak: przejęcie kontroli nad węzłem ROS

# Normalny subscriber - nasłuchuje poleceń
def on_cmd_vel(msg):
    robot.move(msg.linear.x, msg.angular.z)

# Atakujący - publikuje fałszywe polecenia
def attack():
    while True:
        fake_msg = Twist()
        fake_msg.linear.x = 1000  # Maksymalna prędkość do przodu!
        cmd_vel_pub.publish(fake_msg)
        sleep(0.01)  # 100 Hz
```

**Obrona:**
```python
# Autentykacja i autoryzacja
from ros2_security import SecurityMiddleware

# Każdy węzeł musi mieć certyfikat
node.secure(
    authenticate=True,
    authorize=True,
    encrypt=True
)

# Sprawdzanie ograniczeń przed wykonaniem
def safe_cmd_vel(msg):
    # Ograniczenia fizyczne
    msg.linear.x = clamp(msg.linear.x, MAX_VELOCITY)
    msg.angular.z = clamp(msg.angular.z, MAX_TURN)
    
    # Whitelist dozwolonych publisherów
    if not is_authorized(msg._publisher_name, 'cmd_vel'):
        log_attack("Unauthorized cmd_vel")
        return
    
    robot.move(msg.linear.x, msg.angular.z)
```

#### Modbus/OPC-UA (przemysł)

```python
# Atak: zmiana rejestrów PLC

# Normalne odczytanie temperatury
temp = plc.read_holding_register(TEMP_SENSOR_ADDR)

# Atak: manipulacja
plc.write_holding_register(TEMP_SENSOR_ADDR, 9999)  # Fałszywa temp
# lub
plc.write_holding_register(SAFETY_LIMIT_ADDR, 9999)  # Wyłączenie limitu
```

**Obrona:**
```python
# Whitelist operacji
class SecurePLC:
    def write(self, address, value):
        # Sprawdź czy adres jest dozwolony do zapisu
        if address in self.read_only_registers:
            raise SecurityError("Read-only register")
        
        # Sprawdź czy wartość jest w bezpiecznym zakresie
        if not self.is_safe_value(address, value):
            raise SecurityError("Unsafe value")
        
        # Loguj wszystkie operacje
        self.audit_log.write(address, value)
        
        return super().write(address, value)
```

### 2.2. Ataki na sensory

#### LIDAR spoofing

```
Atak:
       
       Fałszywe punkty LIDAR
              ↓
       ┌───────────────────┐
       │     ROBOT         │
       │    myśli że        │
       │  jest ścieżka     │
       └───────────────────┘
       
Wynik: Robot jedzie w ścianę
```

```python
# Obrona: wykrywanie anomalii w chmurze punktów

class LidarAnomalyDetector:
    def __init__(self):
        self.history = deque(maxlen=100)
    
    def detect_spoofing(self, points):
        # Sprawdź czy punkty są fizycznie możliwe
        for p in points:
            # Punkty nie mogą być zbyt blisko
            if p.distance < MIN_VALID_DISTANCE:
                return True
            
            # Punkty nie mogą pojawić się znikąd
            if self.is_new_ghost_point(p):
                return True
        
        # Wykrywanie ataku "tunelowego"
        if self.detect_tunnel_attack(points):
            return True
            
        return False
    
    def detect_tunnel_attack(self, points):
        # Atak tunelowy: fałszywe punkty tworzą "korytarz"
        # przez który atakujący chce przeprowadzić robota
        if self.has_uniform_corridor(points):
            log.warning("Potential tunnel attack detected")
            return True
        return False
```

#### Kamera / Computer Vision

```python
# Atak: adversarial examples
# Niewidoczne wzory które oszukują sieć neuronową

# Obrona: adversarial training
def train_adversarial(model, images, labels):
    # Generuj adversarial examples
    adv_images = generate_adversarial(images)
    
    # Trenuj na mieszance
    combined = concatenate([images, adv_images])
    combined_labels = concatenate([labels, labels])
    
    model.train(combined, combined_labels)

# I testowanie na wielu modelach (ensemble)
def robust_detect(image):
    results = []
    for model in [yolo, rcnn, efficientdet]:
        results.append(model.predict(image))
    
    # Głosowanie większościowe
    final = majority_vote(results)
    return final
```

### 2.3. Ataki na komunikację

#### Man-in-the-Middle na CAN bus

```
Normalnie:
  [ECU1] ----CAN---- [ECU2] ----CAN---- [ECU3]

Atak:
  [ECU1] ----CAN---- [ATAKER] ----CAN---- [ECU2] ----CAN---- [ECU3]
              ↑                       ↑
         przechwytuje           modyfikuje
         wiadomości            wiadomości
```

```python
# Atak: Wstrzykiwanie wiadomości na CAN

import can

bus = can.interface.Bus(channel='vcan0', bustype='socketcan')

# Normalna wiadomość: prędkość 50 km/h
msg = can.Message(
    arbitration_id=0x123,
    data=[0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
)
bus.send(msg)

# Atak: ustawienie max prędkości
attack_msg = can.Message(
    arbitration_id=0x123,
    data=[0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]  # 255 km/h!
)
bus.send(attack_msg)
```

**Obrona:**
```python
# Bezpieczny CAN z autentykacją

class SecureCAN:
    def __init__(self, key):
        self.key = key
        
    def send(self, arbitration_id, data):
        # Dodaj HMAC do każdej wiadomości
        hmac = compute_hmac(self.key, arbitration_id + data)
        secure_data = data + hmac[:4]  # 4 bajty HMAC
        
        return super().send(arbitration_id, secure_data)
    
    def receive(self, arbitration_id, data):
        # Weryfikuj HMAC
        received_hmac = data[-4:]
        expected_hmac = compute_hmac(self.key, arbitration_id + data[:-4])[:4]
        
        if received_hmac != expected_hmac:
            raise SecurityError("Invalid HMAC - possible attack!")
        
        return data[:-4]  # Usuń HMAC
```

---

## 3. Architektura bezpieczeństwa robota

### Model Zero Trust

```
┌─────────────────────────────────────────────────────────────┐
│                    ZERO TRUST ROBOT                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│  │ Sterowanie│    │ Percepcja │    │ Planowanie│            │
│  │  (RTOS)  │    │  (AI)     │    │  (Linux)  │            │
│  └────┬─────┘    └────┬─────┘    └────┬─────┘             │
│       │               │               │                    │
│       └───────────────┼───────────────┘                    │
│                       ↓                                     │
│              ┌─────────────────┐                           │
│              │  Policy Engine   │ ← Każda operacja         │
│              │  (authorization) │   wymaga autoryzacji    │
│              └────────┬────────┘                           │
│                       ↓                                     │
│              ┌─────────────────┐                           │
│              │  Audit Log       │ ← Wszystko logowane     │
│              └─────────────────┘                           │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Implementacja

```python
class RobotSecurityManager:
    def __init__(self):
        self.policies = PolicyEngine()
        self.audit = AuditLog()
        self.tpm = TPM()  # Hardware security
        
    def execute_command(self, cmd):
        # 1. Sprawdź autentyczność
        if not self.authenticate(cmd.source):
            self.audit.log("AUTH_FAILURE", cmd)
            return Result.deny("Not authenticated")
        
        # 2. Sprawdź autoryzację
        if not self.policies.check(cmd):
            self.audit.log("POLICY_FAILURE", cmd)
            return Result.deny("Not authorized")
        
        # 3. Sprawdź integralność
        if not self.verify_integrity(cmd):
            self.audit.log("INTEGRITY_FAILURE", cmd)
            return Result.deny("Integrity check failed")
        
        # 4. Wykonaj z ograniczeniami
        result = self.execute_with_limits(cmd)
        
        # 5. Loguj wynik
        self.audit.log("SUCCESS", cmd, result)
        
        return result
    
    def execute_with_limits(self, cmd):
        # Ograniczenia fizyczne - zawsze
        cmd.velocity = min(cmd.velocity, MAX_SAFE_VELOCITY)
        cmd.force = min(cmd.force, MAX_SAFE_FORCE)
        
        # Limit czasu wykonania
        with timeout(cmd.timeout_ms):
            return cmd.execute()
```

---

## 4. Bezpieczna aktualizacja OTA

### Problem

Aktualizacja OTA to jedno z największych zagrożeń:

1. Atakujący przechwytuje aktualizację
2. Wstrzykuje złośliwy kod
3. Robot instaluje backdoor

### Rozwiązanie: Secure Boot + Signed Updates

```python
class SecureOTAUpdater:
    def __init__(self):
        self.tpm = TPM()
        self.secure_boot = SecureBoot()
        
    def verify_and_apply_update(self, update_package):
        # 1. Sprawdź podpis producenta
        if not self.verify_signature(update_package):
            raise SecurityError("Invalid signature")
        
        # 2. Sprawdź hash pliku
        expected_hash = self.tpm.read_certified_hash()
        actual_hash = compute_hash(update_package.payload)
        if expected_hash != actual_hash:
            raise SecurityError("Hash mismatch - possible tampering")
        
        # 3. Sprawdź wersję (nie wstecz!)
        if update_package.version <= self.current_version:
            raise SecurityError("Downgrade not allowed")
        
        # 4. Weryfikuj certyfikat producenta
        if not self.verify_certificate(update_package.cert):
            raise SecurityError("Invalid certificate")
        
        # 5. Bezpiecznie zapisz i uruchom
        self.secure_boot.apply_update(update_package)
        
        # 6. Jeśli coś nie tak - rollback
        if not self.boot_verification():
            self.rollback()
```

---

## 5. Bezpieczeństwo warstwowe (Defense in Depth)

### Warstwa 1: Network

```python
# Izolacja sieciowa
class NetworkSecurity:
    def setup(self):
        # VLAN dla sieci robotycznej
        self.vlan_setup(ROBOT_VLAN, device_ids)
        
        # Firewall
        self.iptables.add_rule("-j ACCEPT", 
                               src=TRUSTED_ZONE, 
                               dst=ROBOT_ZONE,
                               port=ROS_PORT)
        self.iptables.add_rule("-j DROP", 
                               dst=ROBOT_ZONE)  # Domknij domyślnie
        
        # Sieć bezprzewodowa - izolacja
        self.wifi.set_isolation(True)
```

### Warstwa 2: Application

```python
# Sandboxing aplikacji
class AppSecurity:
    def run_node(self, node):
        # Seccomp - ogranicz syscalls
        seccomp.load_filter(ALLOWED_SYSCALLS)
        
        # Namespaces - izolacja
        self.unshare(CLONE_NEWNS | CLONE_NEWUTS | CLONE_NEWPID)
        
        # Ograniczone uprawnienia
        self.setrlimit(RLIMIT_NPROC, 10)  # Max 10 procesów potomnych
        self.setrlimit(RLIMIT_FSIZE, 100MB)  # Max rozmiar plików
```

### Warstwa 3: Hardware

```python
# Hardware security module
class HardwareSecurity:
    def encrypt_storage(self):
        # Szyfrowanie z TPM
        self.tpm.encrypt_disk("/dev/mmcblk0", self.tpm.key)
        
    def secure_debug(self):
        # Wyłącz JTAG w produkcji
        self.fuse.disable_jtag()
        
        # Wymagaj klucza do debug
        self.fuse.enable_secure_debug(self.tpm.public_key)
```

---

## 6. Wykrywanie ataków

### Anomaly Detection

```python
class IntrusionDetection:
    def __init__(self):
        self.baseline = self.learn_baseline()
        
    def detect(self, event):
        # Wykrywanie anomalii w zachowaniu
        anomaly_score = self.compute_anomaly_score(event)
        
        if anomaly_score > THRESHOLD:
            self.alert("Anomaly detected", event)
            
        # Wykrywanie wzorców ataków
        for pattern in ATTACK_PATTERNS:
            if pattern.matches(event):
                self.alert("Attack pattern detected", pattern)
                
    def learn_baseline(self):
        # Naucz się normalnego zachowania
        # - typowe czasy odpowiedzi
        # - typowe trasy sieciowe
        # - typowe wykorzystanie CPU/mem
        return Baseline()
```

---

## 7. Scenariusze awarii

### Scenariusz 1: Ransomware na PLC

```
Atak:
1. Włamanie przez podatność w web interfejsie
2. Szyfrowanie PLC code
3. Żądanie okupu

Skutek: Linia produkcji stoi

Obrona:
- Air-gapped backup
- Wersjonowanie kodu
- HSM do przechowywania kluczy
- Network segmentation
```

### Scenariusz 2: Dron jako broń

```
Atak:
1. Przejęcie kontroli przez podatność w RC
2. Zmiana flight control software
3. Dron jako broń

Obrona:
- Tryb "kill switch" - fizyczny przycisk
- Geofencing - strefy zakazane
- Autoryzacja lotów
- Monitoring anomaly
```

---

## 8. Checklista bezpieczeństwa

### Projektowanie

- [ ] Network segmentation - oddziel OT od IT
- [ ] Zero trust - nic nie ufaj, wszystko weryfikuj
- [ ] Defense in depth - wiele warstw ochrony
- [ ] Secure by default - bezpieczne od pierwszego uruchomienia

### Implementacja

- [ ] Szyfrowanie wszystkiego (in transit, at rest)
- [ ] Autentykacja i autoryzacja wszędzie
- [ ] Least privilege - minimalne uprawnienia
- [ ] Secure coding - OWASP guidelines

### Operacje

- [ ] Monitoring i alerting
- [ ] Regularne aktualizacje
- [ ] Penetracja testy
- [ ] Incident response plan
- [ ] Backup i disaster recovery

---

## 9. Podsumowanie

### Zasady bezpieczeństwa robotów

1. **Defense in depth** — wiele warstw ochrony
2. **Zero trust** — nic nie ufaj, wszystko weryfikuj
3. **Secure by design** — bezpieczeństwo od początku
4. **Fail secure** — przy błędzie bezpieczny stan
5. **Monitor everything** — wiesz co się dzieje

### Pamiętać

- Roboty to nie komputery — atak może skończyć się śmiercią
- Security by obscurity nie działa
- Ataki są realne i już się dzieją
- Lepiej zapobiegać niż leczyć

---

## Pytania do dyskusji

1. Czy robot medyczny powinien mieć fizyczny "kill switch" dostępny dla pacjenta?
2. Jak balansować między bezpieczeństwem a użytecznością?
3. Kto ponosi odpowiedzialność za atak na robota — producent czy operator?

---

## Źródła

- NIST Cybersecurity Framework
- IEC 62443 (Industrial Automation Security)
- ISO/SAE 21434 (Automotive Cybersecurity)
- ROS 2 Security Working Group