// project-macro-toolkit/src/lib.rs
//
// Librería principal del macro toolkit.
// Re-exporta las macros del crate -derive y define macros declarativas.

// ── Re-exportar proc-macros ──────────────────────────────────────────────────
pub use project_macro_toolkit_derive::{builder, log_call, Describe};

// ── Trait requerido por Describe ─────────────────────────────────────────────

/// Trait para describir una instancia en forma de cadena legible.
pub trait Describe {
    /// Retorna una representación textual del valor con sus campos.
    fn describe(&self) -> String;
}

// ── Macros declarativas ──────────────────────────────────────────────────────

/// Crea un `HashMap` a partir de pares clave-valor.
///
/// ```
/// use project_macro_toolkit::map;
/// let m = map!["uno" => 1, "dos" => 2];
/// assert_eq!(m["uno"], 1);
/// ```
#[macro_export]
macro_rules! map {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($k:expr => $v:expr),+ $(,)?) => {{
        let mut m = ::std::collections::HashMap::new();
        $(m.insert($k, $v);)+
        m
    }};
}

/// Aserta que un valor coincide con un patrón.
///
/// ```
/// use project_macro_toolkit::assert_matches;
/// assert_matches!(Some(42), Some(_));
/// ```
#[macro_export]
macro_rules! assert_matches {
    ($expr:expr, $patron:pat) => {
        match $expr {
            $patron => {}
            ref valor => panic!(
                "assert_matches! falló: {:?} no coincide con {}",
                valor,
                stringify!($patron)
            ),
        }
    };
    ($expr:expr, $patron:pat if $guarda:expr) => {
        match $expr {
            $patron if $guarda => {}
            ref valor => panic!(
                "assert_matches! falló: {:?} no satisface la guarda",
                valor
            ),
        }
    };
}
