# Práctica 03 — `heapless`: estructuras sin heap

## 🎯 Objetivo
Usar `heapless::Vec` y `heapless::String` con capacidad fija en compile-time.

## 🛠️ Cómo ejecutar

```bash
cargo test
```

## ✅ Criterios de Aceptación

- [ ] `BufferTemperatura` con capacidad máxima de 16 lecturas
- [ ] Operaciones: promedio, mínimo, máximo
- [ ] `agregar` retorna `Err` cuando el buffer está lleno
- [ ] `cargo test` pasa
