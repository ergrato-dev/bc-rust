// ============================================
// PRÁCTICA 04: Genéricos Avanzados
// ============================================
// Objetivo: Explorar características avanzadas de genéricos
//
// Temas:
// - Tipos asociados
// - Const generics
// - PhantomData y Type State Pattern
//
// Instrucciones:
// 1. Implementa cada ejercicio marcado con TODO
// 2. Lee los comentarios para entender el contexto
// 3. Ejecuta `cargo test` para verificar

use std::marker::PhantomData;

fn main() {
    println!("=== Práctica 04: Genéricos Avanzados ===\n");

    // Ejercicio 1: Trait con tipo asociado
    let range = NumericRange::new(1, 10);
    println!("Range: {:?}", range.next());
    println!("Range: {:?}", range.next());

    // Ejercicio 2: Const generics
    let buffer: Buffer<u8, 4> = Buffer::new([1, 2, 3, 4]);
    println!("\nBuffer capacity: {}", buffer.capacity());
    println!("Buffer[0]: {}", buffer.get(0).unwrap());

    // Ejercicio 3: Type State Pattern
    let order = Order::new("Pizza Margherita");
    println!("\nOrder created: {}", order.description());

    let paid_order = order.pay();
    println!("Order paid: {}", paid_order.description());

    let shipped_order = paid_order.ship();
    println!("Order shipped: {}", shipped_order.description());

    // Solo los pedidos enviados pueden ser entregados
    let delivered_order = shipped_order.deliver();
    println!("Order delivered: {}", delivered_order.description());

    // Ejercicio 4: PhantomData para IDs tipados
    let user_id: Id<User> = Id::new(1);
    let product_id: Id<Product> = Id::new(1);

    println!("\nUser ID: {}", user_id.value());
    println!("Product ID: {}", product_id.value());

    // Esto NO debería compilar (descomentar para probar):
    // let igual = user_id == product_id; // Error: tipos diferentes!

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Trait con Tipo Asociado
// ============================================
// Implementa un trait SimpleIterator simplificado con un tipo asociado.
// El tipo asociado define qué tipo de elementos produce.

trait SimpleIterator {
    // TODO: Define un tipo asociado llamado Item
    type Item;

    // TODO: Define el método next que devuelve Option<Self::Item>
    fn next(&self) -> Option<Self::Item>;
}

// Struct que genera números en un rango
struct NumericRange {
    current: std::cell::Cell<i32>,
    end: i32,
}

impl NumericRange {
    fn new(start: i32, end: i32) -> Self {
        NumericRange {
            current: std::cell::Cell::new(start),
            end,
        }
    }
}

impl SimpleIterator for NumericRange {
    // TODO: Especifica que Item = i32
    type Item = (); // Cambia () por i32

    fn next(&self) -> Option<Self::Item> {
        // TODO: Implementa la lógica
        // Si current < end, devuelve Some(current) e incrementa
        // Si no, devuelve None
        todo!("Implementa SimpleIterator para NumericRange")
    }
}

// ============================================
// EJERCICIO 2: Const Generics
// ============================================
// Implementa un buffer de tamaño fijo usando const generics.
// N es el tamaño del buffer conocido en tiempo de compilación.

struct Buffer<T, const N: usize> {
    // TODO: Define un campo `data` de tipo [T; N]
    _data: PhantomData<T>, // Elimina esto al implementar
}

impl<T, const N: usize> Buffer<T, N> {
    fn new(_data: [T; N]) -> Self {
        // TODO: Crea un nuevo Buffer con los datos
        todo!("Implementa Buffer::new")
    }

    fn capacity(&self) -> usize {
        // TODO: Devuelve N (la capacidad del buffer)
        todo!("Implementa Buffer::capacity")
    }

    fn get(&self, _index: usize) -> Option<&T> {
        // TODO: Devuelve referencia al elemento en index si existe
        todo!("Implementa Buffer::get")
    }
}

// Implementación adicional para Buffer con Copy
impl<T: Copy, const N: usize> Buffer<T, N> {
    fn get_copy(&self, _index: usize) -> Option<T> {
        // TODO: Devuelve copia del elemento en index
        todo!("Implementa Buffer::get_copy")
    }
}

// ============================================
// EJERCICIO 3: Type State Pattern
// ============================================
// Implementa un sistema de pedidos donde el estado
// está codificado en el tipo, previniendo transiciones inválidas.

// Estados del pedido (tipos marcadores)
struct Pending;
struct Paid;
struct Shipped;
struct Delivered;

// Order genérico sobre su estado
struct Order<State> {
    description: String,
    _state: PhantomData<State>,
}

// Implementación para cualquier estado
impl<State> Order<State> {
    fn description(&self) -> &str {
        &self.description
    }
}

// Solo Order<Pending> puede ser creado y pagado
impl Order<Pending> {
    fn new(description: &str) -> Self {
        // TODO: Crea un pedido en estado Pending
        todo!("Implementa Order::new")
    }

    fn pay(self) -> Order<Paid> {
        // TODO: Transiciona a estado Paid
        // Consume self y devuelve nuevo Order<Paid>
        todo!("Implementa Order::pay")
    }
}

// Solo Order<Paid> puede ser enviado
impl Order<Paid> {
    fn ship(self) -> Order<Shipped> {
        // TODO: Transiciona a estado Shipped
        todo!("Implementa Order::ship")
    }
}

// Solo Order<Shipped> puede ser entregado
impl Order<Shipped> {
    fn deliver(self) -> Order<Delivered> {
        // TODO: Transiciona a estado Delivered
        todo!("Implementa Order::deliver")
    }
}

// ============================================
// EJERCICIO 4: PhantomData para IDs Tipados
// ============================================
// Implementa IDs que son únicos por tipo de entidad.
// Un Id<User> no puede confundirse con un Id<Product>.

// Tipos marcadores para entidades
struct User;
struct Product;

// ID genérico que "pertenece" a un tipo de entidad
#[derive(Debug)]
struct Id<T> {
    value: u64,
    // TODO: Añade PhantomData<T> para "usar" el tipo T
    _marker: PhantomData<T>, // Esto ya está correcto
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        // TODO: Crea un nuevo Id
        todo!("Implementa Id::new")
    }

    fn value(&self) -> u64 {
        // TODO: Devuelve el value interno
        todo!("Implementa Id::value")
    }
}

// Implementa PartialEq solo para IDs del mismo tipo
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compara los values
        todo!("Implementa PartialEq para Id")
    }
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1: Tipo Asociado
    #[test]
    fn test_numeric_range() {
        let range = NumericRange::new(0, 3);
        assert_eq!(range.next(), Some(0));
        assert_eq!(range.next(), Some(1));
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next(), None);
    }

    // Tests Ejercicio 2: Const Generics
    #[test]
    fn test_buffer_capacity() {
        let buffer: Buffer<i32, 5> = Buffer::new([1, 2, 3, 4, 5]);
        assert_eq!(buffer.capacity(), 5);
    }

    #[test]
    fn test_buffer_get() {
        let buffer: Buffer<char, 3> = Buffer::new(['a', 'b', 'c']);
        assert_eq!(buffer.get(0), Some(&'a'));
        assert_eq!(buffer.get(2), Some(&'c'));
        assert_eq!(buffer.get(3), None);
    }

    #[test]
    fn test_buffer_get_copy() {
        let buffer: Buffer<i32, 3> = Buffer::new([10, 20, 30]);
        assert_eq!(buffer.get_copy(1), Some(20));
    }

    // Tests Ejercicio 3: Type State
    #[test]
    fn test_order_complete_flow() {
        let order = Order::new("Test");
        assert_eq!(order.description(), "Test");

        let paid = order.pay();
        assert_eq!(paid.description(), "Test");

        let shipped = paid.ship();
        assert_eq!(shipped.description(), "Test");

        let delivered = shipped.deliver();
        assert_eq!(delivered.description(), "Test");
    }

    // Tests Ejercicio 4: IDs Tipados
    #[test]
    fn test_id_creation() {
        let id: Id<User> = Id::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_id_equality_same_type() {
        let id1: Id<User> = Id::new(1);
        let id2: Id<User> = Id::new(1);
        let id3: Id<User> = Id::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    // Este test verifica que IDs de diferentes tipos NO son comparables
    // Si descomentas la línea, no debería compilar
    #[test]
    fn test_id_different_types_not_comparable() {
        let _user_id: Id<User> = Id::new(1);
        let _product_id: Id<Product> = Id::new(1);

        // La siguiente línea NO debe compilar:
        // assert_ne!(user_id, product_id);

        // Si llegamos aquí, el sistema de tipos funciona correctamente
        assert!(true);
    }
}
