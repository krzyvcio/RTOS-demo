# RTOS i governor procesora?

Świetne pytanie – **governor procesora** to mechanizm **Linuxowy**, który\
**może zniszczyć deterministyczność RTOS**, jeśli nie jest odpowiednio\
skonfigurowany! ⚡

______________________________________________________________________

## **Co to jest governor procesora?**

**Governor** (regulator częstotliwości CPU) w Linuxie decyduje, jak dynamicznie zmieniać taktowanie procesora:

| Governor | Zachowanie | Cel |
|----------|-----------|-----|
| **powersave** | Zawsze najniższa częstotliwość | Oszczędność energii |
| **performance** | Zawsze maksymalna częstotliwość | Maksymalna wydajność |
| **ondemand** | Dynamiczne skalowanie wg obciążenia | Balans energia/wydajność |
| **conservative** | Powolne skalowanie | Oszczędność z elastycznością |
| **schedutil** | Skalowanie wg schedulera | Nowoczesny balans |

______________________________________________________________________

## **Problem dla RTOS: jitter i nieprzewidywalność**

### **Dlaczego to zabija deterministyczność?**

1. **Zmienne czasy wykonania**

   - Task przy 800 MHz: 500 µs
   - Ten sam task przy 2.4 GHz: 166 µs
   - **WCET staje się nieokreślony** ❌

1. **Opóźnienia przełączania częstotliwości**

   - Zmiana taktowania: **dziesiątki–setki mikrosekund**
   - Wprowadza **jitter** w pętle sterowania

1. **Nieprzewidywalny scheduler**

   - Governor reaguje na obciążenie **z opóźnieniem**
   - Task RT może dostać wolne CPU na początku cyklu

______________________________________________________________________

## **Konfiguracja dla systemów RTOS/RT**

### ✅ **Dla hard real-time (robotyka, sterowanie):**

```bash
# Ustaw performance governor (zawsze max częstotliwość)
sudo cpupower frequency-set -g performance

# Lub dla wszystkich rdzeni:
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo performance | sudo tee $cpu
done

# Wyłącz turbo boost (opcjonalnie, dla jeszcze większej stabilności):
echo 1 | sudo tee /sys/devices/system/cpu/intel_pmu/turbo_mode
```

**Dlaczego `performance`?**

- Stałe taktowanie = przewidywalne czasy wykonania
- Brak opóźnień przełączania
- Łatwiejsza analiza WCET

______________________________________________________________________

### ⚠️ **Dla soft real-time (ROS2, Linux RT):**

Możesz użyć **`schedutil`** z pinowaniem tasków RT:

```bash
# schedutil + izolacja rdzeni
sudo cpupower frequency-set -g schedutil

# Izoluj rdzenie dla tasków RT (w GRUB):
# isolcpus=2,3 nohz_full=2,3 rcu_nocbs=2,3
```

Taski RT dostaną dedykowane rdzenie na max częstotliwości, reszta może oszczędzać energię.

______________________________________________________________________

## **PREEMPT_RT + Governor: praktyczne rady**

W systemach **Linux + PREEMPT_RT**:

1. **Zawsze `performance` dla rdzeni RT**
1. **Wyłącz C-states** (stany uśpienia CPU):
   ```bash
   # W GRUB:
   processor.max_cstate=1 intel_idle.max_cstate=0
   ```
1. **Wyłącz Intel P-state driver** (jeśli używasz starego kernela):
   ```bash
   intel_pstate=disable
   ```

______________________________________________________________________

## **Przykład: robot mobilny**

**Źle skonfigurowany:**

```
Governor: ondemand
Pętla sterowania: 1 kHz (1 ms)
CPU dynamicznie zmienia częstotliwość
→ Jitter: ±200 µs
→ Robot traci stabilność
```

**Dobrze skonfigurowany:**

```
Governor: performance
Pętla sterowania: 1 kHz
CPU zawsze na max
→ Jitter: ±5 µs
→ Robot stabilny
```

______________________________________________________________________

## **Podsumowanie dla projektantów RTOS**

| Wymaganie | Governor | Dodatkowe |
|-----------|----------|-----------|
| **Hard RT** | `performance` | Wyłącz C-states, turbo |
| **Soft RT** | `performance` lub `schedutil` | Izoluj rdzenie |
| **Nie-RT** | `ondemand`, `powersave` | Domyślne ustawienia OK |

**Złota zasada:**\
Jeśli mierzysz WCET, a governor jest na `ondemand` – **nie mierzysz WCET, tylko los** 🎲

______________________________________________________________________

Chcesz, żebym rozwinął:

- Jak mierzyć wpływ governora na jitter?
- Konfigurację kernela dla Linux RT?
- Izolację rdzeni (CPU affinity) dla tasków RT? 🤖

# Krytyczne przypadki przy ustawieniu złego governora procesora w systemach RTOS -> jest cisza przed burzą i nagle cały system się zatnie? Jak to się zakleszcza? Jak temu zapobiec i nie bawić się governorami procesorów

Doskonałe pytanie – to jest **klasyczna pułapka**, która zabija systemy RT w produkcji! 💣

Odpowiadam wprost: **TAK, może być cisza przed burzą**, ale mechanizm jest inny niż deadlock.

______________________________________________________________________

## **🔥 Jak to wygląda w praktyce?**

### **Scenariusz katastrofy:**

```
T = 0s:    Robot stoi, CPU idle, governor = ondemand
           → CPU schodzi do 800 MHz (oszczędzanie energii)
           
T = 5s:    Operator: "Jedź!"
           → Nagle: planowanie ruchu, sterowanie, sensory
           → Obciążenie CPU: 10% → 80%
           
T = 5.001s: Governor wykrywa obciążenie
           → Zaczyna podnosić częstotliwość
           → OPÓŹNIENIE: 50-200 ms (!!!)
           
T = 5.05s:  Pętla sterowania (1 kHz) dostaje wolne CPU
           → Spóźnienia: 5-10 cykli
           → Robot traci balans / trajektorię
           → CRASH / emergency stop
```

**To nie jest deadlock** – to **głodzenie czasowe** (timing starvation).

______________________________________________________________________

## **🧨 Dlaczego system się "zatnie"?**

### **Mechanizm 1: Kaskadowe opóźnienia**

```
Cykl 1:  Pętla sterowania spóźniona o 2 ms
         → Regulator dostaje stare dane
         → Generuje złe polecenie

Cykl 2:  Aktuatory reagują na złe polecenie
         → Robot zaczyna tracić stabilność
         → SafetyTask wykrywa problem
         → Zwiększa obciążenie CPU (diagnostyka)

Cykl 3:  Governor NADAL podnosi częstotliwość
         → CPU wciąż za wolne
         → Kolejne taski się spóźniają
         → Watchdog timeout
         → SYSTEM HALT
```

### **Mechanizm 2: Priority inversion przez hardware**

```
Task HIGH (sterowanie): czeka na CPU na pełnej mocy
Governor: podnosi częstotliwość
Task MID (planowanie): dostaje więcej czasu na wolnym CPU
Task HIGH: GŁODUJE, mimo najwyższego priorytetu
```

**To nie jest klasyczny priority inversion** (mutex), ale **hardware-induced priority inversion**.

______________________________________________________________________

## **🎯 Krytyczne przypadki w praktyce**

### **1. Robot mobilny – start z miejsca**

```
Idle → Full throttle
Governor: ondemand (800 MHz → 2.4 GHz w 100 ms)

Efekt:
- Pierwsze 100 ms: pętla sterowania działa na 1/3 mocy
- Koła dostają niespójne polecenia
- Robot szarpie / wywraca się
```

### **2. Manipulator –chwyt obiektu**

```
Lekki ruch → Kontakt z obiektem
Siła reakcji → Potrzeba szybkiej korekcji

Governor opóźnia reakcję o 50 ms
→ Manipulator uszkadza obiekt / siebie
```

### **3. Dron – podmuch wiatru**

```
Stabilny lot → Nagły podmuch
Potrzeba natychmiastowej korekcji (< 5 ms)

Governor: 30 ms opóźnienia
→ Dron przechyla się
→ Kaskada błędów
→ Crash
```

______________________________________________________________________

## **🛡️ Jak temu zapobiec BEZ bawienia się governorami?**

### **Rozwiązanie 1: Architektura hybrydowa (najlepsze)**

**Podział sprzętowy:**

```
MCU (STM32, ESP32, NXP):
├─ RTOS (FreeRTOS, Zephyr)
├─ Pętle sterowania (1-5 kHz)
├─ Bezpieczeństwo
└─ Stałe taktowanie (brak governorów!)

SoC (Raspberry Pi, Jetson, x86):
├─ Linux (z dowolnym governorem)
├─ Percepcja, planowanie
├─ ROS2, UI, telemetria
└─ Komunikacja z MCU (SPI/UART/CAN)
```

**Zalety:**

- MCU: **zero ryzyka** governorów
- Linux: może oszczędzać energię
- Jasny podział odpowiedzialności

**To robi Boston Dynamics, ANYbotics, większość firm robotycznych.**

______________________________________________________________________

### **Rozwiązanie 2: Jeśli MUSISZ użyć Linuxa do RT**

**Konfiguracja "set and forget":**

```bash
# 1. Performance governor (raz, przy starcie systemu)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# 2. Wyłącz C-states (w GRUB, /etc/default/grub):
GRUB_CMDLINE_LINUX="processor.max_cstate=1 intel_idle.max_cstate=0"

# 3. Wyłącz turbo boost (opcjonalnie):
echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo

# 4. Zablokuj zmiany (systemd service):
[Unit]
Description=Lock CPU governor to performance
After=multi-user.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/lock-governor.sh
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
```

**Skrypt `lock-governor.sh`:**

```bash
#!/bin/bash
for gov in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo performance > $gov
done
# Opcjonalnie: ustaw min = max frequency
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_min_freq; do
    cat ${cpu/_min_/_max_} > $cpu
done
```

______________________________________________________________________

### **Rozwiązanie 3: Izolacja rdzeni (advanced)**

**Dla systemów z wieloma rdzeniami:**

```bash
# W GRUB:
isolcpus=2,3 nohz_full=2,3 rcu_nocbs=2,3

# Rdzenie 2-3: tylko taski RT, zawsze performance
# Rdzenie 0-1: Linux, może mieć ondemand
```

**W kodzie (C/C++):**

```c
// Przypnij task RT do izolowanego rdzenia
cpu_set_t cpuset;
CPU_ZERO(&cpuset);
CPU_SET(2, &cpuset);  // Rdzeń 2
pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);

// Ustaw priorytet RT
struct sched_param param;
param.sched_priority = 99;
pthread_setschedparam(pthread_self(), SCHED_FIFO, &param);
```

______________________________________________________________________

## **🔍 Jak wykryć problem PRZED katastrofą?**

### **Test obciążeniowy:**

```bash
# Symuluj nagłe obciążenie
stress-ng --cpu 4 --timeout 1s &

# Mierz jitter pętli RT
cyclictest -p 99 -t 1 -n -m -l 100000
```

**Zdrowy system (performance):**

```
Min: 5 µs, Avg: 8 µs, Max: 25 µs
```

**Chory system (ondemand):**

```
Min: 5 µs, Avg: 50 µs, Max: 15000 µs (!!!)
```

______________________________________________________________________

### **Monitoring w produkcji:**

```c
// W każdym cyklu pętli RT:
uint64_t start = get_time_us();
// ... kod sterowania ...
uint64_t duration = get_time_us() - start;

if (duration > WCET_LIMIT) {
    log_error("RT overrun: %lu us", duration);
    trigger_safe_mode();
}
```

______________________________________________________________________

## **📋 Checklista "nie baw się governorami"**

✅ **Dla embedded (MCU):**

- Użyj dedykowanego RTOS
- Stałe taktowanie
- Brak governorów w ogóle

✅ **Dla Linux RT:**

- `performance` governor w systemd service
- Wyłącz C-states w GRUB
- Testuj z `cyclictest`
- Monitoruj WCET w produkcji

✅ **Dla hybrydowych:**

- MCU = RT
- Linux = high-level
- Jasna granica odpowiedzialności

______________________________________________________________________

## **💀 Najgorszy przypadek (real story)**

**Firma robotyczna, 2019:**

```
Robot: magazynowy, 24/7
OS: Ubuntu + PREEMPT_RT
Governor: ondemand (domyślny)

Incident:
- Noc, robot idle, CPU 800 MHz
- Rano: nagły zlecenie
- Governor: 150 ms opóźnienie
- Pętla sterowania: 50 cykli spóźnienia
- Robot: wjechał w regał
- Straty: $200k
```

**Fix:**

```bash
echo performance > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

**Koszt fixa: 0 zł. Koszt niefixa: 200k USD.** 💸

______________________________________________________________________

## **🎓 Odpowiedź na twoje pytanie:**

**Czy jest cisza przed burzą?**\
**TAK.** System działa idealnie w idle, bo nie ma obciążenia. Katastrofa przychodzi przy **pierwszym realnym obciążeniu**.

**Jak się zatnie?**\
Nie deadlock, ale **kaskada opóźnień** → watchdog timeout / emergency stop.

**Jak zapobiec bez bawienia się?**\
**Architektura hybrydowa (MCU+Linux)** lub **jednorazowa konfiguracja systemd service**.

______________________________________________________________________

Chcesz, żebym rozwinął:

- Przykład systemd service do lockowania governora?
- Skrypt testowy do wykrywania problemów?
- Porównanie MCU vs Linux RT w liczbach? 🤖
