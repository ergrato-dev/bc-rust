# Opción D — Librería de Hashing con API C

## 🎯 Descripción
Implementación de algoritmos FNV-1a y DJB2 expuestos via API C con header generado por `cbindgen`.

## 🛠️ Cómo compilar

```bash
# Build (genera capstone_d.h automáticamente)
cargo build

# Tests
cargo test

# Ver el header generado
cat capstone_d.h
```

## ✅ Criterios de Aceptación

- [ ] `fnv1a_64`, `djb2`, `hash_completo` exportadas con `#[no_mangle]`
- [ ] Header C generado con cbindgen
- [ ] `// SAFETY:` en todos los bloques unsafe
- [ ] `cargo test` pasa
