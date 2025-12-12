# 📦 Semana 07: Módulos y Crates

> **Organizando código Rust de forma profesional**

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

- Organizar código en módulos y submódulos
- Controlar visibilidad con `pub` y sus variantes
- Usar `use` para importar elementos
- Crear y usar crates externos
- Estructurar proyectos Rust profesionalmente

## 📋 Contenido

### Teoría

| # | Tema | Archivo |
|---|------|---------|
| 1 | Sistema de Módulos | [01-sistema-modulos.md](1-teoria/01-sistema-modulos.md) |
| 2 | Visibilidad y pub | [02-visibilidad-pub.md](1-teoria/02-visibilidad-pub.md) |
| 3 | Paths y use | [03-paths-use.md](1-teoria/03-paths-use.md) |
| 4 | Archivos y Carpetas | [04-archivos-carpetas.md](1-teoria/04-archivos-carpetas.md) |
| 5 | Crates y Cargo | [05-crates-cargo.md](1-teoria/05-crates-cargo.md) |

### Práctica

| # | Ejercicio | Descripción |
|---|-----------|-------------|
| 1 | [Módulos Básicos](2-practica/practica-01-modulos-basicos/) | Crear y usar módulos inline |
| 2 | [Visibilidad](2-practica/practica-02-visibilidad/) | Control de acceso con pub |
| 3 | [Estructura de Archivos](2-practica/practica-03-estructura-archivos/) | Módulos en archivos separados |
| 4 | [Crates Externos](2-practica/practica-04-crates-externos/) | Usar dependencias de crates.io |

### Proyecto Semanal

| Proyecto | Descripción |
|----------|-------------|
| [Biblioteca de Geometría](3-proyecto/proyecto-geometria/) | Biblioteca modular con shapes, cálculos y formateo |

## ⏱️ Distribución del Tiempo

| Actividad | Duración |
|-----------|----------|
| Teoría (módulos, visibilidad, paths) | 45 min |
| Teoría (archivos, crates) | 45 min |
| Prácticas guiadas | 90 min |
| Proyecto semanal | 60 min |
| **Total** | **4 horas** |

## 🔑 Conceptos Clave

```rust
// Módulo inline
mod matematicas {
    pub fn sumar(a: i32, b: i32) -> i32 { a + b }
}

// Uso
use matematicas::sumar;
let resultado = sumar(2, 3);

// Visibilidad
pub         // público
pub(crate)  // público dentro del crate
pub(super)  // público para el módulo padre
// (sin pub)  privado por defecto
```

## 📁 Estructura de la Semana

```
semana-07/
├── README.md
├── RUBRICA_EVALUACION.md
├── 0-assets/
│   └── *.svg
├── 1-teoria/
│   ├── 01-sistema-modulos.md
│   ├── 02-visibilidad-pub.md
│   ├── 03-paths-use.md
│   ├── 04-archivos-carpetas.md
│   └── 05-crates-cargo.md
├── 2-practica/
│   ├── practica-01-modulos-basicos/
│   ├── practica-02-visibilidad/
│   ├── practica-03-estructura-archivos/
│   └── practica-04-crates-externos/
├── 3-proyecto/
│   └── proyecto-geometria/
├── 4-recursos/
│   ├── ebook-free/
│   ├── videografia/
│   └── webgrafia/
└── 5-glosario/
    └── glosario.md
```

## 🧭 Navegación

| ← Anterior | Inicio | Siguiente → |
|------------|--------|-------------|
| [Semana 06: Manejo de Errores](../semana-06/) | [Bootcamp](../../) | [Semana 08: Colecciones](../semana-08/) |
