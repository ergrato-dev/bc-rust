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
    
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // TODO: Implementa procesar_numeros
    // Debe: filtrar pares, multiplicar por 2, sumar 10
    let resultado = procesar_numeros(&numeros);
    
    println!("  Entrada: {:?}", numeros);
    println!("  Procesado (pares * 2 + 10): {:?}\n", resultado);
}

/// Procesa números: filtra pares, multiplica por 2, suma 10
fn procesar_numeros(numeros: &[i32]) -> Vec<i32> {
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
    
    let textos = vec![
        "  HOLA  ",
        "mundo",
        "",
        "  RUST  ",
        "   ",
        "programación",
    ];
    
    // TODO: Implementa limpiar_textos
    let resultado = limpiar_textos(&textos);
    
    println!("  Entrada: {:?}", textos);
    println!("  Limpio: {:?}\n", resultado);
}

/// Limpia textos: trim, lowercase, filtra vacíos, ordena por longitud
fn limpiar_textos(textos: &[&str]) -> Vec<String> {
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
    
    let datos = vec![10, 25, 30, 45, 50, 15, 80, 35];
    
    // TODO: Implementa calcular_estadisticas
    let stats = calcular_estadisticas(&datos);
    
    println!("  Datos: {:?}", datos);
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
fn calcular_estadisticas(datos: &[i32]) -> Option<Estadisticas> {
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
        .agregar_etapa(Box::new(|x| x * 2))      // duplicar
        .agregar_etapa(Box::new(|x| x + 1))      // sumar 1
        .agregar_etapa(Box::new(|x| x * x));     // elevar al cuadrado
    
    let datos = vec![1, 2, 3, 4, 5];
    let resultado = pipeline.ejecutar(&datos);
    
    println!("  Entrada: {:?}", datos);
    println!("  Pipeline (x*2+1)²: {:?}\n", resultado);
}

/// Pipeline configurable con etapas dinámicas
struct Pipeline<T> {
    etapas: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: Copy> Pipeline<T> {
    fn new() -> Self {
        // TODO: Implementa constructor
        todo!("Implementa Pipeline::new")
    }

    fn agregar_etapa(&mut self, etapa: Box<dyn Fn(T) -> T>) -> &mut Self {
        // TODO: Añade etapa y retorna &mut self para encadenar
        todo!("Implementa agregar_etapa")
    }

    fn ejecutar(&self, datos: &[T]) -> Vec<T> {
        // TODO: Aplica todas las etapas a cada elemento
        // Usa fold para aplicar cada etapa secuencialmente
        todo!("Implementa ejecutar")
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
        let resultado = procesar_numeros(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(resultado, vec![14, 18, 22, 26, 30]);
    }

    #[test]
    fn test_limpiar_textos() {
        let textos = vec!["  ABC  ", "xy", "", "  Z  "];
        let resultado = limpiar_textos(&textos);
        // Ordenados por longitud: z(1), xy(2), abc(3)
        assert_eq!(resultado, vec!["z", "xy", "abc"]);
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
            .agregar_etapa(Box::new(|x| x + 1))
            .agregar_etapa(Box::new(|x| x * 2));
        
        // (1+1)*2=4, (2+1)*2=6, (3+1)*2=8
        assert_eq!(pipeline.ejecutar(&[1, 2, 3]), vec![4, 6, 8]);
    }
}
