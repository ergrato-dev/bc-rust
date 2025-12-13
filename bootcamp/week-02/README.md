# 📚 Semana 02: Variables y Tipos de Datos

## 📋 Información General

| Campo | Valor |
|-------|-------|
| **Semana** | 02 de 16 |
| **Tema** | Variables y Tipos de Datos |
| **Duración** | 4 horas |
| **Nivel** | Principiante |

---

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

- ✅ Declarar variables inmutables y mutables
- ✅ Entender el sistema de tipos de Rust
- ✅ Usar tipos primitivos (enteros, flotantes, booleanos, caracteres)
- ✅ Aplicar shadowing correctamente
- ✅ Declarar constantes y entender sus diferencias con variables
- ✅ Comprender la inferencia de tipos

---

## 📖 Contenido

### 1. Teoría (1-teoria/)

| # | Archivo | Tema | Duración |
|---|---------|------|----------|
| 01 | [Variables Inmutables](./1-teoria/01-variables-inmutables.md) | `let`, inmutabilidad por defecto | 20 min |
| 02 | [Variables Mutables](./1-teoria/02-variables-mutables.md) | `let mut`, cuándo usar mutabilidad | 20 min |
| 03 | [Tipos Primitivos](./1-teoria/03-tipos-primitivos.md) | Enteros, flotantes, bool, char | 30 min |
| 04 | [Shadowing](./1-teoria/04-shadowing.md) | Re-declaración de variables | 20 min |
| 05 | [Constantes](./1-teoria/05-constantes.md) | `const`, `static`, diferencias | 20 min |

### 2. Prácticas Guiadas (2-practica/)

| # | Práctica | Duración | Descripción |
|---|----------|----------|-------------|
| 01 | [Declaración de Variables](./2-practica/practica-01-declaracion-variables/) | 25 min | Inmutables, mutables, inferencia |
| 02 | [Tipos Numéricos](./2-practica/practica-02-tipos-numericos/) | 30 min | Enteros, flotantes, operaciones |
| 03 | [Tipos Texto](./2-practica/practica-03-tipos-texto/) | 25 min | char, String, &str |
| 04 | [Shadowing Avanzado](./2-practica/practica-04-shadowing-avanzado/) | 30 min | Casos de uso y patrones |

### 3. Proyecto Semanal (3-proyecto/)

| Proyecto | Descripción |
|----------|-------------|
| [Calculadora de Tipos](./3-proyecto/proyecto-calculadora-tipos/) | Calculadora que demuestra conversiones y operaciones entre tipos |

### 4. Recursos (4-recursos/)

- [Ebooks Gratuitos](./4-recursos/ebook-free/)
- [Videografía](./4-recursos/videografia/)
- [Webgrafía](./4-recursos/webgrafia/)

### 5. Glosario (5-glosario/)

- [Términos de la Semana](./5-glosario/)

---

## 🗓️ Distribución del Tiempo (4 horas)

| Actividad | Tiempo | Porcentaje |
|-----------|--------|------------|
| Teoría | 1h 50min | 46% |
| Prácticas | 1h 50min | 46% |
| Proyecto | 20min | 8% |

---

## 📊 Evaluación

| Componente | Peso | Descripción |
|------------|------|-------------|
| Conocimiento | 30% | Comprensión de tipos y variables |
| Desempeño | 40% | Prácticas completadas |
| Producto | 30% | Proyecto semanal funcional |

Ver [Rúbrica de Evaluación](./rubrica-evaluacion.md) para detalles.

---

## 🔗 Conexión con Otras Semanas

```
Semana 01          Semana 02           Semana 03
───────────────────────────────────────────────────
 Setup &     →    Variables    →    Ownership
 Hello World      y Tipos           y Borrowing
                     ↓
            Fundamentos para
            entender ownership
```

---

## 💡 Conceptos Clave

### Por qué Inmutabilidad por Defecto

```rust
let x = 5;     // Inmutable: seguro, predecible
let mut y = 5; // Mutable: explícito, intencional
```

Rust prefiere seguridad: debes **optar explícitamente** por la mutabilidad.

### Sistema de Tipos

```rust
// Rust es fuertemente tipado
let numero: i32 = 42;      // Explícito
let otro = 42;             // Inferido como i32
let decimal: f64 = 3.14;   // Flotante de 64 bits
let activo: bool = true;   // Booleano
let letra: char = 'R';     // Carácter Unicode
```

---

## 📚 Recursos Previos Requeridos

- ✅ Semana 01 completada
- ✅ Entorno Docker funcionando
- ✅ Familiaridad con `cargo run`

---

## 🚀 Cómo Empezar

```bash
# 1. Iniciar contenedor
docker compose run --rm rust-dev

# 2. Navegar a la semana
cd bootcamp/semana-02

# 3. Comenzar con la teoría
cat 1-teoria/01-variables-inmutables.md
```

---

## 🔑 Puntos Clave a Recordar

1. **Variables son inmutables por defecto** - Usa `mut` explícitamente
2. **Rust tiene inferencia de tipos** - Pero puedes ser explícito
3. **Shadowing ≠ Mutabilidad** - Son conceptos diferentes
4. **Constantes son siempre inmutables** - Y requieren tipo explícito
5. **Los tipos tienen tamaño fijo** - `i32` siempre es 32 bits

---

**¡Buena suerte con la Semana 02! 🦀**
