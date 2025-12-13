# 🦀 Bootcamp Rust: Zero to Hero

![Bootcamp Rust Header](_assets/bootcamp-header.svg)

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

> 🎓 Bootcamp intensivo de **17 semanas (68 horas)** para dominar Rust desde cero hasta un nivel avanzado.  
> 🐳 Entorno containerizado con Docker para desarrollo consistente.

<p align="center">
  <a href="README.en.md">
    <img src="https://img.shields.io/badge/🇺🇸_English_Version-Click_Here-blue?style=for-the-badge" alt="English Version">
  </a>
</p>

---

## 📋 Descripción

Este bootcamp está diseñado para llevar a los estudiantes desde los fundamentos de Rust hasta conceptos avanzados como concurrencia, async/await y smart pointers. Utilizamos Docker para garantizar un entorno de desarrollo idéntico para todos los participantes.

### ¿Por qué Rust?

- 🚀 **Rendimiento** - Velocidad comparable a C/C++
- 🔒 **Seguridad** - Prevención de errores de memoria en tiempo de compilación
- 🧵 **Concurrencia** - Fearless concurrency sin data races
- 🛠️ **Herramientas** - Cargo, rustfmt, clippy, excelente documentación
- 💼 **Demanda** - Lenguaje más amado por 8 años consecutivos (Stack Overflow)

---

## 🗓️ Estructura del Bootcamp

| Semana | Tema Principal                             | Nivel | Duración |
| ------ | ------------------------------------------ | ----- | -------- |
| **1**  | [Setup y Hello World](bootcamp/week-01)    | 🟢    | 4 horas  |
| **2**  | [Variables y Tipos](bootcamp/week-02)      | 🟢    | 4 horas  |
| **3**  | [Ownership y Borrowing](bootcamp/week-03)  | 🟡    | 4 horas  |
| **4**  | [Structs y Métodos](bootcamp/week-04)       | 🟢    | 4 horas  |
| **5**  | [Enums y Pattern Matching](bootcamp/week-05) | 🟡  | 4 horas  |
| **6**  | [Error Handling](bootcamp/week-06)         | 🟡    | 4 horas  |
| **7**  | [Módulos y Crates](bootcamp/week-07)       | 🟢    | 4 horas  |
| **8**  | [Colecciones](bootcamp/week-08)            | 🟡    | 4 horas  |
| **9**  | [Traits Básicos](bootcamp/week-09)         | 🟡    | 4 horas  |
| **10** | [Generics](bootcamp/week-10)               | 🟡    | 4 horas  |
| **11** | [Lifetimes](bootcamp/week-11)              | 🔴    | 4 horas  |
| **12** | [Closures e Iteradores](bootcamp/week-12)  | 🟡    | 4 horas  |
| **13** | [Smart Pointers](bootcamp/week-13)         | 🔴    | 4 horas  |
| **14** | [Concurrencia](bootcamp/week-14)           | 🔴    | 4 horas  |
| **15** | [Async/Await](bootcamp/week-15)            | 🔴    | 4 horas  |
| **16** | [Testing y Documentación](bootcamp/week-16) | 🟡   | 4 horas  |
| **17** | [API REST con Axum](bootcamp/week-17)      | 🔴    | 4 horas  |

**Total**: 68 horas de formación intensiva

**Leyenda**: 🟢 Básico | 🟡 Intermedio | 🔴 Avanzado

---

## 🚀 Inicio Rápido

### Prerrequisitos

- [Docker](https://docs.docker.com/get-docker/) instalado
- [VS Code](https://code.visualstudio.com/) con extensión [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- Git

### Opción 1: Dev Container (Recomendado)

```bash
# Clonar repositorio
git clone https://github.com/ergrato-dev/bc-rust.git
cd bc-rust

# Abrir en VS Code
code .

# VS Code detectará el Dev Container automáticamente
# Click en "Reopen in Container"
```

### Opción 2: Docker Compose

```bash
# Clonar repositorio
git clone https://github.com/ergrato-dev/bc-rust.git
cd bc-rust

# Construir imagen
docker compose build

# Iniciar contenedor interactivo
docker compose run --rm rust-dev

# Dentro del contenedor
cargo --version
rustc --version
```

### Opción 3: Docker directo

```bash
# Construir imagen
docker build -t bc-rust .

# Ejecutar contenedor
docker run -it --rm -v $(pwd):/workspace bc-rust

# Ejecutar un ejercicio específico
docker run --rm -v $(pwd):/workspace bc-rust cargo run -p practice-01-hello-axum
```

---

## 📁 Estructura del Repositorio

```
bc-rust/
├── .devcontainer/           # Configuración Dev Container
├── .github/
│   └── copilot-instructions.md
├── _assets/                 # Recursos visuales
├── _docs/                   # Documentación adicional
├── _scripts/                # Scripts de utilidad
├── bootcamp/
│   ├── week-01/             # Setup y Hello World
│   ├── week-02/             # Variables y Tipos
│   ├── ...
│   └── week-17/             # API REST con Axum
├── Cargo.toml               # Workspace configuration
├── docker-compose.yml
├── Dockerfile
└── README.md
```

Cada semana contiene:

```
week-XX/
├── README.md                # Guía principal
├── RUBRICA_EVALUACION.md    # Criterios de evaluación
├── 0-assets/                # Diagramas SVG
├── 1-theory/                # Material teórico
├── 2-practice/              # Ejercicios
│   ├── practice-01-xxx/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── README.md
│   └── project-xxx/         # Proyecto integrador
└── 4-resources/             # Glosario, referencias
```

---

## � Estadísticas del Proyecto

<table>
<tr>
<td align="center"><b>✅ Compila</b></td>
<td align="center"><b>📝 Tests</b></td>
<td align="center"><b>📁 Ejercicios</b></td>
<td align="center"><b>🎯 Proyectos</b></td>
</tr>
<tr>
<td align="center"><code>cargo check</code><br/>✔️ Pasa</td>
<td align="center"><b>812+</b><br/>tests unitarios</td>
<td align="center"><b>65</b><br/>prácticas</td>
<td align="center"><b>14</b><br/>proyectos semanales</td>
</tr>
</table>

```bash
# Verificar compilación
docker compose run --rm rust-dev cargo check --workspace

# Ejecutar tests
docker compose run --rm rust-dev cargo test --workspace

# Linting
docker compose run --rm rust-dev cargo clippy --workspace
```

---

## �🛠️ Comandos Útiles

### Docker

```bash
# Desarrollo interactivo
docker compose run --rm rust-dev

# Ejecutar código
docker compose run --rm rust-run

# Ejecutar tests
docker compose run --rm rust-test

# Watch mode (hot reload)
docker compose run --rm rust-watch

# Linting (clippy + fmt)
docker compose run --rm rust-lint
```

### Cargo (dentro del contenedor)

```bash
cargo build          # Compilar
cargo run            # Ejecutar
cargo test           # Tests
cargo clippy         # Linter
cargo fmt            # Formatear
cargo doc --open     # Documentación
```

---

## 📊 Metodología de Aprendizaje

Cada sesión de 4 horas sigue esta estructura:

| Tiempo      | Actividad           | Tipo          |
| ----------- | ------------------- | ------------- |
| 0:00 - 0:45 | Teoría y conceptos  | 📖 Exposición |
| 0:45 - 1:15 | Demo en vivo        | 💻 Código     |
| 1:15 - 1:30 | **Descanso**        | ☕            |
| 1:30 - 2:30 | Ejercicios guiados  | 🛠️ Práctica   |
| 2:30 - 3:30 | Proyecto individual | 🎯 Aplicación |
| 3:30 - 4:00 | Revisión y cierre   | 📝 Evaluación |

---

## 🎓 Evaluación

| Tipo             | Peso | Descripción            |
| ---------------- | ---- | ---------------------- |
| **Conocimiento** | 30%  | Cuestionarios teóricos |
| **Desempeño**    | 40%  | Ejercicios en clase    |
| **Producto**     | 30%  | Código funcional       |

### Criterios de Código

- ✅ Compila sin warnings (`cargo clippy`)
- ✅ Pasa todos los tests (`cargo test`)
- ✅ Código formateado (`cargo fmt --check`)
- ✅ Manejo correcto de errores (no `unwrap()` en producción)

---

## 📚 Recursos Adicionales

### Documentación Oficial

- [The Rust Book](https://doc.rust-lang.org/book/) - Libro oficial
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Ejemplos prácticos
- [Rust Reference](https://doc.rust-lang.org/reference/) - Referencia del lenguaje
- [Standard Library](https://doc.rust-lang.org/std/) - Documentación std

### Práctica

- [Rustlings](https://github.com/rust-lang/rustlings) - Ejercicios interactivos
- [Exercism Rust](https://exercism.org/tracks/rust) - Mentored exercises
- [Advent of Code](https://adventofcode.com/) - Desafíos de programación

### Comunidad

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://reddit.com/r/rust)

---

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Este es un proyecto **open source** y valoramos tu participación.

### Formas de Contribuir

- 📚 **Contenido**: Mejorar explicaciones, agregar ejemplos
- 💻 **Código**: Nuevos ejercicios, mejoras, tests
- 🐛 **Bugs**: Reportar errores en contenido o código
- 🎨 **Diseño**: Crear diagramas SVG educativos
- 🌐 **Traducciones**: Traducir contenido a otros idiomas

### Primeros Pasos

1. Lee nuestra [Guía de Contribución](CONTRIBUTING.md)
2. Revisa el [Código de Conducta](CODE_OF_CONDUCT.md)
3. Busca issues con etiqueta `good first issue`
4. ¡Haz tu primer PR!

---

## 🔒 Seguridad

Para reportar vulnerabilidades de seguridad, consulta nuestra [Política de Seguridad](SECURITY.md).

---

## 📄 Licencia

Este proyecto está bajo la licencia **MIT**. Ver [LICENSE](LICENSE) para más detalles.

Esto significa que puedes:
- ✅ Usar el material libremente
- ✅ Modificar y adaptar
- ✅ Distribuir copias
- ✅ Uso comercial

---

## 🙏 Agradecimientos

- 🦀 [Rust Community](https://www.rust-lang.org/community) por el increíble lenguaje
- 📚 [The Rust Book](https://doc.rust-lang.org/book/) como referencia principal
- 🐳 [Docker](https://www.docker.com/) por el entorno containerizado
- 💜 Todos los contribuidores del proyecto

---

## ⭐ Apoya el Proyecto

Si este bootcamp te es útil:

- ⭐ Dale una estrella al repositorio
- 🔀 Compártelo con otros
- 🤝 Contribuye con mejoras
- 📢 Menciónalo en redes sociales

---

**Última actualización**: Diciembre 2025  
**Versión**: 1.0  
**Autor**: [ergrato-dev](https://github.com/ergrato-dev)  
**Licencia**: MIT
