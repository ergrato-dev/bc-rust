// practice-03-custom-derive/src/main.rs
//
// Crate consumidor del derive macro `Describe`.
// Una vez que implementes impl_describe en el crate -macro, este archivo
// debe compilar y los tests deben pasar sin modificaciones.

use practice_03_custom_derive_macro::Describe;

/// Trait que el macro genera automáticamente.
/// Ya está definido aquí — el macro solo genera la impl.
pub trait Describe {
    fn describe(&self) -> String;
}

// ── Struct con campos nombrados ──────────────────────────────
#[derive(Describe, Debug)]
struct Usuario {
    nombre: String,
    edad: u32,
    activo: bool,
}

// ── Struct con campos posicionales (tuple struct) ────────────
#[derive(Describe, Debug)]
struct Punto(f64, f64);

// ── Struct unit (sin campos) ─────────────────────────────────
#[derive(Describe, Debug)]
struct Marcador;

// ── Struct con un solo campo ─────────────────────────────────
#[derive(Describe, Debug)]
struct Wrapper {
    valor: i64,
}

fn main() {
    let u = Usuario {
        nombre: String::from("Ana"),
        edad: 30,
        activo: true,
    };
    println!("{}", u.describe());
    // Esperado: Usuario { nombre: "Ana", edad: 30, activo: true }

    let p = Punto(3.14, 2.71);
    println!("{}", p.describe());
    // Esperado: Punto { 0: 3.14, 1: 2.71 }

    let m = Marcador;
    println!("{}", m.describe());
    // Esperado: Marcador

    let w = Wrapper { valor: -42 };
    println!("{}", w.describe());
    // Esperado: Wrapper { valor: -42 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_campos_nombrados() {
        let u = Usuario {
            nombre: String::from("Carlos"),
            edad: 25,
            activo: false,
        };
        let desc = u.describe();
        assert!(desc.contains("Usuario"), "debe contener el nombre del tipo");
        assert!(desc.contains("nombre"), "debe contener el campo 'nombre'");
        assert!(desc.contains("Carlos"), "debe contener el valor del campo");
        assert!(desc.contains("edad"), "debe contener el campo 'edad'");
        assert!(desc.contains("25"), "debe contener el valor numérico");
    }

    #[test]
    fn test_describe_campos_posicionales() {
        let p = Punto(1.0, 2.0);
        let desc = p.describe();
        assert!(desc.contains("Punto"), "debe contener el nombre del tipo");
        assert!(desc.contains('0'), "debe contener índice 0");
        assert!(desc.contains('1'), "debe contener índice 1");
    }

    #[test]
    fn test_describe_unit() {
        let m = Marcador;
        let desc = m.describe();
        assert_eq!(desc, "Marcador");
    }

    #[test]
    fn test_describe_wrapper() {
        let w = Wrapper { valor: 99 };
        let desc = w.describe();
        assert!(desc.contains("Wrapper"));
        assert!(desc.contains("99"));
    }

    #[test]
    fn test_describe_no_modifica_debug() {
        // derive(Describe) no debe interferir con derive(Debug)
        let u = Usuario {
            nombre: String::from("Test"),
            edad: 1,
            activo: true,
        };
        let debug = format!("{:?}", u);
        assert!(debug.contains("Usuario"));
    }
}
