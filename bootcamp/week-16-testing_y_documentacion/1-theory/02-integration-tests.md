# Tests de Integracion

![Tests de Integracion](../0-assets/02-integration-tests.svg)

## Objetivos

- Crear tests de integracion en `tests/`
- Compartir codigo entre tests
- Diferenciar unit vs integration tests

---

## Directorio tests/

Los tests de integracion van en el directorio `tests/` en la raiz:

```
mi_proyecto/
+-- Cargo.toml
+-- src/
|   +-- lib.rs
+-- tests/
    +-- integration_test.rs
    +-- otro_test.rs
```

Cada archivo en `tests/` es un crate separado.

---

## Primer Test de Integracion

```rust
// tests/integration_test.rs

use mi_proyecto;

#[test]
fn test_integracion_suma() {
    let resultado = mi_proyecto::suma(10, 20);
    assert_eq!(resultado, 30);
}

#[test]
fn test_flujo_completo() {
    let a = mi_proyecto::suma(5, 5);
    let b = mi_proyecto::resta(a, 3);
    assert_eq!(b, 7);
}
```

---

## Compartir Codigo entre Tests

Crea un modulo `common` que no se ejecuta como test:

```
tests/
+-- common/
|   +-- mod.rs
+-- integration_test.rs
+-- otro_test.rs
```

```rust
// tests/common/mod.rs

pub fn setup() -> TestContext {
    TestContext {
        datos: vec![1, 2, 3],
    }
}

pub struct TestContext {
    pub datos: Vec<i32>,
}
```

```rust
// tests/integration_test.rs

mod common;

#[test]
fn test_con_setup() {
    let ctx = common::setup();
    assert_eq!(ctx.datos.len(), 3);
}
```

---

## Ejecutar Tests de Integracion

```bash
# Todos los tests (unit + integration)
cargo test

# Solo integration tests
cargo test --test integration_test

# Todos los integration tests
cargo test --test '*'

# Excluir unit tests
cargo test --test integration_test -- --test-threads=1
```

---

## Unit vs Integration Tests

| Aspecto | Unit Tests | Integration Tests |
|---------|------------|-------------------|
| Ubicacion | `src/` junto al codigo | `tests/` |
| Alcance | Una funcion/modulo | API publica |
| Acceso | Codigo privado | Solo pub |
| Velocidad | Rapidos | Mas lentos |
| Aislamiento | Alto | Bajo |

---

## Ejemplo Completo

```rust
// src/lib.rs

pub struct Calculadora {
    historial: Vec<i32>,
}

impl Calculadora {
    pub fn new() -> Self {
        Calculadora { historial: vec![] }
    }

    pub fn suma(&mut self, a: i32, b: i32) -> i32 {
        let resultado = a + b;
        self.historial.push(resultado);
        resultado
    }

    pub fn historial(&self) -> &[i32] {
        &self.historial
    }
}
```

```rust
// tests/calculadora_test.rs

use mi_proyecto::Calculadora;

#[test]
fn test_calculadora_flujo() {
    let mut calc = Calculadora::new();
    
    calc.suma(1, 2);
    calc.suma(3, 4);
    
    let historial = calc.historial();
    assert_eq!(historial, &[3, 7]);
}
```

---

## Tests con Archivos

```rust
// tests/archivo_test.rs

use std::fs;
use std::path::Path;

#[test]
fn test_procesar_archivo() {
    // Setup: crear archivo temporal
    let path = Path::new("test_data.txt");
    fs::write(path, "contenido de prueba").unwrap();
    
    // Test
    let contenido = fs::read_to_string(path).unwrap();
    assert!(contenido.contains("prueba"));
    
    // Cleanup
    fs::remove_file(path).unwrap();
}
```

---

## Buenas Practicas

1. **Testear la API publica**: No internals
2. **Escenarios reales**: Flujos de usuario
3. **Setup/Teardown**: Limpiar recursos
4. **Nombres descriptivos**: Que describan el escenario
