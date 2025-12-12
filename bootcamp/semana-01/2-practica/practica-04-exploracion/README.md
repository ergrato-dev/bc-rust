# 🔬 Práctica 04: Exploración y Experimentación

## 📋 Información

| Campo | Valor |
|-------|-------|
| **Duración** | 40-50 minutos |
| **Dificultad** | ⭐⭐ Principiante-Intermedio |
| **Requisitos** | Prácticas 01-03 completadas |

---

## 🎯 Objetivo

Explorar el compilador de Rust, entender sus mensajes de error, y experimentar con código para aprender de los errores.

---

## 📝 Pasos

### Paso 1: Crear Proyecto de Exploración

```bash
cd bootcamp/semana-01/2-practica
cargo new exploracion
cd exploracion
```

---

### Paso 2: Provocar Errores de Sintaxis

Edita `src/main.rs` con errores intencionales:

```rust
fn main() {
    println!("Hola, mundo!")  // Sin punto y coma
}
```

**Ejecuta:**
```bash
cargo build
```

**Observa el error:**
```
error: expected `;`, found `}`
 --> src/main.rs:2:30
  |
2 |     println!("Hola, mundo!")
  |                              ^ help: add `;` here
```

> 💡 El compilador de Rust es MUY útil. Lee siempre el mensaje `help:`.

**Corrige y continúa.**

---

### Paso 3: Error de Variable No Usada

```rust
fn main() {
    let x = 5;
    println!("Hola!");
}
```

**Ejecuta:**
```bash
cargo build
```

**Warning:**
```
warning: unused variable: `x`
 --> src/main.rs:2:9
  |
2 |     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
```

**Aprende:** Usa `_x` para indicar que es intencional:

```rust
let _x = 5;  // OK, no warning
```

---

### Paso 4: Error de Inmutabilidad

```rust
fn main() {
    let x = 5;
    x = 10;  // Error!
    println!("{}", x);
}
```

**Error:**
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 10;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

**Solución:**
```rust
let mut x = 5;  // Ahora es mutable
x = 10;
```

---

### Paso 5: Error de Tipos

```rust
fn main() {
    let x: i32 = "hola";  // Error!
}
```

**Error:**
```
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let x: i32 = "hola";
  |            ---   ^^^^^^ expected `i32`, found `&str`
  |            |
  |            expected due to this
```

> 💡 Rust tiene tipado estático fuerte. Los tipos deben coincidir.

---

### Paso 6: Shadowing (No es Error)

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
    
    let x = "ahora soy texto";  // Shadowing: es válido!
    println!("x = {}", x);
    
    let x = x.len();  // Shadowing de nuevo
    println!("x = {}", x);
}
```

**Ejecuta:**
```bash
cargo run
```

**Output:**
```
x = 5
x = ahora soy texto
x = 17
```

> 💡 Shadowing permite reusar nombres de variables con tipos diferentes.

---

### Paso 7: Explorar Tipos Básicos

```rust
fn main() {
    // Enteros
    let entero: i32 = 42;
    let entero_grande: i64 = 1_000_000;  // _ como separador
    let sin_signo: u32 = 100;
    
    // Flotantes
    let decimal: f64 = 3.14159;
    
    // Booleanos
    let verdadero: bool = true;
    let falso = false;  // Inferencia de tipo
    
    // Caracteres
    let letra: char = 'R';
    let emoji: char = '🦀';
    
    // Strings
    let texto: &str = "Hola, Rust!";
    
    // Imprimir todo con debug
    println!("entero: {:?}", entero);
    println!("entero_grande: {:?}", entero_grande);
    println!("sin_signo: {:?}", sin_signo);
    println!("decimal: {:?}", decimal);
    println!("verdadero: {:?}", verdadero);
    println!("falso: {:?}", falso);
    println!("letra: {:?}", letra);
    println!("emoji: {:?}", emoji);
    println!("texto: {:?}", texto);
}
```

---

### Paso 8: Operaciones Básicas

```rust
fn main() {
    // Matemáticas
    let suma = 5 + 10;
    let resta = 95 - 50;
    let producto = 4 * 30;
    let cociente = 56.7 / 32.2;
    let residuo = 43 % 5;
    
    println!("Suma: {}", suma);
    println!("Resta: {}", resta);
    println!("Producto: {}", producto);
    println!("Cociente: {:.2}", cociente);
    println!("Residuo: {}", residuo);
    
    // Comparaciones
    println!("\n--- Comparaciones ---");
    println!("5 > 3: {}", 5 > 3);
    println!("5 < 3: {}", 5 < 3);
    println!("5 == 5: {}", 5 == 5);
    println!("5 != 3: {}", 5 != 3);
    
    // Lógicas
    println!("\n--- Lógicas ---");
    println!("true && false: {}", true && false);
    println!("true || false: {}", true || false);
    println!("!true: {}", !true);
}
```

---

### Paso 9: Funciones Básicas

```rust
fn main() {
    saludar();
    saludar_a("Rustacean");
    
    let resultado = sumar(5, 3);
    println!("5 + 3 = {}", resultado);
    
    println!("Es par 4? {}", es_par(4));
    println!("Es par 7? {}", es_par(7));
}

fn saludar() {
    println!("¡Hola desde una función!");
}

fn saludar_a(nombre: &str) {
    println!("¡Hola, {}!", nombre);
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b  // Sin ; retorna el valor
}

fn es_par(n: i32) -> bool {
    n % 2 == 0
}
```

---

### Paso 10: Experimento Libre

Ahora es tu turno. Intenta:

1. **Crear una calculadora simple:**
```rust
fn main() {
    let a = 10;
    let b = 5;
    
    println!("{} + {} = {}", a, b, sumar(a, b));
    println!("{} - {} = {}", a, b, restar(a, b));
    // ... implementa más operaciones
}
```

2. **Crear un generador de texto:**
```rust
fn main() {
    let adjetivo = "increíble";
    let sustantivo = "programador";
    
    generar_frase(adjetivo, sustantivo);
}

fn generar_frase(adj: &str, sust: &str) {
    println!("Eres un {} {}", adj, sust);
}
```

3. **Romper cosas y aprender de los errores:**
   - ¿Qué pasa si divides por cero?
   - ¿Qué pasa si sumas un i32 con un i64?
   - ¿Qué pasa si no retornas nada en una función con `->`?

---

## 📊 Errores Comunes y Soluciones

| Error | Causa | Solución |
|-------|-------|----------|
| `E0384` | Asignar a variable inmutable | Usar `mut` |
| `E0308` | Tipos no coinciden | Verificar tipos |
| `E0425` | Variable no encontrada | Revisar scope/typos |
| `E0382` | Valor movido | (Veremos en Semana 02) |

---

## ✅ Checklist

- [ ] Provocado y entendido errores de sintaxis
- [ ] Entendido warnings de variables no usadas
- [ ] Experimentado con mutabilidad
- [ ] Probado shadowing
- [ ] Explorado tipos básicos
- [ ] Creado funciones simples
- [ ] Realizado experimento libre

---

## 🎓 Lo que Aprendiste

1. El compilador de Rust es tu amigo
2. Los mensajes de error son muy descriptivos
3. Siempre lee las sugerencias `help:`
4. Las variables son inmutables por defecto
5. Rust tiene tipado estático fuerte
6. Shadowing permite reusar nombres

---

## 📸 Evidencia

Toma capturas de pantalla de:
1. Un error que provocaste y cómo lo solucionaste
2. Tu experimento libre funcionando

---

**🎉 ¡Has completado todas las prácticas de la Semana 01!**

**Siguiente paso**: Proyecto Semanal
