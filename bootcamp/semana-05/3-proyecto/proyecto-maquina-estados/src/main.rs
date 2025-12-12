// Proyecto: Máquina de Estados - Sistema de Pedidos
// Semana 05 - Enums y Pattern Matching

// ============================================
// Estados del Pedido
// ============================================

#[derive(Debug, Clone)]
enum EstadoPedido {
    Creado { fecha: String },
    Pagado { fecha: String, monto: f64 },
    Enviado { fecha: String, tracking: String },
    Entregado { fecha: String },
    Cancelado { fecha: String, razon: String },
    Devuelto { fecha: String, razon: String },
}

// ============================================
// Estructura del Pedido
// ============================================

#[derive(Debug, Clone)]
struct Pedido {
    id: u32,
    cliente: String,
    items: Vec<String>,
    estado: EstadoPedido,
}

// Función auxiliar para obtener fecha actual (simulada)
fn obtener_fecha() -> String {
    "2025-01-15".to_string()
}

impl Pedido {
    // TODO: Crear un nuevo pedido en estado Creado
    fn nuevo(id: u32, cliente: &str, items: Vec<String>) -> Self {
        todo!("Crear pedido con estado Creado")
    }

    // TODO: Transición a Pagado (solo desde Creado)
    fn pagar(&mut self, monto: f64) -> Result<(), &'static str> {
        todo!("Implementar transición a Pagado")
    }

    // TODO: Transición a Enviado (solo desde Pagado)
    fn enviar(&mut self, tracking: &str) -> Result<(), &'static str> {
        todo!("Implementar transición a Enviado")
    }

    // TODO: Transición a Entregado (solo desde Enviado)
    fn entregar(&mut self) -> Result<(), &'static str> {
        todo!("Implementar transición a Entregado")
    }

    // TODO: Transición a Cancelado (desde Creado o Pagado)
    fn cancelar(&mut self, razon: &str) -> Result<(), &'static str> {
        todo!("Implementar transición a Cancelado")
    }

    // TODO: Transición a Devuelto (solo desde Entregado)
    fn devolver(&mut self, razon: &str) -> Result<(), &'static str> {
        todo!("Implementar transición a Devuelto")
    }

    // TODO: Verifica si el pedido se puede cancelar
    fn puede_cancelar(&self) -> bool {
        todo!("Usar matches! o match")
    }

    // TODO: Retorna descripción legible del estado
    fn descripcion_estado(&self) -> String {
        todo!("Usar match para cada variante")
    }

    // TODO: Obtiene el monto si está pagado o posterior
    fn obtener_monto(&self) -> Option<f64> {
        todo!("Retornar Some(monto) si aplica, None si no")
    }

    // TODO: Obtiene el tracking si está enviado o entregado
    fn obtener_tracking(&self) -> Option<&str> {
        todo!("Retornar Some(&tracking) si aplica")
    }

    // TODO: Verifica si el pedido está en un estado final
    fn es_estado_final(&self) -> bool {
        todo!("Entregado, Cancelado o Devuelto son finales")
    }
}

// ============================================
// Gestor de Pedidos
// ============================================

#[derive(Debug, Default)]
struct GestorPedidos {
    pedidos: Vec<Pedido>,
    siguiente_id: u32,
}

impl GestorPedidos {
    fn nuevo() -> Self {
        GestorPedidos {
            pedidos: Vec::new(),
            siguiente_id: 1,
        }
    }

    // TODO: Crear y agregar un nuevo pedido
    fn crear_pedido(&mut self, cliente: &str, items: Vec<String>) -> u32 {
        todo!("Crear pedido, agregar a la lista, retornar ID")
    }

    // TODO: Buscar pedido por ID
    fn buscar(&self, id: u32) -> Option<&Pedido> {
        todo!("Usar iter().find()")
    }

    // TODO: Buscar pedido mutable por ID
    fn buscar_mut(&mut self, id: u32) -> Option<&mut Pedido> {
        todo!("Usar iter_mut().find()")
    }

    // TODO: Listar pedidos en un estado específico
    fn listar_por_estado(&self, estado_nombre: &str) -> Vec<&Pedido> {
        todo!("Filtrar usando matches! o match")
        // estado_nombre puede ser: "creado", "pagado", "enviado", etc.
    }

    // TODO: Contar pedidos por estado
    fn estadisticas(&self) -> Estadisticas {
        todo!("Contar cada tipo de estado")
    }

    // TODO: Obtener total de ventas (suma de montos pagados)
    fn total_ventas(&self) -> f64 {
        todo!("Sumar montos de pedidos pagados/enviados/entregados")
    }
}

#[derive(Debug, Default, PartialEq)]
struct Estadisticas {
    creados: u32,
    pagados: u32,
    enviados: u32,
    entregados: u32,
    cancelados: u32,
    devueltos: u32,
}

// ============================================
// Función Principal
// ============================================

fn main() {
    let mut gestor = GestorPedidos::nuevo();

    // Crear algunos pedidos
    let id1 = gestor.crear_pedido("Ana García", vec![
        "Laptop".to_string(),
        "Mouse".to_string(),
    ]);

    let id2 = gestor.crear_pedido("Bob López", vec![
        "Teclado".to_string(),
    ]);

    println!("Pedidos creados: {} y {}", id1, id2);

    // Procesar pedido 1
    if let Some(pedido) = gestor.buscar_mut(id1) {
        println!("\n--- Procesando pedido {} ---", id1);
        
        if let Err(e) = pedido.pagar(1500.0) {
            println!("Error al pagar: {}", e);
        } else {
            println!("Pedido pagado: {}", pedido.descripcion_estado());
        }

        if let Err(e) = pedido.enviar("TRACK123456") {
            println!("Error al enviar: {}", e);
        } else {
            println!("Pedido enviado: {}", pedido.descripcion_estado());
        }
    }

    // Cancelar pedido 2
    if let Some(pedido) = gestor.buscar_mut(id2) {
        println!("\n--- Cancelando pedido {} ---", id2);
        
        if pedido.puede_cancelar() {
            let _ = pedido.cancelar("Cliente cambió de opinión");
            println!("Pedido cancelado: {}", pedido.descripcion_estado());
        }
    }

    // Mostrar estadísticas
    println!("\n--- Estadísticas ---");
    let stats = gestor.estadisticas();
    println!("{:?}", stats);
    println!("Total ventas: ${:.2}", gestor.total_ventas());
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests de Creación ---

    #[test]
    fn test_crear_pedido() {
        let pedido = Pedido::nuevo(1, "Test", vec!["Item".to_string()]);
        assert_eq!(pedido.id, 1);
        assert_eq!(pedido.cliente, "Test");
        assert!(matches!(pedido.estado, EstadoPedido::Creado { .. }));
    }

    // --- Tests de Transiciones Válidas ---

    #[test]
    fn test_transicion_creado_a_pagado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        assert!(pedido.pagar(100.0).is_ok());
        assert!(matches!(pedido.estado, EstadoPedido::Pagado { monto, .. } if monto == 100.0));
    }

    #[test]
    fn test_transicion_pagado_a_enviado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        pedido.pagar(100.0).unwrap();
        assert!(pedido.enviar("TRACK001").is_ok());
        assert!(matches!(pedido.estado, EstadoPedido::Enviado { .. }));
    }

    #[test]
    fn test_transicion_enviado_a_entregado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        pedido.pagar(100.0).unwrap();
        pedido.enviar("TRACK001").unwrap();
        assert!(pedido.entregar().is_ok());
        assert!(matches!(pedido.estado, EstadoPedido::Entregado { .. }));
    }

    #[test]
    fn test_transicion_creado_a_cancelado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        assert!(pedido.cancelar("No quiero").is_ok());
        assert!(matches!(pedido.estado, EstadoPedido::Cancelado { .. }));
    }

    #[test]
    fn test_transicion_entregado_a_devuelto() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        pedido.pagar(100.0).unwrap();
        pedido.enviar("TRACK001").unwrap();
        pedido.entregar().unwrap();
        assert!(pedido.devolver("Defectuoso").is_ok());
        assert!(matches!(pedido.estado, EstadoPedido::Devuelto { .. }));
    }

    // --- Tests de Transiciones Inválidas ---

    #[test]
    fn test_no_puede_pagar_si_ya_pagado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        pedido.pagar(100.0).unwrap();
        assert!(pedido.pagar(200.0).is_err());
    }

    #[test]
    fn test_no_puede_enviar_si_no_pagado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        assert!(pedido.enviar("TRACK001").is_err());
    }

    #[test]
    fn test_no_puede_cancelar_si_enviado() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        pedido.pagar(100.0).unwrap();
        pedido.enviar("TRACK001").unwrap();
        assert!(!pedido.puede_cancelar());
        assert!(pedido.cancelar("Razón").is_err());
    }

    // --- Tests de Consultas ---

    #[test]
    fn test_obtener_monto() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        assert!(pedido.obtener_monto().is_none());
        
        pedido.pagar(150.0).unwrap();
        assert_eq!(pedido.obtener_monto(), Some(150.0));
    }

    #[test]
    fn test_obtener_tracking() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        pedido.pagar(100.0).unwrap();
        assert!(pedido.obtener_tracking().is_none());
        
        pedido.enviar("ABC123").unwrap();
        assert_eq!(pedido.obtener_tracking(), Some("ABC123"));
    }

    #[test]
    fn test_es_estado_final() {
        let mut pedido = Pedido::nuevo(1, "Test", vec![]);
        assert!(!pedido.es_estado_final());
        
        pedido.cancelar("Test").unwrap();
        assert!(pedido.es_estado_final());
    }

    // --- Tests del Gestor ---

    #[test]
    fn test_gestor_crear_pedido() {
        let mut gestor = GestorPedidos::nuevo();
        let id1 = gestor.crear_pedido("Ana", vec!["Item".to_string()]);
        let id2 = gestor.crear_pedido("Bob", vec!["Otro".to_string()]);
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(gestor.pedidos.len(), 2);
    }

    #[test]
    fn test_gestor_buscar() {
        let mut gestor = GestorPedidos::nuevo();
        let id = gestor.crear_pedido("Ana", vec![]);
        
        assert!(gestor.buscar(id).is_some());
        assert!(gestor.buscar(999).is_none());
    }

    #[test]
    fn test_gestor_estadisticas() {
        let mut gestor = GestorPedidos::nuevo();
        gestor.crear_pedido("A", vec![]);
        gestor.crear_pedido("B", vec![]);
        
        if let Some(p) = gestor.buscar_mut(1) {
            p.pagar(100.0).unwrap();
        }
        
        let stats = gestor.estadisticas();
        assert_eq!(stats.creados, 1);
        assert_eq!(stats.pagados, 1);
    }

    #[test]
    fn test_gestor_total_ventas() {
        let mut gestor = GestorPedidos::nuevo();
        gestor.crear_pedido("A", vec![]);
        gestor.crear_pedido("B", vec![]);
        
        if let Some(p) = gestor.buscar_mut(1) {
            p.pagar(100.0).unwrap();
        }
        if let Some(p) = gestor.buscar_mut(2) {
            p.pagar(250.0).unwrap();
        }
        
        assert_eq!(gestor.total_ventas(), 350.0);
    }
}
