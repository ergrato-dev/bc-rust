// ============================================
// SOLUCIONES - Práctica 04: Genéricos Avanzados
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

use std::marker::PhantomData;

// ============================================
// Ejercicio 1: Trait con Tipo Asociado
// ============================================
trait Iterador {
    type Item;
    fn siguiente(&self) -> Option<Self::Item>;
}

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
    type Item = i32;

    fn siguiente(&self) -> Option<Self::Item> {
        let current = self.actual.get();
        if current < self.fin {
            self.actual.set(current + 1);
            Some(current)
        } else {
            None
        }
    }
}

// ============================================
// Ejercicio 2: Const Generics
// ============================================
struct Buffer<T, const N: usize> {
    datos: [T; N],
}

impl<T, const N: usize> Buffer<T, N> {
    fn new(datos: [T; N]) -> Self {
        Buffer { datos }
    }

    fn capacidad(&self) -> usize {
        N
    }

    fn obtener(&self, indice: usize) -> Option<&T> {
        if indice < N {
            Some(&self.datos[indice])
        } else {
            None
        }
    }
}

impl<T: Copy, const N: usize> Buffer<T, N> {
    fn obtener_copia(&self, indice: usize) -> Option<T> {
        if indice < N {
            Some(self.datos[indice])
        } else {
            None
        }
    }
}

// ============================================
// Ejercicio 3: Type State Pattern
// ============================================
struct Pendiente;
struct Pagado;
struct Enviado;
struct Entregado;

struct Pedido<Estado> {
    descripcion: String,
    _estado: PhantomData<Estado>,
}

impl<Estado> Pedido<Estado> {
    fn descripcion(&self) -> &str {
        &self.descripcion
    }
}

impl Pedido<Pendiente> {
    fn nuevo(descripcion: &str) -> Self {
        Pedido {
            descripcion: descripcion.to_string(),
            _estado: PhantomData,
        }
    }

    fn pagar(self) -> Pedido<Pagado> {
        Pedido {
            descripcion: self.descripcion,
            _estado: PhantomData,
        }
    }
}

impl Pedido<Pagado> {
    fn enviar(self) -> Pedido<Enviado> {
        Pedido {
            descripcion: self.descripcion,
            _estado: PhantomData,
        }
    }
}

impl Pedido<Enviado> {
    fn entregar(self) -> Pedido<Entregado> {
        Pedido {
            descripcion: self.descripcion,
            _estado: PhantomData,
        }
    }
}

// ============================================
// Ejercicio 4: PhantomData para IDs Tipados
// ============================================
struct Usuario;
struct Producto;

struct Id<T> {
    valor: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(valor: u64) -> Self {
        Id {
            valor,
            _marker: PhantomData,
        }
    }

    fn valor(&self) -> u64 {
        self.valor
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.valor == other.valor
    }
}
