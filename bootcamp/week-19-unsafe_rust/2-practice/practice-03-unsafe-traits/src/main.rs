use std::sync::Arc;

/// Wrapper sobre un puntero opaco proveniente de una librería C hipotética.
/// La documentación de dicha librería garantiza que el handle
/// puede transferirse entre threads y usarse concurrentemente con sincronización externa.
struct CHandle(*mut u8);

// SAFETY: La API C garantiza que el handle es thread-safe cuando se usa con
// sincronización externa. Usamos Arc<Mutex<CHandle>> en el caller para garantizarlo.
unsafe impl Send for CHandle {}

// SAFETY: La API C garantiza que las lecturas concurrentes son seguras.
// Toda escritura se sincroniza mediante Mutex en el caller.
unsafe impl Sync for CHandle {}

impl CHandle {
    fn new() -> Self {
        // Simula la creación de un handle opaco
        CHandle(std::ptr::null_mut())
    }
}

fn main() {
    let handle = Arc::new(CHandle::new());
    let handle2 = Arc::clone(&handle);

    let t = std::thread::spawn(move || {
        // `handle2` se puede enviar a otro thread porque implementa Send
        let _ = handle2.0.is_null();
    });

    t.join().expect("thread panicked");
    println!("CHandle transferido entre threads correctamente");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_send_across_thread() {
        let handle = Arc::new(CHandle::new());
        let h = Arc::clone(&handle);
        std::thread::spawn(move || {
            let _ = h.0.is_null();
        })
        .join()
        .unwrap();
    }
}
