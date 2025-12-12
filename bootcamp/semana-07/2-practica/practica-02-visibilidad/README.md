# Práctica 02: Visibilidad y Encapsulación

## 🎯 Objetivo

Dominar los niveles de visibilidad en Rust (`pub`, `pub(crate)`, `pub(super)`, privado) para crear APIs bien encapsuladas.

## 📋 Instrucciones

### Ejercicio 1: Cuenta Bancaria Segura

Implementa una cuenta bancaria donde el saldo esté protegido:

```rust
mod banco {
    pub struct CuentaBancaria {
        titular: String,           // privado - solo el módulo banco
        saldo: f64,                // privado - protegido
        pub numero_cuenta: String, // público - visible externamente
    }
    
    impl CuentaBancaria {
        pub fn nueva(titular: &str, numero: &str) -> Self {
            // TODO: Crear cuenta con saldo inicial 0.0
            todo!()
        }
        
        pub fn depositar(&mut self, cantidad: f64) -> Result<(), &'static str> {
            // TODO: Validar que cantidad > 0, incrementar saldo
            todo!()
        }
        
        pub fn retirar(&mut self, cantidad: f64) -> Result<(), &'static str> {
            // TODO: Validar fondos suficientes
            todo!()
        }
        
        pub fn consultar_saldo(&self) -> f64 {
            // Acceso controlado de lectura
            self.saldo
        }
        
        // Método privado - solo uso interno
        fn validar_cantidad(cantidad: f64) -> bool {
            cantidad > 0.0 && cantidad.is_finite()
        }
    }
}
```

### Ejercicio 2: API con Niveles de Visibilidad

Crea un módulo con diferentes niveles de acceso:

```rust
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
            // TODO: Usar helper interno y log
            todo!()
        }
        
        // Solo visible para el módulo padre (api)
        pub(super) fn helper_interno() -> String {
            "datos internos".to_string()
        }
        
        mod implementacion {
            // Solo visible dentro de endpoints
            pub(super) fn procesar() -> String {
                // TODO: Implementar
                todo!()
            }
        }
    }
}
```

### Ejercicio 3: Struct con Campos Mixtos

Implementa un usuario con campos de diferentes visibilidades:

```rust
mod usuarios {
    pub struct Usuario {
        pub id: u64,
        pub nombre: String,
        pub(crate) email: String,        // visible en el crate
        pub(super) rol: String,          // visible en el módulo padre
        password_hash: String,           // completamente privado
    }
    
    impl Usuario {
        pub fn nuevo(nombre: &str, email: &str, password: &str) -> Self {
            // TODO: Hashear password (simular con formato)
            todo!()
        }
        
        pub fn verificar_password(&self, password: &str) -> bool {
            // TODO: Comparar hash
            todo!()
        }
        
        // Solo administradores del crate pueden cambiar roles
        pub(crate) fn cambiar_rol(&mut self, nuevo_rol: &str) {
            self.rol = nuevo_rol.to_string();
        }
    }
}
```

## ✅ Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuenta_deposito() {
        let mut cuenta = banco::CuentaBancaria::nueva("Juan", "ES1234");
        assert!(cuenta.depositar(100.0).is_ok());
        assert_eq!(cuenta.consultar_saldo(), 100.0);
    }

    #[test]
    fn test_cuenta_retiro_insuficiente() {
        let mut cuenta = banco::CuentaBancaria::nueva("Ana", "ES5678");
        cuenta.depositar(50.0).unwrap();
        assert!(cuenta.retirar(100.0).is_err());
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
    fn test_usuario_password_protegido() {
        let usuario = usuarios::Usuario::nuevo("test", "test@mail.com", "secret123");
        assert!(usuario.verificar_password("secret123"));
        assert!(!usuario.verificar_password("wrong"));
    }
}
```

## 🎯 Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Encapsulación de CuentaBancaria | 30% |
| Niveles de visibilidad correctos | 25% |
| Usuario con campos protegidos | 25% |
| Tests pasan | 20% |

## 💡 Pistas

1. `pub` = público para todos
2. `pub(crate)` = público solo dentro del crate actual
3. `pub(super)` = público solo para el módulo padre
4. Sin modificador = privado (solo el módulo actual)
5. Los campos de struct son privados por defecto
6. Usa métodos públicos para acceso controlado a campos privados
