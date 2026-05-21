//! Demostración de primitivos de `core::` disponibles en `no_std`.
#![no_std]

use core::mem;
use core::ptr;

/// Retorna el tamaño en bytes de un tipo en compile-time.
pub const fn tamano_de<T>() -> usize {
    mem::size_of::<T>()
}

/// Retorna la alineación de un tipo en compile-time.
pub const fn alineacion_de<T>() -> usize {
    mem::align_of::<T>()
}

/// Intercambia dos valores en memoria.
pub fn intercambiar<T>(a: &mut T, b: &mut T) {
    mem::swap(a, b);
}

/// Copia bytes de `src` a `dst` con `core::ptr::copy_nonoverlapping`.
///
/// # Safety
/// `dst` debe tener al menos `n` bytes de espacio disponible.
/// `src` y `dst` no deben solaparse.
pub unsafe fn copiar_bytes(src: *const u8, dst: *mut u8, n: usize) {
    // SAFETY: la función es unsafe — el caller garantiza los invariantes.
    unsafe { ptr::copy_nonoverlapping(src, dst, n) };
}

/// Rellena un slice con un valor constante.
pub fn rellenar<T: Copy>(slice: &mut [T], valor: T) {
    for elem in slice.iter_mut() {
        *elem = valor;
    }
}

/// Demuestra tamaños de tipos comunes (verificable en compile-time).
pub const U8_SIZE: usize = tamano_de::<u8>();
pub const U32_SIZE: usize = tamano_de::<u32>();
pub const F64_SIZE: usize = tamano_de::<f64>();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tamanos_correctos() {
        assert_eq!(U8_SIZE, 1);
        assert_eq!(U32_SIZE, 4);
        assert_eq!(F64_SIZE, 8);
    }

    #[test]
    fn intercambiar_funciona() {
        let (mut a, mut b) = (10i32, 20i32);
        intercambiar(&mut a, &mut b);
        assert_eq!(a, 20);
        assert_eq!(b, 10);
    }

    #[test]
    fn rellenar_funciona() {
        let mut datos = [0u8; 8];
        rellenar(&mut datos, 0xFF);
        assert!(datos.iter().all(|&x| x == 0xFF));
    }

    #[test]
    fn copiar_bytes_seguro() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        // SAFETY: dst tiene 4 bytes, src tiene 4 bytes, no se solapan.
        unsafe { copiar_bytes(src.as_ptr(), dst.as_mut_ptr(), 4) };
        assert_eq!(dst, src);
    }
}
