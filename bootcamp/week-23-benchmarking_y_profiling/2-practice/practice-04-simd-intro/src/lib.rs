/// Suma escalar de dos slices elemento a elemento.
pub fn suma_escalar(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

/// Suma usando iteradores (el compilador puede autovectorizar).
pub fn suma_iteradores(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    out.iter_mut()
        .zip(a.iter().zip(b.iter()))
        .for_each(|(o, (x, y))| *o = x + y);
}

/// Producto punto (dot product) escalar.
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Normaliza un vector (divide cada elemento por la norma).
pub fn normalizar(v: &[f32]) -> Vec<f32> {
    let norma: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norma == 0.0 {
        return v.to_vec();
    }
    v.iter().map(|x| x / norma).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_es_correcta() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0];
        let b = vec![10.0f32, 20.0, 30.0, 40.0];
        let mut out = vec![0.0f32; 4];
        suma_escalar(&a, &b, &mut out);
        assert_eq!(out, vec![11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    fn dot_product_correcto() {
        let a = vec![1.0f32, 2.0, 3.0];
        let b = vec![4.0f32, 5.0, 6.0];
        assert!((dot_product(&a, &b) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn normalizar_produce_unit_vector() {
        let v = vec![3.0f32, 4.0];
        let n = normalizar(&v);
        let norma: f32 = n.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norma - 1.0).abs() < 1e-6);
    }
}
