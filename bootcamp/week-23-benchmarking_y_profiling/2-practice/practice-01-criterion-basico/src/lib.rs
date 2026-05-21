/// Fibonacci recursivo (lento — para benchmarking de comparación).
pub fn fib_recursivo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursivo(n - 1) + fib_recursivo(n - 2),
    }
}

/// Fibonacci iterativo (rápido).
pub fn fib_iterativo(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ambas_implementaciones_coinciden() {
        for i in 0..=15 {
            assert_eq!(
                fib_recursivo(i),
                fib_iterativo(i),
                "Fibonacci({i}) debe coincidir"
            );
        }
    }
}
