// Práctica 01 — macro_rules! Básico
//
// Implementa las tres macros marcadas con todo!() siguiendo las instrucciones
// de cada bloque. Los tests al final del archivo validan tu implementación.

// ─────────────────────────────────────────────────────────────
// EJERCICIO 1: macro `saludar!`
//
// Crea una macro que acepte:
//   - saludar!("Mundo")              → imprime "¡Hola, Mundo!"
//   - saludar!("Rust", "desde W18") → imprime "¡Hola, Rust! — desde W18"
// ─────────────────────────────────────────────────────────────
macro_rules! saludar {
    // TODO: rama con un argumento (solo nombre)
    ($nombre:expr) => {
        todo!("implementar rama de un argumento")
    };

    // TODO: rama con dos argumentos (nombre y contexto)
    ($nombre:expr, $contexto:expr) => {
        todo!("implementar rama de dos argumentos")
    };
}

// ─────────────────────────────────────────────────────────────
// EJERCICIO 2: macro `map!`
//
// Crea una macro que construya un HashMap<K, V> a partir de pares K => V.
// Debe aceptar:
//   - map!{}                           → HashMap vacío
//   - map!{ "a" => 1 }                 → un par
//   - map!{ "a" => 1, "b" => 2 }       → varios pares
//   - map!{ "a" => 1, "b" => 2, }      → coma final permitida
// ─────────────────────────────────────────────────────────────
macro_rules! map {
    // TODO: patrón con repetición y coma final opcional
    ($($clave:expr => $valor:expr),* $(,)?) => {
        todo!("implementar cuerpo de map!")
    };
}

// ─────────────────────────────────────────────────────────────
// EJERCICIO 3: macro `assert_matches!`
//
// Crea una macro que verifique si una expresión coincide con un patrón.
// Si no coincide, hace pánico con el mensaje indicado.
//
// Uso:
//   assert_matches!(Some(42), Some(_));
//   assert_matches!(valor, Ok(x) if x > 0, "esperaba Ok positivo, got {:?}", valor);
// ─────────────────────────────────────────────────────────────
macro_rules! assert_matches {
    // Rama básica: solo expresión y patrón
    ($expresion:expr, $patron:pat) => {
        // TODO: verificar que $expresion coincide con $patron
        // Pista: usa un bloque match con _ => panic!(...)
        todo!("implementar rama básica de assert_matches!")
    };

    // Rama con guard: expresión, patrón con guard, y mensaje de error
    ($expresion:expr, $patron:pat if $guard:expr, $($msg:tt)+) => {
        // TODO: verificar con patrón y guard, mensaje personalizado
        todo!("implementar rama con guard y mensaje")
    };
}

fn main() {
    // Prueba manual — ejecutar con `cargo run -p practice-01-macro-rules-basico`
    saludar!("Mundo");
    saludar!("Rust", "desde Semana 18");

    let m = map! {
        "uno" => 1,
        "dos" => 2,
        "tres" => 3,
    };
    println!("map tiene {} entradas", m.len());

    let valor: Option<i32> = Some(42);
    assert_matches!(valor, Some(_));
    println!("¡Todos los ejercicios pasan!");
}

// ─────────────────────────────────────────────────────────────
// TESTS — no modificar
// ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── map! ────────────────────────────────────────────────────
    #[test]
    fn test_map_vacio() {
        let m: std::collections::HashMap<&str, i32> = map! {};
        assert!(m.is_empty());
    }

    #[test]
    fn test_map_un_par() {
        let m = map! { "clave" => 99 };
        assert_eq!(m["clave"], 99);
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn test_map_varios_pares() {
        let m = map! {
            "x" => 10,
            "y" => 20,
            "z" => 30,
        };
        assert_eq!(m["x"], 10);
        assert_eq!(m["y"], 20);
        assert_eq!(m["z"], 30);
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn test_map_coma_final() {
        // La coma final no debe causar error de compilación
        let m = map! { "a" => 1, };
        assert_eq!(m["a"], 1);
    }

    // ── assert_matches! ─────────────────────────────────────────
    #[test]
    fn test_assert_matches_some() {
        let v: Option<i32> = Some(42);
        assert_matches!(v, Some(_));  // debe pasar
    }

    #[test]
    fn test_assert_matches_ok() {
        let r: Result<i32, &str> = Ok(7);
        assert_matches!(r, Ok(_));
    }

    #[test]
    #[should_panic]
    fn test_assert_matches_falla() {
        let v: Option<i32> = None;
        assert_matches!(v, Some(_));  // debe hacer pánico
    }

    #[test]
    fn test_assert_matches_con_guard() {
        let v: Option<i32> = Some(10);
        assert_matches!(v, Some(x) if x > 0, "esperaba Some positivo, got {:?}", v);
    }

    #[test]
    #[should_panic]
    fn test_assert_matches_guard_falla() {
        let v: Option<i32> = Some(-5);
        assert_matches!(v, Some(x) if x > 0, "esperaba positivo");
    }
}
