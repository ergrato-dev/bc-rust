# 🔒 Visibilidad y pub

> **Controlando el acceso a tu código**

![Visibilidad](../0-assets/02-visibilidad.svg)

---

## Privado por Defecto

En Rust, **todo es privado por defecto**:

```rust
mod secretos {
    fn funcion_privada() {
        println!("Soy secreta");
    }
    
    pub fn funcion_publica() {
        println!("Soy pública");
        funcion_privada(); // ✅ OK desde el mismo módulo
    }
}

fn main() {
    secretos::funcion_publica();  // ✅ OK
    // secretos::funcion_privada(); // ❌ Error: privada
}
```

---

## El Modificador pub

`pub` hace un elemento visible fuera de su módulo:

```rust
mod biblioteca {
    pub struct Libro {
        pub titulo: String,    // público
        pub autor: String,     // público
        isbn: String,          // privado
    }
    
    impl Libro {
        pub fn nuevo(titulo: &str, autor: &str) -> Self {
            Libro {
                titulo: titulo.to_string(),
                autor: autor.to_string(),
                isbn: Self::generar_isbn(), // uso interno
            }
        }
        
        fn generar_isbn() -> String {
            "978-0-00-000000-0".to_string()
        }
    }
}

fn main() {
    let libro = biblioteca::Libro::nuevo("Rust Book", "Steve Klabnik");
    println!("{} por {}", libro.titulo, libro.autor);
    // println!("{}", libro.isbn); // ❌ Error: isbn es privado
}
```

---

## Niveles de Visibilidad

| Modificador | Visible en |
|-------------|------------|
| (ninguno) | Solo el módulo actual |
| `pub` | Cualquier lugar |
| `pub(crate)` | Solo este crate |
| `pub(super)` | Módulo padre |
| `pub(in path)` | Path específico |

---

## pub(crate) - Público en el Crate

```rust
mod interno {
    // Solo visible dentro de este crate
    pub(crate) fn funcion_interna() {
        println!("Solo para uso interno del crate");
    }
}

// Otros módulos en el mismo crate pueden usarla
mod otro {
    pub fn usar_interno() {
        crate::interno::funcion_interna(); // ✅ OK
    }
}

// Pero si este fuera un crate de biblioteca,
// los usuarios externos NO podrían acceder a funcion_interna
```

---

## pub(super) - Público para el Padre

```rust
mod padre {
    pub mod hijo {
        // Solo visible para el módulo padre
        pub(super) fn secreto_familiar() {
            println!("Solo papá puede verme");
        }
        
        pub fn publico() {
            println!("Todos pueden verme");
        }
    }
    
    pub fn usar_hijo() {
        hijo::secreto_familiar(); // ✅ OK - soy el padre
        hijo::publico();          // ✅ OK
    }
}

fn main() {
    padre::usar_hijo();
    padre::hijo::publico();       // ✅ OK
    // padre::hijo::secreto_familiar(); // ❌ Error
}
```

---

## pub(in path) - Visibilidad Específica

```rust
mod a {
    pub mod b {
        pub mod c {
            // Solo visible hasta el módulo 'a'
            pub(in crate::a) fn solo_para_a() {
                println!("Solo accesible desde 'a' y sus hijos");
            }
        }
    }
    
    pub fn en_a() {
        b::c::solo_para_a(); // ✅ OK
    }
}

fn main() {
    a::en_a();
    // a::b::c::solo_para_a(); // ❌ Error
}
```

---

## Structs: Campos Públicos vs Privados

```rust
mod usuarios {
    pub struct Usuario {
        pub nombre: String,      // Público
        pub email: String,       // Público
        password_hash: String,   // Privado - ¡Seguridad!
    }
    
    impl Usuario {
        pub fn nuevo(nombre: &str, email: &str, password: &str) -> Self {
            Usuario {
                nombre: nombre.to_string(),
                email: email.to_string(),
                password_hash: Self::hash(password),
            }
        }
        
        pub fn verificar_password(&self, password: &str) -> bool {
            self.password_hash == Self::hash(password)
        }
        
        fn hash(s: &str) -> String {
            // Simulación de hash
            format!("hashed_{}", s)
        }
    }
}

fn main() {
    let usuario = usuarios::Usuario::nuevo("Ana", "ana@mail.com", "secreto123");
    println!("Usuario: {}", usuario.nombre);
    println!("¿Password correcta? {}", usuario.verificar_password("secreto123"));
    // println!("{}", usuario.password_hash); // ❌ Error: privado
}
```

---

## Enums: Todo o Nada

A diferencia de structs, las variantes de un enum público son todas públicas:

```rust
mod estados {
    pub enum Estado {
        Activo,
        Inactivo,
        Pendiente,
    }
    // Todas las variantes son públicas automáticamente
}

fn main() {
    let estado = estados::Estado::Activo; // ✅ OK
}
```

---

## Re-exportación con pub use

```rust
mod interno {
    pub mod profundo {
        pub fn funcion_util() {
            println!("¡Hola desde las profundidades!");
        }
    }
}

// Re-exportar para acceso más fácil
pub use interno::profundo::funcion_util;

fn main() {
    // En lugar de:
    interno::profundo::funcion_util();
    
    // Podemos usar directamente:
    funcion_util();
}
```

---

## Patrón: API Pública Limpia

```rust
mod mi_biblioteca {
    // Implementación interna
    mod implementacion {
        pub fn helper_a() { /* ... */ }
        pub fn helper_b() { /* ... */ }
    }
    
    // API pública limpia
    pub fn funcion_principal() {
        implementacion::helper_a();
        implementacion::helper_b();
    }
    
    // Re-exportar solo lo necesario
    pub use implementacion::helper_a;
    // helper_b permanece interno
}
```

---

## Principio de Mínima Exposición

> **Exporta solo lo necesario**

```rust
// ❌ Malo: todo es público
pub mod malo {
    pub fn api_publica() {}
    pub fn detalle_implementacion() {} // No debería ser público
    pub const CONSTANTE_INTERNA: i32 = 42; // No debería ser público
}

// ✅ Bueno: solo lo necesario
pub mod bueno {
    pub fn api_publica() {
        detalle_implementacion();
    }
    
    fn detalle_implementacion() {} // Privado
    const CONSTANTE_INTERNA: i32 = 42; // Privado
}
```

---

## Resumen

```rust
// Niveles de visibilidad:
fn privada() {}           // Solo este módulo
pub fn publica() {}       // Todos
pub(crate) fn crate_() {} // Solo este crate
pub(super) fn padre() {}  // Solo módulo padre
pub(in path) fn path() {} // Solo path específico

// Para structs:
pub struct S {
    pub campo_publico: i32,
    campo_privado: i32,    // Oculta implementación
}
```

---

## 📚 Siguiente

[Paths y use →](03-paths-use.md)
