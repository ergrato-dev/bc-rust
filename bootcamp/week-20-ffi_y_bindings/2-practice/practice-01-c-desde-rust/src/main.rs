use std::ffi::CString;

extern "C" {
    fn strlen(s: *const libc::c_char) -> libc::size_t;
    fn abs(n: libc::c_int) -> libc::c_int;
}

/// Wrapper seguro sobre `strlen` de C.
fn strlen_safe(s: &str) -> usize {
    let c_str = CString::new(s).expect("string contains null byte");
    // SAFETY: `c_str.as_ptr()` apunta a una cadena C válida (terminada en null)
    // que vive mientras `c_str` esté en scope. `strlen` solo lee memoria válida.
    unsafe { strlen(c_str.as_ptr()) }
}

/// Wrapper seguro sobre `abs` de C.
fn abs_safe(n: i32) -> i32 {
    // SAFETY: `abs` es una función C estándar sin efectos secundarios ni UB para i32.
    unsafe { abs(n) }
}

fn main() {
    println!("strlen(\"hola\") = {}", strlen_safe("hola"));
    println!("abs(-42) = {}", abs_safe(-42));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_vacio() {
        assert_eq!(strlen_safe(""), 0);
    }

    #[test]
    fn strlen_ascii() {
        assert_eq!(strlen_safe("hello"), 5);
    }

    #[test]
    fn abs_positivo() {
        assert_eq!(abs_safe(10), 10);
    }

    #[test]
    fn abs_negativo() {
        assert_eq!(abs_safe(-10), 10);
    }
}
