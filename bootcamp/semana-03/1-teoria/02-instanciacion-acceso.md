# 🔧 Instanciación y Acceso a Campos

## Crear una Instancia

Para crear una instancia de un struct, especificamos valores para **todos** los campos:

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

fn main() {
    let usuario = Usuario {
        nombre: String::from("Ana García"),
        email: String::from("ana@email.com"),
        edad: 28,
        activo: true,
    };
}
```

---

## Acceder a Campos

Usamos la notación de punto (`.`) para acceder a los campos:

```rust
println!("Nombre: {}", usuario.nombre);
println!("Email: {}", usuario.email);
println!("Edad: {}", usuario.edad);
println!("Activo: {}", usuario.activo);
```

---

## Modificar Campos

Para modificar campos, la instancia debe ser **mutable**:

```rust
let mut usuario = Usuario {
    nombre: String::from("Ana"),
    email: String::from("ana@email.com"),
    edad: 28,
    activo: true,
};

// Modificar campos
usuario.edad = 29;
usuario.email = String::from("ana.nueva@email.com");
```

⚠️ **Importante**: Todo el struct es mutable o inmutable. No puedes tener algunos campos mutables y otros no.

---

## Field Init Shorthand

Cuando el nombre de la variable coincide con el nombre del campo:

```rust
fn crear_usuario(nombre: String, email: String) -> Usuario {
    // Forma larga
    Usuario {
        nombre: nombre,
        email: email,
        edad: 0,
        activo: true,
    }
}

fn crear_usuario_corto(nombre: String, email: String) -> Usuario {
    // Forma corta (field init shorthand)
    Usuario {
        nombre,    // Equivale a nombre: nombre
        email,     // Equivale a email: email
        edad: 0,
        activo: true,
    }
}
```

---

## Struct Update Syntax

Crear un nuevo struct basado en otro existente:

```rust
let usuario1 = Usuario {
    nombre: String::from("Ana"),
    email: String::from("ana@email.com"),
    edad: 28,
    activo: true,
};

// Crear usuario2 basado en usuario1
let usuario2 = Usuario {
    email: String::from("maria@email.com"),  // Solo cambia email
    ..usuario1  // El resto viene de usuario1
};
```

⚠️ **Cuidado con Ownership**: 

```rust
// Después de esto, usuario1.nombre ya no es válido
// porque String no implementa Copy y fue movido a usuario2
// Pero usuario1.edad y usuario1.activo siguen siendo válidos
// porque i32 y bool implementan Copy
```

---

## Ejemplo Completo

```rust
struct Producto {
    id: u64,
    nombre: String,
    precio: f64,
    stock: i32,
}

fn main() {
    // Crear producto
    let mut laptop = Producto {
        id: 1,
        nombre: String::from("Laptop Pro"),
        precio: 999.99,
        stock: 50,
    };

    // Leer campos
    println!("Producto: {}", laptop.nombre);
    println!("Precio: ${:.2}", laptop.precio);

    // Modificar campos
    laptop.precio = 899.99;  // Descuento
    laptop.stock -= 1;       // Venta

    println!("Nuevo precio: ${:.2}", laptop.precio);
    println!("Stock restante: {}", laptop.stock);
}
```

---

## Structs como Parámetros

```rust
fn mostrar_usuario(usuario: &Usuario) {
    println!("Usuario: {} ({})", usuario.nombre, usuario.email);
}

fn cumplir_anios(usuario: &mut Usuario) {
    usuario.edad += 1;
}

fn main() {
    let mut ana = Usuario {
        nombre: String::from("Ana"),
        email: String::from("ana@email.com"),
        edad: 28,
        activo: true,
    };

    mostrar_usuario(&ana);      // Préstamo inmutable
    cumplir_anios(&mut ana);    // Préstamo mutable
    
    println!("Nueva edad: {}", ana.edad);
}
```

---

## Retornar Structs

```rust
fn nuevo_usuario(nombre: String, email: String) -> Usuario {
    Usuario {
        nombre,
        email,
        edad: 0,
        activo: true,
    }
}

fn main() {
    let usuario = nuevo_usuario(
        String::from("Carlos"),
        String::from("carlos@email.com")
    );
}
```

---

## 📝 Resumen

| Operación | Sintaxis |
|-----------|----------|
| Crear instancia | `let x = Struct { campo: valor }` |
| Acceder campo | `x.campo` |
| Modificar campo | `x.campo = nuevo_valor` (requiere `mut`) |
| Field init shorthand | `Struct { campo }` cuando variable = campo |
| Struct update | `Struct { campo: v, ..otro }` |

---

*Siguiente: [03-metodos-impl.md](./03-metodos-impl.md)*
