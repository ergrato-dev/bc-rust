// Práctica 01: Enums Básicos
// Semana 05 - Enums y Pattern Matching

// TODO: Define el enum DiaSemana con los 7 días
// Deriva: Debug, Clone, Copy, PartialEq
#[derive(Debug, Clone, Copy, PartialEq)]
enum DiaSemana {
    // Completa las variantes
}

// TODO: Implementa esta función
// Retorna true si es día laboral (lunes a viernes)
fn es_laboral(dia: DiaSemana) -> bool {
    todo!("Implementar usando match")
}

// TODO: Implementa métodos para DiaSemana
impl DiaSemana {
    // Retorna el siguiente día (Domingo -> Lunes)
    fn siguiente(&self) -> DiaSemana {
        todo!("Implementar ciclo de días")
    }
    
    // Retorna el nombre del día como &str
    fn nombre(&self) -> &str {
        todo!("Implementar con match")
    }
}

fn main() {
    let hoy = DiaSemana::Miercoles;
    
    println!("Hoy es: {:?}", hoy);
    println!("¿Es laboral?: {}", es_laboral(hoy));
    println!("Mañana es: {:?}", hoy.siguiente());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_laboral_lunes() {
        assert!(es_laboral(DiaSemana::Lunes));
    }

    #[test]
    fn test_es_laboral_viernes() {
        assert!(es_laboral(DiaSemana::Viernes));
    }

    #[test]
    fn test_no_es_laboral_sabado() {
        assert!(!es_laboral(DiaSemana::Sabado));
    }

    #[test]
    fn test_no_es_laboral_domingo() {
        assert!(!es_laboral(DiaSemana::Domingo));
    }

    #[test]
    fn test_siguiente_lunes() {
        assert_eq!(DiaSemana::Lunes.siguiente(), DiaSemana::Martes);
    }

    #[test]
    fn test_siguiente_domingo() {
        assert_eq!(DiaSemana::Domingo.siguiente(), DiaSemana::Lunes);
    }

    #[test]
    fn test_nombre_miercoles() {
        assert_eq!(DiaSemana::Miercoles.nombre(), "Miércoles");
    }
}
