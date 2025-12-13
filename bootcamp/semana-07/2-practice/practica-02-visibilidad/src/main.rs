// Práctica 02: Visibilidad y Encapsulación
// Semana 07 - Módulos y Crates

// =============================================================================
// EJERCICIO 1: Cuenta Bancaria Segura
// =============================================================================

mod banco {
    pub struct CuentaBancaria {
        titular: String,           // privado - solo el módulo banco
        saldo: f64,                // privado - protegido
        pub numero_cuenta: String, // público - visible externamente
    }

    impl CuentaBancaria {
        pub fn nueva(titular: &str, numero: &str) -> Self {
            Self {
                titular: titular.to_string(),
                saldo: 0.0,
                numero_cuenta: numero.to_string(),
            }
        }

        pub fn depositar(&mut self, cantidad: f64) -> Result<(), &'static str> {
            if !Self::validar_cantidad(cantidad) {
                return Err("Cantidad inválida");
            }
            self.saldo += cantidad;
            Ok(())
        }

        pub fn retirar(&mut self, cantidad: f64) -> Result<(), &'static str> {
            if !Self::validar_cantidad(cantidad) {
                return Err("Cantidad inválida");
            }
            if cantidad > self.saldo {
                return Err("Fondos insuficientes");
            }
            self.saldo -= cantidad;
            Ok(())
        }

        pub fn consultar_saldo(&self) -> f64 {
            self.saldo
        }

        pub fn obtener_titular(&self) -> &str {
            &self.titular
        }

        // Método privado - solo uso interno
        fn validar_cantidad(cantidad: f64) -> bool {
            cantidad > 0.0 && cantidad.is_finite()
        }
    }
}

// =============================================================================
// EJERCICIO 2: API con Niveles de Visibilidad
// =============================================================================

pub mod api {
    // Público a todo el mundo
    pub const VERSION: &str = "1.0.0";

    // Solo visible dentro del crate
    pub(crate) fn log_interno(mensaje: &str) {
        println!("[LOG] {}", mensaje);
    }

    pub mod endpoints {
        // Público
        pub fn obtener_datos() -> Vec<String> {
            super::log_interno("Obteniendo datos...");
            let interno = helper_interno();
            let procesado = implementacion::procesar();
            vec![interno, procesado]
        }

        // Solo visible para el módulo padre (api)
        pub(super) fn helper_interno() -> String {
            "datos internos".to_string()
        }

        mod implementacion {
            // Solo visible dentro de endpoints
            pub(super) fn procesar() -> String {
                "datos procesados".to_string()
            }
        }
    }

    // Esta función puede usar helper_interno porque está en el módulo padre
    pub fn demo_helper() -> String {
        endpoints::helper_interno()
    }
}

// =============================================================================
// EJERCICIO 3: Usuario con Campos Mixtos
// =============================================================================

mod usuarios {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub struct Usuario {
        pub id: u64,
        pub nombre: String,
        pub(crate) email: String,  // visible en el crate
        pub(super) rol: String,    // visible en el módulo padre
        password_hash: String,     // completamente privado
    }

    impl Usuario {
        pub fn nuevo(nombre: &str, email: &str, password: &str) -> Self {
            static mut ID_COUNTER: u64 = 0;

            let id = unsafe {
                ID_COUNTER += 1;
                ID_COUNTER
            };

            Self {
                id,
                nombre: nombre.to_string(),
                email: email.to_string(),
                rol: "usuario".to_string(),
                password_hash: Self::hashear_password(password),
            }
        }

        pub fn verificar_password(&self, password: &str) -> bool {
            self.password_hash == Self::hashear_password(password)
        }

        // Solo administradores del crate pueden cambiar roles
        pub(crate) fn cambiar_rol(&mut self, nuevo_rol: &str) {
            self.rol = nuevo_rol.to_string();
        }

        pub fn obtener_rol(&self) -> &str {
            &self.rol
        }

        // Función privada para hashear
        fn hashear_password(password: &str) -> String {
            let mut hasher = DefaultHasher::new();
            password.hash(&mut hasher);
            format!("hash_{:x}", hasher.finish())
        }
    }
}

// Función en el módulo raíz que puede acceder a pub(crate) y pub(super)
fn administrar_usuario(usuario: &mut usuarios::Usuario) {
    // Podemos acceder a email porque es pub(crate)
    println!("Email: {}", usuario.email);

    // Podemos acceder a rol porque estamos en el módulo padre (crate root)
    println!("Rol actual: {}", usuario.rol);

    // Podemos cambiar el rol porque cambiar_rol es pub(crate)
    usuario.cambiar_rol("admin");
}

// =============================================================================
// FUNCIÓN PRINCIPAL
// =============================================================================

fn main() {
    println!("=== Práctica 02: Visibilidad y Encapsulación ===\n");

    // Ejercicio 1: Cuenta Bancaria
    println!("--- Ejercicio 1: Cuenta Bancaria ---");
    let mut cuenta = banco::CuentaBancaria::nueva("Juan Pérez", "ES1234567890");

    println!("Cuenta: {}", cuenta.numero_cuenta);
    println!("Titular: {}", cuenta.obtener_titular());
    println!("Saldo inicial: {:.2}€", cuenta.consultar_saldo());

    cuenta.depositar(1000.0).unwrap();
    println!("Después de depositar 1000€: {:.2}€", cuenta.consultar_saldo());

    cuenta.retirar(250.0).unwrap();
    println!("Después de retirar 250€: {:.2}€", cuenta.consultar_saldo());

    match cuenta.retirar(10000.0) {
        Ok(_) => println!("Retiro exitoso"),
        Err(e) => println!("Error al retirar: {}", e),
    }

    // Ejercicio 2: API
    println!("\n--- Ejercicio 2: API ---");
    println!("Versión API: {}", api::VERSION);

    // log_interno es pub(crate), así que podemos llamarlo desde aquí
    api::log_interno("Mensaje desde main");

    let datos = api::endpoints::obtener_datos();
    println!("Datos obtenidos: {:?}", datos);

    // Ejercicio 3: Usuario
    println!("\n--- Ejercicio 3: Usuario ---");
    let mut usuario = usuarios::Usuario::nuevo("alice", "alice@example.com", "secreto123");

    println!("ID: {}", usuario.id);
    println!("Nombre: {}", usuario.nombre);
    println!("Email: {}", usuario.email); // Accesible porque es pub(crate)
    println!("Rol: {}", usuario.obtener_rol());

    // Verificar password
    println!(
        "Password correcto: {}",
        usuario.verificar_password("secreto123")
    );
    println!(
        "Password incorrecto: {}",
        usuario.verificar_password("wrong")
    );

    // Administrar usuario (accede a campos pub(crate) y pub(super))
    println!("\n--- Administración ---");
    administrar_usuario(&mut usuario);
    println!("Nuevo rol: {}", usuario.obtener_rol());
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuenta_nueva() {
        let cuenta = banco::CuentaBancaria::nueva("Test", "ES0000");
        assert_eq!(cuenta.consultar_saldo(), 0.0);
        assert_eq!(cuenta.numero_cuenta, "ES0000");
    }

    #[test]
    fn test_cuenta_deposito() {
        let mut cuenta = banco::CuentaBancaria::nueva("Juan", "ES1234");
        assert!(cuenta.depositar(100.0).is_ok());
        assert_eq!(cuenta.consultar_saldo(), 100.0);
    }

    #[test]
    fn test_cuenta_deposito_invalido() {
        let mut cuenta = banco::CuentaBancaria::nueva("Juan", "ES1234");
        assert!(cuenta.depositar(-50.0).is_err());
        assert!(cuenta.depositar(0.0).is_err());
    }

    #[test]
    fn test_cuenta_retiro_exitoso() {
        let mut cuenta = banco::CuentaBancaria::nueva("Ana", "ES5678");
        cuenta.depositar(100.0).unwrap();
        assert!(cuenta.retirar(50.0).is_ok());
        assert_eq!(cuenta.consultar_saldo(), 50.0);
    }

    #[test]
    fn test_cuenta_retiro_insuficiente() {
        let mut cuenta = banco::CuentaBancaria::nueva("Ana", "ES5678");
        cuenta.depositar(50.0).unwrap();
        assert!(cuenta.retirar(100.0).is_err());
        assert_eq!(cuenta.consultar_saldo(), 50.0); // Saldo sin cambios
    }

    #[test]
    fn test_api_version_publica() {
        assert_eq!(api::VERSION, "1.0.0");
    }

    #[test]
    fn test_api_log_interno_accesible_en_crate() {
        // Este test funciona porque estamos en el mismo crate
        api::log_interno("test");
    }

    #[test]
    fn test_api_endpoints() {
        let datos = api::endpoints::obtener_datos();
        assert_eq!(datos.len(), 2);
        assert!(datos.contains(&"datos internos".to_string()));
    }

    #[test]
    fn test_usuario_campos_publicos() {
        let usuario = usuarios::Usuario::nuevo("test", "test@mail.com", "pass");
        assert!(!usuario.nombre.is_empty());
        assert!(usuario.id > 0);
    }

    #[test]
    fn test_usuario_email_pub_crate() {
        // Podemos acceder a email porque es pub(crate)
        let usuario = usuarios::Usuario::nuevo("test", "test@mail.com", "pass");
        assert_eq!(usuario.email, "test@mail.com");
    }

    #[test]
    fn test_usuario_password_protegido() {
        let usuario = usuarios::Usuario::nuevo("test", "test@mail.com", "secret123");
        assert!(usuario.verificar_password("secret123"));
        assert!(!usuario.verificar_password("wrong"));
    }

    #[test]
    fn test_usuario_cambiar_rol() {
        let mut usuario = usuarios::Usuario::nuevo("test", "test@mail.com", "pass");
        assert_eq!(usuario.obtener_rol(), "usuario");

        // cambiar_rol es pub(crate), así que podemos llamarlo
        usuario.cambiar_rol("admin");
        assert_eq!(usuario.obtener_rol(), "admin");
    }
}
