# 📝 Práctica 03: Reglas de Elision de Lifetimes

## 🎯 Objetivo

Dominar las reglas de elision para saber cuándo puedes omitir anotaciones de lifetime y cuándo son obligatorias.

## 📋 Las 3 Reglas de Elision

| Regla | Descripción | Ejemplo |
|-------|-------------|---------|
| **1** | Cada ref de entrada obtiene su propio lifetime | `fn(x: &T, y: &U)` → `fn<'a,'b>(x: &'a T, y: &'b U)` |
| **2** | Si hay 1 input lifetime, se aplica a outputs | `fn(x: &T) -> &T` → `fn<'a>(x: &'a T) -> &'a T` |
| **3** | Si hay &self, su lifetime se aplica a outputs | `fn(&self) -> &T` → `fn<'a>(&'a self) -> &'a T` |

## 📋 Ejercicios

### Ejercicio 1: Funciones Simples ⭐
Funciones con una sola referencia de entrada.

**Pregunta:** ¿Por qué no necesitan anotación?

### Ejercicio 2: Métodos con &self ⭐⭐
Métodos que retornan referencias.

**Pregunta:** ¿Qué pasa cuando hay otro parámetro de referencia además de &self?

### Ejercicio 3: Requieren Anotación ⭐⭐
Funciones con múltiples referencias.

**Pregunta:** ¿Por qué las reglas no aplican aquí?

### Ejercicio 4: Casos Mixtos ⭐⭐
Analiza si necesitan anotación o no.

**Pista:** `char`, `usize`, `bool` no son referencias.

### Ejercicio 5: Análisis ⭐⭐⭐
Determina qué regla aplica para cada caso.

## 🏃 Ejecución

```bash
cargo run
cargo test
```

## 💡 Regla de Oro

> **Si el compilador no se queja, no necesitas anotación.**
> 
> Empieza sin lifetimes, agrégalos solo cuando el compilador los pida.

## ✅ Criterios de Éxito

- [ ] Identificas correctamente qué regla aplica
- [ ] Sabes simplificar firmas omitiendo lifetimes innecesarios
- [ ] Distingues cuándo la anotación es obligatoria

## 📚 Tabla de Decisión Rápida

| Firma | ¿Necesita Anotación? |
|-------|---------------------|
| `fn f(x: &T) -> &T` | ❌ No (Regla 2) |
| `fn f(&self) -> &T` | ❌ No (Regla 3) |
| `fn f(&self, x: &T) -> &U` | ❌ No (Regla 3) |
| `fn f(x: &T, y: &U) -> &V` | ✅ Sí |
| `fn f(x: &T, n: i32) -> &T` | ❌ No (n no es ref) |
