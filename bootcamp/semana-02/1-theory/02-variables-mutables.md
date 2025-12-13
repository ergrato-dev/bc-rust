# 📖 Variables Mutables

## 🎯 Objetivo de Aprendizaje

Aprender cuándo y cómo usar variables mutables con `let mut`.

---

## 📚 Contenido

### Declarando Variables Mutables

Para permitir que una variable cambie, usa la palabra clave `mut`:

```rust
fn main() {
    let mut x = 5;      // ✅ Mutable
    println!("x = {}", x);
    
    x = 10;             // ✅ Ahora sí podemos cambiar
    println!("x = {}", x);
}
```

**Output:**
```
x = 5
x = 10
```

![Diagrama Mutabilidad](../0-assets/02-mutabilidad.svg)

### Sintaxis

```rust
let mut nombre_variable = valor;
//  ↑
//  └── Palabra clave para mutabilidad
```

---

## 🔄 Inmutable vs Mutable

| Característica | `let x` | `let mut x` |
|---------------|---------|-------------|
| Puede cambiar | ❌ No | ✅ Sí |
| Seguridad | Mayor | Menor |
| Uso típico | Valores fijos | Contadores, acumuladores |
| Performance | Puede optimizarse más | Menos optimizable |

### Ejemplo Comparativo

```rust
fn main() {
    // Inmutable - valor calculado una vez
    let pi = 3.14159;
    
    // Mutable - valor que cambia
    let mut contador = 0;
    
    contador = contador + 1;  // 1
    contador = contador + 1;  // 2
    contador = contador + 1;  // 3
    
    println!("Pi: {}", pi);
    println!("Contador: {}", contador);
}
```

---

## 🎯 Cuándo Usar Mutabilidad

### ✅ Usa `mut` cuando:

1. **Contadores**
```rust
let mut intentos = 0;
intentos += 1;
```

2. **Acumuladores**
```rust
let mut suma = 0;
suma += 10;
suma += 20;
```

3. **Estado que cambia**
```rust
let mut conectado = false;
// ... después de conectar
conectado = true;
```

4. **Buffers/Colecciones**
```rust
let mut nombres = Vec::new();
nombres.push("Ana");
nombres.push("Luis");
```

### ❌ Evita `mut` cuando:

1. **Valores calculados una vez**
```rust
// ❌ No necesita mut
let mut resultado = 5 + 10;

// ✅ Mejor así
let resultado = 5 + 10;
```

2. **Puedes usar shadowing**
```rust
// ❌ Usando mut
let mut x = "42";
x = "modificado";  // Error! No puedes cambiar tipo

// ✅ Usando shadowing
let x = "42";
let x = x.parse::<i32>().unwrap();  // Nuevo tipo
```

---

## ⚠️ Errores Comunes

### Error 1: Olvidar `mut`

```rust
fn main() {
    let contador = 0;
    contador += 1;  // ❌ Error!
}
```

**Solución:**
```rust
let mut contador = 0;
```

### Error 2: `mut` innecesario

```rust
fn main() {
    let mut x = 5;  // Warning: variable does not need to be mutable
    println!("{}", x);
}
```

**Warning:**
```
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 5;
  |         ----^
  |         |
  |         help: remove this `mut`
```

### Error 3: Cambiar el tipo

```rust
fn main() {
    let mut x = 5;
    x = "texto";  // ❌ Error! No puedes cambiar el tipo
}
```

> 💡 `mut` permite cambiar el **valor**, no el **tipo**. Para cambiar tipo, usa shadowing.

---

## 🔧 Operadores de Asignación Compuesta

Con variables mutables, puedes usar operadores compuestos:

```rust
fn main() {
    let mut x = 10;
    
    x += 5;   // x = x + 5  → 15
    x -= 3;   // x = x - 3  → 12
    x *= 2;   // x = x * 2  → 24
    x /= 4;   // x = x / 4  → 6
    x %= 4;   // x = x % 4  → 2
    
    println!("x = {}", x);  // 2
}
```

---

## 🧪 Ejercicio Práctico

Crea un contador que:
1. Empiece en 0
2. Incremente 3 veces
3. Imprima cada valor

<details>
<summary>Ver solución</summary>

```rust
fn main() {
    let mut contador = 0;
    println!("Inicio: {}", contador);
    
    contador += 1;
    println!("Después de +1: {}", contador);
    
    contador += 1;
    println!("Después de +1: {}", contador);
    
    contador += 1;
    println!("Final: {}", contador);
}
```

**Output:**
```
Inicio: 0
Después de +1: 1
Después de +1: 2
Final: 3
```

</details>

---

## 💡 Patrón: Mutable a Inmutable

A veces construyes un valor y luego lo "congelas":

```rust
fn main() {
    // Fase de construcción (mutable)
    let mut config = String::new();
    config.push_str("modo=");
    config.push_str("produccion");
    
    // "Congelar" - ya no necesita cambiar
    let config = config;  // Ahora es inmutable (shadowing)
    
    // config.push_str("algo");  // ❌ Ya no compila
    
    println!("Config: {}", config);
}
```

---

## 📌 Resumen

| Concepto | Descripción |
|----------|-------------|
| `let mut x = 5;` | Variable mutable |
| `mut` | Palabra clave para permitir cambios |
| Cambio de valor | ✅ Permitido |
| Cambio de tipo | ❌ No permitido (usa shadowing) |
| Operadores | `+=`, `-=`, `*=`, `/=`, `%=` |

---

## 🔗 Navegación

[← Variables Inmutables](./01-variables-inmutables.md) | [Tipos Primitivos →](./03-tipos-primitivos.md)
