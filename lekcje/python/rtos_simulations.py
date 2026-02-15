"""
RTOS HAZARDS - PYTHON SIMULATIONS
=================================

Kolekcja symulacji zagrożeń w systemach czasu rzeczywistego.
Python służy do demonstracji i wizualizacji.

KATEGORIE:
1. Synchronizacja - deadlock, race conditions
2. Timing - jitter, deadline miss
3. Scheduling - priorytety, preemption
4. Komunikacja - kolejki, message passing
5. Safety - watchdog, failsafe
"""

import threading
import time
import random
import queue
import multiprocessing
from dataclasses import dataclass
from typing import List, Dict, Optional
from enum import Enum
from collections import deque
import matplotlib.pyplot as plt
import numpy as np

# ============================================================================
# TYPY I STAŁE
# ============================================================================

class TaskState(Enum):
    READY = "ready"
    RUNNING = "running"
    BLOCKED = "blocked"
    SUSPENDED = "suspended"

@dataclass
class Task:
    id: int
    name: str
    priority: int  # Wyższy = ważniejszy
    period_ms: float
    wcet_ms: float
    deadline_ms: float
    state: TaskState = TaskState.READY
    remaining_ms: float = 0.0
    next_activation_ms: float = 0.0
    deadline_misses: int = 0
    preemptions: int = 0

# ============================================================================
# #PY_SYNC_001 - DEADLOCK SYMULACJA
# ============================================================================

def deadlock_ab_ba_demo():
    """
    Symulacja klasycznego deadlocku AB-BA.

    Wątek 1: lock(A) -> lock(B)
    Wątek 2: lock(B) -> lock(A)

    Wynik: Deadlock!
    """
    lock_a = threading.Lock()
    lock_b = threading.Lock()

    stop_event = threading.Event()
    success = [True]  # Mutable dla closure

    def thread_1():
        with lock_a:
            print("[T1] Mam lock A")
            time.sleep(0.1)  # Daj czas T2 na lock B
            print("[T1] Czekam na lock B...")
            acquired = lock_b.acquire(timeout=2.0)
            if not acquired:
                print("[T1] DEADLOCK! Nie mogę zdobyć B")
                success[0] = False
            else:
                print("[T1] Mam oba locki")
                lock_b.release()

    def thread_2():
        with lock_b:
            print("[T2] Mam lock B")
            time.sleep(0.1)
            print("[T2] Czekam na lock A...")
            acquired = lock_a.acquire(timeout=2.0)
            if not acquired:
                print("[T2] DEADLOCK! Nie mogę zdobyć A")
                success[0] = False
            else:
                print("[T2] Mam oba locki")
                lock_a.release()

    t1 = threading.Thread(target=thread_1)
    t2 = threading.Thread(target=thread_2)

    t1.start()
    t2.start()

    t1.join()
    t2.join()

    return success[0]

def deadlock_solution_ordered():
    """
    Rozwiązanie: Zawsze blokuj w tej samej kolejności.
    """
    lock_a = threading.Lock()
    lock_b = threading.Lock()

    def thread_1():
        with lock_a:
            print("[T1] Mam lock A")
            time.sleep(0.01)
            with lock_b:  # Ta sama kolejność: A -> B
                print("[T1] Mam oba locki - SUKCES")

    def thread_2():
        with lock_a:  # Ta sama kolejność: A -> B
            print("[T2] Mam lock A")
            time.sleep(0.01)
            with lock_b:
                print("[T2] Mam oba locki - SUKCES")

    t1 = threading.Thread(target=thread_1)
    t2 = threading.Thread(target=thread_2)

    t1.start()
    t2.start()

    t1.join()
    t2.join()

    return True

# ============================================================================
# #PY_SYNC_002 - RACE CONDITION SYMULACJA
# ============================================================================

def race_condition_demo():
    """
    Symulacja race condition na współdzielonym liczniku.
    """
    counter = [0]  # Mutable list dla closure

    def increment():
        for _ in range(10000):
            # Race condition: read-modify-write bez synchronizacji
            temp = counter[0]
            time.sleep(0.000001)  # Symulacja opóźnienia
            counter[0] = temp + 1

    threads = [threading.Thread(target=increment) for _ in range(10)]

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    print(f"Wynik: {counter[0]} (oczekiwano: 100000)")
    print(f"Utracone incrementy: {100000 - counter[0]}")

    return counter[0] == 100000

def race_condition_solution_lock():
    """
    Rozwiązanie: Mutex dla synchronizacji.
    """
    counter = [0]
    lock = threading.Lock()

    def increment():
        for _ in range(10000):
            with lock:
                counter[0] += 1

    threads = [threading.Thread(target=increment) for _ in range(10)]

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    print(f"Wynik (z lock): {counter[0]}")
    return counter[0] == 100000

# ============================================================================
# #PY_TIME_001 - SCHEDULER SYMULACJA
# ============================================================================

class RTOSSimulator:
    """
    Prosty symulator RTOS preemptive priority-based.
    """

    def __init__(self, tasks: List[Task]):
        self.tasks = tasks
        self.current_time = 0.0
        self.running_task: Optional[Task] = None
        self.timeline: List[Dict] = []
        self.context_switches = 0

    def tick(self, delta_ms: float = 1.0):
        """Pojedynczy tick symulacji."""
        self.current_time += delta_ms

        # Aktywuj zadania na ich okresie
        for task in self.tasks:
            if self.current_time >= task.next_activation_ms:
                task.state = TaskState.READY
                task.remaining_ms = task.wcet_ms
                task.next_activation_ms += task.period_ms
                self._log(f"[ACTIVATE] {task.name}", "activation")

        # Wybierz zadanie do uruchomienia (preemptive priority)
        ready_tasks = [t for t in self.tasks
                      if t.state == TaskState.READY and t.remaining_ms > 0]

        if ready_tasks:
            # Sort by priority (descending)
            ready_tasks.sort(key=lambda t: -t.priority)
            best_task = ready_tasks[0]

            # Context switch?
            if self.running_task != best_task:
                if self.running_task and self.running_task.state == TaskState.RUNNING:
                    self.running_task.state = TaskState.READY
                    self.running_task.preemptions += 1
                    self._log(f"[PREEMPT] {self.running_task.name} -> {best_task.name}", "preempt")

                self.running_task = best_task
                best_task.state = TaskState.RUNNING
                self.context_switches += 1
                self._log(f"[RUN] {best_task.name}", "switch")

            # Wykonaj
            best_task.remaining_ms -= delta_ms

            # Check deadline
            deadline_remaining = (best_task.next_activation_ms - best_task.period_ms +
                                  best_task.deadline_ms - self.current_time)
            if deadline_remaining < 0:
                best_task.deadline_misses += 1
                self._log(f"[DEADLINE MISS] {best_task.name}", "deadline")

            # Zakończ?
            if best_task.remaining_ms <= 0:
                best_task.state = TaskState.SUSPENDED
                self.running_task = None
                self._log(f"[COMPLETE] {best_task.name}", "complete")
        else:
            # Idle
            self.running_task = None

        # Log timeline
        self.timeline.append({
            'time': self.current_time,
            'task': self.running_task.name if self.running_task else 'IDLE',
            'task_id': self.running_task.id if self.running_task else -1
        })

    def run(self, duration_ms: float):
        """Uruchom symulację."""
        while self.current_time < duration_ms:
            self.tick()

    def get_stats(self) -> Dict:
        """Pobierz statystyki."""
        return {
            'context_switches': self.context_switches,
            'deadline_misses': sum(t.deadline_misses for t in self.tasks),
            'preemptions': sum(t.preemptions for t in self.tasks),
            'timeline': self.timeline
        }

    def _log(self, msg: str, category: str):
        """Loguj zdarzenie."""
        pass  # Można dodać real logging

def simulate_priority_inversion():
    """
    Symulacja inwersji priorytetów.
    """
    tasks = [
        Task(0, "LOW",    1, period_ms=100, wcet_ms=50, deadline_ms=100),
        Task(1, "MED",    2, period_ms=80,  wcet_ms=20, deadline_ms=80),
        Task(2, "HIGH",   3, period_ms=50,  wcet_ms=10, deadline_ms=50),
    ]

    sim = RTOSSimulator(tasks)
    sim.run(200)

    print("Symulacja Priority Scheduling:")
    print(f"  Context switches: {sim.context_switches}")
    print(f"  Deadline misses: {sum(t.deadline_misses for t in tasks)}")

    return sim

# ============================================================================
# #PY_TIME_002 - JITTER ANALYSIS
# ============================================================================

def jitter_simulation():
    """
    Symulacja i analiza jittera w zadaniu RT.
    """
    nominal_period = 10.0  # ms
    num_cycles = 100

    # Generuj rzeczywiste czasy aktywacji z jitterem
    base_time = 0.0
    activation_times = []

    for i in range(num_cycles):
        # Jitter z różnych źródeł
        isr_jitter = random.uniform(-0.5, 0.5)  # ISR latency
        scheduler_jitter = random.uniform(-0.3, 0.3)  # Scheduling delay
        cache_jitter = random.uniform(-0.2, 0.2)  # Cache miss

        total_jitter = isr_jitter + scheduler_jitter + cache_jitter
        actual_time = base_time + total_jitter

        activation_times.append(actual_time)
        base_time += nominal_period

    # Analiza jittera
    expected_times = [i * nominal_period for i in range(num_cycles)]
    jitters = [actual - expected for actual, expected
               in zip(activation_times, expected_times)]

    jitter_stats = {
        'min': min(jitters),
        'max': max(jitters),
        'avg': sum(jitters) / len(jitters),
        'std': np.std(jitters),
        'peak_to_peak': max(jitters) - min(jitters)
    }

    print("Analiza Jitter:")
    print(f"  Min: {jitter_stats['min']:.3f} ms")
    print(f"  Max: {jitter_stats['max']:.3f} ms")
    print(f"  Avg: {jitter_stats['avg']:.3f} ms")
    print(f"  Std: {jitter_stats['std']:.3f} ms")
    print(f"  Peak-to-Peak: {jitter_stats['peak_to_peak']:.3f} ms")

    return jitter_stats, jitters

# ============================================================================
# #PY_TIME_003 - DEADLINE MISS ANALYSIS
# ============================================================================

def deadline_miss_simulation():
    """
    Symulacja przekroczenia deadline.
    """
    tasks = [
        Task(0, "Control",   3, period_ms=10,  wcet_ms=6,  deadline_ms=8),
        Task(1, "Sensor",    2, period_ms=20,  wcet_ms=8,  deadline_ms=15),
        Task(2, "Comm",      1, period_ms=50,  wcet_ms=20, deadline_ms=40),
    ]

    sim = RTOSSimulator(tasks)
    sim.run(1000)

    for task in tasks:
        print(f"{task.name}:")
        print(f"  Deadline misses: {task.deadline_misses}")
        print(f"  Preemptions: {task.preemptions}")

    return sim

# ============================================================================
# #PY_COMM_001 - MESSAGE QUEUE SIMULATION
# ============================================================================

def message_queue_demo():
    """
    Symulacja kolejki komunikatów producer-consumer.
    """
    q = queue.Queue(maxsize=10)
    produced = []
    consumed = []

    stop_event = threading.Event()

    def producer():
        for i in range(50):
            try:
                q.put(i, timeout=0.1)
                produced.append(i)
                print(f"[PROD] Produced: {i}, Queue size: {q.qsize()}")
            except queue.Full:
                print(f"[PROD] Queue FULL! Dropped: {i}")

    def consumer():
        while not stop_event.is_set() or not q.empty():
            try:
                item = q.get(timeout=0.5)
                consumed.append(item)
                print(f"[CONS] Consumed: {item}, Queue size: {q.qsize()}")
            except queue.Empty:
                pass

    p = threading.Thread(target=producer)
    c = threading.Thread(target=consumer)

    p.start()
    c.start()

    p.join()
    stop_event.set()
    c.join()

    print(f"\nProduced: {len(produced)}, Consumed: {len(consumed)}")
    print(f"Lost: {len(produced) - len(consumed)}")

    return len(produced) == len(consumed)

# ============================================================================
# #PY_SAFE_001 - WATCHDOG SIMULATION
# ============================================================================

class Watchdog:
    """
    Symulacja watchdog timera.
    """

    def __init__(self, timeout_ms: float):
        self.timeout_ms = timeout_ms
        self.last_feed = 0.0
        self.current_time = 0.0
        self.expired = False
        self.feed_count = 0

    def feed(self):
        """Karm watchdog."""
        self.last_feed = self.current_time
        self.feed_count += 1

    def tick(self, delta_ms: float):
        """Tick czasowy."""
        self.current_time += delta_ms

        if self.current_time - self.last_feed > self.timeout_ms:
            self.expired = True

    def is_ok(self) -> bool:
        """Sprawdź czy watchdog jest OK."""
        return not self.expired

def watchdog_simulation():
    """
    Symulacja watchdog timeout.
    """
    wd = Watchdog(timeout_ms=100.0)

    # Normal operation - feed watchdog
    for i in range(10):
        wd.tick(10.0)
        wd.feed()
        print(f"Tick {i}: Watchdog OK, feeds: {wd.feed_count}")

    # Simulate task hang - stop feeding
    print("\n--- Task hangs, no more feeds ---")
    for i in range(15):
        wd.tick(10.0)
        if wd.is_ok():
            print(f"Tick {i}: Watchdog OK (time since feed: {wd.current_time - wd.last_feed:.0f}ms)")
        else:
            print(f"Tick {i}: WATCHDOG EXPIRED! System reset!")
            break

    return wd.expired

# ============================================================================
# WIZUALIZACJA
# ============================================================================

def plot_gantt_chart(simulator: RTOSSimulator):
    """
    Rysuj wykres Gantta z timeline symulacji.
    """
    timeline = simulator.timeline

    # Agreguj dane
    task_names = list(set(t['task'] for t in timeline))
    task_colors = {name: plt.cm.tab10(i) for i, name in enumerate(task_names)}

    fig, ax = plt.subplots(figsize=(12, 4))

    current_task = None
    start_time = 0

    for entry in timeline:
        if entry['task'] != current_task:
            if current_task is not None:
                ax.barh(0, entry['time'] - start_time, left=start_time,
                       color=task_colors[current_task], height=0.8)
            current_task = entry['task']
            start_time = entry['time']

    ax.set_xlabel('Czas (ms)')
    ax.set_yticks([])
    ax.set_title('RTOS Execution Timeline')

    # Legenda
    handles = [plt.Rectangle((0,0),1,1, color=task_colors[n]) for n in task_names]
    ax.legend(handles, task_names, loc='upper right')

    plt.tight_layout()
    plt.savefig('gantt_chart.png', dpi=150)
    plt.show()

def plot_jitter_histogram(jitters: List[float]):
    """
    Rysuj histogram jittera.
    """
    fig, ax = plt.subplots(figsize=(10, 4))

    ax.hist(jitters, bins=20, edgecolor='black', alpha=0.7)
    ax.axvline(x=0, color='r', linestyle='--', label='Nominal')
    ax.axvline(x=sum(jitters)/len(jitters), color='g', linestyle='-', label='Average')

    ax.set_xlabel('Jitter (ms)')
    ax.set_ylabel('Count')
    ax.set_title('Jitter Distribution')
    ax.legend()

    plt.tight_layout()
    plt.savefig('jitter_histogram.png', dpi=150)
    plt.show()

# ============================================================================
# MAIN
# ============================================================================

if __name__ == "__main__":
    print("=" * 60)
    print("RTOS HAZARDS - PYTHON DEMONSTRATIONS")
    print("=" * 60)

    print("\n--- #PY_SYNC_001: Deadlock Demo ---")
    deadlock_ab_ba_demo()

    print("\n--- #PY_SYNC_002: Race Condition Demo ---")
    race_condition_demo()

    print("\n--- #PY_TIME_001: Scheduler Simulation ---")
    sim = simulate_priority_inversion()

    print("\n--- #PY_TIME_002: Jitter Analysis ---")
    jitter_stats, jitters = jitter_simulation()

    print("\n--- #PY_SAFE_001: Watchdog Simulation ---")
    watchdog_simulation()

    print("\n--- Visualization ---")
    # Uncomment to generate plots:
    # plot_gantt_chart(sim)
    # plot_jitter_histogram(jitters)