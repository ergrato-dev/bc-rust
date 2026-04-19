# 📦 Definición de Structs

![Anatomía de un Struct](../0-assets/01-anatomia-struct.svg)

## ¿Qué es un Struct?

Un **struct** (estructura) es un tipo de dato personalizado que agrupa valores relacionados bajo un mismo nombre. Es similar a una clase en otros lenguajes, pero sin herencia.

---

## Sintaxis Básica

```rust
struct NombreStruct {
    campo1: Tipo1,
    campo2: Tipo2,
    campo3: Tipo3,
}
```

### Convenciones de Nombres

| Elemento | Convención | Ejemplo |
|----------|------------|---------|
| Nombre del struct | PascalCase | `Usuario`, `Producto` |
| Campos | snake_case | `nombre_completo`, `fecha_creacion` |

---

## Ejemplo: Struct Usuario

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}
```

### Anatomía del Struct

```
struct Usuario {           // Declaración
    nombre: String,        // Campo 1: tipo String
    email: String,         // Campo 2: tipo String  
    edad: u32,             // Campo 3: tipo u32
    activo: bool,          // Campo 4: tipo bool
}                          // Fin (sin punto y coma después de })
```

---

## Ejemplo: Struct Producto

```rust
struct Producto {
    id: u64,
    nombre: String,
    precio: f64,
    stock: i32,
    disponible: bool,
}
```

---

## Ejemplo: Struct Punto

```rust
struct Punto {
    x: f64,
    y: f64,
}
```

---

## Ejemplo: Struct Rectangulo

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}
```

---

## Structs con Tipos Complejos

```rust
struct Pedido {
    id: u64,
    productos: Vec<String>,  // Vector de strings
    total: f64,
    fecha: String,           // Por ahora usamos String
}
```

---

## ⚠️ Ownership en Structs

Cuando un struct tiene campos `String` (no `&str`), el struct **es dueño** de esos datos:

```rust
struct Persona {
    nombre: String,   // El struct es dueño del String
    edad: u32,
}
```

Para usar referencias (`&str`), necesitamos **lifetimes** (Semana 10):

```rust
// Esto requiere lifetimes (lo veremos después)
struct PersonaRef<'a> {
    nombre: &'a str,
    edad: u32,
}
```

**Por ahora**: Usa `String` en lugar de `&str` para campos de texto.

---

## 📌 Buenas Prácticas

1. **Agrupar datos relacionados**: Un struct debe representar un concepto coherente
2. **Nombres descriptivos**: El nombre debe indicar qué representa
3. **Tipos apropiados**: Elegir el tipo correcto para cada campo
4. **Documentar**: Usar `///` para documentar el struct y sus campos

```rust
/// Representa un usuario del sistema
struct Usuario {
    /// Nombre completo del usuario
    nombre: String,
    /// Correo electrónico (único)
    email: String,
    /// Edad en años
    edad: u32,
    /// Si la cuenta está activa
    activo: bool,
}
```

---

## 🔗 Relación con Otros Conceptos

| Concepto | Relación |
|----------|----------|
| Tipos | Cada campo tiene un tipo |
| Ownership | El struct puede ser dueño de sus datos |
| Módulos | Los structs pueden ser públicos o privados |
| Traits | Los structs pueden implementar traits |

---

## 📝 Resumen

- Un struct agrupa datos relacionados
- Usa `PascalCase` para el nombre del struct
- Usa `snake_case` para los campos
- Cada campo debe tener un tipo explícito
- Por ahora, usa `String` para texto (no `&str`)

---

*Siguiente: [02-instanciacion-acceso.md](./02-instanciacion-acceso.md)*
