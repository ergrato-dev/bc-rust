# 🧩 Práctica 04: Combinaciones de Smart Pointers

## 🎯 Objetivos

- Combinar `Rc<RefCell<T>>` para estructuras mutables compartidas
- Usar `Weak<T>` para relaciones bidireccionales sin ciclos
- Implementar estructuras de datos complejas

---

## 📋 Ejercicios

### Ejercicio 1: Lista Doblemente Enlazada

Una lista donde cada nodo conoce al anterior y al siguiente:

```rust
struct NodoListaInner<T> {
    valor: T,
    siguiente: NodoLista<T>,        // Rc - mantiene vivo
    anterior: NodoWeak<T>,          // Weak - no ciclos
}
```

**Implementa:**
- `push_back(valor)` - Agregar al final
- `push_front(valor)` - Agregar al inicio
- `pop_front()` - Remover del inicio

### Ejercicio 2: Sistema de Usuarios y Grupos

Relación muchos-a-muchos entre usuarios y grupos:

```rust
struct Usuario {
    nombre: String,
    grupos: Vec<Weak<RefCell<Grupo>>>,    // Weak a grupos
}

struct Grupo {
    nombre: String,
    miembros: Vec<Weak<RefCell<Usuario>>>, // Weak a usuarios
}
```

**Implementa:**
- `agregar_a_grupo(usuario, grupo)`
- `grupos_de(usuario)` - Listar grupos
- `miembros_de(grupo)` - Listar miembros

### Ejercicio 3: Árbol DOM Simplificado

Estructura de árbol con navegación padre-hijo:

```rust
struct DomNodeInner {
    tag: String,
    parent: Weak<RefCell<DomNodeInner>>,  // Weak al padre
    children: Vec<DomNodeRef>,             // Rc a hijos
}
```

**Implementa:**
- `append_child(padre, hijo)`
- `path(nodo)` - Retorna ruta (ej: "html > body > p")

---

## 💡 Pistas

### Lista Doblemente Enlazada

```rust
// Crear nodo con anterior como Weak
let nuevo = Rc::new(RefCell::new(Nodo {
    valor,
    siguiente: None,
    anterior: self.cola.clone(),  // Weak a la cola actual
}));

// Actualizar la cola vieja para apuntar al nuevo
if let Some(cola) = self.cola.as_ref().and_then(|w| w.upgrade()) {
    cola.borrow_mut().siguiente = Some(Rc::clone(&nuevo));
}

// Actualizar cola
self.cola = Some(Rc::downgrade(&nuevo));
```

### Relaciones Bidireccionales

```rust
// Usuario → Grupo: Weak (usuario no "posee" al grupo)
usuario.borrow_mut().grupos.push(Rc::downgrade(&grupo));

// Grupo → Usuario: Weak (grupo no "posee" al usuario)
grupo.borrow_mut().miembros.push(Rc::downgrade(&usuario));

// Ninguno previene la liberación del otro
```

### Navegación hacia arriba

```rust
fn path(nodo: &NodeRef) -> String {
    let mut partes = vec![nodo.borrow().tag.clone()];
    let mut actual = nodo.borrow().parent.upgrade();
    
    while let Some(padre) = actual {
        partes.push(padre.borrow().tag.clone());
        actual = padre.borrow().parent.upgrade();
    }
    
    partes.reverse();
    partes.join(" > ")
}
```

---

## 🧪 Tests

```bash
cargo test
```

---

## ✅ Criterios de Éxito

- [ ] Lista doblemente enlazada funciona correctamente
- [ ] Sistema de usuarios/grupos sin memory leaks
- [ ] Árbol DOM con navegación bidireccional
- [ ] Todos los Weak se usan correctamente
- [ ] Todos los tests pasan
