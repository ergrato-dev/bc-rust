# 📝 Práctica 01: Lifetimes Básicos en Funciones

## 🎯 Objetivo

Aprender a identificar cuándo se necesitan anotaciones de lifetime en funciones y cómo aplicarlas correctamente.

## 📋 Ejercicios

### Ejercicio 1: `longest` ⭐⭐
Función que retorna la cadena más larga de dos.

**Pregunta clave:** ¿Por qué el compilador no puede inferir el lifetime?

### Ejercicio 2: `first_word` ⭐
Retorna la primera palabra de una cadena.

**Pregunta clave:** ¿Necesita lifetime explícito? ¿Por qué?

### Ejercicio 3: `get_prefix` ⭐
Retorna el texto antes de un delimitador.

**Pregunta clave:** ¿El parámetro `char` afecta la necesidad de lifetimes?

### Ejercicio 4: `pick_one` ⭐⭐
Selecciona una de dos cadenas basándose en un flag.

**Pregunta clave:** ¿Qué similitud tiene con `longest`?

### Ejercicio 5: `skip_prefix` ⭐
Elimina caracteres iniciales que coincidan con un patrón.

**Pregunta clave:** ¿Por qué funciona sin anotaciones?

## 🏃 Ejecución

```bash
# Desde el directorio del ejercicio
cargo run

# Ejecutar tests
cargo test

# Ver la solución
cargo run --bin solucion
```

## 💡 Pistas

1. **Regla de Elision #2**: Si hay exactamente una referencia de entrada, su lifetime se aplica a la salida.

2. **Múltiples referencias**: Cuando hay varias referencias que podrían retornarse, necesitas anotación explícita.

3. **Tipos por valor**: `char`, `bool`, `i32`, etc. no son referencias, no cuentan para las reglas de elision.

## ✅ Criterios de Éxito

- [ ] El código compila sin errores
- [ ] Todos los tests pasan
- [ ] Entiendes por qué algunos ejercicios necesitan `'a` y otros no

## 🔍 Reglas de Elision

| Regla | Descripción |
|-------|-------------|
| **#1** | Cada ref de entrada obtiene su propio lifetime |
| **#2** | Si hay 1 input lifetime, se aplica a outputs |
| **#3** | Si hay `&self`, su lifetime se aplica a outputs |

## 📚 Recursos

- [The Rust Book - Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust by Example - Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
