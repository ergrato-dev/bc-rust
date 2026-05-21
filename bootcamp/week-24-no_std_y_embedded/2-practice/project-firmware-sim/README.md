# Proyecto — Firmware Simulator

## 🎯 Objetivo
Simular el núcleo de un firmware embebido: cola de eventos y buffer ADC, sin heap.

## 📋 Componentes

| Componente | Descripción |
|------------|-------------|
| `ColaEventos` | Cola SPSC de hasta 16 eventos (simula ISR → main loop) |
| `Evento` | Enum de eventos del sistema |
| `BufferAdc` | Buffer de hasta 32 muestras ADC de 12 bits |

## 🛠️ Cómo ejecutar

```bash
cargo test
```

## ✅ Criterios de Aceptación

- [ ] `#![no_std]` con `heapless`
- [ ] `ColaEventos` con capacidad 16, FIFO correcto
- [ ] `BufferAdc` con promedio en punto fijo
- [ ] Máscara de 12 bits aplicada al ingresar muestras
- [ ] `cargo test` pasa
