# 📦 Semana 13: Smart Pointers

## 🎯 Objetivos de la Semana

Al finalizar esta semana, serás capaz de:

- Comprender qué son los smart pointers y por qué existen
- Usar `Box<T>` para datos en el heap
- Implementar conteo de referencias con `Rc<T>` y `Arc<T>`
- Aplicar mutabilidad interior con `RefCell<T>`
- Combinar smart pointers para estructuras complejas

---

## 📚 Contenido

### 1. Teoría

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-introduccion-smart-pointers.md](1-teoria/01-introduccion-smart-pointers.md) | ¿Qué son los Smart Pointers? | 20 min |
| [02-box.md](1-teoria/02-box.md) | Box<T> - Datos en el Heap | 25 min |
| [03-rc-arc.md](1-teoria/03-rc-arc.md) | Rc<T> y Arc<T> - Conteo de Referencias | 30 min |
| [04-refcell.md](1-teoria/04-refcell.md) | RefCell<T> - Mutabilidad Interior | 25 min |
| [05-patrones-avanzados.md](1-teoria/05-patrones-avanzados.md) | Patrones y Combinaciones | 20 min |

### 2. Práctica

| Ejercicio | Descripción | Dificultad |
|-----------|-------------|------------|
| [practica-01-box](2-practica/practica-01-box/) | Estructuras recursivas con Box | ⭐⭐ |
| [practica-02-rc-arc](2-practica/practica-02-rc-arc/) | Referencias compartidas | ⭐⭐⭐ |
| [practica-03-refcell](2-practica/practica-03-refcell/) | Mutabilidad interior | ⭐⭐⭐ |
| [practica-04-combinaciones](2-practica/practica-04-combinaciones/) | Rc<RefCell<T>> y otros patrones | ⭐⭐⭐⭐ |

### 3. Proyecto Semanal

| Proyecto | Descripción |
|----------|-------------|
| [proyecto-arbol](3-proyecto/proyecto-arbol/) | Árbol con nodos compartidos y mutables |

---

## 🗓️ Distribución del Tiempo (4 horas)

| Bloque | Actividad | Tiempo |
|--------|-----------|--------|
| 1 | Teoría: Smart Pointers y Box | 45 min |
| 2 | Práctica 01: Box | 30 min |
| 3 | Teoría: Rc/Arc y RefCell | 55 min |
| 4 | Práctica 02-03: Rc y RefCell | 50 min |
| 5 | Teoría: Patrones combinados | 20 min |
| 6 | Práctica 04: Combinaciones | 30 min |
| 7 | Proyecto: Árbol | 30 min |

---

## 🧠 Conceptos Clave

### Smart Pointers en Rust

```
┌─────────────────────────────────────────────────────────────┐
│                     SMART POINTERS                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Box<T>         → Heap allocation, único dueño              │
│  Rc<T>          → Reference counting, single-thread         │
│  Arc<T>         → Atomic reference counting, multi-thread   │
│  RefCell<T>     → Interior mutability, runtime checks       │
│  Mutex<T>       → Interior mutability + thread safety       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Cuándo Usar Cada Uno

| Smart Pointer | Caso de Uso |
|---------------|-------------|
| `Box<T>` | Tipos recursivos, datos grandes en heap |
| `Rc<T>` | Múltiples dueños en un solo thread |
| `Arc<T>` | Múltiples dueños entre threads |
| `RefCell<T>` | Mutar datos con referencia inmutable |
| `Rc<RefCell<T>>` | Múltiples dueños + mutabilidad |

---

## ⚠️ Errores Comunes

### 1. Ciclos de Referencias con Rc

```rust
// ❌ PROBLEMA: Ciclo que nunca se libera
struct Node {
    next: Option<Rc<Node>>,
}

// ✅ SOLUCIÓN: Usar Weak para romper ciclos
struct Node {
    next: Option<Rc<Node>>,
    parent: Option<Weak<Node>>,  // Weak no aumenta el conteo
}
```

### 2. Panic con RefCell

```rust
// ❌ PROBLEMA: Dos borrows mutables
let cell = RefCell::new(5);
let a = cell.borrow_mut();
let b = cell.borrow_mut();  // PANIC en runtime!

// ✅ SOLUCIÓN: Limitar scope de borrows
let cell = RefCell::new(5);
{
    let mut a = cell.borrow_mut();
    *a += 1;
}  // a se libera aquí
let b = cell.borrow();  // OK ahora
```

### 3. Rc vs Arc

```rust
// ❌ PROBLEMA: Rc no es Send
let rc = Rc::new(5);
std::thread::spawn(move || {
    println!("{}", rc);  // ERROR: Rc no es thread-safe
});

// ✅ SOLUCIÓN: Usar Arc para threads
let arc = Arc::new(5);
std::thread::spawn(move || {
    println!("{}", arc);  // OK
});
```

---

## 📖 Recursos

- [4-recursos/RECURSOS.md](4-recursos/RECURSOS.md) - Enlaces y material adicional
- [5-glosario/GLOSARIO.md](5-glosario/GLOSARIO.md) - Términos clave

---

## ✅ Criterios de Evaluación

Ver [RUBRICA_EVALUACION.md](RUBRICA_EVALUACION.md) para los criterios detallados.

| Criterio | Peso |
|----------|------|
| Conocimiento teórico | 30% |
| Ejercicios prácticos | 40% |
| Proyecto semanal | 30% |

---

## 🔗 Navegación

| ← Anterior | Actual | Siguiente → |
|------------|--------|-------------|
| [Semana 12: Closures e Iteradores](../semana-12/) | **Semana 13** | [Semana 14: Concurrencia](../semana-14/) |
