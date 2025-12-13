# Práctica 04: Patrones de Concurrencia

## 🎯 Objetivos

- Implementar Worker Pool (Thread Pool)
- Aplicar el patrón Map-Reduce
- Construir pipelines de procesamiento

## 📚 Patrones

### Worker Pool

```
[Cola de Jobs] --> [Worker 1]
                   [Worker 2]
                   [Worker 3]
                   [Worker N]
```

Un pool de threads que procesan jobs de una cola compartida.

### Map-Reduce

```
[Data] --> [Map 1] --\
       --> [Map 2] ----> [Reduce] --> [Result]
       --> [Map 3] --/
```

Dividir datos, procesar en paralelo, combinar resultados.

### Pipeline

```
[Etapa 1] --> [Etapa 2] --> [Etapa 3] --> [Resultado]
```

Procesamiento en cadena donde cada etapa corre en su thread.

## 📝 Ejercicios

### Ejercicio 1: Worker Pool

```rust
struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}

impl WorkerPool {
    fn new(num_workers: usize) -> Self
    fn execute<F>(&self, job: F) where F: FnOnce() + Send + 'static
    fn shutdown(self)
}
```

**Clave:** Los workers comparten el receiver con `Arc<Mutex<Receiver<Job>>>`

### Ejercicio 2: Map-Reduce

```rust
fn map_reduce<T, K, V, M, R>(
    datos: &[T],
    map_fn: M,
    reduce_fn: R,
    num_threads: usize,
) -> Vec<(K, V)>
```

**Ejemplo:** Contar frecuencia de palabras

### Ejercicio 3: Pipeline de Imágenes

```rust
fn pipeline_imagenes(imagenes: Vec<i32>) -> Vec<i32>
```

4 etapas: Generar → Filtrar → Transformar → Guardar

## 💡 Tips Worker Pool

```rust
// Receiver compartido
let receiver = Arc::new(Mutex::new(rx));

for _ in 0..num_workers {
    let receiver = Arc::clone(&receiver);
    thread::spawn(move || {
        loop {
            // Lock solo para recv, no durante ejecución
            let job = {
                let rx = receiver.lock().unwrap();
                rx.recv()
            };
            
            match job {
                Ok(job) => job(),
                Err(_) => break, // Channel cerrado
            }
        }
    });
}
```

## ▶️ Ejecución

```bash
cargo run
cargo test
cargo test -- --nocapture
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Worker Pool funcional | 35 |
| Map-Reduce correcto | 30 |
| Pipeline 4 etapas | 25 |
| Código limpio | 10 |

## 🔗 Recursos

- [Rust Book - Final Project](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [rayon crate](https://docs.rs/rayon/) (referencia)
