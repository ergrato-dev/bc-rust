//! Solución - Proyecto: Pipeline de Procesamiento de Datos

fn main() {
    println!("=== Solución: Pipeline de Procesamiento ===\n");

    demo_numeros();
    demo_strings();
    demo_estadisticas();
    demo_pipeline_personalizado();

    println!("\n✅ Proyecto completado!");
}

// PARTE 1: Pipeline de Números
fn demo_numeros() {
    println!("--- Demo 1: Pipeline de Números ---");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = procesar_numeros(&numbers);
    println!("  Entrada: {:?}", numbers);
    println!("  Procesado: {:?}\n", result);
}

fn procesar_numeros(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&x| x % 2 == 0)  // pares
        .map(|x| x * 2)            // duplicar
        .map(|x| x + 10)           // sumar 10
        .collect()
}

// PARTE 2: Pipeline de Strings
fn demo_strings() {
    println!("--- Demo 2: Pipeline de Strings ---");
    let texts = vec!["  HOLA  ", "mundo", "", "  RUST  ", "   ", "programación"];
    let result = limpiar_textos(&texts);
    println!("  Entrada: {:?}", texts);
    println!("  Limpio: {:?}\n", result);
}

fn limpiar_textos(texts: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = texts
        .iter()
        .map(|s| s.trim())                    // quitar espacios
        .filter(|s| !s.is_empty())            // filtrar vacíos
        .map(|s| s.to_lowercase())            // minúsculas
        .collect();
    
    result.sort_by_key(|s| s.len());       // ordenar por longitud
    result
}

// PARTE 3: Pipeline con Estadísticas
fn demo_estadisticas() {
    println!("--- Demo 3: Pipeline con Estadísticas ---");
    let data = vec![10, 25, 30, 45, 50, 15, 80, 35];
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

fn calcular_estadisticas(data: &[i32]) -> Option<Estadisticas> {
    if data.is_empty() {
        return None;
    }

    let cantidad = data.len();
    let suma: i32 = data.iter().sum();
    let promedio = suma as f64 / cantidad as f64;
    let minimo = *data.iter().min()?;
    let maximo = *data.iter().max()?;

    Some(Estadisticas {
        cantidad,
        suma,
        promedio,
        minimo,
        maximo,
    })
}

// PARTE 4: Pipeline Configurable
fn demo_pipeline_personalizado() {
    println!("--- Demo 4: Pipeline Configurable ---");
    
    let mut pipeline: Pipeline<i32> = Pipeline::new();
    pipeline
        .add_stage(Box::new(|x| x * 2))
        .add_stage(Box::new(|x| x + 1))
        .add_stage(Box::new(|x| x * x));
    
    let data = vec![1, 2, 3, 4, 5];
    let result = pipeline.execute(&data);
    
    println!("  Entrada: {:?}", data);
    println!("  Pipeline (x*2+1)²: {:?}\n", result);
}

struct Pipeline<T> {
    stages: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: Copy> Pipeline<T> {
    fn new() -> Self {
        Pipeline { stages: Vec::new() }
    }

    fn add_stage(&mut self, stage: Box<dyn Fn(T) -> T>) -> &mut Self {
        self.stages.push(stage);
        self
    }

    fn execute(&self, data: &[T]) -> Vec<T> {
        data
            .iter()
            .map(|&value| {
                // Aplica todas las etapas secuencialmente
                self.stages.iter().fold(value, |acc, stage| stage(acc))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procesar_numeros() {
        let result = procesar_numeros(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(result, vec![14, 18, 22, 26, 30]);
    }

    #[test]
    fn test_limpiar_textos() {
        let texts = vec!["  ABC  ", "xy", "", "  Z  "];
        let result = limpiar_textos(&texts);
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
        assert_eq!(pipeline.execute(&[1, 2, 3]), vec![4, 6, 8]);
    }
}
