// src/productos/precio.rs
// Cálculos de precios e impuestos

use super::inventario::Producto;

/// Tasa de IVA (21%)
pub const IVA: f64 = 0.21;

/// Calcula el precio final incluyendo IVA
pub fn calcular_precio_final(producto: &Producto, cantidad: u32) -> f64 {
    let subtotal = producto.precio_base() * cantidad as f64;
    subtotal * (1.0 + IVA)
}

/// Aplica un porcentaje de descuento al precio
pub fn aplicar_descuento(precio: f64, porcentaje: f64) -> f64 {
    let descuento = precio * (porcentaje / 100.0);
    precio - descuento
}

/// Calcula el subtotal sin impuestos
pub fn calcular_subtotal(producto: &Producto, cantidad: u32) -> f64 {
    producto.precio_base() * cantidad as f64
}

/// Calcula solo el monto de IVA
pub fn calcular_iva(subtotal: f64) -> f64 {
    subtotal * IVA
}

#[cfg(test)]
mod tests {
    use super::*;

    fn producto_test() -> Producto {
        Producto::nuevo(1, "Test", 100.0)
    }

    #[test]
    fn test_precio_final_una_unidad() {
        let p = producto_test();
        let precio = calcular_precio_final(&p, 1);
        assert!((precio - 121.0).abs() < 0.01); // 100 * 1.21
    }

    #[test]
    fn test_precio_final_multiples_unidades() {
        let p = producto_test();
        let precio = calcular_precio_final(&p, 3);
        assert!((precio - 363.0).abs() < 0.01); // 300 * 1.21
    }

    #[test]
    fn test_aplicar_descuento_10() {
        let precio = aplicar_descuento(100.0, 10.0);
        assert!((precio - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_aplicar_descuento_25() {
        let precio = aplicar_descuento(200.0, 25.0);
        assert!((precio - 150.0).abs() < 0.01);
    }

    #[test]
    fn test_calcular_subtotal() {
        let p = producto_test();
        let subtotal = calcular_subtotal(&p, 5);
        assert!((subtotal - 500.0).abs() < 0.01);
    }

    #[test]
    fn test_calcular_iva() {
        let iva = calcular_iva(100.0);
        assert!((iva - 21.0).abs() < 0.01);
    }
}
