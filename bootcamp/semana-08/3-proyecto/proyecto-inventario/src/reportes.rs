//! Módulo de generación de reportes

use crate::inventario::Inventario;

/// Generador de reportes para el inventario
pub struct GeneradorReportes<'a> {
    inventario: &'a Inventario,
}

impl<'a> GeneradorReportes<'a> {
    /// Crea un nuevo generador de reportes
    pub fn new(inventario: &'a Inventario) -> Self {
        Self { inventario }
    }

    /// Genera un reporte resumen del inventario
    pub fn reporte_resumen(&self) -> String {
        let mut reporte = String::new();

        reporte.push_str("╔══════════════════════════════════════════════════════╗\n");
        reporte.push_str("║         📊 REPORTE DE INVENTARIO                     ║\n");
        reporte.push_str("╠══════════════════════════════════════════════════════╣\n");

        // Estadísticas generales
        reporte.push_str(&format!(
            "║ Total productos:     {:>10}                     ║\n",
            self.inventario.total_productos()
        ));
        reporte.push_str(&format!(
            "║ Total unidades:      {:>10}                     ║\n",
            self.inventario.total_unidades()
        ));
        reporte.push_str(&format!(
            "║ Valor total:         ${:>9.2}                     ║\n",
            self.inventario.valor_total()
        ));
        reporte.push_str(&format!(
            "║ Precio promedio:     ${:>9.2}                     ║\n",
            self.inventario.precio_promedio()
        ));

        reporte.push_str("╚══════════════════════════════════════════════════════╝\n");

        reporte
    }

    /// Genera un reporte de productos por categoría
    pub fn reporte_por_categoria(&self) -> String {
        let mut reporte = String::new();

        reporte.push_str("\n📁 PRODUCTOS POR CATEGORÍA\n");
        reporte.push_str("─".repeat(50).as_str());
        reporte.push('\n');

        for categoria in self.inventario.categorias() {
            let productos = self.inventario.buscar_por_categoria(&categoria);
            let valor: f64 = productos.iter().map(|p| p.valor_inventario()).sum();

            reporte.push_str(&format!("\n🏷️  {} ({} productos, valor: ${:.2})\n",
                categoria, productos.len(), valor));

            for producto in productos {
                reporte.push_str(&format!(
                    "   • {} - ${:.2} (Stock: {})\n",
                    producto.nombre, producto.precio, producto.stock
                ));
            }
        }

        reporte
    }

    /// Genera un reporte de alertas de stock
    pub fn reporte_alertas(&self, umbral_bajo: u32) -> String {
        let mut reporte = String::new();

        reporte.push_str("\n⚠️  ALERTAS DE STOCK\n");
        reporte.push_str("─".repeat(50).as_str());
        reporte.push('\n');

        // Sin stock
        let sin_stock = self.inventario.productos_sin_stock();
        if !sin_stock.is_empty() {
            reporte.push_str("\n🔴 SIN STOCK:\n");
            for producto in sin_stock {
                reporte.push_str(&format!("   • {} [{}]\n", producto.nombre, producto.categoria));
            }
        }

        // Stock bajo
        let stock_bajo: Vec<_> = self.inventario.productos_stock_bajo(umbral_bajo)
            .into_iter()
            .filter(|p| p.stock > 0)
            .collect();

        if !stock_bajo.is_empty() {
            reporte.push_str(&format!("\n🟡 STOCK BAJO (< {} unidades):\n", umbral_bajo));
            for producto in stock_bajo {
                reporte.push_str(&format!(
                    "   • {} - {} unidades [{}]\n",
                    producto.nombre, producto.stock, producto.categoria
                ));
            }
        }

        if sin_stock.is_empty() && self.inventario.productos_stock_bajo(umbral_bajo).is_empty() {
            reporte.push_str("\n✅ No hay alertas de stock\n");
        }

        reporte
    }

    /// Genera un reporte de top productos por valor
    pub fn reporte_top_valor(&self, n: usize) -> String {
        let mut reporte = String::new();

        reporte.push_str(&format!("\n🏆 TOP {} PRODUCTOS POR VALOR\n", n));
        reporte.push_str("─".repeat(50).as_str());
        reporte.push('\n');

        let top = self.inventario.top_productos_por_valor(n);

        for (i, producto) in top.iter().enumerate() {
            reporte.push_str(&format!(
                "   {}. {} - ${:.2} ({}×${:.2})\n",
                i + 1,
                producto.nombre,
                producto.valor_inventario(),
                producto.stock,
                producto.precio
            ));
        }

        reporte
    }

    /// Genera un reporte de historial de transacciones
    pub fn reporte_transacciones(&self) -> String {
        let mut reporte = String::new();

        reporte.push_str("\n📋 HISTORIAL DE TRANSACCIONES\n");
        reporte.push_str("─".repeat(50).as_str());
        reporte.push('\n');

        let transacciones = self.inventario.historial_transacciones();

        if transacciones.is_empty() {
            reporte.push_str("\nNo hay transacciones registradas.\n");
        } else {
            for transaccion in transacciones.iter().rev().take(10) {
                let producto_nombre = self.inventario
                    .obtener_producto(transaccion.producto_id)
                    .map(|p| p.nombre.as_str())
                    .unwrap_or("Producto eliminado");

                reporte.push_str(&format!(
                    "\n   {} | {} {} unidades de {}\n",
                    transaccion.fecha,
                    transaccion.tipo,
                    transaccion.cantidad,
                    producto_nombre
                ));

                if let Some(nota) = &transaccion.nota {
                    reporte.push_str(&format!("      📝 {}\n", nota));
                }
            }

            if transacciones.len() > 10 {
                reporte.push_str(&format!(
                    "\n   ... y {} transacciones más\n",
                    transacciones.len() - 10
                ));
            }
        }

        reporte
    }

    /// Genera un reporte completo
    pub fn reporte_completo(&self) -> String {
        let mut reporte = String::new();

        reporte.push_str(&self.reporte_resumen());
        reporte.push_str(&self.reporte_por_categoria());
        reporte.push_str(&self.reporte_alertas(10));
        reporte.push_str(&self.reporte_top_valor(5));
        reporte.push_str(&self.reporte_transacciones());

        reporte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_inventario_prueba() -> Inventario {
        let mut inv = Inventario::new();
        inv.agregar_producto("Laptop", "Portátil", 999.99, "Electrónica", 10);
        inv.agregar_producto("Mouse", "Inalámbrico", 29.99, "Electrónica", 50);
        inv.agregar_producto("Silla", "Ergonómica", 199.99, "Muebles", 3);
        inv
    }

    #[test]
    fn test_reporte_resumen() {
        let inv = crear_inventario_prueba();
        let gen = GeneradorReportes::new(&inv);
        let reporte = gen.reporte_resumen();
        
        assert!(reporte.contains("REPORTE DE INVENTARIO"));
        assert!(reporte.contains("Total productos"));
    }

    #[test]
    fn test_reporte_por_categoria() {
        let inv = crear_inventario_prueba();
        let gen = GeneradorReportes::new(&inv);
        let reporte = gen.reporte_por_categoria();
        
        assert!(reporte.contains("Electrónica"));
        assert!(reporte.contains("Muebles"));
    }

    #[test]
    fn test_reporte_alertas() {
        let inv = crear_inventario_prueba();
        let gen = GeneradorReportes::new(&inv);
        let reporte = gen.reporte_alertas(5);
        
        assert!(reporte.contains("ALERTAS DE STOCK"));
        assert!(reporte.contains("Silla")); // Stock = 3 < 5
    }
}
