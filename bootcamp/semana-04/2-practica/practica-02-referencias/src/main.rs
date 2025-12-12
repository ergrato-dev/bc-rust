// Práctica 02: Referencias
// =========================
// Completa los ejercicios siguiendo las instrucciones en README.md

fn main() {
    println!("=== Práctica 02: Referencias ===\n");
    
    ejercicio1();
    ejercicio2();
    ejercicio3();
    ejercicio4();
}

// Ejercicio 1: Referencia Inmutable
// TODO: Cambia el parámetro para recibir una referencia
fn longitud(s: String) -> usize {
    s.len()
}

fn ejercicio1() {
    println!("--- Ejercicio 1: Referencia Inmutable ---");
    
    let texto = String::from("Rust es genial");
    
    // TODO: Llama a longitud() sin mover texto
    let len = longitud(texto.clone()); // <- Modifica esta línea
    
    // Descomenta cuando funcione:
    // println!("'{}' tiene {} caracteres", texto, len);
    println!("Longitud: {}", len);
    println!();
}

// Ejercicio 2: Referencia Mutable
// TODO: Cambia el parámetro para recibir una referencia mutable
fn agregar_signo(s: String) -> String {
    // TODO: Modificar para usar &mut String
    let mut resultado = s;
    resultado.push('!');
    resultado
}

fn ejercicio2() {
    println!("--- Ejercicio 2: Referencia Mutable ---");
    
    let mut saludo = String::from("Hola");
    
    // TODO: Modifica para usar referencia mutable
    saludo = agregar_signo(saludo);
    
    println!("Saludo: {}", saludo);
    println!();
}

// Ejercicio 3: Analiza qué bloques compilan
fn ejercicio3() {
    println!("--- Ejercicio 3: Múltiples Referencias ---");
    
    // Bloque A: ¿Compila?
    {
        let s = String::from("hola");
        let r1 = &s;
        let r2 = &s;
        println!("  Bloque A: {} {}", r1, r2);
    }
    
    // Bloque B: ¿Compila? (Descomenta para probar)
    // {
    //     let mut s = String::from("hola");
    //     let r1 = &mut s;
    //     let r2 = &mut s;
    //     println!("  Bloque B: {} {}", r1, r2);
    // }
    
    // Bloque C: ¿Compila? (Descomenta para probar)
    // {
    //     let mut s = String::from("hola");
    //     let r1 = &s;
    //     let r2 = &mut s;
    //     println!("  Bloque C: {}", r1);
    // }
    
    println!();
}

// Ejercicio 4: Comparar strings
// TODO: Esta función tiene un problema con lifetimes
// Por ahora, retorna String en lugar de &String
fn mas_larga(s1: &String, s2: &String) -> String {
    if s1.len() >= s2.len() {
        s1.clone()
    } else {
        s2.clone()
    }
}

fn ejercicio4() {
    println!("--- Ejercicio 4: Comparar Strings ---");
    
    let corta = String::from("Rust");
    let larga = String::from("Programación");
    
    let resultado = mas_larga(&corta, &larga);
    
    println!("  Corta: '{}' ({} chars)", corta, corta.len());
    println!("  Larga: '{}' ({} chars)", larga, larga.len());
    println!("  Más larga: '{}'", resultado);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longitud_con_referencia() {
        let s = String::from("hola");
        // Si longitud usa &String, esto debe compilar:
        // let len = longitud(&s);
        // assert_eq!(len, 4);
        // assert_eq!(s, "hola"); // s sigue válido
    }
    
    #[test]
    fn test_agregar_signo() {
        let mut s = String::from("test");
        // Si agregar_signo usa &mut String:
        // agregar_signo(&mut s);
        // assert_eq!(s, "test!");
    }
    
    #[test]
    fn test_multiples_refs_inmutables() {
        let s = String::from("rust");
        let r1 = &s;
        let r2 = &s;
        let r3 = &s;
        
        assert_eq!(*r1, *r2);
        assert_eq!(*r2, *r3);
    }
}
