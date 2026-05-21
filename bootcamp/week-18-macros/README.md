# 🦀 Semana 18: Macros — Declarativas y Proc-Macro

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 18 de 25 |
| **Tema** | Macros: `macro_rules!`, Proc-Macros, `#[derive]` custom |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Diseño de Librerías |
| **Requisitos** | Semana 17 (API REST), conocimiento sólido de traits y generics |

---

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Comprender** qué son las macros y cuándo usarlas
2. **Crear** macros declarativas con `macro_rules!` y patrones de matching
3. **Implementar** macros procedurales (`proc-macro`) con `syn` y `quote`
4. **Escribir** un `#[derive]` macro personalizado
5. **Aplicar** macros de atributo para transformar código
6. **Organizar** un workspace con crates de proc-macro separados

---

## 📚 Contenido Teórico

### El Sistema de Macros de Rust

```
┌─────────────────────────────────────────────────────────────┐
│                  TIPOS DE MACROS EN RUST                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  macro_rules! (Declarativas)      Proc-Macros               │
│  ┌─────────────────────────┐     ┌───────────────────────┐  │
│  │  Basadas en patrones    │     │  Custom Derive        │  │
│  │  macro_rules! vec { }   │     │  #[derive(MiTrait)]   │  │
│  │                         │     ├───────────────────────┤  │
│  │  Hygiene automática     │     │  Attribute Macros     │  │
│  │  Rápidas de escribir    │     │  #[mi_atributo]       │  │
│  └─────────────────────────┘     ├───────────────────────┤  │
│                                  │  Function-like        │  │
│                                  │  sql!("SELECT ...")   │  │
│                                  └───────────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Comparación: macro_rules! vs proc-macro

| Característica | `macro_rules!` | Proc-Macro |
|----------------|----------------|------------|
| Complejidad | Baja-Media | Alta |
| Flexibilidad | Limitada | Total |
| Velocidad compilación | Rápida | Más lenta |
| Debugging | Difícil | Más fácil con cargo-expand |
| Dependencias | Ninguna | `syn`, `quote`, `proc-macro2` |
| Crate separado | No | Sí (obligatorio) |

---

## 📖 Material de Estudio

### Teoría

| # | Tema | Archivo | Diagrama |
|---|------|---------|----------|
| 1 | Introducción a las Macros | [01-intro-macros.md](1-theory/01-intro-macros.md) | [SVG](0-assets/01-intro-macros.svg) |
| 2 | `macro_rules!` Declarativas | [02-macro-rules.md](1-theory/02-macro-rules.md) | [SVG](0-assets/02-macro-rules.svg) |
| 3 | Proc-Macros: Introducción | [03-proc-macros-intro.md](1-theory/03-proc-macros-intro.md) | [SVG](0-assets/03-proc-macros-intro.svg) |
| 4 | Custom Derive con syn/quote | [04-derive-macros.md](1-theory/04-derive-macros.md) | [SVG](0-assets/04-derive-macros.svg) |
| 5 | Attribute y Function-like Macros | [05-macros-avanzadas.md](1-theory/05-macros-avanzadas.md) | [SVG](0-assets/05-macros-avanzadas.svg) |

### Prácticas

| # | Práctica | Descripción | Tiempo |
|---|----------|-------------|--------|
| 1 | [practice-01-macro-rules-basico](2-practice/practice-01-macro-rules-basico/) | `macro_rules!` básico: `map!`, `assert_matches!` | 30 min |
| 2 | [practice-02-macro-rules-avanzado](2-practice/practice-02-macro-rules-avanzado/) | Patrones avanzados, repetición, recursión | 30 min |
| 3 | [practice-03-custom-derive](2-practice/practice-03-custom-derive/) | `#[derive(Describe)]` con `syn` + `quote` | 45 min |
| 4 | [practice-04-attribute-macro](2-practice/practice-04-attribute-macro/) | `#[log_call]` attribute macro | 45 min |
| P | [project-macro-toolkit](2-practice/project-macro-toolkit/) | Librería de macros utilitarias | 90 min |

---

## 🛠️ Herramientas de la Semana

### cargo-expand

Visualiza la expansión de macros:

```bash
# Instalar (versión exacta)
cargo install cargo-expand@1.0.95

# Expandir todas las macros del crate
cargo expand

# Expandir solo un módulo
cargo expand mi_modulo

# Expandir en modo release
cargo expand --release
```

### Estructura de Workspace para Proc-Macros

```
mi-crate/                   ← crate consumidor
├── Cargo.toml
└── src/
    └── main.rs

mi-crate-derive/            ← crate proc-macro (separado, obligatorio)
├── Cargo.toml              ← [lib] proc-macro = true
└── src/
    └── lib.rs
```

### Dependencias Necesarias

```toml
# En el crate proc-macro (mi-crate-derive/Cargo.toml)
[lib]
proc-macro = true

[dependencies]
syn       = { version = "2.0.101", features = ["full"] }
quote     = "1.0.40"
proc-macro2 = "1.0.95"
```

---

## 🚀 Cómo Empezar

```bash
# Verificar que las herramientas están disponibles
cargo --version
cargo expand --version  # requiere: cargo install cargo-expand@1.0.95

# Ejecutar práctica 1
cargo run -p practice-01-macro-rules-basico

# Expandir macros de práctica 1
cargo expand -p practice-01-macro-rules-basico

# Ejecutar todos los tests de la semana
cargo test -p practice-01-macro-rules-basico
cargo test -p practice-02-macro-rules-avanzado
cargo test -p practice-03-custom-derive
cargo test -p practice-04-attribute-macro
cargo test -p project-macro-toolkit
```

---

## 📌 Conceptos Clave

### Hygiene en Macros

Las macros declarativas son **higiénicas**: las variables que introducen no colisionan con las del entorno que las llama.

```rust
macro_rules! crear_variable {
    ($valor:expr) => {
        let x = $valor;  // Este `x` no contamina el scope del caller
        x
    };
}

let x = 10;
let y = crear_variable!(42);  // x del macro ≠ x del caller
assert_eq!(x, 10);  // ✅ x no fue modificado
```

### TokenStream

Todos los proc-macros trabajan con `TokenStream`: una secuencia de tokens que representa código Rust.

```rust
// La firma de una macro derive
#[proc_macro_derive(MiDerive)]
pub fn mi_derive(input: TokenStream) -> TokenStream {
    // input: el código del struct/enum donde se aplica
    // retorna: código adicional a generar
    todo!()
}
```

---

## 🔗 Referencias Rápidas

- [The Rust Reference - Macros](https://doc.rust-lang.org/reference/macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [syn crate docs](https://docs.rs/syn/latest/syn/)
- [quote crate docs](https://docs.rs/quote/latest/quote/)
- [proc-macro2 docs](https://docs.rs/proc-macro2/latest/proc_macro2/)
- [Recursos completos](4-resources/RECURSOS.md) | [Glosario](4-resources/GLOSARIO.md)
