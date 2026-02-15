# Wykład: Integracja Rust + FreeRTOS (FFI) - Dyskusja ze Studentami

## 📋 Agenda

1. Wprowadzenie do problemu migracji
1. Czy można połączyć Rust i C?
1. Deklarowanie C API w Rust
1. Wrappery RAII - bezpieczeństwo
1. ❓ Pytania studentów
1. Demonstracja kodu
1. Typowe błędy
1. ❓ Dyskusja
1. Podsumowanie

______________________________________________________________________

## 1. Wprowadzenie do Problem Migracji

**Prowadzący:** Wyobraźmy sobie sytuację: pracujemy w firmie która od 10 lat używa FreeRTOS. Mamy setki tysięcy linii kodu w C. Chcemy zacząć używać Rust, ale nie możemy przepisać całego systemu od zera!

Jak możemy to zrobić?

**Student 1:** Możemy przepisać wszystko?

**Prowadzący:** teoretycznie tak, ale:

- To setki miesięcy pracy
- Ryzyko błędów przy przepisywaniu
- Wszystkie testy do zrobienia od nowa
- Firma potrzebuje dostarczać produkt

**Student 2:** A może piszmy nowy kod w Rust?

**Prowadzący:** Dokładnie! Stopniowa migracja to klucz!

______________________________________________________________________

## 2. Czy Można Połączyć Rust i C?

**Prowadzący:** Rust i C mają wspólny ABI - to znaczy że mogą się "rozumieć"!

```
┌─────────────┐     ┌─────────────┐
│   Rust      │     │     C       │
│  (nowy)    │ ←→  │  (stary)    │
│   kod      │ FFI │   kod       │
└─────────────┘     └─────────────┘
         │                 │
         └────────┬────────┘
                  │
            Wspólny ABI
```

**Student 3:** Co to jest FFI?

**Prowadzący:** FFI = Foreign Function Interface. To sposób wywoływania funkcji z jednego języka w drugim.

______________________________________________________________________

## 3. Deklarowanie C API w Rust

**Prowadzący:** Zobaczmy jak wygląda deklaracja funkcji C w Rust:

```rust
#![no_std]

mod freertos {
    use core::ffi::c_void;
    use core::ffi::c_int;

    #[link(name = "freertos")]
    extern "C" {
        // Tworzenie mutexa
        pub fn xSemaphoreCreateMutex() -> *mut c_void;
        
        // Blokada mutexa
        pub fn xSemaphoreTake(sem: *mut c_void, wait: u32) -> c_int;
        
        // Odblokowanie mutexa
        pub fn xSemaphoreGive(sem: *mut c_void) -> c_int;
        
        // Kolejka
        pub fn xQueueCreate(len: u32, size: u32) -> *mut c_void;
        pub fn xQueueSend(q: *mut c_void, data: *const c_void, wait: u32) -> c_int;
        pub fn xQueueReceive(q: *mut c_void, data: *mut c_void, wait: u32) -> c_int;
    }
}
```

### Co tu się dzieje?

| Element | Znaczenie |
|---------|-----------|
| `#[link(name = "freertos")]` | Powiąż z biblioteką C |
| `extern "C"` | Używaj C ABI |
| `*mut c_void` | Surowy wskaźnik C |
| `c_int` | Typ całkowity C |

______________________________________________________________________

## 4. Wrappery RAII - Bezpieczeństwo

**Prowadzący:** Problem z C API: łatwo zapomnieć o odblokowaniu!

```c
// C - łatwo o błąd!
pthread_mutex_lock(&mutex);
if (error) {
    return; // BŁĄD! Mutex nadal zablokowany!
}
pthread_mutex_unlock(&mutex);
```

**Student 4:** A jak to wygląda w Rust?

**Prowadzący:** Rust daje nam RAII - Resource Acquisition Is Initialization:

```rust
pub struct Mutex {
    handle: *mut core::ffi::c_void,
}

impl Mutex {
    pub fn new() -> Option<Self> {
        let handle = unsafe { freertos::xSemaphoreCreateMutex() };
        if handle.is_null() {
            None
        } else {
            Some(Self { handle })
        }
    }
    
    pub fn lock(&self) {
        unsafe { freertos::xSemaphoreTake(self.handle, u32::MAX) };
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe { freertos::xSemaphoreGive(self.handle) };
    }
}
```

### Jak to działa?

```rust
fn main() {
    let mutex = Mutex::new().unwrap();
    
    // RAII - automatyczne odblokowanie!
    {
        mutex.lock();
        // operacje na danych
    } // <- drop() wywołany automatycznie!
    
    // Lub ze scope:
    {
        let _guard = mutex.lock();
        // operacje
    } // <- mutex odblokowany!
}
```

______________________________________________________________________

## 5. ❓ Pytania Studentów

### Student 5: Dlaczego `Option<Self>`?

**Prowadzący:** Bo alokacja może się nie powieść:

```rust
pub fn new() -> Option<Self> {
    let handle = unsafe { freertos::xSemaphoreCreateMutex() };
    if handle.is_null() {
        None  // Brak pamięci!
    } else {
        Some(Self { handle })
    }
}
```

W C często to ignorujemy - w Rust musimy obsłużyć!

### Student 6: Co to jest `*mut c_void`?

**Prowadzący:** To surowy wskaźnik C (void pointer):

- `*mut` = mutowalny
- `c_void` = "nieokreślony typ"
- W Rust to `unsafe` - sami musimy zagwarantować poprawność

### Student 7: Dlaczego `unsafe`?

**Prowadzący:** Bo:

1. Wywołujemy kod C - nie wiemy co robi
1. Surowe wskaźniki - możemy pomylić typy
1. Brak borrow checkera dla C API

______________________________________________________________________

## 6. Demonstracja Kodu

**Prowadzący:** Zobaczmy pełny przykład:

```rust
#![no_std]

mod freertos {
    use core::ffi::c_void;
    use core::ffi::c_int;

    #[link(name = "freertos")]
    extern "C" {
        pub fn xSemaphoreCreateMutex() -> *mut c_void;
        pub fn xSemaphoreTake(sem: *mut c_void, wait: u32) -> c_int;
        pub fn xSemaphoreGive(sem: *mut c_void) -> c_int;
        
        pub fn xQueueCreate(len: u32, size: u32) -> *mut c_void;
        pub fn xQueueSend(q: *mut c_void, data: *const c_void, wait: u32) -> c_int;
        pub fn xQueueReceive(q: *mut c_void, data: *mut c_void, wait: u32) -> c_int;
    }
}

// ====== BEZPIECZNE WRAPPERY ======

pub struct Mutex {
    handle: *mut core::ffi::c_void,
}

impl Mutex {
    pub fn new() -> Option<Self> {
        let handle = unsafe { freertos::xSemaphoreCreateMutex() };
        if handle.is_null() { None } else { Some(Self { handle }) }
    }
    
    pub fn lock(&self) {
        unsafe { freertos::xSemaphoreTake(self.handle, u32::MAX) };
    }
    
    pub fn try_lock(&self, ms: u32) -> bool {
        unsafe { freertos::xSemaphoreTake(self.handle, ms) } == 0
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe { freertos::xSemaphoreGive(self.handle) };
    }
}

// Kolejka
pub struct Queue<T> {
    handle: *mut core::ffi::c_void,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Queue<T> {
    pub fn new(len: u32) -> Option<Self> {
        let handle = unsafe { 
            freertos::xQueueCreate(len, core::mem::size_of::<T>() as u32) 
        };
        if handle.is_null() { None } 
        else { Some(Self { handle, _phantom: core::marker::PhantomData }) }
    }
    
    pub fn send(&self, value: &T, timeout: u32) -> bool {
        unsafe { 
            freertos::xQueueSend(
                self.handle, 
                value as *const T as *const core::ffi::c_void, 
                timeout
            ) == 0 
        }
    }
    
    pub fn receive(&self, timeout: u32) -> Option<T> {
        let mut value: T = unsafe { core::mem::zeroed() };
        let ok = unsafe { 
            freertos::xQueueReceive(
                self.handle,
                &mut value as *mut T as *mut core::ffi::c_void,
                timeout
            ) == 0 
        };
        if ok { Some(value) } else { None }
    }
}
```

### Użycie:

```rust
static SHARED_DATA: Mutex = Mutex::new();

fn rust_task() {
    // Mutex
    let m = Mutex::new().unwrap();
    m.lock();
    // ... operacje ...
    // automatyczne unlock przy drop!
    
    // Kolejka
    let q: Queue<u32> = Queue::new(10).unwrap();
    q.send(&42, 1000);
    if let Some(val) = q.receive(1000) {
        println!("Odebrano: {}", val);
    }
}
```

______________________________________________________________________

## 7. Typowe Błędy

### Błąd 1: Lock bez unlock

```rust
// ŹLE - mutex nie odblokowany przy early return
fn bad_function() {
    let m = Mutex::new().unwrap();
    m.lock();
    
    if something_bad() {
        return; // BŁĄD! mutex ciągle zablokowany!
    }
    
    m.unlock(); // nigdy nie wywołane
}

// DOBRZE - RAII automatycznie odblokuje
fn good_function() {
    let m = Mutex::new().unwrap();
    
    {
        let _guard = m.lock();
        if something_bad() {
            return; // OK! _guard drop automatycznie!
        }
    }
}
```

### Błąd 2: Null handle

```rust
// ŹLE - ignorujemy null
fn bad_use() {
    let m = freertos::xSemaphoreCreateMutex();
    // m może być null!
    freertos::xSemaphoreTake(m, u32::MAX); // CRASH!
}

// DOBRZE - sprawdzamy
fn good_use() {
    let m = freertos::xSemaphoreCreateMutex();
    if m.is_null() {
        panic!("Brak pamięci!");
    }
    freertos::xSemaphoreTake(m, u32::MAX); // OK
}
```

### Błąd 3: Zły rozmiar kolejki

```rust
// ŹLE - rozmiar nie zgadza się z C
struct BigData { a: u64, b: u64, c: u64 } // 24 bajty

fn bad_queue() {
    let q = xQueueCreate(10, 8); // BŁĄD! powinno być 24!
}

// DOBRZE - automatyczny rozmiar
fn good_queue() {
    let q = xQueueCreate(10, core::mem::size_of::<BigData>() as u32);
}
```

______________________________________________________________________

## 8. ❓ Dyskusja

### Student 8: Czy to jest bezpieczne?

**Prowadzący:** Częściowo! Spójrzmy:

| Aspekt | Bezpieczeństwo |
|--------|----------------|
| RAII unlock | ✅ automatyczne |
| Null check | ✅ Option |
| Type safety | ⚠️ surowe wskaźniki |
| Thread safety | ⚠️ zależy od C |

### Student 9: Czy można to certyfikować?

**Prowadzący:** Tak, ale wymaga dodatkowej pracy:

- Dokumentacja że wszystkie unsafe są poprawne
- Testy na granicy
- Code review wszystkich extern "C"
- Można użyć `cargo-geiger` do śledzenia unsafe

### Student 10: A co z nowym kodem w Rust?

**Prowadzący:** Strategia:

1. Nowe moduły w Rust
1. Stare moduły w C
1. FFI tylko na granicy
1. Stopniowo przepisuj C → Rust

______________________________________________________________________

## 9. Podsumowanie

**Prowadzący:** Podsumujmy:

| Aspekt | C | Rust + FreeRTOS |
|--------|---|------------------|
| Stopniowa migracja | ❌ | ✅ |
| Auto unlock | ❌ Ręcznie | ✅ RAII |
| Error handling | ❌ Często brak | ✅ Option/Result |
| Type safety | ❌ Słaby | ✅ Silny |
| Learning curve | - | ⚠️ Nowy język |

### Co zapamiętać?

1. ✅ FFI łączy Rust i C
1. ✅ RAII = bezpieczne wrappery
1. ✅ Zawsze sprawdzaj null
1. ✅ `core::mem::size_of` dla kolejek
1. ✅ Stopniowa migracja zamiast przepisywania

### Następna lekcja

**Prowadzący:** Omówmy też alternatywy:

- `cortex-m-rtic` - natywny RTOS w Rust
- `embassy` - async RTOS
- Kiedy warto porzucić FreeRTOS?

______________________________________________________________________

## 📚 Materiały dodatkowe

- Rustonomicon - Unsafe Rust
- "Programming Rust" - Jim Blandy
- Dokumentacja FreeRTOS C API
- `cortex-m-rtic` - przykłady integracji
