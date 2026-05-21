/// Busca el máximo de un slice — versión v1 (simple).
pub fn maximo_v1(datos: &[i64]) -> Option<i64> {
    if datos.is_empty() {
        return None;
    }
    let mut max = datos[0];
    for &x in &datos[1..] {
        if x > max {
            max = x;
        }
    }
    Some(max)
}

/// Busca el máximo — versión v2 (usando iterator fold).
pub fn maximo_v2(datos: &[i64]) -> Option<i64> {
    datos.iter().copied().reduce(i64::max)
}

/// Cuenta ocurrencias — v1 (con HashMap allocation).
pub fn contar_v1(datos: &[i64], target: i64) -> usize {
    datos.iter().filter(|&&x| x == target).count()
}

/// Suma de cuadrados — v1 (collect intermedio).
pub fn suma_cuadrados_v1(datos: &[i64]) -> i64 {
    let cuadrados: Vec<i64> = datos.iter().map(|x| x * x).collect();
    cuadrados.iter().sum()
}

/// Suma de cuadrados — v2 (sin allocación intermedia).
pub fn suma_cuadrados_v2(datos: &[i64]) -> i64 {
    datos.iter().map(|x| x * x).sum()
}

/// Concatena strings — v1 (+ operator, cuadrático).
pub fn concatenar_v1(palabras: &[&str]) -> String {
    let mut resultado = String::new();
    for &p in palabras {
        resultado = resultado + p;
    }
    resultado
}

/// Concatena strings — v2 (pre-alloca con capacity).
pub fn concatenar_v2(palabras: &[&str]) -> String {
    let capacidad: usize = palabras.iter().map(|s| s.len()).sum();
    let mut resultado = String::with_capacity(capacidad);
    for &p in palabras {
        resultado.push_str(p);
    }
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximo_coincide() {
        let d = vec![3i64, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(maximo_v1(&d), maximo_v2(&d));
        assert_eq!(maximo_v1(&d), Some(9));
    }

    #[test]
    fn suma_cuadrados_coincide() {
        let d: Vec<i64> = (1..=10).collect();
        assert_eq!(suma_cuadrados_v1(&d), suma_cuadrados_v2(&d));
        assert_eq!(suma_cuadrados_v1(&d), 385);
    }

    #[test]
    fn concatenar_coincide() {
        let palabras = ["hola", " ", "mundo"];
        assert_eq!(concatenar_v1(&palabras), concatenar_v2(&palabras));
    }
}
