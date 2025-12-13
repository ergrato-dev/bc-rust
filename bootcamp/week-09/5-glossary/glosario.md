# 📖 Glosario - Semana 09: Traits Básicos

## Términos Fundamentales

### Trait
**Definición**: Colección de métodos que definen un comportamiento compartido entre tipos.  
**Analogía**: Similar a interfaces en otros lenguajes, pero más poderoso.  
**Ejemplo**:
```rust
trait Saludar {
    fn saludar(&self) -> String;
}
```

### Trait Implementation (impl)
**Definición**: Implementación de los métodos de un trait para un tipo específico.  
**Sintaxis**: `impl Trait for Type { ... }`  
**Ejemplo**:
```rust
impl Saludar for Persona {
    fn saludar(&self) -> String {
        format!("Hola, soy {}", self.nombre)
    }
}
```

### Trait Bound
**Definición**: Restricción en un tipo genérico que requiere implementar ciertos traits.  
**Sintaxis**: `<T: Trait>` o `where T: Trait`  
**Ejemplo**:
```rust
fn imprimir<T: Display>(valor: T) {
    println!("{}", valor);
}
```

### Trait Object
**Definición**: Tipo que permite polimorfismo dinámico usando traits.  
**Sintaxis**: `&dyn Trait` o `Box<dyn Trait>`  
**Ejemplo**:
```rust
let forma: &dyn Forma = &circulo;
```

---

## Tipos de Métodos

### Self Method (Método de instancia)
**Definición**: Método que recibe `self`, `&self` o `&mut self`.  
**Uso**: Operar sobre una instancia del tipo.  
```rust
fn area(&self) -> f64;
```

### Associated Method (Método asociado)
**Definición**: Método sin parámetro `self`, asociado al tipo.  
**Uso**: Constructores, funciones de utilidad.  
```rust
fn new(radio: f64) -> Self;
```

### Default Method (Método por defecto)
**Definición**: Método con implementación en el trait, sobrescribible.  
**Uso**: Comportamiento común reutilizable.  
```rust
trait Describir {
    fn describir(&self) -> String {
        String::from("Sin descripción")
    }
}
```

---

## Traits Derivables

### #[derive()]
**Definición**: Atributo que genera implementación automática de traits.  
**Sintaxis**: `#[derive(Debug, Clone, PartialEq)]`  
**Traits comunes**: Debug, Clone, Copy, PartialEq, Eq, Hash, Default

### Debug
**Definición**: Permite formateo de depuración con `{:?}`.  
**Derivable**: ✅ Sí  
```rust
#[derive(Debug)]
struct Punto { x: i32, y: i32 }
```

### Clone
**Definición**: Permite crear una copia explícita con `.clone()`.  
**Derivable**: ✅ Sí  
**Requiere**: Todos los campos deben implementar Clone

### Copy
**Definición**: Permite copia implícita (semántica de copia, no move).  
**Derivable**: ✅ Sí  
**Requiere**: Clone + tipo "trivialmente copiable" (stack-only)

### PartialEq
**Definición**: Permite comparación de igualdad con `==` y `!=`.  
**Derivable**: ✅ Sí  
**Nota**: Permite implementaciones que no son reflexivas (NaN != NaN)

### Eq
**Definición**: Marca igualdad como reflexiva, simétrica y transitiva.  
**Derivable**: ✅ Sí  
**Requiere**: PartialEq

### PartialOrd
**Definición**: Permite comparaciones `<`, `>`, `<=`, `>=`.  
**Derivable**: ✅ Sí  
**Requiere**: PartialEq

### Ord
**Definición**: Ordenamiento total (todo valor es comparable).  
**Derivable**: ✅ Sí  
**Requiere**: Eq + PartialOrd

### Hash
**Definición**: Permite calcular hash para uso en HashMap/HashSet.  
**Derivable**: ✅ Sí  
**Nota**: Si `a == b`, entonces `hash(a) == hash(b)`

### Default
**Definición**: Proporciona valor por defecto con `Type::default()`.  
**Derivable**: ✅ Sí  
**Requiere**: Todos los campos deben implementar Default

---

## Traits Estándar (No Derivables)

### Display
**Definición**: Formateo para usuarios finales con `{}`.  
**Derivable**: ❌ No  
**Debe implementarse manualmente**

### From / Into
**Definición**: Conversiones infalibles entre tipos.  
**Nota**: Implementar From da Into automáticamente.  
```rust
impl From<i32> for MiTipo { ... }
```

### TryFrom / TryInto
**Definición**: Conversiones que pueden fallar (retornan Result).  
```rust
impl TryFrom<String> for MiTipo {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> { ... }
}
```

### Iterator
**Definición**: Permite iterar sobre una secuencia de valores.  
**Método requerido**: `fn next(&mut self) -> Option<Self::Item>`

### Drop
**Definición**: Permite ejecutar código cuando un valor sale del scope.  
**Uso**: Liberación de recursos, cleanup.

---

## Conceptos Avanzados

### Orphan Rule (Regla del huérfano)
**Definición**: No puedes implementar un trait externo para un tipo externo.  
**Razón**: Prevenir conflictos entre crates.  
**Solución**: Usar newtype pattern.

### Blanket Implementation
**Definición**: Implementación de un trait para todos los tipos que cumplen ciertos bounds.  
**Ejemplo**:
```rust
impl<T: Display> ToString for T { ... }
```

### Supertrait
**Definición**: Trait que requiere otro trait como prerequisito.  
**Sintaxis**: `trait SubTrait: SuperTrait { ... }`  
```rust
trait Animal: Debug {
    fn nombre(&self) -> &str;
}
```

### Associated Type (Tipo asociado)
**Definición**: Tipo placeholder dentro de un trait.  
**Uso**: Evitar múltiples parámetros genéricos.  
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Coherence
**Definición**: Propiedad que garantiza una única implementación de trait por tipo.  
**Aplica**: El compilador asegura que no haya implementaciones conflictivas.

### Object Safety
**Definición**: Traits que pueden usarse como trait objects (`dyn Trait`).  
**Requisitos**:
- No retornar Self
- No tener parámetros genéricos
- Todos los métodos deben ser object-safe

### Marker Trait
**Definición**: Trait sin métodos, usado para marcar propiedades.  
**Ejemplos**: Copy, Send, Sync, Sized

### Static Dispatch
**Definición**: El compilador genera código específico para cada tipo (monomorphization).  
**Sintaxis**: `fn foo<T: Trait>(x: T)` o `fn foo(x: impl Trait)`  
**Ventaja**: Sin overhead en runtime

### Dynamic Dispatch
**Definición**: La llamada al método se resuelve en runtime via vtable.  
**Sintaxis**: `&dyn Trait` o `Box<dyn Trait>`  
**Ventaja**: Flexibilidad, heterogeneous collections

---

## Sintaxis Especial

### impl Trait (Return position)
**Uso**: Ocultar el tipo concreto de retorno.  
```rust
fn crear_iter() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}
```

### impl Trait (Argument position)
**Uso**: Simplificar bounds en argumentos.  
```rust
fn imprimir(valor: impl Display) {
    println!("{}", valor);
}
```

### where Clause
**Uso**: Bounds más legibles para casos complejos.  
```rust
fn procesar<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Debug + Default,
{ ... }
```

### Turbofish ::<>
**Uso**: Especificar tipos genéricos explícitamente.  
```rust
let num = "42".parse::<i32>().unwrap();
```

---

## Errores Comunes

### E0119 - Conflicting implementations
**Causa**: Dos implementaciones del mismo trait para el mismo tipo.

### E0117 - Orphan rule violation
**Causa**: Implementar trait externo para tipo externo.

### E0277 - Trait not implemented
**Causa**: Usar un tipo donde se requiere un trait que no implementa.

### E0038 - Not object safe
**Causa**: Intentar usar un trait no object-safe como `dyn Trait`.
