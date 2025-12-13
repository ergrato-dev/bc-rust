# 🦀 Semana 10: Generics

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Tema** | Generics (Tipos Genéricos) |
| **Duración** | 4 horas |
| **Nivel** | Intermedio |
| **Prerrequisitos** | Semana 09 (Traits Básicos) |

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Comprender** qué son los tipos genéricos y por qué son útiles
2. **Definir** funciones genéricas con parámetros de tipo
3. **Crear** structs y enums genéricos
4. **Aplicar** trait bounds para restringir tipos genéricos
5. **Usar** cláusulas `where` para bounds complejos
6. **Entender** la monomorphization y su impacto en el rendimiento

## 📚 Contenido

### 1. Teoría (1.5 horas)

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-introduccion-generics.md](1-teoria/01-introduccion-generics.md) | ¿Qué son los Generics? | 20 min |
| [02-funciones-genericas.md](1-teoria/02-funciones-genericas.md) | Funciones Genéricas | 20 min |
| [03-structs-genericos.md](1-teoria/03-structs-genericos.md) | Structs y Enums Genéricos | 20 min |
| [04-trait-bounds.md](1-teoria/04-trait-bounds.md) | Trait Bounds en Generics | 20 min |
| [05-generics-avanzados.md](1-teoria/05-generics-avanzados.md) | Generics Avanzados | 20 min |

### 2. Práctica (1.5 horas)

| Ejercicio | Tema | Duración |
|-----------|------|----------|
| [practica-01](2-practica/practica-01-funciones-genericas/) | Funciones Genéricas | 20 min |
| [practica-02](2-practica/practica-02-structs-genericos/) | Structs Genéricos | 25 min |
| [practica-03](2-practica/practica-03-trait-bounds/) | Trait Bounds | 25 min |
| [practica-04](2-practica/practica-04-generics-avanzados/) | Generics Avanzados | 20 min |

### 3. Proyecto (1 hora)

| Proyecto | Descripción |
|----------|-------------|
| [proyecto-contenedor](3-proyecto/proyecto-contenedor/) | Contenedor genérico con operaciones |

## 🗺️ Mapa Conceptual

```
                    GENERICS
                       │
        ┌──────────────┼──────────────┐
        │              │              │
   Funciones      Structs/Enums   Trait Bounds
        │              │              │
   fn foo<T>()    struct S<T>    <T: Trait>
        │              │              │
        └──────────────┼──────────────┘
                       │
              Monomorphization
                       │
            Código especializado
            para cada tipo concreto
```

## 💡 Conceptos Clave

### ¿Por qué Generics?

```rust
// ❌ Sin generics: duplicación de código
fn mayor_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn mayor_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

// ✅ Con generics: código reutilizable
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### Sintaxis Básica

```rust
// Función genérica
fn identidad<T>(valor: T) -> T {
    valor
}

// Struct genérico
struct Punto<T> {
    x: T,
    y: T,
}

// Enum genérico
enum Resultado<T, E> {
    Ok(T),
    Err(E),
}

// Método genérico
impl<T> Punto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

### Trait Bounds

```rust
// Sintaxis con :
fn imprimir<T: Display>(valor: T) {
    println!("{}", valor);
}

// Múltiples bounds con +
fn comparar<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("{} es mayor", a);
    }
}

// Cláusula where
fn procesar<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Debug + Default,
{
    // ...
}
```

## ⚠️ Errores Comunes

### 1. Olvidar Trait Bounds

```rust
// ❌ Error: T no implementa Display
fn imprimir<T>(valor: T) {
    println!("{}", valor);
}

// ✅ Correcto: agregar bound
fn imprimir<T: Display>(valor: T) {
    println!("{}", valor);
}
```

### 2. Tipos Incompatibles en Struct

```rust
struct Punto<T> {
    x: T,
    y: T,  // ⚠️ x e y deben ser del mismo tipo
}

// ❌ Error
let p = Punto { x: 5, y: 4.0 };

// ✅ Solución: dos parámetros de tipo
struct Punto<T, U> {
    x: T,
    y: U,
}
```

### 3. Bounds en impl vs en Método

```rust
struct Contenedor<T> {
    valor: T,
}

// Bounds en impl: aplican a TODOS los métodos
impl<T: Clone> Contenedor<T> {
    fn clonar(&self) -> T {
        self.valor.clone()
    }
}

// Bounds en método: solo para ESE método
impl<T> Contenedor<T> {
    fn mostrar(&self) where T: Display {
        println!("{}", self.valor);
    }
}
```

## 🔧 Herramientas

### Turbofish `::<>`

```rust
// Especificar tipo explícitamente
let numeros: Vec<i32> = Vec::new();
let numeros = Vec::<i32>::new();  // Turbofish

let resultado = "42".parse::<i32>().unwrap();
```

### Inferencia de Tipos

```rust
// El compilador infiere T = i32
let x = identidad(5);

// El compilador infiere T = &str
let s = identidad("hola");
```

## 📊 Distribución del Tiempo

```
┌─────────────────────────────────────────────────────┐
│                    4 HORAS                          │
├─────────────┬─────────────┬─────────────┬───────────┤
│   Teoría    │  Práctica   │  Proyecto   │  Buffer   │
│   1.5h      │    1.5h     │    0.75h    │   0.25h   │
│   37.5%     │    37.5%    │    18.75%   │   6.25%   │
└─────────────┴─────────────┴─────────────┴───────────┘
```

## 📖 Recursos Adicionales

- [The Rust Book - Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [Rust Reference - Generic Parameters](https://doc.rust-lang.org/reference/items/generics.html)

## ✅ Checklist de Aprendizaje

- [ ] Entiendo qué son los tipos genéricos
- [ ] Puedo definir funciones genéricas
- [ ] Puedo crear structs y enums genéricos
- [ ] Sé usar trait bounds con `<T: Trait>`
- [ ] Puedo usar cláusulas `where` para bounds complejos
- [ ] Entiendo qué es la monomorphization
- [ ] Sé cuándo usar genéricos vs trait objects

## 🔗 Navegación

| Anterior | Índice | Siguiente |
|----------|--------|-----------|
| [Semana 09: Traits](../semana-09/) | [Bootcamp](../BOOTCAMP-COMPLETO.md) | [Semana 11: Lifetimes](../semana-11/) |
