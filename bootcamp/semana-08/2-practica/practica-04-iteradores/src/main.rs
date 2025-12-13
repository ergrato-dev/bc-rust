//! # Práctica 04: Iteradores
//!
//! Ejercicios para dominar iteradores en Rust.

use std::collections::HashMap;

fn main() {
    println!("=== Práctica 04: Iteradores ===\n");

    // Ejercicio 1: Tipos de iteradores
    demo_tipos_iteradores();

    // Ejercicio 2: Adaptadores
    demo_adaptadores();

    // Ejercicio 3: Consumidores
    demo_consumidores();

    // Ejercicio 4: Procesador de datos
    demo_procesador();
}

// ============================================================================
// EJERCICIO 1: Tipos de Iteradores
// ============================================================================

/// Suma elementos usando iter() - préstamo inmutable
fn sumar_elementos(numeros: &[i32]) -> i32 {
    numeros.iter().sum()
}

/// Duplica elementos in-place usando iter_mut() - préstamo mutable
fn duplicar_elementos(numeros: &mut [i32]) {
    for num in numeros.iter_mut() {
        *num *= 2;
    }
}

/// Convierte a mayúsculas consumiendo el vector - into_iter()
fn a_mayusculas(palabras: Vec<String>) -> Vec<String> {
    palabras.into_iter().map(|s| s.to_uppercase()).collect()
}

/// Filtra y transforma sin consumir el original
fn filtrar_positivos(numeros: &[i32]) -> Vec<i32> {
    numeros.iter().filter(|&&n| n > 0).copied().collect()
}

/// Encuentra el máximo sin consumir
fn encontrar_maximo(numeros: &[i32]) -> Option<i32> {
    numeros.iter().copied().max()
}

fn demo_tipos_iteradores() {
    println!("--- Ejercicio 1: Tipos de Iteradores ---");

    // iter() - préstamo inmutable
    let numeros = vec![1, 2, 3, 4, 5];
    let suma = sumar_elementos(&numeros);
    println!("Suma de {:?}: {}", numeros, suma);

    // iter_mut() - préstamo mutable
    let mut nums = vec![1, 2, 3];
    println!("Antes de duplicar: {:?}", nums);
    duplicar_elementos(&mut nums);
    println!("Después de duplicar: {:?}", nums);

    // into_iter() - consume el vector
    let palabras = vec!["hola".to_string(), "mundo".to_string()];
    let mayusculas = a_mayusculas(palabras);
    // palabras ya no es accesible aquí
    println!("En mayúsculas: {:?}", mayusculas);

    // Filtrar positivos
    let mixtos = vec![-2, -1, 0, 1, 2, 3];
    let positivos = filtrar_positivos(&mixtos);
    println!("Positivos de {:?}: {:?}", mixtos, positivos);

    // Máximo
    let valores = vec![42, 17, 93, 5, 68];
    if let Some(max) = encontrar_maximo(&valores) {
        println!("Máximo de {:?}: {}", valores, max);
    }

    println!();
}

// ============================================================================
// EJERCICIO 2: Adaptadores
// ============================================================================

/// Aplica una función a cada elemento
fn mapear_cuadrados(numeros: &[i32]) -> Vec<i32> {
    numeros.iter().map(|n| n * n).collect()
}

/// Filtra elementos que cumplen un predicado
fn filtrar_pares(numeros: &[i32]) -> Vec<i32> {
    numeros.iter().filter(|&&n| n % 2 == 0).copied().collect()
}

/// Combina filter y map
fn cuadrados_de_pares(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

/// Usa enumerate para obtener índices
fn con_indices(elementos: &[&str]) -> Vec<(usize, String)> {
    elementos
        .iter()
        .enumerate()
        .map(|(i, s)| (i, s.to_string()))
        .collect()
}

/// Combina dos iteradores con zip
fn combinar_pares(primeros: &[i32], segundos: &[i32]) -> Vec<(i32, i32)> {
    primeros.iter().zip(segundos.iter()).map(|(&a, &b)| (a, b)).collect()
}

/// Aplana estructuras anidadas con flat_map
fn aplanar(anidado: &[Vec<i32>]) -> Vec<i32> {
    anidado.iter().flat_map(|v| v.iter().copied()).collect()
}

/// Toma los primeros n elementos
fn primeros_n(elementos: &[i32], n: usize) -> Vec<i32> {
    elementos.iter().take(n).copied().collect()
}

/// Salta los primeros n elementos
fn sin_primeros_n(elementos: &[i32], n: usize) -> Vec<i32> {
    elementos.iter().skip(n).copied().collect()
}

/// Toma mientras se cumple la condición
fn tomar_mientras_positivo(elementos: &[i32]) -> Vec<i32> {
    elementos.iter().take_while(|&&n| n > 0).copied().collect()
}

/// Cadena de operaciones
fn pipeline_completo(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter()
        .filter(|&&n| n > 0)         // Solo positivos
        .map(|n| n * 2)               // Duplicar
        .filter(|&n| n < 20)          // Menores que 20
        .take(3)                      // Solo 3 primeros
        .collect()
}

fn demo_adaptadores() {
    println!("--- Ejercicio 2: Adaptadores ---");

    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Map
    println!("Cuadrados: {:?}", mapear_cuadrados(&numeros));

    // Filter
    println!("Pares: {:?}", filtrar_pares(&numeros));

    // Filter + Map
    println!("Cuadrados de pares: {:?}", cuadrados_de_pares(&numeros));

    // Enumerate
    let frutas = vec!["manzana", "pera", "naranja"];
    println!("Con índices: {:?}", con_indices(&frutas));

    // Zip
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];
    println!("Combinados: {:?}", combinar_pares(&a, &b));

    // Flat map
    let anidado = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    println!("Aplanado: {:?}", aplanar(&anidado));

    // Take y Skip
    println!("Primeros 3: {:?}", primeros_n(&numeros, 3));
    println!("Sin primeros 3: {:?}", sin_primeros_n(&numeros, 3));

    // Take while
    let mixtos = vec![5, 3, 8, -1, 4, 6];
    println!("Mientras positivo: {:?}", tomar_mientras_positivo(&mixtos));

    // Pipeline completo
    println!("Pipeline completo: {:?}", pipeline_completo(&numeros));

    println!();
}

// ============================================================================
// EJERCICIO 3: Consumidores
// ============================================================================

/// Recolecta en un vector
fn a_vector(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    iter.collect()
}

/// Recolecta en un HashMap
fn a_hashmap(pares: impl Iterator<Item = (String, i32)>) -> HashMap<String, i32> {
    pares.collect()
}

/// Suma con fold
fn suma_con_fold(numeros: &[i32]) -> i32 {
    numeros.iter().fold(0, |acc, &n| acc + n)
}

/// Producto con fold
fn producto_con_fold(numeros: &[i32]) -> i32 {
    numeros.iter().fold(1, |acc, &n| acc * n)
}

/// Concatena strings con fold
fn concatenar_con_separador(palabras: &[&str], sep: &str) -> String {
    palabras
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (i, palabra)| {
            if i > 0 {
                acc.push_str(sep);
            }
            acc.push_str(palabra);
            acc
        })
}

/// Reduce para encontrar el máximo
fn maximo_con_reduce(numeros: &[i32]) -> Option<i32> {
    numeros.iter().copied().reduce(|max, n| if n > max { n } else { max })
}

/// Encuentra el primer elemento que cumple
fn primer_par(numeros: &[i32]) -> Option<i32> {
    numeros.iter().find(|&&n| n % 2 == 0).copied()
}

/// Encuentra la posición del primer elemento que cumple
fn posicion_mayor_que(numeros: &[i32], umbral: i32) -> Option<usize> {
    numeros.iter().position(|&n| n > umbral)
}

/// Verifica si algún elemento cumple
fn alguno_negativo(numeros: &[i32]) -> bool {
    numeros.iter().any(|&n| n < 0)
}

/// Verifica si todos los elementos cumplen
fn todos_positivos(numeros: &[i32]) -> bool {
    numeros.iter().all(|&n| n > 0)
}

/// Cuenta elementos que cumplen
fn contar_pares(numeros: &[i32]) -> usize {
    numeros.iter().filter(|&&n| n % 2 == 0).count()
}

/// Particiona en dos grupos
fn particionar_por_paridad(numeros: &[i32]) -> (Vec<i32>, Vec<i32>) {
    numeros.iter().partition(|&&n| n % 2 == 0)
}

fn demo_consumidores() {
    println!("--- Ejercicio 3: Consumidores ---");

    let numeros = vec![1, 2, 3, 4, 5];

    // Collect a vector
    let cuadrados: Vec<i32> = a_vector(numeros.iter().map(|n| n * n));
    println!("Cuadrados: {:?}", cuadrados);

    // Collect a HashMap
    let pares = vec![
        ("uno".to_string(), 1),
        ("dos".to_string(), 2),
    ];
    let mapa = a_hashmap(pares.into_iter());
    println!("HashMap: {:?}", mapa);

    // Fold
    println!("Suma con fold: {}", suma_con_fold(&numeros));
    println!("Producto con fold: {}", producto_con_fold(&numeros));

    // Concatenar
    let palabras = vec!["Rust", "es", "genial"];
    println!("Concatenado: {}", concatenar_con_separador(&palabras, " "));

    // Reduce
    let vals = vec![5, 2, 9, 1, 7];
    if let Some(max) = maximo_con_reduce(&vals) {
        println!("Máximo con reduce: {}", max);
    }

    // Find
    if let Some(par) = primer_par(&numeros) {
        println!("Primer par: {}", par);
    }

    // Position
    if let Some(pos) = posicion_mayor_que(&numeros, 3) {
        println!("Posición del primer > 3: {}", pos);
    }

    // Any y All
    let mixtos = vec![1, -2, 3, -4];
    let positivos = vec![1, 2, 3, 4];
    println!("Mixtos tiene negativos: {}", alguno_negativo(&mixtos));
    println!("Positivos todos > 0: {}", todos_positivos(&positivos));

    // Count
    println!("Cantidad de pares en {:?}: {}", numeros, contar_pares(&numeros));

    // Partition
    let (pares, impares) = particionar_por_paridad(&numeros);
    println!("Pares: {:?}, Impares: {:?}", pares, impares);

    println!();
}

// ============================================================================
// EJERCICIO 4: Procesador de Datos
// ============================================================================

#[derive(Debug, Clone)]
struct Producto {
    id: u32,
    nombre: String,
    precio: f64,
    categoria: String,
    stock: u32,
}

impl Producto {
    fn new(id: u32, nombre: &str, precio: f64, categoria: &str, stock: u32) -> Self {
        Self {
            id,
            nombre: nombre.to_string(),
            precio,
            categoria: categoria.to_string(),
            stock,
        }
    }

    fn valor_inventario(&self) -> f64 {
        self.precio * self.stock as f64
    }
}

struct Inventario {
    productos: Vec<Producto>,
}

impl Inventario {
    fn new(productos: Vec<Producto>) -> Self {
        Self { productos }
    }

    /// Filtra productos por categoría
    fn por_categoria(&self, categoria: &str) -> Vec<&Producto> {
        self.productos
            .iter()
            .filter(|p| p.categoria == categoria)
            .collect()
    }

    /// Productos con stock bajo
    fn stock_bajo(&self, umbral: u32) -> Vec<&Producto> {
        self.productos
            .iter()
            .filter(|p| p.stock < umbral)
            .collect()
    }

    /// Valor total del inventario
    fn valor_total(&self) -> f64 {
        self.productos.iter().map(|p| p.valor_inventario()).sum()
    }

    /// Producto más caro
    fn mas_caro(&self) -> Option<&Producto> {
        self.productos
            .iter()
            .max_by(|a, b| a.precio.partial_cmp(&b.precio).unwrap())
    }

    /// Producto más barato
    fn mas_barato(&self) -> Option<&Producto> {
        self.productos
            .iter()
            .min_by(|a, b| a.precio.partial_cmp(&b.precio).unwrap())
    }

    /// Precio promedio
    fn precio_promedio(&self) -> f64 {
        if self.productos.is_empty() {
            return 0.0;
        }
        let total: f64 = self.productos.iter().map(|p| p.precio).sum();
        total / self.productos.len() as f64
    }

    /// Nombres de productos ordenados por precio
    fn nombres_por_precio(&self) -> Vec<String> {
        let mut productos: Vec<&Producto> = self.productos.iter().collect();
        productos.sort_by(|a, b| a.precio.partial_cmp(&b.precio).unwrap());
        productos.iter().map(|p| p.nombre.clone()).collect()
    }

    /// Agrupa productos por categoría
    fn agrupar_por_categoria(&self) -> HashMap<String, Vec<&Producto>> {
        let mut grupos: HashMap<String, Vec<&Producto>> = HashMap::new();
        for producto in &self.productos {
            grupos.entry(producto.categoria.clone())
                .or_default()
                .push(producto);
        }
        grupos
    }

    /// Valor total por categoría
    fn valor_por_categoria(&self) -> HashMap<String, f64> {
        self.agrupar_por_categoria()
            .into_iter()
            .map(|(cat, prods)| {
                let valor: f64 = prods.iter().map(|p| p.valor_inventario()).sum();
                (cat, valor)
            })
            .collect()
    }

    /// Busca productos por nombre (contiene)
    fn buscar(&self, texto: &str) -> Vec<&Producto> {
        let texto_lower = texto.to_lowercase();
        self.productos
            .iter()
            .filter(|p| p.nombre.to_lowercase().contains(&texto_lower))
            .collect()
    }

    /// Aplicar descuento a una categoría
    fn aplicar_descuento(&mut self, categoria: &str, porcentaje: f64) {
        self.productos
            .iter_mut()
            .filter(|p| p.categoria == categoria)
            .for_each(|p| {
                p.precio *= 1.0 - (porcentaje / 100.0);
            });
    }

    /// Estadísticas por categoría
    fn estadisticas(&self) -> Vec<(String, usize, f64, f64)> {
        self.agrupar_por_categoria()
            .into_iter()
            .map(|(cat, prods)| {
                let cantidad = prods.len();
                let precio_prom = prods.iter().map(|p| p.precio).sum::<f64>() / cantidad as f64;
                let valor_total = prods.iter().map(|p| p.valor_inventario()).sum();
                (cat, cantidad, precio_prom, valor_total)
            })
            .collect()
    }

    /// Genera reporte de inventario
    fn reporte(&self) -> String {
        let mut resultado = String::from("=== REPORTE DE INVENTARIO ===\n\n");

        resultado.push_str(&format!(
            "Total productos: {}\n",
            self.productos.len()
        ));
        resultado.push_str(&format!(
            "Valor total: ${:.2}\n",
            self.valor_total()
        ));
        resultado.push_str(&format!(
            "Precio promedio: ${:.2}\n\n",
            self.precio_promedio()
        ));

        if let Some(caro) = self.mas_caro() {
            resultado.push_str(&format!(
                "Más caro: {} (${:.2})\n",
                caro.nombre, caro.precio
            ));
        }

        if let Some(barato) = self.mas_barato() {
            resultado.push_str(&format!(
                "Más barato: {} (${:.2})\n\n",
                barato.nombre, barato.precio
            ));
        }

        resultado.push_str("Por categoría:\n");
        for (cat, cantidad, prom, valor) in self.estadisticas() {
            resultado.push_str(&format!(
                "  - {}: {} productos, precio prom: ${:.2}, valor: ${:.2}\n",
                cat, cantidad, prom, valor
            ));
        }

        let bajo_stock: Vec<_> = self.stock_bajo(5).iter().map(|p| &p.nombre).collect();
        if !bajo_stock.is_empty() {
            resultado.push_str(&format!("\n⚠️ Stock bajo: {:?}\n", bajo_stock));
        }

        resultado
    }
}

fn demo_procesador() {
    println!("--- Ejercicio 4: Procesador de Datos ---");

    let mut inventario = Inventario::new(vec![
        Producto::new(1, "Laptop", 999.99, "Electrónica", 10),
        Producto::new(2, "Mouse", 29.99, "Electrónica", 50),
        Producto::new(3, "Teclado", 79.99, "Electrónica", 3),
        Producto::new(4, "Silla", 199.99, "Muebles", 15),
        Producto::new(5, "Escritorio", 299.99, "Muebles", 8),
        Producto::new(6, "Lámpara", 49.99, "Muebles", 2),
        Producto::new(7, "Cuaderno", 5.99, "Oficina", 100),
        Producto::new(8, "Bolígrafo", 1.99, "Oficina", 200),
    ]);

    // Por categoría
    println!("Electrónica:");
    for p in inventario.por_categoria("Electrónica") {
        println!("  - {} (${:.2})", p.nombre, p.precio);
    }

    // Stock bajo
    println!("\nStock bajo (<5):");
    for p in inventario.stock_bajo(5) {
        println!("  - {}: {} unidades", p.nombre, p.stock);
    }

    // Búsqueda
    println!("\nBuscar 'la':");
    for p in inventario.buscar("la") {
        println!("  - {}", p.nombre);
    }

    // Aplicar descuento
    println!("\nAplicando 10% descuento a Muebles...");
    inventario.aplicar_descuento("Muebles", 10.0);

    // Reporte completo
    println!("\n{}", inventario.reporte());
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_sumar_elementos() {
        assert_eq!(sumar_elementos(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sumar_elementos(&[]), 0);
    }

    #[test]
    fn test_duplicar_elementos() {
        let mut nums = vec![1, 2, 3];
        duplicar_elementos(&mut nums);
        assert_eq!(nums, vec![2, 4, 6]);
    }

    #[test]
    fn test_a_mayusculas() {
        let palabras = vec!["hola".to_string(), "mundo".to_string()];
        assert_eq!(a_mayusculas(palabras), vec!["HOLA", "MUNDO"]);
    }

    #[test]
    fn test_filtrar_positivos() {
        assert_eq!(filtrar_positivos(&[-2, -1, 0, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn test_encontrar_maximo() {
        assert_eq!(encontrar_maximo(&[3, 1, 4, 1, 5]), Some(5));
        assert_eq!(encontrar_maximo(&[]), None);
    }

    // Tests Ejercicio 2
    #[test]
    fn test_mapear_cuadrados() {
        assert_eq!(mapear_cuadrados(&[1, 2, 3]), vec![1, 4, 9]);
    }

    #[test]
    fn test_filtrar_pares() {
        assert_eq!(filtrar_pares(&[1, 2, 3, 4, 5]), vec![2, 4]);
    }

    #[test]
    fn test_cuadrados_de_pares() {
        assert_eq!(cuadrados_de_pares(&[1, 2, 3, 4]), vec![4, 16]);
    }

    #[test]
    fn test_combinar_pares() {
        assert_eq!(
            combinar_pares(&[1, 2], &[10, 20]),
            vec![(1, 10), (2, 20)]
        );
    }

    #[test]
    fn test_aplanar() {
        let anidado = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(aplanar(&anidado), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_primeros_n() {
        assert_eq!(primeros_n(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_sin_primeros_n() {
        assert_eq!(sin_primeros_n(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
    }

    #[test]
    fn test_tomar_mientras_positivo() {
        assert_eq!(tomar_mientras_positivo(&[3, 2, -1, 4]), vec![3, 2]);
    }

    // Tests Ejercicio 3
    #[test]
    fn test_suma_con_fold() {
        assert_eq!(suma_con_fold(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_producto_con_fold() {
        assert_eq!(producto_con_fold(&[1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_concatenar_con_separador() {
        assert_eq!(
            concatenar_con_separador(&["a", "b", "c"], "-"),
            "a-b-c"
        );
    }

    #[test]
    fn test_maximo_con_reduce() {
        assert_eq!(maximo_con_reduce(&[3, 7, 2, 9, 1]), Some(9));
        assert_eq!(maximo_con_reduce(&[]), None);
    }

    #[test]
    fn test_primer_par() {
        assert_eq!(primer_par(&[1, 3, 4, 5]), Some(4));
        assert_eq!(primer_par(&[1, 3, 5]), None);
    }

    #[test]
    fn test_posicion_mayor_que() {
        assert_eq!(posicion_mayor_que(&[1, 2, 5, 3], 4), Some(2));
        assert_eq!(posicion_mayor_que(&[1, 2, 3], 10), None);
    }

    #[test]
    fn test_alguno_negativo() {
        assert!(alguno_negativo(&[1, -2, 3]));
        assert!(!alguno_negativo(&[1, 2, 3]));
    }

    #[test]
    fn test_todos_positivos() {
        assert!(todos_positivos(&[1, 2, 3]));
        assert!(!todos_positivos(&[1, -2, 3]));
    }

    #[test]
    fn test_contar_pares() {
        assert_eq!(contar_pares(&[1, 2, 3, 4, 5, 6]), 3);
    }

    #[test]
    fn test_particionar_por_paridad() {
        let (pares, impares) = particionar_por_paridad(&[1, 2, 3, 4]);
        assert_eq!(pares, vec![2, 4]);
        assert_eq!(impares, vec![1, 3]);
    }

    // Tests Ejercicio 4
    #[test]
    fn test_producto_valor_inventario() {
        let p = Producto::new(1, "Test", 10.0, "Cat", 5);
        assert_eq!(p.valor_inventario(), 50.0);
    }

    #[test]
    fn test_inventario_por_categoria() {
        let inv = Inventario::new(vec![
            Producto::new(1, "A", 10.0, "X", 1),
            Producto::new(2, "B", 20.0, "Y", 1),
            Producto::new(3, "C", 30.0, "X", 1),
        ]);
        assert_eq!(inv.por_categoria("X").len(), 2);
    }

    #[test]
    fn test_inventario_stock_bajo() {
        let inv = Inventario::new(vec![
            Producto::new(1, "A", 10.0, "X", 2),
            Producto::new(2, "B", 20.0, "X", 10),
        ]);
        assert_eq!(inv.stock_bajo(5).len(), 1);
    }

    #[test]
    fn test_inventario_valor_total() {
        let inv = Inventario::new(vec![
            Producto::new(1, "A", 10.0, "X", 2),  // 20
            Producto::new(2, "B", 20.0, "X", 3),  // 60
        ]);
        assert_eq!(inv.valor_total(), 80.0);
    }

    #[test]
    fn test_inventario_mas_caro() {
        let inv = Inventario::new(vec![
            Producto::new(1, "A", 10.0, "X", 1),
            Producto::new(2, "B", 50.0, "X", 1),
        ]);
        assert_eq!(inv.mas_caro().unwrap().nombre, "B");
    }

    #[test]
    fn test_inventario_buscar() {
        let inv = Inventario::new(vec![
            Producto::new(1, "Laptop", 10.0, "X", 1),
            Producto::new(2, "Lampara", 20.0, "X", 1),
        ]);
        assert_eq!(inv.buscar("la").len(), 2);
    }

    #[test]
    fn test_inventario_aplicar_descuento() {
        let mut inv = Inventario::new(vec![
            Producto::new(1, "A", 100.0, "X", 1),
            Producto::new(2, "B", 200.0, "Y", 1),
        ]);
        inv.aplicar_descuento("X", 10.0);
        assert_eq!(inv.productos[0].precio, 90.0);
        assert_eq!(inv.productos[1].precio, 200.0);
    }
}
