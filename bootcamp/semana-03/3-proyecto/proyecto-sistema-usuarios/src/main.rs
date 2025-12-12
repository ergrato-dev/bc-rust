// ============================================
// Proyecto Semanal: Sistema de Usuarios
// ============================================
// Semana 03: Structs y Métodos
// ============================================

// ============================================
// NIVEL 1: STRUCTS BÁSICOS
// ============================================

/// ID único de usuario (newtype pattern)
#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(u64);

impl UserId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    fn valor(&self) -> u64 {
        self.0
    }
}

/// Representa un usuario del sistema
struct Usuario {
    id: UserId,
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

impl Usuario {
    // -----------------------------------------
    // Constructores
    // -----------------------------------------
    
    /// Constructor principal
    fn new(id: u64, nombre: String, email: String) -> Self {
        Self {
            id: UserId::new(id),
            nombre,
            email,
            edad: 0,
            activo: true,
        }
    }

    /// Constructor con todos los datos
    fn completo(id: u64, nombre: String, email: String, edad: u32) -> Self {
        Self {
            id: UserId::new(id),
            nombre,
            email,
            edad,
            activo: true,
        }
    }

    // -----------------------------------------
    // Métodos de Lectura
    // -----------------------------------------
    
    fn id(&self) -> UserId {
        self.id
    }

    fn nombre(&self) -> &str {
        &self.nombre
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn edad(&self) -> u32 {
        self.edad
    }

    fn esta_activo(&self) -> bool {
        self.activo
    }

    fn es_adulto(&self) -> bool {
        self.edad >= 18
    }

    fn mostrar(&self) {
        println!(
            "[{}] {} ({}) - {} años - {}",
            self.id.valor(),
            self.nombre,
            self.email,
            self.edad,
            if self.activo { "✓ Activo" } else { "✗ Inactivo" }
        );
    }

    // -----------------------------------------
    // Métodos de Modificación
    // -----------------------------------------
    
    fn cumplir_anios(&mut self) {
        self.edad += 1;
    }

    fn cambiar_email(&mut self, nuevo_email: String) {
        self.email = nuevo_email;
    }

    fn activar(&mut self) {
        self.activo = true;
    }

    fn desactivar(&mut self) {
        self.activo = false;
    }
}

// ============================================
// NIVEL 1: PERFIL DE USUARIO
// ============================================

/// Información adicional del perfil
struct Perfil {
    bio: String,
    sitio_web: Option<String>,
    ubicacion: String,
}

impl Perfil {
    fn new(bio: String, ubicacion: String) -> Self {
        Self {
            bio,
            sitio_web: None,
            ubicacion,
        }
    }

    fn con_sitio_web(bio: String, ubicacion: String, sitio: String) -> Self {
        Self {
            bio,
            sitio_web: Some(sitio),
            ubicacion,
        }
    }

    fn mostrar(&self) {
        println!("  Bio: {}", self.bio);
        println!("  Ubicación: {}", self.ubicacion);
        match &self.sitio_web {
            Some(url) => println!("  Web: {}", url),
            None => println!("  Web: No especificado"),
        }
    }

    fn set_sitio_web(&mut self, url: String) {
        self.sitio_web = Some(url);
    }
}

// ============================================
// NIVEL 1: USUARIO COMPLETO
// ============================================

/// Usuario con perfil completo
struct UsuarioCompleto {
    usuario: Usuario,
    perfil: Perfil,
}

impl UsuarioCompleto {
    fn new(usuario: Usuario, perfil: Perfil) -> Self {
        Self { usuario, perfil }
    }

    fn mostrar(&self) {
        println!("╔══════════════════════════════╗");
        println!("║       USUARIO COMPLETO       ║");
        println!("╠══════════════════════════════╣");
        self.usuario.mostrar();
        self.perfil.mostrar();
        println!("╚══════════════════════════════╝");
    }

    // Acceso delegado
    fn nombre(&self) -> &str {
        self.usuario.nombre()
    }

    fn ubicacion(&self) -> &str {
        &self.perfil.ubicacion
    }

    fn es_adulto(&self) -> bool {
        self.usuario.es_adulto()
    }
}

// ============================================
// NIVEL 2: SISTEMA DE ROLES
// ============================================

/// Roles disponibles en el sistema
struct Rol {
    nombre: String,
    puede_editar: bool,
    puede_eliminar: bool,
    es_admin: bool,
}

impl Rol {
    fn usuario() -> Self {
        Self {
            nombre: String::from("Usuario"),
            puede_editar: false,
            puede_eliminar: false,
            es_admin: false,
        }
    }

    fn editor() -> Self {
        Self {
            nombre: String::from("Editor"),
            puede_editar: true,
            puede_eliminar: false,
            es_admin: false,
        }
    }

    fn admin() -> Self {
        Self {
            nombre: String::from("Administrador"),
            puede_editar: true,
            puede_eliminar: true,
            es_admin: true,
        }
    }

    fn mostrar(&self) {
        println!(
            "Rol: {} [editar:{}, eliminar:{}, admin:{}]",
            self.nombre,
            self.puede_editar,
            self.puede_eliminar,
            self.es_admin
        );
    }
}

// ============================================
// NIVEL 3: GESTOR DE USUARIOS
// ============================================

/// Gestor que contiene múltiples usuarios
struct GestorUsuarios {
    usuarios: Vec<Usuario>,
    siguiente_id: u64,
}

impl GestorUsuarios {
    fn new() -> Self {
        Self {
            usuarios: Vec::new(),
            siguiente_id: 1,
        }
    }

    fn agregar(&mut self, nombre: String, email: String, edad: u32) -> UserId {
        let id = self.siguiente_id;
        self.siguiente_id += 1;
        
        let usuario = Usuario::completo(id, nombre, email, edad);
        let user_id = usuario.id();
        self.usuarios.push(usuario);
        
        user_id
    }

    fn cantidad(&self) -> usize {
        self.usuarios.len()
    }

    fn listar(&self) {
        println!("\n=== Lista de Usuarios ({}) ===", self.cantidad());
        for usuario in &self.usuarios {
            usuario.mostrar();
        }
    }

    fn buscar_por_id(&self, id: UserId) -> Option<&Usuario> {
        self.usuarios.iter().find(|u| u.id() == id)
    }

    fn usuarios_activos(&self) -> Vec<&Usuario> {
        self.usuarios.iter().filter(|u| u.esta_activo()).collect()
    }

    fn usuarios_adultos(&self) -> Vec<&Usuario> {
        self.usuarios.iter().filter(|u| u.es_adulto()).collect()
    }
}

// ============================================
// MAIN
// ============================================

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   👥 SISTEMA DE USUARIOS - RUST       ║");
    println!("║      Proyecto Semana 03               ║");
    println!("╚════════════════════════════════════════╝\n");

    // -----------------------------------------
    // Nivel 1: Usuarios básicos
    // -----------------------------------------
    println!("═══ NIVEL 1: USUARIOS BÁSICOS ═══\n");
    
    let mut usuario1 = Usuario::new(
        1,
        String::from("Ana García"),
        String::from("ana@email.com"),
    );
    usuario1.cumplir_anios();
    usuario1.cumplir_anios();
    
    let usuario2 = Usuario::completo(
        2,
        String::from("Carlos López"),
        String::from("carlos@email.com"),
        25,
    );

    println!("--- Usuarios creados ---");
    usuario1.mostrar();
    usuario2.mostrar();

    println!("\n¿Ana es adulta?: {}", usuario1.es_adulto());
    println!("¿Carlos es adulto?: {}", usuario2.es_adulto());

    // -----------------------------------------
    // Nivel 1: Usuario completo con perfil
    // -----------------------------------------
    println!("\n═══ USUARIO COMPLETO ═══\n");
    
    let usuario = Usuario::completo(
        3,
        String::from("María Rodríguez"),
        String::from("maria@email.com"),
        30,
    );
    
    let perfil = Perfil::con_sitio_web(
        String::from("Desarrolladora Rust apasionada"),
        String::from("Madrid, España"),
        String::from("https://maria.dev"),
    );

    let usuario_completo = UsuarioCompleto::new(usuario, perfil);
    usuario_completo.mostrar();

    // -----------------------------------------
    // Nivel 2: Roles
    // -----------------------------------------
    println!("\n═══ NIVEL 2: ROLES ═══\n");
    
    let rol_user = Rol::usuario();
    let rol_editor = Rol::editor();
    let rol_admin = Rol::admin();

    rol_user.mostrar();
    rol_editor.mostrar();
    rol_admin.mostrar();

    // -----------------------------------------
    // Nivel 3: Gestor de usuarios
    // -----------------------------------------
    println!("\n═══ NIVEL 3: GESTOR ═══");
    
    let mut gestor = GestorUsuarios::new();
    
    let id1 = gestor.agregar(
        String::from("Pedro"),
        String::from("pedro@email.com"),
        17,
    );
    
    gestor.agregar(
        String::from("Laura"),
        String::from("laura@email.com"),
        22,
    );
    
    gestor.agregar(
        String::from("Miguel"),
        String::from("miguel@email.com"),
        35,
    );

    gestor.listar();

    println!("\n--- Búsqueda por ID ---");
    if let Some(usuario) = gestor.buscar_por_id(id1) {
        print!("Encontrado: ");
        usuario.mostrar();
    }

    println!("\n--- Usuarios adultos ({}) ---", gestor.usuarios_adultos().len());
    for u in gestor.usuarios_adultos() {
        println!("  - {}", u.nombre());
    }

    println!("\n✅ Proyecto completado");
}

// ============================================
// TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usuario_new() {
        let u = Usuario::new(1, String::from("Test"), String::from("t@t.com"));
        
        assert_eq!(u.id().valor(), 1);
        assert_eq!(u.nombre(), "Test");
        assert!(u.esta_activo());
    }

    #[test]
    fn test_usuario_es_adulto() {
        let menor = Usuario::completo(1, String::from("A"), String::from("a@a"), 17);
        let adulto = Usuario::completo(2, String::from("B"), String::from("b@b"), 18);
        
        assert!(!menor.es_adulto());
        assert!(adulto.es_adulto());
    }

    #[test]
    fn test_usuario_cumplir_anios() {
        let mut u = Usuario::new(1, String::from("T"), String::from("t@t"));
        assert_eq!(u.edad(), 0);
        
        u.cumplir_anios();
        assert_eq!(u.edad(), 1);
    }

    #[test]
    fn test_usuario_desactivar() {
        let mut u = Usuario::new(1, String::from("T"), String::from("t@t"));
        assert!(u.esta_activo());
        
        u.desactivar();
        assert!(!u.esta_activo());
    }

    #[test]
    fn test_perfil_sin_web() {
        let p = Perfil::new(String::from("Bio"), String::from("Ciudad"));
        assert!(p.sitio_web.is_none());
    }

    #[test]
    fn test_perfil_con_web() {
        let p = Perfil::con_sitio_web(
            String::from("Bio"),
            String::from("Ciudad"),
            String::from("https://example.com"),
        );
        assert!(p.sitio_web.is_some());
    }

    #[test]
    fn test_rol_admin() {
        let admin = Rol::admin();
        assert!(admin.es_admin);
        assert!(admin.puede_editar);
        assert!(admin.puede_eliminar);
    }

    #[test]
    fn test_gestor_agregar() {
        let mut g = GestorUsuarios::new();
        g.agregar(String::from("A"), String::from("a@a"), 20);
        g.agregar(String::from("B"), String::from("b@b"), 25);
        
        assert_eq!(g.cantidad(), 2);
    }

    #[test]
    fn test_gestor_buscar() {
        let mut g = GestorUsuarios::new();
        let id = g.agregar(String::from("Test"), String::from("t@t"), 20);
        
        let encontrado = g.buscar_por_id(id);
        assert!(encontrado.is_some());
        assert_eq!(encontrado.unwrap().nombre(), "Test");
    }

    #[test]
    fn test_gestor_adultos() {
        let mut g = GestorUsuarios::new();
        g.agregar(String::from("Menor"), String::from("m@m"), 15);
        g.agregar(String::from("Adulto"), String::from("a@a"), 25);
        
        assert_eq!(g.usuarios_adultos().len(), 1);
    }
}
