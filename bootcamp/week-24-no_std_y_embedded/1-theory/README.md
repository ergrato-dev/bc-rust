# Semana 24 — Teoría: `no_std` e Embedded

> ⚠️ **TODO**: Completar con teoría completa (~180 líneas).

## Secciones pendientes

1. El ecosistema: `core`, `alloc`, `std` — jerarquía y dependencias
2. `#![no_std]` — qué se pierde y qué queda disponible
3. `core::fmt`, `core::mem`, `core::ptr` — primitivos siempre disponibles
4. Panic handlers: `#[panic_handler]` y `panic = "abort"`
5. Heapless: colecciones con capacidad en compile-time
6. `#[global_allocator]` — usar alloc sin std
7. HAL (Hardware Abstraction Layer) — arquitectura en Rust embedded
8. `embedded-hal` traits: GPIO, UART, SPI, I2C
9. RTIC framework — Real-Time Interrupt-driven Concurrency
10. Depuración sin std: probe-rs, defmt, RTT
