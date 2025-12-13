# 🐳 Práctica 01: Setup del Entorno Docker

## 📋 Información

| Campo | Valor |
|-------|-------|
| **Duración** | 20-30 minutos |
| **Dificultad** | ⭐ Principiante |
| **Requisitos** | Docker Desktop instalado |

---

## 🎯 Objetivo

Configurar el entorno de desarrollo usando Docker para tener Rust listo sin instalación local.

---

## 📝 Pasos

### Paso 1: Verificar Docker

Abre una terminal y verifica que Docker está instalado:

```bash
docker --version
```

**Output esperado:**
```
Docker version 24.x.x, build xxxxxxx
```

> ⚠️ Si no tienes Docker, descárgalo de https://docker.com/get-docker

---

### Paso 2: Clonar el Repositorio

```bash
# Clonar el bootcamp
git clone https://github.com/ergrato-dev/bc-rust.git

# Entrar al directorio
cd bc-rust
```

---

### Paso 3: Construir la Imagen Docker

```bash
# Construir la imagen (puede tomar unos minutos la primera vez)
docker compose build
```

**Qué está pasando:**
- Docker descarga la imagen base `rust:1.92-slim-bookworm`
- Instala herramientas adicionales (clippy, rustfmt, cargo-watch, etc.)
- Crea una imagen lista para desarrollo

---

### Paso 4: Iniciar el Contenedor

```bash
# Iniciar shell interactivo en el contenedor
docker compose run --rm rust-dev
```

**Ahora estás dentro del contenedor!** Verás algo como:
```
root@abc123:/workspace#
```

---

### Paso 5: Verificar Rust

Dentro del contenedor, ejecuta:

```bash
# Verificar versión de Rust
rustc --version

# Verificar Cargo
cargo --version

# Verificar Clippy
cargo clippy --version

# Verificar rustfmt
rustfmt --version
```

**Output esperado:**
```
rustc 1.92.0 (xxxxxxx 2025-xx-xx)
cargo 1.92.0 (xxxxxxx 2025-xx-xx)
clippy 0.1.92 (xxxxxxx 2025-xx-xx)
rustfmt 1.8.0-stable (xxxxxxx 2025-xx-xx)
```

---

### Paso 6: Crear un Proyecto de Prueba

```bash
# Crear proyecto
cargo new test-setup

# Entrar al proyecto
cd test-setup

# Ejecutar
cargo run
```

**Output esperado:**
```
   Compiling test-setup v0.1.0 (/workspace/test-setup)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/test-setup`
Hello, world!
```

---

### Paso 7: Verificar Herramientas

```bash
# Verificar formato
cargo fmt --check

# Verificar linting
cargo clippy

# Verificar tests (aunque no hay tests aún)
cargo test
```

---

### Paso 8: Salir del Contenedor

```bash
# Salir del contenedor
exit
```

---

## ✅ Checklist de Verificación

- [ ] Docker versión 24+ instalado
- [ ] Repositorio clonado correctamente
- [ ] Imagen Docker construida sin errores
- [ ] `rustc --version` muestra 1.92
- [ ] `cargo run` ejecuta "Hello, world!"
- [ ] `cargo clippy` funciona
- [ ] `cargo fmt` funciona

---

## 🐛 Solución de Problemas

### Error: "Cannot connect to Docker daemon"

**Solución**: Asegúrate de que Docker Desktop está corriendo.

```bash
# En Linux, inicia el servicio
sudo systemctl start docker
```

### Error: "Permission denied"

**Solución**: Añade tu usuario al grupo docker:

```bash
sudo usermod -aG docker $USER
# Luego cierra sesión y vuelve a entrar
```

### Error: "Port already in use"

**Solución**: Otro servicio está usando el puerto. Detén el servicio o cambia el puerto en docker-compose.yml.

---

## 📸 Evidencia de Completación

Toma una captura de pantalla mostrando:
1. Output de `rustc --version`
2. Output de `cargo run` con "Hello, world!"

---

## 🎉 ¡Felicidades!

Tu entorno de desarrollo Rust está listo. Puedes:
- Escribir código en tu editor favorito
- Los cambios se reflejan automáticamente en el contenedor
- Usar todos los comandos de Cargo

**Siguiente práctica**: Cargo Basics
