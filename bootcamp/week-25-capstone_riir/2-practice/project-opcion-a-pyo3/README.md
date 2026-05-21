# Opción A — Parser/Lexer + PyO3

## 🎯 Descripción
Tokenizador de expresiones matemáticas implementado en Rust y expuesto a Python via PyO3.

## 🛠️ Build y uso

```bash
# Requiere maturin
pip install maturin

# Build en modo desarrollo
maturin develop

# Probar desde Python
python3 -c "
import capstone_opcion_a as m
tokens = m.tokenizar('3 + 4 * 2')
print(tokens)
print(m.evaluar_simple('10 + 5 - 3'))  # 12.0
"
```

## ✅ Criterios de Aceptación

- [ ] `tokenizar(expr)` retorna lista de `Token`
- [ ] `evaluar_simple(expr)` retorna `float`
- [ ] Errores de Python correctos (ValueError, ZeroDivisionError)
- [ ] `cargo test` pasa
- [ ] `maturin develop` compila sin errores
