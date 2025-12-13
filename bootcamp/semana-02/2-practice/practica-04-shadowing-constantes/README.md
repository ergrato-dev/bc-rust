# 🔄 Práctica 04: Shadowing y Constantes

## 🎯 Objetivo

Dominar el shadowing de variables y el uso de constantes en Rust.

## 📋 Instrucciones

### Parte 1: Shadowing Básico

1. Declara una variable y haz shadowing con nuevo valor
2. Observa que se crea una nueva variable
3. Compara con mutabilidad

### Parte 2: Shadowing con Cambio de Tipo

1. Declara un &str y haz shadowing a usize (longitud)
2. Convierte un String a su longitud
3. Entiende por qué esto no funciona con mut

### Parte 3: Shadowing en Scopes

1. Experimenta con shadowing dentro de bloques {}
2. Observa qué valor tiene la variable fuera del bloque

### Parte 4: Constantes

1. Declara constantes con `const`
2. Usa SCREAMING_SNAKE_CASE
3. Entiende cuándo usar const vs let

## ✅ Criterios de Éxito

- [ ] Entender la diferencia entre shadowing y mut
- [ ] Poder cambiar el tipo de una variable con shadowing
- [ ] Usar constantes correctamente

## 🧪 Ejecutar

```bash
cargo run
cargo test
```
