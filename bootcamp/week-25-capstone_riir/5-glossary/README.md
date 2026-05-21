# Glosario — Semana 25: Capstone RIIR

| Término | Definición |
|---------|-----------|
| **RIIR** | "Rewrite It In Rust" — tendencia de reescribir herramientas en Rust |
| **Capstone** | Proyecto integrador final que aplica todas las competencias del bootcamp |
| **maturin** | Herramienta de build para librerías Python escritas en Rust (PyO3) |
| **cbindgen** | Genera headers C/C++ automáticamente desde código Rust con `#[repr(C)]` |
| **clap** | "Command Line Argument Parser" — crate estándar de facto para CLIs Rust |
| **`#[derive(Parser)]`** | Atributo de clap que genera el parser de argumentos desde la struct |
| **Subcommand** | Comando hijo de un CLI (e.g., `git commit`, `git push`) |
| **FNV-1a** | Fowler-Noll-Vo hash — función de hash no criptográfica muy rápida |
| **DJB2** | Hash de Daniel J. Bernstein — simple y eficiente para strings cortos |
| **`#![deny(missing_docs)]`** | Lint que convierte la falta de documentación en error de compilación |
| **Zero-dependency crate** | Crate publicado en crates.io sin dependencias externas |
| **Pratt parser** | Técnica de parsing de expresiones con precedencia de operadores |
| **Extension module** | Módulo Python compilado como `.so`/`.pyd` — lo que PyO3 genera |
| **`wasm-pack publish`** | Publica el paquete WASM en npm |
| **pdqsort** | Pattern-defeating quicksort — algoritmo usado por `slice::sort` en Rust |
