# 🦀 Bootcamp Rust - Instrucciones para GitHub Copilot

## 📋 Información del Proyecto

Este repositorio contiene el **Bootcamp de Rust: Zero to Hero**, un programa de formación intensivo de **25 semanas (100 horas totales)** diseñado para llevar a los estudiantes desde los fundamentos hasta el nivel de **Rust Library/Systems Author**.

- **Duración**: 25 semanas
- **Dedicación**: 4 horas por semana
- **Modalidad**: Presencial / Virtual
- **Entorno**: Docker (contenedor Rust oficial)
- **Nivel**: De principiante a Rust Library/Systems Author

---

## 🐳 Entorno de Desarrollo: Docker

### Configuración del Contenedor

El bootcamp utiliza Docker para garantizar un entorno de desarrollo consistente:

```dockerfile
# Imagen base oficial de Rust
FROM rust:1.92-slim-bookworm

# Herramientas adicionales
RUN rustup component add rustfmt clippy rust-src rust-docs rust-analyzer
RUN cargo install cargo-watch cargo-edit cargo-expand bacon \
    cargo-criterion wasm-pack cbindgen maturin
RUN apt-get update && apt-get install -y python3 python3-pip nodejs npm && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
```

### Comandos Docker Frecuentes

```bash
# Construir imagen del bootcamp
docker build -t bc-rust .

# Ejecutar contenedor interactivo
docker run -it --rm -v $(pwd):/workspace bc-rust

# Ejecutar cargo dentro del contenedor
docker run --rm -v $(pwd):/workspace bc-rust cargo run

# Verificar código con clippy
docker run --rm -v $(pwd):/workspace bc-rust cargo clippy

# Formatear código
docker run --rm -v $(pwd):/workspace bc-rust cargo fmt
```

### VS Code Dev Container

El proyecto incluye configuración para VS Code Dev Containers en `.devcontainer/`.

---

## 📚 Estructura del Bootcamp (17 Semanas)

### Fase 1: Fundamentos (Semanas 1-4)

| Semana | Tema                  | Descripción                                  |
| ------ | --------------------- | -------------------------------------------- |
| **1**  | Setup y Hello World   | Instalación Docker, Cargo, primer programa   |
| **2**  | Variables y Tipos     | Tipos primitivos, mutabilidad, shadowing     |
| **3**  | Ownership y Borrowing | Sistema de propiedad, referencias, préstamos |
| **4**  | Structs y Métodos     | Estructuras, impl blocks, métodos asociados  |

### Fase 2: Control de Flujo y Datos (Semanas 5-8)

| Semana | Tema                     | Descripción                              |
| ------ | ------------------------ | ---------------------------------------- |
| **5**  | Enums y Pattern Matching | Enums, match, if let, while let          |
| **6**  | Error Handling           | Result, Option, operador ?, propagación  |
| **7**  | Módulos y Crates         | Organización de código, visibilidad, pub |
| **8**  | Colecciones              | Vec, String, HashMap, iteradores básicos |

### Fase 3: Abstracción (Semanas 9-11)

| Semana | Tema           | Descripción                                    |
| ------ | -------------- | ---------------------------------------------- |
| **9**  | Traits Básicos | Definición, implementación, traits derivables  |
| **10** | Generics       | Funciones genéricas, structs genéricos, bounds |
| **11** | Lifetimes      | Anotaciones de lifetime, elision rules         |

### Fase 4: Avanzado (Semanas 12-14)

| Semana | Tema                  | Descripción                                |
| ------ | --------------------- | ------------------------------------------ |
| **12** | Closures e Iteradores | Fn, FnMut, FnOnce, iteradores avanzados    |
| **13** | Smart Pointers        | Box, Rc, Arc, RefCell, interior mutability |
| **14** | Concurrencia          | Threads, channels, Mutex, Send/Sync        |

### Fase 5: Integración (Semanas 15-17)

| Semana | Tema            | Descripción                            |
| ------ | --------------- | -------------------------------------- |
| **15** | Async/Await     | Futures, tokio básico, async runtime   |
| **16** | Testing y Docs  | Unit tests, integration tests, rustdoc |
| **17** | API REST        | Axum, endpoints, SQLite, middleware    |

### Fase 6: Diseño de Librerías (Semanas 18-21)

| Semana | Tema                          | Descripción                                                    |
| ------ | ----------------------------- | -------------------------------------------------------------- |
| **18** | Macros: declarativas y proc   | `macro_rules!`, `proc-macro`, `#[derive]` custom               |
| **19** | `unsafe` Rust y raw pointers  | Unsafe blocks, raw ptrs, invariantes, `std::mem`               |
| **20** | FFI y Language Bindings       | C interop, `PyO3` (Python), `napi-rs` (Node.js), `cbindgen`    |
| **21** | API Design + `crates.io`      | Ergonomía de APIs, doctests, semver, publicación de crates      |

### Fase 7: Sistemas y Performance (Semanas 22-24)

| Semana | Tema                         | Descripción                                              |
| ------ | ---------------------------- | -------------------------------------------------------- |
| **22** | WebAssembly                  | `wasm-pack`, `wasm-bindgen`, Wasmtime, Deno              |
| **23** | Benchmarking y Profiling     | `criterion`, flamegraph, perf, SIMD básico               |
| **24** | `no_std` e Intro a Embedded  | Bare metal Rust, `#![no_std]`, HAL, IoT                  |

### Semana 25: Capstone — "Rewrite It In Rust" (RIIR)

| Opción | Proyecto                            | Stack                            |
| ------ | ----------------------------------- | -------------------------------- |
| A      | Parser/lexer expuesto a Python      | Rust + PyO3 + maturin            |
| B      | CLI tool que reemplaza herramienta  | Rust + clap + indicatif          |
| C      | Motor numérico compilado a WASM     | Rust + wasm-pack + TypeScript    |
| D      | Librería criptográfica con API C    | Rust + cbindgen + unsafe         |

---

## 📁 Estructura de Carpetas

```
bc-rust/
├── .github/
│   └── copilot-instructions.md    # Este archivo
├── .devcontainer/
│   ├── devcontainer.json          # Configuración Dev Container
│   └── Dockerfile                 # Imagen Docker del bootcamp
├── assets/
│   └── bootcamp-header.svg        # Imagen del header
├── _docs/
│   ├── GUIA-DOCKER.md             # Guía de Docker
│   ├── GUIA-VSCODE.md             # Guía de VS Code
│   └── tema.md                    # Estructura temática
├── scripts/
│   └── setup.sh                   # Script de configuración
├── bootcamp/
│   ├── semana-01/
│   │   ├── README.md
│   │   ├── RUBRICA_EVALUACION.md
│   │   ├── 0-assets/              # SVGs y recursos visuales
│   │   ├── 1-teoria/              # Material teórico
│   │   ├── 2-practica/            # Ejercicios prácticos
│   │   │   └── practica-01-xxx/
│   │   │       ├── Cargo.toml
│   │   │       ├── src/
│   │   │       │   └── main.rs
│   │   │       └── README.md
│   │   └── 3-recursos/            # Glosario, referencias
│   ├── semana-02/
│   │   └── ...
│   └── semana-17/
│       └── ...
├── Cargo.toml                     # Workspace Cargo
├── Dockerfile                     # Imagen principal
├── docker-compose.yml             # Orquestación
└── README.md
```

---

## 🎯 Convenciones de Código

### Estilo de Código Rust

- Usar `rustfmt` para formateo automático
- Aplicar `clippy` para linting
- Seguir las convenciones oficiales de Rust:
  - `snake_case` para funciones y variables
  - `PascalCase` para tipos y traits
  - `SCREAMING_SNAKE_CASE` para constantes

### 🎨 Estilo de SVGs

Todos los diagramas y recursos visuales SVG deben seguir estas reglas:

| Aspecto              | Especificación                                |
| -------------------- | --------------------------------------------- |
| **Tema**             | Dark mode obligatorio                         |
| **Degradados**       | ❌ No usar gradientes                         |
| **Fuentes**          | Sans-serif únicamente                         |
| **Colores de fondo** | `#1e1e1e` (principal), `#252526` (secundario) |
| **Colores de texto** | `#d4d4d4` (principal), `#808080` (secundario) |
| **Color de acento**  | `#CE422B` (Rust Orange)                       |
| **Bordes**           | `#3c3c3c` o `#454545`                         |

#### Paleta de Colores SVG

```
Fondos:
  - Principal:    #1e1e1e
  - Secundario:   #252526
  - Terciario:    #2d2d2d

Texto:
  - Principal:    #d4d4d4
  - Secundario:   #808080
  - Destacado:    #ffffff

Acentos (Rust):
  - Orange:       #CE422B
  - Dark:         #A72145
  - Light:        #F46623

Código:
  - Keywords:     #569cd6
  - Strings:      #ce9178
  - Functions:    #dcdcaa
  - Types:        #4ec9b0
  - Comments:     #6a9955

Bordes:
  - Normal:       #3c3c3c
  - Hover:        #454545
```

#### Fuentes Recomendadas para SVG

```
Títulos:    "Segoe UI", "Helvetica Neue", Arial, sans-serif
Código:     "Fira Code", "JetBrains Mono", "Consolas", monospace
Texto:      "Segoe UI", "Roboto", "Open Sans", sans-serif
```

#### Ejemplo de SVG Base

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 400">
  <style>
    .title { font-family: 'Segoe UI', sans-serif; font-size: 24px; fill: #d4d4d4; }
    .text { font-family: 'Segoe UI', sans-serif; font-size: 14px; fill: #808080; }
    .code { font-family: 'Fira Code', monospace; font-size: 12px; fill: #d4d4d4; }
    .accent { fill: #CE422B; }
  </style>
  <rect width="100%" height="100%" fill="#1e1e1e"/>
  <!-- Contenido aquí -->
</svg>
```

### Estructura de Ejercicios

Cada ejercicio debe ser un proyecto Cargo independiente:

```
ejercicio-XX-nombre/
├── Cargo.toml
├── src/
│   └── main.rs (o lib.rs)
├── tests/           # (opcional) integration tests
└── README.md        # Instrucciones del ejercicio
```

### Comentarios y Documentación

````rust
/// Documentación de función/struct (rustdoc)
///
/// # Examples
///
/// ```
/// let result = my_function(42);
/// assert_eq!(result, 84);
/// ```
pub fn my_function(x: i32) -> i32 {
    // Comentario de implementación
    x * 2
}
````

---

## 🔧 Comandos Cargo Esenciales

```bash
# Crear nuevo proyecto
cargo new ejercicio-01-hello-world

# Compilar
cargo build

# Compilar en modo release
cargo build --release

# Ejecutar
cargo run

# Ejecutar tests
cargo test

# Verificar errores sin compilar
cargo check

# Linting con clippy
cargo clippy

# Formatear código
cargo fmt

# Documentación
cargo doc --open

# Agregar dependencia (SIEMPRE con versión exacta)
cargo add serde@1.0.219

# Auditoría de CVEs (ejecutar después de cada cambio en dependencias)
cargo audit --deny warnings

# Ver árbol de dependencias
cargo tree
```

---

## 📝 Patrones de Enseñanza

### Progresión de Conceptos

1. **Introducción teórica** (30-45 min)

   - Explicación del concepto con analogías
   - Diagramas visuales (SVG)
   - Comparación con otros lenguajes

2. **Demostración en vivo** (30 min)

   - Código paso a paso
   - Mostrar errores del compilador
   - Explicar mensajes de error

3. **Ejercicios guiados** (60-90 min)

   - Ejercicios progresivos
   - Scaffolding inicial
   - Tests que validan la solución

4. **Práctica individual** (45-60 min)
   - Proyecto del dominio asignado
   - Aplicar conceptos de la semana

### Manejo de Errores del Compilador

El compilador de Rust es muy estricto pero educativo. Enseñar a:

1. **Leer** el mensaje de error completo
2. **Identificar** la línea y el tipo de error
3. **Seguir** las sugerencias del compilador (`help:`)
4. **Entender** el "why" detrás del error

---

## 🎓 Evaluación

### Distribución de Notas

| Tipo             | Peso | Descripción                        |
| ---------------- | ---- | ---------------------------------- |
| **Conocimiento** | 30%  | Cuestionarios, preguntas teóricas  |
| **Desempeño**    | 40%  | Ejercicios en clase, debugging     |
| **Producto**     | 30%  | Código funcional, proyecto semanal |

### Criterios de Código

- ✅ Compila sin warnings (`cargo clippy`)
- ✅ Pasa todos los tests (`cargo test`)
- ✅ Código formateado (`cargo fmt --check`)
- ✅ Documentación básica
- ✅ Manejo correcto de errores (no `unwrap()` en producción)
- ✅ Dependencias con **versión exacta** en `Cargo.toml` (sin `^`, `~`, `*`, `>=`)
- ✅ Auditoría CVE aprobada: `cargo audit --deny warnings` antes de cada commit

---

## 🛠️ Herramientas Recomendadas

### VS Code Extensions

- `rust-analyzer` - Soporte de lenguaje
- `Even Better TOML` - Soporte Cargo.toml
- `Error Lens` - Errores inline
- `crates` - Versiones de dependencias
- `Docker` - Soporte Docker
- `Dev Containers` - Desarrollo en contenedor
- `WebAssembly DWARF Debugging` - Debug de WASM
- `CodeLLDB` - Depurador nativo para unsafe/FFI

### Recursos Online — Fundamentos

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

### Recursos Online — Fases Avanzadas (18-25)

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe Rust en profundidad
- [Rust Reference - Macros](https://doc.rust-lang.org/reference/macros.html)
- [PyO3 User Guide](https://pyo3.rs/) — bindings Python
- [napi-rs docs](https://napi.rs/) — bindings Node.js
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/) — benchmarking
- [The Embedded Rust Book](https://doc.rust-lang.org/stable/embedded-book/)

---

## 📌 Notas para Copilot

### Generación de Código

Cuando generes código Rust para este bootcamp:

1. **Preferir** código idiomático sobre código "estilo C"
2. **Usar** `Result<T, E>` para operaciones que pueden fallar
3. **Evitar** `unwrap()` y `expect()` excepto en ejemplos simples
4. **Documentar** funciones públicas con `///`
5. **Incluir** tests cuando sea apropiado
6. **Formatear** con rustfmt

### Niveles de Complejidad

- **Semanas 1-3**: Código simple, sin genéricos ni lifetimes
- **Semanas 4-7**: Introducir enums, Result, Option
- **Semanas 8-10**: Traits y genéricos básicos
- **Semanas 11-14**: Código avanzado, lifetimes explícitos, concurrencia
- **Semanas 15-17**: Async/await, APIs REST, integración
- **Semanas 18-19**: Macros procedurales, `unsafe` Rust, raw pointers
- **Semanas 20-21**: FFI, bindings Python/Node.js, publicación en `crates.io`
- **Semanas 22-23**: WebAssembly, benchmarking, SIMD
- **Semana 24**: `no_std`, bare metal, embedded
- **Semana 25**: Capstone — proyecto de librería real con bindings

### Reglas para Código de Fases 6-7 (Semanas 18-25)

- **`unsafe`**: siempre documentar invariantes con `// SAFETY:` antes del bloque
- **FFI**: usar `#[repr(C)]` para tipos que crucen la frontera ABI
- **Macros**: preferir `proc-macro` sobre `macro_rules!` para complejidad alta
- **WASM**: no usar `std::thread`, usar `wasm-bindgen-futures` para async
- **`no_std`**: prohibido `println!`, usar `core::` en vez de `std::`
- **Capstone**: el crate debe tener `#![deny(missing_docs)]` y pasar `cargo clippy`

### Ejemplos del Mundo Real

Preferir ejemplos prácticos:

- Sistema de inventario
- Gestión de usuarios
- Procesamiento de archivos
- APIs y CLIs simples

---

## 🐳 Docker Compose

```yaml
version: '3.8'

services:
  rust:
    build: .
    volumes:
      - .:/workspace
    working_dir: /workspace
    command: cargo watch -x run

  test:
    build: .
    volumes:
      - .:/workspace
    working_dir: /workspace
    command: cargo test --workspace
```

---

## �️ Workflow de Creación de Contenido

### Modelo de trabajo: solodev

- Repositorio de **un solo desarrollador**: no se usan ramas ni Pull Requests
- El trabajo va directamente a `main`
- Cada semana completa se cierra con un **commit + push** al finalizar
- No usar `git branch`, `git checkout -b`, ni abrir PRs

### Orden obligatorio de creación por semana

Cada semana se desarrolla **en este orden exacto**, sin saltarse pasos:

| Paso | Artefacto | Descripción |
|------|-----------|-------------|
| 1 | `README.md` | Guía principal de la semana (objetivos, tabla de contenidos, cómo ejecutar) |
| 2 | `RUBRICA_EVALUACION.md` | Criterios de evaluación con pesos y escala |
| 3 | `1-theory/README.md` + archivos adicionales | Teoría completa; **extensión promedio: 180 líneas** por archivo |
| 4 | `0-assets/` | Diagramas SVG de apoyo a la teoría (dark mode) |
| 5 | `2-practice/practice-NN-*/` | Prácticas con scaffolding y tests |
| 6 | `2-practice/project-*/` (o `3-project/`) | Proyecto integrador de la semana |
| 7 | `4-resources/README.md` | Links, referencias y recursos externos |
| 8 | `5-glossary/README.md` | Glosario de términos clave de la semana |
| 9 | `git add . && git commit -m "..." && git push` | Commit y push directo a `main` |

### Convención de mensajes de commit

```bash
# Formato
git commit -m "week-NN: descripción breve del contenido añadido"

# Ejemplos
git commit -m "week-18: macros declarativas y proc-macro con derive custom"
git commit -m "week-19: unsafe rust, raw pointers y SAFETY invariants"
git commit -m "week-25: capstone RIIR - opción A PyO3 scaffolding"
```

### Extensión esperada de archivos de teoría

- **Target**: ~180 líneas por archivo de teoría
- Mínimo: 120 líneas (no dejar teoría incompleta)
- Máximo orientativo: 250 líneas (si se necesita más, dividir en múltiples archivos)
- Dividir en secciones: conceptos, ejemplos comentados, errores comunes, comparación con otros lenguajes

---

## �📅 Cronograma de Desarrollo del Bootcamp

### Prioridad de Creación

**Fase base (ya estructuradas)**
1. ✅ Semana 01 - Setup
2. ✅ Semana 02 - Variables
3. ✅ Semana 03 - Structs
4. ✅ Semanas 04-17

**Fases de expansión**
5. ⬜ Semana 18 - Macros declarativas y procedurales
6. ⬜ Semana 19 - `unsafe` Rust y raw pointers
7. ⬜ Semana 20 - FFI y Language Bindings
8. ⬜ Semana 21 - API Design + `crates.io`
9. ⬜ Semana 22 - WebAssembly
10. ⬜ Semana 23 - Benchmarking y Profiling
11. ⬜ Semana 24 - `no_std` e Intro a Embedded
12. ⬜ Semana 25 - Capstone RIIR

---

**Última actualización**: Mayo 2026  
**Versión**: 2.0  
**Bootcamp**: Rust Zero to Hero → Library/Systems Author
