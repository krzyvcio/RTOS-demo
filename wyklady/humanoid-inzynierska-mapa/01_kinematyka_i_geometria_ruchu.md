# Wykład 1: Kinematyka, geometria ruchu, manipulatory

## Czesc I: Wstep teoretyczny — dlaczego kinematyka jest fundamentem

### 1.1 Geneza — od czego zaczynamy

Proszę wyobrazić sobie sytuację: budujemy robota humanoidalnego. Mamy 30 stopni swobody (przegubów), kilka łańcuchów kinematycznych (nogi, ręce, głowa), i chcemy, żeby robot podszedł do stołu, sięgnął po kubek i wrócił.

**Gdzie zaczynamy?**

Odpowiedź brzmi: od pytania "gdzie jest moja ręka i gdzie będzie, jeśli poruszę przegubami?"

To jest dokładnie pytanie, na które odpowiada **kinematyka**.

### 1.2 Dlaczego to jest takie wazne

Profesor zawsze powtarza: **"Kinematyka to język, którym robot mówi o pozycjach. Bez niego nie da się zaplanować ruchu, sterować ani estymować stanu."**

Wyobraźmy sobie konsekwencje błędnej kinematyki:

- Planujesz trajektorię ręki — robot uderza w przeszkodę
- Estymujesz pozycję stopy — robot stawia stopę w powietrzu
- Sterujesz chodem — robot traci równowagę

Wszystko zaczyna się od pytania: **"gdzie jest ta część robota?"**

### 1.3 Co to jest kinematyka

Kinematyka to dział mechaniki opisujący ruch ciał bez uwzględniania sił. W kontekście robota:

- **Pozycja** — gdzie jest dany punkt (x, y, z)
- **Orientacja** — jak jest obrócony ( Roll, Pitch, Yaw lub kąty Eulera)
- **Prędkość** — jak szybko się zmienia
- **Przyspieszenie** — jak szybko zmienia się prędkość

**Kinematyka prosta vs odwrotna:**

```
Kinematyka prosta (Forward Kinematics - FK):
    q (przegięby) → T (transformacja) → pozycja końcówki

Kinematyka odwrotna (Inverse Kinematics - IK):
    pozycja końcówki → q (przeguby)
```

---

## Czesc II: Notacja i obiekty geometryczne

### 2.1 Układy współrzędnych (ramki)

W praktyce wszystko sprowadza się do spójnej definicji układów współrzędnych (ramek):

```
Ramka świata (World Frame):
    |
    +-- Ramka bazy (Base Frame) - tułów robota
            |
            +-- Ramka przedramienia (Forearm Frame)
                    |
                    +-- Ramka dłoni (Hand Frame)
```

Każda ramka ma:
- **Origin** — punkt początkowy
- **Osie** — kierunki X, Y, Z

### 2.2 Transformacje sztywne

Transformacja sztywna w 3D to element grupy Liego SE(3):

```text
T = [ R  p ]     R ∈ SO(3), p ∈ R³
    [ 0  1 ]
```

Wyjaśnienie:
- **R** — macierz obrotu (3×3)
- **p** — wektor translacji (3×1)
- **SO(3)** — specjalna grupa ortogonalna (macierze obrotu)

### 2.3 Składanie transformacji

Łańcuch kinematyczny to zwykłe mnożenie macierzy:

```text
T_A^C = T_A^B × T_B^C
```

**Przykład:**
```
Ramka dłoni względem ramki świata:
T_world^hand = T_world^base × T_base^shoulder × T_shoulder_arm × T_arm_hand
```

### 2.4 Konwencje — ustal na początku projektu!

To jest **najczęstsze źródło problemów** w projektach:

| Co ustalić | Przykład |
|------------|----------|
| Kierunki osi | X "do przodu", Y "w lewo", Z "w górę" |
| Notacja transformacji | T_A^B (B względem A) lub T_B^A |
| Jednostki | Metry, radiany |
| Kierunki dodatnie przegubów | Zgodnie z regułą prawej ręki |

---

## Czesc III: Grafy i struktury kinematyczne

### 3.1 Drzewo kinematyczne (Kinematic Tree)

Najczęstsza reprezentacja: każdy link ma dokładnie jednego rodzica:

```
                    [Baza/Tułów]
                       /  |  \
                      /   |   \
                 [Noga1] [Noga2] [Ręka]
                    |         |
                 [Stopa]    [Dłoń]
```

**Zalety:**
- Szybki Forward Kinematics: `T_world^link(q)` dla wielu linków
- Szybka propagacja prędkości i przyspieszeń

### 3.2 Graf przegubów (Joint Graph)

Bardziej ogólna forma — przydatna gdy:
- Masz mechaniczne sprzężenia (np. przekładnie różnicowe)
- Pojawiają się zamknięte łańcuchy (np. chwyt obiema rękami)

### 3.3 Graf łańcucha kinematycznego

"Wycięty" fragment drzewa: baza → wybrana końcówka

```
[Baza] → [Ramię] → [Przedramię] → [Dłoń]
```

Wiele algorytmów IK pracuje na takim łańcuchu.

### 3.4 Graf kontaktów

W humanoidzie kontakty są dynamiczne:

```
Graf kontaktów:
    |
    +-- Aktywne kontakty (np. lewa stopa na podłodze)
    |       +-- punkt kontaktu
    |       +-- normalna kontaktu
    |       +-- tarcie (stożek tarcia)
    |
    +-- Model kontaktu (twardy/miękki)
```

### 3.5 Graf brył sztywnych (Rigid Body Graph)

Wierzchołki = bryły, krawędzie = przeguby/wiązania

---

## Czesc IV: Reprezentacja orientacji

### 4.1 Trzy sposoby reprezentacji

| Reprezentacja | Liczby | Zalety | Wady |
|---------------|--------|--------|-------|
| Macierz obrotu R ∈ SO(3) | 9 | Brak osobliwości | 9 liczb, nadmiarowe |
| Kwaterniony | 4 | Kompaktowe, brak osobliwości | Wymaga normalizacji, znak ±q |
| Kąty Eulera/RPY | 3 | Intuicyjne | Osobliwości (gimbal lock) |

### 4.2 Zasada praktyczna

> **Do obliczeń i propagacji używaj SO(3) lub kwaternionów.**
> **Do interfejsów i wizualizacji możesz użyć RPY, ale nie mieszaj ich w pętli sterowania!**

### 4.3 Problem z kątami Eulera

Kąty Eulera mają **osobliwości** — sytuacje, gdy tracisz stopień swobody:

```python
# Przykład: yaw + pitch 90° = problem
# RPY może "zwinąć" się do osobliwości
# W pętli sterowania - KATASTROFA!
```

---

## Czesc V: SO(3), SE(3), algebry Liego

### 5.1 Grupy Liego w robotyce

Grupy Liego są naturalnym językiem dla ruchu brył sztywnych:

- **SO(3)** — obroty (rotacje)
- **SE(3)** — obroty + translacje (transformacje sztywne)
- **se(3)** — algebra Liego SO(3)/SE(3) (infinitesymalne obroty)

### 5.2 Co to daje w praktyce

**Spójny "błąd" w przestrzeni konfiguracji:**

```text
E = log( T_des⁻¹ × T(q) ) ∈ se(3)
```

gdzie `log` mapuje transformację z SE(3) do wektora skrętu (twist) w `se(3)`.

**Mapy log/exp** — do małych przyrostów:

```python
# Mały obrót
delta_R = exp(theta * axis)

# Odwrotnie: obrót → kąt+axis
axis, theta = log(R)
```

---

## Czesc VI: FK i Jacobian

### 6.1 Forward Kinematics

FK dostaje konfiguracje przegubów `q` i zwraca transformacje `T(q)` dla wybranych linków.

```python
# Pseudokod FK
def forward_kinematics(q):
    T = identity()
    for joint in chain:
        T = T * joint.transform(q[joint])
    return T
```

### 6.2 Jacobian

Jacobian łączy prędkości przegubów z prędkością końcówki:

```text
V = J(q) × qd
```

**Praktycznie potrzebujesz:**
- Jacobian dla końcówki (dłoń, stopa)
- Jacobian dla środka masy (CoM)
- Jacobian dla orientacji tułowia

```python
# Obliczanie Jacobianu
def compute_jacobian(q):
    # Geometria przegubów → macierz J
    # J = [J_linear; J_angular]
    return J
```

---

## Czesc VII: Odwrotna kinematyka (IK)

### 7.1 Problem IK

Najprostszy model:
```text
znajdź q, aby T(q) ≈ T_des
```

W praktyce IK to optymalizacja z ograniczeniami:
- Błędy zadania (pozycja/orientacja)
- Limity przegubów: `q_min ≤ q ≤ q_max`
- Limity prędkości i przyspieszeń
- Unikanie kolizji

### 7.2 Metody rozwiazywania

**Podejście iteracyjne:**
```text
q_{k+1} = q_k + Δq
```

gdzie `Δq` wynika z przybliżenia liniowego błędu.

### 7.3 Damped Least Squares (DLS)

Problem przy osobliwościach — DLS dodaje tłumienie:

```text
Δq = J^T (J × J^T + λ²I)⁻¹ × e
```

**Intuicja:**
- Gdy J×J^T ma małe wartości własne (osobliwość)
- Składnik λ²I stabilizuje odwracanie

---

## Czesc VIII: Osobliwości i analiza SVD

### 8.1 Czym sa osobliwości

Osobliwość to sytuacja, gdy tracisz stopień swobody w przestrzeni zadania.

**Przykład planarnego ramienia 2-DOF:**
```
Gdy ramię jest wyprostowane:
- Możesz poruszać końcówką tylko w jednym kierunku
- Drugi kierunek = "zniknął"
```

### 8.2 Wykrywanie osobliwości

Przez wartości osobliwe Jacobianu:

```python
# SVD Jacobianu
U, S, V = np.linalg.svd(J)

# Miara osobliwości
sigma_min = S[-1]  # Najmniejsza wartość osobliwa
condition_number = S[0] / S[-1]  # Uwarunkowanie
```

### 8.3 Co robisz w systemie

- Zwiększasz damping λ
- Zmieniasz priorytety zadań
- Przełączasz parametry (np. śledzenie tylko pozycji, bez orientacji)

---

## Czesc IX: Praktyka inzynierska

### 9.1 Najczestsze problemy

| Problem | Skutek | Rozwiązanie |
|---------|--------|-------------|
| Niespójne ramki | Błędne trajektorie | Dokumentacja konwencji |
| Brak limitów | IK wysyła przeguby poza zakres | Dodaj ograniczenia |
| Brak tłumienia | Oscylacje przy osobliwościach | DLS z λ |
| RPY w pętli | Gimbal lock | Kwaterniony/SO(3) |

### 9.2 Testy sanity

**Najszybszy test Jacobianu:**
```python
# Różniczkowanie numeryczne FK
def test_jacobian():
    for _ in range(100):
        q = random_config()
        
        # FK -> numeryczny Jacobian
        J_numeric = numerical_jacobian(fk, q)
        
        # FK -> analityczny Jacobian  
        J_analytic = compute_jacobian(q)
        
        assert np.allclose(J_numeric, J_analytic, atol=1e-5)
```

---

## Czesc X: Podsumowanie i checklisty

### Checklisty:

- [ ] Testy jednostkowe FK dla losowych q i porównanie z symulatorem
- [ ] Testy Jacobianu przez różniczkowanie numeryczne
- [ ] Logowanie σ_min(J) i ||Δq|| jako metryk "zdrowia" IK
- [ ] Twarde ograniczenia na prędkość/akcelerację w warstwie wykonawczej

### Zasady:

| Zasada | Wyjaśnienie |
|--------|-------------|
| Konwencje na początku | Dokumentuj i trzymaj się |
| Kwaterniony w obliczeniach | Nie RPY w pętli |
| Ograniczenia zawsze | IK bez limitów = katastrofa |
| Testuj przez różniczkowanie | Numeryczne vs analityczne |

---

## Czesc XI: Pytania do dyskusji

1. Jak sprawdzisz poprawność FK i Jacobianu bez "zaufania" do jednej biblioteki?
2. Dlaczego DLS/Levenberg–Marquardt pomaga przy osobliwościach i jak dobrać damping λ?
3. Jakie ograniczenia (limity, kolizje) muszą być w IK, żeby wynik był użyteczny w realnym robocie?
4. Kiedy lepiej śledzić tylko pozycję końcówki, a kiedy pozycję i orientację?

---

## Czesc XII: Zadania praktyczne

### Zadanie 1: FK + testy regresji

Zaimplementuj Forward Kinematics dla przykładowego łańcucha i porównaj z:
- Symulatorem (np. PyBullet)
- Różniczkowaniem numerycznym

### Zadanie 2: IK z DLS

Zaimplementuj IK z Damped Least Squares z limitami przegubów:
- Obsługa osobliwości przez SVD
- Ograniczenia q_min, q_max

### Zadanie 3: Wizualizacja manipulowalności

Stwórz "mapę osobliwości" dla wybranego łańcucha:
- Przestrzeń konfiguracji
- σ_min(J) jako kolor
- Wizualizacja 2D/3D

---

## BONUS: Test sanity

Najszybszy test sanity dla Jacobianu: różniczkowanie numeryczne FK i porównanie z J(q) w kilku losowych punktach oraz blisko osobliwości.

---

*(Koniec wykladu 1)*
