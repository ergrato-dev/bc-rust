# Proyecto — Calculadora WASM

## 🎯 Objetivo
Construir una calculadora completa compilada a WASM, con estado persistente y accesible desde JavaScript/Node.js.

## 📋 Descripción

La `Calculadora` mantiene un acumulador y un historial de operaciones. Funciones adicionales de utilidad matemática (`raiz_cuadrada`, `potencia`) se exponen como funciones libres.

## 🛠️ Cómo compilar y probar

```bash
# Compilar para Node.js
wasm-pack build --target nodejs

# Probar desde Node.js
node -e "
const { Calculadora, raiz_cuadrada, potencia } = require('./pkg/project_calculadora_wasm.js');
const calc = new Calculadora();
calc.sumar(10);
calc.multiplicar(2);
calc.restar(5);
console.log(calc.valor());          // 15
console.log(calc.num_operaciones()); // 3
console.log(raiz_cuadrada(16));     // 4
console.log(potencia(2, 8));        // 256
"

# Tests nativos
cargo test

# Tests WASM
wasm-pack test --node
```

## ✅ Criterios de Aceptación

- [ ] `Calculadora` con `sumar`, `restar`, `multiplicar`, `dividir`, `resetear`
- [ ] División por cero retorna `NaN`
- [ ] Historial de operaciones con `num_operaciones()`
- [ ] Funciones libres `raiz_cuadrada` y `potencia`
- [ ] `cargo test` pasa
- [ ] `wasm-pack build --target nodejs` sin errores
