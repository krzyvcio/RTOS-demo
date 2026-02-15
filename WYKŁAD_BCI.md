# Wykład: Interfejsy mózg-komputer — sterowanie myślami

______________________________________________________________________

## Wprowadzenie: Granice człowieka i maszyny

Interfejs mózg-komputer (BCI - Brain-Computer Interface) to technologia, która pozwala na bezpośrednią komunikację między mózgiem a urządzeniem zewnętrznym.

W robotyce BCI otwiera zupełnie nowe możliwości:

- Sterowanie protezami neuralnymi
- Interakcja człowiek-robot bez fizycznego interfejsu
- Rehabilitacja po urazach rdzenia kręgowego
- Wspomaganie osób z niepełnosprawnościami

Ale to też technologia z ogromnym ryzykiem — myśli to najintymniejsze dane jakie posiadamy.

______________________________________________________________________

## 1. Anatomia interfejsu mózg-komputer

### Rodzaje BCI

| Typ | Inwazyjność | Sygnał | Rozdzielczość | Zastosowanie |
|-----|-------------|---------|---------------|--------------|
| **Inwazyjny** | Elektrody w korze mózgowej | LFP, single-unit | Bardzo wysoka | Protezy neuralne |
| **Półinwazyjny** | Elektrody na powierzchni | ECoG | Wysoka | Badania, neurofeedback |
| **Nieinwazyjny** | Elektrody na skórze | EEG | Niska | Sterowanie, badania |
| **Optyczny** | Bliskie podczerwieni | fNIRS | Średnia | Fuzja z EEG |

### Jak to działa (EEG)

```
Mózg → Aktywność elektryczna → Elektrody na skórze 
→ Wzmacniacz → ADC → Feature Extraction → Klasyfikacja → Komenda
```

```python
# Przykład: prosty pipeline EEG

import mne
import numpy as np

class BCIProcessor:
    def __init__(self):
        self.model = load_model("motor_imagery_classifier")
        self.sampling_rate = 250  # Hz
        
    def process(self, raw_data):
        # 1. Preprocessing
        filtered = self.bandpass_filter(raw_data, 8, 30)  # Pasmo mu/beta
        
        # 2. Artifact removal
        clean = self.remove_artifacts(filtered)
        
        # 3. Feature extraction
        features = self.extract_csp(clean)  # Common Spatial Patterns
        
        # 4. Classification
        intent = self.model.predict(features)
        
        return self.map_to_command(intent)
    
    def bandpass_filter(self, data, low, high):
        # Filtr Butterwortha
        return mne.filter.filter_data(
            data, 
            sfreq=self.sampling_rate,
            l_freq=low,
            h_freq=high,
            method='iir'
        )
```

______________________________________________________________________

## 2. BCI w robotyce — zastosowania

### 2.1. Sterowanie protez

```python
# Neural decoder dla protezy ręki

class NeuralDecoder:
    def __init__(self):
        self.rnn = load_model("lstm_neural_decoder")
        self.kalman = KalmanFilter()
        
    def decode(self, neural_signals):
        # LSTM przetwarza sygnał neuralny
        latent = self.rnn.predict(neural_signals)
        
        # Kalman filter mapuje do przestrzeni ruchu
        position = self.kalman.predict(latent)
        velocity = self.kalman.update(position)
        
        # Komenda dla protezy
        return self.prosthesis_command(position, velocity)
    
    def map_to_command(self, position, velocity):
        # Mapowanie do 10 stopni swobody protezy
        commands = {
            'grip': self.compute_grip(position),
            'wrist_flex': position[0],
            'wrist_rot': position[1],
            'fingers': self.compute_finger_positions(position[2:])
        }
        return commands
```

### 2.2. Teleoperacja

```
[Operator z EEG] → [Dekoder] → [Komenda] → [Robot] → [Wykonanie]
                                    ↓
                            [Kamera] → [Wizja] → [Biofeedback]
                                    ↓
                            [Wrażenie wzrokowe]
```

```python
class BCI_Teleoperation:
    def __init__(self):
        self.bci = BCIProcessor()
        self.robot = RobotInterface()
        self.feedback = VisualFeedback()
        
    def operate(self):
        while True:
            # Odczytaj sygnał EEG
            eeg_data = self.eeg_device.read(duration=0.1)
            
            # Zdekoduj intencję
            intent = self.bci.process(eeg_data)
            
            # Wykonaj komendę
            self.robot.execute(intent)
            
            # Biofeedback
            view = self.robot.get_camera_feed()
            self.feedback.show(view)
            
            # Korekta w czasie rzeczywistym
            correction = self.bci.get_correction(eeg_data, view)
            if correction:
                self.robot.adjust(correction)
```

### 2.3. Rehabilitacja

```python
# Robot rehabilitacyjny wspomagany BCI

class RehabilitationBCI:
    def __init__(self):
        self.bci = BCIProcessor()
        self.exoskeleton = Exoskeleton()
        self.therapy = TherapySession()
        
    def session(self, patient_id, duration_minutes):
        # Sprawdź czy pacjent jest gotowy
        if not self.bci.is_ready(patient_id):
            self.calibrate(patient_id)
        
        start_time = time.time()
        
        while time.time() - start_time < duration_minutes * 60:
            # Odczytaj intencję ruchu
            intent = self.bci.process_current()
            
            if intent == "MOVE":
                # Wspomaganie ruchu przez egzoszkielet
                self.exoskeleton.assist(
                    direction=self.bci.get_direction(),
                    force=THERAPY_FORCE_LEVEL
                )
                self.therapy.record("assisted_move", time.time())
                
            elif intent == "REST":
                self.exoskeleton.release()
                
            elif intent == "RESIST":
                # Ćwiczenie czynne - pacjent sam wykonuje ruch
                self.exoskeleton.set_mode("active")
                self.therapy.record("active_move", time.time())
            
            # Adaptacyjne dostosowanie trudności
            self.therapy.adapt_difficulty()
        
        return self.therapy.get_summary()
```

______________________________________________________________________

## 3. Wyzwania techniczne

### 3.1. Szum i artefacty

```python
class ArtifactRemover:
    def __init__(self):
        self.ica = load_model("ica_decomposition")
        
    def remove_ocular(self, eeg):
        # Wykrywanie artefaktów ocznych (EOG)
        eog_channels = self.get_eog_channels(eeg)
        
        if self.detect_artifact(eog_channels):
            # ICA dekompozycja
            components = self.ica.decompose(eeg)
            
            # Usuń komponenty związane z oczami
            clean_components = self.remove_eye_artifacts(components)
            
            # Rekonstrukcja
            return self.ica.reconstruct(clean_components)
        
        return eeg
    
    def remove_muscle(self, eeg):
        # Artefakty mięśniowe (EMG) - wysokie częstotliwości
        high_freq = self.bandpass_filter(eeg, 30, 100)
        
        if self.detect_muscle_activity(high_freq):
            # Podobna procedura jak dla EOG
            return self.remove_component(eeg, 'muscle')
        
        return eeg
```

### 3.2. Transfer między użytkownikami

Problem: każdy mózg jest inny. Model wytrenowany na jednej osobie nie działa na innej.

```python
class TransferLearningBCI:
    def __init__(self):
        self.base_model = load_model("base_eeg_encoder")
        self.calibration_samples = 100
        
    def adapt_to_user(self, user_id):
        # Pobierz dane kalibracyjne
        calibration_data = self.get_calibration_data(user_id)
        
        if len(calibration_data) >= self.calibration_samples:
            # Fine-tune ostatnich warstw
            self.base_model.fine_tune(
                calibration_data,
                epochs=10,
                learning_rate=0.001
            )
        else:
            # Jeśli za mało danych - użyj metadata transfer
            user_metadata = self.get_user_metadata(user_id)
            self.apply_meta_learning(user_metadata)
    
    def apply_meta_learning(self, metadata):
        # MAML-like approach
        # Model uczy się jak się uczyć z małej próbki
        adapted_params = self.maml_adapt(
            self.base_model.parameters(),
            metadata
        )
        return adapted_params
```

### 3.3. Latencja

W robotyce latencja jest krytyczna.

| Etap | Typowa latencja | Cel |
|------|-----------------|-----|
| Akwizycja EEG | 10-50ms | < 100ms |
| Preprocessing | 5-20ms | < 50ms |
| Klasyfikacja | 10-100ms | < 100ms |
| Wykonanie robota | 1-10ms | < 50ms |
| **Suma** | **26-180ms** | **< 300ms** |

```python
# Optymalizacja dla niskiej latencji

class LowLatencyBCI:
    def __init__(self):
        # Model quantization - szybsze inferencje
        self.classifier = load_quantized_model("model_int8.bin")
        
        # Async processing
        self.executor = ThreadPoolExecutor(max_workers=2)
        
    def process_async(self, eeg_data):
        # Równoległy preprocessing i inference
        future = self.executor.submit(self.classifier.predict, eeg_data)
        
        # In the meantime - odbierz nowe dane
        new_data = self.get_next_chunk()
        
        # Czekaj na wynik (timeout!)
        try:
            result = future.result(timeout=0.1)
            return result
        except TimeoutError:
            # Fallback do poprzedniej komendy
            return self.last_known_command
```

______________________________________________________________________

## 4. Zagrożenia i bezpieczeństwo

### 4.1. Prywatność myśli

```python
# Zagrożenie: Odczyt emocji/intencji bez zgody

# Obrona: Szyfrowanie sygnału EEG

from cryptography.fernet import Fernet

class SecureBCI:
    def __init__(self):
        self.key = self.load_hardware_key()  # TPM/HSM
        self.cipher = Fernet(self.key)
        
    def transmit_encrypted(self, eeg_data):
        # Szyfruj przed transmisją
        encrypted = self.cipher.encrypt(eeg_data.tobytes())
        return self.send_secure(encrypted)
    
    def process_with_consent(self, eeg_data):
        # Sprawdź consent bit
        if not self.verify_consent(eeg_data):
            raise SecurityError("No consent for processing")
        
        # Ale też - czy to na pewno ten użytkownik?
        if not self.verify_identity(eeg_data):
            raise SecurityError("Identity mismatch")
        
        return self.process(eeg_data)
```

### 4.2. Ataki na BCI

| Atak | Skutek | Obrona |
|------|--------|--------|
| **Signal jamming** | Brak możliwości sterowania | Redundancja (inne wejście) |
| **Injection** | Fałszywe komendy | Wieloczynnikowa autoryzacja |
| **Brain malware** | Infekcja przez neural implant | Secure boot, signed code |
| **Extraction** | Wyciek danych myśli | Szyfrowanie end-to-end |

```python
# Obrona przed injection attack

class BCI_Integrity_Check:
    def __init__(self):
        self.baseline = self.learn_baseline()
        
    def verify_command_integrity(self, command, eeg_raw):
        # Sprawdź czy komenda jest spójna z sygnałem EEG
        
        # 1. Czy command odpowiada intention w EEG?
        predicted_intent = self.decode_intention(eeg_raw)
        if predicted_intent != command.intent:
            log.warning("Intent mismatch - possible injection!")
            return False
        
        # 2. Czy timing jest możliwy?
        if not self.verify_timing(command.timestamp, eeg_raw):
            log.warning("Timing anomaly")
            return False
        
        # 3. Czy amplitude są możliwe?
        if not self.verify_amplitude(eeg_raw):
            log.warning("Amplitude anomaly")
            return False
        
        return True
```

______________________________________________________________________

## 5. Etyka i społeczeństwo

### Problemy etyczne

1. **Inwazyjność** — Czy mamy prawo wkładać elektrody do mózgu?
1. **Prywatność** — Czy "myśli" powinny być chronione jak dane osobowe?
1. **Tożsamość** — Gdzie kończy się człowiek a zaczyna maszyna?
1. **Dostęp** — Czy BCI pogłębi nierówności?

### Ramy prawne

```python
# Propozycja: Prawa neuralne

NEURAL_RIGHTS = {
    # Prawo do prywatności myśli
    "right_to_mental_privacy": {
        "description": "Prawo do kontrolowania swoich danych neuralnych",
        "implementation": "Opt-in consent, encryption, right to deletion"
    },
    
    # Prawo do integralności neuralnej
    "right_to_neural_integrity": {
        "description": "Prawo do nienaruszalności neuralnego systemu",
        "implementation": "Anti-tampering, secure boot, audit logs"
    },
    
    # Prawo do autonomii
    "right_to_autonomy": {
        "description": "Prawo do decyzji o używaniu BCI",
        "implementation": "Easy on/off, override capability"
    },
    
    # Prawo do niedyskryminacji
    "right_to_non_discrimination": {
        "description": "Zakaz dyskryminacji ze względu na używanie BCI",
        "implementation": "Anti-bias testing, transparency"
    }
}
```

______________________________________________________________________

## 6. Przyszłość BCI

### Krótkoterminowo (5 lat)

- Nieinwazyjne BCI do sterowania egzoszkieletami
- Neuralne protezy z lepszą rozdzielczością
- BCI do terapii stroke/urazów

### Średnioterminowo (10 lat)

- Półinwazyjne implanty (stenter)
- Dwukierunkowa komunikacja (odczyt + stymulacja)
- BCI do augmentacji

### Długoterminowo (20+ lat)

- W pełni inwacyje neuralne
- Bezpośrednia wymiana informacji między mózgami
- Integracja z AI

______________________________________________________________________

## 7. Podsumowanie

### Kluczowe zasady

1. **Intencja > Sygnał** — Dekoduj co użytkownik chce zrobić
1. **Latencja jest krytyczna** — < 300ms od myśli do działania
1. **Adaptacja** — Model musi się uczyć użytkownika
1. **Bezpieczeństwo** — Myśli to najwrażliwsze dane
1. **Etyka** — Technologia służy człowiekowi

### Wyzwania

- Szum i artefakty
- Transfer między użytkownikami
- Niska rozdzielczość nieinwazyjnych metod
- Bezpieczeństwo i prywatność
- Etyka i regulacje

______________________________________________________________________

## Pytania do dyskusji

1. Czy pracodawca powinien mieć prawo wymagać BCI do pracy?
1. Kto ponosi odpowiedzialność za błędną komendę z BCI - użytkownik czy producent?
1. Czy powinniśmy pozwolić na augmentację zdrowych ludzi?

______________________________________________________________________

## Źródła

- IEEE Brain Initiative
- Neuralink Documentation
- OpenBCI SDK
- "Brain-Computer Interfaces" - Nature
