//! Demostración de la biblioteca de contenedores genéricos

use proyecto_contenedor::{Cache, Cola, Deque, Limitado};

fn main() {
    println!("=== Biblioteca de Contenedores Genéricos ===\n");

    demo_cola();
    demo_deque();
    demo_limitado();
    demo_cache();

    println!("✅ ¡Todas las demos completadas!");
}

fn demo_cola() {
    println!("📦 Demo: Cola<T>");
    println!("{}", "-".repeat(40));

    let mut cola: Cola<&str> = Cola::new();
    cola.encolar("primero");
    cola.encolar("segundo");
    cola.encolar("tercero");

    println!("Cola creada con 3 elementos");
    println!("Frente: {:?}", cola.frente());
    println!("Desencolando: {:?}", cola.desencolar());
    println!("Nuevo frente: {:?}", cola.frente());
    println!("Longitud: {}\n", cola.len());
}

fn demo_deque() {
    println!("📦 Demo: Deque<T>");
    println!("{}", "-".repeat(40));

    let mut deque: Deque<i32> = Deque::new();
    deque.push_atras(2);
    deque.push_frente(1);
    deque.push_atras(3);

    println!("Deque: [1, 2, 3]");
    println!("Frente: {:?}", deque.frente());
    println!("Atrás: {:?}", deque.atras());
    println!("Pop frente: {:?}", deque.pop_frente());
    println!("Pop atrás: {:?}", deque.pop_atras());
    println!("Longitud restante: {}\n", deque.len());
}

fn demo_limitado() {
    println!("📦 Demo: Limitado<T, 3>");
    println!("{}", "-".repeat(40));

    let mut limitado: Limitado<char, 3> = Limitado::new();

    println!("Capacidad: {}", limitado.capacidad());
    println!("Insertando 'a': {:?}", limitado.insertar('a'));
    println!("Insertando 'b': {:?}", limitado.insertar('b'));
    println!("Insertando 'c': {:?}", limitado.insertar('c'));
    println!("¿Está lleno?: {}", limitado.esta_lleno());
    println!("Insertando 'd' (debería fallar): {:?}", limitado.insertar('d'));
    println!("Longitud: {}\n", limitado.len());
}

fn demo_cache() {
    println!("📦 Demo: Cache<K, V>");
    println!("{}", "-".repeat(40));

    let mut cache: Cache<&str, i32> = Cache::new(2);

    cache.insertar("uno", 1);
    cache.insertar("dos", 2);
    println!("Cache con 'uno' y 'dos'");

    println!("Obteniendo 'uno': {:?}", cache.obtener(&"uno"));

    cache.insertar("tres", 3); // Esto debería eliminar 'dos' (LRU)
    println!("Insertado 'tres' (capacidad 2)");

    println!("¿Contiene 'uno'?: {}", cache.contiene(&"uno"));
    println!("¿Contiene 'dos'?: {}", cache.contiene(&"dos")); // Debería ser false
    println!("¿Contiene 'tres'?: {}", cache.contiene(&"tres"));
    println!("Longitud: {}\n", cache.len());
}
