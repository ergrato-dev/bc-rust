//! Soluciones - Práctica 02: Modos de Captura

fn main() {
    println!("=== Soluciones: Modos de Captura ===\n");

    ejercicio_1_referencia();
    ejercicio_2_referencia_mut();
    ejercicio_3_move();
    ejercicio_4_multiples();
    ejercicio_5_move_copy();

    println!("\n✅ Todas las soluciones funcionan!");
}

// SOLUCIÓN Ejercicio 1: Captura por referencia
fn ejercicio_1_referencia() {
    let mensaje = String::from("Hola, Rust!");
    
    // Captura &mensaje (referencia inmutable)
    let imprimir = || {
        println!("  -> {}", mensaje);
    };
    
    imprimir();
    imprimir();
    
    println!("Ejercicio 1 - Original: {}", mensaje);
}

// SOLUCIÓN Ejercicio 2: Captura por referencia mutable
fn ejercicio_2_referencia_mut() {
    let mut buffer = String::new();
    
    // Captura &mut buffer
    let mut agregar = |texto: &str| {
        buffer.push_str(texto);
    };
    
    agregar("Hola");
    agregar(" ");
    agregar("Mundo");
    
    drop(agregar);
    
    println!("Ejercicio 2 - Buffer: {}", buffer);
    assert_eq!(buffer, "Hola Mundo");
}

// SOLUCIÓN Ejercicio 3: Move
fn ejercicio_3_move() {
    let datos = vec![1, 2, 3, 4, 5];
    
    // move transfiere ownership de datos al closure
    let calcular_suma = move || -> i32 {
        datos.iter().sum()
    };
    
    let suma = calcular_suma();
    println!("Ejercicio 3 - Suma: {}", suma);
    // datos ya no disponible
}

// SOLUCIÓN Ejercicio 4: Múltiples capturas
fn ejercicio_4_multiples() {
    let constante = 10;
    let mut contador = 0;
    let mensaje = String::from("Procesando");
    
    let mut procesar = || {
        println!("  -> {}", mensaje);  // &String
        contador += 1;                   // &mut i32
        constante + contador             // &i32
    };
    
    let r1 = procesar();
    let r2 = procesar();
    
    drop(procesar);
    
    println!("Ejercicio 4 - Resultados: {}, {}", r1, r2);
    println!("Ejercicio 4 - Contador final: {}", contador);
}

// SOLUCIÓN Ejercicio 5: Move con Copy
fn ejercicio_5_move_copy() {
    let numero = 42;
    
    // move copia el valor porque i32: Copy
    let mostrar = move || {
        println!("  -> Número en closure: {}", numero);
    };
    
    mostrar();
    
    // numero sigue disponible
    println!("Ejercicio 5 - Numero original: {}", numero);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_soluciones() {
        // Test básico de que las funciones no hacen panic
        super::ejercicio_1_referencia();
        super::ejercicio_2_referencia_mut();
        super::ejercicio_3_move();
        super::ejercicio_4_multiples();
        super::ejercicio_5_move_copy();
    }
}
