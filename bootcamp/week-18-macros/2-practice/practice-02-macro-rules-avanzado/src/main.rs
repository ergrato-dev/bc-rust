// Práctica 02 — macro_rules! Avanzado
//
// Tres ejercicios que cubren recursión, generación de impls y hygiene.
// Implementa los bloques marcados con todo!(). No modifiques los tests.

// ─────────────────────────────────────────────────────────────
// EJERCICIO 1: macro recursiva `maximo!`
//
// Calcula el máximo de 2 o más valores comparables.
//   maximo!(3, 1)         → 3
//   maximo!(3, 1, 4, 1, 5, 9) → 9
//
// Pista: caso base (1 arg) y caso recursivo (primer + resto).
// ─────────────────────────────────────────────────────────────

/// Calcula el máximo de dos o más valores.
///
/// # Examples
/// ```
/// # use practice_02_macro_rules_avanzado::maximo;
/// assert_eq!(maximo!(1, 5, 3), 5);
/// ```
#[macro_export]
macro_rules! maximo {
    // Caso base: un solo elemento
    ($x:expr) => {
        todo!("implementar caso base")
    };

    // Caso recursivo: primero + resto (uno o más)
    ($x:expr, $($resto:expr),+) => {
        todo!("implementar caso recursivo")
    };
}

// ─────────────────────────────────────────────────────────────
// EJERCICIO 2: macro generadora de newtype `newtype!`
//
// Genera un newtype struct con conversiones básicas.
// newtype!(Metros, f64) debe generar:
//   struct Metros(f64);
//   impl From<f64> para Metros
//   impl From<Metros> para f64
//   impl Display para Metros
//
// Uso:
//   newtype!(Metros, f64);
//   let m = Metros::from(3.14);
//   println!("{}", m);  // "3.14 Metros"
// ─────────────────────────────────────────────────────────────

/// Genera un newtype con conversiones básicas.
///
/// # Examples
/// ```
/// # use practice_02_macro_rules_avanzado::newtype;
/// newtype!(Kilogramos, f64);
/// let k = Kilogramos::from(70.0);
/// let valor: f64 = f64::from(k);
/// assert!((valor - 70.0).abs() < f64::EPSILON);
/// ```
#[macro_export]
macro_rules! newtype {
    ($nombre:ident, $tipo:ty) => {
        // TODO: generar el struct y los tres impls
        // Recuerda: en quote! no tienes quote, pero en macro_rules!
        // usas stringify!($nombre) para obtener el nombre como &str
        todo!("implementar newtype!")
    };
}

// ─────────────────────────────────────────────────────────────
// EJERCICIO 3: macro `impl_ops!` — implementar operadores
//
// Genera implementaciones de Add, Sub y Mul para un tipo newtype.
//
// impl_ops!(Metros, f64);
// →  impl Add<Metros> for Metros { type Output = Metros; ... }
// →  impl Sub<Metros> for Metros { ... }
// →  impl Mul<f64>    for Metros { ... }
// ─────────────────────────────────────────────────────────────

/// Genera implementaciones de operadores aritméticos para un newtype.
///
/// # Examples
/// ```
/// # use practice_02_macro_rules_avanzado::{newtype, impl_ops};
/// newtype!(Euros, f64);
/// impl_ops!(Euros, f64);
/// let a = Euros::from(10.0);
/// let b = Euros::from(5.0);
/// let c = a + b;
/// let d = c * 2.0;
/// ```
#[macro_export]
macro_rules! impl_ops {
    ($nombre:ident, $tipo:ty) => {
        // TODO: implementar Add<$nombre> for $nombre
        //       implementar Sub<$nombre> for $nombre
        //       implementar Mul<$tipo>   for $nombre
        todo!("implementar impl_ops!")
    };
}

fn main() {
    // Prueba manual
    let max = maximo!(3, 1, 4, 1, 5, 9, 2, 6);
    println!("máximo: {}", max);

    newtype!(Metros, f64);
    impl_ops!(Metros, f64);

    let a = Metros::from(3.0);
    let b = Metros::from(2.0);
    let c = a + b;
    println!("3m + 2m = {}", c);  // "5 Metros"
}

// ─────────────────────────────────────────────────────────────
// TESTS — no modificar
// ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── maximo! ─────────────────────────────────────────────────
    #[test]
    fn test_maximo_dos() {
        assert_eq!(maximo!(3, 7), 7);
        assert_eq!(maximo!(10, 2), 10);
    }

    #[test]
    fn test_maximo_tres() {
        assert_eq!(maximo!(3, 1, 4), 4);
    }

    #[test]
    fn test_maximo_cinco() {
        assert_eq!(maximo!(3, 1, 4, 1, 5), 5);
    }

    #[test]
    fn test_maximo_igual() {
        assert_eq!(maximo!(7, 7), 7);
    }

    // ── newtype! ─────────────────────────────────────────────────
    #[test]
    fn test_newtype_from() {
        newtype!(Gramos, f64);
        let g = Gramos::from(100.0);
        let v: f64 = f64::from(g);
        assert!((v - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_newtype_display() {
        newtype!(Litros, f64);
        let l = Litros::from(2.5);
        assert_eq!(format!("{}", l), "2.5 Litros");
    }

    // ── impl_ops! ────────────────────────────────────────────────
    #[test]
    fn test_impl_ops_add() {
        newtype!(Cms, f64);
        impl_ops!(Cms, f64);
        let a = Cms::from(10.0);
        let b = Cms::from(5.0);
        let c = a + b;
        assert!((f64::from(c) - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_impl_ops_sub() {
        newtype!(Kgs, f64);
        impl_ops!(Kgs, f64);
        let a = Kgs::from(10.0);
        let b = Kgs::from(3.0);
        let c = a - b;
        assert!((f64::from(c) - 7.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_impl_ops_mul() {
        newtype!(Segs, f64);
        impl_ops!(Segs, f64);
        let a = Segs::from(4.0);
        let c = a * 3.0;
        assert!((f64::from(c) - 12.0).abs() < f64::EPSILON);
    }

    // ── Hygiene ─────────────────────────────────────────────────
    #[test]
    fn test_hygiene_maximo() {
        // Las variables internas de la macro no deben contaminar este scope
        let primero = "no_pisado";
        let _ = maximo!(1, 2, 3);
        assert_eq!(primero, "no_pisado");
    }
}
