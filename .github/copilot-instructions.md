# 🦀 Bootcamp Rust - Instrucciones para GitHub Copilot

## 📋 Información del Proyecto

Este repositorio contiene el **Bootcamp de Rust: Zero to Hero**, un programa de formación intensivo de **17 semanas (68 horas totales)** diseñado para llevar a los estudiantes desde los fundamentos hasta un nivel avanzado en el lenguaje de programación Rust.

- **Duración**: 17 semanas
- **Dedicación**: 4 horas por semana
- **Modalidad**: Presencial / Virtual
- **Entorno**: Docker (contenedor Rust oficial)
- **Nivel**: De principiante a avanzado

---

## 🐳 Entorno de Desarrollo: Docker

### Configuración del Contenedor

El bootcamp utiliza Docker para garantizar un entorno de desarrollo consistente:

```dockerfile
# Imagen base oficial de Rust
FROM rust:1.92-slim-bookworm

# Herramientas adicionales
RUN rustup component add rustfmt clippy rust-src rust-docs
RUN cargo install cargo-watch cargo-edit cargo-expand bacon

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

### Fase 1: Fundamentos (Semanas 0-3)

| Semana | Tema                  | Descripción                                  |
| ------ | --------------------- | -------------------------------------------- |
| **0**  | Setup y Hello World   | Instalación Docker, Cargo, primer programa   |
| **1**  | Variables y Tipos     | Tipos primitivos, mutabilidad, shadowing     |
| **2**  | Ownership y Borrowing | Sistema de propiedad, referencias, préstamos |
| **3**  | Structs y Métodos     | Estructuras, impl blocks, métodos asociados  |

### Fase 2: Control de Flujo y Datos (Semanas 4-7)

| Semana | Tema                     | Descripción                              |
| ------ | ------------------------ | ---------------------------------------- |
| **4**  | Enums y Pattern Matching | Enums, match, if let, while let          |
| **5**  | Error Handling           | Result, Option, operador ?, propagación  |
| **6**  | Módulos y Crates         | Organización de código, visibilidad, pub |
| **7**  | Colecciones              | Vec, String, HashMap, iteradores básicos |

### Fase 3: Abstracción (Semanas 8-10)

| Semana | Tema           | Descripción                                    |
| ------ | -------------- | ---------------------------------------------- |
| **8**  | Traits Básicos | Definición, implementación, traits derivables  |
| **9**  | Generics       | Funciones genéricas, structs genéricos, bounds |
| **10** | Lifetimes      | Anotaciones de lifetime, elision rules         |

### Fase 4: Avanzado (Semanas 11-13)

| Semana | Tema                  | Descripción                                |
| ------ | --------------------- | ------------------------------------------ |
| **11** | Closures e Iteradores | Fn, FnMut, FnOnce, iteradores avanzados    |
| **12** | Smart Pointers        | Box, Rc, Arc, RefCell, interior mutability |
| **13** | Concurrencia          | Threads, channels, Mutex, Send/Sync        |

### Fase 5: Integración (Semanas 14-16)

| Semana | Tema            | Descripción                            |
| ------ | --------------- | -------------------------------------- |
| **14** | Async/Await     | Futures, tokio básico, async runtime   |
| **15** | Testing y Docs  | Unit tests, integration tests, rustdoc |
| **16** | API REST        | Axum, endpoints, SQLite, middleware    |

### Fase 6: Proyecto Final (Semana 17)

| Semana | Tema           | Descripción                        |
| ------ | -------------- | ---------------------------------- |
| **17** | Proyecto Final | Aplicación CLI o API REST completa |

---

## 📁 Estructura de Carpetas

```
bc-rust/
├── .github/
│   └── copilot-instructions.md    # Este archivo
├── .devcontainer/
│   ├── devcontainer.json          # Configuración Dev Container
│   └── Dockerfile                 # Imagen Docker del bootcamp
├── _assets/
│   └── bootcamp-header.svg        # Imagen del header
├── _docs/
│   ├── GUIA-DOCKER.md             # Guía de Docker
│   ├── GUIA-VSCODE.md             # Guía de VS Code
│   └── tema.md                    # Estructura temática
├── _scripts/
│   └── setup.sh                   # Script de configuración
├── bootcamp/
│   ├── BOOTCAMP-COMPLETO.md       # Resumen del bootcamp
│   ├── semana-00/
│   │   ├── README.md
│   │   ├── RUBRICA_EVALUACION.md
│   │   ├── 0-assets/              # SVGs y recursos visuales
│   │   ├── 1-teoria/              # Material teórico
│   │   ├── 2-practica/            # Ejercicios prácticos
│   │   │   └── ejercicio-01-hello-world/
│   │   │       ├── Cargo.toml
│   │   │       ├── src/
│   │   │       │   └── main.rs
│   │   │       └── README.md
│   │   └── 3-recursos/            # Glosario, referencias
│   ├── semana-01/
│   │   └── ...
│   └── semana-15/
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

# Agregar dependencia
cargo add serde
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

---

## 🛠️ Herramientas Recomendadas

### VS Code Extensions

- `rust-analyzer` - Soporte de lenguaje
- `Even Better TOML` - Soporte Cargo.toml
- `Error Lens` - Errores inline
- `crates` - Versiones de dependencias
- `Docker` - Soporte Docker
- `Dev Containers` - Desarrollo en contenedor

### Recursos Online

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

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

- **Semanas 0-3**: Código simple, sin genéricos ni lifetimes
- **Semanas 4-7**: Introducir enums, Result, Option
- **Semanas 8-10**: Traits y genéricos básicos
- **Semanas 11+**: Código avanzado, async, concurrencia

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

## 📅 Cronograma de Desarrollo del Bootcamp

### Prioridad de Creación

1. ⬜ Semana 00 - Setup (base del bootcamp)
2. ⬜ Semana 01 - Variables
3. ⬜ Semana 02 - Ownership (crítico)
4. ⬜ Semana 03 - Structs
5. ⬜ Semanas 04-07
6. ⬜ Semanas 08-10
7. ⬜ Semanas 11-15

---

**Última actualización**: Diciembre 2025  
**Versión**: 1.0  
**Bootcamp**: Rust Zero to Hero
