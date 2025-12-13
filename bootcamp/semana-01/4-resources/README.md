# 📚 Recursos - Semana 01

## 🔗 Enlaces Oficiales

### Documentación

| Recurso | Descripción |
|---------|-------------|
| [The Rust Book](https://doc.rust-lang.org/book/) | El libro oficial de Rust |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | Aprende Rust con ejemplos |
| [Rust Standard Library](https://doc.rust-lang.org/std/) | Documentación de la biblioteca estándar |
| [Cargo Book](https://doc.rust-lang.org/cargo/) | Documentación de Cargo |

### Herramientas

| Herramienta | Descripción |
|-------------|-------------|
| [Rust Playground](https://play.rust-lang.org/) | Ejecuta Rust en el navegador |
| [crates.io](https://crates.io/) | Registro de paquetes Rust |
| [docs.rs](https://docs.rs/) | Documentación de crates |

---

## 📖 Lecturas Recomendadas

### Capítulos del Rust Book (Semana 01)

1. **[Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)**
   - 1.1 Installation
   - 1.2 Hello, World!
   - 1.3 Hello, Cargo!

2. **[Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)** (preview)
   - Solo las primeras secciones

### Artículos

- [Why Rust?](https://www.rust-lang.org/learn) - Página oficial
- [Rust vs C++: A Performance Comparison](https://blog.rust-lang.org/)
- [Companies Using Rust](https://www.rust-lang.org/production)

---

## 🎥 Videos Recomendados

### Español

| Video | Canal | Duración |
|-------|-------|----------|
| Introducción a Rust | - | ~20 min |
| Cargo Basics | - | ~15 min |

### Inglés

| Video | Canal | Duración |
|-------|-------|----------|
| [Rust in 100 Seconds](https://www.youtube.com/watch?v=5C_HPTJg5ek) | Fireship | 2 min |
| [Introduction to Rust](https://www.youtube.com/watch?v=OX9HJsJUDxA) | Microsoft | 30 min |

---

## 🛠️ Cheat Sheets

### Comandos Cargo

```bash
# Crear proyecto
cargo new nombre-proyecto

# Compilar
cargo build              # Debug
cargo build --release    # Release

# Ejecutar
cargo run

# Testing
cargo test

# Verificar
cargo check              # Rápido, sin generar binario

# Calidad
cargo fmt                # Formatear código
cargo clippy             # Linting

# Documentación
cargo doc --open         # Generar y abrir docs
```

### Anatomía de `main.rs`

```rust
// Comentario de una línea

/// Documentación (rustdoc)

fn main() {
    // Punto de entrada
    println!("Hello!");      // Macro de impresión
    let x = 5;               // Variable inmutable
    let mut y = 10;          // Variable mutable
}
```

---

## 🔧 Configuración VS Code

### Extensiones Esenciales

1. **rust-analyzer** - Soporte de lenguaje
2. **Even Better TOML** - Soporte para Cargo.toml
3. **Error Lens** - Errores inline
4. **CodeLLDB** - Debugger

### Settings Recomendados

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

---

## 🔗 Comunidad

### Discord/Chat

- [Official Rust Discord](https://discord.gg/rust-lang)
- [Rust Community Discord](https://discord.gg/rust-lang-community)

### Reddit

- [r/rust](https://www.reddit.com/r/rust/)
- [r/learnrust](https://www.reddit.com/r/learnrust/)

### Foros

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Internals](https://internals.rust-lang.org/)

---

## 📝 Ejercicios Extra

### Rustlings

[Rustlings](https://github.com/rust-lang/rustlings) es una colección de pequeños ejercicios:

```bash
# Instalar
cargo install rustlings

# Iniciar
rustlings watch
```

### Exercism

[Exercism Rust Track](https://exercism.org/tracks/rust) - Ejercicios con mentores.

---

## 🗓️ Próxima Semana

**Semana 02: Variables y Tipos**

- Variables inmutables vs mutables
- Tipos primitivos (i32, f64, bool, char)
- Shadowing
- Constantes

**Prepárate leyendo:**
- [Chapter 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
