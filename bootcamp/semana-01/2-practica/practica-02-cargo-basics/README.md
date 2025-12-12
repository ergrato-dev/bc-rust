# 📦 Práctica 02: Cargo Basics

## 📋 Información

| Campo | Valor |
|-------|-------|
| **Duración** | 25-35 minutos |
| **Dificultad** | ⭐ Principiante |
| **Requisitos** | Práctica 01 completada |

---

## 🎯 Objetivo

Dominar los comandos básicos de Cargo: crear proyectos, compilar, ejecutar y verificar código.

---

## 📝 Pasos

### Paso 1: Iniciar el Contenedor

```bash
cd bc-rust
docker compose run --rm rust-dev
```

---

### Paso 2: Crear un Nuevo Proyecto

```bash
# Navegar al directorio de prácticas
cd bootcamp/semana-01/2-practica

# Crear nuevo proyecto
cargo new mi-primer-proyecto

# Ver estructura creada
ls -la mi-primer-proyecto
```

**Output esperado:**
```
mi-primer-proyecto/
├── Cargo.toml
└── src/
    └── main.rs
```

---

### Paso 3: Explorar Cargo.toml

```bash
cd mi-primer-proyecto
cat Cargo.toml
```

**Contenido:**
```toml
[package]
name = "mi-primer-proyecto"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**Explora cada campo:**
- `name`: Nombre del proyecto
- `version`: Versión semántica (MAJOR.MINOR.PATCH)
- `edition`: Versión de Rust (2015, 2018, 2021)
- `[dependencies]`: Donde agregarás crates externos

---

### Paso 4: Explorar main.rs

```bash
cat src/main.rs
```

**Contenido:**
```rust
fn main() {
    println!("Hello, world!");
}
```

---

### Paso 5: Compilar con cargo build

```bash
# Compilar en modo debug
cargo build
```

**Output:**
```
   Compiling mi-primer-proyecto v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

**Observa:**
```bash
# Ver archivos generados
ls -la target/debug/
```

El ejecutable está en `target/debug/mi-primer-proyecto`

---

### Paso 6: Ejecutar con cargo run

```bash
# Compilar Y ejecutar en un solo comando
cargo run
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s)
     Running `target/debug/mi-primer-proyecto`
Hello, world!
```

> 💡 `cargo run` es más conveniente que `cargo build` + ejecutar manualmente

---

### Paso 7: Verificar con cargo check

```bash
# Verificar errores sin generar binario (más rápido)
cargo check
```

**Output:**
```
    Checking mi-primer-proyecto v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

> 💡 Usa `cargo check` mientras desarrollas para feedback rápido

---

### Paso 8: Formatear con cargo fmt

Primero, desordena el código:

```bash
# Editar main.rs (puedes usar nano, vim, o tu editor)
cat > src/main.rs << 'EOF'
fn main(){println!("Hello, world!");let x=5;let y=10;println!("{}",x+y);}
EOF
```

Ahora formatea:

```bash
cargo fmt
```

Verifica el resultado:
```bash
cat src/main.rs
```

**Resultado formateado:**
```rust
fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 10;
    println!("{}", x + y);
}
```

---

### Paso 9: Linting con cargo clippy

```bash
cargo clippy
```

Si hay sugerencias, Clippy las mostrará. Por ejemplo:
```
warning: this could be simplified...
```

> 💡 Clippy detecta patrones que podrían mejorarse

---

### Paso 10: Ejecutar Tests

```bash
cargo test
```

**Output:**
```
   Compiling mi-primer-proyecto v0.1.0
    Finished `test` profile [unoptimized + debuginfo] target(s)
     Running unittests src/main.rs

running 0 tests

test result: ok. 0 passed; 0 filtered out; finished in 0.00s
```

> No hay tests aún, pero el framework está listo.

---

### Paso 11: Build en Release

```bash
# Compilar optimizado para producción
cargo build --release
```

**Output:**
```
   Compiling mi-primer-proyecto v0.1.0
    Finished `release` profile [optimized] target(s)
```

El binario optimizado está en `target/release/`

**Compara tamaños:**
```bash
ls -lh target/debug/mi-primer-proyecto
ls -lh target/release/mi-primer-proyecto
```

---

### Paso 12: Limpiar

```bash
# Eliminar archivos compilados
cargo clean

# Verificar
ls target/
# Error: No such file or directory (porque se eliminó)
```

---

## 📊 Resumen de Comandos

| Comando | Descripción | Uso |
|---------|-------------|-----|
| `cargo new` | Crear proyecto | Inicio |
| `cargo build` | Compilar | Desarrollo |
| `cargo run` | Compilar + Ejecutar | Desarrollo |
| `cargo check` | Verificar errores | Feedback rápido |
| `cargo test` | Ejecutar tests | Testing |
| `cargo fmt` | Formatear código | Calidad |
| `cargo clippy` | Linting | Calidad |
| `cargo doc` | Generar docs | Documentación |
| `cargo clean` | Limpiar builds | Mantenimiento |

---

## ✅ Checklist

- [ ] Proyecto creado con `cargo new`
- [ ] Entendido `Cargo.toml`
- [ ] Compilado con `cargo build`
- [ ] Ejecutado con `cargo run`
- [ ] Verificado con `cargo check`
- [ ] Formateado con `cargo fmt`
- [ ] Analizado con `cargo clippy`
- [ ] Build release generado

---

## 🎉 ¡Felicidades!

Ahora dominas los comandos básicos de Cargo. Estos comandos los usarás constantemente durante todo el bootcamp.

**Siguiente práctica**: Hello World Personalizado
