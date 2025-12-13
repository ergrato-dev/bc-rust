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
    let rango = RangoNumerico::new(1, 10);
    println!("Rango: {:?}", rango.siguiente());
    println!("Rango: {:?}", rango.siguiente());

    // Ejercicio 2: Const generics
    let buffer: Buffer<u8, 4> = Buffer::new([1, 2, 3, 4]);
    println!("\nBuffer capacidad: {}", buffer.capacidad());
    println!("Buffer[0]: {}", buffer.obtener(0).unwrap());

    // Ejercicio 3: Type State Pattern
    let pedido = Pedido::nuevo("Pizza Margherita");
    println!("\nPedido creado: {}", pedido.descripcion());

    let pedido_pagado = pedido.pagar();
    println!("Pedido pagado: {}", pedido_pagado.descripcion());

    let pedido_enviado = pedido_pagado.enviar();
    println!("Pedido enviado: {}", pedido_enviado.descripcion());

    // Solo los pedidos enviados pueden ser entregados
    let pedido_entregado = pedido_enviado.entregar();
    println!("Pedido entregado: {}", pedido_entregado.descripcion());

    // Ejercicio 4: PhantomData para IDs tipados
    let user_id: Id<Usuario> = Id::new(1);
    let product_id: Id<Producto> = Id::new(1);

    println!("\nUser ID: {}", user_id.valor());
    println!("Product ID: {}", product_id.valor());

    // Esto NO debería compilar (descomentar para probar):
    // let igual = user_id == product_id; // Error: tipos diferentes!

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Trait con Tipo Asociado
// ============================================
// Implementa un trait Iterador simplificado con un tipo asociado.
// El tipo asociado define qué tipo de elementos produce.

trait Iterador {
    // TODO: Define un tipo asociado llamado Item
    type Item;

    // TODO: Define el método siguiente que devuelve Option<Self::Item>
    fn siguiente(&self) -> Option<Self::Item>;
}

// Struct que genera números en un rango
struct RangoNumerico {
    actual: std::cell::Cell<i32>,
    fin: i32,
}

impl RangoNumerico {
    fn new(inicio: i32, fin: i32) -> Self {
        RangoNumerico {
            actual: std::cell::Cell::new(inicio),
            fin,
        }
    }
}

impl Iterador for RangoNumerico {
    // TODO: Especifica que Item = i32
    type Item = (); // Cambia () por i32

    fn siguiente(&self) -> Option<Self::Item> {
        // TODO: Implementa la lógica
        // Si actual < fin, devuelve Some(actual) e incrementa
        // Si no, devuelve None
        todo!("Implementa Iterador para RangoNumerico")
    }
}

// ============================================
// EJERCICIO 2: Const Generics
// ============================================
// Implementa un buffer de tamaño fijo usando const generics.
// N es el tamaño del buffer conocido en tiempo de compilación.

struct Buffer<T, const N: usize> {
    // TODO: Define un campo `datos` de tipo [T; N]
    _datos: PhantomData<T>, // Elimina esto al implementar
}

impl<T, const N: usize> Buffer<T, N> {
    fn new(_datos: [T; N]) -> Self {
        // TODO: Crea un nuevo Buffer con los datos
        todo!("Implementa Buffer::new")
    }

    fn capacidad(&self) -> usize {
        // TODO: Devuelve N (la capacidad del buffer)
        todo!("Implementa Buffer::capacidad")
    }

    fn obtener(&self, _indice: usize) -> Option<&T> {
        // TODO: Devuelve referencia al elemento en indice si existe
        todo!("Implementa Buffer::obtener")
    }
}

// Implementación adicional para Buffer con Copy
impl<T: Copy, const N: usize> Buffer<T, N> {
    fn obtener_copia(&self, _indice: usize) -> Option<T> {
        // TODO: Devuelve copia del elemento en indice
        todo!("Implementa Buffer::obtener_copia")
    }
}

// ============================================
// EJERCICIO 3: Type State Pattern
// ============================================
// Implementa un sistema de pedidos donde el estado
// está codificado en el tipo, previniendo transiciones inválidas.

// Estados del pedido (tipos marcadores)
struct Pendiente;
struct Pagado;
struct Enviado;
struct Entregado;

// Pedido genérico sobre su estado
struct Pedido<Estado> {
    descripcion: String,
    _estado: PhantomData<Estado>,
}

// Implementación para cualquier estado
impl<Estado> Pedido<Estado> {
    fn descripcion(&self) -> &str {
        &self.descripcion
    }
}

// Solo Pedido<Pendiente> puede ser creado y pagado
impl Pedido<Pendiente> {
    fn nuevo(descripcion: &str) -> Self {
        // TODO: Crea un pedido en estado Pendiente
        todo!("Implementa Pedido::nuevo")
    }

    fn pagar(self) -> Pedido<Pagado> {
        // TODO: Transiciona a estado Pagado
        // Consume self y devuelve nuevo Pedido<Pagado>
        todo!("Implementa Pedido::pagar")
    }
}

// Solo Pedido<Pagado> puede ser enviado
impl Pedido<Pagado> {
    fn enviar(self) -> Pedido<Enviado> {
        // TODO: Transiciona a estado Enviado
        todo!("Implementa Pedido::enviar")
    }
}

// Solo Pedido<Enviado> puede ser entregado
impl Pedido<Enviado> {
    fn entregar(self) -> Pedido<Entregado> {
        // TODO: Transiciona a estado Entregado
        todo!("Implementa Pedido::entregar")
    }
}

// ============================================
// EJERCICIO 4: PhantomData para IDs Tipados
// ============================================
// Implementa IDs que son únicos por tipo de entidad.
// Un Id<Usuario> no puede confundirse con un Id<Producto>.

// Tipos marcadores para entidades
struct Usuario;
struct Producto;

// ID genérico que "pertenece" a un tipo de entidad
#[derive(Debug)]
struct Id<T> {
    valor: u64,
    // TODO: Añade PhantomData<T> para "usar" el tipo T
    _marker: PhantomData<T>, // Esto ya está correcto
}

impl<T> Id<T> {
    fn new(valor: u64) -> Self {
        // TODO: Crea un nuevo Id
        todo!("Implementa Id::new")
    }

    fn valor(&self) -> u64 {
        // TODO: Devuelve el valor interno
        todo!("Implementa Id::valor")
    }
}

// Implementa PartialEq solo para IDs del mismo tipo
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compara los valores
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
    fn test_rango_numerico() {
        let rango = RangoNumerico::new(0, 3);
        assert_eq!(rango.siguiente(), Some(0));
        assert_eq!(rango.siguiente(), Some(1));
        assert_eq!(rango.siguiente(), Some(2));
        assert_eq!(rango.siguiente(), None);
    }

    // Tests Ejercicio 2: Const Generics
    #[test]
    fn test_buffer_capacidad() {
        let buffer: Buffer<i32, 5> = Buffer::new([1, 2, 3, 4, 5]);
        assert_eq!(buffer.capacidad(), 5);
    }

    #[test]
    fn test_buffer_obtener() {
        let buffer: Buffer<char, 3> = Buffer::new(['a', 'b', 'c']);
        assert_eq!(buffer.obtener(0), Some(&'a'));
        assert_eq!(buffer.obtener(2), Some(&'c'));
        assert_eq!(buffer.obtener(3), None);
    }

    #[test]
    fn test_buffer_obtener_copia() {
        let buffer: Buffer<i32, 3> = Buffer::new([10, 20, 30]);
        assert_eq!(buffer.obtener_copia(1), Some(20));
    }

    // Tests Ejercicio 3: Type State
    #[test]
    fn test_pedido_flujo_completo() {
        let pedido = Pedido::nuevo("Test");
        assert_eq!(pedido.descripcion(), "Test");

        let pagado = pedido.pagar();
        assert_eq!(pagado.descripcion(), "Test");

        let enviado = pagado.enviar();
        assert_eq!(enviado.descripcion(), "Test");

        let entregado = enviado.entregar();
        assert_eq!(entregado.descripcion(), "Test");
    }

    // Tests Ejercicio 4: IDs Tipados
    #[test]
    fn test_id_creacion() {
        let id: Id<Usuario> = Id::new(42);
        assert_eq!(id.valor(), 42);
    }

    #[test]
    fn test_id_igualdad_mismo_tipo() {
        let id1: Id<Usuario> = Id::new(1);
        let id2: Id<Usuario> = Id::new(1);
        let id3: Id<Usuario> = Id::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    // Este test verifica que IDs de diferentes tipos NO son comparables
    // Si descomentas la línea, no debería compilar
    #[test]
    fn test_id_tipos_diferentes_no_comparables() {
        let _user_id: Id<Usuario> = Id::new(1);
        let _product_id: Id<Producto> = Id::new(1);

        // La siguiente línea NO debe compilar:
        // assert_ne!(user_id, product_id);

        // Si llegamos aquí, el sistema de tipos funciona correctamente
        assert!(true);
    }
}
