# Semana 16: Testing y Documentacion

## Objetivos de la Semana

Al finalizar esta semana seras capaz de:

- Escribir tests unitarios efectivos con `#[test]`
- Crear tests de integracion en el directorio `tests/`
- Usar `cargo test` con filtros y opciones avanzadas
- Documentar codigo con rustdoc y `///`
- Generar documentacion HTML con `cargo doc`
- Aplicar Test-Driven Development (TDD)

---

## Contenido

### 1. Teoria

| Archivo | Tema | Duracion |
|---------|------|----------|
| [01-unit-tests.md](1-teoria/01-unit-tests.md) | Tests Unitarios | 30 min |
| [02-integration-tests.md](1-teoria/02-integration-tests.md) | Tests de Integracion | 25 min |
| [03-test-organization.md](1-teoria/03-test-organization.md) | Organizacion de Tests | 25 min |
| [04-rustdoc.md](1-teoria/04-rustdoc.md) | Documentacion con rustdoc | 30 min |
| [05-tdd.md](1-teoria/05-tdd.md) | Test-Driven Development | 30 min |

### 2. Practicas

| Practica | Descripcion | Dificultad |
|----------|-------------|------------|
| [practica-01](2-practica/practica-01-unit-tests/) | Tests unitarios basicos | :star: |
| [practica-02](2-practica/practica-02-integration/) | Tests de integracion | :star::star: |
| [practica-03](2-practica/practica-03-doctests/) | Documentation tests | :star::star: |
| [practica-04](2-practica/practica-04-tdd/) | Test-Driven Development | :star::star::star: |

### 3. Proyecto

| Proyecto | Descripcion | Dificultad |
|----------|-------------|------------|
| [proyecto-calculadora-testeada](2-practica/proyecto-calculadora-testeada/) | Calculadora con cobertura completa | :star::star::star: |

---

## Estructura de Tests en Rust

```
mi_proyecto/
├── src/
│   ├── lib.rs          # Modulo con tests unitarios inline
│   └── main.rs
├── tests/              # Tests de integracion
│   ├── common/
│   │   └── mod.rs      # Codigo compartido
│   └── integration_test.rs
└── examples/           # Ejemplos ejecutables
    └── basic.rs
```

---

## Conceptos Clave

### Tests Unitarios

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma() {
        assert_eq!(suma(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "division por cero")]
    fn test_division_por_cero() {
        dividir(10, 0);
    }

    #[test]
    fn test_resultado() -> Result<(), String> {
        if suma(2, 2) == 4 {
            Ok(())
        } else {
            Err("La suma fallo".into())
        }
    }
}
```

### Documentacion

```rust
/// Suma dos numeros enteros.
///
/// # Argumentos
///
/// * `a` - Primer numero
/// * `b` - Segundo numero
///
/// # Ejemplo
///
/// ```
/// let resultado = mi_crate::suma(2, 3);
/// assert_eq!(resultado, 5);
/// ```
///
/// # Panics
///
/// Esta funcion no produce panic.
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## Comandos Esenciales

```bash
# Ejecutar todos los tests
cargo test

# Test especifico
cargo test nombre_del_test

# Tests en un modulo
cargo test modulo::

# Solo tests de integracion
cargo test --test integration_test

# Ver output de tests exitosos
cargo test -- --show-output

# Ejecutar tests en paralelo (default)
cargo test -- --test-threads=4

# Ejecutar tests secuencialmente
cargo test -- --test-threads=1

# Ignorar tests marcados con #[ignore]
cargo test -- --ignored

# Generar documentacion
cargo doc --open

# Doc tests solamente
cargo test --doc
```

---

## Flujo de la Sesion (4 horas)

| Tiempo | Actividad | Tipo |
|--------|-----------|------|
| 0:00 - 0:45 | Teoria: Tests unitarios y macros assert | Exposicion |
| 0:45 - 1:15 | Demo: Escribiendo tests efectivos | Codigo |
| 1:15 - 1:30 | **Descanso** | Pausa |
| 1:30 - 2:15 | Practica 1-2: Unit tests e integration | Ejercicios |
| 2:15 - 3:00 | Teoria + Demo: rustdoc y TDD | Exposicion |
| 3:00 - 3:45 | Proyecto: Calculadora testeada | Proyecto |
| 3:45 - 4:00 | Revision y cierre | Evaluacion |

---

## Recursos

- [Glosario](3-recursos/GLOSARIO.md)
- [Recursos adicionales](3-recursos/RECURSOS.md)

---

## Evaluacion

Ver [RUBRICA_EVALUACION.md](RUBRICA_EVALUACION.md) para los criterios de evaluacion.
