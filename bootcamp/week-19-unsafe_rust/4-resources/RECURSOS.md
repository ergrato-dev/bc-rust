# Recursos — Semana 19: `unsafe` Rust

## 📖 Lectura Obligatoria

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — La biblia de unsafe Rust
- [Rustonomicon: Data Races and Send/Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)
- [std::mem — Documentación oficial](https://doc.rust-lang.org/std/mem/index.html)
- [std::ptr — Documentación oficial](https://doc.rust-lang.org/std/ptr/index.html)

## 🛠️ Herramientas

- **Miri**: `cargo +nightly miri test` — detecta UB en tiempo de ejecución
- **AddressSanitizer**: `RUSTFLAGS="-Z sanitizer=address" cargo +nightly test`
- **Valgrind** (Linux): detecta leaks desde fuera del proceso

## 🎥 Videos Recomendados

- Jon Gjengset — "Crust of Rust: Smart Pointers and Interior Mutability"
- Jon Gjengset — "Crust of Rust: Raw Pointers"

## 📚 Artículos

- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Unsafe Rust: How and when (not) to use it](https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/)
