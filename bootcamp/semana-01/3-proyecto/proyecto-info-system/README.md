# 🎯 Proyecto Semana 01: Sistema de Información Personal

## 📋 Descripción

Crear un programa CLI que muestre información personal y del bootcamp de forma organizada y visualmente atractiva.

---

## 🎯 Objetivos de Aprendizaje

Al completar este proyecto, habrás practicado:

- ✅ Crear proyectos con Cargo
- ✅ Usar `println!` con diferentes formatos
- ✅ Declarar variables con `let`
- ✅ Usar strings y números
- ✅ Organizar código con funciones

---

## 📝 Requisitos

### Funcionalidad Mínima

1. **Banner de bienvenida** con ASCII art
2. **Información personal**:
   - Nombre
   - Rol/Profesión
   - País/Ciudad
3. **Información del bootcamp**:
   - Semana actual
   - Temas de la semana
   - Progreso (1/16 semanas)
4. **Estadísticas del día**:
   - Fecha actual (puedes hardcodear)
   - Horas de estudio planeadas
   - Mood/Energía

### Formato de Output

```
╔══════════════════════════════════════════╗
║    🦀 BOOTCAMP RUST: ZERO TO HERO 🦀    ║
╠══════════════════════════════════════════╣
║  Estudiante: [Tu Nombre]                 ║
║  Rol: [Tu Rol]                           ║
║  Ubicación: [Tu Ciudad]                  ║
╠══════════════════════════════════════════╣
║  📅 Semana: 01/16                        ║
║  📚 Tema: Introducción a Rust            ║
║  📊 Progreso: ▓░░░░░░░░░░░░░░░ 6%       ║
╠══════════════════════════════════════════╣
║  ⚡ Energía hoy: ████████░░ 80%          ║
║  ⏰ Horas planeadas: 4                   ║
╚══════════════════════════════════════════╝
```

---

## 🏗️ Estructura del Proyecto

```
proyecto-info-system/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

---

## 💡 Guía de Implementación

### Paso 1: Crear el proyecto

```bash
cargo new proyecto-info-system
cd proyecto-info-system
```

### Paso 2: Planificar las funciones

```rust
fn main() {
    mostrar_banner();
    mostrar_info_personal();
    mostrar_info_bootcamp();
    mostrar_estadisticas();
    mostrar_footer();
}

fn mostrar_banner() {
    // TODO: Implementar
}

// ... más funciones
```

### Paso 3: Implementar cada función

Empieza por `mostrar_banner()` y avanza función por función.

---

## ✅ Rúbrica de Evaluación

| Criterio | Puntos |
|----------|--------|
| Compila sin errores | 20 |
| Estructura organizada (funciones) | 20 |
| Output formateado y legible | 20 |
| Información completa | 20 |
| Código limpio (clippy) | 10 |
| Creatividad/Extras | 10 |
| **Total** | **100** |

---

## 🚀 Extras Opcionales

- [ ] Agregar colores (investigar `println!` con ANSI codes)
- [ ] Calcular progreso dinámicamente
- [ ] Agregar una sección de "Próximos pasos"
- [ ] Incluir un quiz interactivo (aunque aún no sabemos input)

---

## 📚 Recursos

- [ASCII Box Drawing Characters](https://en.wikipedia.org/wiki/Box-drawing_character)
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors)
- [Rust Formatting](https://doc.rust-lang.org/std/fmt/)

---

## 📤 Entrega

1. Asegúrate de que `cargo run` funciona
2. Verifica con `cargo clippy`
3. Formatea con `cargo fmt`
4. Captura de pantalla del output

---

**¡Buena suerte! 🦀**
