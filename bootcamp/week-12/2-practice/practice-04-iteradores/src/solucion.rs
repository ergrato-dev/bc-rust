//! Soluciones - Práctica 04: Iteradores

fn main() {
    println!("=== Soluciones: Iteradores ===\n");

    // Ejercicio 1
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Cuadrados de pares: {:?}", cuadrados_pares(&numbers));

    // Ejercicio 2
    let words = vec!["Hola", " ", "Rust", "!"];
    println!("Concatenado: {}", concatenar(&words));

    // Ejercicio 3
    let data = vec![10, 25, 30, 45, 50];
    if let Some((pos, val)) = primer_mayor_que(&data, 28) {
        println!("Primer mayor que 28: {} en pos {}", val, pos);
    }

    // Ejercicio 4
    let names = vec!["Ana", "Bob", "Carlos"];
    let ages = vec![25, 30, 35];
    println!("Personas: {:?}", combinar_datos(&names, &ages));

    // Ejercicio 5
    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fibonacci: {:?}", fib);

    println!("\n✅ Todas las soluciones funcionan!");
}

// SOLUCIÓN Ejercicio 1
fn cuadrados_pares(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect()
}

// SOLUCIÓN Ejercicio 2
fn concatenar(palabras: &[&str]) -> String {
    palabras.iter().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc
    })
}

// Alternativa más idiomática:
// palabras.iter().copied().collect()
// o palabras.join("")

// SOLUCIÓN Ejercicio 3
fn primer_mayor_que(datos: &[i32], umbral: i32) -> Option<(usize, i32)> {
    datos
        .iter()
        .enumerate()
        .find(|(_, &val)| val > umbral)
        .map(|(pos, &val)| (pos, val))
}

// SOLUCIÓN Ejercicio 4
fn combinar_datos<'a>(names: &'a [&str], ages: &[i32]) -> Vec<(&'a str, i32)> {
    names
        .iter()
        .zip(ages.iter())
        .map(|(&name, &age)| (name, age))
        .collect()
}

// SOLUCIÓN Ejercicio 5
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.current;
        let new_next = self.current + self.next;
        
        self.current = self.next;
        self.next = new_next;
        
        Some(current_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_solutions() {
        // Cuadrados pares
        assert_eq!(cuadrados_pares(&[1, 2, 3, 4]), vec![4, 16]);

        // Concatenar
        assert_eq!(concatenar(&["a", "b"]), "ab");

        // Primer mayor
        assert_eq!(primer_mayor_que(&[1, 5, 10], 4), Some((1, 5)));

        // Combinar
        assert_eq!(combinar_datos(&["A"], &[1]), vec![("A", 1)]);

        // Fibonacci
        let fib: Vec<u64> = Fibonacci::new().take(5).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3]);
    }
}
