use wasm_bindgen::prelude::*;

/// Contador con estado expuesto a JavaScript.
///
/// Accesible desde JS como:
/// ```javascript
/// const c = new Contador(0);
/// c.incrementar();
/// console.log(c.valor()); // 1
/// ```
#[wasm_bindgen]
pub struct Contador {
    valor: i32,
}

#[wasm_bindgen]
impl Contador {
    /// Crea un contador con valor inicial.
    #[wasm_bindgen(constructor)]
    pub fn new(inicial: i32) -> Contador {
        Contador { valor: inicial }
    }

    /// Retorna el valor actual.
    pub fn valor(&self) -> i32 {
        self.valor
    }

    /// Incrementa el contador en 1.
    pub fn incrementar(&mut self) {
        self.valor += 1;
    }

    /// Decrementa el contador en 1.
    pub fn decrementar(&mut self) {
        self.valor -= 1;
    }

    /// Resetea el contador a 0.
    pub fn resetear(&mut self) {
        self.valor = 0;
    }

    /// Suma un valor al contador.
    pub fn sumar(&mut self, n: i32) {
        self.valor += n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contador_incrementa() {
        let mut c = Contador::new(0);
        c.incrementar();
        c.incrementar();
        assert_eq!(c.valor(), 2);
    }

    #[test]
    fn contador_resetea() {
        let mut c = Contador::new(10);
        c.resetear();
        assert_eq!(c.valor(), 0);
    }

    #[test]
    fn contador_suma() {
        let mut c = Contador::new(5);
        c.sumar(3);
        assert_eq!(c.valor(), 8);
    }
}
