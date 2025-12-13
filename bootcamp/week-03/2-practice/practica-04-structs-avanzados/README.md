# 📦 Práctica 04: Structs Avanzados

## 🎯 Objetivo

Practicar tuple structs, unit structs y structs anidados.

## 📋 Instrucciones

### Parte 1: Tuple Structs

1. Crea `Color(u8, u8, u8)` para RGB
2. Crea `Punto(f64, f64)` para coordenadas
3. Implementa métodos para ambos

### Parte 2: Newtype Pattern

1. Crea `UserId(u64)` y `ProductId(u64)`
2. Demuestra que no se pueden mezclar

### Parte 3: Structs Anidados

1. Crea `Direccion` con calle, ciudad, cp
2. Crea `Persona` que contiene `Direccion`

## ✅ Criterios de Éxito

- [ ] Tuple structs con métodos
- [ ] Newtype pattern aplicado
- [ ] Structs anidados funcionando

## 🧪 Ejecutar

```bash
cargo run
cargo test
```
