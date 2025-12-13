# 📋 Rúbrica de Evaluación - Semana 13: Smart Pointers

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Comprensión de smart pointers y su propósito |
| **C2** | Uso correcto de Box<T> |
| **C3** | Implementación de Rc<T> y Arc<T> |
| **C4** | Aplicación de RefCell<T> |
| **C5** | Combinación de smart pointers |

---

## 📊 Distribución de Notas

| Componente | Peso | Descripción |
|------------|------|-------------|
| **Conocimiento** | 30% | Comprensión teórica |
| **Desempeño** | 40% | Ejercicios y prácticas |
| **Producto** | 30% | Proyecto semanal |

---

## 📝 Rúbrica Detallada

### 1. Conocimiento (30%)

#### 1.1 Conceptos de Smart Pointers (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Explica qué son los smart pointers, traits Deref/Drop, y cuándo usar cada tipo |
| **Bueno** | 8 | Comprende la diferencia entre stack y heap, conoce los tipos principales |
| **Suficiente** | 6 | Conoce Box y Rc básicamente |
| **Insuficiente** | 0-5 | No distingue entre tipos de smart pointers |

#### 1.2 Ownership y Referencias Compartidas (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Comprende conteo de referencias, ciclos, y Weak |
| **Bueno** | 8 | Entiende Rc vs Arc y cuándo usar cada uno |
| **Suficiente** | 6 | Puede usar Rc para compartir datos |
| **Insuficiente** | 0-5 | No comprende referencias compartidas |

#### 1.3 Mutabilidad Interior (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Comprende borrowing rules en runtime, patterns con RefCell |
| **Bueno** | 8 | Usa RefCell correctamente, entiende borrow/borrow_mut |
| **Suficiente** | 6 | Puede usar RefCell básico |
| **Insuficiente** | 0-5 | No entiende mutabilidad interior |

---

### 2. Desempeño (40%)

#### 2.1 Práctica 01: Box (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Lista enlazada funcional, implementa métodos correctamente |
| **Bueno** | 8 | Estructura recursiva compila, operaciones básicas |
| **Suficiente** | 6 | Usa Box para tipo recursivo |
| **Insuficiente** | 0-5 | No compila o no usa Box |

#### 2.2 Práctica 02: Rc/Arc (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Grafo con nodos compartidos, maneja referencias correctamente |
| **Bueno** | 8 | Comparte datos entre múltiples owners |
| **Suficiente** | 6 | Usa Rc::clone() correctamente |
| **Insuficiente** | 0-5 | No implementa compartición |

#### 2.3 Práctica 03: RefCell (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Implementa patrón observer o similar con mutabilidad interior |
| **Bueno** | 8 | Muta a través de referencia inmutable correctamente |
| **Suficiente** | 6 | Usa borrow_mut() sin panics |
| **Insuficiente** | 0-5 | Código hace panic o no usa RefCell |

#### 2.4 Práctica 04: Combinaciones (10%)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 10 | Combina Rc<RefCell<T>>, usa Weak para evitar ciclos |
| **Bueno** | 8 | Usa Rc<RefCell<T>> para datos compartidos mutables |
| **Suficiente** | 6 | Intenta combinación básica |
| **Insuficiente** | 0-5 | No combina smart pointers |

---

### 3. Producto (30%)

#### 3.1 Proyecto: Árbol con Nodos Compartidos

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| **Excelente** | 30 | Árbol completo con parent/children, usa Weak, métodos de navegación |
| **Bueno** | 24 | Árbol funcional con hijos compartidos, operaciones básicas |
| **Suficiente** | 18 | Estructura de árbol básica con smart pointers |
| **Insuficiente** | 0-17 | No implementa árbol o tiene memory leaks |

##### Criterios Específicos del Proyecto

| Aspecto | Puntos | Requisito |
|---------|--------|-----------|
| Estructura del nodo | 6 | Usa Rc<RefCell<Node>> o similar |
| Referencias padre/hijo | 6 | Parent con Weak, children con Rc |
| Inserción de nodos | 6 | Método para agregar hijos |
| Navegación | 6 | Métodos para recorrer el árbol |
| Sin memory leaks | 6 | No hay ciclos de Rc |

---

## ✅ Lista de Verificación

### Código

- [ ] Compila sin warnings (`cargo clippy`)
- [ ] Pasa todos los tests (`cargo test`)
- [ ] Código formateado (`cargo fmt`)
- [ ] No usa `unwrap()` innecesarios
- [ ] Maneja correctamente los borrows de RefCell

### Smart Pointers

- [ ] Usa Box para tipos recursivos
- [ ] Usa Rc para múltiples dueños
- [ ] Usa RefCell para mutabilidad interior
- [ ] Usa Weak para evitar ciclos
- [ ] No tiene memory leaks

### Documentación

- [ ] Funciones públicas documentadas
- [ ] Comentarios en código complejo
- [ ] README del proyecto completo

---

## 🏆 Niveles de Logro

| Nivel | Rango | Descripción |
|-------|-------|-------------|
| **Sobresaliente** | 90-100 | Domina smart pointers y patrones avanzados |
| **Notable** | 80-89 | Usa smart pointers correctamente en la mayoría de casos |
| **Aprobado** | 60-79 | Comprende conceptos básicos de smart pointers |
| **Insuficiente** | 0-59 | Necesita reforzar conceptos fundamentales |

---

## 📌 Notas Adicionales

### Errores Críticos (Penalización)

| Error | Penalización |
|-------|--------------|
| Memory leak por ciclos de Rc | -10 puntos |
| Panic por RefCell mal usado | -5 puntos |
| Usar Rc en contexto multi-thread | -5 puntos |
| No usar Box donde es necesario | -5 puntos |

### Bonus

| Logro | Bonus |
|-------|-------|
| Implementa Drop personalizado | +5 puntos |
| Usa Cow<T> apropiadamente | +3 puntos |
| Implementa árbol balanceado | +5 puntos |
