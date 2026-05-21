# Recursos — Semana 25: Capstone RIIR

## 📖 Por opción

### Opción A — PyO3 + maturin
- [PyO3 User Guide](https://pyo3.rs/) — guía oficial completa
- [maturin docs](https://www.maturin.rs/) — build y distribución de módulos Python
- [PyO3 changelog](https://github.com/PyO3/pyo3/blob/main/CHANGELOG.md) — novedades de la versión actual
- [Integrating Python and Rust (blog)](https://developers.redhat.com/blog/2021/03/15/using-pyo3) — tutorial con ejemplos reales

### Opción B — CLI con clap
- [clap docs (docs.rs)](https://docs.rs/clap/latest/clap/) — referencia completa
- [clap derive tutorial](https://docs.rs/clap/latest/clap/_derive/index.html) — guía del macroatributo `#[derive(Parser)]`
- [Command Line Rust (O'Reilly)](https://www.oreilly.com/library/view/command-line-rust/9781098109424/) — libro dedicado a CLIs en Rust
- [indicatif](https://docs.rs/indicatif) — barras de progreso y spinners para CLI

### Opción C — WebAssembly
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/) — integración Rust↔JS
- [wasm-pack Book](https://rustwasm.github.io/docs/wasm-pack/) — build, test y publicación en npm
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/) — tutorial del juego of life
- [console_error_panic_hook](https://docs.rs/console_error_panic_hook) — ver panics de Rust en DevTools

### Opción D — FFI + cbindgen
- [cbindgen docs](https://github.com/mozilla/cbindgen/blob/master/docs.md) — configuración y uso
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe Rust en profundidad
- [libc crate](https://docs.rs/libc) — tipos C estándar (`c_uchar`, `size_t`, etc.)
- [Foreign Function Interface (RFC)](https://doc.rust-lang.org/reference/items/external-blocks.html) — referencia oficial de FFI en Rust

---

## 📚 Diseño de API y publicación

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — guía oficial de diseño de APIs idiomáticas
- [crates.io categories](https://crates.io/categories) — lista oficial de categorías válidas para `Cargo.toml`
- [docs.rs](https://docs.rs) — documentación automática generada para todos los crates publicados
- [semver spec](https://semver.org/) — especificación de versionado semántico
- [cargo-release](https://github.com/crate-ci/cargo-release) — automatiza el bump de versión y el publish

## 🔒 Seguridad y auditoría

- [cargo-audit](https://rustsec.org/) — auditoría de CVEs en dependencias
- [RustSec Advisory Database](https://rustsec.org/advisories/) — base de datos de vulnerabilidades conocidas
- [cargo-deny](https://embarkstudios.github.io/cargo-deny/) — políticas de licencias y dependencias en CI

## 🧪 Testing y benchmarking

- [Criterion.rs Book](https://bheisler.github.io/criterion.rs/book/) — benchmarking estadístico
- [proptest](https://docs.rs/proptest) — property-based testing (alternativa a QuickCheck)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) — cobertura de código para Rust

## 📋 Checklist final antes de entregar

```bash
cargo fmt --check              # ✅ código formateado
cargo clippy -- -D warnings    # ✅ cero warnings
cargo test                     # ✅ unit + integration + doctests
cargo audit                    # ✅ cero CVEs
cargo doc --no-deps            # ✅ docs generan sin error
cargo publish --dry-run        # ✅ simulacro de publicación
```

