//! Uso de `heapless` para estructuras de datos sin allocación dinámica.
#![no_std]

use heapless::{String, Vec};

/// Buffer de temperatura con capacidad máxima de 16 lecturas.
pub struct BufferTemperatura {
    lecturas: Vec<f32, 16>,
}

impl BufferTemperatura {
    /// Crea un buffer vacío.
    pub fn new() -> Self {
        BufferTemperatura {
            lecturas: Vec::new(),
        }
    }

    /// Agrega una lectura. Retorna `Err(())` si el buffer está lleno.
    pub fn agregar(&mut self, temp: f32) -> Result<(), ()> {
        self.lecturas.push(temp).map_err(|_| ())
    }

    /// Calcula el promedio de las lecturas.
    pub fn promedio(&self) -> Option<f32> {
        if self.lecturas.is_empty() {
            return None;
        }
        let suma: f32 = self.lecturas.iter().sum();
        Some(suma / self.lecturas.len() as f32)
    }

    /// Retorna la temperatura mínima.
    pub fn minima(&self) -> Option<f32> {
        self.lecturas
            .iter()
            .copied()
            .reduce(f32::min)
    }

    /// Retorna la temperatura máxima.
    pub fn maxima(&self) -> Option<f32> {
        self.lecturas
            .iter()
            .copied()
            .reduce(f32::max)
    }

    /// Retorna el número de lecturas almacenadas.
    pub fn len(&self) -> usize {
        self.lecturas.len()
    }

    /// Retorna `true` si el buffer está vacío.
    pub fn is_empty(&self) -> bool {
        self.lecturas.is_empty()
    }

    /// Retorna `true` si el buffer está lleno.
    pub fn is_full(&self) -> bool {
        self.lecturas.is_full()
    }
}

impl Default for BufferTemperatura {
    fn default() -> Self {
        Self::new()
    }
}

/// Formatea un número en una cadena `heapless::String<32>`.
pub fn formatear_temp(temp: f32) -> String<32> {
    let mut s: String<32> = String::new();
    // Usamos formato manual simple ya que write! requiere std en algunos contextos.
    // En un sistema real se usaría `ufmt` o `defmt`.
    let entero = temp as i32;
    let decimal = ((temp - entero as f32) * 10.0).abs() as u32;
    let _ = core::fmt::write(
        &mut s,
        format_args!("{entero}.{decimal}C"),
    );
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_promedio() {
        let mut buf = BufferTemperatura::new();
        buf.agregar(20.0).unwrap();
        buf.agregar(22.0).unwrap();
        buf.agregar(24.0).unwrap();
        let prom = buf.promedio().unwrap();
        assert!((prom - 22.0).abs() < 0.001);
    }

    #[test]
    fn buffer_lleno_retorna_err() {
        let mut buf = BufferTemperatura::new();
        for i in 0..16 {
            buf.agregar(i as f32).unwrap();
        }
        assert!(buf.is_full());
        assert!(buf.agregar(99.0).is_err());
    }

    #[test]
    fn min_max_correcto() {
        let mut buf = BufferTemperatura::new();
        for t in [15.0f32, 22.0, 30.0, 10.0, 25.0] {
            buf.agregar(t).unwrap();
        }
        assert_eq!(buf.minima(), Some(10.0));
        assert_eq!(buf.maxima(), Some(30.0));
    }
}
