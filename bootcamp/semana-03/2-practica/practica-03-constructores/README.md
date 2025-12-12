# 🏭 Práctica 03: Constructores

## 🎯 Objetivo

Implementar funciones asociadas y el patrón constructor `new()`.

## 📋 Instrucciones

### Parte 1: Constructor Básico

1. Crea un struct `Usuario` con nombre, email, edad
2. Implementa `Usuario::new(nombre, email)` con edad = 0

### Parte 2: Múltiples Constructores

1. Implementa `Usuario::con_edad(nombre, email, edad)`
2. Implementa `Usuario::anonimo()` con valores por defecto

### Parte 3: Validación en Constructor

1. Implementa validación de edad (no negativa)
2. Retorna Option<Usuario> si hay validación

## ✅ Criterios de Éxito

- [ ] Usar `Self` en lugar del nombre del struct
- [ ] Constructor `new()` implementado
- [ ] Llamadas con `Tipo::funcion()`

## 🧪 Ejecutar

```bash
cargo run
cargo test
```
