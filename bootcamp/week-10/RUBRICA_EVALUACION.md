# 📊 Rúbrica de Evaluación - Semana 10: Generics

## 🎯 Competencias a Evaluar

### 1. Comprensión de Generics (25%)

| Nivel | Descripción | Puntos |
|-------|-------------|--------|
| **Excelente** | Explica qué son los generics, por qué existen y cómo funciona la monomorphization | 25 |
| **Bueno** | Entiende generics y los usa correctamente | 20 |
| **Básico** | Usa generics pero con dificultad para explicar su funcionamiento | 15 |
| **Insuficiente** | No comprende el concepto de tipos genéricos | 0-10 |

### 2. Funciones Genéricas (20%)

| Nivel | Descripción | Puntos |
|-------|-------------|--------|
| **Excelente** | Define funciones genéricas con múltiples parámetros de tipo y bounds apropiados | 20 |
| **Bueno** | Crea funciones genéricas simples correctamente | 16 |
| **Básico** | Puede crear funciones genéricas con ayuda | 12 |
| **Insuficiente** | No logra definir funciones genéricas | 0-8 |

### 3. Structs y Enums Genéricos (20%)

| Nivel | Descripción | Puntos |
|-------|-------------|--------|
| **Excelente** | Diseña structs/enums genéricos con métodos e implementaciones correctas | 20 |
| **Bueno** | Crea structs genéricos básicos con métodos | 16 |
| **Básico** | Puede definir structs genéricos simples | 12 |
| **Insuficiente** | No logra crear tipos genéricos | 0-8 |

### 4. Trait Bounds (20%)

| Nivel | Descripción | Puntos |
|-------|-------------|--------|
| **Excelente** | Usa bounds complejos con `where`, múltiples traits, y entiende cuándo aplicarlos | 20 |
| **Bueno** | Aplica trait bounds correctamente en la mayoría de casos | 16 |
| **Básico** | Usa trait bounds simples | 12 |
| **Insuficiente** | No comprende los trait bounds | 0-8 |

### 5. Aplicación Práctica (15%)

| Nivel | Descripción | Puntos |
|-------|-------------|--------|
| **Excelente** | Código genérico idiomático, bien organizado y reutilizable | 15 |
| **Bueno** | Código funcional con buen uso de generics | 12 |
| **Básico** | Código funciona pero con redundancias | 9 |
| **Insuficiente** | Código no compila o tiene errores graves | 0-6 |

## 📝 Ejercicios de Evaluación

### Ejercicio 1: Funciones Genéricas (20 puntos)

Implementar funciones genéricas que:

```rust
// a) Encuentre el mayor de dos valores
fn mayor<T: PartialOrd>(a: T, b: T) -> T

// b) Intercambie dos valores
fn intercambiar<T>(a: &mut T, b: &mut T)

// c) Encuentre el elemento en una posición
fn obtener<T>(slice: &[T], index: usize) -> Option<&T>
```

| Criterio | Puntos |
|----------|--------|
| Sintaxis correcta | 5 |
| Trait bounds apropiados | 5 |
| Manejo de referencias | 5 |
| Tests pasan | 5 |

### Ejercicio 2: Struct Genérico (25 puntos)

Crear un struct `Par<T, U>` con:

```rust
struct Par<T, U> {
    primero: T,
    segundo: U,
}

impl<T, U> Par<T, U> {
    fn new(primero: T, segundo: U) -> Self
    fn primero(&self) -> &T
    fn segundo(&self) -> &U
    fn intercambiar(self) -> Par<U, T>
}

impl<T: Display, U: Display> Par<T, U> {
    fn mostrar(&self)
}
```

| Criterio | Puntos |
|----------|--------|
| Definición correcta del struct | 5 |
| Métodos básicos | 5 |
| Método `intercambiar` | 5 |
| Trait bounds en impl separado | 5 |
| Tests completos | 5 |

### Ejercicio 3: Contenedor Genérico (30 puntos)

Implementar un contenedor genérico:

```rust
struct Contenedor<T> {
    items: Vec<T>,
}

impl<T> Contenedor<T> {
    fn new() -> Self
    fn agregar(&mut self, item: T)
    fn len(&self) -> usize
    fn esta_vacio(&self) -> bool
}

impl<T: Clone> Contenedor<T> {
    fn obtener(&self, index: usize) -> Option<T>
    fn primero(&self) -> Option<T>
    fn ultimo(&self) -> Option<T>
}

impl<T: PartialOrd> Contenedor<T> {
    fn mayor(&self) -> Option<&T>
    fn menor(&self) -> Option<&T>
}

impl<T: Default> Default for Contenedor<T> { ... }
```

| Criterio | Puntos |
|----------|--------|
| Estructura básica | 5 |
| Métodos sin bounds | 5 |
| Métodos con Clone | 5 |
| Métodos con PartialOrd | 5 |
| Implementación de Default | 5 |
| Tests exhaustivos | 5 |

### Ejercicio 4: Cláusula Where (15 puntos)

Refactorizar usando `where`:

```rust
// Convertir bounds complejos a cláusula where
fn procesar<T, U, V>(t: T, u: U, v: V) -> String
where
    T: Display + Clone,
    U: Debug + Default,
    V: Into<String>,
{
    // ...
}
```

| Criterio | Puntos |
|----------|--------|
| Uso correcto de where | 5 |
| Legibilidad mejorada | 5 |
| Funcionalidad correcta | 5 |

## 🎓 Proyecto Final: Sistema de Almacenamiento Genérico (10 puntos extra)

Implementar un sistema con:

- `Almacen<K, V>` genérico (similar a HashMap)
- Métodos: `insertar`, `obtener`, `eliminar`, `contiene`
- Iteración sobre elementos
- Trait bounds apropiados

| Criterio | Puntos |
|----------|--------|
| Diseño del sistema | 3 |
| Implementación completa | 4 |
| Tests | 3 |

## 📊 Escala de Calificación

| Rango | Calificación | Descripción |
|-------|--------------|-------------|
| 90-100 | A | Excelente dominio de generics |
| 80-89 | B | Buen manejo de generics |
| 70-79 | C | Comprensión básica |
| 60-69 | D | Necesita práctica |
| 0-59 | F | No alcanza objetivos mínimos |

## ✅ Checklist de Entrega

- [ ] Todos los ejercicios compilan sin errores
- [ ] `cargo clippy` sin warnings
- [ ] `cargo fmt` aplicado
- [ ] Tests pasan (`cargo test`)
- [ ] Código documentado
- [ ] Uso apropiado de trait bounds
- [ ] Sin código duplicado (aprovecha generics)

## 🔍 Criterios de Código

### Uso Correcto de Generics

```rust
// ✅ Bien: genérico cuando hay reutilización
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// ❌ Mal: genérico innecesario
fn sumar_uno<T>(x: i32) -> i32 {
    x + 1
}
```

### Trait Bounds Apropiados

```rust
// ✅ Bien: solo los bounds necesarios
fn clonar<T: Clone>(valor: &T) -> T {
    valor.clone()
}

// ❌ Mal: bounds excesivos
fn clonar<T: Clone + Debug + Display + Default>(valor: &T) -> T {
    valor.clone()
}
```

### Organización de impl

```rust
// ✅ Bien: separar impl por bounds
impl<T> Contenedor<T> {
    fn new() -> Self { ... }
}

impl<T: Clone> Contenedor<T> {
    fn clonar_items(&self) -> Vec<T> { ... }
}

// ❌ Mal: bounds innecesarios en impl general
impl<T: Clone + Debug + Display> Contenedor<T> {
    fn new() -> Self { ... }  // No necesita esos bounds
}
```
