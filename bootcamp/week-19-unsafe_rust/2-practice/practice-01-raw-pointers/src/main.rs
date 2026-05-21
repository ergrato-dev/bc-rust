fn main() {
    // TODO: Ejercicio 1 — Crear y desreferenciar raw pointers
    //
    // 1. Crear un valor `let x: i32 = 42;`
    // 2. Obtener un raw pointer: `let ptr: *const i32 = &x;`
    // 3. Desreferenciar con un bloque unsafe y comentario SAFETY:
    // 4. Crear un *mut pointer y modificar el valor
    // 5. Verificar que el puntero no es null antes de desreferenciar

    let x: i32 = 42;
    let ptr: *const i32 = &x;

    // SAFETY: `ptr` apunta a `x` que vive en este stack frame.
    // No hay otros owners ni aliasing mutable mientras usamos esta referencia.
    let value = unsafe { *ptr };
    println!("Valor: {value}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn raw_pointer_roundtrip() {
        let x = 99_i32;
        let ptr: *const i32 = &x;
        // SAFETY: `ptr` apunta a `x` que sigue vivo en este scope.
        let got = unsafe { *ptr };
        assert_eq!(got, 99);
    }

    #[test]
    fn null_check() {
        let ptr: *const i32 = std::ptr::null();
        assert!(ptr.is_null());
    }
}
