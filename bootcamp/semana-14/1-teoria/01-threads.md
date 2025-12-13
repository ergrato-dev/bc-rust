# Threads y std::thread

## 🎯 Objetivos

- Crear threads con `thread::spawn`
- Esperar threads con `join()`
- Transferir datos con `move`
- Recuperar valores de threads

![Threads](../0-assets/01-threads.svg)

## 📚 Conceptos

### ¿Qué es un Thread?

Un thread es una unidad de ejecución independiente dentro de un proceso. Rust usa threads del sistema operativo (1:1 threading).

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Crear un nuevo thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread hijo: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // El thread principal continúa
    for i in 1..3 {
        println!("Thread principal: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // Esperar a que el thread hijo termine
    handle.join().unwrap();
}
```

### El Closure `move`

Para transferir ownership de datos al thread, usamos `move`:

```rust
use std::thread;

fn main() {
    let datos = vec![1, 2, 3];

    // ❌ Error: el closure puede vivir más que `datos`
    // let handle = thread::spawn(|| {
    //     println!("{:?}", datos);
    // });

    // ✅ Correcto: transferimos ownership
    let handle = thread::spawn(move || {
        println!("{:?}", datos);
    });

    // datos ya no es accesible aquí
    // println!("{:?}", datos); // Error!

    handle.join().unwrap();
}
```

### Recuperar Valores de Threads

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // Calcular algo
        let suma: i32 = (1..=100).sum();
        suma  // Retornar valor
    });

    // join() retorna Result<T, Error>
    let resultado = handle.join().unwrap();
    println!("Suma: {}", resultado);
}
```

### Múltiples Threads

```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} ejecutándose", i);
            i * 2
        });
        handles.push(handle);
    }

    // Esperar todos los threads
    let resultados: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    println!("Resultados: {:?}", resultados);
}
```

## 🔧 API de Threads

### `thread::spawn`

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
```

Requisitos:
- `FnOnce`: Se ejecuta una vez
- `Send`: Los datos pueden enviarse entre threads
- `'static`: No puede tener referencias a datos locales

### `JoinHandle`

```rust
let handle = thread::spawn(|| { /* ... */ });

// Esperar y obtener resultado
let result: Result<T, Box<dyn Any + Send>> = handle.join();

// Si el thread entra en panic, join() retorna Err
match handle.join() {
    Ok(valor) => println!("Éxito: {:?}", valor),
    Err(_) => println!("El thread entró en panic"),
}
```

### Thread Builder

Para más control sobre la creación de threads:

```rust
use std::thread;

let builder = thread::Builder::new()
    .name("worker-1".to_string())
    .stack_size(4 * 1024 * 1024); // 4 MB

let handle = builder.spawn(|| {
    println!("Soy el thread: {:?}", thread::current().name());
}).unwrap();

handle.join().unwrap();
```

### Información del Thread Actual

```rust
use std::thread;

fn main() {
    println!("Thread principal: {:?}", thread::current().id());
    
    thread::spawn(|| {
        let current = thread::current();
        println!("ID: {:?}", current.id());
        println!("Nombre: {:?}", current.name());
    }).join().unwrap();
}
```

## ⚡ Paralelismo con Threads

### Ejemplo: Suma Paralela

```rust
use std::thread;

fn suma_paralela(numeros: Vec<i32>, num_threads: usize) -> i32 {
    let chunk_size = (numeros.len() + num_threads - 1) / num_threads;
    let mut handles = vec![];

    for chunk in numeros.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            chunk.iter().sum::<i32>()
        });
        handles.push(handle);
    }

    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}

fn main() {
    let numeros: Vec<i32> = (1..=1000).collect();
    let resultado = suma_paralela(numeros, 4);
    println!("Suma: {}", resultado);
}
```

## ⚠️ Errores Comunes

### 1. No usar `move` cuando es necesario

```rust
// ❌ Error: `data` borrowed in closure which may outlive function
let data = String::from("hello");
thread::spawn(|| println!("{}", data));

// ✅ Correcto
thread::spawn(move || println!("{}", data));
```

### 2. Olvidar `join()`

```rust
fn main() {
    thread::spawn(|| {
        println!("Puede que no se imprima!");
    });
    // El programa termina antes que el thread
}

// ✅ Correcto
fn main() {
    let handle = thread::spawn(|| {
        println!("Se imprimirá!");
    });
    handle.join().unwrap();
}
```

### 3. Intentar compartir referencias

```rust
// ❌ Error: referencias no cumplen 'static
let data = vec![1, 2, 3];
let reference = &data;
thread::spawn(move || println!("{:?}", reference)); // Error!

// ✅ Usar Arc para compartir
use std::sync::Arc;
let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);
thread::spawn(move || println!("{:?}", data_clone));
```

## 📊 Cuándo Usar Threads

| Situación | Recomendación |
|-----------|---------------|
| Tareas CPU-bound | ✅ Threads |
| Tareas I/O-bound | Considerar async |
| Muchas tareas pequeñas | Thread pool |
| Una tarea larga | ✅ Thread dedicado |

## 🎯 Resumen

```rust
use std::thread;

// Crear thread
let handle = thread::spawn(move || {
    // código del thread
    42  // valor de retorno
});

// Esperar y obtener resultado
let resultado = handle.join().unwrap();
```

## 📖 Siguiente

[02-channels.md](02-channels.md) - Comunicación entre threads
