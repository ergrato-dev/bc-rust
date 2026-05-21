use wasm_bindgen::prelude::*;

/// Retorna el cuadrado de cada número en el arreglo.
///
/// Demuestra cómo pasar y retornar `Vec<f64>` a través de la frontera WASM/JS.
#[wasm_bindgen]
pub fn cuadrados(valores: &[f64]) -> Vec<f64> {
    valores.iter().map(|x| x * x).collect()
}

/// Filtra solo los valores positivos del arreglo.
#[wasm_bindgen]
pub fn solo_positivos(valores: &[f64]) -> Vec<f64> {
    valores.iter().copied().filter(|x| *x > 0.0).collect()
}

/// Retorna `true` si todos los elementos son pares.
#[wasm_bindgen]
pub fn todos_pares(valores: &[i32]) -> bool {
    valores.iter().all(|x| x % 2 == 0)
}

/// Convierte un slice de strings en sus longitudes.
#[wasm_bindgen]
pub fn longitudes(palabras: Vec<String>) -> Vec<u32> {
    palabras.iter().map(|s| s.len() as u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuadrados_correctos() {
        assert_eq!(cuadrados(&[1.0, 2.0, 3.0]), vec![1.0, 4.0, 9.0]);
    }

    #[test]
    fn solo_positivos_filtra() {
        assert_eq!(solo_positivos(&[-1.0, 2.0, -3.0, 4.0]), vec![2.0, 4.0]);
    }

    #[test]
    fn todos_pares_correcto() {
        assert!(todos_pares(&[2, 4, 6]));
        assert!(!todos_pares(&[2, 3, 6]));
    }
}
