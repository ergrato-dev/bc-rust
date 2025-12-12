//! # Práctica 01: Vectores
//!
//! Ejercicios para dominar Vec<T> en Rust.

fn main() {
    println!("=== Práctica 01: Vectores ===\n");

    // Ejercicio 1: Operaciones básicas
    demo_operaciones_basicas();

    // Ejercicio 2: Estadísticas
    demo_estadisticas();

    // Ejercicio 3: Filtrado y transformación
    demo_filtrado();

    // Ejercicio 4: Gestión de inventario
    demo_inventario();
}

// ============================================================================
// EJERCICIO 1: Operaciones Básicas
// ============================================================================

/// Crea un vector con los números del 1 al n
fn crear_rango(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

/// Agrega un elemento al final si no existe
fn agregar_si_no_existe(vec: &mut Vec<i32>, valor: i32) -> bool {
    if vec.contains(&valor) {
        false
    } else {
        vec.push(valor);
        true
    }
}

/// Elimina la primera ocurrencia de un valor
fn eliminar_primera_ocurrencia(vec: &mut Vec<i32>, valor: i32) -> Option<i32> {
    if let Some(pos) = vec.iter().position(|&x| x == valor) {
        Some(vec.remove(pos))
    } else {
        None
    }
}

/// Obtiene el elemento en la posición de forma segura
fn obtener_seguro(vec: &[i32], indice: usize) -> Option<i32> {
    vec.get(indice).copied()
}

/// Actualiza el elemento en la posición si existe
fn actualizar_en_posicion(vec: &mut [i32], indice: usize, nuevo_valor: i32) -> bool {
    if let Some(elemento) = vec.get_mut(indice) {
        *elemento = nuevo_valor;
        true
    } else {
        false
    }
}

fn demo_operaciones_basicas() {
    println!("--- Ejercicio 1: Operaciones Básicas ---");

    let mut v = crear_rango(5);
    println!("Rango 1-5: {:?}", v);

    let agregado = agregar_si_no_existe(&mut v, 6);
    println!("Agregar 6: {} -> {:?}", agregado, v);

    let no_agregado = agregar_si_no_existe(&mut v, 3);
    println!("Agregar 3 (existe): {} -> {:?}", no_agregado, v);

    let eliminado = eliminar_primera_ocurrencia(&mut v, 3);
    println!("Eliminar 3: {:?} -> {:?}", eliminado, v);

    let valor = obtener_seguro(&v, 2);
    println!("Obtener índice 2: {:?}", valor);

    actualizar_en_posicion(&mut v, 0, 100);
    println!("Actualizar índice 0 a 100: {:?}", v);

    println!();
}

// ============================================================================
// EJERCICIO 2: Estadísticas
// ============================================================================

/// Calcula la suma de todos los elementos
fn suma(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

/// Calcula el promedio (devuelve None si está vacío)
fn promedio(vec: &[i32]) -> Option<f64> {
    if vec.is_empty() {
        None
    } else {
        Some(suma(vec) as f64 / vec.len() as f64)
    }
}

/// Encuentra el valor mínimo
fn minimo(vec: &[i32]) -> Option<i32> {
    vec.iter().min().copied()
}

/// Encuentra el valor máximo
fn maximo(vec: &[i32]) -> Option<i32> {
    vec.iter().max().copied()
}

/// Cuenta cuántos elementos cumplen una condición
fn contar_si<F>(vec: &[i32], predicado: F) -> usize
where
    F: Fn(&i32) -> bool,
{
    vec.iter().filter(|x| predicado(x)).count()
}

/// Estructura para almacenar estadísticas
#[derive(Debug)]
struct Estadisticas {
    suma: i32,
    promedio: f64,
    minimo: i32,
    maximo: i32,
    cantidad: usize,
}

/// Calcula todas las estadísticas de un vector
fn calcular_estadisticas(vec: &[i32]) -> Option<Estadisticas> {
    if vec.is_empty() {
        return None;
    }

    Some(Estadisticas {
        suma: suma(vec),
        promedio: promedio(vec).unwrap(),
        minimo: minimo(vec).unwrap(),
        maximo: maximo(vec).unwrap(),
        cantidad: vec.len(),
    })
}

fn demo_estadisticas() {
    println!("--- Ejercicio 2: Estadísticas ---");

    let numeros = vec![10, 25, 3, 47, 8, 32, 15];
    println!("Números: {:?}", numeros);

    if let Some(stats) = calcular_estadisticas(&numeros) {
        println!("Estadísticas: {:?}", stats);
    }

    let pares = contar_si(&numeros, |x| x % 2 == 0);
    let mayores_20 = contar_si(&numeros, |x| *x > 20);
    println!("Números pares: {}", pares);
    println!("Mayores a 20: {}", mayores_20);

    println!();
}

// ============================================================================
// EJERCICIO 3: Filtrado y Transformación
// ============================================================================

/// Filtra elementos que cumplen una condición
fn filtrar<F>(vec: &[i32], predicado: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    vec.iter().filter(|x| predicado(x)).copied().collect()
}

/// Transforma cada elemento aplicando una función
fn transformar<F>(vec: &[i32], transformacion: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.iter().map(|&x| transformacion(x)).collect()
}

/// Filtra y transforma en una sola operación
fn filtrar_y_transformar<P, T>(vec: &[i32], predicado: P, transformacion: T) -> Vec<i32>
where
    P: Fn(&i32) -> bool,
    T: Fn(i32) -> i32,
{
    vec.iter()
        .filter(|x| predicado(x))
        .map(|&x| transformacion(x))
        .collect()
}

/// Agrupa números en pares e impares
fn agrupar_par_impar(vec: &[i32]) -> (Vec<i32>, Vec<i32>) {
    vec.iter().partition(|x| *x % 2 == 0)
}

/// Elimina duplicados manteniendo el orden
fn eliminar_duplicados(vec: &[i32]) -> Vec<i32> {
    let mut vistos = Vec::new();
    for &n in vec {
        if !vistos.contains(&n) {
            vistos.push(n);
        }
    }
    vistos
}

fn demo_filtrado() {
    println!("--- Ejercicio 3: Filtrado y Transformación ---");

    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numeros);

    let pares = filtrar(&numeros, |x| x % 2 == 0);
    println!("Solo pares: {:?}", pares);

    let duplicados = transformar(&numeros, |x| x * 2);
    println!("Duplicados: {:?}", duplicados);

    let pares_al_cuadrado = filtrar_y_transformar(&numeros, |x| x % 2 == 0, |x| x * x);
    println!("Pares al cuadrado: {:?}", pares_al_cuadrado);

    let (pares_grupo, impares_grupo) = agrupar_par_impar(&numeros);
    println!("Pares: {:?}, Impares: {:?}", pares_grupo, impares_grupo);

    let con_duplicados = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let sin_duplicados = eliminar_duplicados(&con_duplicados);
    println!("Sin duplicados: {:?}", sin_duplicados);

    println!();
}

// ============================================================================
// EJERCICIO 4: Gestión de Inventario
// ============================================================================

#[derive(Debug, Clone)]
struct Producto {
    id: u32,
    nombre: String,
    precio: f64,
    cantidad: u32,
}

impl Producto {
    fn new(id: u32, nombre: &str, precio: f64, cantidad: u32) -> Self {
        Self {
            id,
            nombre: nombre.to_string(),
            precio,
            cantidad,
        }
    }

    fn valor_total(&self) -> f64 {
        self.precio * self.cantidad as f64
    }
}

struct Inventario {
    productos: Vec<Producto>,
    siguiente_id: u32,
}

impl Inventario {
    fn new() -> Self {
        Self {
            productos: Vec::new(),
            siguiente_id: 1,
        }
    }

    /// Agrega un nuevo producto
    fn agregar(&mut self, nombre: &str, precio: f64, cantidad: u32) -> u32 {
        let id = self.siguiente_id;
        self.productos
            .push(Producto::new(id, nombre, precio, cantidad));
        self.siguiente_id += 1;
        id
    }

    /// Busca un producto por ID
    fn buscar(&self, id: u32) -> Option<&Producto> {
        self.productos.iter().find(|p| p.id == id)
    }

    /// Busca un producto por nombre (parcial, case-insensitive)
    fn buscar_por_nombre(&self, nombre: &str) -> Vec<&Producto> {
        let nombre_lower = nombre.to_lowercase();
        self.productos
            .iter()
            .filter(|p| p.nombre.to_lowercase().contains(&nombre_lower))
            .collect()
    }

    /// Actualiza la cantidad de un producto
    fn actualizar_cantidad(&mut self, id: u32, cantidad: u32) -> bool {
        if let Some(producto) = self.productos.iter_mut().find(|p| p.id == id) {
            producto.cantidad = cantidad;
            true
        } else {
            false
        }
    }

    /// Elimina un producto por ID
    fn eliminar(&mut self, id: u32) -> Option<Producto> {
        if let Some(pos) = self.productos.iter().position(|p| p.id == id) {
            Some(self.productos.remove(pos))
        } else {
            None
        }
    }

    /// Productos con stock bajo (< umbral)
    fn stock_bajo(&self, umbral: u32) -> Vec<&Producto> {
        self.productos
            .iter()
            .filter(|p| p.cantidad < umbral)
            .collect()
    }

    /// Valor total del inventario
    fn valor_total(&self) -> f64 {
        self.productos.iter().map(|p| p.valor_total()).sum()
    }

    /// Productos ordenados por precio
    fn ordenados_por_precio(&self) -> Vec<&Producto> {
        let mut productos: Vec<_> = self.productos.iter().collect();
        productos.sort_by(|a, b| a.precio.partial_cmp(&b.precio).unwrap());
        productos
    }

    /// Cantidad total de productos
    fn cantidad_total(&self) -> u32 {
        self.productos.iter().map(|p| p.cantidad).sum()
    }
}

fn demo_inventario() {
    println!("--- Ejercicio 4: Gestión de Inventario ---");

    let mut inv = Inventario::new();

    // Agregar productos
    inv.agregar("Laptop HP", 899.99, 5);
    inv.agregar("Mouse Logitech", 29.99, 50);
    inv.agregar("Teclado Mecánico", 79.99, 25);
    inv.agregar("Monitor 27\"", 299.99, 3);
    inv.agregar("Laptop Dell", 999.99, 2);

    println!("Inventario inicial:");
    for p in &inv.productos {
        println!(
            "  #{} - {} (${:.2} x {} = ${:.2})",
            p.id,
            p.nombre,
            p.precio,
            p.cantidad,
            p.valor_total()
        );
    }

    println!("\nBúsqueda por ID 3: {:?}", inv.buscar(3).map(|p| &p.nombre));

    println!("\nBúsqueda 'laptop':");
    for p in inv.buscar_por_nombre("laptop") {
        println!("  - {}", p.nombre);
    }

    println!("\nStock bajo (< 5 unidades):");
    for p in inv.stock_bajo(5) {
        println!("  ⚠️  {} ({} unidades)", p.nombre, p.cantidad);
    }

    println!("\nValor total del inventario: ${:.2}", inv.valor_total());
    println!("Cantidad total de unidades: {}", inv.cantidad_total());

    println!("\nProductos ordenados por precio:");
    for p in inv.ordenados_por_precio() {
        println!("  ${:.2} - {}", p.precio, p.nombre);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_crear_rango() {
        assert_eq!(crear_rango(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(crear_rango(0), vec![]);
        assert_eq!(crear_rango(1), vec![1]);
    }

    #[test]
    fn test_agregar_si_no_existe() {
        let mut v = vec![1, 2, 3];
        assert!(agregar_si_no_existe(&mut v, 4));
        assert!(!agregar_si_no_existe(&mut v, 2));
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_eliminar_primera_ocurrencia() {
        let mut v = vec![1, 2, 3, 2, 4];
        assert_eq!(eliminar_primera_ocurrencia(&mut v, 2), Some(2));
        assert_eq!(v, vec![1, 3, 2, 4]);
        assert_eq!(eliminar_primera_ocurrencia(&mut v, 5), None);
    }

    #[test]
    fn test_obtener_seguro() {
        let v = vec![10, 20, 30];
        assert_eq!(obtener_seguro(&v, 1), Some(20));
        assert_eq!(obtener_seguro(&v, 10), None);
    }

    #[test]
    fn test_actualizar_en_posicion() {
        let mut v = vec![1, 2, 3];
        assert!(actualizar_en_posicion(&mut v, 1, 100));
        assert_eq!(v[1], 100);
        assert!(!actualizar_en_posicion(&mut v, 10, 999));
    }

    // Tests Ejercicio 2
    #[test]
    fn test_suma() {
        assert_eq!(suma(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(suma(&[]), 0);
    }

    #[test]
    fn test_promedio() {
        assert_eq!(promedio(&[2, 4, 6]), Some(4.0));
        assert_eq!(promedio(&[]), None);
    }

    #[test]
    fn test_minimo_maximo() {
        let v = vec![3, 1, 4, 1, 5];
        assert_eq!(minimo(&v), Some(1));
        assert_eq!(maximo(&v), Some(5));
        assert_eq!(minimo(&[]), None);
    }

    #[test]
    fn test_contar_si() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(contar_si(&v, |x| x % 2 == 0), 3);
        assert_eq!(contar_si(&v, |x| *x > 10), 0);
    }

    // Tests Ejercicio 3
    #[test]
    fn test_filtrar() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(filtrar(&v, |x| x % 2 == 0), vec![2, 4]);
    }

    #[test]
    fn test_transformar() {
        let v = vec![1, 2, 3];
        assert_eq!(transformar(&v, |x| x * 2), vec![2, 4, 6]);
    }

    #[test]
    fn test_filtrar_y_transformar() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(filtrar_y_transformar(&v, |x| x % 2 == 0, |x| x * 10), vec![20, 40]);
    }

    #[test]
    fn test_agrupar_par_impar() {
        let v = vec![1, 2, 3, 4, 5];
        let (pares, impares) = agrupar_par_impar(&v);
        assert_eq!(pares, vec![2, 4]);
        assert_eq!(impares, vec![1, 3, 5]);
    }

    #[test]
    fn test_eliminar_duplicados() {
        assert_eq!(eliminar_duplicados(&[1, 2, 2, 3, 1]), vec![1, 2, 3]);
    }

    // Tests Ejercicio 4
    #[test]
    fn test_inventario_agregar() {
        let mut inv = Inventario::new();
        let id1 = inv.agregar("Producto A", 10.0, 5);
        let id2 = inv.agregar("Producto B", 20.0, 3);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(inv.productos.len(), 2);
    }

    #[test]
    fn test_inventario_buscar() {
        let mut inv = Inventario::new();
        inv.agregar("Test", 10.0, 5);
        assert!(inv.buscar(1).is_some());
        assert!(inv.buscar(999).is_none());
    }

    #[test]
    fn test_inventario_buscar_por_nombre() {
        let mut inv = Inventario::new();
        inv.agregar("Laptop HP", 899.99, 5);
        inv.agregar("Laptop Dell", 999.99, 3);
        inv.agregar("Mouse", 29.99, 10);

        let laptops = inv.buscar_por_nombre("laptop");
        assert_eq!(laptops.len(), 2);
    }

    #[test]
    fn test_inventario_valor_total() {
        let mut inv = Inventario::new();
        inv.agregar("A", 10.0, 5);  // 50
        inv.agregar("B", 20.0, 2);  // 40
        assert!((inv.valor_total() - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_inventario_stock_bajo() {
        let mut inv = Inventario::new();
        inv.agregar("A", 10.0, 2);
        inv.agregar("B", 20.0, 10);
        inv.agregar("C", 30.0, 1);

        let bajo = inv.stock_bajo(5);
        assert_eq!(bajo.len(), 2);
    }

    #[test]
    fn test_inventario_eliminar() {
        let mut inv = Inventario::new();
        inv.agregar("Test", 10.0, 5);
        
        let eliminado = inv.eliminar(1);
        assert!(eliminado.is_some());
        assert_eq!(eliminado.unwrap().nombre, "Test");
        assert!(inv.productos.is_empty());
    }
}
