# Práctica 01: Módulos Básicos

## 🎯 Objetivo

Aprender a crear y organizar módulos en Rust, entender la declaración `mod` y la navegación con paths.

## 📋 Instrucciones

### Ejercicio 1: Módulos Inline

Crea módulos inline para una calculadora básica:

```rust
// TODO: Crear un módulo 'operaciones' con funciones:
// - sumar(a: i32, b: i32) -> i32
// - restar(a: i32, b: i32) -> i32

// TODO: Crear un submódulo 'avanzadas' dentro de 'operaciones' con:
// - multiplicar(a: i32, b: i32) -> i32
// - dividir(a: i32, b: i32) -> Option<i32>

fn main() {
    // Usa paths absolutos para llamar las funciones
    let suma = crate::operaciones::sumar(10, 5);
    let producto = crate::operaciones::avanzadas::multiplicar(3, 4);
    
    println!("Suma: {}", suma);
    println!("Producto: {}", producto);
}
```

### Ejercicio 2: Navegación con super y self

Implementa módulos que usen `super` y `self`:

```rust
mod biblioteca {
    pub const NOMBRE: &str = "Biblioteca Rust";
    
    pub mod libros {
        pub fn titulo_completo(titulo: &str) -> String {
            // TODO: Usar super:: para acceder a NOMBRE
            // Retornar: "NOMBRE: titulo"
            todo!()
        }
        
        mod interno {
            pub fn procesar(titulo: &str) -> String {
                // TODO: Usar super:: para llamar a titulo_completo
                todo!()
            }
        }
        
        pub fn procesar_publico(titulo: &str) -> String {
            // TODO: Usar self:: para llamar a interno::procesar
            todo!()
        }
    }
}
```

### Ejercicio 3: Árbol de Módulos

Crea la siguiente estructura de módulos:

```
crate
├── vehiculos
│   ├── terrestres
│   │   ├── automovil (struct Automovil)
│   │   └── motocicleta (struct Motocicleta)
│   └── aereos
│       └── avion (struct Avion)
└── main
```

Cada struct debe tener:
- Un campo `marca: String`
- Un método `describir() -> String`

## ✅ Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operaciones_basicas() {
        assert_eq!(operaciones::sumar(5, 3), 8);
        assert_eq!(operaciones::restar(10, 4), 6);
    }

    #[test]
    fn test_operaciones_avanzadas() {
        assert_eq!(operaciones::avanzadas::multiplicar(6, 7), 42);
        assert_eq!(operaciones::avanzadas::dividir(10, 2), Some(5));
        assert_eq!(operaciones::avanzadas::dividir(10, 0), None);
    }

    #[test]
    fn test_biblioteca() {
        let titulo = biblioteca::libros::titulo_completo("El Quijote");
        assert!(titulo.contains("Biblioteca Rust"));
        assert!(titulo.contains("El Quijote"));
    }

    #[test]
    fn test_vehiculos() {
        let auto = vehiculos::terrestres::automovil::Automovil::new("Toyota");
        assert!(auto.describir().contains("Toyota"));
        
        let avion = vehiculos::aereos::avion::Avion::new("Boeing");
        assert!(avion.describir().contains("Boeing"));
    }
}
```

## 🎯 Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Módulos inline correctos | 25% |
| Uso correcto de super/self | 25% |
| Árbol de módulos completo | 30% |
| Tests pasan | 20% |

## 💡 Pistas

1. Los módulos se declaran con `mod nombre { ... }`
2. `crate::` siempre apunta a la raíz del crate
3. `super::` sube un nivel en el árbol de módulos
4. `self::` es opcional pero hace explícito el módulo actual
5. Todo es privado por defecto - usa `pub` para hacer visible
