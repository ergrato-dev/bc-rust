# Recursos - Semana 15: Async/Await

## 📚 Documentación Oficial

### Rust
- [The Async Book](https://rust-lang.github.io/async-book/) - Guía completa de async en Rust
- [std::future](https://doc.rust-lang.org/std/future/) - Documentación del módulo future
- [std::pin](https://doc.rust-lang.org/std/pin/) - Documentación de Pin

### Tokio
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Tutorial oficial paso a paso
- [Tokio API Docs](https://docs.rs/tokio/) - Documentación de la API
- [Tokio Mini-Redis](https://github.com/tokio-rs/mini-redis) - Ejemplo de servidor Redis con Tokio

## 📖 Libros

- **"Programming Rust, 2nd Edition"** - Capítulo sobre async
- **"Rust for Rustaceans"** - Secciones avanzadas de async
- **"Zero to Production in Rust"** - Aplicación web real con async

## 🎥 Videos

### Conferencias
- [Jon Gjengset - The What and How of Futures and async/await](https://www.youtube.com/watch?v=9_3krAQtD2k)
- [Steve Klabnik - Async/Await in Rust](https://www.youtube.com/watch?v=lJ3NC-R3gSI)
- [Without Boats - Async/Await in Rust: Current State and Future Directions](https://www.youtube.com/watch?v=NNwK5ZPAJCk)

### Tutoriales
- [Let's Get Rusty - Async Programming in Rust](https://www.youtube.com/watch?v=K8LNPYNvT-U)
- [Ryan Levick - Understanding Rust Futures](https://www.youtube.com/watch?v=NomPX8vL2WU)

## 📦 Crates Relacionados

### Runtime
| Crate | Descripción |
|-------|-------------|
| [tokio](https://crates.io/crates/tokio) | Runtime async más popular |
| [async-std](https://crates.io/crates/async-std) | Runtime con API similar a std |
| [smol](https://crates.io/crates/smol) | Runtime minimalista |

### Utilidades
| Crate | Descripción |
|-------|-------------|
| [futures](https://crates.io/crates/futures) | Utilidades para futures |
| [tokio-stream](https://crates.io/crates/tokio-stream) | Streams async |
| [tokio-util](https://crates.io/crates/tokio-util) | Utilidades adicionales |
| [async-trait](https://crates.io/crates/async-trait) | Traits con métodos async |

### HTTP
| Crate | Descripción |
|-------|-------------|
| [reqwest](https://crates.io/crates/reqwest) | Cliente HTTP async |
| [hyper](https://crates.io/crates/hyper) | HTTP de bajo nivel |
| [axum](https://crates.io/crates/axum) | Framework web ergonómico |
| [actix-web](https://crates.io/crates/actix-web) | Framework web rápido |

### Bases de Datos
| Crate | Descripción |
|-------|-------------|
| [sqlx](https://crates.io/crates/sqlx) | SQL async con compile-time checking |
| [sea-orm](https://crates.io/crates/sea-orm) | ORM async |
| [redis](https://crates.io/crates/redis) | Cliente Redis async |

## 🔗 Artículos y Blogs

### Fundamentos
- [How Rust Futures Work](https://cfsamson.github.io/books-futures-explained/)
- [Async: What is blocking?](https://ryhl.io/blog/async-what-is-blocking/)
- [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)

### Patrones
- [Async in depth](https://tokio.rs/tokio/tutorial/async)
- [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown)
- [Bridging sync and async](https://tokio.rs/tokio/topics/bridging)

### Performance
- [Async: Why It's Faster](https://blog.mozilla.org/nnethercote/2020/09/08/how-to-speed-up-the-rust-compiler-one-last-time/)
- [Tokio internals](https://cafbit.com/post/tokio_internals/)

## 🛠️ Herramientas

### Debugging
- [tokio-console](https://github.com/tokio-rs/console) - Debugger para Tokio
- [tracing](https://crates.io/crates/tracing) - Framework de logging/tracing

### Testing
- [tokio-test](https://crates.io/crates/tokio-test) - Utilidades de testing
- [wiremock](https://crates.io/crates/wiremock) - Mock server para HTTP

## 📝 Ejercicios Adicionales

### Rustlings
- No hay ejercicios específicos de async en rustlings estándar

### Exercism
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Varios ejercicios aplicables

### Otros
- [Tokio Mini-Redis](https://github.com/tokio-rs/mini-redis) - Proyecto tutorial completo
- [Async Book Exercises](https://rust-lang.github.io/async-book/) - Al final de cada capítulo

## 🎯 Proyectos de Práctica

1. **Chat Server** - Servidor de chat multi-cliente con websockets
2. **Web Scraper** - Descargador de páginas concurrente
3. **API Client** - Cliente que consume múltiples APIs en paralelo
4. **File Watcher** - Monitor de cambios en archivos
5. **Task Queue** - Sistema de cola de trabajos con workers

## ❓ FAQ y Problemas Comunes

### "Future is not Send"
```rust
// Problema: Usar tipos no-Send a través de .await
let rc = Rc::new(5);
some_async_op().await;  // Error!

// Solución: Usar Arc en su lugar
let arc = Arc::new(5);
some_async_op().await;  // OK
```

### "Cannot borrow across await"
```rust
// Problema: Mantener borrow a través de await
let mut data = vec![];
let ref_data = &mut data;
async_op().await;
ref_data.push(1);  // Error!

// Solución: Reestructurar para soltar borrow antes de await
```

### "Blocking in async context"
```rust
// Problema: Usar std::thread::sleep en contexto async
std::thread::sleep(Duration::from_secs(1));  // Bloquea el runtime!

// Solución: Usar tokio::time::sleep
tokio::time::sleep(Duration::from_secs(1)).await;
```

## 📅 Actualizaciones

- **Rust 1.75+**: async fn in traits (estable)
- **Rust 1.78+**: Mejoras en async closures
- **Tokio 1.x**: Versión LTS actual

---

**Última actualización**: Enero 2025
