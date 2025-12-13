# 📚 Introducción a Closures

![Introducción a Closures](../0-assets/01-introduccion-closures.svg)

## 🎯 Objetivos de Aprendizaje

- Entender qué son los closures
- Dominar la sintaxis de closures
- Usar closures como argumentos y valores de retorno

---

## 🤔 ¿Qué es un Closure?

Un **closure** es una función anónima que puede:
1. Capturar variables de su entorno
2. Ser almacenada en variables
3. Ser pasada como argumento

```rust
// Función tradicional
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

// Closure equivalente
let sumar = |a, b| a + b;

// Ambas se usan igual
let resultado = sumar(2, 3);  // 5
```

---

## 📝 Sintaxis de Closures

### Forma Básica

```rust
|parámetros| expresión
```

### Ejemplos de Sintaxis

```rust
// Sin parámetros
let saludar = || println!("¡Hola!");

// Un parámetro
let doble = |x| x * 2;

// Múltiples parámetros
let suma = |a, b| a + b;

// Con bloque de código
let procesar = |x| {
    let temp = x * 2;
    temp + 1
};

// Con tipos explícitos
let suma_tipada = |a: i32, b: i32| -> i32 {
    a + b
};
```

### Inferencia de Tipos

Rust infiere los tipos del closure basándose en su uso:

```rust
let duplicar = |x| x * 2;

// Rust infiere tipos del primer uso
let a: i32 = duplicar(5);   // x es i32

// ❌ Error: ya está fijado como i32
// let b: f64 = duplicar(5.0);
```

---

## 🆚 Closures vs Funciones

| Aspecto | Función | Closure |
|---------|---------|---------|
| Nombre | Requerido | Opcional (anónimo) |
| Tipos | Explícitos | Inferidos |
| Captura entorno | No | Sí |
| Sintaxis | `fn name()` | `\|args\| body` |

```rust
// Función: NO puede capturar entorno
fn multiplicar_por_dos(x: i32) -> i32 {
    x * 2
}

// Closure: PUEDE capturar entorno
let factor = 3;
let multiplicar_por_factor = |x| x * factor;  // Captura 'factor'
```

---

## 📦 Closures como Valores

### Almacenar en Variables

```rust
let operacion = |x, y| x + y;
let resultado = operacion(10, 20);
```

### Almacenar en Estructuras

```rust
struct Calculadora<F>
where
    F: Fn(i32, i32) -> i32,
{
    operacion: F,
}

let calc = Calculadora {
    operacion: |a, b| a * b,
};
```

---

## 🔄 Closures como Argumentos

Los closures brillan cuando se pasan como argumentos:

```rust
fn aplicar_a_cinco<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5)
}

// Usando con closure
let resultado = aplicar_a_cinco(|x| x * 2);  // 10

// Usando con función
fn doble(x: i32) -> i32 { x * 2 }
let resultado2 = aplicar_a_cinco(doble);     // 10
```

### Ejemplos en la Biblioteca Estándar

```rust
let numeros = vec![1, 2, 3, 4, 5];

// map() recibe un closure
let dobles: Vec<i32> = numeros.iter()
    .map(|x| x * 2)
    .collect();

// filter() recibe un closure (predicado)
let pares: Vec<&i32> = numeros.iter()
    .filter(|x| *x % 2 == 0)
    .collect();

// sort_by() recibe un closure
let mut nums = vec![3, 1, 4, 1, 5];
nums.sort_by(|a, b| b.cmp(a));  // Orden descendente
```

---

## ↩️ Closures como Valor de Retorno

Para retornar closures, usamos `impl Fn`:

```rust
fn crear_sumador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let sumar_5 = crear_sumador(5);
let resultado = sumar_5(10);  // 15
```

### ¿Por qué `move`?

```rust
fn crear_saludo(nombre: String) -> impl Fn() {
    // Sin move: 'nombre' no viviría lo suficiente
    // Con move: el closure toma ownership
    move || println!("¡Hola, {}!", nombre)
}

let saludar = crear_saludo(String::from("Rust"));
saludar();  // ¡Hola, Rust!
```

---

## 💡 Casos de Uso Comunes

### 1. Callbacks

```rust
fn procesar_con_callback<F>(datos: &[i32], callback: F)
where
    F: Fn(i32),
{
    for &item in datos {
        callback(item);
    }
}

procesar_con_callback(&[1, 2, 3], |x| {
    println!("Procesando: {}", x);
});
```

### 2. Configuración Diferida

```rust
struct Config<F>
where
    F: Fn() -> String,
{
    generador_url: F,
}

let config = Config {
    generador_url: || String::from("https://api.ejemplo.com"),
};
```

### 3. Estrategias Intercambiables

```rust
fn ordenar_usuarios<F>(usuarios: &mut Vec<String>, criterio: F)
where
    F: Fn(&String, &String) -> std::cmp::Ordering,
{
    usuarios.sort_by(criterio);
}

let mut usuarios = vec!["carlos".into(), "ana".into(), "bob".into()];

// Orden alfabético
ordenar_usuarios(&mut usuarios, |a, b| a.cmp(b));

// Orden por longitud
ordenar_usuarios(&mut usuarios, |a, b| a.len().cmp(&b.len()));
```

---

## ⚠️ Errores Comunes

### 1. Tipos Inconsistentes

```rust
let closure = |x| x * 2;

let a: i32 = closure(5);
// let b: i64 = closure(5);  // ❌ Error: tipo ya fijado
```

### 2. Olvidar que Closures son Únicos

```rust
// Cada closure tiene un tipo único
let f1 = |x: i32| x + 1;
let f2 = |x: i32| x + 1;

// f1 y f2 son del mismo "tipo conceptual" pero tipos distintos
// No puedes hacer: let f = if condition { f1 } else { f2 };
```

### 3. Captura No Deseada

```rust
let mut contador = 0;
let incrementar = || contador += 1;  // Captura &mut contador

// ❌ No puedes usar contador mientras el closure existe
// println!("{}", contador);
```

---

## 🎯 Resumen

| Concepto | Descripción |
|----------|-------------|
| Closure | Función anónima que captura entorno |
| Sintaxis | `\|params\| expresión` |
| Inferencia | Tipos se infieren del uso |
| Como argumento | `F: Fn(T) -> U` |
| Como retorno | `impl Fn(T) -> U` |
| `move` | Transfiere ownership al closure |

---

## 🔗 Siguiente

[02 - Captura de Entorno](02-captura-entorno.md)
