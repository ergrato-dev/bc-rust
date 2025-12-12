# 🤝 Guía de Contribución

¡Gracias por tu interés en contribuir al **Bootcamp Rust: Zero to Hero**! 🦀

Este documento proporciona las directrices para contribuir al proyecto de manera efectiva.

---

## 📋 Tabla de Contenidos

- [Código de Conducta](#código-de-conducta)
- [¿Cómo Puedo Contribuir?](#cómo-puedo-contribuir)
- [Configuración del Entorno](#configuración-del-entorno)
- [Flujo de Trabajo](#flujo-de-trabajo)
- [Guía de Estilo](#guía-de-estilo)
- [Commits](#commits)
- [Pull Requests](#pull-requests)
- [Reportar Bugs](#reportar-bugs)
- [Sugerir Mejoras](#sugerir-mejoras)

---

## 📜 Código de Conducta

Este proyecto adhiere a un [Código de Conducta](CODE_OF_CONDUCT.md). Al participar, se espera que respetes este código. Por favor, reporta comportamientos inaceptables a los mantenedores del proyecto.

---

## 🎯 ¿Cómo Puedo Contribuir?

### 📚 Contenido Educativo

- Mejorar explicaciones teóricas
- Agregar ejemplos de código
- Crear diagramas SVG (siguiendo la guía de estilo)
- Traducir contenido
- Corregir errores tipográficos

### 💻 Código

- Agregar ejercicios prácticos
- Mejorar ejercicios existentes
- Agregar tests
- Optimizar código de ejemplo

### 🐛 Bugs y Mejoras

- Reportar errores en el contenido
- Sugerir nuevos temas
- Proponer mejoras en la estructura

### 📖 Documentación

- Mejorar README y guías
- Documentar procesos
- Crear tutoriales adicionales

---

## ⚙️ Configuración del Entorno

### Prerrequisitos

- [Docker](https://docs.docker.com/get-docker/)
- [Git](https://git-scm.com/)
- [VS Code](https://code.visualstudio.com/) (recomendado)

### Clonar el Repositorio

```bash
# Fork del repositorio en GitHub, luego:
git clone git@github.com:TU-USUARIO/bc-rust.git
cd bc-rust

# Agregar upstream
git remote add upstream git@github.com:ergrato-dev/bc-rust.git
```

### Configurar Entorno de Desarrollo

```bash
# Opción 1: Dev Container (recomendado)
# Abrir en VS Code y usar "Reopen in Container"

# Opción 2: Docker Compose
docker compose build
docker compose run --rm rust-dev

# Verificar instalación
cargo --version
rustc --version
```

---

## 🔄 Flujo de Trabajo

### 1. Sincronizar con upstream

```bash
git checkout main
git fetch upstream
git merge upstream/main
```

### 2. Crear rama de trabajo

```bash
# Formato: tipo/descripcion-corta
git checkout -b docs/mejorar-semana-02
git checkout -b feat/ejercicio-ownership
git checkout -b fix/error-codigo-semana-05
```

### 3. Hacer cambios

- Seguir la [guía de estilo](#guía-de-estilo)
- Probar los cambios localmente
- Verificar con `cargo clippy` y `cargo fmt`

### 4. Commit y Push

```bash
git add .
git commit -m "tipo(scope): descripción"
git push origin tu-rama
```

### 5. Crear Pull Request

- Ir a GitHub y crear PR hacia `main`
- Completar la plantilla de PR
- Esperar revisión

---

## 🎨 Guía de Estilo

### Código Rust

```bash
# Antes de hacer commit
cargo fmt          # Formatear código
cargo clippy       # Verificar linting
cargo test         # Ejecutar tests
```

- Usar `snake_case` para funciones y variables
- Usar `PascalCase` para tipos y traits
- Documentar funciones públicas con `///`
- Evitar `unwrap()` en código de producción

### Archivos Markdown

- Usar encabezados jerárquicos (`#`, `##`, `###`)
- Incluir emojis para mejor legibilidad
- Agregar ejemplos de código con syntax highlighting
- Mantener líneas menores a 100 caracteres

### Diagramas SVG

| Aspecto | Especificación |
|---------|----------------|
| **Tema** | Dark mode obligatorio |
| **Degradados** | ❌ No usar |
| **Fuentes** | Sans-serif únicamente |
| **Fondo principal** | `#1e1e1e` |
| **Texto principal** | `#d4d4d4` |
| **Color acento** | `#CE422B` (Rust Orange) |

---

## 📝 Commits

Usamos [Conventional Commits](https://www.conventionalcommits.org/):

```
tipo(scope): descripción corta

Cuerpo opcional con más detalles.

Footer opcional (referencias a issues, breaking changes, etc.)
```

### Tipos de Commit

| Tipo | Descripción |
|------|-------------|
| `feat` | Nueva funcionalidad |
| `fix` | Corrección de bug |
| `docs` | Cambios en documentación |
| `style` | Cambios de formato (no afectan código) |
| `refactor` | Refactorización de código |
| `test` | Agregar o modificar tests |
| `chore` | Tareas de mantenimiento |
| `build` | Cambios en build o dependencias |

### Scopes Comunes

- `semana-00` a `semana-15` - Contenido por semana
- `docker` - Configuración Docker
- `assets` - Recursos visuales
- `docs` - Documentación general

### Ejemplos

```bash
git commit -m "docs(semana-02): add ownership diagram"
git commit -m "feat(semana-05): add error handling exercises"
git commit -m "fix(semana-03): correct struct example syntax"
git commit -m "chore(docker): update rust version to 1.92"
```

---

## 🔀 Pull Requests

### Antes de Crear un PR

- [ ] Código formateado (`cargo fmt`)
- [ ] Sin warnings de clippy (`cargo clippy`)
- [ ] Tests pasan (`cargo test`)
- [ ] Documentación actualizada
- [ ] Commits siguen convención

### Plantilla de PR

```markdown
## Descripción
Breve descripción de los cambios.

## Tipo de Cambio
- [ ] 📚 Documentación
- [ ] ✨ Nueva funcionalidad
- [ ] 🐛 Corrección de bug
- [ ] 🎨 Estilo/formato
- [ ] ♻️ Refactorización

## Semana(s) Afectada(s)
- [ ] Semana XX

## Checklist
- [ ] He probado los cambios localmente
- [ ] He actualizado la documentación
- [ ] Los tests pasan
```

### Proceso de Revisión

1. Un mantenedor revisará tu PR
2. Puede solicitar cambios
3. Una vez aprobado, se hará merge
4. Tu contribución aparecerá en el proyecto 🎉

---

## 🐛 Reportar Bugs

### Antes de Reportar

- Verificar que el bug no haya sido reportado antes
- Intentar reproducir el bug con la última versión
- Recopilar información relevante

### Crear Issue

Usa la plantilla de bug report e incluye:

- **Descripción clara** del problema
- **Pasos para reproducir**
- **Comportamiento esperado**
- **Comportamiento actual**
- **Capturas de pantalla** (si aplica)
- **Entorno**: OS, versión de Docker, etc.

---

## 💡 Sugerir Mejoras

### Crear Feature Request

- Describir la mejora propuesta
- Explicar el problema que resuelve
- Proporcionar ejemplos o mockups
- Indicar si estás dispuesto a implementarla

---

## 🏆 Reconocimientos

Los contribuidores serán reconocidos en:

- README principal del proyecto
- Sección de agradecimientos
- Release notes cuando aplique

---

## ❓ ¿Preguntas?

- Abre un [Issue](https://github.com/ergrato-dev/bc-rust/issues) con la etiqueta `question`
- Revisa los [Discussions](https://github.com/ergrato-dev/bc-rust/discussions)

---

¡Gracias por contribuir al Bootcamp Rust! 🦀❤️
