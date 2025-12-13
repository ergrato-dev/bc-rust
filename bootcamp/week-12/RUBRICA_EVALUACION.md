# 📋 Rúbrica de Evaluación - Semana 12: Closures e Iteradores

## 📊 Distribución de Puntos

| Componente | Puntos | Porcentaje |
|------------|--------|------------|
| Práctica 01: Closures Básicos | 15 | 15% |
| Práctica 02: Captura de Entorno | 15 | 15% |
| Práctica 03: Fn Traits | 20 | 20% |
| Práctica 04: Iteradores | 20 | 20% |
| Proyecto: Pipeline de Datos | 30 | 30% |
| **Total** | **100** | **100%** |

---

## 📝 Práctica 01: Closures Básicos (15 puntos)

### Criterios de Evaluación

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| Sintaxis correcta | 5 | Usar sintaxis de closure correctamente |
| Tipos inferidos | 3 | Aprovechar inferencia de tipos |
| Closures como argumentos | 4 | Pasar closures a funciones |
| Closures como retorno | 3 | Retornar closures de funciones |

### Niveles de Desempeño

| Nivel | Puntos | Descripción |
|-------|--------|-------------|
| Excelente | 14-15 | Domina sintaxis, tipos y uso de closures |
| Bueno | 11-13 | Usa closures correctamente con errores menores |
| Satisfactorio | 8-10 | Comprende closures pero con dificultades |
| En desarrollo | 0-7 | Necesita práctica adicional |

---

## 📝 Práctica 02: Captura de Entorno (15 puntos)

### Criterios de Evaluación

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| Captura por referencia | 4 | Capturar variables como `&T` |
| Captura por referencia mutable | 4 | Capturar como `&mut T` |
| Captura por valor (move) | 4 | Usar `move` correctamente |
| Identificar modo de captura | 3 | Predecir qué modo usa el compilador |

### Niveles de Desempeño

| Nivel | Puntos | Descripción |
|-------|--------|-------------|
| Excelente | 14-15 | Domina los tres modos de captura |
| Bueno | 11-13 | Usa captura correctamente, pequeñas confusiones |
| Satisfactorio | 8-10 | Comprende captura básica |
| En desarrollo | 0-7 | Confunde modos de captura |

---

## 📝 Práctica 03: Fn Traits (20 puntos)

### Criterios de Evaluación

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| Trait Fn | 5 | Usar closures que implementan `Fn` |
| Trait FnMut | 5 | Usar closures que implementan `FnMut` |
| Trait FnOnce | 5 | Usar closures que implementan `FnOnce` |
| Bounds en funciones | 5 | Aplicar bounds `F: Fn/FnMut/FnOnce` |

### Niveles de Desempeño

| Nivel | Puntos | Descripción |
|-------|--------|-------------|
| Excelente | 18-20 | Distingue y aplica correctamente los tres traits |
| Bueno | 14-17 | Usa traits correctamente con confusiones menores |
| Satisfactorio | 10-13 | Comprende diferencias básicas |
| En desarrollo | 0-9 | Confunde los traits Fn |

---

## 📝 Práctica 04: Iteradores (20 puntos)

### Criterios de Evaluación

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| Crear iteradores | 4 | Usar `iter()`, `into_iter()`, `iter_mut()` |
| Adaptadores (map, filter) | 5 | Transformar iteradores |
| Consumidores (collect, fold) | 5 | Consumir iteradores |
| Iteradores personalizados | 6 | Implementar trait `Iterator` |

### Niveles de Desempeño

| Nivel | Puntos | Descripción |
|-------|--------|-------------|
| Excelente | 18-20 | Domina API de iteradores y crea personalizados |
| Bueno | 14-17 | Usa iteradores efectivamente |
| Satisfactorio | 10-13 | Comprende iteradores básicos |
| En desarrollo | 0-9 | Dificultad con conceptos de iteradores |

---

## 🎯 Proyecto: Pipeline de Datos (30 puntos)

### Descripción

Crear un sistema de procesamiento de datos en pipeline usando closures e iteradores.

### Criterios de Evaluación

| Criterio | Puntos | Descripción |
|----------|--------|-------------|
| **Funcionalidad** | 12 | |
| - Pipeline configurable | 4 | Agregar/quitar etapas dinámicamente |
| - Transformaciones | 4 | Map, filter, reduce funcionando |
| - Procesamiento lazy | 4 | No procesar hasta consumir |
| **Diseño** | 10 | |
| - API ergonómica | 4 | Builder pattern o similar |
| - Uso correcto de traits | 3 | Fn bounds apropiados |
| - Manejo de errores | 3 | Result/Option donde corresponde |
| **Calidad** | 8 | |
| - Tests unitarios | 3 | Mínimo 5 tests |
| - Documentación | 2 | Documentar funciones públicas |
| - Código idiomático | 3 | Estilo Rust, clippy clean |

### Niveles de Desempeño

| Nivel | Puntos | Descripción |
|-------|--------|-------------|
| Excelente | 27-30 | Pipeline completo, bien diseñado, testeado |
| Bueno | 21-26 | Pipeline funcional con buen diseño |
| Satisfactorio | 15-20 | Pipeline básico funcionando |
| En desarrollo | 0-14 | Pipeline incompleto |

---

## ✅ Lista de Verificación

### Antes de Entregar

- [ ] Todo el código compila sin errores
- [ ] `cargo clippy` sin warnings
- [ ] `cargo fmt` aplicado
- [ ] Tests pasan (`cargo test`)
- [ ] Documentación en funciones públicas

### Conceptos Demostrados

- [ ] Crear closures con diferentes sintaxis
- [ ] Captura por referencia, mutable y move
- [ ] Distinguir Fn, FnMut, FnOnce
- [ ] Usar adaptadores de iteradores
- [ ] Usar consumidores de iteradores
- [ ] Crear iterador personalizado

---

## 📈 Escala de Calificación

| Puntos | Calificación | Descripción |
|--------|--------------|-------------|
| 90-100 | A | Excelente dominio de closures e iteradores |
| 80-89 | B | Buen manejo con áreas de mejora |
| 70-79 | C | Comprensión adecuada |
| 60-69 | D | Necesita refuerzo |
| 0-59 | F | No demuestra competencia |

---

## 🔄 Retroalimentación

### Fortalezas Comunes
- Sintaxis de closures intuitiva
- Iteradores más expresivos que loops

### Áreas de Mejora Frecuentes
- Confusión entre Fn/FnMut/FnOnce
- Olvidar que iteradores son lazy
- No usar `move` cuando es necesario

### Recursos de Apoyo
- [Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Iterator Trait Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
