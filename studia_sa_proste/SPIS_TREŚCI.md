# Spis Treści - Studia Są Proste

## Podstawy

1. **[01_kolejka_spsc.md](01_kolejka_spsc.md)** - Lock-free kolejka SPSC

   - Problem: blokady w RTOS
   - Rozwiązanie: ring buffer z atomowymi
   - Błędy: race conditions, overflow

1. **[02_scheduler_priorytety.md](02_scheduler_priorytety.md)** - Scheduler z priorytetami

   - Problem: priority inversion
   - Rozwiązanie: BinaryHeap
   - Błędy: brak inheritance, przepełnienie

1. **[03_atomowe_operacje.md](03_atomowe_operacje.md)** - Atomowe operacje

   - Problem: współdzielone dane
   - Rozwiązanie: Atomic\* typy
   - Błędy: zły ordering

1. **[04_bariery_pamieci.md](04_bariery_pamieci.md)** - Bariery pamięci

   - Problem: memory reorder
   - Rozwiązanie: Acquire/Release
   - Błędy: brak barier

## Zaawansowane

5. **[05_async_await_embedded.md](05_async_await_embedded.md)** - Async/await w embedded

   - Problem: za dużo wątków
   - Rozwiązanie: futures
   - Błędy: brak .await

1. **[06_panic_handler.md](06_panic_handler.md)** - Obsługa panic w no_std

   - Problem: brak std
   - Rozwiązanie: własny handler
   - Błędy: zbyt duży handler

1. **[07_freertos_ffi.md](07_freertos_ffi.md)** - Integracja Rust + FreeRTOS

   - Problem: istniejący kod C
   - Rozwiązanie: RAII wrappery
   - Błędy: lock bez unlock

## Dodatkowe

8. **[08_16_dodatkowe_lekcje.md](08_16_dodatkowe_lekcje.md)** - Kolejne lekcje
   - Deadlock - jak unikać
   - Priority Inversion
   - Sekcje krytyczne
   - Obsługa ISR
   - Alokacja pamięci
   - Zero-cost abstractions
   - Type-level programming
   - Bezpieczeństwo pamięci
   - Embassy (async RTOS)

______________________________________________________________________

## Szybki Start

```bash
# Kompilacja przykładów
cargo build --release

# Uruchomienie testów
cargo test

# Sprawdzenie clippy
cargo clippy
```

## Wymagania

- Rust 1.70+
- cargo
- (dla embedded) rustup + target

## Źródła

- [1] https://ubuntu.com/blog/real-time-linux-vs-rtos-2
- [2] https://www.perplexity.ai/search/0a7b8300-9680-4ac3-94e9-4f153c27d9f3
- [3] https://www.perplexity.ai/search/284cb83b-2058-4263-b48f-d97fe167e9e9
- [4] https://www.embedded.com/the-rtos-renaissance-closing-the-os-gap-with-linux-in-iot/
- [5] https://www.perplexity.ai/search/e67efac0-ea48-43f2-b9b6-afc3c2f5e217
- [6] https://www.reddit.com/r/linux/comments/1fl88vk/linux_is_now_a_rtos_preempt_rt_real-time_kernel/
- [7] https://en.wikipedia.org/wiki/PREEMPT_RT
