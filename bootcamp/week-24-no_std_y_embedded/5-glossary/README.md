# Glosario — Semana 24: `no_std` e Embedded

| Término | Definición |
|---------|-----------|
| **`#![no_std]`** | Atributo que excluye la librería estándar, dejando solo `core` y opcionalmente `alloc` |
| **`core`** | Subconjunto de `std` sin heap ni OS — siempre disponible |
| **`alloc`** | Crate que provee `Vec`, `Box`, `String` cuando hay allocator pero no OS |
| **`#[global_allocator]`** | Atributo para registrar un allocator de memoria personalizado |
| **`panic = "abort"`** | Estrategia de panic que termina inmediatamente sin unwinding |
| **`#[panic_handler]`** | Función que define el comportamiento en un panic en `no_std` |
| **heapless** | Crate con estructuras de datos de capacidad fija en compile-time (sin heap) |
| **`heapless::Vec<T, N>`** | Vector con capacidad máxima `N` conocida en tiempo de compilación |
| **`heapless::spsc::Queue`** | Cola single-producer/single-consumer para ISR ↔ main loop |
| **HAL** | Hardware Abstraction Layer — capa de abstracción sobre registros de hardware |
| **embedded-hal** | Colección de traits Rust para abstraer periféricos (GPIO, UART, SPI, I2C) |
| **RTIC** | Real-Time Interrupt-driven Concurrency — framework de concurrencia sin OS |
| **probe-rs** | Herramienta Rust para flashear y depurar microcontroladores |
| **defmt** | Framework de logging eficiente para sistemas embebidos |
| **RTT** | Real-Time Transfer — protocolo de logging via debug probe |
| **ISR** | Interrupt Service Routine — rutina ejecutada al ocurrir una interrupción |
| **bare metal** | Programación directa sobre el hardware sin sistema operativo |
