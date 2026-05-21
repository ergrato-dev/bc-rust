# Proyecto — `mi-crate`: Librería Lista para `crates.io`

## 🎯 Objetivo
Crear una librería Rust completa, documentada y lista para publicar en `crates.io`.

## 📋 Instrucciones

1. Implementar `slugify`, `truncar`, `contar_palabras`, `titular` con doctests
2. Activar `#![deny(missing_docs)]` — toda la API pública debe estar documentada
3. Completar el `Cargo.toml` con `description`, `license`, `keywords`, `categories`, `repository`
4. Verificar con `cargo publish --dry-run` (no publicar realmente)
5. Generar documentación con `cargo doc --open`

## ✅ Criterios de Aceptación

- [ ] `#![deny(missing_docs)]` activo y sin warnings
- [ ] `cargo test --doc` pasa todos los doctests
- [ ] `cargo publish --dry-run` pasa sin errores
- [ ] `cargo clippy -- -D warnings` pasa limpio
- [ ] Metadata completa en `Cargo.toml`

## 💡 Pistas

- `cargo publish --dry-run` simula la publicación sin subir nada
- `cargo package --list` muestra qué archivos se incluirían en el crate
- Añadir un `README.md` con la sección `[package] readme = "README.md"` en Cargo.toml
