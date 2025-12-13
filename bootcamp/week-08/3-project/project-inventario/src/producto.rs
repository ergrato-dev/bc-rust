//! Módulo de productos

use std::fmt;

/// Representa un producto en el inventario
#[derive(Debug, Clone)]
pub struct Producto {
    pub id: u32,
    pub nombre: String,
    pub descripcion: String,
    pub precio: f64,
    pub categoria: String,
    pub stock: u32,
}

impl Producto {
    /// Crea un nuevo producto
    pub fn new(
        id: u32,
        nombre: impl Into<String>,
        descripcion: impl Into<String>,
        precio: f64,
        categoria: impl Into<String>,
        stock: u32,
    ) -> Self {
        Self {
            id,
            nombre: nombre.into(),
            descripcion: descripcion.into(),
            precio,
            categoria: categoria.into(),
            stock,
        }
    }

    /// Calcula el valor total en inventario
    pub fn valor_inventario(&self) -> f64 {
        self.precio * self.stock as f64
    }

    /// Verifica si el stock está bajo el umbral
    pub fn stock_bajo(&self, umbral: u32) -> bool {
        self.stock < umbral
    }

    /// Actualiza el stock (entrada)
    pub fn agregar_stock(&mut self, cantidad: u32) {
        self.stock += cantidad;
    }

    /// Actualiza el stock (salida), retorna true si fue exitoso
    pub fn retirar_stock(&mut self, cantidad: u32) -> bool {
        if self.stock >= cantidad {
            self.stock -= cantidad;
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Producto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} - ${:.2} (Stock: {}) [{}]",
            self.id, self.nombre, self.precio, self.stock, self.categoria
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuevo_producto() {
        let p = Producto::new(1, "Test", "Desc", 10.0, "Cat", 5);
        assert_eq!(p.id, 1);
        assert_eq!(p.nombre, "Test");
        assert_eq!(p.stock, 5);
    }

    #[test]
    fn test_valor_inventario() {
        let p = Producto::new(1, "Test", "Desc", 10.0, "Cat", 5);
        assert_eq!(p.valor_inventario(), 50.0);
    }

    #[test]
    fn test_stock_bajo() {
        let p = Producto::new(1, "Test", "Desc", 10.0, "Cat", 3);
        assert!(p.stock_bajo(5));
        assert!(!p.stock_bajo(2));
    }

    #[test]
    fn test_agregar_stock() {
        let mut p = Producto::new(1, "Test", "Desc", 10.0, "Cat", 5);
        p.agregar_stock(10);
        assert_eq!(p.stock, 15);
    }

    #[test]
    fn test_retirar_stock_exitoso() {
        let mut p = Producto::new(1, "Test", "Desc", 10.0, "Cat", 10);
        assert!(p.retirar_stock(5));
        assert_eq!(p.stock, 5);
    }

    #[test]
    fn test_retirar_stock_insuficiente() {
        let mut p = Producto::new(1, "Test", "Desc", 10.0, "Cat", 3);
        assert!(!p.retirar_stock(5));
        assert_eq!(p.stock, 3);
    }
}
