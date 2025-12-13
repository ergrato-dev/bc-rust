# 🔄 Práctica 02: Rc<T> y Arc<T>

## 🎯 Objetivos

- Usar `Rc<T>` para compartir ownership entre múltiples partes
- Usar `Weak<T>` para evitar ciclos de referencias
- Usar `Arc<T>` para compartir datos entre threads

---

## 📋 Ejercicios

### Ejercicio 1: Rc Básico

Familiarízate con el conteo de referencias:

```rust
fn crear_datos_compartidos() -> Rc<Vec<i32>> {
    Rc::new(vec![1, 2, 3, 4, 5])
}
```

**Observa:**
- `Rc::strong_count()` muestra cuántas referencias existen
- `Rc::clone()` incrementa el contador (no clona los datos)
- `drop()` decrementa el contador

### Ejercicio 2: Grafo con Nodos Compartidos

Implementa un grafo donde los nodos pueden tener múltiples vecinos:

```rust
struct Grafo {
    nodos: HashMap<String, NodoRef>,
}
```

**Métodos:**
- `agregar_nodo(id)` - Crear y guardar un nodo
- `conectar(desde, hacia)` - Conectar dos nodos
- `vecinos_de(id)` - Listar vecinos de un nodo

### Ejercicio 3: Árbol con Parent (Weak)

Implementa un árbol donde los hijos conocen a su padre:

```rust
struct NodoArbolInner {
    valor: i32,
    padre: Weak<RefCell<NodoArbolInner>>,  // Weak para evitar ciclos
    hijos: Vec<NodoArbolRef>,               // Rc para mantener vivos
}
```

**Métodos:**
- `agregar_hijo(padre, hijo)` - Conectar padre e hijo
- `profundidad(nodo)` - Calcular distancia a la raíz

### Ejercicio 4: Arc con Threads

Procesa datos en paralelo:

```rust
fn procesar_en_paralelo(datos: Vec<i32>) -> Vec<i32> {
    // 3 threads calculan: suma, producto, máximo
}
```

---

## 💡 Pistas

### Rc::clone vs .clone()

```rust
// ✅ Incrementa contador (barato)
let r2 = Rc::clone(&r1);

// ⚠️ También incrementa, pero menos claro
let r3 = r1.clone();

// ❌ Clona el contenido (caro)
let copia = (*r1).clone();
```

### Weak para romper ciclos

```rust
// Rc::downgrade crea una referencia Weak
hijo.padre = Rc::downgrade(&padre);

// upgrade() intenta obtener Rc (retorna Option)
if let Some(padre_rc) = weak_ref.upgrade() {
    // padre todavía existe
}
```

### Arc con threads

```rust
let arc = Arc::new(datos);

let arc_clone = Arc::clone(&arc);
thread::spawn(move || {
    // usar arc_clone aquí
});
```

---

## 🧪 Tests

```bash
cargo test
```

---

## ✅ Criterios de Éxito

- [ ] Rc comparte datos correctamente
- [ ] Grafo permite nodos compartidos
- [ ] Árbol usa Weak para el padre (sin memory leaks)
- [ ] Arc funciona entre threads
- [ ] Todos los tests pasan
