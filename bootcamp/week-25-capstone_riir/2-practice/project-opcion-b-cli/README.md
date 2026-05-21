# Opción B — CLI Tool: `rwc`

## 🎯 Descripción
Reescritura de la utilidad `wc` de Unix en Rust usando `clap`.

## 🛠️ Cómo ejecutar

```bash
cargo run -- --help
cargo run -- -l -w src/main.rs
echo "hola mundo" | cargo run
cargo test
```

## ✅ Criterios de Aceptación

- [ ] Flags `-l`, `-w`, `-c` funcionan individualmente y combinados
- [ ] Lee desde stdin si no se especifican archivos
- [ ] Muestra total cuando se procesan múltiples archivos
- [ ] `cargo test` pasa
