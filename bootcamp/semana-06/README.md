# 🚨 Semana 06: Manejo de Errores

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Tema** | Manejo de Errores en Rust |
| **Duración** | 4 horas |
| **Nivel** | Intermedio |
| **Prerequisitos** | Semana 05 (Enums, Option, Result) |

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. ✅ Diferenciar entre errores recuperables e irrecuperables
2. ✅ Usar `Result<T, E>` para manejo robusto de errores
3. ✅ Propagar errores con el operador `?`
4. ✅ Crear tipos de error personalizados
5. ✅ Implementar el trait `Error`
6. ✅ Usar `panic!` apropiadamente
7. ✅ Aplicar patrones de manejo de errores idiomáticos

## 📚 Contenido

### Teoría

| # | Tema | Archivo | Tiempo |
|---|------|---------|--------|
| 1 | Errores Recuperables vs Irrecuperables | [01-tipos-de-errores.md](1-teoria/01-tipos-de-errores.md) | 30 min |
| 2 | Result en Profundidad | [02-result-profundidad.md](1-teoria/02-result-profundidad.md) | 35 min |
| 3 | Propagación con ? | [03-propagacion-errores.md](1-teoria/03-propagacion-errores.md) | 30 min |
| 4 | Errores Personalizados | [04-errores-personalizados.md](1-teoria/04-errores-personalizados.md) | 35 min |
| 5 | Patrones y Buenas Prácticas | [05-patrones-practicas.md](1-teoria/05-patrones-practicas.md) | 30 min |

### Prácticas

| # | Práctica | Descripción | Tiempo |
|---|----------|-------------|--------|
| 1 | [Result Básico](2-practica/practica-01-result-basico/) | Funciones que retornan Result | 30 min |
| 2 | [Propagación](2-practica/practica-02-propagacion/) | Uso del operador ? | 30 min |
| 3 | [Errores Custom](2-practica/practica-03-errores-custom/) | Crear tipos de error | 35 min |
| 4 | [Conversión](2-practica/practica-04-conversion/) | From, Into para errores | 30 min |

### Proyecto Semanal

| Proyecto | Descripción | Tiempo |
|----------|-------------|--------|
| [Validador de Configuración](3-proyecto/proyecto-validador-config/) | Sistema que parsea y valida archivos de configuración | 60 min |

## 📊 Distribución del Tiempo

```
Total: 4 horas (240 minutos)

Teoría:          160 min (67%)
├── Lectura:      80 min
└── Ejemplos:     80 min

Práctica:        125 min (52%)
├── Ejercicios:   65 min
└── Proyecto:     60 min
```

## 🔑 Conceptos Clave

```rust
// Errores recuperables
fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(ruta)
}

// Propagación con ?
fn procesar() -> Result<(), Error> {
    let contenido = leer_archivo("config.txt")?;
    let config = parsear(&contenido)?;
    validar(&config)?;
    Ok(())
}

// Error personalizado
#[derive(Debug)]
enum MiError {
    Io(io::Error),
    Parse(String),
    Validacion { campo: String, mensaje: String },
}
```

## 📦 Recursos

- [E-books gratuitos](4-recursos/ebook-free/)
- [Videografía](4-recursos/videografia/)
- [Webgrafía](4-recursos/webgrafia/)

## 📖 Glosario

Ver [Glosario de la Semana](5-glosario/README.md)

## ✅ Criterios de Evaluación

Ver [Rúbrica de Evaluación](RUBRICA_EVALUACION.md)

---

## 🧭 Navegación

| ← Anterior | Inicio | Siguiente → |
|------------|--------|-------------|
| [Semana 05: Enums](../semana-05/) | [Bootcamp](../BOOTCAMP-COMPLETO.md) | [Semana 07: Módulos](../semana-07/) |
