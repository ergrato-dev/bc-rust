// Práctica 04: Crates Externos
// Semana 07 - Módulos y Crates
//
// Demostración del uso de dependencias externas de crates.io

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// EJERCICIO 1: Serialización con Serde
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evento {
    pub id: u64,
    pub titulo: String,
    pub descripcion: String,
    pub fecha: DateTime<Utc>,
    pub participantes: Vec<String>,
}

impl Evento {
    pub fn nuevo(titulo: &str, descripcion: &str) -> Self {
        Self {
            id: GeneradorTokens::generar_id(),
            titulo: titulo.to_string(),
            descripcion: descripcion.to_string(),
            fecha: Utc::now(),
            participantes: Vec::new(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn agregar_participante(&mut self, nombre: &str) {
        if !self.participantes.contains(&nombre.to_string()) {
            self.participantes.push(nombre.to_string());
        }
    }
}

// =============================================================================
// EJERCICIO 2: Generación de Datos Aleatorios
// =============================================================================

pub struct GeneradorTokens;

impl GeneradorTokens {
    /// Genera un token alfanumérico de longitud específica
    pub fn generar_token(longitud: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..longitud)
            .map(|_| {
                let idx = rng.gen_range(0..62);
                match idx {
                    0..=9 => (b'0' + idx) as char,
                    10..=35 => (b'A' + idx - 10) as char,
                    _ => (b'a' + idx - 36) as char,
                }
            })
            .collect()
    }

    /// Genera un ID numérico aleatorio
    pub fn generar_id() -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(100000..999999)
    }

    /// Genera un código de verificación de 6 dígitos
    pub fn generar_codigo_verificacion() -> String {
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(0..1000000))
    }

    /// Genera un UUID simplificado
    pub fn generar_uuid() -> String {
        let p1 = Self::generar_token(8);
        let p2 = Self::generar_token(4);
        let p3 = Self::generar_token(4);
        let p4 = Self::generar_token(4);
        let p5 = Self::generar_token(12);
        format!("{}-{}-{}-{}-{}", p1, p2, p3, p4, p5)
    }
}

// =============================================================================
// EJERCICIO 3: Manejo de Fechas con Chrono
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tarea {
    pub id: u64,
    pub titulo: String,
    pub creada: DateTime<Utc>,
    pub fecha_limite: Option<DateTime<Utc>>,
    pub completada: Option<DateTime<Utc>>,
}

impl Tarea {
    pub fn nueva(titulo: &str, dias_limite: Option<i64>) -> Self {
        let ahora = Utc::now();
        let fecha_limite = dias_limite.map(|dias| ahora + Duration::days(dias));

        Self {
            id: GeneradorTokens::generar_id(),
            titulo: titulo.to_string(),
            creada: ahora,
            fecha_limite,
            completada: None,
        }
    }

    pub fn completar(&mut self) {
        self.completada = Some(Utc::now());
    }

    pub fn esta_completada(&self) -> bool {
        self.completada.is_some()
    }

    pub fn esta_vencida(&self) -> bool {
        if self.esta_completada() {
            return false;
        }

        match self.fecha_limite {
            Some(limite) => Utc::now() > limite,
            None => false,
        }
    }

    pub fn tiempo_restante(&self) -> Option<Duration> {
        if self.esta_completada() {
            return None;
        }

        self.fecha_limite.map(|limite| limite - Utc::now())
    }

    pub fn descripcion_tiempo(&self) -> String {
        if self.esta_completada() {
            return "✓ Completada".to_string();
        }

        match self.tiempo_restante() {
            Some(duracion) if duracion.num_seconds() < 0 => {
                format!("⚠ Vencida hace {} días", duracion.num_days().abs())
            }
            Some(duracion) if duracion.num_days() == 0 => "⏰ Vence hoy".to_string(),
            Some(duracion) if duracion.num_days() == 1 => "📅 Vence mañana".to_string(),
            Some(duracion) => format!("📅 {} días restantes", duracion.num_days()),
            None => "Sin fecha límite".to_string(),
        }
    }
}

// =============================================================================
// EJERCICIO 4: Integración Completa - Gestor de Eventos
// =============================================================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GestorEventos {
    eventos: HashMap<u64, Evento>,
}

impl GestorEventos {
    pub fn nuevo() -> Self {
        Self {
            eventos: HashMap::new(),
        }
    }

    pub fn crear_evento(&mut self, titulo: &str, descripcion: &str) -> u64 {
        let evento = Evento::nuevo(titulo, descripcion);
        let id = evento.id;
        self.eventos.insert(id, evento);
        id
    }

    pub fn obtener_evento(&self, id: u64) -> Option<&Evento> {
        self.eventos.get(&id)
    }

    pub fn agregar_participante(&mut self, evento_id: u64, nombre: &str) -> Result<(), String> {
        match self.eventos.get_mut(&evento_id) {
            Some(evento) => {
                evento.agregar_participante(nombre);
                Ok(())
            }
            None => Err(format!("Evento {} no encontrado", evento_id)),
        }
    }

    pub fn listar_eventos(&self) -> Vec<&Evento> {
        self.eventos.values().collect()
    }

    pub fn eliminar_evento(&mut self, id: u64) -> bool {
        self.eventos.remove(&id).is_some()
    }

    pub fn exportar_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn importar_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("Error al parsear JSON: {}", e))
    }

    pub fn cantidad(&self) -> usize {
        self.eventos.len()
    }
}

// =============================================================================
// FUNCIÓN PRINCIPAL
// =============================================================================

fn main() {
    println!("=== Práctica 04: Crates Externos ===\n");

    // Ejercicio 1: Serde
    println!("--- Ejercicio 1: Serialización con Serde ---");
    let mut evento = Evento::nuevo("Reunión de Equipo", "Revisión semanal del proyecto");
    evento.agregar_participante("Alice");
    evento.agregar_participante("Bob");
    evento.agregar_participante("Charlie");

    let json = evento.to_json();
    println!("Evento serializado:\n{}\n", json);

    let recuperado = Evento::from_json(&json).expect("Error al deserializar");
    println!("Evento recuperado: {:?}\n", recuperado.titulo);

    // Ejercicio 2: Rand
    println!("--- Ejercicio 2: Generación Aleatoria ---");
    println!("Token (16 chars): {}", GeneradorTokens::generar_token(16));
    println!("ID numérico: {}", GeneradorTokens::generar_id());
    println!(
        "Código verificación: {}",
        GeneradorTokens::generar_codigo_verificacion()
    );
    println!("UUID: {}", GeneradorTokens::generar_uuid());

    // Ejercicio 3: Chrono
    println!("\n--- Ejercicio 3: Manejo de Fechas ---");

    let mut tarea_urgente = Tarea::nueva("Revisar código", Some(1));
    let tarea_normal = Tarea::nueva("Documentar API", Some(7));
    let tarea_vencida = Tarea::nueva("Tarea atrasada", Some(-2)); // Vencida hace 2 días
    let tarea_sin_limite = Tarea::nueva("Tarea eventual", None);

    println!("Tarea urgente: {}", tarea_urgente.descripcion_tiempo());
    println!("Tarea normal: {}", tarea_normal.descripcion_tiempo());
    println!("Tarea vencida: {}", tarea_vencida.descripcion_tiempo());
    println!(
        "Tarea sin límite: {}",
        tarea_sin_limite.descripcion_tiempo()
    );

    tarea_urgente.completar();
    println!(
        "\nDespués de completar urgente: {}",
        tarea_urgente.descripcion_tiempo()
    );

    // Ejercicio 4: Integración
    println!("\n--- Ejercicio 4: Gestor de Eventos ---");
    let mut gestor = GestorEventos::nuevo();

    let id1 = gestor.crear_evento("Hackathon", "Competencia de programación");
    let id2 = gestor.crear_evento("Workshop Rust", "Introducción a Rust");

    gestor.agregar_participante(id1, "Dev1").unwrap();
    gestor.agregar_participante(id1, "Dev2").unwrap();
    gestor.agregar_participante(id2, "Estudiante1").unwrap();

    println!("Eventos creados: {}", gestor.cantidad());

    // Exportar e importar
    let export = gestor.exportar_json();
    println!("\nGestor exportado (primeros 200 chars):");
    println!("{}...", &export[..export.len().min(200)]);

    let importado = GestorEventos::importar_json(&export).expect("Error al importar");
    println!("\nEventos después de importar: {}", importado.cantidad());

    println!("\n=== Fin de la práctica ===");
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evento_serializacion() {
        let mut evento = Evento::nuevo("Test", "Descripción");
        evento.agregar_participante("User1");

        let json = evento.to_json();
        let recuperado = Evento::from_json(&json).unwrap();

        assert_eq!(evento.titulo, recuperado.titulo);
        assert_eq!(evento.descripcion, recuperado.descripcion);
        assert_eq!(evento.participantes, recuperado.participantes);
    }

    #[test]
    fn test_evento_participantes_unicos() {
        let mut evento = Evento::nuevo("Test", "");
        evento.agregar_participante("Alice");
        evento.agregar_participante("Alice"); // Duplicado

        assert_eq!(evento.participantes.len(), 1);
    }

    #[test]
    fn test_token_longitud() {
        let token = GeneradorTokens::generar_token(32);
        assert_eq!(token.len(), 32);

        let token_corto = GeneradorTokens::generar_token(8);
        assert_eq!(token_corto.len(), 8);
    }

    #[test]
    fn test_token_alfanumerico() {
        let token = GeneradorTokens::generar_token(100);
        assert!(token.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_codigo_verificacion_formato() {
        let codigo = GeneradorTokens::generar_codigo_verificacion();
        assert_eq!(codigo.len(), 6);
        assert!(codigo.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_id_rango() {
        for _ in 0..100 {
            let id = GeneradorTokens::generar_id();
            assert!(id >= 100000 && id < 999999);
        }
    }

    #[test]
    fn test_tarea_nueva() {
        let tarea = Tarea::nueva("Test", Some(5));
        assert!(!tarea.esta_completada());
        assert!(!tarea.esta_vencida());
        assert!(tarea.fecha_limite.is_some());
    }

    #[test]
    fn test_tarea_completar() {
        let mut tarea = Tarea::nueva("Test", Some(1));
        assert!(!tarea.esta_completada());

        tarea.completar();
        assert!(tarea.esta_completada());
        assert!(tarea.completada.is_some());
    }

    #[test]
    fn test_tarea_vencida() {
        let tarea = Tarea::nueva("Test", Some(-1));
        assert!(tarea.esta_vencida());
    }

    #[test]
    fn test_tarea_completada_no_vencida() {
        let mut tarea = Tarea::nueva("Test", Some(-1));
        tarea.completar();
        // Una tarea completada no se considera vencida
        assert!(!tarea.esta_vencida());
    }

    #[test]
    fn test_tarea_sin_limite() {
        let tarea = Tarea::nueva("Test", None);
        assert!(!tarea.esta_vencida());
        assert!(tarea.tiempo_restante().is_none());
    }

    #[test]
    fn test_gestor_crear_evento() {
        let mut gestor = GestorEventos::nuevo();
        let id = gestor.crear_evento("Test", "Desc");

        assert_eq!(gestor.cantidad(), 1);
        assert!(gestor.obtener_evento(id).is_some());
    }

    #[test]
    fn test_gestor_agregar_participante() {
        let mut gestor = GestorEventos::nuevo();
        let id = gestor.crear_evento("Test", "Desc");

        assert!(gestor.agregar_participante(id, "User1").is_ok());
        assert!(gestor.agregar_participante(999999, "User1").is_err());
    }

    #[test]
    fn test_gestor_eliminar() {
        let mut gestor = GestorEventos::nuevo();
        let id = gestor.crear_evento("Test", "Desc");

        assert!(gestor.eliminar_evento(id));
        assert_eq!(gestor.cantidad(), 0);
        assert!(!gestor.eliminar_evento(id));
    }

    #[test]
    fn test_gestor_exportar_importar() {
        let mut original = GestorEventos::nuevo();
        original.crear_evento("Evento 1", "Desc 1");
        original.crear_evento("Evento 2", "Desc 2");

        let json = original.exportar_json();
        let importado = GestorEventos::importar_json(&json).unwrap();

        assert_eq!(original.cantidad(), importado.cantidad());
    }
}
