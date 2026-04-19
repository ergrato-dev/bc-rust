# Práctica 01: Box y Tipos Recursivos

## 🎯 Objetivos

- Usar `Box<T>` para almacenar datos en el heap
- Implementar tipos recursivos (lista enlazada, árbol)
- Entender cuándo es necesario usar Box

## 📋 Ejercicios

### Ejercicio 1: Lista Enlazada

Implementa los métodos de una lista enlazada simple:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self { ... }
    fn prepend(self, value: i32) -> Self { ... }
    fn len(&self) -> usize { ... }
    fn sum(&self) -> i32 { ... }
}
```

### Ejercicio 2: Árbol Binario

Implementa un árbol binario con métodos para calcular profundidad y suma:

```rust
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self { ... }
    fn with_children(...) -> Self { ... }
    fn depth(&self) -> usize { ... }
    fn sum(&self) -> i32 { ... }
}
```

### Ejercicio 3: Expresiones Matemáticas

Implementa un evaluador de expresiones:

```rust
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i32 { ... }
}
```

## 🔧 Ejecución

```bash
# Ejecutar
cargo run -p practica-01-box

# Ejecutar tests
cargo test -p practica-01-box
```

## ✅ Tests Esperados

- `test_list_empty`: Lista vacía tiene longitud 0
- `test_list_prepend`: Lista con elementos tiene longitud correcta
- `test_tree_leaf`: Nodo hoja tiene profundidad 1
- `test_tree_with_children`: Árbol tiene profundidad y suma correctas
- `test_expr_eval`: Expresión `(2 + 3) * 4 = 20`

## 💡 Pistas

1. **Lista**: Usa recursión en `len()` y `sum()`
2. **Árbol**: `Option::map()` convierte `Option<T>` a `Option<Box<T>>`
3. **Expresiones**: El `match` debe evaluar recursivamente

## 📚 Recursos

- [Box en The Rust Book](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Teoría: Box y Heap](../../1-teoria/01-box.md)
