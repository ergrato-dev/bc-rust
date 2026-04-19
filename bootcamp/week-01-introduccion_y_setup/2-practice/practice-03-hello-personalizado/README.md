# 🎉 Práctica 03: Hello World Personalizado

## 📋 Información

| Campo | Valor |
|-------|-------|
| **Duración** | 30-40 minutos |
| **Dificultad** | ⭐ Principiante |
| **Requisitos** | Práctica 02 completada |

---

## 🎯 Objetivo

Crear un programa Hello World personalizado usando `println!`, variables y formato de strings.

---

## 📝 Pasos

### Paso 1: Crear el Proyecto

```bash
# En el contenedor Docker
cd bootcamp/semana-01/2-practica
cargo new hello-personalizado
cd hello-personalizado
```

---

### Paso 2: Entender println!

Edita `src/main.rs`:

```rust
fn main() {
    // Forma básica
    println!("Hello, World!");
    
    // Con salto de línea explícito
    println!("Primera línea");
    println!("Segunda línea");
    
    // Línea vacía
    println!();
    
    // Sin salto de línea (print! en vez de println!)
    print!("Sin ");
    print!("salto ");
    println!("de línea");
}
```

**Ejecuta:**
```bash
cargo run
```

---

### Paso 3: Variables y Formato Básico

```rust
fn main() {
    // Declarar variables con let
    let nombre = "Rustacean";
    let edad = 1;
    
    // Usar {} como placeholder
    println!("Hola, {}!", nombre);
    println!("Edad: {} años", edad);
    
    // Múltiples placeholders
    println!("{} tiene {} año(s) de experiencia", nombre, edad);
}
```

**Ejecuta y observa el output.**

---

### Paso 4: Formato con Posiciones

```rust
fn main() {
    let a = "primero";
    let b = "segundo";
    
    // Por posición
    println!("{0} antes de {1}", a, b);
    println!("{1} después de {0}", a, b);
    
    // Repetir valores
    println!("{0}, {0}, {0}!", "Rust");
}
```

---

### Paso 5: Formato con Nombres

```rust
fn main() {
    println!(
        "{nombre} está aprendiendo {lenguaje} en {anio}",
        nombre = "Estudiante",
        lenguaje = "Rust",
        anio = 2025
    );
}
```

---

### Paso 6: Formato de Números

```rust
fn main() {
    let numero = 42;
    let decimal = 3.14159;
    
    // Enteros
    println!("Número: {}", numero);
    
    // Decimales con precisión
    println!("Pi: {:.2}", decimal);  // 2 decimales
    println!("Pi: {:.4}", decimal);  // 4 decimales
    
    // Padding (relleno)
    println!("Relleno: {:>10}", numero);  // Derecha, 10 chars
    println!("Relleno: {:<10}", numero);  // Izquierda, 10 chars
    println!("Relleno: {:^10}", numero);  // Centro, 10 chars
    
    // Relleno con ceros
    println!("Con ceros: {:05}", numero);  // 00042
}
```

---

### Paso 7: Caracteres Especiales

```rust
fn main() {
    // Comillas dentro del string
    println!("Él dijo: \"Hola\"");
    
    // Backslash
    println!("Ruta: C:\\Users\\rust");
    
    // Tab
    println!("Col1\tCol2\tCol3");
    
    // Nueva línea dentro del string
    println!("Línea 1\nLínea 2\nLínea 3");
    
    // Raw strings (sin escape)
    println!(r"Esto no escapa \n ni \t");
    
    // Raw string con comillas
    println!(r#"Puedo usar "comillas" directamente"#);
}
```

---

### Paso 8: Emojis y Unicode

```rust
fn main() {
    // Emojis funcionan directamente
    println!("🦀 Rust es genial! 🚀");
    println!("Ferris dice: 🎉🎊🎈");
    
    // Caracteres Unicode
    println!("Español: ñ, á, é, í, ó, ú");
    println!("Símbolos: ★ ☆ ♠ ♣ ♥ ♦");
    println!("Flechas: → ← ↑ ↓ ↔");
}
```

---

### Paso 9: Debug Format

```rust
fn main() {
    let texto = "Hola";
    let numero = 42;
    
    // {:?} para debug
    println!("Debug texto: {:?}", texto);
    println!("Debug número: {:?}", numero);
    
    // {:#?} para pretty debug
    println!("Pretty: {:#?}", texto);
    
    // dbg! macro (imprime archivo, línea y valor)
    dbg!(texto);
    dbg!(numero);
}
```

---

### Paso 10: Proyecto Final - Tarjeta Personal

Crea tu tarjeta de presentación:

```rust
fn main() {
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║                                      ║");
    println!("║   🦀 TARJETA DE PRESENTACIÓN 🦀     ║");
    println!("║                                      ║");
    println!("╠══════════════════════════════════════╣");
    
    let nombre = "Tu Nombre";
    let titulo = "Rust Developer";
    let email = "tu@email.com";
    let github = "@tu-usuario";
    
    println!("║  Nombre:  {:<25} ║", nombre);
    println!("║  Título:  {:<25} ║", titulo);
    println!("║  Email:   {:<25} ║", email);
    println!("║  GitHub:  {:<25} ║", github);
    
    println!("║                                      ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  🌟 Rust Bootcamp 2025 - Semana 01  ║");
    println!("╚══════════════════════════════════════╝");
    println!();
}
```

---

## 🎯 Reto: Personaliza tu Tarjeta

1. Cambia los datos a los tuyos
2. Agrega más campos (país, hobby, etc.)
3. Usa colores ANSI (opcional, avanzado):

```rust
// Colores ANSI básicos
println!("\x1b[31mTexto Rojo\x1b[0m");
println!("\x1b[32mTexto Verde\x1b[0m");
println!("\x1b[33mTexto Amarillo\x1b[0m");
println!("\x1b[34mTexto Azul\x1b[0m");
```

---

## ✅ Checklist

- [ ] Entendido `println!` básico
- [ ] Usado variables en `println!`
- [ ] Aplicado formato con `{}`
- [ ] Probado formato numérico
- [ ] Usado caracteres especiales
- [ ] Creada tarjeta personal

---

## 📚 Referencia Rápida

| Formato | Descripción | Ejemplo |
|---------|-------------|---------|
| `{}` | Display normal | `42` |
| `{:?}` | Debug | `"hello"` |
| `{:#?}` | Pretty debug | Multi-línea |
| `{:.2}` | 2 decimales | `3.14` |
| `{:>10}` | Pad derecha | `       42` |
| `{:<10}` | Pad izquierda | `42       ` |
| `{:^10}` | Centrado | `   42    ` |
| `{:05}` | Pad con ceros | `00042` |

---

**Siguiente práctica**: Exploración y Experimentación
