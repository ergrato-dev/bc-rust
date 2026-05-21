# 🦀 Semana 24: `no_std` e Intro a Embedded

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 24 de 25 |
| **Tema** | Bare metal Rust, `#![no_std]`, HAL, IoT |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Sistemas y Performance |
| **Requisitos** | Semana 23, conocimiento básico de sistemas embebidos |

---

## 🎯 Objetivos de Aprendizaje

1. **Entender** por qué existe `#![no_std]` y cuándo usarlo
2. **Programar** usando solo `core::` (sin `std::`)
3. **Gestionar** memoria sin el allocator estándar
4. **Usar** `heapless` para estructuras de datos sin heap
5. **Implementar** un panic handler manual
6. **Entender** el modelo HAL (Hardware Abstraction Layer)

---

## ⚠️ Restricciones en `no_std`

| Prohibido | Alternativa |
|-----------|-------------|
| `println!` | `core::fmt::Write` + UART |
| `Vec`, `String` | `heapless::Vec`, `heapless::String` |
| `std::thread` | Bare metal + interrupciones |
| `std::sync` | `core::sync::atomic` |
| Allocator estándar | `#[global_allocator]` custom o sin heap |
