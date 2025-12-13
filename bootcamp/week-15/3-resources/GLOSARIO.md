# Glosario - Semana 15: Async/Await

## A

### async
Palabra clave que marca una función como asíncrona. La función retorna un `Future` en lugar de ejecutarse inmediatamente.

### await
Palabra clave que suspende la ejecución de una función async hasta que el `Future` esté listo. Solo puede usarse dentro de funciones `async`.

### AsyncRead
Trait que define lectura asíncrona de bytes. Equivalente async de `std::io::Read`.

### AsyncWrite
Trait que define escritura asíncrona de bytes. Equivalente async de `std::io::Write`.

## B

### Block on
Ejecutar un future de forma bloqueante desde código síncrono. Usado para iniciar el runtime.

### BufReader
Wrapper que añade buffering a un reader, mejorando rendimiento en lecturas pequeñas frecuentes.

### Broadcast Channel
Canal que envía cada mensaje a múltiples receptores (patrón pub/sub).

## C

### Cancellation
Proceso de abortar una task async antes de que complete.

### CancellationToken
Mecanismo para señalizar y coordinar la cancelación de múltiples tasks.

### Channel
Primitiva de comunicación entre tasks. Tipos: mpsc, oneshot, broadcast, watch.

### Concurrency
Múltiples tareas en progreso al mismo tiempo, potencialmente intercalando ejecución.

### Context
Estructura que contiene el `Waker` necesario para notificar al runtime cuando un future puede progresar.

## E

### Executor
Componente del runtime que ejecuta futures llamando a `poll()` repetidamente.

### Event Loop
Bucle que espera eventos de I/O y despacha las notificaciones correspondientes.

## F

### Future
Trait que representa un valor que estará disponible en el futuro. Núcleo de async en Rust.

### FutureExt
Trait de extensión que añade métodos útiles a los futures (map, then, boxed, etc.).

### Fuse
Wrapper que hace seguro pollrar un future después de que haya completado.

## G

### Graceful Shutdown
Proceso de terminar una aplicación de forma ordenada, completando trabajo pendiente.

## J

### join!
Macro que ejecuta múltiples futures concurrentemente y espera a que todos completen.

### JoinHandle
Handle retornado por `spawn()` que permite esperar el resultado de una task.

### JoinSet
Colección para manejar múltiples tasks spawneadas, permitiendo esperar resultados dinámicamente.

## L

### Lazy Future
Característica de los futures en Rust: no hacen nada hasta que son polleados.

## M

### mpsc
Multiple Producer, Single Consumer channel. Múltiples senders, un receiver.

## O

### oneshot
Canal para enviar exactamente un mensaje. Útil para request/response.

## P

### Parallelism
Múltiples tareas ejecutándose verdaderamente al mismo tiempo en diferentes cores.

### Pin
Wrapper que garantiza que un valor no se moverá en memoria. Necesario para futures con auto-referencias.

### Poll
Método del trait `Future` que avanza la ejecución. Retorna `Ready(T)` o `Pending`.

### Poll::Pending
Indica que el future aún no está listo y debe ser polleado de nuevo más tarde.

### Poll::Ready
Indica que el future completó y contiene el valor resultante.

## R

### Rate Limiting
Técnica para limitar la frecuencia de operaciones (ej: requests por segundo).

### Reactor
Componente del runtime que escucha eventos de I/O (epoll/kqueue/IOCP).

### Runtime
El ejecutor de futures. Combina executor y reactor. Ejemplos: Tokio, async-std.

## S

### select!
Macro que espera múltiples futures y ejecuta el branch del primero en completar.

### Semaphore
Primitiva de sincronización que limita el número de accesos concurrentes.

### Send
Marker trait que indica que un tipo puede ser enviado entre threads de forma segura.

### spawn
Crear una nueva task que se ejecuta concurrentemente.

### spawn_blocking
Ejecutar código bloqueante en un thread pool separado para no bloquear el runtime async.

### State Machine
Representación interna de una función async generada por el compilador.

### Stream
Secuencia asíncrona de valores (equivalente async de `Iterator`).

### Sync
Marker trait que indica que un tipo puede ser accedido concurrentemente de forma segura.

## T

### Task
Unidad de trabajo async spawneada en el runtime. Similar a un green thread.

### timeout
Envolver un future con un límite de tiempo.

### Tokio
El runtime async más popular para Rust.

### try_join!
Como `join!` pero para futures que retornan `Result`. Falla rápido si alguno retorna error.

## U

### Unpin
Marker trait que indica que un tipo puede moverse libremente en memoria.

## W

### Waker
Mecanismo para notificar al runtime que un future puede progresar.

### watch
Canal donde los receptores siempre obtienen el último valor enviado.

### Worker Thread
Thread del runtime que ejecuta tasks.

### Work Stealing
Técnica donde workers ociosos toman trabajo de workers ocupados para balancear carga.
