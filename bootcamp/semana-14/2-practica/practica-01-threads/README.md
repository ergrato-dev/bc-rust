# Práctica 01: Creación y Gestión de Threads

## 🎯 Objetivos

- Crear threads con `std::thread::spawn`
- Esperar resultados con `join()`
- Transferir ownership con `move`
- Dividir trabajo entre múltiples threads

## 📚 Conceptos Clave

### Crear un Thread

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hola desde otro thread!");
    42 // Valor de retorno
});

let resultado = handle.join().unwrap(); // Esperar y obtener resultado
```

### Mover Datos al Thread

```rust
let datos = vec![1, 2, 3];

let handle = thread::spawn(move || {
    // `move` transfiere ownership de `datos`
    datos.iter().sum::<i32>()
});
```

### Múltiples Threads

```rust
let mut handles = vec![];

for i in 0..4 {
    handles.push(thread::spawn(move || {
        i * 10
    }));
}

let resultados: Vec<_> = handles
    .into_iter()
    .map(|h| h.join().unwrap())
    .collect();
```

## 📝 Ejercicios

### Ejercicio 1: Suma Paralela

Implementa `suma_paralela` que divida un vector en partes y calcule la suma en paralelo.

```rust
fn suma_paralela(numeros: &[i32], num_threads: usize) -> i32
```

**Pistas:**
- Usa `chunks()` para dividir el slice
- Clona cada chunk a un `Vec` para moverlo al thread
- Suma los resultados parciales

### Ejercicio 2: Búsqueda Paralela

Implementa `busqueda_paralela` que busque un elemento usando múltiples threads.

```rust
fn busqueda_paralela(datos: &[i32], objetivo: i32, num_threads: usize) -> Option<usize>
```

**Pistas:**
- Calcula el índice global: `chunk_index * chunk_size + local_index`
- Retorna el primer `Some` encontrado

### Ejercicio 3: Procesador de Datos

Implementa `procesar_paralelo` que aplique una transformación en paralelo manteniendo el orden.

```rust
fn procesar_paralelo<F>(datos: &[i32], transformar: F, num_threads: usize) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Sync + Clone + 'static
```

**Pistas:**
- Usa `enumerate()` para trackear el índice del chunk
- Ordena los resultados por índice antes de aplanar

## ▶️ Ejecución

```bash
# Ejecutar ejemplos
cargo run

# Ejecutar tests
cargo test

# Ver output de tests
cargo test -- --nocapture
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| `suma_paralela` funciona correctamente | 30 |
| `busqueda_paralela` retorna índice correcto | 30 |
| `procesar_paralelo` mantiene orden | 30 |
| Código limpio y documentado | 10 |

## 🔗 Recursos

- [std::thread](https://doc.rust-lang.org/std/thread/)
- [JoinHandle](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
