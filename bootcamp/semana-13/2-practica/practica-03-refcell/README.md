# 🔓 Práctica 03: RefCell<T>

## 🎯 Objetivos

- Usar `RefCell<T>` para mutabilidad interior
- Implementar mock objects para testing
- Crear un sistema de caché
- Implementar el patrón Observer

---

## 📋 Ejercicios

### Ejercicio 1: Contador con RefCell

Un contador que se puede incrementar con `&self`:

```rust
struct Contador {
    valor: RefCell<i32>,
}

impl Contador {
    fn incrementar(&self) {  // &self, no &mut self
        // ...
    }
}
```

### Ejercicio 2: Mock Messenger

Un mock object para testing que registra mensajes:

```rust
trait Mensajero {
    fn enviar(&self, mensaje: &str);  // &self
}

struct MockMensajero {
    mensajes: RefCell<Vec<String>>,
}
```

**Implementa:**
- `new()` - Crear mock vacío
- `enviar()` - Guardar mensaje (implementa Mensajero)
- `mensajes()` - Obtener copia de los mensajes

### Ejercicio 3: Calculadora con Caché

Una calculadora que cachea resultados:

```rust
struct Calculadora {
    cache: RefCell<HashMap<u64, u64>>,
    hits: RefCell<u32>,
}
```

**Implementa:**
- `factorial(n)` - Calcular con caché
- `cache_hits()` - Retornar número de cache hits

### Ejercicio 4: Observable

Implementa el patrón observer:

```rust
struct Observable {
    valor: RefCell<i32>,
    observers: RefCell<Vec<Callback>>,
}
```

**Implementa:**
- `suscribir(callback)` - Agregar observer
- `set_valor(nuevo)` - Cambiar y notificar

---

## 💡 Pistas

### borrow() vs borrow_mut()

```rust
// Lectura inmutable
let valor = *celda.borrow();

// Escritura mutable
*celda.borrow_mut() = nuevo_valor;
```

### Evitar panics

```rust
// ❌ PANIC: dos borrows mutables
let a = celda.borrow_mut();
let b = celda.borrow_mut();

// ✅ Limitar scope
{
    let mut a = celda.borrow_mut();
    *a = 10;
}  // a se libera
let b = celda.borrow();  // OK
```

### Caché pattern

```rust
// Verificar caché primero
if let Some(&valor) = self.cache.borrow().get(&key) {
    return valor;
}

// Calcular y guardar
let resultado = calcular();
self.cache.borrow_mut().insert(key, resultado);
resultado
```

---

## 🧪 Tests

```bash
cargo test
```

---

## ✅ Criterios de Éxito

- [ ] Contador funciona con &self
- [ ] Mock registra mensajes correctamente
- [ ] Caché evita recálculos
- [ ] Observable notifica a todos los observers
- [ ] Ningún test hace panic por RefCell
