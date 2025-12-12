// Práctica 04: Ownership en Funciones
// ====================================

fn main() {
    println!("=== Práctica 04: Ownership en Funciones ===\n");
    
    ejercicio2();
    ejercicio3();
    ejercicio4();
}

// ============================================
// Ejercicio 2: Implementar Funciones
// ============================================

// a) Contar caracteres - solo necesita leer
fn contar_caracteres(s: &str) -> usize {
    s.chars().count()
}

// b) Convertir a mayúsculas in-place
fn a_mayusculas(s: &mut String) {
    *s = s.to_uppercase();
}

// c) Crear saludo - retorna nuevo String
fn crear_saludo(nombre: &str) -> String {
    format!("¡Hola, {}!", nombre)
}

// d) Tomar primer elemento - consume el vector
fn tomar_primero<T>(mut vec: Vec<T>) -> Option<T> {
    if vec.is_empty() {
        None
    } else {
        Some(vec.remove(0))
    }
}

fn ejercicio2() {
    println!("--- Ejercicio 2: Implementar Funciones ---");
    
    // a) Contar caracteres
    let texto = String::from("Rust 🦀");
    let chars = contar_caracteres(&texto);
    println!("  '{}' tiene {} caracteres", texto, chars);
    
    // b) A mayúsculas
    let mut mensaje = String::from("hola mundo");
    a_mayusculas(&mut mensaje);
    println!("  Mayúsculas: {}", mensaje);
    
    // c) Crear saludo
    let saludo = crear_saludo("Ferris");
    println!("  {}", saludo);
    
    // d) Tomar primero
    let numeros = vec![10, 20, 30];
    if let Some(primero) = tomar_primero(numeros) {
        println!("  Primer elemento: {}", primero);
    }
    // numeros ya no es válido aquí (fue consumido)
    
    println!();
}

// ============================================
// Ejercicio 3: Refactorizar (sin clones innecesarios)
// ============================================

// ANTES (con clones innecesarios):
// fn procesar_datos(datos: Vec<i32>) -> i32 {
//     let copia = datos.clone();
//     let suma: i32 = copia.iter().sum();
//     let max = copia.iter().max().unwrap_or(&0);
//     suma + max
// }

// DESPUÉS (usando referencia):
fn procesar_datos(datos: &[i32]) -> i32 {
    let suma: i32 = datos.iter().sum();
    let max = *datos.iter().max().unwrap_or(&0);
    suma + max
}

fn ejercicio3() {
    println!("--- Ejercicio 3: Refactorizar ---");
    
    let mis_datos = vec![1, 2, 3, 4, 5];
    
    // Ahora solo pasamos referencia, sin clonar
    let resultado = procesar_datos(&mis_datos);
    
    println!("  Datos: {:?}", mis_datos);  // ✅ sigue válido
    println!("  Resultado: {} (suma + max)", resultado);
    println!();
}

// ============================================
// Ejercicio 4: API de Struct
// ============================================

struct Inventario {
    items: Vec<String>,
}

impl Inventario {
    // Constructor - no necesita self
    fn new() -> Self {
        Inventario { items: Vec::new() }
    }
    
    // Agregar - necesita modificar → &mut self
    fn agregar(&mut self, item: &str) {
        self.items.push(item.to_string());
    }
    
    // Listar - solo lectura → &self
    fn listar(&self) -> &[String] {
        &self.items
    }
    
    // Contiene - solo lectura → &self
    fn contiene(&self, item: &str) -> bool {
        self.items.iter().any(|i| i == item)
    }
    
    // Quitar - necesita modificar → &mut self
    fn quitar(&mut self, item: &str) -> Option<String> {
        if let Some(pos) = self.items.iter().position(|i| i == item) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
    
    // Cantidad - solo lectura → &self
    fn cantidad(&self) -> usize {
        self.items.len()
    }
}

fn ejercicio4() {
    println!("--- Ejercicio 4: API de Struct ---");
    
    let mut inv = Inventario::new();
    
    // Agregar items
    inv.agregar("Espada");
    inv.agregar("Escudo");
    inv.agregar("Poción");
    
    println!("  Items: {:?}", inv.listar());
    println!("  Cantidad: {}", inv.cantidad());
    
    // Buscar
    println!("  ¿Tiene Espada? {}", inv.contiene("Espada"));
    println!("  ¿Tiene Arco? {}", inv.contiene("Arco"));
    
    // Quitar
    if let Some(item) = inv.quitar("Escudo") {
        println!("  Quitado: {}", item);
    }
    
    println!("  Items finales: {:?}", inv.listar());
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contar_caracteres() {
        assert_eq!(contar_caracteres("hola"), 4);
        assert_eq!(contar_caracteres("🦀"), 1);
    }
    
    #[test]
    fn test_a_mayusculas() {
        let mut s = String::from("rust");
        a_mayusculas(&mut s);
        assert_eq!(s, "RUST");
    }
    
    #[test]
    fn test_crear_saludo() {
        let saludo = crear_saludo("Mundo");
        assert_eq!(saludo, "¡Hola, Mundo!");
    }
    
    #[test]
    fn test_tomar_primero() {
        let vec = vec![1, 2, 3];
        assert_eq!(tomar_primero(vec), Some(1));
        
        let vacio: Vec<i32> = vec![];
        assert_eq!(tomar_primero(vacio), None);
    }
    
    #[test]
    fn test_inventario() {
        let mut inv = Inventario::new();
        
        inv.agregar("Item1");
        assert!(inv.contiene("Item1"));
        assert_eq!(inv.cantidad(), 1);
        
        let quitado = inv.quitar("Item1");
        assert_eq!(quitado, Some(String::from("Item1")));
        assert_eq!(inv.cantidad(), 0);
    }
    
    #[test]
    fn test_procesar_datos_no_consume() {
        let datos = vec![1, 2, 3, 4, 5];
        let _ = procesar_datos(&datos);
        
        // datos sigue siendo válido
        assert_eq!(datos.len(), 5);
    }
}
