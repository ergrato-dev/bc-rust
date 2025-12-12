// Práctica 04: Patrones Avanzados
// Semana 05 - Enums y Pattern Matching

// ============================================
// Ejercicio 1: if let y while let
// ============================================

#[derive(Debug)]
enum Mensaje {
    Texto(String),
    Numero(i32),
    Vacio,
}

// TODO: Imprime solo si es Texto, usando if let
fn imprimir_si_texto(msg: &Mensaje) {
    todo!("Usar if let")
}

// TODO: Procesa una pila hasta vaciarla, usando while let
fn procesar_pila(pila: &mut Vec<i32>) -> i32 {
    let mut suma = 0;
    todo!("Usar while let con pop()")
    // Retorna la suma de todos los elementos
}

// TODO: Extrae todos los textos de un vector de mensajes
fn extraer_textos(mensajes: Vec<Mensaje>) -> Vec<String> {
    let mut textos = Vec::new();
    todo!("Usar for con if let o filter_map")
}

// ============================================
// Ejercicio 2: let else (Early Return)
// ============================================

#[derive(Debug)]
struct Config {
    puerto: Option<u16>,
    host: Option<String>,
    timeout: Option<u32>,
}

// TODO: Usa let else para validar config
// Retorna error string si algún campo es None
fn validar_config(config: &Config) -> Result<String, &'static str> {
    todo!("Usar let else para extraer cada campo o retornar error")
    // Si todo está bien, retorna Ok("Config válida: host:puerto, timeout=X")
}

// TODO: Obtiene el primer elemento mayor a 10
fn primer_mayor_a_diez(numeros: &[i32]) -> Option<i32> {
    todo!("Usar let else o ? operator")
}

// ============================================
// Ejercicio 3: Binding con @
// ============================================

#[derive(Debug)]
enum Evento {
    Click { x: i32, y: i32 },
    Tecla(char),
    Scroll(f64),
}

// TODO: Clasifica el evento usando @ para capturar valores en rangos
fn describir_evento(evento: &Evento) -> String {
    // Click en zona (0-100, 0-100): "Click en esquina superior izquierda en (x, y)"
    // Click en otra zona: "Click en (x, y)"
    // Tecla letra minúscula: "Tecla letra: X"
    // Tecla letra mayúscula: "Tecla MAYÚSCULA: X"
    // Tecla otro: "Tecla especial: X"
    // Scroll positivo: "Scroll arriba: X"
    // Scroll negativo: "Scroll abajo: X"
    todo!("Usar @ para capturar valores mientras verificas patrones")
}

// TODO: Clasifica edad usando @ binding
fn clasificar_edad(edad: u8) -> &'static str {
    todo!("Usar edad @ 0..=2 => ... etc")
    // 0-2: "bebé"
    // 3-12: "niño"
    // 13-19: "adolescente"
    // 20-64: "adulto"
    // 65+: "senior"
}

// ============================================
// Ejercicio 4: matches! Macro
// ============================================

#[derive(Debug)]
enum Estado {
    Activo,
    Inactivo,
    Pendiente,
    Error(String),
}

// TODO: Usa matches! para verificar si está activo
fn esta_activo(estado: &Estado) -> bool {
    todo!("Usar matches!")
}

// TODO: Usa matches! para verificar si es error
fn es_error(estado: &Estado) -> bool {
    todo!("Usar matches! con patrón")
}

// TODO: Cuenta cuántos elementos son activos
fn contar_activos(estados: &[Estado]) -> usize {
    todo!("Usar filter con matches!")
}

// TODO: Verifica si un número está en rango válido
fn en_rango_valido(n: i32) -> bool {
    todo!("Usar matches! con rango 1..=100")
}

fn main() {
    // if let demo
    let msg = Mensaje::Texto("Hola".to_string());
    imprimir_si_texto(&msg);

    // while let demo
    let mut pila = vec![1, 2, 3, 4, 5];
    println!("Suma: {}", procesar_pila(&mut pila));

    // let else demo
    let config = Config {
        puerto: Some(8080),
        host: Some("localhost".to_string()),
        timeout: Some(30),
    };
    println!("{:?}", validar_config(&config));

    // @ binding demo
    let evento = Evento::Click { x: 50, y: 50 };
    println!("{}", describir_evento(&evento));

    // matches! demo
    let estado = Estado::Activo;
    println!("¿Activo? {}", esta_activo(&estado));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procesar_pila() {
        let mut pila = vec![1, 2, 3, 4, 5];
        assert_eq!(procesar_pila(&mut pila), 15);
        assert!(pila.is_empty());
    }

    #[test]
    fn test_extraer_textos() {
        let mensajes = vec![
            Mensaje::Texto("Hola".to_string()),
            Mensaje::Numero(42),
            Mensaje::Texto("Mundo".to_string()),
            Mensaje::Vacio,
        ];
        let textos = extraer_textos(mensajes);
        assert_eq!(textos, vec!["Hola", "Mundo"]);
    }

    #[test]
    fn test_validar_config_ok() {
        let config = Config {
            puerto: Some(8080),
            host: Some("localhost".to_string()),
            timeout: Some(30),
        };
        assert!(validar_config(&config).is_ok());
    }

    #[test]
    fn test_validar_config_sin_puerto() {
        let config = Config {
            puerto: None,
            host: Some("localhost".to_string()),
            timeout: Some(30),
        };
        assert!(validar_config(&config).is_err());
    }

    #[test]
    fn test_primer_mayor_a_diez() {
        assert_eq!(primer_mayor_a_diez(&[5, 8, 15, 3]), Some(15));
        assert_eq!(primer_mayor_a_diez(&[1, 2, 3]), None);
    }

    #[test]
    fn test_clasificar_edad() {
        assert_eq!(clasificar_edad(1), "bebé");
        assert_eq!(clasificar_edad(8), "niño");
        assert_eq!(clasificar_edad(16), "adolescente");
        assert_eq!(clasificar_edad(30), "adulto");
        assert_eq!(clasificar_edad(70), "senior");
    }

    #[test]
    fn test_esta_activo() {
        assert!(esta_activo(&Estado::Activo));
        assert!(!esta_activo(&Estado::Inactivo));
    }

    #[test]
    fn test_es_error() {
        assert!(es_error(&Estado::Error("fallo".to_string())));
        assert!(!es_error(&Estado::Activo));
    }

    #[test]
    fn test_contar_activos() {
        let estados = vec![
            Estado::Activo,
            Estado::Inactivo,
            Estado::Activo,
            Estado::Pendiente,
        ];
        assert_eq!(contar_activos(&estados), 2);
    }

    #[test]
    fn test_en_rango_valido() {
        assert!(en_rango_valido(50));
        assert!(!en_rango_valido(0));
        assert!(!en_rango_valido(101));
    }
}
