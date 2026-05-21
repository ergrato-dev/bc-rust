---
applyTo: "**"
---

# Gestión de Dependencias Cargo — Bootcamp bc-rust

## ⛔ PROHIBIDO en Cargo.toml

Nunca usar rangos de versión con comodines, rangos abiertos ni palabras clave flotantes:

```toml
# ❌ NUNCA — versiones flotantes (riesgo de CVE y builds no reproducibles)
serde = "*"
tokio = ">=1.0"
axum  = "^0.7"
reqwest = "~0.11"
```

## ✅ OBLIGATORIO — versión exacta siempre

```toml
# ✅ SIEMPRE — versión exacta, sin prefijos
serde   = { version = "1.0.219", features = ["derive"] }
tokio   = { version = "1.44.2",  features = ["full"] }
axum    = "0.8.4"
reqwest = { version = "0.12.15", default-features = false, features = ["json"] }
```

## Motivación

| Riesgo | Detalle |
|--------|---------|
| CVEs no controlados | `^0.7` puede instalar `0.7.99` con vulnerabilidades conocidas |
| Builds no reproducibles | Dos `cargo build` en fechas distintas pueden usar versiones distintas |
| Supply-chain attacks | Una actualización automática puede inyectar código malicioso |
| Auditorías imposibles | No se puede fijar exactamente qué versión se ejecuta en producción |

## Auditoría CVE — obligatoria antes de usar cualquier dependencia

### Flujo obligatorio al agregar una dependencia nueva

```bash
# 1. Consultar la versión exacta más reciente
cargo search nombre-crate | head -3

# 2. Agregar con versión exacta (usar = para forzar exacta)
cargo add nombre-crate@X.Y.Z

# 3. Auditar inmediatamente
cargo audit

# 4. Si hay advisories: evaluar, actualizar o elegir alternativa
# NO continuar si hay vulnerabilidades HIGH o CRITICAL sin resolver
```

### Niveles de severidad y acción requerida

| Nivel | Acción |
|-------|--------|
| **CRITICAL** | Bloquea el commit. Debe resolverse antes de merge. |
| **HIGH** | Bloquea el commit. Debe resolverse o justificarse con `cargo audit --ignore RUSTSEC-XXXX-XXXX`. |
| **MEDIUM** | Registrar en CHANGELOG. Resolver en el próximo sprint. |
| **LOW** | Informativo. Monitorear. |

### Comando de auditoría fuerte

```bash
# Auditoría con fallo en MEDIUM o superior (recomendado para CI)
cargo audit --deny warnings

# Auditoría con fallo solo en HIGH/CRITICAL
cargo audit --deny unsound --deny yanked
```

## Herramientas

```bash
# Instalar cargo-audit (versión exacta)
cargo install cargo-audit@0.21.2

# Ver árbol de dependencias (detectar duplicados)
cargo tree

# Ver qué versiones están disponibles
cargo search nombre-crate

# Actualizar lockfile manteniendo restricciones de Cargo.toml
cargo update --precise X.Y.Z --package nombre-crate
```

## Cargo.lock — política de commit

- **Binarios / aplicaciones**: `Cargo.lock` SIEMPRE en git → builds reproducibles
- **Librerías**: `Cargo.lock` en `.gitignore` → permite compatibilidad con dependientes

## Regla para ejercicios y proyectos del bootcamp

```toml
# ✅ Ejemplo correcto en un ejercicio del bootcamp
[package]
name    = "practice-17-api-rest"
version = "0.1.0"
edition = "2021"

[dependencies]
axum        = "0.8.4"
tokio       = { version = "1.44.2", features = ["full"] }
serde       = { version = "1.0.219", features = ["derive"] }
serde_json  = "1.0.140"
sqlx        = { version = "0.8.5", features = ["sqlite", "runtime-tokio"] }

[dev-dependencies]
tokio-test  = "0.4.4"
```

## Cuando Copilot genere código

- Al escribir `Cargo.toml`, usar siempre versión exacta sin `^`, `~`, `*`, `>=`
- Si no se especifica versión, consultar `cargo search` para obtener la última estable
- Nunca sugerir `cargo add paquete` sin `@X.Y.Z` al final
- Después de agregar dependencias, incluir siempre `cargo audit` en el siguiente paso
