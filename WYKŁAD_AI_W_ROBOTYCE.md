# Wykład: AI w robotyce — od klastrów do krawędzi

______________________________________________________________________

## Wprowadzenie: AI to nie wygoda, to necessity

Robotyka bez AI to automat. Robotyka z AI to **autonomia**. I to właśnie autonomiczne roboty zmieniają świat — od magazynów po Marsa.

Ale tu jest haczyk: **AI w robotyce to zupełnie inna bestia niż AI w chmurze**.

W centrum danych masz GPU za tysiące dolarów i nieograniczony prąd. Na robocie masz mikrokontroler za 5 dolarów i baterię.

Ten wykład to o tym, jak zmusić AI do pracy w ograniczeniach robotyki.

______________________________________________________________________

## 1. Dlaczego AI w robotyce jest trudna?

### Problem: moc obliczeniowa

| Środowisko | TOPS (biliony operacji na sekundę) | Pobór mocy |
|------------|-----------------------------------|------------|
| Nvidia H100 (chmura) | 1000+ TOPS | 700W |
| Nvidia Orin (robot) | 275 TOPS | 15-60W |
| Google Edge TPU | 4 TOPS | 2W |
| ARM Cortex-M4 (mikro) | 0.001 TOPS | 0.05W |

Różnica jest **milion razy**.

### Problem: latency

Chmura = sieć = opóźnienie. W robotyce decyzja musi zapadać **teraz**.

- Latencja sieci do AWS: 20-100 ms
- Latencja krytyczna w sterowaniu: < 1 ms
- **Wniosek: AI musi być na pokładzie**

### Problem: środowisko

Robot pracuje w:

- Wibracjach
- Zmiennych temperaturach
- Ograniczonej pamięci
- Bez chłodzenia wodnego

Serwer w klimatyzowanej serwerowni ma lepiej.

______________________________________________________________________

## 2. Architektura AI w robotyce — trzy warstwy

### Warstwa 1: Edge AI (na pokładzie)

```python
# Przykład: detekcja przeszkód na MCU
# Użycie TensorFlow Lite Micro

import tensorflow as tf

# Kwantyzacja do int8 - zmniejsza rozmiar 4x
converter = tf.lite.TFLiteConverter.from_saved_model("model.pb")
converter.optimizations = [tf.lite.Optimize.DEFAULT]
converter.target_spec.supported_types = [tf.int8]

# Model 250KB zamiast 1MB
quantized_model = converter.convert()
```

**Cel: decyzje w milisekundach, bez sieci**

### Warstwa 2: Smart Edge (komputer pokładowy)

```python
# NVIDIA Jetson Orin - percepcja w czasie rzeczywistym

class PerceptionPipeline:
    def __init__(self):
        self.yolo = YOLOv8("yolov8n.onnx")  # Detekcja obiektów
        self.depth = DepthAnything("depth.onnx")  # Estymacja głębi
        self.tracker = ByteTracker()  # Śledzenie
    
    def process(self, frame):
        # 30+ FPS przy 20W
        boxes = self.yolo.detect(frame)
        depth = self.depth.estimate(frame)
        tracks = self.tracker.update(boxes, depth)
        return tracks
```

**Cel: fuzja sensorów, SLAM, śledzenie**

### Warstwa 3: Cloud AI (opcjonalne)

```python
# Offload do chmury dla ciężkich zadań

async def heavy_analysis(point_cloud):
    # Segmentacja semantyczna - za ciężka dla edge
    result = await cloud_client.send(
        "/api/v1/segment",
        payload=point_cloud,
        priority="low"  # nieblokujące
    )
    return result
```

**Cel: mapowanie globalne, trenowanie, optymalizacja**

______________________________________________________________________

## 3. TinyML — AI na mikrokontrolerach

### Co to jest?

TinyML to machine learning na urządzeniach o < 1MB RAM i < 1mW poboru.

### Przykłady zastosowań

| Aplikacja | MCU | Model | RAM |
|-----------|-----|-------|-----|
| Wake word detection | Cortex-M4 | DNN | 32KB |
| Gesture recognition | ESP32 | CNN | 200KB |
| Anomaly detection | Cortex-M7 | LSTM | 64KB |
| Visual wake words | ESP32-S3 | MobileNetV2 | 350KB |

### Techniki optymalizacji

#### Kwantyzacja

```python
# Zmiana precyzji: float32 -> int8

# Wpływ na dokładność:
# float32: 95.2% dokładności
# int8:   94.8% dokładności (strata 0.4%)
# Rozmiar: 4x mniejszy
# Szybkość: 2-4x szybszy
```

#### Przycinanie (pruning)

```python
# Usuwanie nieważnych połączeń

# Przed: 1M parametrów
# Po pruning: 200K parametrów (80% zer)
# Wpływ: minimalny na dokładność przy odpowiednim fine-tuningu
```

#### Destylacja

```python
# Mały model uczy się od dużego

teacher = LargeModel()  # 100M params
student = SmallModel()   # 1M params

# Student uczy się "dark knowledge" od teachera
student.train(teacher.outputs)
```

______________________________________________________________________

## 4. Case study: Robot magazynowy Amazon

### Architektura

```
┌─────────────────────────────────────────────┐
│              AWS Cloud                      │
│  - Trenowanie modeli                        │
│  - Mapowanie globalne                       │
│  - Optymalizacja floty                     │
└────────────────────┬────────────────────────┘
                     │ OTA Updates
                     │ (rzadko)
┌────────────────────┴────────────────────────┐
│           Edge Controller (Jetson)          │
│  - SLAM lokalny                            │
│  - Planowanie ścieżek                      │
│  - Detekcja przeszkód                     │
└────────────────────┬────────────────────────┘
                     │
┌────────────────────┴────────────────────────┐
│           STM32 (MCU)                       │
│  - Sterowanie silników                      │
│  - Odczyt sensorów                         │
│  - Safety functions                        │
└─────────────────────────────────────────────┘
```

### AI na pokładzie

- **Detekcja przeszkód**: YOLOv5s (5MB) → kwantyzacja do 1.5MB
- **Predykcja ruchu**: LSTM na ostatnich 100 klatkach
- **Nawigacja**: Neural SLAM z Attention

### Częstotliwości

| Funkcja | Częstotliwość | Latencja |
|---------|--------------|----------|
| Sterowanie | 1 kHz | \<1ms |
| Detekcja | 30 Hz | \<33ms |
| SLAM | 10 Hz | \<100ms |
| Planowanie | 1 Hz | \<1s |

______________________________________________________________________

## 5. Scenariusze awarii

### Awaria 1: Adversarial attack na detekcję

**Co się dzieje:**

- Atakujący nakleja specjalne wzory na przeszkody
- Sieć neuronowa przestaje je widzieć
- Robot wjeżdża w przeszkodę

**Obrona:**

```python
# Adversarial training - trenowanie na przykładach ataku
adversarial_images = generate_adversarial(original_images)
model.train(adversarial_images)

# Ensemble różnych modeli
predictions = model1(img) + model2(img) + model3(img)
# Trudniej oszukać wszystkie naraz
```

### Awaria 2: Model drift

**Co się dzieje:**

- Środowisko się zmienia (nowe oświetlenie, kurz)
- Model nauczony na danych z fabryki nie działa
- Dokładność spada z 95% do 60%

**Obrona:**

```python
# Continuous learning na pokładzie
class AdaptiveModel:
    def update(self, new_data, labels):
        # Aktualizacja wag z małym learning rate
        self.model.fit(new_data, labels, lr=0.001)
        
        # Jeśli dokładność spada - rollback
        if self.accuracy() < self.threshold:
            self.rollback()
```

### Awaria 3: Overfitting na edge

**Co się dzieje:**

- Model za duży dla available RAM
- Wymiana danych na stacku = crash
- Robot się zawiesza

**Obrona:**

```python
# Static memory allocation - wymagane w RTOS

class Model:
    def __init__(self):
        # Alokacja na etapie kompilacji
        self.buffer = numpy.zeros(MODEL_INPUT_SIZE, dtype=np.int8)
        
    def predict(self, data):
        # Określony rozmiar - brak dynamicznej alokacji
        np.copyto(self.buffer, data)
        return self.model(self.buffer)
```

______________________________________________________________________

## 6. Dobre praktyki projektowe

### Zasada 1: Fail-safe AI

```python
# Zawsze miej fallback

def perceive(obstacle_detector, lidar_data, camera_data):
    try:
        # Spróbuj AI
        obstacles = obstacle_detector(camera_data)
    except AIError:
        # Fallback do klasycznego
        obstacles = classical_detection(lidar_data)
    
    # Jeśli nic nie działa - STOP
    if obstacles is None:
        robot.stop()
        return []
    
    return obstacles
```

### Zasada 2: Graceful degradation

```python
# W zależności od obciążenia - różne modele

def get_model(battery_level, cpu_load):
    if cpu_load > 80%:
        return light_model  # Mniej dokładny, ale szybki
    elif cpu_load > 50%:
        return medium_model
    else:
        return full_model
```

### Zasada 3: Watchdog dla AI

```python
# Monitoruj czy AI działa poprawnie

def ai_watchdog():
    while True:
        if time_since_last_ai_result() > 200ms:
            # AI może być zawieszona
            log_error("AI timeout - restarting")
            reset_ai_subsystem()
        sleep(50ms)
```

### Zasada 4: Anomaly detection na wyjściach

```python
# Sprawdzaj czy wyjście AI ma sens

def validate_ai_output(output):
    # Fizyczne ograniczenia
    if output.velocity > MAX_VELOCITY:
        return clamp(output.velocity, MAX_VELOCITY)
    
    # Spójność z poprzednim stanem
    if abs(output.position - last_position) > MAX_DELTA:
        return last_position  # Użyj poprzedniego
    
    # Wykrywanie anomalii przez ML
    if anomaly_detector.is_anomaly(output):
        log_warning("AI output anomaly detected")
        return safe_default
    
    return output
```

______________________________________________________________________

## 7. Narzędzia

### Frameworki

| Framework | Zastosowanie | Rozmiar min |
|-----------|------------|-------------|
| TensorFlow Lite Micro | Embedded ML | 20KB |
| ONNX Runtime | Edge inference | 1MB |
| PyTorch Mobile | Mobile/Edge | 10MB |
| tinyML Forge | MCU | Zależy |

### Hardware

| Platform | TOPS | Pobór | Cena |
|----------|------|-------|------|
| ESP32-S3 | 0.01 | 0.3W | $10 |
| STM32H7 | 0.05 | 0.5W | $15 |
| Raspberry Pi 4 | 1 | 5W | $55 |
| Jetson Orin Nano | 40 | 7W | $500 |
| Google Edge TPU | 4 | 2W | $40 |

______________________________________________________________________

## 8. Podsumowanie

### Kluczowe zasady

1. **AI na krawędzi to konieczność** — latency i niezawodność
1. **Mniej znaczy więcej** — małe modele > duże
1. **Kwantyzacja jest przyjacielem** — int8 wystarcza
1. **Zawsze miej fallback** — AI zawodzi
1. **Monitoruj co się dzieje** — watchdog i anomaly detection

### Co pamiętać

- Robot to nie serwer — ograniczenia są realne
- Model edge musi być mały i deterministyczny
- Bezpieczeństwo > wydajność
- Testuj w realnych warunkach, nie tylko na laptopie

______________________________________________________________________

## Pytania do dyskusji

1. Jak zapewnić bezpieczeństwo AI na robocie, gdy aktualizacje OTA mogą być atakowane?
1. Czy autonomiczny robot powinien zawsze słuchać AI, czy mieć override?
1. Jak testować AI w warunkach których nie przewidzieliśmy?

______________________________________________________________________

## Źródła i dalsze czytanie

- TinyML: Machine Learning with TensorFlow Lite on Arduino and Ultra-Low-Power Devices
- NVIDIA Jetson Documentation
- TensorFlow Lite for Microcontrollers
- "Deep Learning for Computer Vision" — Adrian Rosebrock
