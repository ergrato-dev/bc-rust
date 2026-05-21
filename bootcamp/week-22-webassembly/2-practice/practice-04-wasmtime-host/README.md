# Práctica 04 — Wasmtime Host

## 🎯 Objetivo
Ejecutar un módulo WASM desde Rust usando Wasmtime como runtime embebido.

## 📋 Instrucciones

1. Crear un `Engine` y `Store` con Wasmtime
2. Compilar un módulo WAT con una función `suma`
3. Instanciar el módulo e invocar la función exportada
4. Verificar el resultado

## 🛠️ Cómo ejecutar

```bash
cargo run
# Salida esperada:
# suma(10, 32) = 42
# suma(100, -58) = 42
# ✓ Wasmtime ejecutó el módulo WASM correctamente
```

## ✅ Criterios de Aceptación

- [ ] El host Rust carga y ejecuta un módulo WASM
- [ ] La función exportada retorna el valor correcto
- [ ] `cargo run` sin errores
