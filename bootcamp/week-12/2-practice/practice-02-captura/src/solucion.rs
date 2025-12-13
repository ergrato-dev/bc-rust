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
    let message = String::from("Hola, Rust!");
    
    // Captura &message (referencia inmutable)
    let print_msg = || {
        println!("  -> {}", message);
    };
    
    print_msg();
    print_msg();
    
    println!("Ejercicio 1 - Original: {}", message);
}

// SOLUCIÓN Ejercicio 2: Captura por referencia mutable
fn ejercicio_2_referencia_mut() {
    let mut buffer = String::new();
    
    // Captura &mut buffer
    let mut append = |text: &str| {
        buffer.push_str(text);
    };
    
    append("Hola");
    append(" ");
    append("Mundo");
    
    drop(append);
    
    println!("Ejercicio 2 - Buffer: {}", buffer);
    assert_eq!(buffer, "Hola Mundo");
}

// SOLUCIÓN Ejercicio 3: Move
fn ejercicio_3_move() {
    let data = vec![1, 2, 3, 4, 5];
    
    // move transfiere ownership de data al closure
    let calculate_sum = move || -> i32 {
        data.iter().sum()
    };
    
    let sum = calculate_sum();
    println!("Ejercicio 3 - Suma: {}", sum);
    // data ya no disponible
}

// SOLUCIÓN Ejercicio 4: Múltiples capturas
fn ejercicio_4_multiples() {
    let constant = 10;
    let mut counter = 0;
    let message = String::from("Procesando");
    
    let mut process = || {
        println!("  -> {}", message);  // &String
        counter += 1;                   // &mut i32
        constant + counter             // &i32
    };
    
    let r1 = process();
    let r2 = process();
    
    drop(process);
    
    println!("Ejercicio 4 - Resultados: {}, {}", r1, r2);
    println!("Ejercicio 4 - Contador final: {}", counter);
}

// SOLUCIÓN Ejercicio 5: Move con Copy
fn ejercicio_5_move_copy() {
    let number = 42;
    
    // move copia el valor porque i32: Copy
    let show = move || {
        println!("  -> Número en closure: {}", number);
    };
    
    show();
    
    // number sigue disponible
    println!("Ejercicio 5 - Numero original: {}", number);
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
