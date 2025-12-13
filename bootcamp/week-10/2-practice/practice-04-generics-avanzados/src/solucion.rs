// ============================================
// SOLUCIONES - Práctica 04: Genéricos Avanzados
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

use std::marker::PhantomData;

// ============================================
// Ejercicio 1: Trait con Tipo Asociado
// ============================================
trait SimpleIterator {
    type Item;
    fn next(&self) -> Option<Self::Item>;
}

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
    type Item = i32;

    fn next(&self) -> Option<Self::Item> {
        let curr = self.current.get();
        if curr < self.end {
            self.current.set(curr + 1);
            Some(curr)
        } else {
            None
        }
    }
}

// ============================================
// Ejercicio 2: Const Generics
// ============================================
struct Buffer<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Buffer<T, N> {
    fn new(data: [T; N]) -> Self {
        Buffer { data }
    }

    fn capacity(&self) -> usize {
        N
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self.data[index])
        } else {
            None
        }
    }
}

impl<T: Copy, const N: usize> Buffer<T, N> {
    fn get_copy(&self, index: usize) -> Option<T> {
        if index < N {
            Some(self.data[index])
        } else {
            None
        }
    }
}

// ============================================
// Ejercicio 3: Type State Pattern
// ============================================
struct Pending;
struct Paid;
struct Shipped;
struct Delivered;

struct Order<State> {
    description: String,
    _state: PhantomData<State>,
}

impl<State> Order<State> {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Order<Pending> {
    fn new(description: &str) -> Self {
        Order {
            description: description.to_string(),
            _state: PhantomData,
        }
    }

    fn pay(self) -> Order<Paid> {
        Order {
            description: self.description,
            _state: PhantomData,
        }
    }
}

impl Order<Paid> {
    fn ship(self) -> Order<Shipped> {
        Order {
            description: self.description,
            _state: PhantomData,
        }
    }
}

impl Order<Shipped> {
    fn deliver(self) -> Order<Delivered> {
        Order {
            description: self.description,
            _state: PhantomData,
        }
    }
}

// ============================================
// Ejercicio 4: PhantomData para IDs Tipados
// ============================================
struct User;
struct Product;

struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        Id {
            value,
            _marker: PhantomData,
        }
    }

    fn value(&self) -> u64 {
        self.value
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
