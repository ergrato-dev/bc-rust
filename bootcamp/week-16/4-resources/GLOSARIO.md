# Glosario - Semana 16: Testing y Documentacion

## Terminos de Testing

### #[test]
Atributo que marca una funcion como test. Solo se compila en modo test.

### #[cfg(test)]
Compilacion condicional que incluye codigo solo en modo test.

### #[should_panic]
Indica que el test pasa si la funcion produce panic.

### #[ignore]
Marca un test para ser ignorado por defecto. Se ejecuta con `--ignored`.

### assert!
Macro que verifica que una expresion sea true.

### assert_eq!
Macro que verifica que dos valores sean iguales.

### assert_ne!
Macro que verifica que dos valores sean diferentes.

### Unit Test
Test que verifica una unidad pequena de codigo (funcion, metodo).

### Integration Test
Test que verifica la interaccion entre multiples componentes.

### Doc Test
Test embebido en la documentacion que verifica ejemplos de codigo.

### Test Double
Objeto que reemplaza una dependencia real en tests (mock, stub, fake).

### Fixture
Datos de prueba reutilizables para tests.

### Coverage
Porcentaje de codigo ejecutado durante los tests.

## Terminos de Documentacion

### ///
Comentario de documentacion para el item siguiente.

### //!
Comentario de documentacion para el item contenedor (modulo, crate).

### rustdoc
Herramienta que genera documentacion HTML desde comentarios.

### cargo doc
Comando que invoca rustdoc para generar documentacion.

### Doc Comment Sections
Secciones estandar: Arguments, Returns, Example, Errors, Panics.

### Intra-doc Links
Enlaces entre items de documentacion usando [`nombre`].

## Comandos

| Comando | Descripcion |
|---------|-------------|
| `cargo test` | Ejecuta todos los tests |
| `cargo test nombre` | Ejecuta tests que contienen "nombre" |
| `cargo test --lib` | Solo tests unitarios |
| `cargo test --test archivo` | Solo un test de integracion |
| `cargo test --doc` | Solo doc tests |
| `cargo test -- --ignored` | Tests ignorados |
| `cargo test -- --show-output` | Muestra println! |
| `cargo doc` | Genera documentacion |
| `cargo doc --open` | Genera y abre en navegador |
