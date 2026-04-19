# 🎰 Proyecto: Máquina de Estados

## 📋 Descripción

Implementarás una **máquina de estados** para un sistema de pedidos de una tienda online, aplicando todos los conceptos de enums y pattern matching de esta semana.

## 🎯 Objetivos de Aprendizaje

- Modelar estados con enums
- Transiciones con pattern matching
- Manejar datos asociados a cada estado
- Usar Option y Result para operaciones
- Aplicar if let y guards

## 📦 El Sistema de Pedidos

Un pedido puede estar en los siguientes estados:

```
Creado → Pagado → Enviado → Entregado
   ↓        ↓        ↓
Cancelado  Cancelado  Devuelto
```

## 🔧 Requisitos

### 1. Enum `EstadoPedido`

```rust
enum EstadoPedido {
    Creado { fecha: String },
    Pagado { fecha: String, monto: f64 },
    Enviado { fecha: String, tracking: String },
    Entregado { fecha: String },
    Cancelado { fecha: String, razon: String },
    Devuelto { fecha: String, razon: String },
}
```

### 2. Struct `Pedido`

```rust
struct Pedido {
    id: u32,
    cliente: String,
    items: Vec<String>,
    estado: EstadoPedido,
}
```

### 3. Métodos del Pedido

| Método | Descripción |
|--------|-------------|
| `nuevo()` | Crea un pedido en estado Creado |
| `pagar()` | Transición a Pagado (solo desde Creado) |
| `enviar()` | Transición a Enviado (solo desde Pagado) |
| `entregar()` | Transición a Entregado (solo desde Enviado) |
| `cancelar()` | Transición a Cancelado (desde Creado o Pagado) |
| `devolver()` | Transición a Devuelto (solo desde Entregado) |
| `puede_cancelar()` | Verifica si se puede cancelar |
| `descripcion_estado()` | Descripción legible del estado |

### 4. Sistema de Gestión

```rust
struct GestorPedidos {
    pedidos: Vec<Pedido>,
}
```

Con métodos para:
- Agregar pedidos
- Buscar por ID
- Listar por estado
- Obtener estadísticas

## 🧪 Tests Requeridos

```bash
cargo test
```

Mínimo 12 tests cubriendo:
- Transiciones válidas
- Transiciones inválidas (retornan Error)
- Búsquedas con Option
- Filtrado por estado

## ✅ Criterios de Evaluación

| Criterio | Peso |
|----------|------|
| Enums bien definidos | 20% |
| Transiciones correctas | 25% |
| Pattern matching exhaustivo | 20% |
| Uso de Option/Result | 20% |
| Tests completos | 15% |

## 💡 Pistas

<details>
<summary>Pista: Transición con Result</summary>

```rust
fn pagar(&mut self, monto: f64) -> Result<(), &'static str> {
    match &self.estado {
        EstadoPedido::Creado { .. } => {
            self.estado = EstadoPedido::Pagado { 
                fecha: obtener_fecha(), 
                monto 
            };
            Ok(())
        }
        _ => Err("Solo se puede pagar un pedido creado")
    }
}
```

</details>

<details>
<summary>Pista: matches! para filtrar</summary>

```rust
fn listar_pagados(&self) -> Vec<&Pedido> {
    self.pedidos.iter()
        .filter(|p| matches!(p.estado, EstadoPedido::Pagado { .. }))
        .collect()
}
```

</details>

## 🚀 Extensiones Opcionales

1. **Historial**: Guardar historial de estados
2. **Reembolsos**: Calcular reembolsos según estado
3. **Notificaciones**: Enum de notificaciones por transición
4. **Validaciones**: Tiempos mínimos entre transiciones
