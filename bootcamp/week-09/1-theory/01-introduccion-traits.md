# 🎯 Introducción a Traits

## ¿Qué es un Trait?

Un **trait** es una forma de definir funcionalidad compartida en Rust. Similar a las interfaces en otros lenguajes, un trait especifica un conjunto de métodos que un tipo debe implementar.

> 🦀 **Analogía**: Piensa en un trait como un "contrato" - si un tipo implementa el trait, garantiza que tiene ciertos comportamientos.

## ¿Por Qué Necesitamos Traits?

### El Problema

```rust
// Sin traits, ¿cómo hacemos que diferentes tipos 
// se comporten de manera similar?

struct Perro {
    nombre: String,
}

struct Gato {
    nombre: String,
}

// Queremos que ambos puedan "hablar"
// Pero Rust no tiene herencia...
```

### La Solución: Traits

```rust
// Definimos el comportamiento compartido
trait Animal {
    fn hablar(&self) -> String;
}

// Cada tipo implementa su versión
impl Animal for Perro {
    fn hablar(&self) -> String {
        format!("{} dice: ¡Guau!", self.nombre)
    }
}

impl Animal for Gato {
    fn hablar(&self) -> String {
        format!("{} dice: ¡Miau!", self.nombre)
    }
}
```

## Sintaxis Básica

### Definir un Trait

```rust
trait NombreDelTrait {
    // Método requerido (sin cuerpo)
    fn metodo_requerido(&self) -> TipoRetorno;
    
    // Método con implementación default (opcional)
    fn metodo_default(&self) -> String {
        String::from("Implementación por defecto")
    }
}
```

### Partes de un Trait

| Elemento | Descripción | Obligatorio |
|----------|-------------|-------------|
| `trait` | Palabra clave | Sí |
| Nombre | PascalCase | Sí |
| Métodos requeridos | Sin cuerpo | Al menos uno* |
| Métodos default | Con cuerpo | Opcional |

*Un trait puede tener solo métodos default (marker traits)

## Ejemplo Completo

```rust
// Definición del trait
trait Resumen {
    // Método requerido
    fn resumir(&self) -> String;
    
    // Método con implementación default
    fn autor(&self) -> String {
        String::from("Anónimo")
    }
    
    // Método default que usa otro método del trait
    fn vista_previa(&self) -> String {
        format!("Por {}: {}", self.autor(), self.resumir())
    }
}

// Estructura que implementará el trait
struct Articulo {
    titulo: String,
    contenido: String,
    autor: String,
}

// Implementación del trait
impl Resumen for Articulo {
    fn resumir(&self) -> String {
        format!("{}: {}...", self.titulo, &self.contenido[..50.min(self.contenido.len())])
    }
    
    // Sobrescribimos el método default
    fn autor(&self) -> String {
        self.autor.clone()
    }
}

struct Tweet {
    usuario: String,
    contenido: String,
}

impl Resumen for Tweet {
    fn resumir(&self) -> String {
        format!("@{}: {}", self.usuario, self.contenido)
    }
    // autor() usa la implementación default
}
```

## Uso del Trait

```rust
fn main() {
    let articulo = Articulo {
        titulo: String::from("Rust es genial"),
        contenido: String::from("Rust ofrece seguridad de memoria..."),
        autor: String::from("Ferris"),
    };
    
    let tweet = Tweet {
        usuario: String::from("rustlang"),
        contenido: String::from("¡Rust 2024 está aquí!"),
    };
    
    println!("{}", articulo.vista_previa());
    // Por Ferris: Rust es genial: Rust ofrece seguridad...
    
    println!("{}", tweet.vista_previa());
    // Por Anónimo: @rustlang: ¡Rust 2024 está aquí!
}
```

## Traits vs Otros Lenguajes

| Concepto | Rust | Java | TypeScript | Go |
|----------|------|------|------------|-----|
| Definición | `trait` | `interface` | `interface` | `interface` |
| Implementación | `impl T for S` | `implements` | `implements` | implícita |
| Default methods | ✅ Sí | ✅ Sí (Java 8+) | ❌ No | ❌ No |
| Herencia múltiple | Vía traits | No | No | Composición |

## La Regla del Huérfano (Orphan Rule)

Rust tiene una regla importante:

> **Solo puedes implementar un trait si el trait O el tipo es local a tu crate.**

```rust
// ✅ Correcto: trait local para tipo externo
trait MiTrait {}
impl MiTrait for Vec<i32> {}

// ✅ Correcto: trait externo para tipo local
struct MiTipo;
impl std::fmt::Display for MiTipo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MiTipo")
    }
}

// ❌ Error: trait externo para tipo externo
// impl std::fmt::Display for Vec<i32> {} // NO PERMITIDO
```

## Resumen

| Concepto | Descripción |
|----------|-------------|
| Trait | Define comportamiento compartido |
| Método requerido | Debe ser implementado |
| Método default | Implementación opcional |
| Orphan rule | Trait o tipo debe ser local |

---

## 🔗 Navegación

| ⬅️ Anterior | 🏠 Índice | ➡️ Siguiente |
|:------------|:--------:|-------------:|
| [README](../README.md) | [Semana 09](../README.md) | [Implementación](02-implementacion-traits.md) |
