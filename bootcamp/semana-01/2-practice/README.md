# 🛠️ Prácticas Guiadas - Semana 01

## 📋 Descripción

Esta carpeta contiene **prácticas guiadas paso a paso** para la Semana 01 del Bootcamp Rust.

Cada práctica te lleva de la mano con instrucciones detalladas, comandos específicos y checkpoints de verificación.

---

## 📚 Lista de Prácticas

| # | Práctica | Duración | Descripción |
|---|----------|----------|-------------|
| 01 | [Setup Docker](./practica-01-setup-docker/) | 20-30 min | Configurar el entorno de desarrollo |
| 02 | [Cargo Basics](./practica-02-cargo-basics/) | 25-35 min | Dominar los comandos básicos de Cargo |
| 03 | [Hello Personalizado](./practica-03-hello-personalizado/) | 30-40 min | Crear programa con println! y formato |
| 04 | [Exploración](./practica-04-exploracion/) | 40-50 min | Experimentar con el compilador y errores |

---

## ⏱️ Tiempo Total Estimado

**~2 horas** para completar todas las prácticas.

---

## 🎯 Objetivos de Aprendizaje

Al completar estas prácticas, serás capaz de:

- ✅ Configurar un entorno Rust con Docker
- ✅ Crear proyectos con `cargo new`
- ✅ Usar comandos básicos: `build`, `run`, `check`, `fmt`, `clippy`
- ✅ Usar `println!` con diferentes formatos
- ✅ Declarar variables con `let` y `let mut`
- ✅ Crear funciones básicas
- ✅ Leer y entender errores del compilador

---

## 📋 Prerrequisitos

1. **Docker Desktop** instalado y corriendo
2. **Repositorio clonado**: `git clone https://github.com/ergrato-dev/bc-rust.git`
3. **Editor de código** (VS Code recomendado)

---

## 🚀 Cómo Empezar

```bash
# 1. Clonar el repositorio (si no lo has hecho)
git clone https://github.com/ergrato-dev/bc-rust.git
cd bc-rust

# 2. Iniciar el contenedor
docker compose run --rm rust-dev

# 3. Navegar a las prácticas
cd bootcamp/semana-01/2-practica

# 4. Comenzar con la Práctica 01
cat practica-01-setup-docker/README.md
```

---

## ✅ Orden de Realización

Las prácticas están diseñadas para realizarse **en orden**:

```
Práctica 01 → Práctica 02 → Práctica 03 → Práctica 04
    ↓              ↓              ↓              ↓
  Setup        Cargo          println!      Explorar
```

> ⚠️ No saltes prácticas. Cada una construye sobre la anterior.

---

## 📊 Evaluación

Las prácticas representan el **40%** de la evaluación semanal.

| Práctica | Puntos |
|----------|--------|
| 01 - Setup Docker | 10 |
| 02 - Cargo Basics | 10 |
| 03 - Hello Personalizado | 10 |
| 04 - Exploración | 10 |
| **Total** | **40** |

---

## 📸 Evidencia Requerida

Para cada práctica, debes entregar:

- [ ] Captura de pantalla del output final
- [ ] Código fuente (si aplica)
- [ ] Checklist completado

---

## 🆘 Ayuda

Si te atascas:

1. **Lee el error completo** - El compilador de Rust es muy descriptivo
2. **Busca la sección `help:`** - Casi siempre hay una sugerencia
3. **Revisa la práctica anterior** - Puede que hayas saltado algo
4. **Pregunta en Discord/Foro** - La comunidad está para ayudar

---

## 📁 Estructura de Carpetas

```
2-practica/
├── README.md                              # Este archivo
├── practica-01-setup-docker/
│   └── README.md                          # Instrucciones paso a paso
├── practica-02-cargo-basics/
│   └── README.md
├── practica-03-hello-personalizado/
│   └── README.md
├── practica-04-exploracion/
│   └── README.md
└── ejercicio-01-hello-world/              # Ejercicio adicional (opcional)
    ├── Cargo.toml
    ├── src/main.rs
    └── README.md
```

---

**¡Buena suerte con las prácticas! 🦀**
