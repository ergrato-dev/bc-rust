# Práctica 02: Comunicación con Channels

## 🎯 Objetivos

- Crear channels con `std::sync::mpsc`
- Implementar patrones productor-consumidor
- Usar múltiples productores (MPSC)
- Construir pipelines de procesamiento

## 📚 Conceptos Clave

### Channel Básico

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

// Enviar
tx.send("mensaje").unwrap();

// Recibir (bloqueante)
let msg = rx.recv().unwrap();
```

### Múltiples Productores

```rust
let (tx, rx) = mpsc::channel();

for i in 0..3 {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        tx_clone.send(i).unwrap();
    });
}
drop(tx); // Importante: cerrar el original

for msg in rx {
    println!("Recibido: {}", msg);
}
```

### Iterando Mensajes

```rust
// rx implementa Iterator
for mensaje in rx {
    procesar(mensaje);
}
// El loop termina cuando todos los senders se dropean
```

## 📝 Ejercicios

### Ejercicio 1: Pipeline

Implementa un pipeline de 3 etapas:

```
[Generador] --> [Filtro Pares] --> [*10]
```

```rust
fn pipeline(n: i32) -> Vec<i32>
```

**Ejemplo:** `pipeline(10)` → `[20, 40, 60, 80, 100]`

### Ejercicio 2: Agregador

Sistema donde workers procesan tareas y envían resultados:

```rust
fn agregador(tareas: &[i32], num_workers: usize) -> i32
```

**Ejemplo:** `agregador(&[1,2,3], 2)` → `14` (suma de cuadrados)

### Ejercicio 3: Broadcast

Un productor envía a múltiples consumidores:

```rust
fn broadcast(num_mensajes: usize, num_consumidores: usize) -> Vec<usize>
```

**Ejemplo:** `broadcast(5, 3)` → `[5, 5, 5]`

## ▶️ Ejecución

```bash
# Ejecutar ejemplos
cargo run

# Ejecutar tests
cargo test

# Ver output de tests
cargo test -- --nocapture
```

## 💡 Tips

1. **Siempre dropear el tx original** cuando usas clones
2. **recv() es bloqueante** - espera hasta recibir o error
3. **try_recv()** no bloquea, retorna `Err` si no hay mensaje
4. El channel se cierra cuando **todos los senders** se dropean

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Pipeline con 3 etapas conectadas | 35 |
| Agregador con múltiples workers | 35 |
| Broadcast a múltiples consumidores | 20 |
| Código limpio y documentado | 10 |

## 🔗 Recursos

- [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/)
- [Sender](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html)
- [Receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
