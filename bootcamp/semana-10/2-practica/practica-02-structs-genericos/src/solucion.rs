// ============================================
// SOLUCIONES - Práctica 02: Structs y Enums Genéricos
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

// ============================================
// Ejercicio 1: Par Genérico
// ============================================
struct Par<T> {
    primero: T,
    segundo: T,
}

impl<T> Par<T> {
    fn new(primero: T, segundo: T) -> Self {
        Par { primero, segundo }
    }

    fn primero(&self) -> &T {
        &self.primero
    }

    fn segundo(&self) -> &T {
        &self.segundo
    }

    fn invertir(self) -> Par<T> {
        Par {
            primero: self.segundo,
            segundo: self.primero,
        }
    }
}

// ============================================
// Ejercicio 2: Caja Genérica
// ============================================
struct Caja<T> {
    contenido: T,
}

impl<T> Caja<T> {
    fn new(valor: T) -> Self {
        Caja { contenido: valor }
    }

    fn valor(&self) -> &T {
        &self.contenido
    }

    fn desenvolver(self) -> T {
        self.contenido
    }

    fn map<U, F>(self, f: F) -> Caja<U>
    where
        F: FnOnce(T) -> U,
    {
        Caja {
            contenido: f(self.contenido),
        }
    }
}

// ============================================
// Ejercicio 3: Punto Genérico
// ============================================
struct Punto<T> {
    x: T,
    y: T,
}

impl<T> Punto<T> {
    fn new(x: T, y: T) -> Self {
        Punto { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Punto<f64> {
    fn distancia_origen(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ============================================
// Ejercicio 4: Resultado Simplificado
// ============================================
enum Resultado<T, E> {
    Exito(T),
    Fallo(E),
}

impl<T, E> Resultado<T, E> {
    fn exito(valor: T) -> Self {
        Resultado::Exito(valor)
    }

    fn fallo(error: E) -> Self {
        Resultado::Fallo(error)
    }

    fn es_exito(&self) -> bool {
        matches!(self, Resultado::Exito(_))
    }

    fn es_fallo(&self) -> bool {
        matches!(self, Resultado::Fallo(_))
    }

    fn obtener_valor(self) -> Option<T> {
        match self {
            Resultado::Exito(valor) => Some(valor),
            Resultado::Fallo(_) => None,
        }
    }
}

// ============================================
// Ejercicio 5: Pila Genérica
// ============================================
struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    fn push(&mut self, valor: T) {
        self.elementos.push(valor);
    }

    fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }

    fn len(&self) -> usize {
        self.elementos.len()
    }

    fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }
}
