// ============================================
// PRÁCTICA 02: Structs y Enums Genéricos
// ============================================
// Objetivo: Crear estructuras de datos genéricas reutilizables
//
// Instrucciones:
// 1. Implementa cada struct/enum marcado con TODO
// 2. Implementa los métodos asociados
// 3. Ejecuta `cargo test` para verificar tu implementación

fn main() {
    println!("=== Práctica 02: Structs y Enums Genéricos ===\n");

    // Ejercicio 1: Par genérico
    let par_numeros = Par::new(10, 20);
    println!("Par: ({}, {})", par_numeros.primero(), par_numeros.segundo());
    let par_invertido = par_numeros.invertir();
    println!("Invertido: ({}, {})", par_invertido.primero(), par_invertido.segundo());

    // Ejercicio 2: Caja genérica
    let caja_numero: Caja<i32> = Caja::new(42);
    println!("\nCaja contiene: {}", caja_numero.valor());
    let caja_doble = caja_numero.map(|x| x * 2);
    println!("Caja mapeada: {}", caja_doble.valor());

    // Ejercicio 3: Punto genérico
    let punto = Punto::new(3.0, 4.0);
    println!("\nPunto: ({}, {})", punto.x(), punto.y());
    println!("Distancia al origen: {:.2}", punto.distancia_origen());

    // Ejercicio 4: Resultado simplificado
    let exito: Resultado<i32, String> = Resultado::exito(100);
    let fallo: Resultado<i32, String> = Resultado::fallo("Error de cálculo".to_string());

    println!("\n¿Es éxito?: {}", exito.es_exito());
    println!("¿Es fallo?: {}", fallo.es_fallo());

    if let Some(valor) = exito.obtener_valor() {
        println!("Valor del éxito: {valor}");
    }

    // Ejercicio 5: Pila genérica
    let mut pila: Pila<i32> = Pila::new();
    pila.push(1);
    pila.push(2);
    pila.push(3);
    println!("\nPila tiene {} elementos", pila.len());
    println!("Tope: {:?}", pila.peek());
    println!("Pop: {:?}", pila.pop());
    println!("Pila tiene {} elementos", pila.len());

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Par Genérico
// ============================================
// Implementa un struct que almacena dos valores del mismo tipo.
//
// Métodos requeridos:
// - new(primero, segundo) -> Par<T>
// - primero(&self) -> &T
// - segundo(&self) -> &T
// - invertir(self) -> Par<T>

struct Par<T> {
    // TODO: Define los campos
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Par<T> {
    fn new(_primero: T, _segundo: T) -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa Par::new")
    }

    fn primero(&self) -> &T {
        // TODO: Devuelve referencia al primer elemento
        todo!("Implementa Par::primero")
    }

    fn segundo(&self) -> &T {
        // TODO: Devuelve referencia al segundo elemento
        todo!("Implementa Par::segundo")
    }

    fn invertir(self) -> Par<T> {
        // TODO: Devuelve un nuevo Par con los elementos invertidos
        todo!("Implementa Par::invertir")
    }
}

// ============================================
// EJERCICIO 2: Caja Genérica
// ============================================
// Implementa una "caja" que envuelve cualquier valor.
// Similar a Box pero sin heap allocation.
//
// Métodos requeridos:
// - new(valor) -> Caja<T>
// - valor(&self) -> &T
// - desenvolver(self) -> T
// - map<U, F>(self, f: F) -> Caja<U> where F: FnOnce(T) -> U

struct Caja<T> {
    // TODO: Define el campo
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Caja<T> {
    fn new(_valor: T) -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa Caja::new")
    }

    fn valor(&self) -> &T {
        // TODO: Devuelve referencia al valor
        todo!("Implementa Caja::valor")
    }

    fn desenvolver(self) -> T {
        // TODO: Consume la caja y devuelve el valor
        todo!("Implementa Caja::desenvolver")
    }

    fn map<U, F>(self, _f: F) -> Caja<U>
    where
        F: FnOnce(T) -> U,
    {
        // TODO: Aplica la función al valor y devuelve nueva Caja
        todo!("Implementa Caja::map")
    }
}

// ============================================
// EJERCICIO 3: Punto Genérico
// ============================================
// Implementa un punto 2D genérico con métodos especializados.
//
// Para cualquier T:
// - new(x, y) -> Punto<T>
// - x(&self) -> &T
// - y(&self) -> &T
//
// Solo para f64:
// - distancia_origen(&self) -> f64

struct Punto<T> {
    // TODO: Define los campos x, y
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Punto<T> {
    fn new(_x: T, _y: T) -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa Punto::new")
    }

    fn x(&self) -> &T {
        // TODO: Devuelve referencia a x
        todo!("Implementa Punto::x")
    }

    fn y(&self) -> &T {
        // TODO: Devuelve referencia a y
        todo!("Implementa Punto::y")
    }
}

// Implementación especializada solo para Punto<f64>
impl Punto<f64> {
    fn distancia_origen(&self) -> f64 {
        // TODO: Calcula sqrt(x² + y²)
        // Usa: (self.x.powi(2) + self.y.powi(2)).sqrt()
        todo!("Implementa Punto::distancia_origen")
    }
}

// ============================================
// EJERCICIO 4: Resultado Simplificado
// ============================================
// Implementa un enum similar a Result pero simplificado.
//
// Variantes:
// - Exito(T)
// - Fallo(E)
//
// Métodos:
// - exito(valor) -> Resultado<T, E>
// - fallo(error) -> Resultado<T, E>
// - es_exito(&self) -> bool
// - es_fallo(&self) -> bool
// - obtener_valor(self) -> Option<T>

enum Resultado<T, E> {
    // TODO: Define las variantes
    _Temporal(std::marker::PhantomData<(T, E)>), // Elimina esto al implementar
}

impl<T, E> Resultado<T, E> {
    fn exito(_valor: T) -> Self {
        // TODO: Crea un Resultado exitoso
        todo!("Implementa Resultado::exito")
    }

    fn fallo(_error: E) -> Self {
        // TODO: Crea un Resultado fallido
        todo!("Implementa Resultado::fallo")
    }

    fn es_exito(&self) -> bool {
        // TODO: Devuelve true si es Exito
        todo!("Implementa Resultado::es_exito")
    }

    fn es_fallo(&self) -> bool {
        // TODO: Devuelve true si es Fallo
        todo!("Implementa Resultado::es_fallo")
    }

    fn obtener_valor(self) -> Option<T> {
        // TODO: Devuelve Some(valor) si es Exito, None si es Fallo
        todo!("Implementa Resultado::obtener_valor")
    }
}

// ============================================
// EJERCICIO 5: Pila Genérica
// ============================================
// Implementa una pila (stack) genérica usando Vec.
//
// Métodos:
// - new() -> Pila<T>
// - push(&mut self, valor: T)
// - pop(&mut self) -> Option<T>
// - peek(&self) -> Option<&T>
// - len(&self) -> usize
// - esta_vacia(&self) -> bool

struct Pila<T> {
    // TODO: Define el campo (usa Vec<T>)
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Pila<T> {
    fn new() -> Self {
        // TODO: Crea una pila vacía
        todo!("Implementa Pila::new")
    }

    fn push(&mut self, _valor: T) {
        // TODO: Agrega un elemento al tope
        todo!("Implementa Pila::push")
    }

    fn pop(&mut self) -> Option<T> {
        // TODO: Remueve y devuelve el elemento del tope
        todo!("Implementa Pila::pop")
    }

    fn peek(&self) -> Option<&T> {
        // TODO: Devuelve referencia al elemento del tope sin removerlo
        todo!("Implementa Pila::peek")
    }

    fn len(&self) -> usize {
        // TODO: Devuelve la cantidad de elementos
        todo!("Implementa Pila::len")
    }

    fn esta_vacia(&self) -> bool {
        // TODO: Devuelve true si la pila está vacía
        todo!("Implementa Pila::esta_vacia")
    }
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Tests para Par
    #[test]
    fn test_par_new() {
        let par = Par::new(1, 2);
        assert_eq!(*par.primero(), 1);
        assert_eq!(*par.segundo(), 2);
    }

    #[test]
    fn test_par_invertir() {
        let par = Par::new("a", "b");
        let invertido = par.invertir();
        assert_eq!(*invertido.primero(), "b");
        assert_eq!(*invertido.segundo(), "a");
    }

    // Tests para Caja
    #[test]
    fn test_caja_new_y_valor() {
        let caja = Caja::new(42);
        assert_eq!(*caja.valor(), 42);
    }

    #[test]
    fn test_caja_desenvolver() {
        let caja = Caja::new(String::from("hola"));
        let valor = caja.desenvolver();
        assert_eq!(valor, "hola");
    }

    #[test]
    fn test_caja_map() {
        let caja = Caja::new(10);
        let caja_mapeada = caja.map(|x| x * 3);
        assert_eq!(*caja_mapeada.valor(), 30);
    }

    #[test]
    fn test_caja_map_tipo_diferente() {
        let caja = Caja::new(42);
        let caja_string = caja.map(|x| x.to_string());
        assert_eq!(*caja_string.valor(), "42");
    }

    // Tests para Punto
    #[test]
    fn test_punto_new() {
        let punto = Punto::new(3, 4);
        assert_eq!(*punto.x(), 3);
        assert_eq!(*punto.y(), 4);
    }

    #[test]
    fn test_punto_distancia_origen() {
        let punto = Punto::new(3.0, 4.0);
        assert!((punto.distancia_origen() - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_punto_distancia_origen_cero() {
        let punto = Punto::new(0.0, 0.0);
        assert!((punto.distancia_origen() - 0.0).abs() < 0.0001);
    }

    // Tests para Resultado
    #[test]
    fn test_resultado_exito() {
        let resultado: Resultado<i32, &str> = Resultado::exito(42);
        assert!(resultado.es_exito());
        assert!(!resultado.es_fallo());
    }

    #[test]
    fn test_resultado_fallo() {
        let resultado: Resultado<i32, &str> = Resultado::fallo("error");
        assert!(!resultado.es_exito());
        assert!(resultado.es_fallo());
    }

    #[test]
    fn test_resultado_obtener_valor_exito() {
        let resultado: Resultado<i32, &str> = Resultado::exito(100);
        assert_eq!(resultado.obtener_valor(), Some(100));
    }

    #[test]
    fn test_resultado_obtener_valor_fallo() {
        let resultado: Resultado<i32, &str> = Resultado::fallo("error");
        assert_eq!(resultado.obtener_valor(), None);
    }

    // Tests para Pila
    #[test]
    fn test_pila_new() {
        let pila: Pila<i32> = Pila::new();
        assert!(pila.esta_vacia());
        assert_eq!(pila.len(), 0);
    }

    #[test]
    fn test_pila_push_pop() {
        let mut pila = Pila::new();
        pila.push(1);
        pila.push(2);
        pila.push(3);

        assert_eq!(pila.pop(), Some(3));
        assert_eq!(pila.pop(), Some(2));
        assert_eq!(pila.pop(), Some(1));
        assert_eq!(pila.pop(), None);
    }

    #[test]
    fn test_pila_peek() {
        let mut pila = Pila::new();
        assert_eq!(pila.peek(), None);

        pila.push(42);
        assert_eq!(pila.peek(), Some(&42));
        assert_eq!(pila.len(), 1); // peek no remueve
    }

    #[test]
    fn test_pila_len() {
        let mut pila = Pila::new();
        assert_eq!(pila.len(), 0);

        pila.push("a");
        pila.push("b");
        assert_eq!(pila.len(), 2);

        pila.pop();
        assert_eq!(pila.len(), 1);
    }
}
