/// Ordenamiento burbuja O(n²).
pub fn burbuja(datos: &mut Vec<i64>) {
    let n = datos.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if datos[j] > datos[j + 1] {
                datos.swap(j, j + 1);
            }
        }
    }
}

/// Ordenamiento por inserción O(n²) pero eficiente para datos casi ordenados.
pub fn insercion(datos: &mut Vec<i64>) {
    for i in 1..datos.len() {
        let key = datos[i];
        let mut j = i;
        while j > 0 && datos[j - 1] > key {
            datos[j] = datos[j - 1];
            j -= 1;
        }
        datos[j] = key;
    }
}

/// Mergesort O(n log n).
pub fn mergesort(datos: &mut Vec<i64>) {
    let len = datos.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let mut izq = datos[..mid].to_vec();
    let mut der = datos[mid..].to_vec();
    mergesort(&mut izq);
    mergesort(&mut der);
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < izq.len() && j < der.len() {
        if izq[i] <= der[j] {
            datos[k] = izq[i];
            i += 1;
        } else {
            datos[k] = der[j];
            j += 1;
        }
        k += 1;
    }
    while i < izq.len() {
        datos[k] = izq[i];
        i += 1;
        k += 1;
    }
    while j < der.len() {
        datos[k] = der[j];
        j += 1;
        k += 1;
    }
}

/// Ordena usando la implementación de la biblioteca estándar (pdqsort).
pub fn stdlib_sort(datos: &mut Vec<i64>) {
    datos.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verificar(mut datos: Vec<i64>, f: fn(&mut Vec<i64>)) {
        let mut esperado = datos.clone();
        esperado.sort();
        f(&mut datos);
        assert_eq!(datos, esperado);
    }

    #[test]
    fn todos_algoritmos_correctos() {
        let datos = vec![5, 3, 8, 1, 9, 2, 7, 4, 6, 0];
        verificar(datos.clone(), burbuja);
        verificar(datos.clone(), insercion);
        verificar(datos.clone(), mergesort);
        verificar(datos.clone(), stdlib_sort);
    }
}
