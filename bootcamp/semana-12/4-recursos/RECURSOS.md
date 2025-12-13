# Recursos - Semana 12: Closures e Iteradores

## 📚 Documentación Oficial

### Closures
- [The Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
- [std::ops::Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html)
- [std::ops::FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html)
- [std::ops::FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)

### Iteradores
- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust by Example - Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
- [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [std::iter module](https://doc.rust-lang.org/std/iter/index.html)

## 🎥 Videos Recomendados

- [Closures in Rust - Let's Get Rusty](https://www.youtube.com/watch?v=dHkzSZnYXmk)
- [Iterators in Rust - Let's Get Rusty](https://www.youtube.com/watch?v=4GcKrj4By8k)
- [Rust Iterators - Jon Gjengset](https://www.youtube.com/watch?v=yozQ9C69pNs)

## 📖 Artículos

- [Rust Closures in Depth](https://huonw.github.io/blog/2015/05/finding-closure-in-rust/)
- [Effectively Using Iterators In Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
- [Rust Iterator Cheat Sheet](https://danielkeep.github.io/itercheat_baked.html)

## 🔧 Métodos de Iterator Más Usados

### Adaptadores (Lazy)
| Método | Descripción | Ejemplo |
|--------|-------------|---------|
| `map` | Transforma elementos | `.map(\|x\| x * 2)` |
| `filter` | Filtra elementos | `.filter(\|x\| x > 0)` |
| `take` | Toma n primeros | `.take(5)` |
| `skip` | Salta n primeros | `.skip(2)` |
| `enumerate` | Añade índice | `.enumerate()` |
| `zip` | Combina iteradores | `.zip(otro)` |
| `chain` | Concatena iteradores | `.chain(otro)` |
| `flatten` | Aplana iteradores anidados | `.flatten()` |
| `flat_map` | map + flatten | `.flat_map(\|x\| x.iter())` |
| `peekable` | Permite ver siguiente | `.peekable()` |
| `rev` | Invierte orden | `.rev()` |
| `cycle` | Repite infinitamente | `.cycle()` |

### Consumidores
| Método | Descripción | Ejemplo |
|--------|-------------|---------|
| `collect` | A colección | `.collect::<Vec<_>>()` |
| `fold` | Reduce con acumulador | `.fold(0, \|a, x\| a + x)` |
| `reduce` | Fold sin valor inicial | `.reduce(\|a, x\| a + x)` |
| `sum` | Suma elementos | `.sum::<i32>()` |
| `product` | Producto de elementos | `.product::<i32>()` |
| `count` | Cuenta elementos | `.count()` |
| `find` | Busca primer match | `.find(\|x\| x > 5)` |
| `position` | Índice del primer match | `.position(\|x\| x > 5)` |
| `any` | ¿Alguno cumple? | `.any(\|x\| x > 5)` |
| `all` | ¿Todos cumplen? | `.all(\|x\| x > 0)` |
| `max` / `min` | Máximo / mínimo | `.max()` |
| `for_each` | Ejecuta por cada uno | `.for_each(\|x\| println!("{}", x))` |

## 💡 Patrones Comunes

### Transformar y Coleccionar
```rust
let resultado: Vec<_> = datos
    .iter()
    .filter(|x| condicion(x))
    .map(|x| transformar(x))
    .collect();
```

### Acumular con Fold
```rust
let suma = numeros.iter().fold(0, |acc, x| acc + x);
```

### Buscar con Find
```rust
if let Some(encontrado) = datos.iter().find(|x| x.id == buscado) {
    // usar encontrado
}
```

### Combinar con Zip
```rust
let pares: Vec<_> = nombres.iter()
    .zip(edades.iter())
    .collect();
```

### Aplanar Estructuras
```rust
let todos: Vec<_> = matriz
    .iter()
    .flatten()
    .collect();
```

## 🎯 Ejercicios Adicionales

1. **Rustlings**: ejercicios `iterators1` a `iterators5`
2. **Exercism**: track de Rust, ejercicios con iteradores
3. **Advent of Code**: muchos problemas se resuelven elegantemente con iteradores
