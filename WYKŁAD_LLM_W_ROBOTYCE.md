# Wykład: LLM i Generative AI w robotyce — gdy roboty uczą się mówić i myśleć

______________________________________________________________________

## Wprowadzenie: Koniec ery głupich automatów

Przez dekady roboty były głupie:

- Musiałyśmy im powiedzieć dokładnie co mają zrobić
- Każdy ruch = tysiące linii kodu
- Brak zrozumienia kontekstu
- Zero elastyczności

Ale nadeszła nowa era:

**LLM (Large Language Models)** i **Generative AI** zmieniają wszystko.

Teraz robot może:

- Zrozumieć polecenie w języku naturalnym
- Wygenerować plan działania
- Odpowiedzieć na pytania
- Wyjaśnić swoje decyzje
- Uczyć się zamiast być programowanym

W tym wykładzie dowiesz się jak to działa i jak to połączyć z RTOS o którym już wiesz.

______________________________________________________________________

## 1. Czym jest LLM?

### Transformer — serce LLM

```
WEJŚCIE: "Podnieś kubek"
     │
     ▼
┌─────────────────┐
│   TOKENIZER     │  ← podziel na tokeny
│   "Podnieś"    │
│   "kubek"       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ TRANSFORMER     │  ← self-attention
│                 │
│ Query, Key,    │
│ Value matrices │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   FEED-FORWARD │
│   NETWORKS      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   SOFTMAX      │  ← prawdopodobieństwo
└────────┬────────┘
         │
         ▼
WYJŚCIE: "Prawdopodobieństwo następnego tokenu"
```

### Jak LLM rozumie świat?

```
Tradycyjne AI:          LLM:
                      
Symbol → Reguła      Symbol → Wektor (embedding)
"kubek" = KUBEK       "kubek" = [0.12, -0.34, 0.56, ...]
                      
Reguły = program       Wektory = wiedza

Problem: reguł miliony  Rozwiązanie: wektory!
```

______________________________________________________________________

## 2. LLM w robotyce — zastosowania

### 2.1. Natural Language Interface

```python
# Robot rozumie polecenia po polsku!

class NaturalLanguageController:
    def __init__(self):
        self.llm = load_model("llama-3-8b")  # lub GPT-4
        self.skills = SkillLibrary()
        
    def understand_and_execute(self, command):
        """
        Konwertuj polecenie na akcję
        """
        # 1. Parsowanie rozumienia
        understanding = self.llm.chat(
            f"""You are a robot controller. 
            Break down this command into steps:
            {command}
            
            Output format:
            - action: <verb>
            - object: <noun>  
            - constraints: <list>
            """
        )
        
        # 2. Mapowanie na umiejętności
        action_plan = self.skill_library.map(understanding)
        
        # 3. Walidacja bezpieczeństwa
        if not self.safety_validator.validate(action_plan):
            return "Nie mogę wykonać - niebezpieczne!"
        
        # 4. Wykonanie
        result = self.executor.run(action_plan)
        
        return result
```

**Przykłady:**

| Polecenie | Akcja |
|-----------|-------|
| "Weź kubek ze stołu" | 1. Znajdź kubek 2. Podejdź 3. Chwyć 4. Unieś |
| "Idź do kuchni" | 1. Lokalizacja 2. Mapowanie 3. Nawigacja |
| "Omiń przeszkodę" | 1. Detekcja 2. Planowanie ścieżki 3. Unikanie |

### 2.2. Code Generation

````python
# Robot sam pisze swój kod!

class CodeGeneration:
    def __init__(self):
        self.llm = load_model("code-llama")
        
    def generate_control_code(self, task_description):
        """
        Generuj kod sterowania na podstawie opisu
        """
        code = self.llm.chat(
            f"""Write Python code for a robot control task.
            
            Task: {task_description}
            
            Requirements:
            - Use ROS2
            - Include error handling
            - Be deterministic (no random)
            - Include safety checks
            
            Code:
            ```python
            import rclpy
            from rclpy.node import Node
            
            class ControlNode(Node):
                def __init__(self):
                    super().__init__('controller')
                    # TODO: add code
            ```
            """
        )
        
        # Walidacja wygenerowanego kodu
        if self.code_validator.is_safe(code):
            return self.deploy(code)
        else:
            return "Generated code not safe, retrying..."
````

### 2.3. Reasoning and Planning

```python
# Robot myśli zanim działa!

class ReasoningRobot:
    def __init__(self):
        self.llm = load_model("gpt-4-turbo")
        self.world_model = WorldModel()
        
    def plan_with_reasoning(self, goal, observation):
        """
        Robot myśli "co by było gdybym..."
        """
        # 1. Symulacja mentalna
        reasoning = self.llm.chat(
            f"""You are a robot. Think step by step about how to achieve:
            
            Goal: {goal}
            
            Current state: {observation}
            
            Think about:
            1. What information is missing?
            2. What could go wrong?
            3. What is the safest approach?
            4. What resources do I need?
            
            Output a detailed plan with contingencies.
            """
        )
        
        # 2. Walidacja planu
        valid_plan = self.validate_plan(reasoning)
        
        # 3. Wykonanie z monitorowaniem
        return self.execute_with_monitoring(valid_plan)
```

______________________________________________________________________

## 3. Architektura LLM + RTOS

### Problem: LLM jest ciężki!

| Model | Parametry | RAM | Latencja (GPU) |
|-------|-----------|-----|----------------|
| GPT-4 | 1.7T | 100GB+ | sekundy |
| LLaMA 70B | 70B | 140GB | sekundy |
| LLaMA 7B | 7B | 14GB | sekundy |
| TinyLLaMA | 1B | 2GB | ~100ms |
| **Robot** | **Tiny!** | **\<100MB** | **\<10ms** |

### Rozwiązanie: Distributed Architecture

```
                    ┌──────────────────┐
                    │   CLOUD LLM      │
                    │   (GPT-4)        │
                    │ - Planowanie     │
                    │ - Debugowanie    │
                    │ - Uczenie       │
                    └────────┬─────────┘
                             │
                    (OTA / WiFi)
                             │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│   ROBOT 1     │   │   ROBOT 2     │   │   ROBOT 3     │
│  Tiny LLM     │   │  Tiny LLM     │   │  Tiny LLM     │
│  (onboard)    │   │  (onboard)    │   │  (onboard)    │
│ - Lokalne decyzje│  │ - Lokalne decyzje│  │ - Lokalne decyzje│
│ - RTOS control │   │ - RTOS control │   │ - RTOS control │
└───────────────┘   └───────────────┘   └───────────────┘
```

### Edge LLM inference

```python
# LLM na tablecie NVIDIA Jetson

class EdgeLLM:
    def __init__(self):
        # Kwantyzowany model (INT4)
        self.model = load_quantized("tinyllama-1b-int4.bin")
        
        # Specyficzny dla robotyki
        self.prompt_template = """
        You are a robot controller.
        Current sensors: {sensors}
        Available actions: {actions}
        
        What should you do next? Choose from available actions.
        Respond with just the action and parameters.
        """
        
    def think(self, sensors, actions):
        # Szybkie wnioskowanie
        prompt = self.prompt_template.format(
            sensors=sensors,
            actions=actions
        )
        
        # Maksymalnie 50 tokenów output
        response = self.model.generate(
            prompt,
            max_new_tokens=50,
            temperature=0.1  # Mało losowości!
        )
        
        return self.parse_action(response)
```

______________________________________________________________________

## 4. Multimodalne LLM — robot widzi i rozumie

### Vision-Language Models (VLM)

```python
# Robot widzi i rozumie co widzi!

class VisionLanguageRobot:
    def __init__(self):
        self.vlm = load_model("llava-1.5-13b")  # vision + language
        self.planner = TaskPlanner()
        
    def see_and_understand(self, image):
        # Zadaj pytanie o obraz
        response = self.vlm.chat(
            image=image,
            text="What do you see? What's in this image?"
        )
        
        return response
    
    def execute_visual_command(self, command, image):
        """
        "Weź tę czerwoną puszkę"
        """
        # 1. Zrozum co jest na obrazie
        understanding = self.vlm.chat(
            image=image,
            text=f"Identify: {command}. Return object coordinates."
        )
        
        # 2. Znajdź obiekt
        object_pos = self.understand_to_position(understanding)
        
        # 3. Wykonaj
        return self.manipulator.grab(object_pos)
```

______________________________________________________________________

## 5. Generatywna AI w robotyce

### 5.1. Motion Generation

```python
# Wygeneruj ruch na podstawie opisu!

class MotionDiffusion:
    def __init__(self):
        # Diffusion model dla trajektorii
        self.diffusion = load_model("motion-diffusion-model")
        
    def generate_motion(self, task_description, start_state):
        """
        "Podnieś rękę płynnym ruchem"
        """
        # Warunki
        conditions = {
            'task': task_description,
            'start': start_state,
            'constraints': ['smooth', 'no_collision']
        }
        
        # Generuj trajektorię
        trajectory = self.diffusion.sample(
            conditions=conditions,
            steps=50  # diffusion steps
        )
        
        return trajectory
```

### 5.2. World Model

```python
# Robot wyobraża sobie przyszłość!

class WorldModel:
    def __init__(self):
        # Model świata - co się stanie?
        self.model = load_model("world-model-v2")
        
    def imagine(self, current_state, action_sequence):
        """
        Symuluj przyszłość bez wykonywania!
        """
        # Predykcja następnych stanów
        future_states = []
        state = current_state
        
        for action in action_sequence:
            next_state = self.model.predict(state, action)
            future_states.append(next_state)
            state = next_state
        
        return future_states
    
    def find_best_action(self, goal, current_state):
        """
        MPC z LLM jako prior!
        """
        # LLM sugeruje plan
        suggested_plan = self.llm.suggest(
            f"How would you achieve {goal} from {current_state}?"
        )
        
        # Model świata weryfikuje
        simulated = self.imagine(current_state, suggested_plan)
        
        # Wybierz najlepszy
        return self.select_best(suggested_plan, simulated)
```

### 5.3. Sim-to-Real Transfer

```python
# Generuj dane treningowe w symulacji!

class SyntheticDataGenerator:
    def __init__(self):
        self.diffusion = load_model("grasp-generator")
        
    def generate_training_data(self, object_type, n_samples):
        """
        Wygeneruj miliony przykładów chwytania!
        """
        dataset = []
        
        for _ in range(n_samples):
            # Losowe parametry
            obj_pose = random_pose()
            gripper_pose = random_pose()
            grasp_quality = random()
            
            # Generuj obraz syntetyczny
            image = self.renderer.render(obj_pose, gripper_pose)
            
            dataset.append({
                'image': image,
                'grasp_pose': gripper_pose,
                'success': grasp_quality > 0.8
            })
        
        # Trenuj model na danych syntetycznych
        model = train_grasp_model(dataset)
        
        return model
```

______________________________________________________________________

## 6. Safety i LLM — krytyczne!

### Problem: Halucynacje

```
LLM: "Wejdź po schodach"
Robot: *wchodzi w ścianę*

LLM nie wie że jest ściana!
```

### Rozwiązania

```python
class SafeLLMController:
    def __init__(self):
        self.llm = load_model("robot-llama")
        self.safety = SafetyValidator()
        self.world_model = WorldModel()
        
    def safe_execute(self, command):
        # 1. Wygeneruj plan
        plan = self.llm.generate_plan(command)
        
        # 2. Waliduj w modelu świata
        for action in plan:
            if not self.world_model.is_safe(action):
                return "Niebezpieczne!"
        
        # 3. Sprawdź constraints
        if not self.safety.validate(plan):
            return "Narusza zasady bezpieczeństwa!"
        
        # 4. Monitoruj wykonanie
        return self.execute_with_watchdog(plan)
```

### Guardrails

```python
class Guardrails:
    """
    Zasady których LLM musi przestrzegać
    """
    
    RULES = """
    NEVER:
    - Move faster than 0.5 m/s near humans
    - Approach closer than 0.3m to humans
    - Use more than 10N force
    - Enter restricted zones
    
    ALWAYS:
    - Check sensors before moving
    - Stop if sensor fails
    - Return to safe state on error
    - Report confidence level
    """
    
    def __init__(self):
        self.llm = load_model("guardrail-llama")
        
    def check(self, plan):
        """
        Sprawdź czy plan nie łamie zasad
        """
        response = self.llm.judge(
            f"Does this plan violate any rules?\n\nRules:\n{self.RULES}\n\nPlan:\n{plan}"
        )
        
        if "VIOLATES" in response:
            return False, response
        
        return True, "OK"
```

______________________________________________________________________

## 7. Przypadki użycia

### 7.1. Robot domowy

```
"Przygotuj mi kanapkę"

LLM: *rozkłada na kroki*
1. Znajdź chleb
2. Znajdź masło  
3. Otwórz szufladę
4. Weź nóż
5. Posmaruj chleb
6. Dodaj składniki
7. Podaj

RTOS: Wykonuje każdy krok z bezpieczeństwem
```

### 7.2. Robot przemysłowy

```
"Zrób inspekcję hali"

LLM: Planuje trasę
- Które strefy?
- Co sprawdzić?
- W jakiej kolejności?

RTOS: Wykonuje nawigację, unikanie ludzi
```

### 7.3. Robot ratunkowy

```
"Gdzie są roz Survivors?"

LLM: *analizuje obrazy z drona*
- Widzę strefę zniszczeń
- Możliwe ofiary w sektorze B
- Droga zablokowana

RTOS: Nawiguje bezpiecznie
```

______________________________________________________________________

## 8. Przyszłość: Foundation Models dla robotyki

### Robot Foundation Model

```python
# Jeden model dla wszystkiego!

class RobotFoundationModel:
    """
    Jak GPT dla robotyki!
    """
    def __init__(self):
        # Model na miliardy parametrów
        # Trenowany na milionach robotów
        self.model = load_model("robot-foundation-v1")
        
    def generalize(self, task_description, observations):
        """
        "Zrób coś czego nigdy nie robiłem"
        """
        # Model extrapoluje z doświadczenia!
        return self.model.predict(task_description, observations)
```

### Simulacja = nauka

```
PRZESZŁOŚĆ:            TERAŹNIEJSZOŚĆ:
                        
Tysiące godzin        Miliony godzin
programowania          w symulacji
                        
Teraz robot może się uczyć 
w 10 minut więcej niż 
człowiek przez całe życie!
```

______________________________________________________________________

## 9. Podsumowanie

### Co daje LLM w robotyce?

| Zastosowanie | Korzyść |
|--------------|---------|
| **Natural Interface** | Każdy może sterować robotem |
| **Code Generation** | Robot sam się programuje |
| **Reasoning** | Robot myśli przed działaniem |
| **Multimodal** | Robot rozumie co widzi |
| **Generative** | Robot wyobraża sobie przyszłość |

### Ryzyka

- Halucynacje — robot robi głupoty
- Bezpieczeństwo — LLM może dać złe polecenie
- Latencja — myślenie trwa czasem sekundy
- Koszt — duże modele = duże GPU

### Złote zasady

1. **LLM jako planer, RTOS jako wykonawca**
1. **Zawsze waliduj wyjście LLM**
1. **Safety guardrails to podstawa**
1. **Edge + Cloud = optymalne**
1. **Testuj w symulacji zanim na realu**

______________________________________________________________________

## Pytania do dyskusji

1. Czy robot z LLM może być "świadomy"? Gdzie granica?
1. Kto odpowiada za błędy robota - LLM czy operator?
1. Czy LLM powinien mieć "prawo veta" przed wykonaniem?

______________________________________________________________________

## Źródła

- "Language Models as Zero-Shot Planners" - Huang et al.
- "Code as Policies" - Google Robotics
- "PaLM-E: An Embodied Multimodal Language Model"
- "RT-2: Vision-Language-Action Models"
