# 🦀 Semana 12: Closures e Iteradores

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Tema** | Closures e Iteradores |
| **Duración** | 4 horas |
| **Nivel** | Avanzado |
| **Prerrequisitos** | Semanas 1-11 (Lifetimes, Traits, Generics) |

---

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Crear y usar closures** con diferentes modos de captura
2. **Distinguir entre Fn, FnMut y FnOnce**
3. **Dominar la API de iteradores** de Rust
4. **Crear iteradores personalizados**
5. **Combinar closures e iteradores** para código expresivo

---

## 📚 Contenido

### 1. Teoría (1.5 horas)

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-introduccion-closures.md](1-teoria/01-introduccion-closures.md) | Qué son y sintaxis | 20 min |
| [02-captura-entorno.md](1-teoria/02-captura-entorno.md) | Modos de captura | 20 min |
| [03-fn-traits.md](1-teoria/03-fn-traits.md) | Fn, FnMut, FnOnce | 20 min |
| [04-iteradores.md](1-teoria/04-iteradores.md) | El trait Iterator | 15 min |
| [05-iteradores-avanzados.md](1-teoria/05-iteradores-avanzados.md) | Adaptadores y colectores | 15 min |

### 2. Práctica (1.5 horas)

| Ejercicio | Tema | Puntos |
|-----------|------|--------|
| [practica-01](2-practica/practica-01-closures-basicos/) | Closures básicos | 15 |
| [practica-02](2-practica/practica-02-captura/) | Modos de captura | 15 |
| [practica-03](2-practica/practica-03-fn-traits/) | Fn, FnMut, FnOnce | 20 |
| [practica-04](2-practica/practica-04-iteradores/) | Iteradores | 20 |

### 3. Proyecto (1 hora)

| Proyecto | Descripción | Puntos |
|----------|-------------|--------|
| [proyecto-pipeline](3-proyecto/proyecto-pipeline/) | Pipeline de procesamiento de datos | 30 |

---

## 🗺️ Mapa Conceptual

```
                    CLOSURES E ITERADORES
                           │
           ┌───────────────┴───────────────┐
           │                               │
      CLOSURES                        ITERADORES
           │                               │
    ┌──────┼──────┐              ┌─────────┼─────────┐
    │      │      │              │         │         │
  Sintaxis │   Traits         Trait    Adaptadores  Collect
    │      │      │           Iterator      │         │
 |x| x+1   │   ┌──┴──┐           │     map/filter  Vec/HashMap
           │   │     │           │     take/skip   String/Sum
       Captura │     │       next()    chain/zip
           │   Fn  FnMut       │
    ┌──────┴──────┐  │     into_iter()
    │      │      │  │     iter()
  move   &T    &mut T │     iter_mut()
                FnOnce
```

---

## 🔑 Conceptos Clave

### Closures

```rust
// Sintaxis básica
let suma = |a, b| a + b;

// Con tipos explícitos
let suma: fn(i32, i32) -> i32 = |a, b| a + b;

// Capturando entorno
let factor = 2;
let multiplicar = |x| x * factor;  // Captura 'factor'
```

### Los Tres Traits Fn

```rust
// FnOnce - consume valores capturados (una vez)
let s = String::from("hello");
let consume = || drop(s);  // FnOnce

// FnMut - modifica valores capturados
let mut count = 0;
let mut incrementar = || count += 1;  // FnMut

// Fn - solo lee valores capturados
let x = 5;
let leer = || println!("{}", x);  // Fn
```

### Iteradores

```rust
let nums = vec![1, 2, 3, 4, 5];

// Métodos de iterador
let resultado: Vec<i32> = nums.iter()
    .filter(|&&x| x % 2 == 0)  // Solo pares
    .map(|&x| x * 2)           // Duplicar
    .collect();                 // Recolectar

// resultado = [4, 8]
```

---

## 📊 Comparación: Loops vs Iteradores

```rust
// Estilo imperativo (loop)
let mut suma = 0;
for num in &numeros {
    if *num > 0 {
        suma += num;
    }
}

// Estilo funcional (iteradores)
let suma: i32 = numeros.iter()
    .filter(|&&n| n > 0)
    .sum();
```

| Aspecto | Loops | Iteradores |
|---------|-------|------------|
| Legibilidad | Más verboso | Más conciso |
| Performance | Manual | Zero-cost abstraction |
| Composición | Difícil | Fácil encadenar |
| Lazy | No | Sí |

---

## ⚠️ Errores Comunes

### 1. Confundir FnOnce con Fn

```rust
// ❌ Error: closure es FnOnce
fn llamar_dos_veces<F: Fn()>(f: F) {
    f(); f();
}
let s = String::from("hi");
llamar_dos_veces(|| drop(s));  // FnOnce, no Fn!

// ✅ Solución: usar FnOnce o cambiar closure
fn llamar_una_vez<F: FnOnce()>(f: F) {
    f();
}
```

### 2. No consumir iterador

```rust
// ❌ Iterador no hace nada (lazy)
vec![1, 2, 3].iter().map(|x| println!("{}", x));

// ✅ Consumir con for_each o collect
vec![1, 2, 3].iter().for_each(|x| println!("{}", x));
```

### 3. Olvidar move en threads

```rust
// ❌ Error: closure puede outlive datos
let data = vec![1, 2, 3];
std::thread::spawn(|| println!("{:?}", data));

// ✅ Usar move
std::thread::spawn(move || println!("{:?}", data));
```

---

## 🛠️ Herramientas de Iteradores

### Adaptadores (lazy)

| Método | Descripción |
|--------|-------------|
| `map(f)` | Transforma cada elemento |
| `filter(p)` | Filtra por predicado |
| `take(n)` | Toma primeros n |
| `skip(n)` | Salta primeros n |
| `chain(iter)` | Concatena iteradores |
| `zip(iter)` | Combina en pares |
| `enumerate()` | Añade índices |
| `flatten()` | Aplana iteradores anidados |

### Consumidores (eager)

| Método | Descripción |
|--------|-------------|
| `collect()` | Recolecta en colección |
| `sum()` | Suma elementos |
| `count()` | Cuenta elementos |
| `fold(init, f)` | Reduce a un valor |
| `for_each(f)` | Ejecuta efecto |
| `find(p)` | Busca elemento |
| `any(p)` / `all(p)` | Tests booleanos |

---

## 📁 Estructura de la Semana

```
semana-12/
├── README.md
├── RUBRICA_EVALUACION.md
├── 0-assets/
│   ├── 01-introduccion-closures.svg
│   ├── 02-captura-entorno.svg
│   ├── 03-fn-traits.svg
│   ├── 04-iteradores.svg
│   └── 05-iteradores-avanzados.svg
├── 1-teoria/
│   ├── 01-introduccion-closures.md
│   ├── 02-captura-entorno.md
│   ├── 03-fn-traits.md
│   ├── 04-iteradores.md
│   └── 05-iteradores-avanzados.md
├── 2-practica/
│   ├── practica-01-closures-basicos/
│   ├── practica-02-captura/
│   ├── practica-03-fn-traits/
│   └── practica-04-iteradores/
├── 3-proyecto/
│   └── proyecto-pipeline/
├── 4-recursos/
│   └── RECURSOS.md
└── 5-glosario/
    └── GLOSARIO.md
```

---

## 🔗 Navegación

| Anterior | Índice | Siguiente |
|----------|--------|-----------|
| [Semana 11: Lifetimes](../semana-11/) | [Bootcamp](../BOOTCAMP-COMPLETO.md) | [Semana 13: Smart Pointers](../semana-13/) |

---

## 📖 Recursos Adicionales

- [The Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust by Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
- [Iterator Cheat Sheet](https://danielkeep.github.io/itercheat_baked.html)
