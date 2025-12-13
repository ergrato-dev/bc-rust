# 📖 Glosario - Semana 08: Colecciones

## A

### Adaptador (Adapter)
Método de iterador que transforma un iterador en otro sin consumirlo.
```rust
// map, filter, take son adaptadores
let nums = vec![1, 2, 3];
let doubled = nums.iter().map(|x| x * 2); // No consume aún
```

### Allocator
El sistema que gestiona la memoria del heap donde viven las colecciones.

## B

### Borrowing (Préstamo)
Acceder a datos sin tomar ownership. Fundamental con colecciones.
```rust
let vec = vec![1, 2, 3];
for item in &vec { // Préstamo inmutable
    println!("{}", item);
}
```

### Buffer
Área de memoria reservada. Vec mantiene un buffer con capacidad extra.

## C

### Capacity (Capacidad)
Espacio reservado en memoria para una colección, puede ser mayor que len.
```rust
let mut v = Vec::with_capacity(100);
println!("Capacidad: {}", v.capacity()); // 100
println!("Longitud: {}", v.len());       // 0
```

### Collect
Consumidor que transforma un iterador en una colección.
```rust
let squares: Vec<i32> = (1..5).map(|x| x * x).collect();
```

### Consumidor (Consumer)
Método que consume un iterador produciendo un valor final.
```rust
let sum: i32 = vec![1, 2, 3].iter().sum(); // sum es consumidor
```

## D

### Deref Coercion
Conversión automática de &String a &str.
```rust
fn procesar(s: &str) { }
let s = String::from("hola");
procesar(&s); // &String se convierte a &str
```

### Drain
Método que remueve elementos de una colección retornando un iterador.
```rust
let mut v = vec![1, 2, 3, 4, 5];
let drenado: Vec<_> = v.drain(1..3).collect();
```

## E

### Entry API
API de HashMap para acceso condicional eficiente.
```rust
map.entry(key)
   .or_insert(default)
   .and_modify(|v| *v += 1);
```

### Enumerate
Adaptador que agrega índices a un iterador.
```rust
for (i, val) in vec.iter().enumerate() {
    println!("Índice {}: {}", i, val);
}
```

## F

### Filter
Adaptador que selecciona elementos según un predicado.
```rust
let pares: Vec<_> = (1..10).filter(|x| x % 2 == 0).collect();
```

### Flat Map
Adaptador que mapea y aplana estructuras anidadas.
```rust
let nested = vec![vec![1, 2], vec![3, 4]];
let flat: Vec<_> = nested.iter().flat_map(|v| v.iter()).collect();
```

### Fold
Consumidor que reduce un iterador a un valor acumulado.
```rust
let sum = vec![1, 2, 3].iter().fold(0, |acc, x| acc + x);
```

## G

### Grapheme Cluster
Unidad visual de texto que puede consistir en múltiples code points Unicode.

### Grow (Crecer)
Proceso de aumentar la capacidad de un Vec cuando se llena.

## H

### HashMap
Colección que mapea claves a valores usando una función hash.
```rust
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("clave".to_string(), 42);
```

### Hash Function
Función que convierte una clave en un índice de tabla hash.

### Heap
Región de memoria donde se almacenan los datos de las colecciones.

## I

### Into Iterator
Trait que permite convertir algo en un iterador.
```rust
for item in vec.into_iter() { } // Consume vec
```

### Iterator
Trait que define la interfaz de iteración en Rust.
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Iter
Método que crea un iterador de referencias inmutables.
```rust
for item in vec.iter() { } // &T
```

### Iter Mut
Método que crea un iterador de referencias mutables.
```rust
for item in vec.iter_mut() { } // &mut T
```

## L

### Lazy Evaluation
Evaluación diferida: los iteradores no procesan hasta ser consumidos.
```rust
let iter = vec![1, 2, 3].iter().map(|x| x * 2); // Nada sucede aún
let vec: Vec<_> = iter.collect(); // Ahora se procesa
```

### Length (Longitud)
Número de elementos actualmente en una colección.

## M

### Map
Adaptador que transforma cada elemento de un iterador.
```rust
let doubled: Vec<_> = vec![1, 2, 3].iter().map(|x| x * 2).collect();
```

### Move Semantics
Transferencia de ownership al iterar con `into_iter()`.

## O

### Owned vs Borrowed
String es owned, &str es borrowed.
```rust
let owned: String = String::from("hola");
let borrowed: &str = &owned;
```

## P

### Peekable
Adaptador que permite ver el siguiente elemento sin consumirlo.
```rust
let mut iter = vec![1, 2, 3].iter().peekable();
if iter.peek() == Some(&&1) { }
```

### Push
Agregar un elemento al final de un Vec.
```rust
let mut v = Vec::new();
v.push(1);
```

## R

### Reduce
Similar a fold pero sin valor inicial.
```rust
let product = vec![1, 2, 3, 4].iter().copied().reduce(|a, b| a * b);
```

### Reserve
Pre-asignar capacidad en una colección.
```rust
let mut v = Vec::new();
v.reserve(100);
```

### Retain
Mantener solo elementos que cumplen un predicado.
```rust
let mut v = vec![1, 2, 3, 4, 5];
v.retain(|x| x % 2 == 0); // [2, 4]
```

## S

### Slice
Vista de una porción contigua de memoria.
```rust
let v = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &v[1..4]; // [2, 3, 4]
```

### String
Tipo owned de cadena UTF-8 en el heap.
```rust
let s = String::from("hola");
```

### str
Tipo primitivo de string slice (siempre usado como &str).
```rust
let s: &str = "hola"; // String literal
```

## T

### Take
Adaptador que limita el número de elementos.
```rust
let first_3: Vec<_> = (1..100).take(3).collect();
```

### Turbofish
Sintaxis para especificar tipos genéricos en métodos.
```rust
let parsed = "42".parse::<i32>().unwrap();
let collected = iter.collect::<Vec<_>>();
```

## U

### UTF-8
Codificación de caracteres Unicode usada por String y str.

## V

### Vec<T>
Vector dinámico, array de tamaño variable en el heap.
```rust
let v: Vec<i32> = vec![1, 2, 3];
```

## W

### With Capacity
Crear colección con capacidad inicial reservada.
```rust
let v = Vec::with_capacity(1000);
let s = String::with_capacity(100);
let m = HashMap::with_capacity(50);
```

## Z

### Zero-Cost Abstraction
Los iteradores de Rust compilan a código tan eficiente como loops manuales.
```rust
// Esto:
let sum: i32 = v.iter().filter(|x| **x > 0).sum();
// Es tan rápido como un for loop manual
```

## Zip
Adaptador que combina dos iteradores en pares.
```rust
let a = [1, 2, 3];
let b = [10, 20, 30];
let pairs: Vec<_> = a.iter().zip(b.iter()).collect();
// [(1, 10), (2, 20), (3, 30)]
```
