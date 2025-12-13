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
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let resultado = procesar_numeros(&numeros);
    println!("  Entrada: {:?}", numeros);
    println!("  Procesado: {:?}\n", resultado);
}

fn procesar_numeros(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter()
        .filter(|&x| x % 2 == 0)  // pares
        .map(|x| x * 2)            // duplicar
        .map(|x| x + 10)           // sumar 10
        .collect()
}

// PARTE 2: Pipeline de Strings
fn demo_strings() {
    println!("--- Demo 2: Pipeline de Strings ---");
    let textos = vec!["  HOLA  ", "mundo", "", "  RUST  ", "   ", "programación"];
    let resultado = limpiar_textos(&textos);
    println!("  Entrada: {:?}", textos);
    println!("  Limpio: {:?}\n", resultado);
}

fn limpiar_textos(textos: &[&str]) -> Vec<String> {
    let mut resultado: Vec<String> = textos
        .iter()
        .map(|s| s.trim())                    // quitar espacios
        .filter(|s| !s.is_empty())            // filtrar vacíos
        .map(|s| s.to_lowercase())            // minúsculas
        .collect();
    
    resultado.sort_by_key(|s| s.len());       // ordenar por longitud
    resultado
}

// PARTE 3: Pipeline con Estadísticas
fn demo_estadisticas() {
    println!("--- Demo 3: Pipeline con Estadísticas ---");
    let datos = vec![10, 25, 30, 45, 50, 15, 80, 35];
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

fn calcular_estadisticas(datos: &[i32]) -> Option<Estadisticas> {
    if datos.is_empty() {
        return None;
    }

    let cantidad = datos.len();
    let suma: i32 = datos.iter().sum();
    let promedio = suma as f64 / cantidad as f64;
    let minimo = *datos.iter().min()?;
    let maximo = *datos.iter().max()?;

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
        .agregar_etapa(Box::new(|x| x * 2))
        .agregar_etapa(Box::new(|x| x + 1))
        .agregar_etapa(Box::new(|x| x * x));
    
    let datos = vec![1, 2, 3, 4, 5];
    let resultado = pipeline.ejecutar(&datos);
    
    println!("  Entrada: {:?}", datos);
    println!("  Pipeline (x*2+1)²: {:?}\n", resultado);
}

struct Pipeline<T> {
    etapas: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: Copy> Pipeline<T> {
    fn new() -> Self {
        Pipeline { etapas: Vec::new() }
    }

    fn agregar_etapa(&mut self, etapa: Box<dyn Fn(T) -> T>) -> &mut Self {
        self.etapas.push(etapa);
        self
    }

    fn ejecutar(&self, datos: &[T]) -> Vec<T> {
        datos
            .iter()
            .map(|&valor| {
                // Aplica todas las etapas secuencialmente
                self.etapas.iter().fold(valor, |acc, etapa| etapa(acc))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procesar_numeros() {
        let resultado = procesar_numeros(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(resultado, vec![14, 18, 22, 26, 30]);
    }

    #[test]
    fn test_limpiar_textos() {
        let textos = vec!["  ABC  ", "xy", "", "  Z  "];
        let resultado = limpiar_textos(&textos);
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
        assert_eq!(pipeline.ejecutar(&[1, 2, 3]), vec![4, 6, 8]);
    }
}
