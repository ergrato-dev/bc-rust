# Práctica 04: Ownership en Funciones

## 🎯 Objetivo

Diseñar funciones que manejen ownership correctamente.

## 📋 Ejercicios

### Ejercicio 1: Elegir la Firma Correcta

Para cada descripción, elige la firma de función correcta:

1. **Contar palabras** en un texto (sin modificar)
2. **Ordenar** un vector in-place
3. **Concatenar** dos strings en uno nuevo
4. **Vaciar** un vector y retornar sus elementos

Opciones:
- `fn f(x: T) -> Y`
- `fn f(x: &T) -> Y`
- `fn f(x: &mut T)`

### Ejercicio 2: Implementar Funciones

Implementa cada función con la firma correcta:

```rust
// a) Contar caracteres (solo lectura)
fn contar_caracteres(???) -> usize

// b) Convertir a mayúsculas (modificar in-place)
fn a_mayusculas(???)

// c) Crear saludo (retornar nuevo String)
fn crear_saludo(nombre: ???) -> String

// d) Tomar primer elemento (consumir vector)
fn tomar_primero(???) -> Option<T>
```

### Ejercicio 3: Refactorizar

Este código usa `.clone()` innecesariamente. Refactorízalo:

```rust
fn procesar_datos(datos: Vec<i32>) -> i32 {
    let copia = datos.clone();
    let suma: i32 = copia.iter().sum();
    let max = copia.iter().max().unwrap_or(&0);
    suma + max
}

fn main() {
    let mis_datos = vec![1, 2, 3, 4, 5];
    let resultado = procesar_datos(mis_datos.clone());
    println!("Datos: {:?}", mis_datos);
    println!("Resultado: {}", resultado);
}
```

### Ejercicio 4: API de Struct

Diseña métodos para este struct siguiendo las mejores prácticas:

```rust
struct Inventario {
    items: Vec<String>,
}

impl Inventario {
    // Constructor
    fn new() -> Self
    
    // Agregar item (¿qué tipo de self?)
    fn agregar(&??? self, item: ???)
    
    // Listar items (¿qué tipo de self?)
    fn listar(&??? self) -> ???
    
    // Buscar item (¿qué tipo de self?)
    fn contiene(&??? self, item: ???) -> bool
    
    // Quitar item (¿qué tipo de self?)
    fn quitar(&??? self, item: ???) -> Option<String>
}
```

---

## ✅ Criterios de Éxito

- [ ] Eliges `&T`, `&mut T`, o `T` apropiadamente
- [ ] No usas `.clone()` innecesariamente
- [ ] Tus funciones son flexibles (aceptan `&str` cuando es posible)
- [ ] El código compila sin warnings de clippy

---

## 💡 Reglas Generales

| Necesidad | Usar |
|-----------|------|
| Solo leer | `&T` o `&str` |
| Modificar | `&mut T` |
| Consumir/transformar | `T` |
| Crear nuevo | `-> T` |
