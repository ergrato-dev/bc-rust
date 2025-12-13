# 📊 Rúbrica de Evaluación - Semana 03

## Structs y Métodos

---

## 📋 Distribución de Notas

| Componente | Peso | Descripción |
|------------|------|-------------|
| **Conocimiento** | 30% | Comprensión teórica |
| **Desempeño** | 40% | Prácticas y ejercicios |
| **Producto** | 30% | Proyecto semanal |

---

## 🎯 Conocimiento (30%)

### Conceptos Evaluados

| Concepto | Puntos | Criterio |
|----------|--------|----------|
| Definición de structs | 5 | Sintaxis correcta de campos |
| Instanciación | 5 | Crear instancias correctamente |
| Métodos vs funciones asociadas | 8 | Diferenciar `&self` de `Self` |
| Tipos de self | 7 | `self`, `&self`, `&mut self` |
| Constructor new() | 5 | Patrón idiomático |
| **Total** | **30** | |

### Preguntas Tipo

1. ¿Cuál es la diferencia entre un método y una función asociada?
2. ¿Cuándo usar `&self` vs `&mut self`?
3. ¿Por qué se usa `Self` en lugar del nombre del struct?
4. ¿Qué es un tuple struct y cuándo usarlo?

---

## 💻 Desempeño (40%)

### Prácticas Evaluadas

| Práctica | Puntos | Criterios |
|----------|--------|-----------|
| **Práctica 01**: Struct básico | 10 | Define struct, crea instancias |
| **Práctica 02**: Métodos | 10 | Implementa métodos con impl |
| **Práctica 03**: Constructores | 10 | Usa new() y funciones asociadas |
| **Práctica 04**: Structs avanzados | 10 | Tuple structs, structs anidados |
| **Total** | **40** | |

### Criterios de Evaluación por Práctica

#### Excelente (10 pts)
- Código compila sin warnings
- Todos los TODOs completados
- Tests pasan
- Código bien documentado

#### Bueno (7-9 pts)
- Código compila
- Mayoría de TODOs completados
- Tests pasan parcialmente

#### Suficiente (5-6 pts)
- Código compila con warnings
- Algunos TODOs completados
- Intento válido

#### Insuficiente (0-4 pts)
- No compila
- TODOs sin completar
- No demuestra comprensión

---

## 🏗️ Producto (30%)

### Proyecto: Sistema de Usuarios

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| **Estructura Usuario** | 6 | Campos apropiados y tipos correctos |
| **Constructor new()** | 4 | Función asociada implementada |
| **Métodos de lectura** | 5 | Getters con `&self` |
| **Métodos de modificación** | 5 | Setters con `&mut self` |
| **Funcionalidad extra** | 5 | Validaciones, Display |
| **Tests unitarios** | 5 | Cobertura de casos |
| **Total** | **30** | |

### Niveles de Logro

| Nivel | Puntos | Descripción |
|-------|--------|-------------|
| **Destacado** | 27-30 | Excede requisitos, código ejemplar |
| **Logrado** | 21-26 | Cumple todos los requisitos |
| **En proceso** | 15-20 | Cumple requisitos mínimos |
| **Inicial** | 0-14 | No cumple requisitos mínimos |

---

## 📝 Lista de Verificación

### Código

- [ ] Compila sin errores (`cargo build`)
- [ ] Sin warnings (`cargo clippy`)
- [ ] Formateado (`cargo fmt --check`)
- [ ] Tests pasan (`cargo test`)

### Structs

- [ ] Campos con tipos apropiados
- [ ] Nombres en PascalCase para structs
- [ ] Nombres en snake_case para campos

### Métodos

- [ ] Usa `&self` para lectura
- [ ] Usa `&mut self` para modificación
- [ ] Constructor `new()` implementado
- [ ] Documentación con `///`

---

## 🔍 Errores Comunes a Evitar

| Error | Penalización | Corrección |
|-------|--------------|------------|
| Olvidar `&` en self | -2 pts | Usar `&self` para no consumir |
| No usar Self | -1 pt | Preferir `Self` sobre nombre |
| Campos públicos sin razón | -1 pt | Encapsular con métodos |
| Sin constructor new() | -2 pts | Implementar patrón estándar |
| Tests ausentes | -3 pts | Agregar tests unitarios |

---

## 📅 Entrega

- **Formato**: Carpeta con proyecto Cargo
- **Fecha límite**: Fin de la semana
- **Método**: Push al repositorio

---

*Bootcamp Rust: Zero to Hero - Semana 03*
