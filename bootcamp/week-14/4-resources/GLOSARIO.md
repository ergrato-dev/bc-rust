# Glosario - Semana 14: Concurrencia

## Términos Fundamentales

### Thread (Hilo)
Unidad de ejecución dentro de un proceso. Múltiples threads comparten el mismo espacio de memoria pero tienen su propia pila de ejecución.

```rust
use std::thread;

let handle = thread::spawn(|| {
    // Código que corre en otro thread
});
```

### Spawn
Crear un nuevo thread. En Rust, `thread::spawn` inicia un thread y retorna un `JoinHandle`.

### Join
Esperar a que un thread termine su ejecución. `handle.join()` bloquea hasta que el thread finaliza.

```rust
let resultado = handle.join().unwrap();
```

### Move Closure
Closure que toma ownership de las variables capturadas. Necesario para pasar datos a threads.

```rust
let datos = vec![1, 2, 3];
thread::spawn(move || {
    // `datos` ahora pertenece a este thread
    println!("{:?}", datos);
});
```

---

## Comunicación entre Threads

### Channel
Canal de comunicación unidireccional entre threads. Consiste en un Sender y un Receiver.

### MPSC (Multiple Producer, Single Consumer)
Tipo de channel donde múltiples threads pueden enviar pero solo uno recibe.

```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
```

### Sender (Transmisor)
Extremo del channel que envía mensajes. Puede clonarse para múltiples productores.

```rust
tx.send(mensaje).unwrap();
let tx2 = tx.clone(); // Múltiples senders
```

### Receiver (Receptor)
Extremo del channel que recibe mensajes. No puede clonarse (single consumer).

```rust
let msg = rx.recv().unwrap();  // Bloqueante
let msg = rx.try_recv();       // No bloqueante
```

---

## Sincronización

### Mutex (Mutual Exclusion)
Primitiva que garantiza acceso exclusivo a datos. Solo un thread puede tener el lock a la vez.

```rust
use std::sync::Mutex;
let datos = Mutex::new(0);
let mut guard = datos.lock().unwrap();
*guard += 1;
```

### MutexGuard
RAII guard que mantiene el lock del Mutex. El lock se libera cuando el guard sale de scope.

### RwLock (Read-Write Lock)
Lock que permite múltiples lectores simultáneos O un único escritor.

```rust
use std::sync::RwLock;
let datos = RwLock::new(vec![]);
let r = datos.read().unwrap();   // Múltiples lectores OK
let w = datos.write().unwrap();  // Escritor exclusivo
```

### Arc (Atomic Reference Counting)
Smart pointer thread-safe para compartir ownership entre threads.

```rust
use std::sync::Arc;
let datos = Arc::new(Mutex::new(0));
let datos_clone = Arc::clone(&datos);
```

### Deadlock
Situación donde dos o más threads se bloquean mutuamente esperando recursos que el otro tiene.

```rust
// Thread 1: lock A, luego lock B
// Thread 2: lock B, luego lock A
// = DEADLOCK
```

---

## Traits de Concurrencia

### Send
Marker trait que indica que un tipo puede transferirse entre threads (ownership).

```rust
// Tipos Send pueden moverse a otro thread
thread::spawn(move || {
    let x: SendType = valor; // OK
});
```

### Sync
Marker trait que indica que un tipo puede compartirse entre threads (referencias).

```rust
// T es Sync si &T es Send
// Múltiples threads pueden tener &T simultáneamente
```

### Send + Sync
- `i32`, `String`, `Vec<T>`: Send + Sync
- `Arc<T>`: Send + Sync (si T lo es)
- `Mutex<T>`: Send + Sync (si T es Send)
- `Rc<T>`: Ni Send ni Sync
- `RefCell<T>`: Send pero no Sync

---

## Patrones de Concurrencia

### Producer-Consumer
Un thread produce datos, otro los consume. Conectados por un channel.

### Worker Pool (Thread Pool)
Conjunto de threads pre-creados que procesan jobs de una cola compartida.

### Fork-Join
Dividir trabajo en subtareas paralelas (fork) y esperar resultados (join).

### Map-Reduce
Dividir datos, procesar en paralelo (map), combinar resultados (reduce).

### Pipeline
Cadena de etapas de procesamiento, cada una en su thread, conectadas por channels.

---

## Funciones y Métodos Comunes

### thread::spawn
```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
```

### JoinHandle::join
```rust
pub fn join(self) -> Result<T>
```

### mpsc::channel
```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
```

### Sender::send
```rust
pub fn send(&self, t: T) -> Result<(), SendError<T>>
```

### Receiver::recv
```rust
pub fn recv(&self) -> Result<T, RecvError>
```

### Mutex::lock
```rust
pub fn lock(&self) -> LockResult<MutexGuard<T>>
```

### Arc::clone
```rust
pub fn clone(this: &Arc<T>) -> Arc<T>
```

---

## Errores Comunes

### PoisonError
Ocurre cuando un thread hace panic mientras tiene un lock. El Mutex queda "envenenado".

```rust
// Manejar mutex envenenado
let guard = mutex.lock().unwrap_or_else(|e| e.into_inner());
```

### SendError
Error al enviar por un channel cerrado (receiver dropeado).

### RecvError
Error al recibir de un channel cerrado (todos los senders dropeados).

---

## Abreviaciones

| Abreviación | Significado |
|-------------|-------------|
| MPSC | Multiple Producer, Single Consumer |
| RAII | Resource Acquisition Is Initialization |
| Arc | Atomic Reference Counting |
| Rc | Reference Counting |
| Mutex | Mutual Exclusion |
| RwLock | Read-Write Lock |
