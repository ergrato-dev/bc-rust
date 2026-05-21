//! Simulación de firmware embebido con cola de eventos y buffer de sensores.
//!
//! Usa solo `core::` y `heapless` — sin allocación dinámica.
#![no_std]

use heapless::spsc::Queue;
use heapless::Vec;

/// Tipos de evento del sistema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evento {
    BotonPresionado,
    SensorListo(u16),
    Timeout,
    ErrorHardware,
    Reset,
}

/// Cola de eventos — capacidad máxima 16, comunicación entre "ISR" y main loop.
pub struct ColaEventos {
    cola: Queue<Evento, 16>,
}

impl ColaEventos {
    /// Crea una cola de eventos vacía.
    pub fn new() -> Self {
        ColaEventos {
            cola: Queue::new(),
        }
    }

    /// Encola un evento (llamado desde "ISR"). Retorna `Err` si la cola está llena.
    pub fn enviar(&mut self, evento: Evento) -> Result<(), Evento> {
        self.cola.enqueue(evento)
    }

    /// Desencola el próximo evento (llamado desde el main loop).
    pub fn recibir(&mut self) -> Option<Evento> {
        self.cola.dequeue()
    }

    /// Retorna el número de eventos pendientes.
    pub fn pendientes(&self) -> usize {
        self.cola.len()
    }
}

impl Default for ColaEventos {
    fn default() -> Self {
        Self::new()
    }
}

/// Buffer circular de lecturas de un sensor ADC (12 bits, max 32 muestras).
pub struct BufferAdc {
    muestras: Vec<u16, 32>,
}

impl BufferAdc {
    /// Crea un buffer vacío.
    pub fn new() -> Self {
        BufferAdc {
            muestras: Vec::new(),
        }
    }

    /// Agrega una muestra (valor 0–4095). Retorna `Err` si el buffer está lleno.
    pub fn agregar(&mut self, muestra: u16) -> Result<(), ()> {
        self.muestras.push(muestra & 0x0FFF).map_err(|_| ())
    }

    /// Calcula el promedio de las muestras en punto fijo (×100 para 2 decimales).
    pub fn promedio_fp(&self) -> Option<u32> {
        if self.muestras.is_empty() {
            return None;
        }
        let suma: u32 = self.muestras.iter().map(|&x| x as u32).sum();
        Some(suma * 100 / self.muestras.len() as u32)
    }

    /// Retorna el número de muestras almacenadas.
    pub fn len(&self) -> usize {
        self.muestras.len()
    }

    /// Retorna `true` si no hay muestras.
    pub fn is_empty(&self) -> bool {
        self.muestras.is_empty()
    }
}

impl Default for BufferAdc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cola_enviar_recibir() {
        let mut cola = ColaEventos::new();
        cola.enviar(Evento::BotonPresionado).unwrap();
        cola.enviar(Evento::SensorListo(1234)).unwrap();
        assert_eq!(cola.pendientes(), 2);
        assert_eq!(cola.recibir(), Some(Evento::BotonPresionado));
        assert_eq!(cola.recibir(), Some(Evento::SensorListo(1234)));
        assert_eq!(cola.recibir(), None);
    }

    #[test]
    fn cola_llena_retorna_err() {
        let mut cola = ColaEventos::new();
        for _ in 0..16 {
            cola.enviar(Evento::Timeout).unwrap();
        }
        assert!(cola.enviar(Evento::Reset).is_err());
    }

    #[test]
    fn adc_promedio() {
        let mut buf = BufferAdc::new();
        buf.agregar(100).unwrap();
        buf.agregar(200).unwrap();
        buf.agregar(300).unwrap();
        // promedio = 200, × 100 = 20000
        assert_eq!(buf.promedio_fp(), Some(20000));
    }

    #[test]
    fn adc_mascara_12bits() {
        let mut buf = BufferAdc::new();
        buf.agregar(0xFFFF).unwrap(); // se enmasca a 0x0FFF = 4095
        assert_eq!(buf.muestras[0], 4095);
    }
}
