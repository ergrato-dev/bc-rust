// ============================================
// Práctica 01: Declaración de Variables
// ============================================
// Objetivo: Practicar let vs let mut
// ============================================

fn main() {
    println!("=== Práctica 01: Variables ===\n");

    // -----------------------------------------
    // PARTE 1: Variables Inmutables
    // -----------------------------------------
    // TODO: Declara las siguientes variables inmutables:
    
    // 1. Una variable 'nombre' con tu nombre (tipo &str)
    // let nombre = ...;
    
    // 2. Una variable 'edad' con tu edad (tipo i32)
    // let edad = ...;
    
    // 3. Una variable 'es_estudiante' con true o false
    // let es_estudiante = ...;
    
    // TODO: Imprime las variables
    // println!("Nombre: {}", nombre);
    // println!("Edad: {}", edad);
    // println!("Es estudiante: {}", es_estudiante);

    // -----------------------------------------
    // PARTE 2: Variables Mutables
    // -----------------------------------------
    println!("\n--- Variables Mutables ---");
    
    // TODO: Declara un contador mutable iniciando en 0
    // let mut contador = ...;
    
    // TODO: Incrementa el contador 3 veces
    // contador = contador + 1;  // o contador += 1;
    // ...
    
    // TODO: Imprime el valor final
    // println!("Contador final: {}", contador);

    // -----------------------------------------
    // PARTE 3: Experimenta con Errores
    // -----------------------------------------
    // Descomenta el siguiente código y observa el error:
    
    // let inmutable = 5;
    // inmutable = 10;  // Error E0384!
    
    // Ahora corrígelo añadiendo 'mut':
    // let mut mutable = 5;
    // mutable = 10;  // ¡Ahora funciona!

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inmutabilidad() {
        let x = 5;
        // x no puede cambiar
        assert_eq!(x, 5);
    }

    #[test]
    fn test_mutabilidad() {
        let mut y = 0;
        y += 1;
        y += 1;
        y += 1;
        assert_eq!(y, 3);
    }

    #[test]
    fn test_tipos_basicos() {
        let nombre: &str = "Rust";
        let edad: i32 = 10;
        let activo: bool = true;
        
        assert_eq!(nombre, "Rust");
        assert_eq!(edad, 10);
        assert!(activo);
    }
}
