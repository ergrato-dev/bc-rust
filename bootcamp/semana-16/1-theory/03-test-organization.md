# Organizacion de Tests

![Organizacion de Tests](../0-assets/03-test-organization.svg)

## Objetivos

- Estructurar tests de forma mantenible
- Usar fixtures y helpers
- Aplicar patrones de testing

---

## Estructura Recomendada

```
mi_proyecto/
+-- Cargo.toml
+-- src/
|   +-- lib.rs
|   +-- calculadora.rs
|   +-- usuario.rs
+-- tests/
|   +-- common/
|   |   +-- mod.rs
|   +-- calculadora_tests.rs
|   +-- usuario_tests.rs
+-- benches/           # Benchmarks (opcional)
    +-- benchmark.rs
```

---

## Modulos de Test por Funcionalidad

```rust
// src/lib.rs

pub mod calculadora;
pub mod usuario;

#[cfg(test)]
mod tests {
    mod calculadora_tests;
    mod usuario_tests;
}
```

```rust
// src/tests/calculadora_tests.rs

use crate::calculadora::*;

#[test]
fn test_suma() {
    assert_eq!(suma(2, 3), 5);
}
```

---

## Helpers de Test

```rust
// tests/common/mod.rs

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup_global() {
    INIT.call_once(|| {
        // Inicializacion una sola vez
        println!("Setup global ejecutado");
    });
}

pub fn crear_usuario_prueba() -> Usuario {
    Usuario {
        id: 1,
        nombre: String::from("Test User"),
        email: String::from("test@example.com"),
    }
}

pub struct Usuario {
    pub id: u32,
    pub nombre: String,
    pub email: String,
}
```

---

## Patron Builder para Tests

```rust
#[cfg(test)]
mod tests {
    struct PedidoBuilder {
        producto: String,
        cantidad: u32,
        precio: f64,
    }

    impl PedidoBuilder {
        fn new() -> Self {
            PedidoBuilder {
                producto: String::from("Default"),
                cantidad: 1,
                precio: 10.0,
            }
        }

        fn producto(mut self, p: &str) -> Self {
            self.producto = p.to_string();
            self
        }

        fn cantidad(mut self, c: u32) -> Self {
            self.cantidad = c;
            self
        }

        fn build(self) -> Pedido {
            Pedido {
                producto: self.producto,
                cantidad: self.cantidad,
                precio: self.precio,
            }
        }
    }

    #[test]
    fn test_pedido_builder() {
        let pedido = PedidoBuilder::new()
            .producto("Laptop")
            .cantidad(2)
            .build();
        
        assert_eq!(pedido.producto, "Laptop");
        assert_eq!(pedido.cantidad, 2);
    }
}
```

---

## Agrupar Tests Relacionados

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod suma {
        use super::*;

        #[test]
        fn positivos() {
            assert_eq!(suma(2, 3), 5);
        }

        #[test]
        fn negativos() {
            assert_eq!(suma(-2, -3), -5);
        }

        #[test]
        fn mixtos() {
            assert_eq!(suma(-2, 3), 1);
        }
    }

    mod resta {
        use super::*;

        #[test]
        fn basica() {
            assert_eq!(resta(5, 3), 2);
        }
    }
}
```

Ejecutar solo un grupo:

```bash
cargo test suma::
```

---

## Tests Parametrizados

Rust no tiene parametrizacion nativa, pero puedes simularla:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn casos_suma() -> Vec<(i32, i32, i32)> {
        vec![
            (2, 3, 5),
            (-1, 1, 0),
            (0, 0, 0),
            (100, 200, 300),
        ]
    }

    #[test]
    fn test_suma_parametrizado() {
        for (a, b, esperado) in casos_suma() {
            assert_eq!(
                suma(a, b),
                esperado,
                "Fallo con a={}, b={}",
                a,
                b
            );
        }
    }
}
```

---

## Macros para Tests

```rust
macro_rules! test_suma {
    ($nombre:ident, $a:expr, $b:expr, $esperado:expr) => {
        #[test]
        fn $nombre() {
            assert_eq!(suma($a, $b), $esperado);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    test_suma!(test_suma_2_3, 2, 3, 5);
    test_suma!(test_suma_0_0, 0, 0, 0);
    test_suma!(test_suma_neg, -5, 5, 0);
}
```

---

## Feature Flags para Tests

```toml
# Cargo.toml
[features]
test-utils = []
```

```rust
#[cfg(feature = "test-utils")]
pub mod test_helpers {
    pub fn crear_mock() -> MockService {
        MockService::new()
    }
}
```

```bash
cargo test --features test-utils
```
