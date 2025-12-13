//! Proyecto: Pipeline de Procesamiento de Datos
//!
//! Implementa un sistema de pipeline configurable que procese datos
//! usando closures e iteradores. El pipeline debe ser:
//! - Configurable: añadir etapas dinámicamente
//! - Lazy: no procesar hasta que se consuma
//! - Composable: encadenar múltiples transformaciones

fn main() {
    println!("=== Proyecto: Pipeline de Procesamiento ===\n");

    // Demo 1: Pipeline de números
    demo_numeros();

    // Demo 2: Pipeline de strings
    demo_strings();

    // Demo 3: Pipeline con estadísticas
    demo_estadisticas();

    // Demo 4: Pipeline personalizado
    demo_pipeline_personalizado();

    println!("\n✅ Proyecto completado!");
}

// ============================================================
// PARTE 1: Pipeline Básico de Números (8 puntos)
// ============================================================

fn demo_numeros() {
    println!("--- Demo 1: Pipeline de Números ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // TODO: Implementa procesar_numeros
    // Debe: filtrar pares, multiplicar por 2, sumar 10
    let result = procesar_numeros(&numbers);
    
    println!("  Entrada: {:?}", numbers);
    println!("  Procesado (pares * 2 + 10): {:?}\n", result);
}

/// Procesa números: filtra pares, multiplica por 2, suma 10
fn procesar_numeros(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implementa el pipeline
    // 1. Filtra números pares
    // 2. Multiplica cada uno por 2
    // 3. Suma 10 a cada resultado
    todo!("Implementa procesar_numeros")
}

// ============================================================
// PARTE 2: Pipeline de Strings (8 puntos)
// ============================================================

fn demo_strings() {
    println!("--- Demo 2: Pipeline de Strings ---");
    
    let texts = vec![
        "  HOLA  ",
        "mundo",
        "",
        "  RUST  ",
        "   ",
        "programación",
    ];
    
    // TODO: Implementa limpiar_textos
    let result = limpiar_textos(&texts);
    
    println!("  Entrada: {:?}", texts);
    println!("  Limpio: {:?}\n", result);
}

/// Limpia textos: trim, lowercase, filtra vacíos, ordena por longitud
fn limpiar_textos(texts: &[&str]) -> Vec<String> {
    // TODO: Implementa el pipeline
    // 1. Trim (quitar espacios)
    // 2. Filtrar strings vacíos
    // 3. Convertir a lowercase
    // 4. Ordenar por longitud (más cortos primero)
    todo!("Implementa limpiar_textos")
}

// ============================================================
// PARTE 3: Pipeline con Estadísticas (7 puntos)
// ============================================================

fn demo_estadisticas() {
    println!("--- Demo 3: Pipeline con Estadísticas ---");
    
    let data = vec![10, 25, 30, 45, 50, 15, 80, 35];
    
    // TODO: Implementa calcular_estadisticas
    let stats = calcular_estadisticas(&data);
    
    println!("  Datos: {:?}", data);
    println!("  Estadísticas: {:?}\n", stats);
}

#[derive(Debug, PartialEq)]
struct Estadisticas {
    cantidad: usize,
    suma: i32,
    promedio: f64,
    minimo: i32,
    maximo: i32,
}

/// Calcula estadísticas usando iteradores
fn calcular_estadisticas(data: &[i32]) -> Option<Estadisticas> {
    // TODO: Implementa usando iteradores
    // Retorna None si datos está vacío
    // Usa: count, sum, min, max
    // Para promedio: suma / cantidad
    todo!("Implementa calcular_estadisticas")
}

// ============================================================
// PARTE 4: Pipeline Configurable (7 puntos)
// ============================================================

fn demo_pipeline_personalizado() {
    println!("--- Demo 4: Pipeline Configurable ---");
    
    // TODO: Implementa Pipeline
    let mut pipeline: Pipeline<i32> = Pipeline::new();
    
    // Añadir etapas
    pipeline
        .add_stage(Box::new(|x| x * 2))      // duplicar
        .add_stage(Box::new(|x| x + 1))      // sumar 1
        .add_stage(Box::new(|x| x * x));     // elevar al cuadrado
    
    let data = vec![1, 2, 3, 4, 5];
    let result = pipeline.execute(&data);
    
    println!("  Entrada: {:?}", data);
    println!("  Pipeline (x*2+1)²: {:?}\n", result);
}

/// Pipeline configurable con etapas dinámicas
struct Pipeline<T> {
    stages: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: Copy> Pipeline<T> {
    fn new() -> Self {
        // TODO: Implementa constructor
        todo!("Implementa Pipeline::new")
    }

    fn add_stage(&mut self, stage: Box<dyn Fn(T) -> T>) -> &mut Self {
        // TODO: Añade stage y retorna &mut self para encadenar
        todo!("Implementa add_stage")
    }

    fn execute(&self, data: &[T]) -> Vec<T> {
        // TODO: Aplica todas las etapas a cada elemento
        // Usa fold para aplicar cada stage secuencialmente
        todo!("Implementa execute")
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procesar_numeros() {
        // pares: 2,4,6,8,10 -> *2: 4,8,12,16,20 -> +10: 14,18,22,26,30
        let result = procesar_numeros(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(result, vec![14, 18, 22, 26, 30]);
    }

    #[test]
    fn test_limpiar_textos() {
        let texts = vec!["  ABC  ", "xy", "", "  Z  "];
        let result = limpiar_textos(&texts);
        // Ordenados por longitud: z(1), xy(2), abc(3)
        assert_eq!(result, vec!["z", "xy", "abc"]);
    }

    #[test]
    fn test_estadisticas() {
        let stats = calcular_estadisticas(&[10, 20, 30]).unwrap();
        assert_eq!(stats.cantidad, 3);
        assert_eq!(stats.suma, 60);
        assert_eq!(stats.promedio, 20.0);
        assert_eq!(stats.minimo, 10);
        assert_eq!(stats.maximo, 30);
    }

    #[test]
    fn test_estadisticas_vacio() {
        assert!(calcular_estadisticas(&[]).is_none());
    }

    #[test]
    fn test_pipeline() {
        let mut pipeline: Pipeline<i32> = Pipeline::new();
        pipeline
            .add_stage(Box::new(|x| x + 1))
            .add_stage(Box::new(|x| x * 2));
        
        // (1+1)*2=4, (2+1)*2=6, (3+1)*2=8
        assert_eq!(pipeline.execute(&[1, 2, 3]), vec![4, 6, 8]);
    }
}
