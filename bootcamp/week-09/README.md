# 📚 Semana 09: Traits Básicos

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

- Comprender qué son los traits y por qué son fundamentales en Rust
- Definir e implementar traits personalizados
- Usar traits derivables (`Debug`, `Clone`, `PartialEq`, etc.)
- Implementar traits de la biblioteca estándar
- Aplicar trait bounds en funciones genéricas
- Usar traits como parámetros con `impl Trait`

## 📋 Contenido

| Tema | Archivo | Descripción |
|------|---------|-------------|
| Introducción a Traits | [01-introduccion-traits.md](1-teoria/01-introduccion-traits.md) | Qué son, por qué existen, sintaxis básica |
| Implementación | [02-implementacion-traits.md](1-teoria/02-implementacion-traits.md) | `impl Trait for Type`, métodos default |
| Traits Derivables | [03-traits-derivables.md](1-teoria/03-traits-derivables.md) | `#[derive()]`, Debug, Clone, Copy, PartialEq |
| Traits Estándar | [04-traits-estandar.md](1-teoria/04-traits-estandar.md) | Display, Default, From/Into, Iterator |
| Trait Bounds | [05-trait-bounds.md](1-teoria/05-trait-bounds.md) | Restricciones genéricas, `where` clauses |

## 🗓️ Distribución del Tiempo (4 horas)

| Actividad | Duración | Descripción |
|-----------|----------|-------------|
| Teoría | 60 min | Conceptos de traits y su rol en Rust |
| Práctica Guiada | 45 min | Definir e implementar traits |
| Ejercicios | 90 min | 4 prácticas progresivas |
| Proyecto | 45 min | Sistema de formas geométricas polimórfico |

## 🔑 Conceptos Clave

```rust
// Definir un trait
trait Describible {
    fn describir(&self) -> String;
    
    // Método con implementación default
    fn tipo(&self) -> &str {
        "desconocido"
    }
}

// Implementar trait para un tipo
struct Producto {
    nombre: String,
    precio: f64,
}

impl Describible for Producto {
    fn describir(&self) -> String {
        format!("{}: ${:.2}", self.nombre, self.precio)
    }
}

// Traits derivables
#[derive(Debug, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

// Trait bounds
fn imprimir<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// impl Trait (syntax sugar)
fn crear_descripcion(d: &impl Describible) -> String {
    d.describir()
}
```

## 📁 Estructura de la Semana

```
semana-09/
├── README.md
├── RUBRICA_EVALUACION.md
├── 0-assets/
│   ├── 01-traits-concepto.svg
│   ├── 02-implementacion.svg
│   ├── 03-derivables.svg
│   ├── 04-traits-estandar.svg
│   └── 05-trait-bounds.svg
├── 1-teoria/
│   ├── 01-introduccion-traits.md
│   ├── 02-implementacion-traits.md
│   ├── 03-traits-derivables.md
│   ├── 04-traits-estandar.md
│   └── 05-trait-bounds.md
├── 2-practica/
│   ├── practica-01-definir-traits/
│   ├── practica-02-implementar-traits/
│   ├── practica-03-traits-derivables/
│   └── practica-04-trait-bounds/
├── 3-proyecto/
│   └── proyecto-formas/
├── 4-recursos/
│   ├── ebook-free.md
│   ├── videografia.md
│   └── webgrafia.md
└── 5-glosario/
    └── glosario.md
```

## 🔗 Navegación

| ⬅️ Anterior | 🏠 Inicio | ➡️ Siguiente |
|:------------|:--------:|-------------:|
| [Semana 08: Colecciones](../semana-08/README.md) | [Bootcamp](../BOOTCAMP-COMPLETO.md) | [Semana 10: Generics](../semana-10/README.md) |

## 💡 Tips de la Semana

> 🦀 **Traits vs Interfaces**: Los traits de Rust son similares a las interfaces de otros lenguajes, pero más poderosos gracias a los métodos default y la coherencia de traits.

> ⚡ **Derivar es gratis**: Siempre que sea posible, usa `#[derive()]` - el compilador genera código optimizado.

> 🎯 **Regla del huérfano**: Solo puedes implementar un trait si el trait o el tipo es local a tu crate.
