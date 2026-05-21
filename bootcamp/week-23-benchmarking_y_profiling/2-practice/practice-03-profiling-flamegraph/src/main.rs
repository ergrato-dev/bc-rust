/// Carga de CPU artificial para generar un flamegraph interesante.
///
/// Simula tres "hot paths" con pesos distintos para que el flamegraph
/// muestre claramente cuál función consume más tiempo.

fn calcular_primos(limite: u64) -> Vec<u64> {
    let mut primos = Vec::new();
    for n in 2..=limite {
        if es_primo(n) {
            primos.push(n);
        }
    }
    primos
}

fn es_primo(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn suma_factorial(n: u64) -> u64 {
    (1..=n).fold(1u64, |acc, x| acc.saturating_mul(x))
}

fn hash_simple(datos: &[u8]) -> u64 {
    let mut h: u64 = 5381;
    for &b in datos {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    h
}

fn main() {
    // Hot path 1: primos (dominante)
    let primos = calcular_primos(100_000);
    println!("Primos hasta 100k: {}", primos.len());

    // Hot path 2: factoriales
    let suma: u64 = (1..=20).map(suma_factorial).sum();
    println!("Suma de factoriales 1..20: {suma}");

    // Hot path 3: hash
    let datos: Vec<u8> = (0u8..=255).cycle().take(100_000).collect();
    let h = hash_simple(&datos);
    println!("Hash de 100k bytes: {h}");

    println!("✓ Genera un flamegraph con: cargo flamegraph --bin carga-cpu");
}
