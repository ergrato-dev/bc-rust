# Rúbrica de Evaluación - Semana 14: Concurrencia

## 📊 Distribución de Puntos

| Categoría | Peso | Descripción |
|-----------|------|-------------|
| **Conocimiento** | 30% | Comprensión teórica de concurrencia |
| **Desempeño** | 40% | Ejercicios prácticos y debugging |
| **Producto** | 30% | Proyecto final (Thread Pool) |

---

## 📚 Conocimiento (30%)

### Excelente (90-100%)
- Explica claramente la diferencia entre concurrencia y paralelismo
- Comprende profundamente Send y Sync
- Identifica correctamente cuándo usar channels vs mutex
- Reconoce y previene deadlocks

### Bueno (70-89%)
- Entiende el modelo de concurrencia de Rust
- Usa correctamente las primitivas básicas
- Comprende los conceptos de data races

### Suficiente (50-69%)
- Conoce las primitivas básicas de concurrencia
- Entiende el uso de threads
- Comprende el concepto de mutex

### Insuficiente (<50%)
- No comprende el modelo de concurrencia
- Confunde conceptos fundamentales
- No entiende Send/Sync

---

## 💻 Desempeño (40%)

### Práctica 01: Threads (10%)

| Criterio | Puntos |
|----------|--------|
| Crea threads correctamente | 3 |
| Usa `join()` apropiadamente | 3 |
| Maneja `move` en closures | 2 |
| Recupera valores de threads | 2 |

### Práctica 02: Channels (10%)

| Criterio | Puntos |
|----------|--------|
| Implementa MPSC básico | 3 |
| Maneja múltiples productores | 3 |
| Usa iteradores sobre receiver | 2 |
| Implementa timeout (opcional) | 2 |

### Práctica 03: Mutex (10%)

| Criterio | Puntos |
|----------|--------|
| Usa Arc<Mutex<T>> correctamente | 4 |
| Evita deadlocks | 3 |
| Maneja errores de lock | 3 |

### Práctica 04: Patrones (10%)

| Criterio | Puntos |
|----------|--------|
| Implementa producer-consumer | 4 |
| Usa RwLock apropiadamente | 3 |
| Aplica patrones de sincronización | 3 |

---

## 🏆 Producto - Thread Pool (30%)

### Funcionalidad (15%)

| Criterio | Puntos |
|----------|--------|
| Pool de workers funcional | 5 |
| Distribución de tareas correcta | 4 |
| Shutdown graceful | 3 |
| Manejo de errores | 3 |

### Calidad de Código (10%)

| Criterio | Puntos |
|----------|--------|
| Sin data races | 4 |
| Código idiomático | 3 |
| Documentación | 3 |

### Testing (5%)

| Criterio | Puntos |
|----------|--------|
| Tests unitarios pasan | 2 |
| Tests de concurrencia | 2 |
| Sin race conditions en tests | 1 |

---

## 🎯 Rúbrica de Calificación

| Nota | Rango | Descripción |
|------|-------|-------------|
| A | 90-100% | Excelente dominio de concurrencia |
| B | 80-89% | Buen manejo de primitivas |
| C | 70-79% | Comprensión básica sólida |
| D | 60-69% | Necesita refuerzo en conceptos |
| F | <60% | No cumple objetivos mínimos |

---

## ✅ Criterios de Aprobación

1. **Código compila** sin errores
2. **Tests pasan** (`cargo test`)
3. **Sin data races** detectables
4. **Mínimo 60%** de puntuación total
5. **Thread Pool funcional** con al menos 2 workers

---

## 📝 Notas Adicionales

### Debugging de Concurrencia

- Usar `RUST_BACKTRACE=1` para stack traces
- Herramientas: `cargo-miri` para detectar data races
- Tests con `--test-threads=1` para reproducir bugs

### Criterios Especiales

- **Deadlock detectado**: -5 puntos
- **Data race**: -10 puntos (falla automática)
- **Uso creativo de patrones**: +5 puntos extra
