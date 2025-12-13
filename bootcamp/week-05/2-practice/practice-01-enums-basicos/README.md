# Práctica 01: Enums Básicos

## 🎯 Objetivo

Aprender a definir y usar enums simples en Rust.

## 📋 Instrucciones

### Ejercicio 1: Definir un Enum

Crea un enum `DiaSemana` con las 7 variantes para los días de la semana.

### Ejercicio 2: Función con Enum

Implementa la función `es_laboral` que reciba un `DiaSemana` y retorne `true` si es día laboral (lunes a viernes).

### Ejercicio 3: Enum con Métodos

Agrega un método `siguiente` al enum que retorne el siguiente día de la semana.

## 🧪 Tests

```bash
cargo test
```

## ✅ Criterios de Éxito

- [ ] El enum tiene las 7 variantes
- [ ] `es_laboral` funciona correctamente
- [ ] El método `siguiente` cicla correctamente
- [ ] Todos los tests pasan

## 💡 Pistas

<details>
<summary>Pista 1: Definir enum</summary>

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum DiaSemana {
    Lunes,
    Martes,
    // ...
}
```

</details>

<details>
<summary>Pista 2: Match con OR</summary>

```rust
match dia {
    DiaSemana::Sabado | DiaSemana::Domingo => false,
    _ => true,
}
```

</details>
