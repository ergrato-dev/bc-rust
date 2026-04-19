# Recursos - Semana 16: Testing y Documentacion

## Documentacion Oficial

- [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

## Crates Utiles

### Testing

| Crate | Descripcion |
|-------|-------------|
| [proptest](https://crates.io/crates/proptest) | Property-based testing |
| [quickcheck](https://crates.io/crates/quickcheck) | QuickCheck para Rust |
| [mockall](https://crates.io/crates/mockall) | Framework de mocking |
| [rstest](https://crates.io/crates/rstest) | Tests parametrizados |
| [test-case](https://crates.io/crates/test-case) | Tests con casos |
| [serial_test](https://crates.io/crates/serial_test) | Tests secuenciales |

### Coverage

| Crate | Descripcion |
|-------|-------------|
| [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) | Cobertura de codigo |
| [grcov](https://github.com/mozilla/grcov) | Mozilla coverage tool |

### Benchmarking

| Crate | Descripcion |
|-------|-------------|
| [criterion](https://crates.io/crates/criterion) | Benchmarks estadisticos |
| [divan](https://crates.io/crates/divan) | Benchmarks rapidos |

## Articulos Recomendados

- [Testing in Rust](https://blog.logrocket.com/testing-rust/)
- [Property-Based Testing](https://fsharpforfunandprofit.com/posts/property-based-testing/)
- [TDD in Rust](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/)

## Videos

- [Let's Get Rusty - Testing](https://www.youtube.com/watch?v=18-7NoNPO30)
- [Jon Gjengset - Testing](https://www.youtube.com/watch?v=8XaVlL3lObQ)

## Patrones de Testing

### Arrange-Act-Assert (AAA)

```rust
#[test]
fn test_patron_aaa() {
    // Arrange
    let mut calc = Calculadora::new();
    
    // Act
    calc.sumar(5);
    
    // Assert
    assert_eq!(calc.valor(), 5);
}
```

### Given-When-Then

```rust
#[test]
fn dado_calculadora_cuando_suma_entonces_incrementa() {
    // Given
    let mut calc = Calculadora::new();
    
    // When
    calc.sumar(10);
    
    // Then
    assert_eq!(calc.valor(), 10);
}
```

## Tips

1. **Nombres descriptivos**: `test_usuario_invalido_retorna_error`
2. **Un assert por test**: Facilita debugging
3. **Tests independientes**: No depender del orden
4. **Tests rapidos**: Evitar I/O cuando sea posible
5. **Documentar con ejemplos**: Los doc tests son documentacion viva
