# Rúbrica de Evaluación - Semana 15: Async/Await

## 📊 Distribución de Puntos

| Componente | Peso | Descripción |
|------------|------|-------------|
| **Conocimiento** | 30% | Comprensión teórica de async |
| **Desempeño** | 40% | Ejercicios prácticos |
| **Producto** | 30% | Proyecto Web Crawler |

---

## 📝 Conocimiento (30 puntos)

### Conceptos Fundamentales (15 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Future trait | 3 | Explica poll, Pin, Context |
| async/await | 3 | Entiende transformación a state machine |
| Runtime | 3 | Comprende executor y reactor |
| Diferencia sync/async | 3 | Sabe cuándo usar cada uno |
| Waker | 3 | Entiende mecanismo de notificación |

### Tokio y Ecosistema (10 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| tokio::spawn | 2 | Sabe crear tasks |
| join!/select! | 3 | Entiende concurrencia de futures |
| Channels async | 2 | Usa mpsc de tokio |
| I/O async | 3 | Diferencia entre blocking y async I/O |

### Errores y Debugging (5 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Errores comunes | 2 | Identifica blocking, forget await |
| Manejo de errores | 2 | Usa ? con async |
| Debugging | 1 | Sabe usar tracing |

---

## 💻 Desempeño (40 puntos)

### Práctica 01: Async Básico (8 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Funciones async | 2 | Define y llama funciones async |
| await correcto | 2 | Usa .await apropiadamente |
| Retorno de valores | 2 | Maneja Result en async |
| Código limpio | 2 | Sin warnings |

### Práctica 02: Tokio (10 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Runtime setup | 2 | Configura #[tokio::main] |
| spawn tasks | 3 | Crea y maneja JoinHandle |
| join! macro | 2 | Ejecuta futures concurrentemente |
| select! macro | 3 | Maneja carrera de futures |

### Práctica 03: I/O Async (10 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Archivos async | 3 | Lee/escribe con tokio::fs |
| Timeouts | 2 | Usa tokio::time::timeout |
| Buffers | 2 | Usa BufReader/BufWriter async |
| Red básica | 3 | TcpStream async |

### Práctica 04: Concurrencia (12 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Múltiples requests | 3 | Procesa N requests en paralelo |
| Rate limiting | 3 | Limita concurrencia con Semaphore |
| Channels | 3 | Comunica entre tasks |
| Cancelación | 3 | Maneja abort/cancel |

---

## 🏆 Producto: Web Crawler (30 puntos)

### Funcionalidad (15 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Fetch páginas | 3 | Descarga HTML correctamente |
| Extracción links | 3 | Parsea y extrae URLs |
| Crawling recursivo | 4 | Sigue links hasta profundidad N |
| Respeta límites | 3 | Rate limit, max pages |
| Manejo errores | 2 | No crashea con URLs inválidas |

### Concurrencia (10 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Requests paralelos | 4 | Múltiples fetch simultáneos |
| Sin duplicados | 3 | No visita URL dos veces |
| Bounded concurrency | 3 | Limita conexiones activas |

### Calidad de Código (5 pts)

| Criterio | Pts | Indicador |
|----------|-----|-----------|
| Estructura | 2 | Código bien organizado |
| Documentación | 1 | Funciones documentadas |
| Tests | 2 | Al menos 3 tests |

---

## 📈 Escala de Calificación

| Rango | Nota | Descripción |
|-------|------|-------------|
| 90-100 | A | Excelente dominio de async |
| 80-89 | B | Buen manejo, detalles menores |
| 70-79 | C | Competente, áreas de mejora |
| 60-69 | D | Básico, necesita práctica |
| <60 | F | No alcanza objetivos |

---

## ✅ Checklist de Entrega

### Código
- [ ] Todos los proyectos compilan (`cargo check`)
- [ ] Sin errores de clippy (`cargo clippy`)
- [ ] Código formateado (`cargo fmt`)
- [ ] Tests pasan (`cargo test`)

### Proyecto Crawler
- [ ] README con instrucciones de uso
- [ ] Ejemplo de ejecución funcionando
- [ ] Maneja errores gracefully
- [ ] Límite de concurrencia implementado

### Documentación
- [ ] Funciones públicas documentadas
- [ ] Ejemplos en documentación
- [ ] Notas sobre decisiones de diseño

---

## 🎯 Criterios de Aprobación

**Mínimo para aprobar:** 60 puntos

**Requisitos obligatorios:**
1. ✅ Práctica 01 y 02 completas
2. ✅ Crawler hace fetch de al menos 1 página
3. ✅ No hay código que bloquee el runtime

---

## 💡 Bonificaciones

| Bonus | Pts | Descripción |
|-------|-----|-------------|
| HTTP/2 | +3 | Usa HTTP/2 en reqwest |
| Graceful shutdown | +3 | CTRL+C cancela limpiamente |
| Exportar datos | +2 | Guarda resultados en JSON |
| Métricas | +2 | Muestra estadísticas del crawl |
