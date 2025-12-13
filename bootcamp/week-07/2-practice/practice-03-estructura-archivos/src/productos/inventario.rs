// src/productos/inventario.rs
// Gestión de inventario de productos

#[derive(Debug, Clone)]
pub struct Producto {
    pub id: u32,
    pub nombre: String,
    pub cantidad: u32,
    precio_base: f64, // privado - acceso controlado
}

impl Producto {
    pub fn nuevo(id: u32, nombre: &str, precio: f64) -> Self {
        Self {
            id,
            nombre: nombre.to_string(),
            cantidad: 0,
            precio_base: precio,
        }
    }

    pub fn precio_base(&self) -> f64 {
        self.precio_base
    }

    pub fn agregar_stock(&mut self, cantidad: u32) {
        self.cantidad += cantidad;
    }

    pub fn reducir_stock(&mut self, cantidad: u32) -> Result<(), &'static str> {
        if cantidad > self.cantidad {
            return Err("Stock insuficiente");
        }
        self.cantidad -= cantidad;
        Ok(())
    }

    pub fn hay_stock(&self) -> bool {
        self.cantidad > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_producto() {
        let p = Producto::nuevo(1, "Test", 100.0);
        assert_eq!(p.id, 1);
        assert_eq!(p.nombre, "Test");
        assert_eq!(p.precio_base(), 100.0);
        assert_eq!(p.cantidad, 0);
    }

    #[test]
    fn test_agregar_stock() {
        let mut p = Producto::nuevo(1, "Test", 100.0);
        p.agregar_stock(10);
        assert_eq!(p.cantidad, 10);

        p.agregar_stock(5);
        assert_eq!(p.cantidad, 15);
    }

    #[test]
    fn test_reducir_stock_exitoso() {
        let mut p = Producto::nuevo(1, "Test", 100.0);
        p.agregar_stock(10);

        assert!(p.reducir_stock(5).is_ok());
        assert_eq!(p.cantidad, 5);
    }

    #[test]
    fn test_reducir_stock_insuficiente() {
        let mut p = Producto::nuevo(1, "Test", 100.0);
        p.agregar_stock(5);

        assert!(p.reducir_stock(10).is_err());
        assert_eq!(p.cantidad, 5); // Sin cambios
    }

    #[test]
    fn test_hay_stock() {
        let mut p = Producto::nuevo(1, "Test", 100.0);
        assert!(!p.hay_stock());

        p.agregar_stock(1);
        assert!(p.hay_stock());
    }
}
