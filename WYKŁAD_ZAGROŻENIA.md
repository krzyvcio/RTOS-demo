# Wykład: Zagrożenia dla systemów robotyki — dark side of the force

______________________________________________________________________

## Wprowadzenie: Zanim zbudujesz robota, poznaj jego wrogów

Każda technologia ma ciemną stronę.

Roboty mogą:

- Zabijać (celowo lub przez błąd)
- Być bronią
- Zastąpić miliony miejsc pracy
- Być zhakowane
- Działać przeciwko człowiekowi

W tym wykładzie poznasz:

- Co może pójść nie tak w systemach robotycznych
- Kto i jak atakuje roboty
- Konsekwencje błędów
- Jak się bronić

Zaczynamy podróż do ciemnej strony robotyki.

______________________________________________________________________

## 1. Katastrofy robotyczne — when robots go wrong

### 1.1. Industrial robots

```python
# Fukushimski reactor - roboty które nie dały rady

INCIDENT = """
2011 - Fukushima Daiichi:

Co poszło nie tak:
1. Roboty dostały zbyt wysoki poziom promieniowania
2. Sprzęt elektroniczny przestał działać
3. Komunikacja z robotami została utracona
4. Ludzie musieli wejść do strefy śmiertelnej

Konsekwencje:
- Setki tysięcy ofiar
- Miliony dolarów strat
- Decennia czyszczenia

Lekcja: Roboty nie są niezawodne w ekstremalnych warunkach
"""
```

### 1.2. Medical robots

```python
# Therac-25 - śmiertelna dawka promieniowania

INCIDENT = """
1985-1987 - Therac-25 (terapia radiacyjna):

Co poszło nie tak:
1. Błąd w oprogramowaniu
2. Pacjent dostał 100x większą dawkę
3. 6 osób zmarło, wiele poparzonych

Przyczyna:
- Race condition w kodzie
- Brak redundantnych zabezpieczeń
- Zaufanie do oprogramowania

Lekcja: Software może zabijać
"""
```

### 1.3. Autonomous vehicles

```python
# Uber ATG - potrącenie pieszej

INCIDENT = """
2018 - Uber Autonomous Vehicle:

Co poszło nie tak:
1. AI wykryła pieszą
2. System zdecydował że nie trzeba reagować
3. Piesza zginęła

Przyczyny:
- Zbytni的前提 trust w AI
- Brak backup systemu
- Niewystarczające testy

Lekcja: AI nie jest doskonała
"""
```

### 1.4. Military robots

```python
# Drony - broń autonomiczna

CONCERN = """
Pytanie etyczne:

Czy robot może sam decydować o zabiciu?

 Argumenty ZA:
- Szybsze reakcje niż człowiek
- Brak emocji
- Precyzyjne ataki

 Argumenty PRZECIW:
- Kto odpowiada za błędy?
- AI może się mylić
- Wymyka się spod kontroli
- Zbrojenie autonomiczne

Rozwiązanie: Człowiek w pętli (human-in-the-loop)
"""
```

______________________________________________________________________

## 2. Typy zagrożeń

### 2.1. Zagrożenia sprzętowe

| Zagrożenie | Przykład | Skutek |
|------------|----------|---------|
| **Awaria komponentu** | Spalony silnik | Robot staje |
| **Zużycie** | Zużyte łożyska | Awaria mechaniczna |
| **Warunki środowiskowe** | Wysoka temperatura | Przegrzanie |
| **Promieniowanie** | SEU w kosmosie | Błędy w pamięci |
| **Zakłócenia EMI** | Szum w sensorach | Błędne dane |

```python
# Przykład: EMI zakłóca sensory

class EMIDetector:
    def detect_anomaly(self, sensor_data):
        # Wykryj czy dane wyglądają realistycznie
        if self.is_physically_impossible(sensor_data):
            # Może EMI?
            self.flag_anomaly()
            return self.use_redundant_sensor()
        return sensor_data
```

### 2.2. Zagrożenia programowe

```python
# Katastrofalne scenariusze software

SOFTWARE_FAILURES = {
    "infinite_loop": "Robot przestaje reagować",
    "memory_leak": "Po czasie = crash",
    "race_condition": "Nieprzewidywalne zachowanie", 
    "integer_overflow": "Błędne obliczenia",
    "null_pointer": "Segmentation fault = reset",
    "deadlock": "Robot się zawiesza",
    "stack_overflow": "Nieprzewidywalne crashy"
}
```

### 2.3. Zagrożenia AI/ML

```python
# AI nie jest doskonała

AI_FAILURES = {
    "adversarial_attack": """
        Atakujący nakleja wzór na znak STOP
        AI: "To nie jest znak STOP"
        Robot: Jedzie dalej!
    """,
    
    "distribution_shift": """
        Model nauczony w Kalifornii
        Testowany w Norwegii
        Śnieg = chaos
    """,
    
    "overfitting": """
        Robot uczony na idealnych danych
        W realnym świecie: 
        "Nie widziałem takiego przypadku"
    """,
    
    "bias": """
        AI trenuje na danych z jednego regionu
        W innym regionie: błędne decyzje
    """
}
```

### 2.4. Zagrożenia sieciowe

```python
# Ataki na systemy robotyczne

NETWORK_ATTACKS = {
    "denial_of_service": """
        DDoS na systemie sterowania
        Robot traci połączenie
        Co teraz? Emergency stop!
    """,
    
    "man_in_the_middle": """
        Atakujący przechwytuje komunikację
        Modyfikuje polecenia
        "Zwolnij" -> "Przyspiesz"
    """,
    
    "replay_attack": """
        Nagrywanie poleceń
        Odtwarzanie w pętli
        Robot: "Jedziesz w kółko"
    """,
    
    "firmware_malware": """
        Złośliwy firmware przez OTA
        Backdoor w systemie
        Pełna kontrola atakującego
    """
}
```

______________________________________________________________________

## 3. Wrogowie robotów

### 3.1. Hakerzy

```python
# Kto atakuje i dlaczego?

THREAT_ACTORS = {
    "script_kiddies": {
        "motywation": "fun",
        "skill": "low",
        "target": "any vulnerable robot"
    },
    
    "cybercriminals": {
        "motywation": "ransomware",
        "skill": "medium",
        "target": "industrial robots"
    },
    
    "state_actors": {
        "motywation": "espionage/sabotage",
        "skill": "very high",
        "target": "military, critical infrastructure"
    },
    
    "insiders": {
        "motywation": "revenge/financial",
        "skill": "high",
        "target": "company robots"
    }
}
```

### 3.2. Przypadkowe zagrożenia

```python
# Ludzie też są zagrożeniem

HUMAN_ERRORS = {
    "misconfiguration": """
        Admin: "Zmienię jedno ustawienie"
        System: "Wszystko przestaje działać"
    """,
    
    "poor_testing": """
        Test: "Działa na moim laptopie"
        Produkcja: "Czemu wszystko pada?"
    """,
    
    "social_engineering": """
        Phishing: "Dzień dobry, IT"
        Pracownik: "Oto hasło"
    """,
    
    "complacency": """
        "Zawsze działało"
        Nikt nie sprawdza bezpieczeństwa
    """
}
```

______________________________________________________________________

## 4. Konsekwencje — co jest na stawce?

### 4.1. Ludzkie życie

```python
# Cena błędu = życie ludzkie

LIVES_AT_STAKE = {
    "medical_robot": """
        Błąd w robotcie chirurgicznym
        = Śmierć pacjenta
    """,
    
    "autonomous_vehicle": """
        Błąd w aucie bez kierowcy
        = Śmierć pasażerów/innych
    """,
    
    "industrial_robot": """
        Błąd w fabryce
        = Śmiertelne wypadki
    """,
    
    "military_robot": """
        Błąd w dronie
        = Ofiary cywilne
    """
}
```

### 4.2. Ekonomia

```python
# Koszty awarii

INCIDENT_COSTS = {
    "toyota_recall_2009": "$16 billion",
    "boeing_737_max": "$20 billion", 
    "therac_25_lawsuit": "$150 million",
    "uber_atg_settlement": "$1.5 billion",
    "samsung_note7": "$17 billion"
}
```

### 4.3. Społeczeństwo

```python
# Wpływ na społeczeństwo

SOCIAL_IMPACTS = {
    "unemployment": """
        Automatyzacja -> bezrobocie
        Niektóre prace znikną na dobre
    """,
    
    "inequality": """
        Roboty = dla bogatych
        Pogłębienie nierówności
    """,
    
    "weaponization": """
        Broń autonomiczna
        Nowy wyścig zbrojeń
    """,
    
    "privacy": """
        Roboty widzą wszystko
        Kto ma dostęp do danych?
    """
}
```

______________________________________________________________________

## 5. Specyficzne zagrożenia — szczegóły

### 5.1. Zagrożenia AI

```python
# Attack vectors na AI w robotyce

AI_ATTACKS = {
    "model_inversion": """
        Atakujący odtwarza dane treningowe
        Prywatne dane wyciekają
    """,
    
    "model_poisoning": """
        Zatruwanie danych treningowych
        Model uczy się złych rzeczy
    """,
    
    "adversarial_examples": """
        Niewidoczne wzory które oszukują AI
        Stop = dwa paski = nierozpoznawalne
    """,
    
    "extraction": """
        Atakujący kradnie model
        Może używać za darmo
    """
}
```

### 5.2. Zagrożenia dla sensorów

```python
# Ataki na zmysły robota

SENSOR_ATTACKS = {
    "lidar_spoofing": """
        Fałszywe punkty w chmurze
        Robot: "Jest ścieżka" (nie ma!)
    """,
    
    "camera_blinding": """
        Światło w kamerę
        Robot: "Nic nie widzę"
    """,
    
    "gps_spoofing": """
        Fałszywe sygnały GPS
        Robot: "Jestem w zupełnie innym miejscu"
    """,
    
    "acoustic_attacks": """
        Dźwięki niesłyszalne dla ludzi
        Wpływają na czujniki
    """
}
```

### 5.3. Zagrożenia dla aktuatorów

```python
# Ataki na sterowanie

ACTUATOR_ATTACKS = {
    "command_injection": """
        Wstrzykiwanie fałszywych komend
        "Zatrzymaj się" -> "Jedź dalej"
    """,
    
    "actuator_feedback_attack": """
        Fałszywy feedback
        Robot myśli że wykonał ruch
        W rzeczywistości: stoi
    """,
    
    "override_safety": """
        Wyłączenie limitów bezpieczeństwa
        Więcej mocy = więcej ryzyka
    """
}
```

______________________________________________________________________

## 6. Przypadki z życia

### 6.1. Stuxnet

```python
"""
2010 - Stuxnet:

Co to było:
- Cyberbroń przeciwko irańskim wirówkom
- Zmieniała prędkość wirówki
- Wyglądało na awarię

Skutek:
- 1000 wirówki zniszczone
- Opóźnienie programu nuklearnego Iranu o lata

Lekcja: 
- Roboty przemysłowe mogą być celem
- Stuxnet otworzył erę cyberbroni
"""
```

### 6.2. Jeep Cherokee Hack

```python
"""
2015 - Jeep Cherokee:

Haker (Miller & Valasek):
- Włamali się przez infotainment
- Przejęli kontrolę nad:
  - Klimatyzacją
  - Wycieraczkami
  - Hamulcami
  - Kierownicą

Ogromny recall Chrysler!
Lektura: Każdy system może być zhakowany
"""
```

### 6.3. Robotaxis incidents

```python
"""
2023 - Robotaxi (Cruise, Waymo):

Wypadki:
- Piesza wciągnięta pod koło
- Zablokowane na skrzyżowaniu
- Nieprzewidywalne zachowanie

Ludzie:
- Atakują roboty
- Blokują drogi
- Nie ufają autonomii

Lekcja: Public acceptance = kluczowe
"""
```

______________________________________________________________________

## 7. Jak się bronić

### 7.1. Defense in Depth

```python
# Wiele warstw ochrony

SECURITY_LAYERS = {
    "layer1": "Fizyczna izolacja",
    "layer2": "Sieciowa segmentacja", 
    "layer3": "Szyfrowanie komunikacji",
    "layer4": "Autentykacja i autoryzacja",
    "layer5": "Monitorowanie i alerting",
    "layer6": "Anomaly detection",
    "layer7": "Incident response"
}
```

### 7.2. Safety by Design

```python
# Bezpieczeństwo od początku

SAFETY_PRINCIPLES = {
    "fail_safe": "Przy błędzie = bezpieczny stan",
    "redundancy": "Podwójne/trojne systemy krytyczne",
    "watchdog": "Zawsze monitoruj",
    "graceful_degradation": "Stopniowa utrata funkcji",
    "human_override": "Człowiek może przejąć kontrolę",
    "audit_log": "Wszystko logowane"
}
```

### 7.3. Red Team Testing

```python
# Atakujmy sami siebie!

RED_TEAM_CHECKLIST = {
    "penetration_testing": "Szukaj podatności",
    "social_engineering": "Phishing pracowników",
    "physical_access": "Co z fizycznym dostępem?",
    "supply_chain": "Czy oprogramowanie jest bezpieczne?",
    "failure_modes": "Co się dzieje gdy...",
    "stress_testing": "Testuj do granic"
}
```

______________________________________________________________________

## 8. Przyszłe zagrożenia

### 8.1. AI Arms Race

```python
"""
PRZYSZŁOŚĆ:

Broń autonomiczna + AI =
Nieśmiertelne armie
Kontrolowane przez algorytmy
Kto decyduje o ataku?

Międzynarodowe traktaty:
- Zakaz broni autonomicznej?
- Kto egzekwuje?
"""
```

### 8.2. Mass Unemployment

```
PRZYSZŁOŚĆ:

Roboty zastępują pracę:
- Kierowcy (autonomiczne)
- Kasjerki (self-checkout)
- Pisarze (LLM)
- Lekarze (AI diag)

Co robić z milionami bezrobotnych?
Basic Income?
Przekwalifikowanie?
Rewolucja?
```

### 8.3. Robot Uprising (science fiction?)

```
Czy to możliwe?

Nie w najbliższej przyszłości.

Ale pytanie:
- Czy AI może mieć własne cele?
- Co jeśli cele AI nie = cele ludzi?
- Jak kontrolować superinteligencję?

To pytania na które nie mamy odpowiedzi.
```

______________________________________________________________________

## 9. Checklista bezpieczeństwa

### Dla projektantów

- [ ] Threat modeling
- [ ] Security by design
- [ ] Penetration testing
- [ ] Red team exercises
- [ ] Incident response plan
- [ ] Regularne aktualizacje

### Dla operatorów

- [ ] Monitoring 24/7
- [ ] Training pracowników
- [ ] Backup i disaster recovery
- [ ] Compliance z regulacjami

### Dla regulatorów

- [ ] Standardy bezpieczeństwa
- [ ] Wymagania certyfikacji
- [ ] Odpowiedzialność prawna
- [ ] Ochrona konsumentów

______________________________________________________________________

## 10. Podsumowanie

### Kluczowe zasady

1. **Zagrożenia są realne** — nie ignoruj
1. **Defense in depth** — wiele warstw
1. **Fail safe** — bezpieczny stan zawsze
1. **Test to failure** — atakuj siebie
1. **Monitoring** — wiesz co się dzieje

### Pamiętaj

- Każda technologia może być użyta źle
- Bezpieczeństwo to proces, nie produkt
- Lepsze zapobiegać niż leczyć
- Etyka = nie wszystko co możliwe = dozwolone

______________________________________________________________________

## Pytania do dyskusji

1. Czy powinniśmy zakazać broni autonomicznej?
1. Kto ponosi odpowiedzialność za błędy AI?
1. Jak chronić prywatność w świecie robotów?

______________________________________________________________________

## Źródła

- "The Art of Exploitation" - Jon Erickson
- "Security Engineering" - Ross Anderson
- NIST Cybersecurity Framework
- ISO/IEC 27001
