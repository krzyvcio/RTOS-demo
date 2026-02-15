# Wykład: Percepcja i wizja — oczy robota

______________________________________________________________________

## Wprowadzenie: Robot, który nie widzi, jest ślepy

Wyobraź sobie robota, który:

- Jedzie w ścianę bo nie widzi
- Nie rozpoznaje człowieka i go nie omija
- Nie wie gdzie jest w przestrzeni
- Nie rozumie co trzyma w łapie

To nie robot. To drogi gruz.

**Percepcja to fundament autonomii.** Bez niej robot jest tylko automatem.

W tym wykładzie dowiesz się:

- Jak robot widzi świat (i dlaczego inaczej niż człowiek)
- Jakie sensory używamy i dlaczego
- Jak połączyć różne dane w spójny obraz
- Jak wykrywać, śledzić, rozpoznawać

Zaczynamy!

______________________________________________________________________

## 1. Zmysły robota — co może "widzieć"?

### Rodzaje percepcji

| zmysł | Technologia | Zastosowanie | Zalety | Wady |
|--------|-------------|--------------|---------|-------|
| **Wzrok** | Kamera | Rozpoznawanie | Tanie, bogate dane | Zależne od światła |
| **Głębia** | LiDAR | Mapowanie | Dokładne pomiary | Drogi, rzadkie dane |
| **Głębia** | ToF | Bliski zasięg | Szybki | Szumy |
| **Dotyk** | Tactile | Kontakt | Pełne | Trudne w integracji |
| **Dźwięk** | Mikrofon | Lokalizacja | 360° | Hałas |
| **Płaszczyzna** | Radar | Daleki zasięg | Wszechpogodowy | Mała rozdzielczość |

### Multimodalność — klucz do sukcesu

```
                    PERCEPCJA ROBOTA
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
    ┌────▼────┐      ┌────▼────┐      ┌────▼────┐
    │ KAMERA  │      │ LiDAR   │      │  IMU   │
    │ RGB/D   │      │ 3D      │      │ 6DOF   │
    └────┬────┘      └────┬────┘      └────┬────┘
         │                 │                 │
         └─────────────────┼─────────────────┘
                           │
                    ┌──────▼──────┐
                    │  FUZJA      │
                    │  DANYCH     │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  ŚWIADOMOŚĆ │
                    │  PRZESTRZENI│
                    └─────────────┘
```

______________________________________________________________________

## 2. Kamery — okno do świata

### Rodzaje kamer

```python
# Kamera RGB - zwykły obraz

class RGBCamera:
    def capture(self):
        # Zwraca obraz RGB [H, W, 3]
        return self.read_frame()
    
    # Zastosowania:
    # - Rozpoznawanie obiektów
    # - Segmentacja semantyczna
    # - Odczyt znaków
    # - Detekcja twarzy
```

```python
# Kamera głębi - ile jest daleko?

class DepthCamera:
    def capture(self):
        # Zwraca mapę głębi [H, W]
        return self.read_depth()
    
    # Technologie:
    # - Stereo vision (2 kamery)
    # - Structured Light (iPhone FaceID)
    # - Time of Flight (ToF)
    # - LiDAR (skaner)
```

```python
# Kamera termowizyjna - ciepło

class ThermalCamera:
    def capture(self):
        # Zwraca temperaturę w każdym pikselu
        return self.read_thermal()
    
    # Zastosowania:
    # - Wykrywanie ludzi w ciemności
    # - Diagnostyka termiczna
    # - Wykrywanie pożarów
    # - Monitoring zwierząt
```

```python
# Kamera eventowa - ruch!

class EventCamera:
    def capture(self):
        # Zwraca zdarzenia: (x, y, timestamp, polarity)
        # Reaguje tylko na zmiany!
        return self.read_events()
    
    # Zalety:
    # - Ogromna prędkość (klawisz 10k+ FPS)
    # - Bardzo niska latencja (<1ms)
    # - Niski pobór mocy
    # - HDR - każde oświetlenie
```

______________________________________________________________________

## 3. Wykrywanie obiektów — co to jest?

### Klasyczne podejście — HOG + SVM

```python
# Histogram of Oriented Gradients

def extract_hog(image):
    # Podziel na komórki
    cells = divide_into_cells(image, 8x8)
    
    # Dla każdej komórki oblicz histogram gradientów
    for cell in cells:
        gradients = compute_gradients(cell)
        histogram = create_histogram(gradients, bins=9)
    
    # Połącz w wektor cech
    return concatenate(all_histograms)

# Klasyfikator SVM
classifier = SVM()
classifier.train(training_images, labels)

# Wykrywanie
def detect(image):
    windows = slide_window(image)
    for window in windows:
        features = extract_hog(window)
        if classifier.predict(features):
            yield window
```

### Deep Learning — YOLO, Faster R-CNN

```python
# YOLO - You Only Look Once

class YOLODetector:
    def __init__(self):
        self.model = load_model("yolov8n.pt")
        
    def detect(self, image):
        # JEDNO przejście przez sieć!
        # Output: [batch, num_boxes, (x1,y1,x2,y2,conf,class_probs)]
        predictions = self.model(image)
        
        # Parsowanie wyników
        boxes = []
        for pred in predictions:
            if pred.confidence > CONFIDENCE_THRESHOLD:
                boxes.append({
                    'bbox': pred.bbox,
                    'class': CLASS_NAMES[pred.class_id],
                    'confidence': pred.confidence
                })
        
        # Non-maximum suppression
        return nms(boxes)
```

### Ale jest problem — Edge AI

```python
# YOLO na tablecie = OK
# YOLO na robocie = 💥

# Problem: model jest za duży!

# ROZWIĄZANIE: YOLO Nano

class YOLONano:
    def __init__(self):
        # Około 4M parametrów zamiast 60M
        self.model = load_model("yolov8n.pt")  # nano = 6M
        # lub:
        # self.model = load_model("yolov3-tiny.pt")  # 8M
        
    def optimize_for_edge(self):
        # Kwantyzacja do INT8
        self.model = quantize(self.model, dtype=np.int8)
        
        # Przycinanie
        self.model = prune(self.model, amount=0.5)
        
        return self.model
```

______________________________________________________________________

## 4. Śledzenie obiektów — gdzie jest?

### Problem: Tracking

```
Klatka 1:     Klatka 2:     Klatka 3:
┌─────────┐   ┌─────────┐   ┌─────────┐
│    ●    │   │   ●     │   │    ●    │
│         │   │         │   │         │
└─────────┘   └─────────┘   └─────────┘
    ID=1         ID=1           ID=1
    
Klatka 4:     Klatka 5:
┌─────────┐   ┌───────────────┐
│    ●    │   │      ●        │
│         │   │                │
└─────────┘   └───────────────┘
    ID=1         ID=1 (wyszedł!)
```

### SORT — Simple Online and Realtime Tracking

```python
class SORTTracker:
    def __init__(self):
        self.detector = YOLODetector()
        self.kalman = KalmanFilter()
        self.tracks = {}
        self.next_id = 0
        
    def update(self, frame):
        # 1. Wykryj obiekty
        detections = self.detector.detect(frame)
        
        # 2. Predykcja dla istniejących śladów
        for track_id, track in self.tracks.items():
            track['box'] = self.kalman.predict(track['box'])
        
        # 3. Kojarzenie (matching)
        matched, unmatched_det, unmatched_track = \
            self.match_tracks(detections, self.tracks)
        
        # 4. Aktualizacja śladów
        for det_idx, track_id in matched:
            self.tracks[track_id]['box'] = \
                self.kalman.update(detections[det_idx])
            self.tracks[track_id]['age'] = 0
        
        # 5. Nowe ślady
        for det_idx in unmatched_det:
            self.tracks[self.next_id] = {
                'box': detections[det_idx],
                'age': 0,
                'hits': 1
            }
            self.next_id += 1
        
        # 6. Usuń zaginione
        self.tracks = {
            t: tr for t, tr in self.tracks.items()
            if tr['age'] < MAX_AGE
        }
        
        return self.tracks
```

### Deep SORT — z deep learningiem

```python
# Deep SORT - dodaje Feature Extraction

class DeepSORT:
    def __init__(self):
        self.detector = YOLODetector()
        self.reid = ReIDModel()  # Ekstrakcja cech
        self.kalman = KalmanFilter()
        
    def update(self, frame):
        # Wykryj
        detections = self.detector.detect(frame)
        
        # Ekstrakcja cech (ReID)
        features = self.reid.extract(frame, detections)
        
        # Predykcja + kojarzenie z cechami
        # ... (similar to SORT but with feature distance)
        
        return self.tracks
```

______________________________________________________________________

## 5. Fuzja sensorów — zmysły razem

### Problem: Każdy sensor ma wady

| Sensor | Wada |
|--------|------|
| Kamera | Nie działa w ciemności |
| LiDAR | Nie widzi szyby |
| Radar | Mała rozdzielczość |
| Ultradźwięki | Hałas |

### Rozwiązanie: Fuzja!

```python
# Architektura fuzji

class SensorFusion:
    def __init__(self):
        self.camera = RGBCamera()
        self.lidar = LiDAR()
        self.radar = Radar()
        self.imu = IMU()
        
        # Fuzja na poziomie detekcji (early fusion)
        self.detector = FusionDetector()
        
    def perceive(self):
        # Pobierz dane ze wszystkich sensorów
        rgb = self.camera.capture()
        depth = self.lidar.capture()
        radar = self.radar.capture()
        
        # Wykryj obiekty z każdego
        cam_det = self.camera_detector.detect(rgb)
        lidar_det = self.lidar_detector.detect(depth)
        radar_det = self.radar_detector.detect(radar)
        
        # Połącz (late fusion)
        fused = self.fuse_detections(cam_det, lidar_det, radar_det)
        
        # Śledź
        tracks = self.tracker.update(fused)
        
        return tracks
    
    def fuse_detections(self, *detections):
        """
        Kojarzenie detekcji z różnych sensorów
        """
        all_dets = []
        
        for dets in detections:
            for d in dets:
                all_dets.append({
                    'position': d.position,
                    'velocity': d.velocity,
                    'class': d.class_id,
                    'sensor': d.source,
                    'confidence': d.confidence
                })
        
        # Grupowanie detekcji blisko siebie
        clusters = cluster_by_distance(all_dets)
        
        # Łączenie w jedną detekcję
        fused = []
        for cluster in clusters:
            # Średnia ważona po pewności
            fused_det = weighted_average(cluster, weights='confidence')
            fused_det.confidence = max(d.confidence for d in cluster)
            fused.append(fused_det)
        
        return fused
```

### Extended Kalman Filter (EKF)

```python
# EKF dla fuzji IMU + GPS + Odometria

class ExtendedKalmanFilter:
    def __init__(self):
        # Stan: [x, y, z, roll, pitch, yaw, vx, vy, vz]
        self.state = np.zeros(9)
        self.covariance = np.eye(9) * 0.1
        
    def predict(self, dt):
        # Model ruchu (np. stała prędkość)
        F = self.motion_model(dt)
        self.state = F @ self.state
        self.covariance = F @ self.covariance @ F.T + self.process_noise
        
    def update(self, measurement, sensor_type):
        # Różne sensory = różne modele pomiaru
        
        if sensor_type == 'gps':
            H = np.zeros((3, 9))
            H[:3, :3] = np.eye(3)  # tylko pozycja
            R = np.eye(3) * 0.5   # szum GPS
            
        elif sensor_type == 'imu':
            H = np.zeros((3, 9))
            H[:3, 6:9] = np.eye(3)  # tylko prędkość
            R = np.eye(3) * 0.1
            
        elif sensor_type == 'odometry':
            H = np.zeros((2, 9))
            H[:2, :2] = np.eye(2)  # x, y
            R = np.eye(2) * 0.05
        
        # EKF update
        y = measurement - H @ self.state
        S = H @ self.covariance @ H.T + R
        K = self.covariance @ H.T @ np.linalg.inv(S)
        
        self.state = self.state + K @ y
        self.covariance = (np.eye(9) - K @ H) @ self.covariance
        
        return self.state
```

______________________________________________________________________

## 6. SLAM — gdzie jestem?

### Simultaneous Localization and Mapping

```
SLAM = Mapowanie + Lokalizacja jednocześnie

     ┌─────────────┐
     │    ŚWIAT    │
     │             │
     │    ●───●   │
     │   /      \  │
     │  ●        ● │
     │   \      /  │
     │    ●────●   │
     │             │
     └──────┬──────┘
            │
    ┌───────▼───────┐
    │  Robot:     │
    │  Gdzie jestem│
    │  Co widzę   │
    └─────────────┘
```

### Visual SLAM (vSLAM)

```python
class VisualSLAM:
    def __init__(self):
        self.camera = StereoCamera()
        self.orb = ORBFeatureExtractor()  # Cechy
        self.matcher = BFMatcher()
        self.map = Map()
        self.poses = {}  # Trajektoria
        
    def process_frame(self, frame):
        # 1. Ekstrakcja cech
        keypoints, descriptors = self.orb.detect(frame)
        
        # 2. Jeśli mam poprzednią klatkę
        if self.prev_frame is not None:
            # 3. Dopasowanie
            matches = self.matcher.match(self.prev_des, descriptors)
            
            # 4. Estymacja ruchu (PnP)
            R, t, inliers = self.estimate_motion(matches)
            
            # 5. Triangulacja nowych punktów
            self.triangulate_new_points(matches, R, t)
            
            # 6. Bundle Adjustment (opcjonalnie)
            self.optimize_bundle()
        
        # 7. Aktualizacja mapy
        self.map.update(keypoints, descriptors)
        
        # 8. Lokalizacja w mapie
        pose = self.localize_in_map(keypoints, descriptors)
        
        self.prev_frame = frame
        self.poses[frame.id] = pose
        
        return pose, self.map
```

### LiDAR SLAM

```python
class LiDARSLAM:
    def __init__(self):
        self.lidar = LiDAR()
        self.ICP = ICPAligner()
        self.map = PointCloudMap()
        self.poses = []
        
    def process_scan(self, scan):
        # Downsample dla szybkości
        scan_down = downsample(scan, voxel_size=0.1)
        
        # Initial guess z odometrii
        initial_pose = self.odometry.get_pose()
        
        # ICP alignment
        aligned_scan, delta = self.ICP.align(
            scan_down, 
            self.map.get_nearby(initial_pose),
            initial=initial_pose
        )
        
        # Update mapy
        self.map.add_point_cloud(aligned_scan, delta)
        
        # Loop closure detection
        if self.detect_loop_closure(aligned_scan):
            # Full optimization
            self.optimize_full_map()
        
        return delta
```

______________________________________________________________________

## 7. Segmentacja semantyczna — co to jest?

### Co widzi robot?

```
OBRAZ:                  SEGMENTACJA:
┌───────────────┐      ┌───────────────┐
│   ████       │      │   DROG  CZŁ   │
│  ██████      │  →   │   DROG  ROŚ   │
│       ░░░░   │      │        BUDY   │
│   ▓▓▓▓       │      │   SAMO CHOD   │
└───────────────┘      └───────────────┘

Klasy: droga, człowiek, roślina, budynek, samochód...
```

### DeepLabV3+ — state of the art

```python
class SemanticSegmenter:
    def __init__(self):
        self.model = load_model("deeplabv3plus.pt")
        self.classes = CLASS_NAMES
        
    def segment(self, image):
        # Output: [H, W] z class_id
        mask = self.model.predict(image)
        
        # Kolorowanie
        colored = self.colorize(mask)
        
        return colored, mask
```

______________________________________________________________________

## 8. Percepcja w czasie rzeczywistym

### Problem: Latencja

| Etap | Czas |
|------|------|
| Akwizycja | 10-30ms |
| Preprocessing | 2-5ms |
| Inferencja NN | 10-50ms |
| Postprocessing | 1-2ms |
| **Suma** | **23-87ms** |

### Optymalizacje

```python
class RealTimePerception:
    def __init__(self):
        # Modele
        self.detector = YOLOv8n()  # Najszybszy
        self.tracker = DeepSORT()
        
        # Pipeline asynchroniczny
        self.executor = ThreadPoolExecutor(max_workers=4)
        
        # Bufor klatek
        self.frame_buffer = RingBuffer(2)
        
    async def perceive_async(self, frame):
        # Równoległe przetwarzanie
        future_detect = self.executor.submit(self.detector.detect, frame)
        future_track = self.executor.submit(self.tracker.update, frame)
        
        # Czekaj na wyniki
        detections = future_detect.result()
        tracks = future_track.result()
        
        return {
            'detections': detections,
            'tracks': tracks,
            'timestamp': time.time()
        }
    
    def perceive_low_latency(self, frame):
        # Dla krytycznych ścieżek:
        # 1. Mniejszy input
        frame_small = cv2.resize(frame, (320, 320))
        
        # 2. Lighter model
        detections = self.detector.detect(frame_small)
        
        # 3. Upscale wyników
        detections = self.upscale(detections, frame.shape)
        
        return detections
```

______________________________________________________________________

## 9. Zastosowania praktyczne

### Robot magazynowy

```python
class WarehousePerception:
    def __init__(self):
        # Percepcja 360°
        self.cameras = [
            FrontCamera(), 
            BackCamera(),
            LeftCamera(),
            RightCamera()
        ]
        
        # LiDAR do przeszkód
        self.lidar = LiDAR()
        
    def get_obstacle_map(self):
        # Połącz dane z kamer i LIDAR
        obstacles = []
        
        for cam in self.cameras:
            frame = cam.capture()
            dets = self.detector.detect(frame)
            # Filtruj: ludzie, wózki, regały
            obstacles.extend(self.filter_warehouse_objects(dets))
        
        # LiDAR - pewny zasięg
        lidar_obs = self.lidar.get_obstacles()
        obstacles.extend(lidar_obs)
        
        return obstacles
```

### Robot autonomiczny

```python
class AutonomousCarPerception:
    def __init__(self):
        self.front_camera = Camera()
        self.front_lidar = LiDAR()
        self.rear_radar = Radar()
        self.surround_cameras = [Camera() for _ in range(4)]
        
        # Fuzja
        self.fusion = SensorFusion()
        
    def perceive(self):
        # Detekcja i śledzenie obiektów
        objects = self.fusion.perceive()
        
        # Filtruj: pojazdy, piesi, rowerzystów
        relevant = self.filter_relevant(objects)
        
        # Estymacja trajektorii
        for obj in relevant:
            obj.predicted_path = self.predict_trajectory(obj)
        
        # Mapowanie dróg
        road = self.road_detector.detect()
        
        return {
            'objects': relevant,
            'road': road,
            'free_space': self.compute_free_space(road, objects)
        }
```

______________________________________________________________________

## 10. Podsumowanie

### Kluczowe zasady percepcji

1. **Multimodalność** — jeden sensor to za mało
1. **Fuzja** — łącz dane z różnych źródeł
1. **Czas rzeczywisty** — latencja = bezpieczeństwo
1. **Edge AI** — percepcja na pokładzie
1. **Redundancja** — zawsze miej backup

### Wyzwania

- Złe warunki (deszcz, mgła, ciemność)
- Odbicia, refleksy
- Dynamiczne sceny
- Ograniczone zasoby
- Bezpieczeństwo (ataki na percepcję)

### Przyszłość

- Neural Radiance Fields (NeRF)
- Foundation models dla robotyki
- Percepcja ucząca się
- Quantum sensing

______________________________________________________________________

## Pytania do dyskusji

1. Czy robot może "widzieć" lepiej niż człowiek? W czym?
1. Kto odpowiada za wypadek autonomicznego samochodu?
1. Jak atakować systemy percepcji? Jak się bronić?

______________________________________________________________________

## Źródła

- OpenCV Documentation
- YOLO Papers (Redmon et al.)
- ORB-SLAM Papers
- "Computer Vision: Algorithms and Applications" - Szeliski
