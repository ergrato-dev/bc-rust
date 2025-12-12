# 📚 Proyecto: Sistema de Biblioteca

## 🎯 Objetivo

Implementar un sistema de gestión de biblioteca que modele el concepto de **préstamo (borrowing)** de forma real - cuando prestas un libro, ¡ya no lo tienes!

## 📋 Descripción

Crearás un sistema donde:
- La biblioteca tiene una colección de libros
- Los usuarios pueden **tomar prestados** libros
- Mientras un libro está prestado, no está disponible
- Los usuarios pueden **devolver** libros

## 🏗️ Estructura

```rust
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
}

struct Biblioteca {
    nombre: String,
    libros_disponibles: Vec<Libro>,
    libros_prestados: Vec<(Libro, String)>, // (libro, nombre_usuario)
}
```

## 📝 Funcionalidades Requeridas

### Nivel Básico ⭐

1. **Crear biblioteca** con algunos libros
2. **Listar libros disponibles** (referencia inmutable)
3. **Prestar libro** (mover libro de disponibles a prestados)
4. **Devolver libro** (mover libro de prestados a disponibles)

### Nivel Intermedio ⭐⭐

5. **Buscar libro** por título o autor (referencia)
6. **Ver libros prestados** con nombre del usuario
7. **Contar libros** disponibles y prestados

### Nivel Avanzado ⭐⭐⭐

8. **Historial de préstamos** por usuario
9. **Reservar libro** que está prestado
10. **Estadísticas** (libro más prestado, etc.)

## 🎨 Ejemplo de Uso

```rust
fn main() {
    let mut biblioteca = Biblioteca::new("Biblioteca Central");
    
    // Agregar libros
    biblioteca.agregar_libro(Libro::new(
        "978-0-13-110362-7",
        "The C Programming Language",
        "Kernighan & Ritchie"
    ));
    
    // Listar disponibles
    println!("Disponibles: {:?}", biblioteca.listar_disponibles());
    
    // Prestar libro
    match biblioteca.prestar("978-0-13-110362-7", "Ana") {
        Ok(libro) => println!("Prestado: {}", libro.titulo),
        Err(e) => println!("Error: {}", e),
    }
    
    // El libro ya no está disponible
    println!("Disponibles: {:?}", biblioteca.listar_disponibles());
    
    // Devolver
    biblioteca.devolver("978-0-13-110362-7")?;
}
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Usa `&self` para lectura | 20 |
| Usa `&mut self` para modificación | 20 |
| No usa `.clone()` innecesariamente | 15 |
| Manejo de errores con Result | 15 |
| Tests que verifican ownership | 15 |
| Código limpio y documentado | 15 |

## 💡 Pistas

1. Un libro **se mueve** cuando se presta (no se clona)
2. Usa `Vec::remove()` y `Vec::push()` para mover entre colecciones
3. `Vec::iter().position()` ayuda a encontrar índices
4. Considera usar `Result<T, String>` para errores

## 🚀 Extensiones Opcionales

- Añadir fechas de préstamo
- Límite de libros por usuario
- Multas por devolución tardía
- Persistencia en archivo JSON
