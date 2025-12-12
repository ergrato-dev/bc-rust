// ============================================
// Práctica 03: Constructores
// ============================================
// Objetivo: Funciones asociadas y new()
// ============================================

struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

impl Usuario {
    // -----------------------------------------
    // PARTE 1: Constructor Básico
    // -----------------------------------------
    
    /// Constructor principal
    fn new(nombre: String, email: String) -> Self {
        Self {
            nombre,
            email,
            edad: 0,
            activo: true,
        }
    }

    // -----------------------------------------
    // PARTE 2: Múltiples Constructores
    // -----------------------------------------
    
    /// Constructor con todos los datos
    fn con_edad(nombre: String, email: String, edad: u32) -> Self {
        Self {
            nombre,
            email,
            edad,
            activo: true,
        }
    }

    /// Constructor para usuario anónimo
    fn anonimo() -> Self {
        Self {
            nombre: String::from("Anónimo"),
            email: String::from("anonimo@temp.com"),
            edad: 0,
            activo: false,
        }
    }

    // TODO: Implementa un constructor desde_email que extraiga el nombre del email
    // Ejemplo: "juan.perez@mail.com" -> nombre = "juan.perez"
    // fn desde_email(email: String) -> Self { ... }

    // -----------------------------------------
    // PARTE 3: Validación (Avanzado)
    // -----------------------------------------
    
    /// Constructor con validación de edad
    /// Retorna None si la edad es mayor a 150
    fn validado(nombre: String, email: String, edad: u32) -> Option<Self> {
        if edad > 150 {
            None
        } else {
            Some(Self {
                nombre,
                email,
                edad,
                activo: true,
            })
        }
    }

    // -----------------------------------------
    // Métodos de la instancia
    // -----------------------------------------
    
    fn mostrar(&self) {
        println!(
            "Usuario: {} ({}) - {} años - {}",
            self.nombre,
            self.email,
            self.edad,
            if self.activo { "activo" } else { "inactivo" }
        );
    }

    fn cumplir_anios(&mut self) {
        self.edad += 1;
    }

    fn desactivar(&mut self) {
        self.activo = false;
    }
}

fn main() {
    println!("=== Práctica 03: Constructores ===\n");

    // -----------------------------------------
    // Usando new()
    // -----------------------------------------
    println!("--- Constructor new() ---");
    
    let mut usuario1 = Usuario::new(
        String::from("Ana García"),
        String::from("ana@email.com"),
    );
    usuario1.mostrar();

    // -----------------------------------------
    // Usando con_edad()
    // -----------------------------------------
    println!("\n--- Constructor con_edad() ---");
    
    let usuario2 = Usuario::con_edad(
        String::from("Carlos López"),
        String::from("carlos@email.com"),
        30,
    );
    usuario2.mostrar();

    // -----------------------------------------
    // Usando anonimo()
    // -----------------------------------------
    println!("\n--- Constructor anonimo() ---");
    
    let anonimo = Usuario::anonimo();
    anonimo.mostrar();

    // -----------------------------------------
    // Usando validado()
    // -----------------------------------------
    println!("\n--- Constructor validado() ---");
    
    // Caso válido
    if let Some(usuario) = Usuario::validado(
        String::from("María"),
        String::from("maria@email.com"),
        25,
    ) {
        println!("Usuario creado:");
        usuario.mostrar();
    }

    // Caso inválido
    match Usuario::validado(
        String::from("Imposible"),
        String::from("imp@email.com"),
        200,
    ) {
        Some(u) => u.mostrar(),
        None => println!("Error: edad inválida (> 150)"),
    }

    // -----------------------------------------
    // Modificar con métodos
    // -----------------------------------------
    println!("\n--- Modificar Usuario ---");
    
    usuario1.cumplir_anios();
    usuario1.cumplir_anios();
    println!("Después de 2 cumpleaños:");
    usuario1.mostrar();

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let u = Usuario::new(String::from("Test"), String::from("test@t.com"));
        
        assert_eq!(u.nombre, "Test");
        assert_eq!(u.email, "test@t.com");
        assert_eq!(u.edad, 0);
        assert!(u.activo);
    }

    #[test]
    fn test_con_edad() {
        let u = Usuario::con_edad(
            String::from("Test"),
            String::from("test@t.com"),
            25,
        );
        
        assert_eq!(u.edad, 25);
    }

    #[test]
    fn test_anonimo() {
        let u = Usuario::anonimo();
        
        assert_eq!(u.nombre, "Anónimo");
        assert!(!u.activo);
    }

    #[test]
    fn test_validado_ok() {
        let u = Usuario::validado(
            String::from("Test"),
            String::from("t@t.com"),
            50,
        );
        
        assert!(u.is_some());
    }

    #[test]
    fn test_validado_error() {
        let u = Usuario::validado(
            String::from("Test"),
            String::from("t@t.com"),
            200,
        );
        
        assert!(u.is_none());
    }

    #[test]
    fn test_cumplir_anios() {
        let mut u = Usuario::new(String::from("T"), String::from("t@t.com"));
        u.cumplir_anios();
        
        assert_eq!(u.edad, 1);
    }
}
