//! Módulo principal de inventario

use std::collections::HashMap;
use crate::producto::Producto;
use crate::transaccion::{Transaccion, TipoTransaccion};

/// Sistema de gestión de inventario
#[derive(Debug)]
pub struct Inventario {
    productos: HashMap<u32, Producto>,
    transacciones: Vec<Transaccion>,
    siguiente_producto_id: u32,
    siguiente_transaccion_id: u32,
}

impl Inventario {
    /// Crea un nuevo inventario vacío
    pub fn new() -> Self {
        Self {
            productos: HashMap::new(),
            transacciones: Vec::new(),
            siguiente_producto_id: 1,
            siguiente_transaccion_id: 1,
        }
    }

    // ========== CRUD de Productos ==========

    /// Agrega un nuevo producto y retorna su ID
    pub fn agregar_producto(
        &mut self,
        nombre: impl Into<String>,
        descripcion: impl Into<String>,
        precio: f64,
        categoria: impl Into<String>,
        stock_inicial: u32,
    ) -> u32 {
        let id = self.siguiente_producto_id;
        self.siguiente_producto_id += 1;

        let producto = Producto::new(id, nombre, descripcion, precio, categoria, stock_inicial);
        self.productos.insert(id, producto);

        // Registrar entrada inicial si hay stock
        if stock_inicial > 0 {
            self.registrar_transaccion(id, TipoTransaccion::Entrada, stock_inicial, "Stock inicial");
        }

        id
    }

    /// Obtiene un producto por ID
    pub fn obtener_producto(&self, id: u32) -> Option<&Producto> {
        self.productos.get(&id)
    }

    /// Obtiene un producto mutable por ID
    pub fn obtener_producto_mut(&mut self, id: u32) -> Option<&mut Producto> {
        self.productos.get_mut(&id)
    }

    /// Actualiza el precio de un producto
    pub fn actualizar_precio(&mut self, id: u32, nuevo_precio: f64) -> bool {
        if let Some(producto) = self.productos.get_mut(&id) {
            producto.precio = nuevo_precio;
            true
        } else {
            false
        }
    }

    /// Elimina un producto
    pub fn eliminar_producto(&mut self, id: u32) -> Option<Producto> {
        self.productos.remove(&id)
    }

    /// Retorna todos los productos
    pub fn listar_productos(&self) -> Vec<&Producto> {
        self.productos.values().collect()
    }

    /// Retorna productos ordenados por nombre
    pub fn productos_ordenados_por_nombre(&self) -> Vec<&Producto> {
        let mut productos: Vec<_> = self.productos.values().collect();
        productos.sort_by(|a, b| a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase()));
        productos
    }

    // ========== Búsquedas ==========

    /// Busca productos por nombre (coincidencia parcial)
    pub fn buscar_por_nombre(&self, texto: &str) -> Vec<&Producto> {
        let texto_lower = texto.to_lowercase();
        self.productos
            .values()
            .filter(|p| p.nombre.to_lowercase().contains(&texto_lower))
            .collect()
    }

    /// Busca productos por categoría exacta
    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Producto> {
        self.productos
            .values()
            .filter(|p| p.categoria == categoria)
            .collect()
    }

    /// Busca productos con stock bajo
    pub fn productos_stock_bajo(&self, umbral: u32) -> Vec<&Producto> {
        self.productos
            .values()
            .filter(|p| p.stock_bajo(umbral))
            .collect()
    }

    /// Busca productos sin stock
    pub fn productos_sin_stock(&self) -> Vec<&Producto> {
        self.productos
            .values()
            .filter(|p| p.stock == 0)
            .collect()
    }

    // ========== Categorías ==========

    /// Obtiene todas las categorías únicas
    pub fn categorias(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.productos
            .values()
            .map(|p| p.categoria.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Cuenta productos por categoría
    pub fn contar_por_categoria(&self) -> HashMap<String, usize> {
        let mut conteo: HashMap<String, usize> = HashMap::new();
        for producto in self.productos.values() {
            *conteo.entry(producto.categoria.clone()).or_insert(0) += 1;
        }
        conteo
    }

    // ========== Transacciones de Stock ==========

    /// Registra una entrada de stock
    pub fn entrada_stock(&mut self, producto_id: u32, cantidad: u32, nota: &str) -> bool {
        if let Some(producto) = self.productos.get_mut(&producto_id) {
            producto.agregar_stock(cantidad);
            self.registrar_transaccion(producto_id, TipoTransaccion::Entrada, cantidad, nota);
            true
        } else {
            false
        }
    }

    /// Registra una salida de stock
    pub fn salida_stock(&mut self, producto_id: u32, cantidad: u32, nota: &str) -> bool {
        if let Some(producto) = self.productos.get_mut(&producto_id) {
            if producto.retirar_stock(cantidad) {
                self.registrar_transaccion(producto_id, TipoTransaccion::Salida, cantidad, nota);
                true
            } else {
                false // Stock insuficiente
            }
        } else {
            false // Producto no encontrado
        }
    }

    /// Registra una transacción interna
    fn registrar_transaccion(
        &mut self,
        producto_id: u32,
        tipo: TipoTransaccion,
        cantidad: u32,
        nota: &str,
    ) {
        let id = self.siguiente_transaccion_id;
        self.siguiente_transaccion_id += 1;

        let fecha = obtener_fecha_actual();
        let mut transaccion = Transaccion::new(id, producto_id, tipo, cantidad, fecha);
        if !nota.is_empty() {
            transaccion = transaccion.con_nota(nota);
        }
        self.transacciones.push(transaccion);
    }

    /// Obtiene el historial de transacciones
    pub fn historial_transacciones(&self) -> &[Transaccion] {
        &self.transacciones
    }

    /// Obtiene transacciones de un producto específico
    pub fn transacciones_producto(&self, producto_id: u32) -> Vec<&Transaccion> {
        self.transacciones
            .iter()
            .filter(|t| t.producto_id == producto_id)
            .collect()
    }

    // ========== Estadísticas ==========

    /// Total de productos
    pub fn total_productos(&self) -> usize {
        self.productos.len()
    }

    /// Total de unidades en stock
    pub fn total_unidades(&self) -> u32 {
        self.productos.values().map(|p| p.stock).sum()
    }

    /// Valor total del inventario
    pub fn valor_total(&self) -> f64 {
        self.productos.values().map(|p| p.valor_inventario()).sum()
    }

    /// Precio promedio de productos
    pub fn precio_promedio(&self) -> f64 {
        if self.productos.is_empty() {
            return 0.0;
        }
        let total: f64 = self.productos.values().map(|p| p.precio).sum();
        total / self.productos.len() as f64
    }

    /// Producto más caro
    pub fn producto_mas_caro(&self) -> Option<&Producto> {
        self.productos
            .values()
            .max_by(|a, b| a.precio.partial_cmp(&b.precio).unwrap())
    }

    /// Producto más barato
    pub fn producto_mas_barato(&self) -> Option<&Producto> {
        self.productos
            .values()
            .min_by(|a, b| a.precio.partial_cmp(&b.precio).unwrap())
    }

    /// Top N productos por valor de inventario
    pub fn top_productos_por_valor(&self, n: usize) -> Vec<&Producto> {
        let mut productos: Vec<_> = self.productos.values().collect();
        productos.sort_by(|a, b| {
            b.valor_inventario()
                .partial_cmp(&a.valor_inventario())
                .unwrap()
        });
        productos.into_iter().take(n).collect()
    }

    /// Valor por categoría
    pub fn valor_por_categoria(&self) -> HashMap<String, f64> {
        let mut valores: HashMap<String, f64> = HashMap::new();
        for producto in self.productos.values() {
            *valores.entry(producto.categoria.clone()).or_insert(0.0) 
                += producto.valor_inventario();
        }
        valores
    }
}

impl Default for Inventario {
    fn default() -> Self {
        Self::new()
    }
}

/// Obtiene la fecha actual como string (simplificado)
fn obtener_fecha_actual() -> String {
    // En un proyecto real usaríamos chrono
    "2025-01-15".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_inventario_prueba() -> Inventario {
        let mut inv = Inventario::new();
        inv.agregar_producto("Laptop", "Portátil 15\"", 999.99, "Electrónica", 10);
        inv.agregar_producto("Mouse", "Mouse inalámbrico", 29.99, "Electrónica", 50);
        inv.agregar_producto("Silla", "Silla ergonómica", 199.99, "Muebles", 5);
        inv
    }

    #[test]
    fn test_agregar_producto() {
        let mut inv = Inventario::new();
        let id = inv.agregar_producto("Test", "Desc", 10.0, "Cat", 5);
        assert_eq!(id, 1);
        assert!(inv.obtener_producto(1).is_some());
    }

    #[test]
    fn test_eliminar_producto() {
        let mut inv = crear_inventario_prueba();
        let eliminado = inv.eliminar_producto(1);
        assert!(eliminado.is_some());
        assert!(inv.obtener_producto(1).is_none());
    }

    #[test]
    fn test_buscar_por_nombre() {
        let inv = crear_inventario_prueba();
        let resultados = inv.buscar_por_nombre("lap");
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].nombre, "Laptop");
    }

    #[test]
    fn test_buscar_por_categoria() {
        let inv = crear_inventario_prueba();
        let resultados = inv.buscar_por_categoria("Electrónica");
        assert_eq!(resultados.len(), 2);
    }

    #[test]
    fn test_categorias() {
        let inv = crear_inventario_prueba();
        let cats = inv.categorias();
        assert_eq!(cats.len(), 2);
        assert!(cats.contains(&"Electrónica".to_string()));
        assert!(cats.contains(&"Muebles".to_string()));
    }

    #[test]
    fn test_entrada_stock() {
        let mut inv = crear_inventario_prueba();
        let stock_antes = inv.obtener_producto(1).unwrap().stock;
        
        assert!(inv.entrada_stock(1, 5, "Reposición"));
        
        let stock_despues = inv.obtener_producto(1).unwrap().stock;
        assert_eq!(stock_despues, stock_antes + 5);
    }

    #[test]
    fn test_salida_stock_exitosa() {
        let mut inv = crear_inventario_prueba();
        let stock_antes = inv.obtener_producto(1).unwrap().stock;
        
        assert!(inv.salida_stock(1, 3, "Venta"));
        
        let stock_despues = inv.obtener_producto(1).unwrap().stock;
        assert_eq!(stock_despues, stock_antes - 3);
    }

    #[test]
    fn test_salida_stock_insuficiente() {
        let mut inv = crear_inventario_prueba();
        let stock_antes = inv.obtener_producto(1).unwrap().stock;
        
        assert!(!inv.salida_stock(1, 100, "Venta grande"));
        
        let stock_despues = inv.obtener_producto(1).unwrap().stock;
        assert_eq!(stock_despues, stock_antes); // No cambió
    }

    #[test]
    fn test_historial_transacciones() {
        let mut inv = Inventario::new();
        inv.agregar_producto("Test", "Desc", 10.0, "Cat", 10);
        inv.entrada_stock(1, 5, "Entrada");
        inv.salida_stock(1, 3, "Salida");
        
        let historial = inv.historial_transacciones();
        // 1 entrada inicial + 1 entrada + 1 salida = 3
        assert_eq!(historial.len(), 3);
    }

    #[test]
    fn test_valor_total() {
        let inv = crear_inventario_prueba();
        // Laptop: 999.99 * 10 = 9999.90
        // Mouse: 29.99 * 50 = 1499.50
        // Silla: 199.99 * 5 = 999.95
        // Total: 12499.35
        let valor = inv.valor_total();
        assert!((valor - 12499.35).abs() < 0.01);
    }

    #[test]
    fn test_producto_mas_caro() {
        let inv = crear_inventario_prueba();
        let mas_caro = inv.producto_mas_caro().unwrap();
        assert_eq!(mas_caro.nombre, "Laptop");
    }

    #[test]
    fn test_top_productos_por_valor() {
        let inv = crear_inventario_prueba();
        let top = inv.top_productos_por_valor(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].nombre, "Laptop"); // Mayor valor: 9999.90
    }

    #[test]
    fn test_productos_stock_bajo() {
        let inv = crear_inventario_prueba();
        let bajo = inv.productos_stock_bajo(10);
        assert_eq!(bajo.len(), 1); // Solo Silla (5 unidades)
    }
}
