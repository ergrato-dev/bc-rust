//! # capstone-opcion-d: Librería de hashing con API C
//!
//! Implementa algoritmos de hashing (FNV-1a, DJB2, CRC32) expuestos via API C.
//! El header se genera automáticamente con `cbindgen`.
#![deny(missing_docs)]

use libc::{c_uchar, size_t, uint32_t, uint64_t};

/// Resultado de un hash con metadatos.
#[repr(C)]
pub struct HashResult {
    /// Valor del hash de 64 bits.
    pub valor: uint64_t,
    /// Número de bytes procesados.
    pub bytes_procesados: size_t,
}

/// Calcula el hash FNV-1a de 64 bits de un buffer de bytes.
///
/// # Safety
/// `data` debe ser un puntero válido a `len` bytes de memoria.
/// El puntero no debe ser nulo.
///
/// # Returns
/// Hash FNV-1a de 64 bits. Retorna 0 si `data` es nulo o `len` es 0.
#[no_mangle]
pub extern "C" fn fnv1a_64(data: *const c_uchar, len: size_t) -> uint64_t {
    if data.is_null() || len == 0 {
        return 0;
    }
    // SAFETY: caller garantiza puntero válido y longitud correcta.
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    fnv1a_64_rust(slice)
}

/// Calcula el hash DJB2 de 32 bits.
///
/// # Safety
/// `data` debe ser un puntero válido a `len` bytes. No nulo.
#[no_mangle]
pub extern "C" fn djb2(data: *const c_uchar, len: size_t) -> uint32_t {
    if data.is_null() || len == 0 {
        return 5381;
    }
    // SAFETY: caller garantiza puntero válido.
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    djb2_rust(slice)
}

/// Calcula el hash de un buffer retornando un `HashResult` completo.
///
/// # Safety
/// `data` debe ser un puntero válido a `len` bytes. No nulo.
#[no_mangle]
pub extern "C" fn hash_completo(data: *const c_uchar, len: size_t) -> HashResult {
    if data.is_null() || len == 0 {
        return HashResult { valor: 0, bytes_procesados: 0 };
    }
    // SAFETY: caller garantiza puntero válido.
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    HashResult {
        valor: fnv1a_64_rust(slice),
        bytes_procesados: len,
    }
}

/// Libera la memoria de un `HashResult` (no necesario para tipos por valor, incluido por completitud).
#[no_mangle]
pub extern "C" fn hash_result_free(_result: HashResult) {
    // HashResult es Copy, nada que liberar. La función existe para completitud del API.
}

// Implementaciones internas Rust (no expuestas a C)

fn fnv1a_64_rust(data: &[u8]) -> u64 {
    const OFFSET: u64 = 14695981039346656037;
    const PRIME: u64 = 1099511628211;
    data.iter().fold(OFFSET, |hash, &byte| {
        (hash ^ byte as u64).wrapping_mul(PRIME)
    })
}

fn djb2_rust(data: &[u8]) -> u32 {
    data.iter().fold(5381u32, |hash, &byte| {
        hash.wrapping_mul(33).wrapping_add(byte as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a_vacio_retorna_offset() {
        // FNV-1a de cadena vacía es el offset base
        assert_eq!(fnv1a_64_rust(&[]), 14695981039346656037);
    }

    #[test]
    fn fnv1a_puntero_nulo_retorna_cero() {
        assert_eq!(fnv1a_64(std::ptr::null(), 10), 0);
    }

    #[test]
    fn djb2_conocido() {
        // "hello" → valor conocido
        let data = b"hello";
        let h = djb2_rust(data);
        assert_eq!(h, 210714636441u64 as u32);
    }

    #[test]
    fn hash_completo_bytes_procesados() {
        let data = b"test";
        let r = hash_completo(data.as_ptr(), data.len());
        assert_eq!(r.bytes_procesados, 4);
        assert_ne!(r.valor, 0);
    }
}
