# 📚 Generics Avanzados

## Tipos Asociados vs Parámetros Genéricos

### Parámetros Genéricos en Traits

```rust
// Trait con parámetro genérico
trait Convertidor<T> {
    fn convertir(&self) -> T;
}

// Problema: múltiples implementaciones posibles
impl Convertidor<String> for i32 {
    fn convertir(&self) -> String {
        self.to_string()
    }
}

impl Convertidor<f64> for i32 {
    fn convertir(&self) -> f64 {
        *self as f64
    }
}

// Al usar, debemos especificar el tipo
fn usar<T, C: Convertidor<T>>(c: &C) -> T {
    c.convertir()
}
```

### Tipos Asociados

```rust
// Trait con tipo asociado
trait Iterador {
    type Item;  // Tipo asociado
    
    fn siguiente(&mut self) -> Option<Self::Item>;
}

// Solo UNA implementación por tipo
struct Contador {
    actual: u32,
    max: u32,
}

impl Iterador for Contador {
    type Item = u32;  // El tipo está fijado
    
    fn siguiente(&mut self) -> Option<Self::Item> {
        if self.actual < self.max {
            self.actual += 1;
            Some(self.actual)
        } else {
            None
        }
    }
}
```

### Cuándo Usar Cada Uno

| Situación | Usar |
|-----------|------|
| Un tipo por implementación | Tipo asociado |
| Múltiples tipos por implementación | Parámetro genérico |
| El tipo es "parte" del trait | Tipo asociado |
| El tipo es "configuración" externa | Parámetro genérico |

## Default Type Parameters

```rust
// Parámetro con valor por defecto
struct Contenedor<T = i32> {
    valor: T,
}

fn main() {
    let c1: Contenedor = Contenedor { valor: 42 };  // T = i32
    let c2: Contenedor<String> = Contenedor { valor: String::from("hola") };
}

// En traits
trait Sumar<Rhs = Self> {
    type Output;
    fn sumar(self, rhs: Rhs) -> Self::Output;
}
```

## Const Generics

Los **const generics** permiten parametrizar por valores constantes:

```rust
// Array de tamaño genérico
struct ArrayWrapper<T, const N: usize> {
    datos: [T; N],
}

impl<T: Default + Copy, const N: usize> ArrayWrapper<T, N> {
    fn new() -> Self {
        Self {
            datos: [T::default(); N],
        }
    }
    
    fn len(&self) -> usize {
        N
    }
}

fn main() {
    let arr5: ArrayWrapper<i32, 5> = ArrayWrapper::new();
    let arr10: ArrayWrapper<f64, 10> = ArrayWrapper::new();
    
    println!("Longitud: {}", arr5.len());   // 5
    println!("Longitud: {}", arr10.len());  // 10
}
```

### Funciones con Const Generics

```rust
fn crear_array<T: Default + Copy, const N: usize>() -> [T; N] {
    [T::default(); N]
}

fn imprimir_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 5] = crear_array();
    imprimir_array(&arr);
}
```

## PhantomData

`PhantomData<T>` indica al compilador que un tipo "usa" `T` aunque no lo almacene:

```rust
use std::marker::PhantomData;

// Sin PhantomData: T no se usa realmente
struct Id<T> {
    valor: u64,
    _marker: PhantomData<T>,
}

struct Usuario;
struct Producto;

impl<T> Id<T> {
    fn new(valor: u64) -> Self {
        Self {
            valor,
            _marker: PhantomData,
        }
    }
    
    fn valor(&self) -> u64 {
        self.valor
    }
}

fn main() {
    let id_usuario: Id<Usuario> = Id::new(1);
    let id_producto: Id<Producto> = Id::new(1);
    
    // Aunque ambos tienen valor 1, son tipos diferentes
    // procesar_usuario(id_producto); // ❌ Error de tipos
}

fn procesar_usuario(id: Id<Usuario>) {
    println!("Usuario ID: {}", id.valor());
}
```

### Usos de PhantomData

```rust
use std::marker::PhantomData;

// 1. Type-safe IDs
struct TypedId<T> {
    id: u64,
    _type: PhantomData<T>,
}

// 2. Indicar lifetime
struct Referencia<'a, T> {
    ptr: *const T,
    _lifetime: PhantomData<&'a T>,
}

// 3. Indicar ownership
struct Owner<T> {
    ptr: *mut T,
    _owns: PhantomData<T>,
}
```

## Turbofish Avanzado

```rust
fn main() {
    // Turbofish en métodos
    let v: Vec<i32> = Vec::new();
    let v = Vec::<i32>::new();
    
    // En collect
    let nums: Vec<i32> = (0..10).collect();
    let nums = (0..10).collect::<Vec<i32>>();
    let nums = (0..10).collect::<Vec<_>>();  // Inferir el tipo interno
    
    // En parse
    let n: i32 = "42".parse().unwrap();
    let n = "42".parse::<i32>().unwrap();
    
    // En funciones genéricas
    fn identidad<T>(x: T) -> T { x }
    let x = identidad::<i32>(42);
}
```

## Higher-Ranked Trait Bounds (HRTB)

Para funciones que aceptan closures con referencias:

```rust
// Problema: ¿qué lifetime tiene la referencia?
fn aplicar<F>(f: F)
where
    F: Fn(&i32) -> i32,  // ¿'a? ¿'static?
{
    let x = 42;
    println!("{}", f(&x));
}

// Solución: for<'a> significa "para cualquier lifetime"
fn aplicar_hrtb<F>(f: F)
where
    F: for<'a> Fn(&'a i32) -> i32,
{
    let x = 42;
    println!("{}", f(&x));
}

fn main() {
    aplicar_hrtb(|x| x * 2);
}
```

## Blanket Implementations

Implementaciones que aplican a todos los tipos que cumplen ciertos bounds:

```rust
// En la std: ToString para todo lo que implemente Display
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

// Tu propia blanket implementation
trait MiTrait {
    fn hacer_algo(&self);
}

// Implementar para todos los T que sean Clone + Debug
impl<T: Clone + std::fmt::Debug> MiTrait for T {
    fn hacer_algo(&self) {
        let copia = self.clone();
        println!("{:?}", copia);
    }
}
```

## Generics y Lifetimes

```rust
// Struct genérico con lifetime
struct Referencia<'a, T> {
    valor: &'a T,
}

impl<'a, T> Referencia<'a, T> {
    fn new(valor: &'a T) -> Self {
        Self { valor }
    }
    
    fn obtener(&self) -> &T {
        self.valor
    }
}

// Función con lifetime y generic
fn mayor_ref<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a > b { a } else { b }
}
```

## impl Trait en Posición de Retorno

```rust
// Ocultar el tipo concreto de retorno
fn crear_iterador() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}

fn numeros_pares(hasta: i32) -> impl Iterator<Item = i32> {
    (0..hasta).filter(|n| n % 2 == 0)
}

// Útil para closures (tipos innombrables)
fn crear_sumador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

## Dyn vs Generics

```rust
use std::fmt::Display;

// Static dispatch (generics) - código duplicado, más rápido
fn imprimir_static<T: Display>(valor: T) {
    println!("{}", valor);
}

// Dynamic dispatch (dyn) - un solo código, indirección
fn imprimir_dynamic(valor: &dyn Display) {
    println!("{}", valor);
}

fn main() {
    imprimir_static(42);      // Genera: imprimir_static_i32
    imprimir_static("hola");  // Genera: imprimir_static_str
    
    imprimir_dynamic(&42);    // Usa vtable
    imprimir_dynamic(&"hola"); // Usa vtable
}
```

### Comparación

| Aspecto | Generics | dyn Trait |
|---------|----------|-----------|
| Dispatch | Estático | Dinámico |
| Performance | Más rápido | Overhead de vtable |
| Tamaño binario | Mayor | Menor |
| Colecciones heterogéneas | No | Sí |
| Object safety | No requerido | Requerido |

## Patrones Avanzados

### Type State Pattern

```rust
struct Desconectado;
struct Conectado;

struct Conexion<Estado> {
    _estado: std::marker::PhantomData<Estado>,
}

impl Conexion<Desconectado> {
    fn new() -> Self {
        Self { _estado: std::marker::PhantomData }
    }
    
    fn conectar(self) -> Conexion<Conectado> {
        println!("Conectando...");
        Conexion { _estado: std::marker::PhantomData }
    }
}

impl Conexion<Conectado> {
    fn enviar(&self, msg: &str) {
        println!("Enviando: {}", msg);
    }
    
    fn desconectar(self) -> Conexion<Desconectado> {
        println!("Desconectando...");
        Conexion { _estado: std::marker::PhantomData }
    }
}

fn main() {
    let conn = Conexion::new();
    // conn.enviar("hola"); // ❌ Error: no está conectado
    
    let conn = conn.conectar();
    conn.enviar("hola"); // ✅ OK
    
    let conn = conn.desconectar();
    // conn.enviar("hola"); // ❌ Error: ya no está conectado
}
```

## Resumen

| Concepto | Descripción |
|----------|-------------|
| Tipo asociado | `type Item` - un tipo por implementación |
| Default type | `<T = i32>` - tipo por defecto |
| Const generics | `<const N: usize>` - valores constantes |
| PhantomData | Marcar uso de tipo sin almacenarlo |
| HRTB | `for<'a>` - cualquier lifetime |
| Blanket impl | `impl<T: Bound> Trait for T` |
| impl Trait | Ocultar tipo concreto de retorno |
