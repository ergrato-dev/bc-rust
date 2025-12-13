# Semana 15: Programación Asíncrona (Async/Await)

## 🎯 Objetivos de la Semana

Al finalizar esta semana, serás capaz de:

1. Entender el modelo de programación asíncrona en Rust
2. Usar `async`/`await` para escribir código no bloqueante
3. Trabajar con el runtime Tokio
4. Implementar operaciones I/O asíncronas
5. Manejar concurrencia con futures y tasks

## 📚 Contenido

### Teoría

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-fundamentos.md](1-teoria/01-fundamentos.md) | Fundamentos de Async | 45 min |
| [02-futures.md](1-teoria/02-futures.md) | Futures y el trait Future | 40 min |
| [03-tokio.md](1-teoria/03-tokio.md) | Runtime Tokio | 45 min |
| [04-io-async.md](1-teoria/04-io-async.md) | I/O Asíncrono | 40 min |
| [05-patrones.md](1-teoria/05-patrones.md) | Patrones Async | 35 min |

### Práctica

| Proyecto | Descripción | Dificultad |
|----------|-------------|------------|
| [practica-01-async-basico](2-practica/practica-01-async-basico/) | Funciones async y await | ⭐⭐ |
| [practica-02-tokio](2-practica/practica-02-tokio/) | Runtime y tasks | ⭐⭐⭐ |
| [practica-03-io-async](2-practica/practica-03-io-async/) | Archivos y red async | ⭐⭐⭐ |
| [practica-04-concurrencia](2-practica/practica-04-concurrencia/) | join!, select!, channels | ⭐⭐⭐ |
| [proyecto-crawler](2-practica/proyecto-crawler/) | **Web Crawler Async** | ⭐⭐⭐⭐ |

## 🔑 Conceptos Clave

### Async vs Sync

```rust
// Síncrono: bloquea el thread
fn fetch_sync(url: &str) -> String {
    // El thread espera aquí
    blocking_request(url)
}

// Asíncrono: no bloquea
async fn fetch_async(url: &str) -> String {
    // El thread puede hacer otras cosas mientras espera
    non_blocking_request(url).await
}
```

### El trait Future

```rust
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),    // El valor está listo
    Pending,     // Aún no está listo
}
```

### Async/Await

```rust
async fn procesar_datos() -> Result<(), Error> {
    let datos = fetch_datos().await?;      // Espera sin bloquear
    let resultado = calcular(&datos).await?;
    guardar(resultado).await?;
    Ok(())
}
```

### Runtime Tokio

```rust
#[tokio::main]
async fn main() {
    // Spawn tasks concurrentes
    let t1 = tokio::spawn(async { tarea1().await });
    let t2 = tokio::spawn(async { tarea2().await });
    
    // Esperar ambas
    let (r1, r2) = tokio::join!(t1, t2);
}
```

## 📊 Comparativa: Threads vs Async

| Aspecto | Threads | Async |
|---------|---------|-------|
| **Overhead** | ~8KB stack por thread | ~pocos bytes por future |
| **Cambio contexto** | Costoso (kernel) | Barato (userspace) |
| **Escalabilidad** | Miles | Millones |
| **Uso** | CPU-bound | I/O-bound |
| **Complejidad** | Menor | Mayor |
| **Bloqueo** | Bloquea thread | No bloquea |

## 🏗️ Modelo Mental

```
┌─────────────────────────────────────────────────────────┐
│                     RUNTIME (Tokio)                      │
│  ┌─────────────────────────────────────────────────────┐│
│  │                    EXECUTOR                          ││
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   ││
│  │  │ Task 1  │ │ Task 2  │ │ Task 3  │ │ Task N  │   ││
│  │  │(Future) │ │(Future) │ │(Future) │ │(Future) │   ││
│  │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘   ││
│  │       │           │           │           │         ││
│  │       └───────────┴─────┬─────┴───────────┘         ││
│  │                         │                            ││
│  │                    ┌────▼────┐                       ││
│  │                    │  POLL   │                       ││
│  │                    └────┬────┘                       ││
│  └─────────────────────────┼───────────────────────────┘│
│                            │                             │
│  ┌─────────────────────────▼───────────────────────────┐│
│  │                    REACTOR                           ││
│  │            (epoll/kqueue/IOCP)                       ││
│  │         Notifica cuando I/O está listo              ││
│  └─────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

## 📦 Dependencias

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
futures = "0.3"
reqwest = { version = "0.12", features = ["json"] }
```

## ⚠️ Errores Comunes

### 1. Olvidar `.await`

```rust
// MAL: fetch_data() retorna un Future, no datos
let datos = fetch_data();  // datos es Future, no String

// BIEN
let datos = fetch_data().await;
```

### 2. Bloquear en contexto async

```rust
// MAL: std::thread::sleep bloquea el runtime
async fn malo() {
    std::thread::sleep(Duration::from_secs(1)); // ¡NO!
}

// BIEN: usar versión async
async fn bueno() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### 3. No usar spawn para tareas independientes

```rust
// MAL: secuencial
let a = tarea_a().await;
let b = tarea_b().await;  // Espera a que termine a

// BIEN: paralelo con join!
let (a, b) = tokio::join!(tarea_a(), tarea_b());
```

## 📅 Distribución del Tiempo

| Actividad | Tiempo |
|-----------|--------|
| Teoría (5 temas) | 3.5 horas |
| Prácticas (4) | 2.5 horas |
| Proyecto Crawler | 2 horas |
| Ejercicios extra | 1 hora |
| **Total** | **9 horas** |

## 🔗 Recursos

- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Tokio Docs](https://docs.rs/tokio/)
- [futures crate](https://docs.rs/futures/)

## ✅ Checklist de Aprendizaje

- [ ] Entiendo la diferencia entre sync y async
- [ ] Puedo escribir funciones async con await
- [ ] Sé configurar y usar Tokio runtime
- [ ] Puedo hacer I/O de archivos asíncrono
- [ ] Puedo hacer requests HTTP async
- [ ] Sé usar join!, select!, spawn
- [ ] Puedo manejar errores en código async
- [ ] Completé el proyecto crawler

---

**Siguiente semana:** [Semana 16 - Testing y Documentación](../semana-16/README.md)
