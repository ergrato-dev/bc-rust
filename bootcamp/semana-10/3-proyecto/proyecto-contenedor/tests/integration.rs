//! Tests de integración para la biblioteca de contenedores

use proyecto_contenedor::{Cache, Cola, Deque, Limitado};

#[test]
fn test_cola_strings() {
    let mut cola: Cola<String> = Cola::new();

    cola.encolar("primero".to_string());
    cola.encolar("segundo".to_string());
    cola.encolar("tercero".to_string());

    assert_eq!(cola.desencolar(), Some("primero".to_string()));
    assert_eq!(cola.desencolar(), Some("segundo".to_string()));
    assert_eq!(cola.desencolar(), Some("tercero".to_string()));
    assert_eq!(cola.desencolar(), None);
}

#[test]
fn test_deque_operaciones_mixtas() {
    let mut deque: Deque<i32> = Deque::new();

    // Construir: [1, 2, 3, 4, 5]
    deque.push_atras(3);
    deque.push_frente(2);
    deque.push_frente(1);
    deque.push_atras(4);
    deque.push_atras(5);

    // Verificar extremos
    assert_eq!(deque.frente(), Some(&1));
    assert_eq!(deque.atras(), Some(&5));

    // Remover alternando
    assert_eq!(deque.pop_frente(), Some(1));
    assert_eq!(deque.pop_atras(), Some(5));
    assert_eq!(deque.pop_frente(), Some(2));
    assert_eq!(deque.pop_atras(), Some(4));
    assert_eq!(deque.pop_frente(), Some(3));
    assert!(deque.esta_vacia());
}

#[test]
fn test_limitado_overflow_handling() {
    let mut contenedor: Limitado<char, 3> = Limitado::new();

    // Llenar el contenedor
    assert!(contenedor.insertar('a').is_ok());
    assert!(contenedor.insertar('b').is_ok());
    assert!(contenedor.insertar('c').is_ok());
    assert!(contenedor.esta_lleno());

    // Intentar insertar cuando está lleno
    let resultado = contenedor.insertar('d');
    assert_eq!(resultado, Err('d'));

    // Remover uno y volver a intentar
    contenedor.remover();
    assert!(!contenedor.esta_lleno());
    assert!(contenedor.insertar('d').is_ok());
}

#[test]
fn test_cache_lru_completo() {
    let mut cache: Cache<&str, i32> = Cache::new(3);

    // Insertar tres elementos
    cache.insertar("a", 1);
    cache.insertar("b", 2);
    cache.insertar("c", 3);

    // Orden actual: a, b, c (c es más reciente)

    // Acceder a "a" lo hace más reciente
    // Orden: b, c, a
    cache.obtener(&"a");

    // Insertar "d" debería eliminar "b" (LRU)
    cache.insertar("d", 4);

    assert!(cache.contiene(&"a"));
    assert!(!cache.contiene(&"b")); // Eliminado
    assert!(cache.contiene(&"c"));
    assert!(cache.contiene(&"d"));
}

#[test]
fn test_contenedores_con_structs_custom() {
    #[derive(Debug, Clone, PartialEq)]
    struct Persona {
        nombre: String,
        edad: u32,
    }

    let mut cola: Cola<Persona> = Cola::new();
    cola.encolar(Persona {
        nombre: "Ana".to_string(),
        edad: 25,
    });
    cola.encolar(Persona {
        nombre: "Bob".to_string(),
        edad: 30,
    });

    let persona = cola.desencolar().unwrap();
    assert_eq!(persona.nombre, "Ana");
}

#[test]
fn test_limitado_diferentes_tamaños_compilacion() {
    // Verificar que const generics funciona con diferentes tamaños
    let _tiny: Limitado<u8, 1> = Limitado::new();
    let _small: Limitado<u8, 10> = Limitado::new();
    let _medium: Limitado<u8, 100> = Limitado::new();
    let _large: Limitado<u8, 1000> = Limitado::new();

    assert_eq!(_tiny.capacidad(), 1);
    assert_eq!(_small.capacidad(), 10);
    assert_eq!(_medium.capacidad(), 100);
    assert_eq!(_large.capacidad(), 1000);
}

#[test]
fn test_cache_actualizar_valor() {
    let mut cache: Cache<i32, &str> = Cache::new(2);

    cache.insertar(1, "uno");
    cache.insertar(1, "ONE"); // Actualizar

    assert_eq!(cache.obtener(&1), Some(&"ONE"));
    assert_eq!(cache.len(), 1); // Sigue siendo 1 elemento
}

#[test]
fn test_combinacion_contenedores() {
    // Cola de deques
    let mut cola_de_deques: Cola<Deque<i32>> = Cola::new();

    let mut deque1 = Deque::new();
    deque1.push_atras(1);
    deque1.push_atras(2);

    let mut deque2 = Deque::new();
    deque2.push_atras(3);
    deque2.push_atras(4);

    cola_de_deques.encolar(deque1);
    cola_de_deques.encolar(deque2);

    let mut primera_deque = cola_de_deques.desencolar().unwrap();
    assert_eq!(primera_deque.pop_frente(), Some(1));
}
