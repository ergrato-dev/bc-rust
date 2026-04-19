# Glosario - Semana 12: Closures e Iteradores

## Closures

### Closure
Función anónima que puede capturar variables de su entorno. Se define con la sintaxis `|args| expresión` o `|args| { bloque }`.

```rust
let suma = |a, b| a + b;
let resultado = suma(2, 3); // 5
```

### Captura del Entorno
Mecanismo por el cual un closure accede a variables definidas fuera de él. Rust determina automáticamente el modo de captura.

```rust
let factor = 2;
let multiplicar = |x| x * factor; // captura 'factor'
```

### Captura por Referencia (`&T`)
El closure toma prestada la variable inmutablemente. Permite múltiples llamadas y la variable original sigue disponible.

### Captura por Referencia Mutable (`&mut T`)
El closure toma prestada la variable mutablemente. Puede modificarla, pero requiere que el closure sea `mut`.

### Captura por Valor (`move`)
El closure toma ownership de la variable. Usa la keyword `move` para forzar este comportamiento.

```rust
let datos = vec![1, 2, 3];
let closure = move || datos.len(); // 'datos' movido al closure
```

### `Fn`
Trait para closures que capturan por referencia inmutable. Pueden llamarse múltiples veces sin modificar su entorno.

### `FnMut`
Trait para closures que capturan por referencia mutable. Pueden modificar su entorno y llamarse múltiples veces.

### `FnOnce`
Trait para closures que pueden consumir su entorno. Solo pueden llamarse una vez si consumen valores capturados.

### Jerarquía de Fn Traits
`Fn` ⊂ `FnMut` ⊂ `FnOnce`. Todo `Fn` es también `FnMut` y `FnOnce`.

## Iteradores

### Iterator
Trait que define una secuencia de valores. Requiere implementar `next()` que retorna `Option<Item>`.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Item
Tipo asociado de `Iterator` que representa el tipo de elementos que produce el iterador.

### `iter()`
Método que crea un iterador de referencias inmutables (`&T`) sobre una colección.

### `iter_mut()`
Método que crea un iterador de referencias mutables (`&mut T`) sobre una colección.

### `into_iter()`
Método que consume la colección y crea un iterador de valores owned (`T`).

### Adaptador
Método de iterador que transforma el iterador en otro iterador. Son **lazy** (perezosos), no ejecutan hasta que se consume.

Ejemplos: `map`, `filter`, `take`, `skip`, `enumerate`, `zip`, `chain`.

### Consumidor
Método de iterador que consume el iterador y produce un resultado final. Fuerza la evaluación de la cadena.

Ejemplos: `collect`, `fold`, `sum`, `count`, `find`, `any`, `all`.

### Lazy Evaluation (Evaluación Perezosa)
Los adaptadores no computan valores hasta que un consumidor los solicita. Permite cadenas eficientes.

```rust
// Nada se ejecuta aún
let iter = datos.iter().map(|x| x * 2).filter(|x| x > 5);
// Ahora sí se ejecuta
let resultado: Vec<_> = iter.collect();
```

### `map`
Adaptador que transforma cada elemento aplicando una función.

```rust
vec![1, 2, 3].iter().map(|x| x * 2) // → 2, 4, 6
```

### `filter`
Adaptador que retiene solo los elementos que cumplen un predicado.

```rust
vec![1, 2, 3, 4].iter().filter(|x| *x % 2 == 0) // → 2, 4
```

### `fold`
Consumidor que reduce el iterador a un valor único usando un acumulador.

```rust
vec![1, 2, 3].iter().fold(0, |acc, x| acc + x) // → 6
```

### `collect`
Consumidor que transforma el iterador en una colección. Requiere anotación de tipo.

```rust
let v: Vec<i32> = (1..5).collect();
```

### `enumerate`
Adaptador que añade un índice a cada elemento, produciendo `(índice, valor)`.

### `zip`
Adaptador que combina dos iteradores en uno de tuplas `(a, b)`.

### `chain`
Adaptador que concatena dos iteradores secuencialmente.

### `flatten`
Adaptador que aplana un iterador de iteradores en un solo nivel.

### `flat_map`
Combinación de `map` seguido de `flatten`.

### `take`
Adaptador que limita el iterador a los primeros n elementos.

### `skip`
Adaptador que salta los primeros n elementos.

### `peekable`
Adaptador que permite ver el siguiente elemento sin consumirlo.

### Iterador Infinito
Iterador que nunca retorna `None`. Debe limitarse con `take()` para usar con `collect()`.

```rust
std::iter::repeat(1).take(5) // → 1, 1, 1, 1, 1
```

### Zero-Cost Abstraction
Los iteradores en Rust se compilan a código tan eficiente como loops manuales. No hay overhead en runtime.

### IntoIterator
Trait que permite convertir un tipo en iterador. Usado por el `for` loop.

```rust
for x in vec![1, 2, 3] { // Vec implementa IntoIterator
    println!("{}", x);
}
```
