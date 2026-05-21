/// Suma dos slices elemento a elemento usando aritmética de punteros.
///
/// # Safety
/// El caller debe garantizar:
/// - `a` y `b` apuntan a memoria válida y alineada para `f64`
/// - Ambos tienen exactamente `len` elementos inicializados
/// - Los rangos `[a, a+len)` y `[b, b+len)` no se solapan con `out`
/// - `out` apunta a un buffer de al menos `len` elementos
unsafe fn sum_slices(a: *const f64, b: *const f64, out: *mut f64, len: usize) {
    for i in 0..len {
        // SAFETY: el caller garantiza que los punteros son válidos para `len` elementos.
        *out.add(i) = *a.add(i) + *b.add(i);
    }
}

fn main() {
    let a = [1.0_f64, 2.0, 3.0];
    let b = [10.0_f64, 20.0, 30.0];
    let mut out = [0.0_f64; 3];

    // SAFETY: `a`, `b`, `out` son arrays de longitud 3 en el mismo stack frame.
    // Sus rangos no se solapan y están correctamente alineados.
    unsafe {
        sum_slices(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), 3);
    }

    println!("{out:?}"); // [11.0, 22.0, 33.0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_correcta() {
        let a = [1.0_f64, 2.0, 3.0];
        let b = [10.0_f64, 20.0, 30.0];
        let mut out = [0.0_f64; 3];
        // SAFETY: arrays con longitud 3, sin solapamiento.
        unsafe { sum_slices(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), 3); }
        assert_eq!(out, [11.0, 22.0, 33.0]);
    }

    #[test]
    fn suma_cero_elementos() {
        let a: [f64; 0] = [];
        let b: [f64; 0] = [];
        let mut out: [f64; 0] = [];
        // SAFETY: len=0, ningún acceso a memoria se realiza.
        unsafe { sum_slices(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), 0); }
    }
}
