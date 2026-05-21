use std::alloc::{self, Layout};
use std::ptr::NonNull;

/// Vector mínimo con gestión manual de memoria.
///
/// Demuestra cómo `std::vec::Vec` gestiona memoria internamente
/// usando el allocator global de Rust.
pub struct RawVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> RawVec<T> {
    /// Crea un `RawVec` vacío sin reservar memoria.
    pub fn new() -> Self {
        // Una capacidad 0 no hace alloc; usamos dangling para evitar null.
        RawVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Reserva espacio para al menos un elemento adicional.
    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let layout = Layout::array::<T>(new_cap).expect("layout overflow");

        let new_ptr = if self.cap == 0 {
            // SAFETY: `layout` tiene tamaño > 0 (new_cap >= 4, size_of::<T>() >= 1).
            unsafe { alloc::alloc(layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).expect("layout overflow");
            // SAFETY: `self.ptr` fue obtenido del allocator global con `old_layout`.
            // `new_cap > self.cap` garantiza que el layout es estrictamente mayor.
            unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, layout.size()) }
        };

        self.ptr = NonNull::new(new_ptr as *mut T).expect("allocation failed");
        self.cap = new_cap;
    }

    /// Agrega un elemento al final.
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }
        // SAFETY: `self.len < self.cap`, por lo que `self.ptr.add(self.len)`
        // apunta a memoria válida y no inicializada que nos pertenece.
        unsafe {
            self.ptr.as_ptr().add(self.len).write(value);
        }
        self.len += 1;
    }

    /// Extrae el último elemento.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: `self.len` era > 0; el elemento en `self.len` (post-decremento)
        // fue inicializado por `push` y aún no ha sido leído.
        Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
    }

    /// Devuelve referencia al elemento en `index`.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        // SAFETY: `index < self.len` garantiza que el elemento fue inicializado.
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }
}

impl<T> Default for RawVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap == 0 {
            return;
        }
        // Llamar a `drop` en cada elemento inicializado.
        for i in 0..self.len {
            // SAFETY: índices 0..len fueron inicializados con `push`.
            unsafe { self.ptr.as_ptr().add(i).drop_in_place(); }
        }
        let layout = Layout::array::<T>(self.cap).expect("layout overflow");
        // SAFETY: `self.ptr` fue obtenido del allocator global con este mismo layout.
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout); }
    }
}

fn main() {
    let mut v: RawVec<String> = RawVec::new();
    v.push("hola".to_string());
    v.push("mundo".to_string());
    println!("len={}, cap={}", v.len(), v.capacity());
    while let Some(s) = v.pop() {
        println!("popped: {s}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_basico() {
        let mut v: RawVec<i32> = RawVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn grow_automatico() {
        let mut v: RawVec<i32> = RawVec::new();
        for i in 0..20 {
            v.push(i);
        }
        assert_eq!(v.len(), 20);
        assert!(v.capacity() >= 20);
    }

    #[test]
    fn get_fuera_de_rango() {
        let mut v: RawVec<i32> = RawVec::new();
        v.push(42);
        assert_eq!(v.get(0), Some(&42));
        assert_eq!(v.get(1), None);
    }

    #[test]
    fn drop_no_panic() {
        let mut v: RawVec<String> = RawVec::new();
        v.push("test".to_string());
        // Drop implícito al salir del scope — no debe paniquear
    }

    #[test]
    fn empty_vec() {
        let v: RawVec<i32> = RawVec::new();
        assert!(v.is_empty());
        assert_eq!(v.capacity(), 0);
    }
}
