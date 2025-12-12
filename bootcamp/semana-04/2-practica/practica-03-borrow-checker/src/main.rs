// Práctica 03: Borrow Checker
// ============================
// Aprende a resolver errores del borrow checker

fn main() {
    println!("=== Práctica 03: Borrow Checker ===\n");
    
    ejercicio1_solucion_a();
    ejercicio1_solucion_b();
    ejercicio2();
    ejercicio3();
    ejercicio4();
}

// Ejercicio 1 - Solución A: Reorganizar código
fn ejercicio1_solucion_a() {
    println!("--- Ejercicio 1A: Reorganizar ---");
    
    let mut numeros = vec![1, 2, 3, 4, 5];
    
    // TODO: Reorganiza para que funcione
    // Pista: usa primero ANTES de modificar
    
    let primero = &numeros[0];
    println!("  Primero: {}", primero);
    // numeros.push(6); // <- ¿Dónde debería ir?
    
    println!("  Vector: {:?}", numeros);
    println!();
}

// Ejercicio 1 - Solución B: Clonar el valor
fn ejercicio1_solucion_b() {
    println!("--- Ejercicio 1B: Clonar ---");
    
    let mut numeros = vec![1, 2, 3, 4, 5];
    
    // TODO: Clona el valor para no depender de la referencia
    let primero = numeros[0]; // <- i32 es Copy, ¡no necesita clone!
    
    numeros.push(6);
    
    println!("  Primero (copiado): {}", primero);
    println!("  Vector: {:?}", numeros);
    println!();
}

// Ejercicio 2: Préstamos en Conflicto
fn ejercicio2() {
    println!("--- Ejercicio 2: Préstamos en Conflicto ---");
    
    let mut texto = String::from("Hola");
    
    // TODO: Reorganiza este código para que compile
    // El objetivo es imprimir el texto original Y luego modificarlo
    
    // Versión con error:
    // let r1 = &texto;
    // let r2 = &texto;
    // let r3 = &mut texto;  // ERROR: conflicto
    // println!("{}, {}", r1, r2);
    // r3.push_str(" mundo");
    
    // Versión corregida:
    {
        let r1 = &texto;
        let r2 = &texto;
        println!("  Refs inmutables: {}, {}", r1, r2);
    } // r1 y r2 terminan aquí
    
    {
        let r3 = &mut texto;
        r3.push_str(" mundo");
        println!("  Ref mutable: {}", r3);
    }
    
    println!();
}

// Ejercicio 3: Retornar ownership en lugar de referencia
// Esta función NO puede retornar &String a una variable local
fn crear_mensaje() -> String {
    let s = String::from("Hola desde la función");
    s  // Retornar ownership, no referencia
}

fn ejercicio3() {
    println!("--- Ejercicio 3: Retornar Ownership ---");
    
    let mensaje = crear_mensaje();
    println!("  Mensaje: {}", mensaje);
    println!();
}

// Ejercicio 4: Análisis de Lifetimes (NLL)
fn ejercicio4() {
    println!("--- Ejercicio 4: Non-Lexical Lifetimes ---");
    
    let mut s = String::from("hola");
    
    let r1 = &s;           // Préstamo inmutable inicia
    println!("  r1: {}", r1);  // Último uso de r1 → préstamo TERMINA aquí
    
    let r2 = &s;           // Nuevo préstamo inmutable
    let r3 = &s;           // Otro préstamo inmutable
    println!("  r2: {}, r3: {}", r2, r3);  // Último uso de r2 y r3
    
    // Aquí r1, r2, r3 ya no existen (NLL)
    
    let r4 = &mut s;       // ✅ OK: no hay refs inmutables activas
    r4.push_str("!");
    println!("  r4: {}", r4);
    
    println!("  Final: {}", s);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crear_mensaje() {
        let msg = crear_mensaje();
        assert_eq!(msg, "Hola desde la función");
    }
    
    #[test]
    fn test_nll_permite_secuencial() {
        let mut s = String::from("test");
        
        let r1 = &s;
        let _ = r1.len(); // Usar r1
        // r1 ya no se usa
        
        let r2 = &mut s; // OK gracias a NLL
        r2.push('!');
        
        assert_eq!(s, "test!");
    }
}
