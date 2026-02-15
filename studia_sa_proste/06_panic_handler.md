# Lekcja 6: Obsługa Panic w no_std

## Problem

W embedded nie mamy standardowej biblioteki (`std`). Co robić gdy nastąpi panic? Nie możemy użyć domyślnego `println`! [1]

## Rozwiązanie

Zdefiniuj własny `panic_handler`.

```rust
#![no_std]
#![panic_handler]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Opcja 1: Zatrzymaj system
    loop {
        cortex_m::asm::wfi(); // Wait For Interrupt
    }
}
```

## Rozszerzona wersja z debug UART

```rust
#![no_std]
#![panic_handler]

use core::fmt::Write;

struct Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Wyślij przez UART
        for byte in s.bytes() {
            //uart::write(byte);
        }
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Wyślij info o panic przez UART
    writeln!(Uart, "PANIC: {}", info).ok();
    
    loop {
        cortex_m::asm::wfi();
    }
}
```

## Opcje panic handler

| Metoda | Rozmiar | Zalety | Wady |
|--------|---------|--------|------|
| `panic_halt` | Mały | Prosty | Brak info |
| `panic_abort` | Najmniejszy | Minimalny kod | Brak recovery |
| `panic_never` | Mały | Można recover | Złożone |
| Custom | Zmienny | Pełna kontrola | Więcej kodu |

## Konfiguracja w Cargo.toml

```toml
[profile.release]
panic = "abort"  # Najmniejszy rozmiar

# Lub dla embedded:
[profile.dev]
panic = "abort"
```

## Typowe błędy

| Błąd | Skutek |
|-------|--------|
| Brak panic_handler | Kompilacja się nie powiedzie |
| Zbyt duży handler | Przepełnienie flash |
| Blokowanie w handler | System się zawiesza |

## Porównanie z C

```c
// C - assert lub nic
void HardFault_Handler() {
    while(1); // Zawieszenie
}

// Rust - ładniejsze info
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Można wysłać location, message
}
```

## Godny następca

`defmt` - formatted printing dla embedded przez debug probe (SWD/JTAG). [3]

## Źródła

[1] https://ubuntu.com/blog/real-time-linux-vs-rtos-2
[3] https://www.perplexity.ai/search/284cb83b-2058-4263-b48f-d97fe167e9e9
