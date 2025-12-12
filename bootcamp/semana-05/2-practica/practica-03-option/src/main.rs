// Práctica 03: Option y Métodos
// Semana 05 - Enums y Pattern Matching

// ============================================
// Ejercicio 1: Búsqueda con Option
// ============================================

#[derive(Debug, Clone)]
struct Usuario {
    id: u32,
    nombre: String,
    activo: bool,
}

// TODO: Busca un usuario por ID
// Retorna Some(&Usuario) si existe, None si no
fn buscar_usuario(usuarios: &[Usuario], id: u32) -> Option<&Usuario> {
    todo!("Implementar búsqueda")
}

// TODO: Busca un usuario activo por ID
fn buscar_usuario_activo(usuarios: &[Usuario], id: u32) -> Option<&Usuario> {
    todo!("Usar buscar_usuario y filtrar por activo")
}

// ============================================
// Ejercicio 2: Métodos de Option
// ============================================

// TODO: Obtiene el nombre del usuario o "Anónimo"
fn obtener_nombre(usuarios: &[Usuario], id: u32) -> String {
    todo!("Usar map y unwrap_or")
}

// TODO: Obtiene el nombre en mayúsculas si existe
fn obtener_nombre_mayusculas(usuarios: &[Usuario], id: u32) -> Option<String> {
    todo!("Usar map para transformar")
}

// TODO: Busca un usuario y luego busca su amigo
fn buscar_amigo(usuarios: &[Usuario], id: u32, amigos: &[(u32, u32)]) -> Option<&Usuario> {
    // amigos es una lista de (usuario_id, amigo_id)
    todo!("Usar and_then para encadenar búsquedas")
}

// ============================================
// Ejercicio 3: Option en Structs
// ============================================

#[derive(Debug)]
struct Perfil {
    nombre: String,
    email: Option<String>,
    telefono: Option<String>,
    edad: Option<u8>,
}

impl Perfil {
    fn nuevo(nombre: &str) -> Self {
        Perfil {
            nombre: nombre.to_string(),
            email: None,
            telefono: None,
            edad: None,
        }
    }

    // TODO: Setter que retorna Self para encadenar
    fn con_email(mut self, email: &str) -> Self {
        todo!("Establecer email y retornar self")
    }

    fn con_telefono(mut self, telefono: &str) -> Self {
        todo!("Establecer telefono y retornar self")
    }

    fn con_edad(mut self, edad: u8) -> Self {
        todo!("Establecer edad y retornar self")
    }

    // TODO: Obtener email o un valor por defecto
    fn email_o_default(&self) -> &str {
        todo!("Usar as_ref y unwrap_or")
    }

    // TODO: Verificar si el perfil está completo
    fn esta_completo(&self) -> bool {
        todo!("Verificar que todos los Option son Some")
    }

    // TODO: Obtener información de contacto
    fn info_contacto(&self) -> String {
        todo!("Combinar email y telefono disponibles")
    }
}

// ============================================
// Ejercicio 4: Encadenamiento con ?
// ============================================

// TODO: Obtener el primer carácter del email
fn primer_caracter_email(perfil: &Perfil) -> Option<char> {
    todo!("Usar ? para propagar None")
}

fn main() {
    let usuarios = vec![
        Usuario { id: 1, nombre: "Ana".to_string(), activo: true },
        Usuario { id: 2, nombre: "Bob".to_string(), activo: false },
        Usuario { id: 3, nombre: "Carlos".to_string(), activo: true },
    ];

    // Búsqueda básica
    if let Some(u) = buscar_usuario(&usuarios, 1) {
        println!("Encontrado: {:?}", u);
    }

    // Nombre con fallback
    println!("Usuario 1: {}", obtener_nombre(&usuarios, 1));
    println!("Usuario 99: {}", obtener_nombre(&usuarios, 99));

    // Perfil con builder pattern
    let perfil = Perfil::nuevo("Diana")
        .con_email("diana@email.com")
        .con_edad(28);

    println!("Email: {}", perfil.email_o_default());
    println!("Completo: {}", perfil.esta_completo());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_usuarios() -> Vec<Usuario> {
        vec![
            Usuario { id: 1, nombre: "Ana".to_string(), activo: true },
            Usuario { id: 2, nombre: "Bob".to_string(), activo: false },
            Usuario { id: 3, nombre: "Carlos".to_string(), activo: true },
        ]
    }

    #[test]
    fn test_buscar_usuario_existente() {
        let usuarios = crear_usuarios();
        let resultado = buscar_usuario(&usuarios, 1);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().nombre, "Ana");
    }

    #[test]
    fn test_buscar_usuario_no_existente() {
        let usuarios = crear_usuarios();
        assert!(buscar_usuario(&usuarios, 99).is_none());
    }

    #[test]
    fn test_buscar_usuario_activo() {
        let usuarios = crear_usuarios();
        assert!(buscar_usuario_activo(&usuarios, 1).is_some());
        assert!(buscar_usuario_activo(&usuarios, 2).is_none()); // Bob no está activo
    }

    #[test]
    fn test_obtener_nombre_existente() {
        let usuarios = crear_usuarios();
        assert_eq!(obtener_nombre(&usuarios, 1), "Ana");
    }

    #[test]
    fn test_obtener_nombre_no_existente() {
        let usuarios = crear_usuarios();
        assert_eq!(obtener_nombre(&usuarios, 99), "Anónimo");
    }

    #[test]
    fn test_obtener_nombre_mayusculas() {
        let usuarios = crear_usuarios();
        assert_eq!(obtener_nombre_mayusculas(&usuarios, 1), Some("ANA".to_string()));
        assert!(obtener_nombre_mayusculas(&usuarios, 99).is_none());
    }

    #[test]
    fn test_perfil_builder() {
        let perfil = Perfil::nuevo("Test")
            .con_email("test@test.com")
            .con_telefono("123456");
        
        assert_eq!(perfil.email, Some("test@test.com".to_string()));
        assert_eq!(perfil.telefono, Some("123456".to_string()));
    }

    #[test]
    fn test_perfil_email_default() {
        let perfil = Perfil::nuevo("Test");
        assert_eq!(perfil.email_o_default(), "sin email");
    }

    #[test]
    fn test_perfil_completo() {
        let incompleto = Perfil::nuevo("Test").con_email("a@b.com");
        let completo = Perfil::nuevo("Test")
            .con_email("a@b.com")
            .con_telefono("123")
            .con_edad(25);
        
        assert!(!incompleto.esta_completo());
        assert!(completo.esta_completo());
    }

    #[test]
    fn test_primer_caracter_email() {
        let con_email = Perfil::nuevo("Test").con_email("abc@test.com");
        let sin_email = Perfil::nuevo("Test");
        
        assert_eq!(primer_caracter_email(&con_email), Some('a'));
        assert!(primer_caracter_email(&sin_email).is_none());
    }
}
