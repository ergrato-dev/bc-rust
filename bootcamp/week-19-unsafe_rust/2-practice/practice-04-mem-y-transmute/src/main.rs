use std::mem;

fn main() {
    // --- size_of y align_of ---
    println!("size_of::<i32>()  = {}", mem::size_of::<i32>());
    println!("size_of::<f64>()  = {}", mem::size_of::<f64>());
    println!("align_of::<u8>()  = {}", mem::align_of::<u8>());

    // --- transmute: reinterpretar bits ---
    let bits: u32 = 0x3F80_0000; // IEEE 754 de 1.0f32
    // SAFETY: `u32` y `f32` tienen el mismo tamaño (4 bytes) y alineación.
    // Reinterpretar los bits de un u32 como f32 es comportamiento definido
    // siempre que el patrón de bits sea un valor float válido.
    let valor: f32 = unsafe { mem::transmute(bits) };
    println!("transmute(0x3F800000) = {valor}"); // 1.0

    // --- mem::replace ---
    let mut s = String::from("hola");
    let old = mem::replace(&mut s, String::from("mundo"));
    println!("old={old}, new={s}");

    // --- mem::forget: evitar el Drop ---
    let v = vec![1, 2, 3];
    mem::forget(v); // la memoria se "pierde" (leak intencional)
    println!("Vector olvidado (leak)");
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn transmute_f32_u32() {
        let f: f32 = 1.0;
        // SAFETY: f32 y u32 tienen el mismo tamaño; el patrón de bits de 1.0f32
        // es un u32 válido (0x3F800000).
        let bits: u32 = unsafe { mem::transmute(f) };
        assert_eq!(bits, 0x3F80_0000);
    }

    #[test]
    fn size_of_correctos() {
        assert_eq!(mem::size_of::<u8>(), 1);
        assert_eq!(mem::size_of::<u32>(), 4);
        assert_eq!(mem::size_of::<u64>(), 8);
    }

    #[test]
    fn replace_retorna_viejo() {
        let mut x = 10_i32;
        let old = mem::replace(&mut x, 99);
        assert_eq!(old, 10);
        assert_eq!(x, 99);
    }
}
