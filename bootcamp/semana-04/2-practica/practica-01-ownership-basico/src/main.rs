// Práctica 01: Ownership Básico
// ================================
// Completa los ejercicios siguiendo las instrucciones en README.md

fn main() {
    println!("=== Práctica 01: Ownership Básico ===\n");
    
    ejercicio1();
    ejercicio2();
    ejercicio3();
    ejercicio4();
}

// Ejercicio 1: Corrige usando clone()
fn ejercicio1() {
    println!("--- Ejercicio 1: Clone ---");
    
    let mensaje = String::from("Hola, Rust!");
    // TODO: Corrige esta línea para que mensaje siga siendo válido
    let copia = mensaje; // <- Modifica aquí
    
    // Descomenta cuando hayas corregido:
    // println!("Original: {}", mensaje);
    // println!("Copia: {}", copia);
    
    println!();
}

// Ejercicio 2: Corrige la función para no tomar ownership
fn ejercicio2() {
    println!("--- Ejercicio 2: Funciones ---");
    
    let nombre = String::from("Ferris");
    imprimir_nombre(nombre);
    
    // Descomenta cuando hayas corregido:
    // println!("Nombre después: {}", nombre);
    
    println!();
}

// TODO: Modifica esta función para que no tome ownership
fn imprimir_nombre(n: String) {
    println!("Imprimiendo: {}", n);
}

// Ejercicio 3: Identifica la variable válida
fn ejercicio3() {
    println!("--- Ejercicio 3: Cadena de Moves ---");
    
    let a = String::from("Rust");
    let b = a;
    let c = b;
    let d = c;
    
    // TODO: Descomenta SOLO la línea que compile
    // println!("a = {}", a);
    // println!("b = {}", b);
    // println!("c = {}", c);
    // println!("d = {}", d);
    
    println!();
}

// Ejercicio 4: Observa el orden de Drop
fn ejercicio4() {
    println!("--- Ejercicio 4: Scope y Drop ---");
    
    struct Recurso {
        nombre: String,
    }

    impl Drop for Recurso {
        fn drop(&mut self) {
            println!("  Drop de {}", self.nombre);
        }
    }

    println!("  Inicio");
    
    let _r1 = Recurso { nombre: String::from("R1") };
    println!("  Creando R1");
    
    {
        let _r2 = Recurso { nombre: String::from("R2") };
        println!("  Creando R2");
    } // ¿Cuándo se llama drop de R2?
    
    let _r3 = Recurso { nombre: String::from("R3") };
    println!("  Creando R3");
    
    println!("  Fin");
    // ¿En qué orden se llama drop de R1 y R3?
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clone_preserva_original() {
        let original = String::from("test");
        let copia = original.clone();
        
        assert_eq!(original, "test");
        assert_eq!(copia, "test");
    }
    
    #[test]
    fn test_referencia_no_mueve() {
        let s = String::from("hola");
        let r = &s;
        
        assert_eq!(s, "hola");
        assert_eq!(*r, "hola");
    }
}
