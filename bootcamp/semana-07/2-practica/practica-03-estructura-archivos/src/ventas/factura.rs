// src/ventas/factura.rs
// Generación de facturas

use super::carrito::Carrito;
use crate::productos::precio;

/// Representa una factura generada
#[derive(Debug)]
pub struct Factura {
    pub numero: u64,
    pub cliente: String,
    pub lineas: Vec<LineaFactura>,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
}

/// Una línea de la factura
#[derive(Debug)]
pub struct LineaFactura {
    pub descripcion: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
    pub importe: f64,
}

impl Factura {
    /// Crea una factura desde un carrito
    pub fn desde_carrito(carrito: &Carrito, cliente: &str) -> Self {
        static mut NUMERO_FACTURA: u64 = 1000;

        let numero = unsafe {
            NUMERO_FACTURA += 1;
            NUMERO_FACTURA
        };

        let lineas: Vec<LineaFactura> = carrito
            .items()
            .iter()
            .map(|item| LineaFactura {
                descripcion: item.producto.nombre.clone(),
                cantidad: item.cantidad,
                precio_unitario: item.producto.precio_base(),
                importe: item.subtotal(),
            })
            .collect();

        let subtotal = carrito.subtotal();
        let iva = precio::calcular_iva(subtotal);
        let total = subtotal + iva;

        Self {
            numero,
            cliente: cliente.to_string(),
            lineas,
            subtotal,
            iva,
            total,
        }
    }

    /// Genera el texto de la factura
    pub fn generar(&self) -> String {
        let mut factura = String::new();

        factura.push_str("╔══════════════════════════════════════════╗\n");
        factura.push_str("║              F A C T U R A               ║\n");
        factura.push_str("╠══════════════════════════════════════════╣\n");
        factura.push_str(&format!("║ Nº: {:>36} ║\n", self.numero));
        factura.push_str(&format!("║ Cliente: {:<31} ║\n", self.cliente));
        factura.push_str("╠══════════════════════════════════════════╣\n");
        factura.push_str("║ Descripción          Cant.  Precio Importe║\n");
        factura.push_str("╠══════════════════════════════════════════╣\n");

        for linea in &self.lineas {
            let desc = if linea.descripcion.len() > 18 {
                format!("{}...", &linea.descripcion[..15])
            } else {
                format!("{:<18}", linea.descripcion)
            };

            factura.push_str(&format!(
                "║ {} {:>4} {:>7.2} {:>7.2}║\n",
                desc, linea.cantidad, linea.precio_unitario, linea.importe
            ));
        }

        factura.push_str("╠══════════════════════════════════════════╣\n");
        factura.push_str(&format!("║ Subtotal:                   {:>12.2}€║\n", self.subtotal));
        factura.push_str(&format!(
            "║ IVA ({:>2}%):                  {:>12.2}€║\n",
            (precio::IVA * 100.0) as u32,
            self.iva
        ));
        factura.push_str("╠══════════════════════════════════════════╣\n");
        factura.push_str(&format!("║ TOTAL:                      {:>12.2}€║\n", self.total));
        factura.push_str("╚══════════════════════════════════════════╝");

        factura
    }

    /// Retorna si la factura tiene contenido
    pub fn tiene_lineas(&self) -> bool {
        !self.lineas.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::productos::Producto;

    fn crear_carrito_test() -> Carrito {
        let mut carrito = Carrito::nuevo();
        carrito.agregar(Producto::nuevo(1, "Producto A", 100.0), 2);
        carrito.agregar(Producto::nuevo(2, "Producto B", 50.0), 1);
        carrito
    }

    #[test]
    fn test_crear_factura() {
        let carrito = crear_carrito_test();
        let factura = Factura::desde_carrito(&carrito, "Test Cliente");

        assert_eq!(factura.cliente, "Test Cliente");
        assert_eq!(factura.lineas.len(), 2);
        assert!(factura.tiene_lineas());
    }

    #[test]
    fn test_factura_calculos() {
        let carrito = crear_carrito_test();
        let factura = Factura::desde_carrito(&carrito, "Test");

        // Subtotal: 100*2 + 50*1 = 250
        assert!((factura.subtotal - 250.0).abs() < 0.01);

        // IVA: 250 * 0.21 = 52.5
        assert!((factura.iva - 52.5).abs() < 0.01);

        // Total: 250 + 52.5 = 302.5
        assert!((factura.total - 302.5).abs() < 0.01);
    }

    #[test]
    fn test_factura_numero_incrementa() {
        let carrito = crear_carrito_test();
        let factura1 = Factura::desde_carrito(&carrito, "Cliente 1");
        let factura2 = Factura::desde_carrito(&carrito, "Cliente 2");

        assert!(factura2.numero > factura1.numero);
    }

    #[test]
    fn test_generar_factura_texto() {
        let carrito = crear_carrito_test();
        let factura = Factura::desde_carrito(&carrito, "Juan");
        let texto = factura.generar();

        assert!(texto.contains("FACTURA"));
        assert!(texto.contains("Juan"));
        assert!(texto.contains("Producto A"));
        assert!(texto.contains("TOTAL"));
    }

    #[test]
    fn test_factura_carrito_vacio() {
        let carrito = Carrito::nuevo();
        let factura = Factura::desde_carrito(&carrito, "Vacio");

        assert!(!factura.tiene_lineas());
        assert!((factura.total - 0.0).abs() < 0.01);
    }
}
