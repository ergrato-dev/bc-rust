//! Librería FFI de ejemplo: un mini motor de estadísticas.
//!
//! Expone una API C estable para calcular estadísticas básicas
//! sobre un conjunto de datos `f64`.

use std::os::raw::c_char;
use std::ffi::CStr;

/// Códigos de error de la librería.
#[repr(C)]
pub enum FfiError {
    Ok = 0,
    NullPointer = 1,
    EmptyData = 2,
    InvalidUtf8 = 3,
}

/// Conjunto de datos opaco.
pub struct DataSet {
    values: Vec<f64>,
    label: String,
}

/// Crea un nuevo `DataSet` vacío con la etiqueta dada.
/// El caller DEBE liberar con `dataset_free`.
///
/// # Safety (caller)
/// `label` debe apuntar a una cadena C válida terminada en null.
#[no_mangle]
pub unsafe extern "C" fn dataset_new(label: *const c_char) -> *mut DataSet {
    if label.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: verificamos que `label` no es null.
    let label_str = match CStr::from_ptr(label).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(DataSet {
        values: Vec::new(),
        label: label_str,
    }))
}

/// Libera un `DataSet`. Llamar exactamente una vez.
///
/// # Safety (caller)
/// `ptr` debe haber sido creado por `dataset_new` y no haberse liberado antes.
#[no_mangle]
pub extern "C" fn dataset_free(ptr: *mut DataSet) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: `ptr` proviene de `Box::into_raw` en `dataset_new`.
    unsafe { drop(Box::from_raw(ptr)); }
}

/// Agrega un valor al dataset.
///
/// # Safety (caller)
/// `ptr` debe ser un puntero válido retornado por `dataset_new`.
#[no_mangle]
pub extern "C" fn dataset_push(ptr: *mut DataSet, value: f64) -> FfiError {
    if ptr.is_null() {
        return FfiError::NullPointer;
    }
    // SAFETY: verificamos null arriba; el caller garantiza puntero válido.
    unsafe { (*ptr).values.push(value); }
    FfiError::Ok
}

/// Calcula la media aritmética. Retorna `NaN` si el dataset está vacío.
///
/// # Safety (caller)
/// `ptr` debe ser un puntero válido retornado por `dataset_new`.
#[no_mangle]
pub extern "C" fn dataset_mean(ptr: *const DataSet) -> f64 {
    if ptr.is_null() {
        return f64::NAN;
    }
    // SAFETY: verificamos null arriba.
    let ds = unsafe { &*ptr };
    if ds.values.is_empty() {
        return f64::NAN;
    }
    ds.values.iter().sum::<f64>() / ds.values.len() as f64
}

/// Retorna el número de elementos en el dataset.
///
/// # Safety (caller)
/// `ptr` debe ser un puntero válido retornado por `dataset_new`.
#[no_mangle]
pub extern "C" fn dataset_len(ptr: *const DataSet) -> usize {
    if ptr.is_null() {
        return 0;
    }
    // SAFETY: verificamos null arriba.
    unsafe { (*ptr).values.len() }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn make_dataset(label: &str) -> *mut DataSet {
        let c = CString::new(label).unwrap();
        // SAFETY: `c.as_ptr()` es válido mientras `c` está en scope.
        unsafe { dataset_new(c.as_ptr()) }
    }

    #[test]
    fn ciclo_completo() {
        let ds = make_dataset("test");
        assert!(!ds.is_null());
        assert_eq!(dataset_len(ds), 0);
        assert_eq!(dataset_push(ds, 10.0) as i32, FfiError::Ok as i32);
        assert_eq!(dataset_push(ds, 20.0) as i32, FfiError::Ok as i32);
        assert_eq!(dataset_len(ds), 2);
        assert!((dataset_mean(ds) - 15.0).abs() < 1e-10);
        dataset_free(ds);
    }

    #[test]
    fn null_pointer_seguro() {
        assert_eq!(dataset_len(std::ptr::null()), 0);
        assert!(dataset_mean(std::ptr::null()).is_nan());
        dataset_free(std::ptr::null_mut()); // no debe paniquear
    }

    #[test]
    fn dataset_vacio_mean_nan() {
        let ds = make_dataset("vacio");
        assert!(dataset_mean(ds).is_nan());
        dataset_free(ds);
    }
}
