# Glosario — Semana 21: Diseño de APIs

| Término | Definición |
|---------|-----------|
| **API Guidelines** | Reglas de diseño oficiales de Rust para APIs consistentes y ergonómicas |
| **Semver** | Semantic Versioning: MAJOR.MINOR.PATCH — reglas de compatibilidad |
| **Breaking change** | Cambio que rompe la compatibilidad con código existente → bump de MAJOR |
| **Patrón Builder** | Construcción paso a paso de objetos complejos con métodos encadenables |
| **Typestate** | Patrón que codifica estados en el sistema de tipos para validar en compile-time |
| **Newtype** | Wrapper de un solo campo que da identidad semántica a un tipo primitivo |
| **`thiserror`** | Crate para derivar `std::error::Error` con `#[error(...)]` |
| **`anyhow`** | Crate para manejo de errores en binarios (contexto dinámico) |
| **Doctest** | Ejemplo de código en documentación `///` que `cargo test --doc` ejecuta |
| **`#[deny(missing_docs)]`** | Lint que fuerza documentar toda la API pública |
| **`cargo publish`** | Comando para publicar un crate en crates.io |
| **`--dry-run`** | Flag que simula una operación sin ejecutarla realmente |
| **`impl Into<String>`** | Patrón de API que acepta tanto `&str` como `String` como argumento |
| **Fluent interface** | API donde los métodos retornan `Self` para encadenamiento |
| **CHANGELOG** | Archivo que documenta cambios entre versiones de un proyecto |
| **`#[deprecated]`** | Atributo que marca una función como obsoleta y emite warning al usarla |
