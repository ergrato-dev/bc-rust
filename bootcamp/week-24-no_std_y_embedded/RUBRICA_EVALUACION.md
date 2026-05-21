# 📊 Rúbrica de Evaluación — Semana 24: `no_std` e Embedded

## 📋 Distribución de Puntos

| Tipo | Peso | Puntos |
|------|------|--------|
| **Conocimiento** | 30% | 30 pts |
| **Desempeño** | 40% | 40 pts |
| **Producto** | 30% | 30 pts |

---

## 🧠 Conocimiento (30 pts)

1. ¿Qué partes de la librería estándar de Rust están disponibles en `no_std`? (5 pts)
2. ¿Cuál es la diferencia entre `core`, `alloc` y `std`? (5 pts)
3. ¿Por qué se usa `panic = "abort"` en embedded? (5 pts)
4. ¿Qué es un `#[global_allocator]` y cuándo se necesita? (5 pts)
5. ¿Qué ventajas ofrece `heapless` sobre `Vec` en sistemas con memoria limitada? (5 pts)
6. ¿Qué es un HAL y para qué sirve en Rust embedded? (5 pts)

---

## ⚙️ Desempeño (40 pts)

- [ ] Librería `no_std` que compila sin errores (20 pts)
- [ ] Uso correcto de `heapless::Vec` con capacidad fija (20 pts)

---

## 🏗️ Producto (30 pts): `project-firmware-sim`

| Criterio | Puntos |
|----------|--------|
| `#![no_std]` con `heapless` para buffer de mensajes | 10 pts |
| Simulación de cola de eventos (sin heap) | 10 pts |
| Tests con `cargo test` (usa std en test config) | 10 pts |
