use std::ffi::CStr;
use std::os::raw::c_char;

/// Punto 2D exportado a C.
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Distancia euclidiana entre dos puntos.
///
/// # Safety (para el caller en C)
/// `a` y `b` deben ser punteros válidos y no nulos.
#[no_mangle]
pub extern "C" fn point_distance(a: *const Point, b: *const Point) -> f64 {
    if a.is_null() || b.is_null() {
        return -1.0;
    }
    // SAFETY: verificamos que los punteros no son null arriba.
    // El caller garantiza que apuntan a `Point` válidos y alineados.
    let (a, b) = unsafe { (&*a, &*b) };
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Crea un `Point` en el heap de Rust. El caller DEBE liberar con `point_free`.
#[no_mangle]
pub extern "C" fn point_new(x: f64, y: f64) -> *mut Point {
    Box::into_raw(Box::new(Point { x, y }))
}

/// Libera un `Point` creado con `point_new`. Llamar exactamente una vez.
///
/// # Safety (para el caller en C)
/// `ptr` debe haber sido creado por `point_new` y no haberse liberado antes.
#[no_mangle]
pub extern "C" fn point_free(ptr: *mut Point) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: `ptr` proviene de `Box::into_raw` en `point_new`.
    // El contrato de la API garantiza que se llama exactamente una vez.
    unsafe { drop(Box::from_raw(ptr)); }
}

/// Retorna el número de caracteres ASCII en la cadena C dada.
///
/// # Safety (para el caller en C)
/// `s` debe apuntar a una cadena C válida terminada en null.
#[no_mangle]
pub unsafe extern "C" fn rust_strlen(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }
    // SAFETY: el caller garantiza que `s` apunta a una cadena terminada en null.
    CStr::from_ptr(s).to_bytes().len()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_basica() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 3.0, y: 4.0 };
        let d = point_distance(&a, &b);
        assert!((d - 5.0).abs() < 1e-10);
    }

    #[test]
    fn distance_null_retorna_menos_uno() {
        let a = Point { x: 0.0, y: 0.0 };
        let d = point_distance(&a, std::ptr::null());
        assert_eq!(d, -1.0);
    }

    #[test]
    fn point_new_free() {
        let p = point_new(1.0, 2.0);
        assert!(!p.is_null());
        point_free(p);
        // No double-free: point_free con null es no-op
        point_free(std::ptr::null_mut());
    }
}
