# 📦 Proyecto: Sistema de Gestión de Inventario

## 🎯 Objetivo

Desarrollar un sistema completo de gestión de inventario que demuestre dominio de
las colecciones de Rust: `Vec<T>`, `String`, `HashMap<K, V>` e iteradores.

## 📋 Requisitos Funcionales

### 1. Gestión de Productos

- Crear, leer, actualizar y eliminar productos
- Cada producto tiene: ID, nombre, descripción, precio, categoría, stock
- Búsqueda por nombre (parcial) y por categoría

### 2. Gestión de Categorías

- Lista de categorías dinámicas
- Productos agrupados por categoría
- Estadísticas por categoría

### 3. Transacciones de Inventario

- Registrar entradas de stock
- Registrar salidas de stock  
- Historial de movimientos

### 4. Reportes

- Productos con stock bajo
- Valor total del inventario
- Top productos por valor
- Historial de movimientos

## 🏗️ Estructura del Proyecto

```
proyecto-inventario/
├── src/
│   ├── main.rs          # Punto de entrada y demo
│   ├── lib.rs           # Re-exportaciones
│   ├── producto.rs      # Struct Producto
│   ├── categoria.rs     # Gestión de categorías
│   ├── inventario.rs    # Sistema de inventario
│   ├── transaccion.rs   # Movimientos de stock
│   └── reportes.rs      # Generación de reportes
├── Cargo.toml
└── README.md
```

## 📐 Modelo de Datos

```rust
struct Producto {
    id: u32,
    nombre: String,
    descripcion: String,
    precio: f64,
    categoria: String,
    stock: u32,
}

enum TipoTransaccion {
    Entrada,
    Salida,
}

struct Transaccion {
    producto_id: u32,
    tipo: TipoTransaccion,
    cantidad: u32,
    fecha: String,
    nota: Option<String>,
}
```

## 🚀 Ejecución

```bash
# Ejecutar demo
cargo run

# Ejecutar tests
cargo test

# Tests con output
cargo test -- --nocapture
```

## 📊 Rúbrica de Evaluación

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| Funcionalidad | 40 | CRUD completo, búsquedas, transacciones |
| Uso de colecciones | 30 | Vec, String, HashMap correctamente usados |
| Iteradores | 20 | Uso idiomático de map, filter, fold, etc. |
| Código limpio | 10 | Formateo, documentación, tests |

## 💡 Pistas

1. Usa `HashMap<u32, Producto>` para acceso rápido por ID
2. Usa `HashMap<String, Vec<u32>>` para índice por categoría
3. Implementa `Display` para reportes legibles
4. Usa `Entry API` para operaciones de inventario

## 🎨 Extensiones Opcionales

- Exportar a formato CSV
- Importar desde CSV
- Sistema de alertas de stock
- Categorías jerárquicas
