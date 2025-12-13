# 🦀 Ejercicio 01: Hello World

## 📋 Información

| Campo | Valor |
|-------|-------|
| **Nivel** | Principiante |
| **Duración** | 15 minutos |
| **Objetivo** | Compilar y ejecutar tu primer programa en Rust |
| **Conceptos** | `fn main()`, `println!`, Cargo basics |

---

## 🎯 Objetivo

Crear, compilar y ejecutar un programa que imprima mensajes en la consola usando la macro `println!`.

---

## 📝 Instrucciones

### Paso 1: Crear el proyecto

```bash
# Dentro del contenedor Docker
cargo new hello-world
cd hello-world
```

### Paso 2: Explorar la estructura

Observa los archivos creados:

```
hello-world/
├── Cargo.toml    # Configuración del proyecto
└── src/
    └── main.rs   # Código fuente
```

### Paso 3: Modificar `src/main.rs`

Edita el archivo para que imprima:

1. Un saludo personalizado con tu nombre
2. El año actual
3. Una frase motivacional

### Paso 4: Ejecutar

```bash
cargo run
```

---

## ✅ Criterios de Éxito

- [ ] El programa compila sin errores
- [ ] El programa imprime al menos 3 líneas
- [ ] Usaste la macro `println!` correctamente

---

## 💡 Pistas

```rust
// Imprimir texto simple
println!("Hola, mundo!");

// Imprimir con formato (interpolación)
let nombre = "Estudiante";
println!("Hola, {}!", nombre);

// Imprimir números
let anio = 2025;
println!("Año: {}", anio);

// Imprimir múltiples valores
println!("{} está aprendiendo Rust en {}", nombre, anio);
```

---

## 🔍 Solución

<details>
<summary>Click para ver la solución</summary>

```rust
fn main() {
    // Saludo personalizado
    let nombre = "Estudiante";
    println!("¡Hola, {}! 🦀", nombre);
    
    // Año actual
    let anio = 2025;
    println!("Bienvenido al Bootcamp Rust {}", anio);
    
    // Frase motivacional
    println!("🚀 ¡Hoy comienza tu viaje de Zero to Hero!");
    
    // Bonus: información adicional
    println!();
    println!("=== Tu Primera Semana ===");
    println!("Temas: Setup, Cargo, Hello World");
    println!("Siguiente: Variables y Tipos");
}
```

**Output esperado:**
```
¡Hola, Estudiante! 🦀
Bienvenido al Bootcamp Rust 2025
🚀 ¡Hoy comienza tu viaje de Zero to Hero!

=== Tu Primera Semana ===
Temas: Setup, Cargo, Hello World
Siguiente: Variables y Tipos
```

</details>

---

## 🚀 Retos Extra

### Reto 1: ASCII Art
Imprime un pequeño cangrejo en ASCII:

```
  _____
 /     \
|  o o  |
|   ^   |
 \_____/
  || ||
```

### Reto 2: Formato avanzado
Investiga y usa:
- `{:?}` para debug
- `{:#?}` para pretty debug
- `{:>10}` para alineación

---

## 📚 Recursos

- [The Rust Book - Hello World](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [Rust by Example - Hello](https://doc.rust-lang.org/rust-by-example/hello.html)
- [std::fmt - Formatting](https://doc.rust-lang.org/std/fmt/)

---

**Siguiente ejercicio:** Variables y Tipos Básicos
