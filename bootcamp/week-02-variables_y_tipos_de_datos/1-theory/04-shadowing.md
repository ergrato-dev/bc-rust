# 📖 Shadowing

## 🎯 Objetivo de Aprendizaje

Entender qué es shadowing, cuándo usarlo y cómo difiere de la mutabilidad.

---

## 📚 Contenido

### ¿Qué es Shadowing?

**Shadowing** (sombreado) ocurre cuando declaras una nueva variable con el mismo nombre que una anterior:

```rust
fn main() {
    let x = 5;
    let x = x + 1;      // Shadowing: nueva variable 'x'
    let x = x * 2;      // Shadowing de nuevo
    
    println!("x = {}", x);  // 12
}
```

![Diagrama Shadowing](../0-assets/04-shadowing.svg)

---

## 🔄 Shadowing vs Mutabilidad

| Característica | Shadowing | Mutabilidad |
|----------------|-----------|-------------|
| Sintaxis | `let x = ...` (de nuevo) | `let mut x = ...` |
| Crea nueva variable | ✅ Sí | ❌ No |
| Puede cambiar tipo | ✅ Sí | ❌ No |
| Requiere `let` | ✅ Sí | ❌ No (después de declarar) |

### Ejemplo Comparativo

```rust
fn main() {
    // SHADOWING - Crea nueva variable
    let espacios = "   ";        // &str
    let espacios = espacios.len(); // usize (¡diferente tipo!)
    println!("Espacios: {}", espacios);
    
    // MUTABILIDAD - Misma variable
    let mut contador = 0;        // i32
    contador = 1;                // i32 (mismo tipo)
    // contador = "uno";         // ❌ Error: no puedes cambiar tipo
    println!("Contador: {}", contador);
}
```

---

## ✅ Casos de Uso del Shadowing

### 1. Transformaciones de Tipo

```rust
fn main() {
    let input = "42";           // &str
    let input: i32 = input.parse().unwrap();  // i32
    let input = input * 2;      // i32
    
    println!("Resultado: {}", input);  // 84
}
```

### 2. Procesar Datos en Pasos

```rust
fn main() {
    let datos = "  Rust  ";
    let datos = datos.trim();           // Quitar espacios
    let datos = datos.to_uppercase();   // Mayúsculas
    let datos = format!("¡{}!", datos); // Formatear
    
    println!("{}", datos);  // ¡RUST!
}
```

### 3. Simplificar Nombres

```rust
fn main() {
    let configuracion_del_sistema_muy_larga = obtener_config();
    
    // Simplificar en scope local
    let config = configuracion_del_sistema_muy_larga;
    
    println!("Modo: {}", config);
}

fn obtener_config() -> String {
    String::from("produccion")
}
```

### 4. Hacer Inmutable Después de Construir

```rust
fn main() {
    let mut nombre = String::new();
    nombre.push_str("Juan");
    nombre.push_str(" ");
    nombre.push_str("Pérez");
    
    // "Congelar" la variable
    let nombre = nombre;  // Ahora es inmutable
    
    // nombre.push_str("!");  // ❌ Error
    
    println!("Nombre: {}", nombre);
}
```

---

## ⚠️ Shadowing en Diferentes Scopes

El shadowing respeta los scopes (bloques `{}`):

```rust
fn main() {
    let x = 5;
    
    {
        let x = 10;  // Shadowing SOLO dentro de este bloque
        println!("Dentro del bloque: x = {}", x);  // 10
    }
    
    println!("Fuera del bloque: x = {}", x);  // 5 (el original)
}
```

**Output:**
```
Dentro del bloque: x = 10
Fuera del bloque: x = 5
```

---

## 🚫 Cuándo NO Usar Shadowing

### 1. Si Confunde el Código

```rust
// ❌ Confuso - muchas re-declaraciones
fn main() {
    let x = 1;
    let x = x + 1;
    let x = x * 2;
    let x = x - 1;
    let x = x / 2;
    println!("{}", x);  // ¿Cuánto es?
}

// ✅ Más claro con nombres descriptivos
fn main() {
    let inicial = 1;
    let incrementado = inicial + 1;
    let duplicado = incrementado * 2;
    let resultado = duplicado - 1;
    println!("{}", resultado);
}
```

### 2. Si Solo Necesitas Mutar

```rust
// ❌ Innecesario
let mut suma = 0;
let suma = suma + 10;  // Podrías solo mutar

// ✅ Más simple
let mut suma = 0;
suma += 10;
```

---

## 🧩 Shadowing con Diferentes Tipos

Una de las ventajas únicas del shadowing:

```rust
fn main() {
    // String → número de caracteres
    let texto = "Hola";           // &str
    let texto = texto.len();      // usize
    println!("Longitud: {}", texto);
    
    // Número → String
    let numero = 42;              // i32
    let numero = numero.to_string();  // String
    println!("Como texto: {}", numero);
    
    // Parsing
    let edad = "25";              // &str
    let edad: u32 = edad.parse().unwrap();  // u32
    println!("Edad + 1: {}", edad + 1);
}
```

---

## 🧪 Ejercicio

¿Cuál es el output de este código?

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("Inner: {}", x);
    }
    
    println!("Outer: {}", x);
}
```

<details>
<summary>Ver respuesta</summary>

```
Inner: 12
Outer: 6
```

**Explicación:**
1. `x = 5`
2. `x = 6` (5 + 1, shadowing)
3. Dentro del bloque: `x = 12` (6 * 2, shadowing local)
4. Fuera del bloque: `x` vuelve a ser 6 (el del scope exterior)

</details>

---

## 📌 Resumen

| Concepto | Descripción |
|----------|-------------|
| Shadowing | Re-declarar variable con `let` |
| Cambia tipo | ✅ Sí, a diferencia de `mut` |
| Nueva variable | Crea nueva, no modifica la anterior |
| Scope | Respeta los bloques `{}` |
| Uso típico | Transformaciones, parsing, simplificar nombres |

---

## 🔗 Navegación

[← Tipos Primitivos](./03-tipos-primitivos.md) | [Constantes →](./05-constantes.md)
