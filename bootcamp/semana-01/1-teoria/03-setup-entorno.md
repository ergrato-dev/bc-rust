# 🐳 Setup del Entorno con Docker

## Introducción

En este bootcamp usamos **Docker** para garantizar un entorno de desarrollo consistente. No importa si usas Windows, macOS o Linux, todos tendremos el mismo entorno.

![Setup Docker](../0-assets/05-setup-docker.svg)

---

## 🤔 ¿Por qué Docker?

| Beneficio | Descripción |
|-----------|-------------|
| **Consistencia** | Mismo entorno para todos |
| **Aislamiento** | No afecta tu sistema |
| **Portabilidad** | Funciona en cualquier OS |
| **Versionado** | Rust 1.92 garantizado |
| **Fácil setup** | Un comando para empezar |

---

## 📋 Prerrequisitos

### 1. Docker Desktop

Descarga e instala Docker Desktop:

| Sistema | Enlace |
|---------|--------|
| **Windows** | [Docker Desktop para Windows](https://docs.docker.com/desktop/install/windows-install/) |
| **macOS** | [Docker Desktop para Mac](https://docs.docker.com/desktop/install/mac-install/) |
| **Linux** | [Docker Engine para Linux](https://docs.docker.com/engine/install/) |

### 2. Verificar Instalación

```bash
# Verificar Docker
docker --version
# Docker version 24.x.x

# Verificar Docker Compose
docker compose version
# Docker Compose version v2.x.x
```

### 3. VS Code (Recomendado)

- [Descargar VS Code](https://code.visualstudio.com/)
- Instalar extensión: **Dev Containers**

---

## 🚀 Opción 1: Dev Container (Recomendado)

La forma más fácil de comenzar:

### Paso 1: Clonar el repositorio

```bash
git clone https://github.com/ergrato-dev/bc-rust.git
cd bc-rust
```

### Paso 2: Abrir en VS Code

```bash
code .
```

### Paso 3: Reabrir en Container

VS Code mostrará un popup:

```
Folder contains a Dev Container configuration file.
Reopen folder in container?
          [Reopen in Container]
```

Click en **"Reopen in Container"**

### Paso 4: Esperar la construcción

La primera vez tomará unos minutos. Verás:

```
Starting Dev Container...
Building image...
Installing extensions...
```

### Paso 5: ¡Listo!

Abre una terminal en VS Code y verifica:

```bash
rustc --version
# rustc 1.92.0

cargo --version
# cargo 1.92.0
```

---

## 🐳 Opción 2: Docker Compose

Si prefieres usar la terminal directamente:

### Paso 1: Construir la imagen

```bash
cd bc-rust
docker compose build
```

### Paso 2: Iniciar contenedor interactivo

```bash
docker compose run --rm rust-dev
```

### Paso 3: Verificar instalación

```bash
# Dentro del contenedor
rustc --version
cargo --version
```

### Paso 4: Trabajar en el proyecto

```bash
# Los archivos están en /workspace
cd /workspace
ls -la
```

---

## 💻 Opción 3: Docker Directo

Para comandos rápidos:

```bash
# Ejecutar cargo
docker run --rm -v $(pwd):/workspace -w /workspace rust:1.92-slim-bookworm cargo --version

# Shell interactivo
docker run -it --rm -v $(pwd):/workspace -w /workspace rust:1.92-slim-bookworm bash
```

---

## 📁 Estructura del Contenedor

```
/workspace/                    ← Tu código (montado desde el host)
├── bootcamp/
│   ├── semana-01/
│   └── ...
├── Cargo.toml
└── ...

/usr/local/cargo/             ← Cargo y herramientas
/usr/local/rustup/            ← Rust y componentes
```

---

## 🔧 Comandos Docker Útiles

### Servicios disponibles

| Comando | Descripción |
|---------|-------------|
| `docker compose run --rm rust-dev` | Shell interactivo |
| `docker compose run --rm rust-run` | Ejecutar cargo run |
| `docker compose run --rm rust-test` | Ejecutar tests |
| `docker compose run --rm rust-lint` | Clippy + fmt check |

### Gestión

```bash
# Ver contenedores activos
docker ps

# Limpiar todo
docker compose down -v

# Reconstruir imagen
docker compose build --no-cache
```

---

## ⚠️ Solución de Problemas

### Error: "Cannot connect to Docker daemon"

```bash
# Linux: iniciar Docker
sudo systemctl start docker

# Verificar que tu usuario está en grupo docker
sudo usermod -aG docker $USER
# Reiniciar sesión
```

### Error: "Port already in use"

```bash
# Encontrar proceso
lsof -i :8080
# Terminar proceso
kill -9 <PID>
```

### Compilación muy lenta

El `docker-compose.yml` incluye caché de Cargo. Si aún es lento:

```bash
# Limpiar y reconstruir
docker compose down -v
docker compose build
```

---

## ✅ Verificación Final

Ejecuta estos comandos para confirmar que todo funciona:

```bash
# 1. Entrar al contenedor
docker compose run --rm rust-dev

# 2. Verificar versiones
rustc --version    # rustc 1.92.0
cargo --version    # cargo 1.92.0
rustfmt --version  # rustfmt 1.x.x
cargo clippy --version  # clippy 0.1.x

# 3. Crear proyecto de prueba
cargo new test-project
cd test-project
cargo run
# Output: Hello, world!

# 4. Limpiar
cd ..
rm -rf test-project
```

Si todo funciona, ¡estás listo para comenzar! 🎉

---

## 📝 Resumen

| Paso | Acción |
|------|--------|
| 1 | Instalar Docker |
| 2 | Clonar repositorio |
| 3 | Abrir en VS Code + Dev Container |
| 4 | Verificar `rustc --version` |

---

**Anterior**: [¿Por qué aprender Rust?](02-porque-rust.md)  
**Siguiente**: [Introducción a Cargo](04-cargo-basics.md)
