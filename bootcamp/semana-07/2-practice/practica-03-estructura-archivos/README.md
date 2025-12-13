# Práctica 03: Estructura de Archivos

## 🎯 Objetivo

Aprender a organizar código Rust en múltiples archivos y carpetas usando el estilo moderno (Rust 2018+).

## 📋 Estructura a Crear

```
src/
├── main.rs
├── productos.rs         # Declara submódulos
├── productos/
│   ├── inventario.rs
│   └── precio.rs
├── ventas.rs            # Declara submódulos
└── ventas/
    ├── carrito.rs
    └── factura.rs
```

## 📋 Instrucciones

### Paso 1: Crear `productos.rs`

```rust
// src/productos.rs
pub mod inventario;
pub mod precio;

// Re-exportar items importantes
pub use inventario::Producto;
pub use precio::calcular_precio_final;
```

### Paso 2: Crear `productos/inventario.rs`

```rust
// src/productos/inventario.rs

#[derive(Debug, Clone)]
pub struct Producto {
    pub id: u32,
    pub nombre: String,
    pub cantidad: u32,
    precio_base: f64,  // privado
}

impl Producto {
    pub fn nuevo(id: u32, nombre: &str, precio: f64) -> Self {
        // TODO: Implementar
        todo!()
    }
    
    pub fn precio_base(&self) -> f64 {
        self.precio_base
    }
    
    pub fn agregar_stock(&mut self, cantidad: u32) {
        // TODO: Implementar
        todo!()
    }
    
    pub fn reducir_stock(&mut self, cantidad: u32) -> Result<(), &'static str> {
        // TODO: Verificar que hay suficiente stock
        todo!()
    }
}
```

### Paso 3: Crear `productos/precio.rs`

```rust
// src/productos/precio.rs
use super::inventario::Producto;

pub const IVA: f64 = 0.21;

pub fn calcular_precio_final(producto: &Producto, cantidad: u32) -> f64 {
    // TODO: precio_base * cantidad * (1 + IVA)
    todo!()
}

pub fn aplicar_descuento(precio: f64, porcentaje: f64) -> f64 {
    // TODO: Implementar
    todo!()
}
```

### Paso 4: Crear módulo `ventas`

Crea la misma estructura para ventas con:

- `Carrito`: lista de productos y cantidades
- `Factura`: genera resumen de compra

### Paso 5: Actualizar `main.rs`

```rust
// src/main.rs
mod productos;
mod ventas;

use productos::Producto;
use ventas::carrito::Carrito;

fn main() {
    // Crear productos
    let laptop = Producto::nuevo(1, "Laptop", 999.99);
    let mouse = Producto::nuevo(2, "Mouse", 29.99);
    
    // Crear carrito y agregar productos
    let mut carrito = Carrito::nuevo();
    carrito.agregar(laptop.clone(), 1);
    carrito.agregar(mouse.clone(), 2);
    
    // Mostrar resumen
    println!("{}", carrito.resumen());
}
```

## ✅ Tests

Los tests deben estar en cada archivo de módulo:

```rust
// En productos/inventario.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_producto() {
        let p = Producto::nuevo(1, "Test", 100.0);
        assert_eq!(p.id, 1);
        assert_eq!(p.nombre, "Test");
        assert_eq!(p.precio_base(), 100.0);
    }

    #[test]
    fn test_stock() {
        let mut p = Producto::nuevo(1, "Test", 100.0);
        p.agregar_stock(10);
        assert_eq!(p.cantidad, 10);
        
        assert!(p.reducir_stock(5).is_ok());
        assert_eq!(p.cantidad, 5);
        
        assert!(p.reducir_stock(10).is_err());
    }
}
```

## 🎯 Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Estructura de archivos correcta | 30% |
| Módulo productos funcional | 25% |
| Módulo ventas funcional | 25% |
| Tests en cada módulo | 20% |

## 💡 Pistas

1. El archivo `modulo.rs` declara los submódulos con `pub mod nombre;`
2. Los submódulos van en la carpeta `modulo/`
3. Usa `pub use` para re-exportar y simplificar imports
4. `super::` accede al módulo padre (útil entre submódulos)
5. Ejecuta `cargo build` frecuentemente para verificar la estructura
