use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("el nombre no puede estar vacío")]
    EmptyName,
    #[error("la edad {0} está fuera del rango válido (0-150)")]
    InvalidAge(u32),
    #[error("email inválido: {0}")]
    InvalidEmail(String),
}

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// Builder para `User` con validación en `build()`.
///
/// # Examples
///
/// ```
/// # use practice_01_ergonomia_api::UserBuilder;
/// let user = UserBuilder::new()
///     .name("Ana")
///     .age(25)
///     .email("ana@example.com")
///     .build()
///     .unwrap();
/// assert_eq!(user.name, "Ana");
/// ```
#[derive(Default)]
pub struct UserBuilder {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn build(self) -> Result<User, UserError> {
        let name = self.name.unwrap_or_default();
        if name.is_empty() {
            return Err(UserError::EmptyName);
        }
        let age = self.age.unwrap_or(0);
        if age > 150 {
            return Err(UserError::InvalidAge(age));
        }
        let email = self.email.unwrap_or_default();
        if !email.contains('@') {
            return Err(UserError::InvalidEmail(email));
        }
        Ok(User { name, age, email })
    }
}

fn main() {
    match UserBuilder::new()
        .name("Ana García")
        .age(30)
        .email("ana@example.com")
        .build()
    {
        Ok(u) => println!("Usuario creado: {:?}", u),
        Err(e) => println!("Error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_valido() {
        let u = UserBuilder::new()
            .name("Ana")
            .age(25)
            .email("ana@example.com")
            .build()
            .unwrap();
        assert_eq!(u.name, "Ana");
        assert_eq!(u.age, 25);
    }

    #[test]
    fn nombre_vacio_error() {
        let err = UserBuilder::new().email("a@b.com").build().unwrap_err();
        assert!(matches!(err, UserError::EmptyName));
    }

    #[test]
    fn edad_invalida_error() {
        let err = UserBuilder::new()
            .name("x")
            .age(200)
            .email("x@x.com")
            .build()
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidAge(200)));
    }

    #[test]
    fn email_invalido_error() {
        let err = UserBuilder::new()
            .name("x")
            .email("no-es-email")
            .build()
            .unwrap_err();
        assert!(matches!(err, UserError::InvalidEmail(_)));
    }
}
