# Rúbrica de Evaluación - Semana 13: Smart Pointers

## 📊 Distribución de Puntos

| Categoría | Peso | Descripción |
|-----------|------|-------------|
| **Conocimiento** | 30% | Comprensión teórica de smart pointers |
| **Desempeño** | 40% | Ejercicios prácticos y debugging |
| **Producto** | 30% | Proyecto funcional de la semana |

---

## 🎯 Conocimiento (30 puntos)

### Conceptos de Box<T> (6 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 6 | Explica heap allocation, tipos recursivos y Deref |
| Bueno | 4 | Comprende uso básico de Box y cuándo usarlo |
| Básico | 2 | Sabe crear Box pero no entiende beneficios |
| Insuficiente | 0 | No comprende Box |

### Conceptos de Rc/Arc (6 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 6 | Diferencia Rc/Arc, entiende reference counting y Send/Sync |
| Bueno | 4 | Usa Rc correctamente, conoce Arc para threads |
| Básico | 2 | Usa Rc pero no entiende la diferencia con Arc |
| Insuficiente | 0 | No comprende reference counting |

### Conceptos de RefCell (6 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 6 | Explica borrow checking en runtime, BorrowError, alternativas |
| Bueno | 4 | Usa RefCell correctamente con borrow/borrow_mut |
| Básico | 2 | Sabe que existe pero no entiende cuándo usarlo |
| Insuficiente | 0 | No comprende mutabilidad interior |

### Interior Mutability (6 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 6 | Conoce Cell, RefCell, Mutex, RwLock y sus diferencias |
| Bueno | 4 | Usa RefCell y Mutex correctamente |
| Básico | 2 | Conoce el concepto pero no las variantes |
| Insuficiente | 0 | No comprende interior mutability |

### Prevención de Ciclos con Weak (6 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 6 | Identifica ciclos, usa Weak, implementa upgrade/downgrade |
| Bueno | 4 | Sabe que Weak rompe ciclos y lo usa correctamente |
| Básico | 2 | Conoce Weak pero no sabe cuándo aplicarlo |
| Insuficiente | 0 | No conoce el problema de ciclos |

---

## 💻 Desempeño (40 puntos)

### Práctica 01: Box y Tipos Recursivos (10 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 10 | Implementa lista enlazada y árbol con Box |
| Bueno | 8 | Implementa estructuras recursivas básicas |
| Básico | 5 | Usa Box pero con errores menores |
| Insuficiente | 0 | No compila o no usa Box |

### Práctica 02: Rc y Arc (10 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 10 | Comparte datos correctamente, usa strong/weak count |
| Bueno | 8 | Implementa sharing básico con Rc |
| Básico | 5 | Usa Rc pero con memory leaks o errores |
| Insuficiente | 0 | No compila o no comparte datos |

### Práctica 03: RefCell (10 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 10 | Muta datos compartidos sin panic, maneja errores |
| Bueno | 8 | Usa borrow_mut correctamente en casos simples |
| Básico | 5 | Funciona pero puede causar panic en runtime |
| Insuficiente | 0 | No compila o causa panic siempre |

### Práctica 04: Weak y Ciclos (10 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 10 | Implementa árbol con padre Weak, sin memory leaks |
| Bueno | 8 | Usa Weak para romper ciclos simples |
| Básico | 5 | Intenta usar Weak pero con errores |
| Insuficiente | 0 | Crea ciclos de referencia |

---

## 📦 Producto (30 puntos)

### Proyecto: Sistema de Caché

#### Funcionalidad (15 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 15 | Caché funcional con get/set, expiration, LRU |
| Bueno | 12 | Caché básico con get/set funcionando |
| Básico | 8 | Almacena datos pero sin política de eviction |
| Insuficiente | 0 | No funciona o no compila |

#### Uso de Smart Pointers (10 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 10 | Combina Rc<RefCell<>>, usa Weak para referencias |
| Bueno | 8 | Usa smart pointers apropiados |
| Básico | 5 | Usa Box pero podría usar Rc/RefCell |
| Insuficiente | 0 | No usa smart pointers |

#### Calidad de Código (5 puntos)

| Nivel | Puntos | Criterio |
|-------|--------|----------|
| Excelente | 5 | Sin warnings, documentado, tests completos |
| Bueno | 4 | Compila limpio, tests básicos |
| Básico | 2 | Compila con warnings menores |
| Insuficiente | 0 | No compila o warnings críticos |

---

## ✅ Checklist de Evaluación

### Box<T>
- [ ] Sabe crear valores en el heap con Box::new()
- [ ] Implementa tipos recursivos (listas, árboles)
- [ ] Entiende el trait Deref y dereference coercion
- [ ] Conoce cuándo preferir Box sobre stack allocation

### Rc<T> y Arc<T>
- [ ] Usa Rc::clone() para incrementar el contador
- [ ] Accede a strong_count() y weak_count()
- [ ] Sabe que Rc no es Send/Sync
- [ ] Usa Arc para datos compartidos entre threads

### RefCell<T>
- [ ] Usa borrow() para referencia inmutable
- [ ] Usa borrow_mut() para referencia mutable
- [ ] Maneja BorrowError y BorrowMutError
- [ ] Combina Rc<RefCell<T>> para mutabilidad compartida

### Interior Mutability
- [ ] Conoce Cell<T> para tipos Copy
- [ ] Conoce RefCell<T> para tipos no-Copy
- [ ] Conoce Mutex<T> para multi-threading
- [ ] Entiende las reglas de borrowing en runtime

### Weak<T>
- [ ] Crea Weak desde Rc con Rc::downgrade()
- [ ] Usa upgrade() para obtener Option<Rc<T>>
- [ ] Identifica y evita ciclos de referencia
- [ ] Implementa estructuras con referencias bidireccionales

---

## 🏆 Escala de Calificación

| Puntuación | Calificación | Descripción |
|------------|--------------|-------------|
| 90-100 | A | Excelente dominio de smart pointers |
| 80-89 | B | Buen manejo, pequeñas mejoras necesarias |
| 70-79 | C | Comprensión básica, necesita práctica |
| 60-69 | D | Deficiencias significativas |
| 0-59 | F | No cumple los objetivos mínimos |

---

## 📝 Notas para el Evaluador

### Puntos Críticos

1. **Memory Leaks**: Verificar que no haya ciclos de referencia
2. **Panic en Runtime**: RefCell puede causar panic si se viola borrowing
3. **Thread Safety**: Rc no debe usarse en contextos multi-thread
4. **Weak Upgrade**: Siempre verificar Option al hacer upgrade()

### Comandos de Verificación

```bash
# Verificar que compila
cargo check -p proyecto-cache

# Ejecutar tests
cargo test -p proyecto-cache

# Verificar memory leaks (si disponible)
cargo +nightly miri test -p proyecto-cache

# Clippy para best practices
cargo clippy -p proyecto-cache
```

### Errores Comunes a Buscar

```rust
// ❌ Ciclo de referencia
let a = Rc::new(RefCell::new(Node { next: None }));
a.borrow_mut().next = Some(Rc::clone(&a)); // Self-reference!

// ❌ Double borrow
let cell = RefCell::new(5);
let b1 = cell.borrow_mut();
let b2 = cell.borrow(); // PANIC!

// ❌ Rc en threads
let rc = Rc::new(5);
std::thread::spawn(move || println!("{}", rc)); // Error: Rc is not Send
```

---

**Última actualización**: Diciembre 2025
