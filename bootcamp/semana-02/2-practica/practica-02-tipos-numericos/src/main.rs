// ============================================
// Practice 02: Numeric Types
// ============================================
// Objective: Master integers and floats
// ============================================

fn main() {
    println!("=== Practice 02: Numeric Types ===\n");

    // -----------------------------------------
    // PART 1: Signed Integers
    // -----------------------------------------
    println!("--- Signed Integers ---");
    
    // i8: -128 to 127
    let small_byte: i8 = 100;
    println!("i8: {}", small_byte);
    
    // TODO: Declare i16, i32, i64 variables
    // let medium: i16 = ...;
    // let normal: i32 = ...;     // default type
    // let large: i64 = ...;
    
    // Using type suffixes
    let with_suffix = 42i32;
    println!("With suffix: {}", with_suffix);

    // -----------------------------------------
    // PART 2: Unsigned Integers
    // -----------------------------------------
    println!("\n--- Unsigned Integers ---");
    
    // u8: 0 to 255 (ideal for bytes, RGB colors)
    let red: u8 = 255;
    let green: u8 = 128;
    let blue: u8 = 0;
    println!("RGB: ({}, {}, {})", red, green, blue);
    
    // TODO: Declare a u32 counter
    // let counter: u32 = ...;
    
    // usize: size depends on architecture (32/64 bits)
    // Used for indices and sizes
    let index: usize = 0;
    println!("Index: {}", index);

    // -----------------------------------------
    // PART 3: Floats
    // -----------------------------------------
    println!("\n--- Floats ---");
    
    // f64 is the default type (more precision)
    let pi: f64 = 3.14159265358979;
    println!("PI (f64): {}", pi);
    
    // f32 has less precision but uses less memory
    let pi_32: f32 = 3.14159265358979;
    println!("PI (f32): {}", pi_32);  // Notice the precision loss
    
    // TODO: Calculate the area of a circle with radius 5
    let radius: f64 = 5.0;
    // let area = ...;
    // println!("Area: {}", area);

    // -----------------------------------------
    // PART 4: Operations
    // -----------------------------------------
    println!("\n--- Operations ---");
    
    let a = 10;
    let b = 3;
    
    println!("Addition: {} + {} = {}", a, b, a + b);
    println!("Subtraction: {} - {} = {}", a, b, a - b);
    println!("Multiplication: {} * {} = {}", a, b, a * b);
    println!("Integer division: {} / {} = {}", a, b, a / b);
    println!("Modulo: {} % {} = {}", a, b, a % b);
    
    // Float division
    let x = 10.0;
    let y = 3.0;
    println!("Float division: {} / {} = {:.4}", x, y, x / y);

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_integer_operations() {
        assert_eq!(10 + 3, 13);
        assert_eq!(10 - 3, 7);
        assert_eq!(10 * 3, 30);
        assert_eq!(10 / 3, 3);
        assert_eq!(10 % 3, 1);
    }

    #[test]
    fn test_float_operations() {
        let result = 10.0_f64 / 3.0;
        assert!((result - 3.333333).abs() < 0.001);
    }

    #[test]
    fn test_type_sizes() {
        assert_eq!(std::mem::size_of::<i8>(), 1);
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<f64>(), 8);
    }
}
