# Práctica 03 — Python Bindings con PyO3

## 🎯 Objetivo
Crear una extensión Python nativa en Rust usando PyO3 y `maturin`.

## 📋 Instrucciones

1. Implementar `#[pyfunction] fn suma(a: i64, b: i64) -> i64`
2. Implementar `#[pyfunction] fn contar_palabras(texto: &str) -> usize`
3. Crear `#[pyclass] struct Contador` con métodos `#[pymethods]`
4. Registrar todo en el `#[pymodule]`
5. Compilar y probar desde Python con `maturin develop`

## 🛠️ Cómo compilar y probar

```bash
# Instalar maturin
pip install maturin

# Compilar en modo desarrollo (instala en el venv actual)
maturin develop

# Probar desde Python
python3 -c "
import practice_03_pyo3_basico as m
print(m.suma(2, 3))           # 5
print(m.contar_palabras('hola mundo rust'))  # 3
c = m.Contador(0)
c.incrementar()
print(c.valor())              # 1
"
```

## ✅ Criterios de Aceptación

- [ ] `maturin develop` compila sin errores
- [ ] Las 3 funciones/clase son accesibles desde Python
- [ ] `cargo clippy -- -D warnings` pasa limpio
- [ ] Manejo de errores Python con `PyErr` (al menos 1 función que pueda fallar)

## 💡 Pistas

- `PyResult<T>` es el tipo de retorno para funciones que pueden lanzar excepciones Python
- `#[pyo3(get, set)]` expone campos de un `#[pyclass]` como propiedades Python
- `maturin build --release` genera un `.whl` instalable
