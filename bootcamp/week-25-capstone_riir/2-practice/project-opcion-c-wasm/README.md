# Opción C — Motor Estadístico WASM

## 🎯 Descripción
Motor de cálculo estadístico compilado a WebAssembly para uso en navegadores y Node.js.

## 🛠️ Cómo compilar y probar

```bash
wasm-pack build --target nodejs
cargo test
wasm-pack test --node
```

## ✅ Criterios de Aceptación

- [ ] `Estadisticas` con media, mediana, varianza, desviación estándar, mínimo, máximo
- [ ] `correlacion_pearson` como función libre
- [ ] `cargo test` pasa
- [ ] `wasm-pack build --target nodejs` sin errores
