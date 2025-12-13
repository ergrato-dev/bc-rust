# 🔧 Práctica 02: Métodos

## 🎯 Objetivo

Implementar métodos usando bloques `impl`.

## 📋 Instrucciones

### Parte 1: Métodos de Lectura (&self)

1. Crea un struct `Rectangulo` con ancho y alto
2. Implementa métodos: `area()`, `perimetro()`, `es_cuadrado()`

### Parte 2: Métodos de Modificación (&mut self)

1. Implementa `escalar(&mut self, factor: u32)`
2. Implementa `rotar(&mut self)` que intercambia ancho y alto

### Parte 3: Métodos con Parámetros

1. Implementa `puede_contener(&self, otro: &Rectangulo) -> bool`

## ✅ Criterios de Éxito

- [ ] Usar `&self` para métodos de lectura
- [ ] Usar `&mut self` para métodos de modificación
- [ ] Tests pasan correctamente

## 🧪 Ejecutar

```bash
cargo run
cargo test
```
