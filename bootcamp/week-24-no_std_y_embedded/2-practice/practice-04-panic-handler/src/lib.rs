//! Demostración de `#[panic_handler]` en contexto `no_std`.
//!
//! En un sistema bare metal real, el panic handler controla qué sucede
//! cuando ocurre un panic: resetear el sistema, encender un LED de error,
//! escribir en UART, entrar en bucle infinito, etc.
#![no_std]

/// Códigos de error para el sistema embebido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    /// Sin error.
    Ok = 0,
    /// Buffer lleno.
    BufferFull = 1,
    /// Argumento inválido.
    InvalidArg = 2,
    /// Timeout.
    Timeout = 3,
    /// Error de hardware.
    HardwareFault = 4,
}

/// Resultado embebido — sin heap, usa `ErrorCode` en lugar de `Box<dyn Error>`.
pub type EmbResult<T> = Result<T, ErrorCode>;

/// Registrador de eventos sin heap (capacidad máxima 8 eventos).
pub struct EventLog {
    eventos: [Option<ErrorCode>; 8],
    count: usize,
}

impl EventLog {
    /// Crea un registro vacío.
    pub const fn new() -> Self {
        EventLog {
            eventos: [None; 8],
            count: 0,
        }
    }

    /// Registra un evento. Retorna `Err(BufferFull)` si está lleno.
    pub fn registrar(&mut self, code: ErrorCode) -> EmbResult<()> {
        if self.count >= 8 {
            return Err(ErrorCode::BufferFull);
        }
        self.eventos[self.count] = Some(code);
        self.count += 1;
        Ok(())
    }

    /// Retorna el número de eventos registrados.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Retorna `true` si no hay eventos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Retorna el último evento registrado.
    pub fn ultimo(&self) -> Option<ErrorCode> {
        if self.count == 0 {
            return None;
        }
        self.eventos[self.count - 1]
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

// El `#[panic_handler]` real solo se define cuando compilamos para
// un target que lo requiere (sin std). En tests usa el de std.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // En un sistema real: resetear, encender LED de error, etc.
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_log_registra() {
        let mut log = EventLog::new();
        assert!(log.registrar(ErrorCode::Ok).is_ok());
        assert!(log.registrar(ErrorCode::Timeout).is_ok());
        assert_eq!(log.len(), 2);
        assert_eq!(log.ultimo(), Some(ErrorCode::Timeout));
    }

    #[test]
    fn event_log_lleno() {
        let mut log = EventLog::new();
        for _ in 0..8 {
            log.registrar(ErrorCode::Ok).unwrap();
        }
        assert_eq!(log.registrar(ErrorCode::HardwareFault), Err(ErrorCode::BufferFull));
    }
}
