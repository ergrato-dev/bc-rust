# Práctica 01: Ownership Básico

## 🎯 Objetivo

Comprender las tres reglas del ownership y el concepto de **move**.

## 📋 Ejercicios

### Ejercicio 1: Identificar el Error

El siguiente código **no compila**. Identifica el error y corrígelo de **dos formas diferentes**.

```rust
fn main() {
    let mensaje = String::from("Hola, Rust!");
    let copia = mensaje;
    
    println!("Original: {}", mensaje);
    println!("Copia: {}", copia);
}
```

### Ejercicio 2: Ownership en Funciones

Corrige este código para que compile:

```rust
fn main() {
    let nombre = String::from("Ferris");
    imprimir_nombre(nombre);
    
    println!("Nombre: {}", nombre);
}

fn imprimir_nombre(n: String) {
    println!("Imprimiendo: {}", n);
}
```

### Ejercicio 3: Cadena de Moves

¿Qué variable es válida al final de este código?

```rust
fn main() {
    let a = String::from("Rust");
    let b = a;
    let c = b;
    let d = c;
    
    // ¿Cuáles de estas líneas compilan?
    // println!("{}", a);
    // println!("{}", b);
    // println!("{}", c);
    // println!("{}", d);
}
```

### Ejercicio 4: Scope y Drop

Predice el orden de los mensajes "Creando X" y "Drop de X":

```rust
struct Recurso {
    nombre: String,
}

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("Drop de {}", self.nombre);
    }
}

fn main() {
    println!("Inicio");
    
    let r1 = Recurso { nombre: String::from("R1") };
    println!("Creando R1");
    
    {
        let r2 = Recurso { nombre: String::from("R2") };
        println!("Creando R2");
    }
    
    let r3 = Recurso { nombre: String::from("R3") };
    println!("Creando R3");
    
    println!("Fin");
}
```

---

## ✅ Criterios de Éxito

- [ ] Todos los ejercicios compilan correctamente
- [ ] Puedes explicar por qué ocurre cada error
- [ ] Entiendes la diferencia entre move y clone
- [ ] Comprendes cuándo se llama a `drop()`

---

## 💡 Pistas

<details>
<summary>Pista Ejercicio 1</summary>

Hay dos formas de solucionar:
1. Usar `.clone()` para crear una copia
2. Usar referencias (`&`) en lugar de mover

</details>

<details>
<summary>Pista Ejercicio 2</summary>

La función `imprimir_nombre` toma ownership. Opciones:
1. Cambiar la firma para recibir `&String`
2. Devolver el `String` desde la función

</details>
