# Laboratorium 9: Case Study - Systemy krytyczne

**Czas:** 2 godziny
**Punkty:** 20 pkt

---

## Cel ćwiczenia

1. Analiza rzeczywistych systemów RT
2. Zastosowanie poznanych technik
3. Identyfikacja problemów w istniejącym kodzie
4. Projektowanie systemu z requirements

---

## Case Study 1: Airbag Controller (40 min)

### Opis systemu

```
System sterowania poduszką powietrzną:

Wymagania:
- Czas od wykrycia zderzenia do napełnienia: < 30ms
- Deceleration sensor: próg 2.5g
- Redundant sensors (2 z 3 muszą się zgodzić)
- Self-test przy włączeniu zapłonu
- Fail-safe: poduszka nie może się napełnić przypadkowo

Constraints:
- MCU: 48 MHz, 64KB RAM
- Determinizm: max latency 5ms
- No OS services w critical path
```

### Architektura

```c
// Task priorytety
#define PRIO_SENSOR         5   // Highest
#define PRIO_DECISION       4
#define PRIO_SELFTEST       3
#define PRIO_DIAGNOSTIC     2
#define PRIO_IDLE           0

// Timing
#define SENSOR_PERIOD_MS    1    // 1kHz sampling
#define DECISION_DEADLINE   5   // 5ms max

// Shared data
typedef struct {
    int16_t acceleration[3];  // X, Y, Z
    uint16_t sensor_id;
    TickType_t timestamp;
    bool valid;
} SensorReading;

SensorReading sensor_data[3];  // 3 redundant sensors
SemaphoreHandle_t sensor_mutex;

// ISR - Sensor data ready (highest priority)
void ACC_IRQHandler(void) {
    BaseType_t higher_woken = pdFALSE;

    // Read sensor (fast!)
    int16_t acc[3];
    acc[0] = ACC->X;
    acc[1] = ACC->Y;
    acc[2] = ACC->Z;

    // Determine which sensor
    uint16_t id = ACC->ID;

    // Update data (minimal critical section)
    taskENTER_CRITICAL_FROM_ISR();
    sensor_data[id].acceleration[0] = acc[0];
    sensor_data[id].acceleration[1] = acc[1];
    sensor_data[id].acceleration[2] = acc[2];
    sensor_data[id].timestamp = xTaskGetTickCountFromISR();
    sensor_data[id].valid = true;
    taskEXIT_CRITICAL_FROM_ISR();

    // Signal decision task
    xTaskNotifyFromISR(decision_task_handle, 0, eSetBits, &higher_woken);
    portYIELD_FROM_ISR(higher_woken);
}

// Decision task - Critical!
void vDecisionTask(void *pvParameters) {
    while (1) {
        // Wait for sensor update
        ulTaskNotifyTake(pdTRUE, portMAX_DELAY);

        TickType_t start = xTaskGetTickCount();

        // Read all sensors
        int16_t acc[3][3];
        bool valid[3];

        taskENTER_CRITICAL();
        for (int i = 0; i < 3; i++) {
            acc[i][0] = sensor_data[i].acceleration[0];
            acc[i][1] = sensor_data[i].acceleration[1];
            acc[i][2] = sensor_data[i].acceleration[2];
            valid[i] = sensor_data[i].valid;
        }
        taskEXIT_CRITICAL();

        // Calculate magnitude
        float magnitude[3];
        for (int i = 0; i < 3; i++) {
            if (valid[i]) {
                magnitude[i] = sqrt(acc[i][0]*acc[i][0] +
                                   acc[i][1]*acc[i][1] +
                                   acc[i][2]*acc[i][2]);
            }
        }

        // 2-of-3 voting
        int crash_detected = 0;
        for (int i = 0; i < 3; i++) {
            if (valid[i] && magnitude[i] > 2.5 * 9.81) {  // 2.5g
                crash_detected++;
            }
        }

        if (crash_detected >= 2) {
            // CRITICAL: Fire airbag!
            fire_airbag();
        }

        // Check deadline
        TickType_t elapsed = xTaskGetTickCount() - start;
        if (elapsed > pdMS_TO_TICKS(DECISION_DEADLINE)) {
            // Deadline miss - log but continue
            log_deadline_miss(elapsed);
        }
    }
}

// Self-test task (runs at startup)
void vSelfTestTask(void *pvParameters) {
    // Run diagnostics
    bool all_ok = true;

    all_ok &= test_sensors();
    all_ok &= test_squib();
    all_ok &= test_memory();

    if (!all_ok) {
        // Indicate fault
        set_fault_indicator();
    }

    // Self-delete after test
    vTaskDelete(NULL);
}
```

### Analiza problemów

**TODO:** Znajdź potencjalne problemy w kodzie:

1. Czy ISR jest krótki?
2. Czy critical sections są minimalne?
3. Czy decision task ma deterministyczny czas?
4. Co jeśli sensory się nie zgadzają?

### Ulepszenia

```c
// ULEPSZENIE 1: Pre-compute threshold
#define THRESHOLD_2_5G (int32_t)(2.5 * 9.81 * SENSOR_SCALE)

// Unikaj floating point w decision path!
int32_t magnitude_squared[3];
for (int i = 0; i < 3; i++) {
    if (valid[i]) {
        magnitude_squared[i] = acc[i][0]*acc[i][0] +
                              acc[i][1]*acc[i][1] +
                              acc[i][2]*acc[i][2];
        // Compare with squared threshold
        if (magnitude_squared[i] > THRESHOLD_SQUARED) {
            crash_detected++;
        }
    }
}
```

---

## Case Study 2: Engine Control Unit (40 min)

### Opis

```
ECU sterujący wtryskiem paliwa i zapłonem:

Wymagania:
- RPM range: 600 - 9000 (10-150 Hz)
- Fuel injection timing: precision 0.1° crank angle
- Ignition timing: precision 0.5°
- Multiple injectors (4-8 cylinders)
- Lambda feedback (closed loop)

Tasks:
1. Crank position ISR (highest)
2. Fuel calculation (periodic)
3. Ignition timing (periodic)
4. Lambda control (slower)
5. Diagnostics (background)
```

### Implementacja

```c
// Configuration
#define CYLINDER_COUNT 4
#define MAX_RPM 9000

// Crank position - tooth wheel
#define TEETH_PER_REV 60
#define MISSING_TEETH 2
#define EFFECTIVE_TEETH (TEETH_PER_REV - MISSING_TEETH)

// Timing data
typedef struct {
    uint16_t injection_duration_us;
    uint16_t injection_start_angle;
    uint16_t ignition_advance_deg;
    uint16_t dwell_time_us;
} CylinderTiming;

CylinderTiming cylinder_timing[CYLINDER_COUNT];
volatile uint16_t current_rpm;
volatile uint16_t current_angle;  // 0-359 degrees
SemaphoreHandle_t timing_mutex;

// ISR - Crank tooth detected
void CRANK_IRQHandler(void) {
    static uint32_t last_tooth_time = 0;
    static int tooth_count = 0;
    static bool sync = false;

    uint32_t now = get_cycle_count();

    // Detect sync (missing teeth)
    if (last_tooth_time > 0) {
        uint32_t period = now - last_tooth_time;

        // Missing tooth detection (gap = 2x normal)
        if (period > expected_tooth_period * 1.8) {
            sync = true;
            tooth_count = 0;
            current_angle = 0;  // TDC reference
        }
    }

    last_tooth_time = now;

    if (sync) {
        // Update angle (6° per tooth for 60-2 wheel)
        current_angle += 6;
        if (current_angle >= 360) {
            current_angle -= 360;
        }

        tooth_count++;
        if (tooth_count >= EFFECTIVE_TEETH) {
            // Full revolution - calculate RPM
            uint32_t rev_time = now - revolution_start;
            current_rpm = (60 * SystemCoreClock) / rev_time;
            revolution_start = now;
            tooth_count = 0;
        }

        // Check for injection events
        for (int cyl = 0; cyl < CYLINDER_COUNT; cyl++) {
            if (current_angle == cylinder_timing[cyl].injection_start_angle) {
                start_injection(cyl, cylinder_timing[cyl].injection_duration_us);
            }
        }

        // Check for ignition events (before TDC)
        for (int cyl = 0; cyl < CYLINDER_COUNT; cyl++) {
            int16_t fire_angle = 360 - cylinder_timing[cyl].ignition_advance_deg;
            if (current_angle == fire_angle) {
                start_coil_dwell(cyl, cylinder_timing[cyl].dwell_time_us);
            }
        }
    }
}

// Task - Fuel calculation
void vFuelCalcTask(void *pvParameters) {
    TickType_t last_wake = xTaskGetTickCount();

    while (1) {
        // Read sensors
        uint16_t map = read_map_sensor();
        uint16_t rpm = current_rpm;
        uint16_t tps = read_tps_sensor();
        uint16_t clt = read_clt_sensor();

        // Calculate fuel (lookup table)
        uint16_t base_pulse = fuel_table[map/10][rpm/100];

        // Apply corrections
        float correction = 1.0;
        correction *= temp_correction(clt);
        correction *= accel_enrichment(tps, last_tps);
        last_tps = tps;

        uint16_t final_pulse = base_pulse * correction;

        // Update timing for all cylinders
        xSemaphoreTake(timing_mutex, portMAX_DELAY);
        for (int i = 0; i < CYLINDER_COUNT; i++) {
            cylinder_timing[i].injection_duration_us = final_pulse;
            cylinder_timing[i].injection_start_angle = injection_angle_table[i];
        }
        xSemaphoreGive(timing_mutex);

        // Periodic: 10ms
        vTaskDelayUntil(&last_wake, pdMS_TO_TICKS(10));
    }
}

// Task - Lambda control (slower)
void vLambdaTask(void *pvParameters) {
    TickType_t last_wake = xTaskGetTickCount();
    float lambda_integral = 0;

    while (1) {
        // Read lambda sensor
        float lambda = read_lambda_sensor();

        // Target: lambda = 1.0 (stoichiometric)
        float error = 1.0 - lambda;

        // PI controller
        lambda_integral += error * 0.01;
        float trim = error * 0.5 + lambda_integral;

        // Clamp trim
        if (trim > 0.2) trim = 0.2;
        if (trim < -0.2) trim = -0.2;

        // Apply fuel trim
        // ...

        // Periodic: 100ms
        vTaskDelayUntil(&last_wake, pdMS_TO_TICKS(100));
    }
}
```

### Analiza

**TODO:** Odpowiedz na pytania:

1. Dlaczego fuel calculation jest w tasku, nie ISR?
2. Jaki jest najgorszy czas reakcji na zmianę obrotów?
3. Co się stanie przy missing teeth sync error?
4. Jak zagwarantować timing injection?

---

## Case Study 3: Flight Control System (40 min)

### Opis

```
System sterowania lotem drona:

Sensors:
- IMU (accelerometer + gyro) - 1kHz
- Magnetometer - 100Hz
- GPS - 5Hz
- Barometer - 50Hz

Actuators:
- 4 motors (PWM, 400Hz)
- Servos (optional)

Control loops:
- Inner loop (attitude): 500Hz
- Outer loop (position): 100Hz
- Navigation: 10Hz
```

### Architektura

```c
// Task priorytety
#define PRIO_IMU            5   // Highest
#define PRIO_ATTITUDE       4
#define PRIO_POSITION       3
#define PRIO_MOTOR          2
#define PRIO_TELEMETRY      1

// Shared data structures
typedef struct {
    float accel[3];      // m/s²
    float gyro[3];       // rad/s
    float mag[3];        // normalized
    TickType_t timestamp;
    bool fresh;
} IMUData;

typedef struct {
    float roll, pitch, yaw;
    float roll_rate, pitch_rate, yaw_rate;
} AttitudeState;

typedef struct {
    float lat, lon, alt;
    float vx, vy, vz;
} PositionState;

// Queues for data flow
QueueHandle_t imu_queue;
QueueHandle_t attitude_queue;
QueueHandle_t motor_queue;

// IMU Task - Highest rate
void vIMUTask(void *pvParameters) {
    TickType_t last_wake = xTaskGetTickCount();
    IMUData imu;

    while (1) {
        // Read sensors
        imu.accel[0] = IMU->ACC_X;
        imu.accel[1] = IMU->ACC_Y;
        imu.accel[2] = IMU->ACC_Z;
        imu.gyro[0] = IMU->GYR_X;
        imu.gyro[1] = IMU->GYR_Y;
        imu.gyro[2] = IMU->GYR_Z;
        imu.timestamp = xTaskGetTickCount();
        imu.fresh = true;

        // Send to attitude estimation
        xQueueSend(imu_queue, &imu, 0);

        // 1kHz
        vTaskDelayUntil(&last_wake, pdMS_TO_TICKS(1));
    }
}

// Attitude Task - Complementary filter
void vAttitudeTask(void *pvParameters) {
    TickType_t last_wake = xTaskGetTickCount();
    IMUData imu;
    AttitudeState attitude;

    // Filter state
    float roll = 0, pitch = 0;
    float roll_rate = 0, pitch_rate = 0;

    while (1) {
        // Get IMU data (blocking)
        if (xQueueReceive(imu_queue, &imu, pdMS_TO_TICKS(5)) == pdTRUE) {
            // Complementary filter
            float dt = 0.002;  // 500Hz

            // Gyro integration
            roll_rate = imu.gyro[0];
            pitch_rate = imu.gyro[1];

            roll += roll_rate * dt;
            pitch += pitch_rate * dt;

            // Accelerometer correction
            float accel_roll = atan2(imu.accel[1], imu.accel[2]);
            float accel_pitch = atan2(-imu.accel[0],
                                      sqrt(imu.accel[1]*imu.accel[1] +
                                           imu.accel[2]*imu.accel[2]));

            // Blend (high pass gyro, low pass accel)
            roll = 0.98 * roll + 0.02 * accel_roll;
            pitch = 0.98 * pitch + 0.02 * accel_pitch;

            // Store
            attitude.roll = roll;
            attitude.pitch = pitch;
            attitude.yaw = 0;  // From magnetometer
            attitude.roll_rate = roll_rate;
            attitude.pitch_rate = pitch_rate;

            // Send to controller
            xQueueSend(attitude_queue, &attitude, 0);
        }

        // 500Hz
        vTaskDelayUntil(&last_wake, pdMS_TO_TICKS(2));
    }
}

// Motor Control Task - PID control
void vMotorTask(void *pvParameters) {
    TickType_t last_wake = xTaskGetTickCount();
    AttitudeState attitude;

    // PID gains
    float kp_roll = 1.5, ki_roll = 0.1, kd_roll = 0.05;
    float integral_roll = 0, last_error_roll = 0;

    // Target attitude (from RC)
    float target_roll = 0, target_pitch = 0;

    while (1) {
        // Get current attitude
        if (xQueueReceive(attitude_queue, &attitude, pdMS_TO_TICKS(10)) == pdTRUE) {
            // PID for roll
            float error_roll = target_roll - attitude.roll;
            integral_roll += error_roll * 0.002;
            float derivative_roll = (error_roll - last_error_roll) / 0.002;
            last_error_roll = error_roll;

            float output_roll = kp_roll * error_roll +
                               ki_roll * integral_roll +
                               kd_roll * derivative_roll;

            // Mix to motors (quadcopter X config)
            float motor[4];
            motor[0] = base_throttle + output_roll;  // Front-left
            motor[1] = base_throttle - output_roll;  // Front-right
            motor[2] = base_throttle - output_roll;  // Back-left
            motor[3] = base_throttle + output_roll;  // Back-right

            // Clamp and output
            for (int i = 0; i < 4; i++) {
                if (motor[i] > MAX_THROTTLE) motor[i] = MAX_THROTTLE;
                if (motor[i] < MIN_THROTTLE) motor[i] = MIN_THROTTLE;
                set_motor_pwm(i, motor[i]);
            }
        }

        // 400Hz
        vTaskDelayUntil(&last_wake, pdMS_TO_TICKS(2.5));
    }
}
```

### Zadanie

**TODO:** Dokończ implementację:

1. Dodaj obsługę magnetometru dla yaw
2. Zaimplementuj position control (outer loop)
3. Dodaj GPS handling
4. Zaimplementuj failsafe (loss of signal)

---

## Punkty kontrolne

| Punkt | Opis | Punkty |
|-------|------|--------|
| 1 | Airbag analysis | 5 |
| 2 | ECU implementation | 5 |
| 3 | Flight controller | 5 |
| 4 | Ulepszenia zaproponowane | 5 |

---

## Sprawozdanie

1. Diagram architektury każdego systemu
2. Analiza timing requirements
3. Potencjalne problemy i rozwiązania
4. Wnioski o projektowaniu systemów krytycznych

---

## Pytania kontrolne

1. Dlaczego airbag używa voting 2-of-3?
2. Jak zapewnić deterministyczny timing w ECU?
3. Co to jest inner loop vs outer loop w flight control?
4. Jak obsłużyć sensor failure w systemie RT?
5. Jakie są różnice między tymi systemami w requirements?