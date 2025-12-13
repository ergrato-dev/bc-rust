# Práctica 04: Crates Externos

## 🎯 Objetivo

Aprender a agregar y usar dependencias externas de crates.io en proyectos Rust.

## 📦 Crates a Utilizar

| Crate | Descripción | Uso |
|-------|-------------|-----|
| `serde` | Serialización/Deserialización | JSON, configs |
| `serde_json` | Soporte JSON para serde | Parsear/generar JSON |
| `chrono` | Manejo de fechas y tiempo | Timestamps |
| `rand` | Generación de números aleatorios | IDs, tokens |

## 📋 Instrucciones

### Ejercicio 1: Serialización con Serde

Crea un sistema de eventos serializable:

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Evento {
    pub id: u64,
    pub titulo: String,
    pub descripcion: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub fecha: DateTime<Utc>,
    pub participantes: Vec<String>,
}

impl Evento {
    pub fn nuevo(titulo: &str, descripcion: &str) -> Self {
        // TODO: Generar ID aleatorio con rand
        todo!()
    }
    
    pub fn to_json(&self) -> String {
        // TODO: Serializar a JSON
        todo!()
    }
    
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        // TODO: Deserializar desde JSON
        todo!()
    }
}
```

### Ejercicio 2: Generación de Datos Aleatorios

Usa `rand` para generar datos:

```rust
use rand::Rng;
use rand::distributions::Alphanumeric;

pub struct GeneradorTokens;

impl GeneradorTokens {
    /// Genera un token alfanumérico de longitud específica
    pub fn generar_token(longitud: usize) -> String {
        // TODO: Usar rand para generar
        todo!()
    }
    
    /// Genera un ID numérico aleatorio
    pub fn generar_id() -> u64 {
        // TODO: Implementar
        todo!()
    }
    
    /// Genera un código de verificación de 6 dígitos
    pub fn generar_codigo_verificacion() -> String {
        // TODO: Implementar
        todo!()
    }
}
```

### Ejercicio 3: Manejo de Fechas con Chrono

Implementa un sistema de tareas con fechas:

```rust
use chrono::{DateTime, Utc, Duration, Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarea {
    pub titulo: String,
    pub creada: DateTime<Utc>,
    pub fecha_limite: Option<DateTime<Utc>>,
    pub completada: Option<DateTime<Utc>>,
}

impl Tarea {
    pub fn nueva(titulo: &str, dias_limite: Option<i64>) -> Self {
        // TODO: Crear con fecha actual y límite opcional
        todo!()
    }
    
    pub fn completar(&mut self) {
        // TODO: Marcar como completada con timestamp actual
        todo!()
    }
    
    pub fn esta_vencida(&self) -> bool {
        // TODO: Verificar si pasó la fecha límite
        todo!()
    }
    
    pub fn tiempo_restante(&self) -> Option<Duration> {
        // TODO: Calcular tiempo hasta la fecha límite
        todo!()
    }
}
```

### Ejercicio 4: Integración Completa

Crea un mini sistema de gestión que combine todo:

```rust
use std::collections::HashMap;

pub struct GestorEventos {
    eventos: HashMap<u64, Evento>,
}

impl GestorEventos {
    pub fn nuevo() -> Self { /* TODO */ }
    
    pub fn crear_evento(&mut self, titulo: &str, desc: &str) -> u64 { /* TODO */ }
    
    pub fn agregar_participante(&mut self, evento_id: u64, nombre: &str) -> Result<(), String> { /* TODO */ }
    
    pub fn exportar_json(&self) -> String { /* TODO */ }
    
    pub fn importar_json(&mut self, json: &str) -> Result<usize, String> { /* TODO */ }
}
```

## ✅ Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evento_serializacion() {
        let evento = Evento::nuevo("Reunión", "Revisión de proyecto");
        let json = evento.to_json();
        let recuperado = Evento::from_json(&json).unwrap();
        assert_eq!(evento.titulo, recuperado.titulo);
    }

    #[test]
    fn test_token_longitud() {
        let token = GeneradorTokens::generar_token(32);
        assert_eq!(token.len(), 32);
    }

    #[test]
    fn test_tarea_vencida() {
        let mut tarea = Tarea::nueva("Test", Some(-1)); // Vencida ayer
        assert!(tarea.esta_vencida());
    }
}
```

## 🎯 Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Serde funciona correctamente | 30% |
| Rand genera datos válidos | 20% |
| Chrono maneja fechas bien | 25% |
| Integración completa | 25% |

## 💡 Comandos Útiles

```bash
# Agregar dependencia
cargo add serde --features derive

# Ver árbol de dependencias
cargo tree

# Actualizar dependencias
cargo update

# Verificar documentación de crate
cargo doc --open
```
