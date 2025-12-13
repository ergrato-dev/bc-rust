# 🔧 Práctica 01: Declaración de Variables

## 🎯 Objetivo

Practicar la declaración de variables inmutables (`let`) y mutables (`let mut`) en Rust.

## 📋 Instrucciones

### Parte 1: Variables Inmutables

1. Declara una variable `nombre` con tu nombre
2. Declara una variable `edad` con tu edad
3. Declara una variable `es_estudiante` con valor `true`
4. Imprime todas las variables

### Parte 2: Variables Mutables

1. Declara una variable mutable `contador` inicializada en 0
2. Incrementa el contador 3 veces
3. Imprime el valor final

### Parte 3: Errores Comunes

1. Intenta modificar una variable inmutable (observa el error)
2. Corrige el error usando `mut`

## ✅ Criterios de Éxito

- [ ] El código compila sin warnings (`cargo clippy`)
- [ ] Todas las variables se imprimen correctamente
- [ ] Se entiende la diferencia entre `let` y `let mut`

## 🧪 Ejecutar

```bash
cargo run
cargo test
```

## 💡 Pistas

- Las variables inmutables se declaran con `let`
- Las variables mutables requieren `let mut`
- Usa `println!("Valor: {}", variable)` para imprimir
