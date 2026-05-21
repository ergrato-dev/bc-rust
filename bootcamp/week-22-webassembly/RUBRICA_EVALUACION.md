# 📊 Rúbrica de Evaluación — Semana 22: WebAssembly

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Compilar Rust a WASM con `wasm-pack build` |
| **C2** | Exponer funciones Rust a JavaScript con `#[wasm_bindgen]` |
| **C3** | Pasar tipos complejos entre JS y WASM |
| **C4** | Ejecutar un módulo WASM desde Wasmtime (host Rust) |
| **C5** | Manejar las restricciones de WASM (no threads, no fs) |
| **C6** | Escribir tests para código WASM con `wasm-bindgen-test` |

---

## 📋 Distribución de Puntos

| Tipo | Peso | Puntos |
|------|------|--------|
| **Conocimiento** | 30% | 30 pts |
| **Desempeño** | 40% | 40 pts |
| **Producto** | 30% | 30 pts |
| **Total** | 100% | 100 pts |

---

## 🧠 Evaluación de Conocimiento (30 pts)

1. ¿Qué es WebAssembly y cuáles son sus ventajas sobre JavaScript puro? (5 pts)
2. ¿Por qué no se puede usar `std::thread` en WASM? (5 pts)
3. ¿Qué diferencia hay entre `wasm32-unknown-unknown` y `wasm32-wasi`? (5 pts)
4. ¿Qué hace `wasm-bindgen` a nivel de código? (5 pts)
5. ¿Cómo se transfieren strings entre WASM y JavaScript? (5 pts)
6. ¿Qué es Wasmtime y para qué sirve? (5 pts)

---

## ⚙️ Evaluación de Desempeño (40 pts)

### Crear y ejecutar un módulo WASM (20 pts)

- [ ] `wasm-pack build --target nodejs` genera `.wasm` sin errores (10 pts)
- [ ] Función `greet(name: &str) -> String` accesible desde Node.js (10 pts)

### Ejecutar WASM desde Wasmtime (20 pts)

- [ ] Código Rust que carga un `.wasm` con Wasmtime (10 pts)
- [ ] Llamada a función exportada del módulo WASM (10 pts)

---

## 🏗️ Evaluación de Producto (30 pts)

### Proyecto: `project-calculadora-wasm` (30 pts)

| Criterio | Puntos |
|----------|--------|
| Funciones aritméticas básicas accesibles desde JS/Node | 10 pts |
| Struct `Calculadora` con `#[wasm_bindgen]` y estado | 10 pts |
| Tests con `wasm-bindgen-test` | 5 pts |
| `wasm-pack build` sin warnings | 5 pts |

---

## 📈 Escala de Calificación

| Rango | Nota |
|-------|------|
| 90-100 | Sobresaliente |
| 75-89 | Notable |
| 60-74 | Aprobado |
| < 60 | Suspendido |
