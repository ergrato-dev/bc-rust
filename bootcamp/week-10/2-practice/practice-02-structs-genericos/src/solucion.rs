// ============================================
// SOLUCIONES - Práctica 02: Structs y Enums Genéricos
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

// ============================================
// Ejercicio 1: Pair Genérico
// ============================================
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn first(&self) -> &T {
        &self.first
    }

    fn second(&self) -> &T {
        &self.second
    }

    fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

// ============================================
// Ejercicio 2: Wrapper Genérico
// ============================================
struct Wrapper<T> {
    content: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { content: value }
    }

    fn value(&self) -> &T {
        &self.content
    }

    fn unwrap_value(self) -> T {
        self.content
    }

    fn map<U, F>(self, f: F) -> Wrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        Wrapper {
            content: f(self.content),
        }
    }
}

// ============================================
// Ejercicio 3: Point Genérico
// ============================================
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ============================================
// Ejercicio 4: SimpleResult Simplificado
// ============================================
enum SimpleResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> SimpleResult<T, E> {
    fn success(value: T) -> Self {
        SimpleResult::Success(value)
    }

    fn failure(error: E) -> Self {
        SimpleResult::Failure(error)
    }

    fn is_success(&self) -> bool {
        matches!(self, SimpleResult::Success(_))
    }

    fn is_failure(&self) -> bool {
        matches!(self, SimpleResult::Failure(_))
    }

    fn get_value(self) -> Option<T> {
        match self {
            SimpleResult::Success(value) => Some(value),
            SimpleResult::Failure(_) => None,
        }
    }
}

// ============================================
// Ejercicio 5: Stack Genérico
// ============================================
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
