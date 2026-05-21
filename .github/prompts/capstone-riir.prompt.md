---
mode: agent
description: "Diseña y scaffoldea el proyecto Capstone RIIR (Rewrite It In Rust) de la semana 25."
---

Diseña el proyecto Capstone de la semana 25 del bootcamp bc-rust: **Rewrite It In Rust (RIIR)**.

**Opción elegida**: ${input:option} (A=PyO3 / B=CLI / C=WASM / D=FFI-C)  
**Nombre del proyecto**: ${input:projectName}  
**Descripción**: ${input:projectDescription}

## Opciones disponibles y stacks

| Opción | Proyecto | Stack |
|--------|----------|-------|
| **A** | Parser/lexer expuesto a Python | Rust + PyO3 + maturin |
| **B** | CLI tool que reemplaza herramienta Unix | Rust + clap + indicatif |
| **C** | Motor numérico compilado a WASM | Rust + wasm-pack + TypeScript |
| **D** | Librería criptográfica con API C | Rust + cbindgen + unsafe |

## Estructura del proyecto a generar

```
bootcamp/week-25-capstone_riir/
├── README.md                    ← descripción completa del capstone
├── RUBRICA_EVALUACION.md        ← criterios de evaluación del capstone
├── 0-assets/
│   └── architecture.svg         ← diagrama de arquitectura del proyecto
├── 1-theory/
│   └── README.md                ← recapitulación de conceptos aplicados
├── 2-practice/
│   └── project-${projectSlug}/  ← el proyecto capstone
│       ├── Cargo.toml           ← workspace o crate único
│       ├── src/
│       │   ├── lib.rs           ← lógica principal (bien documentada)
│       │   └── main.rs          ← binario / CLI si aplica
│       ├── tests/
│       │   └── integration_test.rs
│       ├── benches/
│       │   └── bench_main.rs    ← benchmark con criterion
│       ├── README.md            ← README de usuario (como en crates.io)
│       └── [archivos específicos de opción]
└── 5-glossary/
    └── README.md
```

## Requisitos por opción

### Opción A — PyO3 (Python bindings)
Archivos adicionales:
- `pyproject.toml` con maturin config
- `python/tests/test_*.py` — tests desde Python
- Exponer mínimo 3 funciones/tipos a Python
- Benchmark comparando implementación Python nativa vs Rust

### Opción B — CLI Tool
Archivos adicionales:
- `Cargo.toml` con `clap` y `indicatif` (versiones exactas)
- Tests de integración que ejecutan el binario como proceso externo
- `man/` con página de manual generada por `clap`
- Comparativa de performance vs herramienta original (hiperfine o similar)

### Opción C — WebAssembly
Archivos adicionales:
- `www/` con mini-app TypeScript que consume el WASM
- `pkg/` (generado por wasm-pack, en .gitignore)
- `wasm-pack.toml` o `Makefile` con comandos de build
- Demo funcional en `www/index.html`

### Opción D — FFI con C
Archivos adicionales:
- `cbindgen.toml` — configuración de generación de header
- `include/` con header `.h` generado
- `examples/c/` con ejemplo de consumo desde C
- `Makefile` para compilar el ejemplo C

## Requisitos obligatorios para TODAS las opciones

### Calidad del código
- `#![deny(missing_docs)]` en `lib.rs`
- `#![warn(clippy::all, clippy::pedantic)]`
- Todo `unsafe` con `// SAFETY:` comments
- Cero warnings en `cargo clippy`

### Tests
- Tests unitarios en cada módulo
- Al menos 10 tests de integración
- Cobertura mínima de casos borde

### Benchmarks
- Al mínimo 1 benchmark con `criterion`
- Comparativa con implementación anterior (Python/JS/C)

### Documentación
- `README.md` nivel producción: instalación, uso, API, ejemplos
- Todos los tipos y funciones públicas documentados
- `CHANGELOG.md` con entrada de version 0.1.0

## RUBRICA_EVALUACION.md del Capstone

La rúbrica debe evaluar:
- **Corrección funcional** (30%) — el proyecto hace lo prometido
- **Calidad Rust** (25%) — idiomático, clippy, docs
- **Integración con lenguaje destino** (25%) — funciona desde Python/JS/C
- **Performance** (10%) — benchmark muestra mejora vs original
- **Documentación** (10%) — README de usuario, API docs
