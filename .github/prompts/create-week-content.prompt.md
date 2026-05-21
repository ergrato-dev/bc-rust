---
mode: agent
description: "Scaffold completo de una semana del bootcamp respetando el orden de creación: README → rúbrica → teoría → assets → prácticas → proyecto → recursos → glosario → commit+push."
---

Crea el contenido completo para la **semana ${input:weekNumber}** del bootcamp bc-rust.
El tema es: **${input:weekTopic}** (fase: ${input:phase}).

## ⚠️ Política de trabajo: solodev

- **No crear ramas**. Todo va directamente a `main`.
- **No abrir Pull Requests**.
- Al finalizar la semana completa: `git add . && git commit -m "week-${weekNumber}: ${input:weekTopic}" && git push`

## Orden de creación OBLIGATORIO

Seguir este orden exacto, completando cada paso antes de pasar al siguiente:

1. `README.md` — guía principal de la semana
2. `RUBRICA_EVALUACION.md` — criterios de evaluación
3. `1-theory/README.md` + archivos adicionales — **~180 líneas por archivo de teoría**
4. `0-assets/` — diagramas SVG de apoyo (dark mode, sin gradientes)
5. `2-practice/practice-NN-*/` — prácticas con scaffolding y tests
6. `2-practice/project-*/` — proyecto integrador
7. `4-resources/README.md` — links y referencias
8. `5-glossary/README.md` — glosario de términos

## Estructura a generar

```
bootcamp/week-${weekNumber}-${input:weekSlug}/
├── README.md
├── RUBRICA_EVALUACION.md
├── 0-assets/
│   └── .gitkeep
├── 1-theory/
│   └── README.md          ← teoría detallada
├── 2-practice/
│   ├── practice-01-${input:practice1Slug}/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs    ← con TODO scaffolding
│   │   └── README.md
│   ├── practice-02-${input:practice2Slug}/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── README.md
│   └── project-${input:projectSlug}/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   └── lib.rs
│       ├── tests/
│       │   └── integration_test.rs
│       └── README.md
├── 3-project/
│   └── README.md          ← brief del proyecto semanal
├── 4-resources/
│   └── README.md          ← links, referencias
└── 5-glossary/
    └── README.md          ← términos clave de la semana
```

## Requisitos de contenido

### README.md principal
- Badge de semana y nivel (🟢/🟡/🔴)
- Objetivos de aprendizaje (bullets concretos)
- Tabla de contenidos con links a subcarpetas
- Tabla de ejercicios con nombre, descripción y dificultad
- Instrucciones para correr los ejercicios con `cargo test -p nombre`

### 1-theory/README.md
- Explicación teórica completa del tema
- Comparación con otros lenguajes (Python, JavaScript, C++ cuando aplique)
- Al menos 3 ejemplos de código comentados
- Sección "Errores comunes y cómo resolverlos" con errores reales del compilador

### Prácticas (practice-01, practice-02)
- `src/main.rs` con funciones scaffolded usando `todo!()`
- Tests incluidos que el estudiante debe hacer pasar
- `README.md` con objetivo, instrucciones, criterios y pistas

### Proyecto semanal (project-*)
- Conecta los conceptos de la semana en un mini-proyecto realista
- Dominio: sistema de inventario / gestión de tareas / procesamiento de datos
- `#[cfg(test)] mod tests` con al menos 5 tests

### RUBRICA_EVALUACION.md
- 3 secciones: Conocimiento (30%), Desempeño (40%), Producto (30%)
- Criterios binarios o de rango para cada punto
- Escala de 0-100 puntos con distribución por sección

## Convenciones de código
- Semanas 1-10: no usar `unwrap()` en proyectos, sí en prácticas simples
- Semanas 11+: `Result<T, E>` obligatorio, custom error types
- Semanas 18+: aplicar reglas de unsafe-ffi.instructions.md si aplica
- Todo código debe pasar `cargo clippy -- -D warnings`
