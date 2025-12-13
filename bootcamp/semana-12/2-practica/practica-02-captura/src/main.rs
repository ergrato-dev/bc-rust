//! Práctica 02: Modos de Captura
//!
//! En esta práctica aprenderás a:
//! - Capturar por referencia (&T)
//! - Capturar por referencia mutable (&mut T)
//! - Capturar por valor con move (T)
//! - Predecir qué modo usa el compilador

fn main() {
    println!("=== Práctica 02: Modos de Captura ===\n");

    // Ejercicio 1: Captura por referencia
    ejercicio_1_referencia();

    // Ejercicio 2: Captura por referencia mutable
    ejercicio_2_referencia_mut();

    // Ejercicio 3: Captura por valor (move)
    ejercicio_3_move();

    // Ejercicio 4: Closure con múltiples capturas
    ejercicio_4_multiples();

    // Ejercicio 5: Move con tipos Copy
    ejercicio_5_move_copy();

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Captura por Referencia
// ============================================================
// El closure debe leer el valor sin modificarlo.
// Después del closure, el valor original debe seguir disponible.
//
// TODO: Crea un closure que lea 'mensaje' y lo imprima
// ============================================================

fn ejercicio_1_referencia() {
    let mensaje = String::from("Hola, Rust!");
    
    // TODO: Crea un closure 'imprimir' que imprima 'mensaje'
    // El closure debe capturar por referencia (&String)
    let imprimir = || {
        todo!("Implementa el closure que imprime mensaje")
    };
    
    imprimir();
    imprimir(); // Debe poder llamarse múltiples veces
    
    // mensaje debe seguir disponible
    println!("Ejercicio 1 - Original: {}", mensaje);
}

// ============================================================
// EJERCICIO 2: Captura por Referencia Mutable
// ============================================================
// El closure debe poder modificar el valor capturado.
//
// TODO: Crea un closure que agregue texto a 'buffer'
// ============================================================

fn ejercicio_2_referencia_mut() {
    let mut buffer = String::new();
    
    // TODO: Crea un closure 'agregar' que agregue texto a buffer
    // Debe capturar &mut buffer
    // El closure también debe ser mut
    let mut agregar = |texto: &str| {
        todo!("Implementa el closure que agrega texto a buffer")
    };
    
    agregar("Hola");
    agregar(" ");
    agregar("Mundo");
    
    // El closure debe liberarse antes de usar buffer
    drop(agregar);
    
    println!("Ejercicio 2 - Buffer: {}", buffer);
    assert_eq!(buffer, "Hola Mundo");
}

// ============================================================
// EJERCICIO 3: Captura por Valor (move)
// ============================================================
// El closure debe tomar ownership del valor.
//
// TODO: Usa 'move' para transferir ownership al closure
// ============================================================

fn ejercicio_3_move() {
    let datos = vec![1, 2, 3, 4, 5];
    
    // TODO: Crea un closure con 'move' que tome ownership de 'datos'
    // y retorne la suma de sus elementos
    let calcular_suma = todo!("Implementa closure con move");
    
    let suma = calcular_suma();
    println!("Ejercicio 3 - Suma: {}", suma);
    
    // datos ya no debe estar disponible (comentado intencionalmente)
    // println!("{:?}", datos); // ERROR: datos fue movido
}

// ============================================================
// EJERCICIO 4: Múltiples Capturas
// ============================================================
// Un closure puede capturar múltiples variables con diferentes modos.
//
// TODO: Analiza qué modo de captura se usa para cada variable
// ============================================================

fn ejercicio_4_multiples() {
    let constante = 10;           // Se leerá
    let mut contador = 0;         // Se modificará
    let mensaje = String::from("Procesando"); // Se leerá
    
    // TODO: Completa el closure que:
    // - Lee 'constante' (captura &i32)
    // - Modifica 'contador' (captura &mut i32)
    // - Lee 'mensaje' (captura &String)
    let mut procesar = || {
        todo!("Implementa el closure con múltiples capturas")
        // Incrementa contador y retorna constante + contador
        // También imprime el mensaje
    };
    
    let r1 = procesar();
    let r2 = procesar();
    
    drop(procesar);
    
    println!("Ejercicio 4 - Resultados: {}, {}", r1, r2);
    println!("Ejercicio 4 - Contador final: {}", contador);
}

// ============================================================
// EJERCICIO 5: Move con Tipos Copy
// ============================================================
// Para tipos Copy, 'move' crea una copia.
//
// TODO: Demuestra que el valor original sigue disponible
// ============================================================

fn ejercicio_5_move_copy() {
    let numero = 42; // i32 implementa Copy
    
    // TODO: Crea un closure con move que use 'numero'
    let mostrar = todo!("Implementa closure con move para tipo Copy");
    
    mostrar();
    
    // numero sigue disponible porque se copió
    println!("Ejercicio 5 - Numero original: {}", numero);
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    #[test]
    fn test_captura_referencia() {
        let s = String::from("test");
        let len = || s.len();
        
        assert_eq!(len(), 4);
        assert_eq!(len(), 4);
        assert_eq!(s, "test"); // s sigue disponible
    }

    #[test]
    fn test_captura_mut() {
        let mut v = Vec::new();
        let mut push = |x| v.push(x);
        
        push(1);
        push(2);
        
        drop(push);
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn test_captura_move() {
        let s = String::from("moved");
        let consume = move || s.len();
        
        assert_eq!(consume(), 5);
        // s ya no disponible
    }

    #[test]
    fn test_move_copy() {
        let x = 42;
        let f = move || x * 2;
        
        assert_eq!(f(), 84);
        assert_eq!(x, 42); // x sigue disponible (Copy)
    }
}
