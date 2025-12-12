// src/ventas/carrito.rs
// Carrito de compras

use crate::productos::inventario::Producto;
use crate::productos::precio;

/// Item en el carrito
#[derive(Debug, Clone)]
pub struct ItemCarrito {
    pub producto: Producto,
    pub cantidad: u32,
}

impl ItemCarrito {
    pub fn subtotal(&self) -> f64 {
        self.producto.precio_base() * self.cantidad as f64
    }

    pub fn total_con_iva(&self) -> f64 {
        precio::calcular_precio_final(&self.producto, self.cantidad)
    }
}

/// Carrito de compras
#[derive(Debug, Default)]
pub struct Carrito {
    items: Vec<ItemCarrito>,
}

impl Carrito {
    pub fn nuevo() -> Self {
        Self { items: Vec::new() }
    }

    pub fn agregar(&mut self, producto: Producto, cantidad: u32) {
        // Buscar si el producto ya existe en el carrito
        if let Some(item) = self.items.iter_mut().find(|i| i.producto.id == producto.id) {
            item.cantidad += cantidad;
        } else {
            self.items.push(ItemCarrito { producto, cantidad });
        }
    }

    pub fn eliminar(&mut self, producto_id: u32) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.producto.id == producto_id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn items(&self) -> &[ItemCarrito] {
        &self.items
    }

    pub fn cantidad_items(&self) -> usize {
        self.items.iter().map(|i| i.cantidad as usize).sum()
    }

    pub fn subtotal(&self) -> f64 {
        self.items.iter().map(|i| i.subtotal()).sum()
    }

    pub fn iva_total(&self) -> f64 {
        precio::calcular_iva(self.subtotal())
    }

    pub fn total(&self) -> f64 {
        self.items.iter().map(|i| i.total_con_iva()).sum()
    }

    pub fn esta_vacio(&self) -> bool {
        self.items.is_empty()
    }

    pub fn vaciar(&mut self) {
        self.items.clear();
    }

    pub fn resumen(&self) -> String {
        let mut resultado = String::from("=== Resumen del Carrito ===\n");

        for item in &self.items {
            resultado.push_str(&format!(
                "- {} x{}: {:.2}€\n",
                item.producto.nombre,
                item.cantidad,
                item.subtotal()
            ));
        }

        resultado.push_str(&format!("\nSubtotal: {:.2}€\n", self.subtotal()));
        resultado.push_str(&format!("IVA ({}%): {:.2}€\n", precio::IVA * 100.0, self.iva_total()));
        resultado.push_str(&format!("Total: {:.2}€", self.total()));

        resultado
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn producto_test(id: u32, precio: f64) -> Producto {
        Producto::nuevo(id, &format!("Producto {}", id), precio)
    }

    #[test]
    fn test_carrito_nuevo_vacio() {
        let carrito = Carrito::nuevo();
        assert!(carrito.esta_vacio());
        assert_eq!(carrito.cantidad_items(), 0);
    }

    #[test]
    fn test_agregar_producto() {
        let mut carrito = Carrito::nuevo();
        let p = producto_test(1, 100.0);

        carrito.agregar(p, 2);

        assert!(!carrito.esta_vacio());
        assert_eq!(carrito.cantidad_items(), 2);
        assert_eq!(carrito.items().len(), 1);
    }

    #[test]
    fn test_agregar_mismo_producto() {
        let mut carrito = Carrito::nuevo();
        let p1 = producto_test(1, 100.0);
        let p2 = producto_test(1, 100.0);

        carrito.agregar(p1, 2);
        carrito.agregar(p2, 3);

        assert_eq!(carrito.items().len(), 1);
        assert_eq!(carrito.cantidad_items(), 5);
    }

    #[test]
    fn test_eliminar_producto() {
        let mut carrito = Carrito::nuevo();
        carrito.agregar(producto_test(1, 100.0), 1);
        carrito.agregar(producto_test(2, 50.0), 2);

        assert!(carrito.eliminar(1));
        assert_eq!(carrito.items().len(), 1);
        assert!(!carrito.eliminar(99)); // No existe
    }

    #[test]
    fn test_calcular_subtotal() {
        let mut carrito = Carrito::nuevo();
        carrito.agregar(producto_test(1, 100.0), 2); // 200
        carrito.agregar(producto_test(2, 50.0), 3); // 150

        assert!((carrito.subtotal() - 350.0).abs() < 0.01);
    }

    #[test]
    fn test_calcular_total_con_iva() {
        let mut carrito = Carrito::nuevo();
        carrito.agregar(producto_test(1, 100.0), 1);

        // 100 * 1.21 = 121
        assert!((carrito.total() - 121.0).abs() < 0.01);
    }

    #[test]
    fn test_vaciar_carrito() {
        let mut carrito = Carrito::nuevo();
        carrito.agregar(producto_test(1, 100.0), 1);
        carrito.agregar(producto_test(2, 50.0), 2);

        carrito.vaciar();

        assert!(carrito.esta_vacio());
    }
}
