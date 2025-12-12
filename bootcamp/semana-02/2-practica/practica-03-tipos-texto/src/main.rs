// ============================================
// Práctica 03: Tipos de Texto
// ============================================
// Objetivo: Dominar char, &str, String y bool
// ============================================

fn main() {
    println!("=== Práctica 03: Tipos de Texto ===\n");

    // -----------------------------------------
    // PARTE 1: Caracteres (char)
    // -----------------------------------------
    println!("--- Caracteres ---");
    
    // char usa comillas simples ''
    // Representa un valor Unicode (4 bytes)
    let letra: char = 'A';
    let numero: char = '7';
    let simbolo: char = '@';
    let emoji: char = '🦀';       // ¡Rust tiene un emoji!
    let acento: char = 'ñ';
    
    println!("Letra: {}", letra);
    println!("Número: {}", numero);
    println!("Símbolo: {}", simbolo);
    println!("Emoji: {}", emoji);
    println!("Acento: {}", acento);
    
    // Métodos útiles de char
    println!("\nMétodos de char:");
    println!("'A'.is_alphabetic(): {}", 'A'.is_alphabetic());
    println!("'7'.is_numeric(): {}", '7'.is_numeric());
    println!("'a'.is_lowercase(): {}", 'a'.is_lowercase());
    println!("'A'.to_lowercase(): {}", 'A'.to_lowercase());

    // -----------------------------------------
    // PARTE 2: String Slices (&str)
    // -----------------------------------------
    println!("\n--- String Slices (&str) ---");
    
    // &str son referencias a texto, inmutables
    let saludo: &str = "¡Hola, Rust!";
    let vacio: &str = "";
    
    println!("Saludo: {}", saludo);
    println!("Longitud: {} bytes", saludo.len());
    println!("¿Está vacío?: {}", vacio.is_empty());
    
    // TODO: Declara un &str con tu frase favorita
    // let frase: &str = ...;
    // println!("Frase: {}", frase);

    // -----------------------------------------
    // PARTE 3: String (owned)
    // -----------------------------------------
    println!("\n--- String (owned) ---");
    
    // String es texto en el heap, mutable
    let mut mensaje = String::from("Hola");
    println!("Inicial: {}", mensaje);
    
    // push_str añade un &str
    mensaje.push_str(", mundo");
    println!("Después de push_str: {}", mensaje);
    
    // push añade un char
    mensaje.push('!');
    println!("Después de push: {}", mensaje);
    
    // Otra forma de crear String
    let otro = "Rust".to_string();
    println!("Con to_string(): {}", otro);
    
    // TODO: Crea un String y modifícalo
    // let mut mi_string = String::from(...);
    // mi_string.push_str(...);

    // -----------------------------------------
    // PARTE 4: Booleanos
    // -----------------------------------------
    println!("\n--- Booleanos ---");
    
    let verdadero: bool = true;
    let falso: bool = false;
    
    println!("true: {}", verdadero);
    println!("false: {}", falso);
    
    // Operadores lógicos
    println!("\nOperadores lógicos:");
    println!("true && false = {}", true && false);  // AND
    println!("true || false = {}", true || false);  // OR
    println!("!true = {}", !true);                  // NOT
    
    // Comparaciones
    let a = 5;
    let b = 10;
    println!("\nComparaciones:");
    println!("{} == {} : {}", a, b, a == b);
    println!("{} != {} : {}", a, b, a != b);
    println!("{} < {} : {}", a, b, a < b);
    println!("{} >= {} : {}", a, b, a >= b);

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char_unicode() {
        let emoji = '🦀';
        assert_eq!(std::mem::size_of_val(&emoji), 4); // 4 bytes
    }

    #[test]
    fn test_char_metodos() {
        assert!('A'.is_alphabetic());
        assert!('5'.is_numeric());
        assert!(' '.is_whitespace());
    }

    #[test]
    fn test_str_len() {
        let texto = "Hola";
        assert_eq!(texto.len(), 4);
        
        // ¡Cuidado! len() cuenta bytes, no caracteres
        let con_emoji = "🦀";
        assert_eq!(con_emoji.len(), 4); // El emoji usa 4 bytes
    }

    #[test]
    fn test_string_mutable() {
        let mut s = String::from("Hola");
        s.push_str(" mundo");
        assert_eq!(s, "Hola mundo");
    }

    #[test]
    fn test_booleanos() {
        assert!(true && true);
        assert!(true || false);
        assert!(!false);
    }
}
