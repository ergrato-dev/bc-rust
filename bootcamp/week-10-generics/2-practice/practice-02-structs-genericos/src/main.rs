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

    // Ejercicio 1: Pair genérico
    let number_pair = Pair::new(10, 20);
    println!("Pair: ({}, {})", number_pair.first(), number_pair.second());
    let swapped_pair = number_pair.swap();
    println!("Swapped: ({}, {})", swapped_pair.first(), swapped_pair.second());

    // Ejercicio 2: Wrapper genérico
    let number_wrapper: Wrapper<i32> = Wrapper::new(42);
    println!("\nWrapper contains: {}", number_wrapper.value());
    let doubled_wrapper = number_wrapper.map(|x| x * 2);
    println!("Mapped wrapper: {}", doubled_wrapper.value());

    // Ejercicio 3: Point genérico
    let point = Point::new(3.0, 4.0);
    println!("\nPoint: ({}, {})", point.x(), point.y());
    println!("Distance to origin: {:.2}", point.distance_to_origin());

    // Ejercicio 4: SimpleResult simplificado
    let success: SimpleResult<i32, String> = SimpleResult::success(100);
    let failure: SimpleResult<i32, String> = SimpleResult::failure("Calculation error".to_string());

    println!("\nIs success?: {}", success.is_success());
    println!("Is failure?: {}", failure.is_failure());

    if let Some(value) = success.get_value() {
        println!("Success value: {value}");
    }

    // Ejercicio 5: Stack genérico
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("\nStack has {} elements", stack.len());
    println!("Top: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Stack has {} elements", stack.len());

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Pair Genérico
// ============================================
// Implementa un struct que almacena dos valores del mismo tipo.
//
// Métodos requeridos:
// - new(first, second) -> Pair<T>
// - first(&self) -> &T
// - second(&self) -> &T
// - swap(self) -> Pair<T>

struct Pair<T> {
    // TODO: Define los campos
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Pair<T> {
    fn new(_first: T, _second: T) -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa Pair::new")
    }

    fn first(&self) -> &T {
        // TODO: Devuelve referencia al primer elemento
        todo!("Implementa Pair::first")
    }

    fn second(&self) -> &T {
        // TODO: Devuelve referencia al segundo elemento
        todo!("Implementa Pair::second")
    }

    fn swap(self) -> Pair<T> {
        // TODO: Devuelve un nuevo Pair con los elementos intercambiados
        todo!("Implementa Pair::swap")
    }
}

// ============================================
// EJERCICIO 2: Wrapper Genérico
// ============================================
// Implementa un "wrapper" que envuelve cualquier valor.
// Similar a Box pero sin heap allocation.
//
// Métodos requeridos:
// - new(value) -> Wrapper<T>
// - value(&self) -> &T
// - unwrap_value(self) -> T
// - map<U, F>(self, f: F) -> Wrapper<U> where F: FnOnce(T) -> U

struct Wrapper<T> {
    // TODO: Define el campo
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Wrapper<T> {
    fn new(_value: T) -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa Wrapper::new")
    }

    fn value(&self) -> &T {
        // TODO: Devuelve referencia al valor
        todo!("Implementa Wrapper::value")
    }

    fn unwrap_value(self) -> T {
        // TODO: Consume el wrapper y devuelve el valor
        todo!("Implementa Wrapper::unwrap_value")
    }

    fn map<U, F>(self, _f: F) -> Wrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        // TODO: Aplica la función al valor y devuelve nuevo Wrapper
        todo!("Implementa Wrapper::map")
    }
}

// ============================================
// EJERCICIO 3: Point Genérico
// ============================================
// Implementa un punto 2D genérico con métodos especializados.
//
// Para cualquier T:
// - new(x, y) -> Point<T>
// - x(&self) -> &T
// - y(&self) -> &T
//
// Solo para f64:
// - distance_to_origin(&self) -> f64

struct Point<T> {
    // TODO: Define los campos x, y
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Point<T> {
    fn new(_x: T, _y: T) -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa Point::new")
    }

    fn x(&self) -> &T {
        // TODO: Devuelve referencia a x
        todo!("Implementa Point::x")
    }

    fn y(&self) -> &T {
        // TODO: Devuelve referencia a y
        todo!("Implementa Point::y")
    }
}

// Implementación especializada solo para Point<f64>
impl Point<f64> {
    fn distance_to_origin(&self) -> f64 {
        // TODO: Calcula sqrt(x² + y²)
        // Usa: (self.x.powi(2) + self.y.powi(2)).sqrt()
        todo!("Implementa Point::distance_to_origin")
    }
}

// ============================================
// EJERCICIO 4: SimpleResult Simplificado
// ============================================
// Implementa un enum similar a Result pero simplificado.
//
// Variantes:
// - Success(T)
// - Failure(E)
//
// Métodos:
// - success(value) -> SimpleResult<T, E>
// - failure(error) -> SimpleResult<T, E>
// - is_success(&self) -> bool
// - is_failure(&self) -> bool
// - get_value(self) -> Option<T>

enum SimpleResult<T, E> {
    // TODO: Define las variantes
    _Temporal(std::marker::PhantomData<(T, E)>), // Elimina esto al implementar
}

impl<T, E> SimpleResult<T, E> {
    fn success(_value: T) -> Self {
        // TODO: Crea un SimpleResult exitoso
        todo!("Implementa SimpleResult::success")
    }

    fn failure(_error: E) -> Self {
        // TODO: Crea un SimpleResult fallido
        todo!("Implementa SimpleResult::failure")
    }

    fn is_success(&self) -> bool {
        // TODO: Devuelve true si es Success
        todo!("Implementa SimpleResult::is_success")
    }

    fn is_failure(&self) -> bool {
        // TODO: Devuelve true si es Failure
        todo!("Implementa SimpleResult::is_failure")
    }

    fn get_value(self) -> Option<T> {
        // TODO: Devuelve Some(valor) si es Success, None si es Failure
        todo!("Implementa SimpleResult::get_value")
    }
}

// ============================================
// EJERCICIO 5: Stack Genérico
// ============================================
// Implementa una pila (stack) genérica usando Vec.
//
// Métodos:
// - new() -> Stack<T>
// - push(&mut self, value: T)
// - pop(&mut self) -> Option<T>
// - peek(&self) -> Option<&T>
// - len(&self) -> usize
// - is_empty(&self) -> bool

struct Stack<T> {
    // TODO: Define el campo (usa Vec<T>)
    _marker: std::marker::PhantomData<T>, // Elimina esto al implementar
}

impl<T> Stack<T> {
    fn new() -> Self {
        // TODO: Crea un stack vacío
        todo!("Implementa Stack::new")
    }

    fn push(&mut self, _value: T) {
        // TODO: Agrega un elemento al tope
        todo!("Implementa Stack::push")
    }

    fn pop(&mut self) -> Option<T> {
        // TODO: Remueve y devuelve el elemento del tope
        todo!("Implementa Stack::pop")
    }

    fn peek(&self) -> Option<&T> {
        // TODO: Devuelve referencia al elemento del tope sin removerlo
        todo!("Implementa Stack::peek")
    }

    fn len(&self) -> usize {
        // TODO: Devuelve la cantidad de elementos
        todo!("Implementa Stack::len")
    }

    fn is_empty(&self) -> bool {
        // TODO: Devuelve true si el stack está vacío
        todo!("Implementa Stack::is_empty")
    }
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Tests para Pair
    #[test]
    fn test_pair_new() {
        let pair = Pair::new(1, 2);
        assert_eq!(*pair.first(), 1);
        assert_eq!(*pair.second(), 2);
    }

    #[test]
    fn test_pair_swap() {
        let pair = Pair::new("a", "b");
        let swapped = pair.swap();
        assert_eq!(*swapped.first(), "b");
        assert_eq!(*swapped.second(), "a");
    }

    // Tests para Wrapper
    #[test]
    fn test_wrapper_new_and_value() {
        let wrapper = Wrapper::new(42);
        assert_eq!(*wrapper.value(), 42);
    }

    #[test]
    fn test_wrapper_unwrap_value() {
        let wrapper = Wrapper::new(String::from("hola"));
        let value = wrapper.unwrap_value();
        assert_eq!(value, "hola");
    }

    #[test]
    fn test_wrapper_map() {
        let wrapper = Wrapper::new(10);
        let mapped_wrapper = wrapper.map(|x| x * 3);
        assert_eq!(*mapped_wrapper.value(), 30);
    }

    #[test]
    fn test_wrapper_map_different_type() {
        let wrapper = Wrapper::new(42);
        let string_wrapper = wrapper.map(|x| x.to_string());
        assert_eq!(*string_wrapper.value(), "42");
    }

    // Tests para Point
    #[test]
    fn test_point_new() {
        let point = Point::new(3, 4);
        assert_eq!(*point.x(), 3);
        assert_eq!(*point.y(), 4);
    }

    #[test]
    fn test_point_distance_to_origin() {
        let point = Point::new(3.0, 4.0);
        assert!((point.distance_to_origin() - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_point_distance_to_origin_zero() {
        let point = Point::new(0.0, 0.0);
        assert!((point.distance_to_origin() - 0.0).abs() < 0.0001);
    }

    // Tests para SimpleResult
    #[test]
    fn test_simple_result_success() {
        let result: SimpleResult<i32, &str> = SimpleResult::success(42);
        assert!(result.is_success());
        assert!(!result.is_failure());
    }

    #[test]
    fn test_simple_result_failure() {
        let result: SimpleResult<i32, &str> = SimpleResult::failure("error");
        assert!(!result.is_success());
        assert!(result.is_failure());
    }

    #[test]
    fn test_simple_result_get_value_success() {
        let result: SimpleResult<i32, &str> = SimpleResult::success(100);
        assert_eq!(result.get_value(), Some(100));
    }

    #[test]
    fn test_simple_result_get_value_failure() {
        let result: SimpleResult<i32, &str> = SimpleResult::failure("error");
        assert_eq!(result.get_value(), None);
    }

    // Tests para Stack
    #[test]
    fn test_stack_new() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1); // peek no remueve
    }

    #[test]
    fn test_stack_len() {
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);

        stack.push("a");
        stack.push("b");
        assert_eq!(stack.len(), 2);

        stack.pop();
        assert_eq!(stack.len(), 1);
    }
}
