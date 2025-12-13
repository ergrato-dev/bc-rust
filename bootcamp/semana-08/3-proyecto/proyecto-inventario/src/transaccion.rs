//! Módulo de transacciones de inventario

use std::fmt;

/// Tipo de transacción de inventario
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoTransaccion {
    Entrada,
    Salida,
}

impl fmt::Display for TipoTransaccion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipoTransaccion::Entrada => write!(f, "Entrada"),
            TipoTransaccion::Salida => write!(f, "Salida"),
        }
    }
}

/// Representa una transacción de inventario
#[derive(Debug, Clone)]
pub struct Transaccion {
    pub id: u32,
    pub producto_id: u32,
    pub tipo: TipoTransaccion,
    pub cantidad: u32,
    pub fecha: String,
    pub nota: Option<String>,
}

impl Transaccion {
    /// Crea una nueva transacción
    pub fn new(
        id: u32,
        producto_id: u32,
        tipo: TipoTransaccion,
        cantidad: u32,
        fecha: impl Into<String>,
    ) -> Self {
        Self {
            id,
            producto_id,
            tipo,
            cantidad,
            fecha: fecha.into(),
            nota: None,
        }
    }

    /// Agrega una nota a la transacción
    pub fn con_nota(mut self, nota: impl Into<String>) -> Self {
        self.nota = Some(nota.into());
        self
    }

    /// Crea una transacción de entrada
    pub fn entrada(id: u32, producto_id: u32, cantidad: u32, fecha: impl Into<String>) -> Self {
        Self::new(id, producto_id, TipoTransaccion::Entrada, cantidad, fecha)
    }

    /// Crea una transacción de salida
    pub fn salida(id: u32, producto_id: u32, cantidad: u32, fecha: impl Into<String>) -> Self {
        Self::new(id, producto_id, TipoTransaccion::Salida, cantidad, fecha)
    }
}

impl fmt::Display for Transaccion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let simbolo = match self.tipo {
            TipoTransaccion::Entrada => "+",
            TipoTransaccion::Salida => "-",
        };
        write!(
            f,
            "[{}] {} Producto #{}: {}{} unidades",
            self.fecha, self.tipo, self.producto_id, simbolo, self.cantidad
        )?;
        if let Some(nota) = &self.nota {
            write!(f, " ({})", nota)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nueva_transaccion() {
        let t = Transaccion::new(1, 100, TipoTransaccion::Entrada, 50, "2025-01-15");
        assert_eq!(t.id, 1);
        assert_eq!(t.producto_id, 100);
        assert_eq!(t.cantidad, 50);
        assert!(t.nota.is_none());
    }

    #[test]
    fn test_transaccion_con_nota() {
        let t = Transaccion::entrada(1, 100, 50, "2025-01-15")
            .con_nota("Reposición mensual");
        assert_eq!(t.nota, Some("Reposición mensual".to_string()));
    }

    #[test]
    fn test_entrada_y_salida() {
        let entrada = Transaccion::entrada(1, 100, 50, "2025-01-15");
        let salida = Transaccion::salida(2, 100, 10, "2025-01-16");
        
        assert_eq!(entrada.tipo, TipoTransaccion::Entrada);
        assert_eq!(salida.tipo, TipoTransaccion::Salida);
    }
}
