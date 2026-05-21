# Recursos — Semana 18: Macros

## 📚 Documentación Oficial

| Recurso | Enlace | Qué encontrarás |
|---------|--------|-----------------|
| The Rust Reference — Macros | https://doc.rust-lang.org/reference/macros.html | Especificación formal de ambos sistemas de macros |
| The Rust Reference — Macros by Example | https://doc.rust-lang.org/reference/macros-by-example.html | Sintaxis completa de `macro_rules!`, designators, hygiene |
| The Rust Reference — Procedural Macros | https://doc.rust-lang.org/reference/procedural-macros.html | API de proc-macros, tipos, restricciones |
| std::macro | https://doc.rust-lang.org/std/#macros | Todas las macros de la librería estándar |

## 📖 Libros y Guías

| Recurso | Enlace | Nivel |
|---------|--------|-------|
| **The Little Book of Rust Macros** | https://veykril.github.io/tlborm/ | Intermedio — la referencia más completa de `macro_rules!` |
| The Rust Programming Language — Ch. 19.5 | https://doc.rust-lang.org/book/ch19-06-macros.html | Introductorio |
| Rust by Example — Macros | https://doc.rust-lang.org/rust-by-example/macros.html | Introductorio con ejemplos ejecutables |

## 🔧 Crates Esenciales

### syn — Parser de Rust

```toml
syn = { version = "2.0.101", features = ["full"] }
```

- Documentación: https://docs.rs/syn/latest/syn/
- Guía de inicio: https://github.com/dtolnay/syn/tree/master/examples
- `DeriveInput`: https://docs.rs/syn/latest/syn/struct.DeriveInput.html

### quote — Generación de TokenStream

```toml
quote = "1.0.40"
```

- Documentación: https://docs.rs/quote/latest/quote/
- Macro `quote!`: https://docs.rs/quote/latest/quote/macro.quote.html
- `format_ident!`: https://docs.rs/quote/latest/quote/macro.format_ident.html

### proc-macro2 — TokenStream portátil

```toml
proc-macro2 = "1.0.95"
```

- Documentación: https://docs.rs/proc-macro2/latest/proc_macro2/
- Cuándo usarlo: https://docs.rs/proc-macro2/latest/proc_macro2/#usage

## 🛠️ Herramientas

### cargo-expand

Expande macros en código Rust legible:

```bash
# Instalar
cargo install cargo-expand@1.0.95

# Usar
cargo expand                         # todo el crate
cargo expand -p nombre-crate         # crate específico en workspace
cargo expand --test nombre_test      # solo tests

# Expandir un módulo concreto
cargo expand path::to::module
```

### cargo-audit (CVE)

```bash
cargo audit --deny warnings
```

## 🎥 Recursos de Aprendizaje Adicionales

### Talleres Recomendados

- **proc-macro-workshop** (David Tolnay): https://github.com/dtolnay/proc-macro-workshop
  — 5 ejercicios graduales: derive, builder, sorted, debug, seq
  — El recurso práctico más valioso para dominar proc-macros

### Artículos y Posts

- "A Practical Introduction to Derive Macros in Rust" — https://developerlife.com/2022/03/30/rust-proc-macro/
- "Writing Procedural Macros in Rust" — https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
- "Rust Macro Hygiene" — https://doc.rust-lang.org/reference/macros-by-example.html#hygiene

### Proyectos de Referencia Reales

| Proyecto | Por qué estudiar |
|----------|-----------------|
| `serde_derive` | Derive macro con generics, attrs opcionales, complejidad real |
| `thiserror` | Derive macro para errores, manejo de spans |
| `tokio::main` | Attribute macro clásico |
| `clap::Parser` | Derive macro con atributos de campo |

## 📌 Cheatsheet Rápido

```rust
// macro_rules! — estructura general
macro_rules! nombre {
    // arm 1: sin repetición
    ($x:expr) => { /* ... */ };
    // arm 2: con repetición
    ($($x:expr),+) => { /* ... */ };
}

// proc-macro — punto de entrada derive
#[proc_macro_derive(MiDerive)]
pub fn mi_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    // ...generar con quote!...
    quote::quote! { /* ... */ }.into()
}

// proc-macro — punto de entrada attribute
#[proc_macro_attribute]
pub fn mi_attr(attr: proc_macro::TokenStream, item: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    // ...
}
```
