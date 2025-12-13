# Práctica 03: Option y Métodos

## 🎯 Objetivo

Dominar el uso de `Option<T>` y sus métodos principales.

## 📋 Instrucciones

### Ejercicio 1: Búsqueda con Option

Implementa `buscar_usuario` que busque en un vector de usuarios por ID y retorne `Option<&Usuario>`.

### Ejercicio 2: Métodos de Option

Usa los métodos de Option para transformar valores:
- `map` para transformar el valor interno
- `unwrap_or` para valores por defecto
- `and_then` para encadenar operaciones

### Ejercicio 3: Option en Structs

Crea un struct `Perfil` con campos opcionales (email, telefono) y métodos para accederlos de forma segura.

## 🧪 Tests

```bash
cargo test
```

## ✅ Criterios de Éxito

- [ ] No usar `unwrap()` en código de producción
- [ ] Manejo correcto de None
- [ ] Métodos de Option bien aplicados
- [ ] Todos los tests pasan
