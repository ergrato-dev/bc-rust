//! # Práctica 03: HashMaps
//!
//! Ejercicios para dominar HashMap<K, V> en Rust.

use std::collections::HashMap;

fn main() {
    println!("=== Práctica 03: HashMaps ===\n");

    // Ejercicio 1: Operaciones básicas
    demo_operaciones_basicas();

    // Ejercicio 2: Entry API
    demo_entry_api();

    // Ejercicio 3: Contador y agrupación
    demo_contador_agrupacion();

    // Ejercicio 4: Agenda de contactos
    demo_agenda();
}

// ============================================================================
// EJERCICIO 1: Operaciones Básicas
// ============================================================================

/// Crea un HashMap desde vectores de claves y valores
fn crear_desde_vecs<K, V>(claves: Vec<K>, valores: Vec<V>) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    claves.into_iter().zip(valores).collect()
}

/// Obtiene un valor o un default si no existe
fn obtener_o_default<'a>(mapa: &'a HashMap<String, i32>, clave: &str, default: &'a i32) -> &'a i32 {
    mapa.get(clave).unwrap_or(default)
}

/// Elimina entradas con valor menor a un umbral
fn eliminar_menores_que(mapa: &mut HashMap<String, i32>, umbral: i32) {
    mapa.retain(|_, v| *v >= umbral);
}

/// Invierte un mapa (valor -> clave)
fn invertir_mapa(mapa: &HashMap<String, String>) -> HashMap<String, String> {
    mapa.iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect()
}

/// Fusiona dos mapas, sumando valores para claves duplicadas
fn fusionar_sumando(
    mapa1: &HashMap<String, i32>,
    mapa2: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut resultado = mapa1.clone();
    for (clave, valor) in mapa2 {
        *resultado.entry(clave.clone()).or_insert(0) += valor;
    }
    resultado
}

fn demo_operaciones_basicas() {
    println!("--- Ejercicio 1: Operaciones Básicas ---");

    // Crear desde vectores
    let claves = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let valores = vec![1, 2, 3];
    let mapa = crear_desde_vecs(claves, valores);
    println!("Mapa desde vecs: {:?}", mapa);

    // Obtener o default
    let default = 0;
    println!("Valor de 'a': {}", obtener_o_default(&mapa, "a", &default));
    println!("Valor de 'z': {}", obtener_o_default(&mapa, "z", &default));

    // Eliminar menores
    let mut mapa2: HashMap<String, i32> = [
        ("bajo".to_string(), 5),
        ("medio".to_string(), 15),
        ("alto".to_string(), 25),
    ]
    .into_iter()
    .collect();
    eliminar_menores_que(&mut mapa2, 10);
    println!("Después de eliminar < 10: {:?}", mapa2);

    // Invertir
    let traducciones: HashMap<String, String> = [
        ("hola".to_string(), "hello".to_string()),
        ("mundo".to_string(), "world".to_string()),
    ]
    .into_iter()
    .collect();
    let invertido = invertir_mapa(&traducciones);
    println!("Invertido: {:?}", invertido);

    // Fusionar
    let mapa_a: HashMap<String, i32> =
        [("x".to_string(), 10), ("y".to_string(), 20)].into_iter().collect();
    let mapa_b: HashMap<String, i32> =
        [("y".to_string(), 5), ("z".to_string(), 15)].into_iter().collect();
    let fusionado = fusionar_sumando(&mapa_a, &mapa_b);
    println!("Fusionado (sumando): {:?}", fusionado);

    println!();
}

// ============================================================================
// EJERCICIO 2: Entry API
// ============================================================================

/// Incrementa el contador para una clave
fn incrementar(mapa: &mut HashMap<String, i32>, clave: &str) {
    *mapa.entry(clave.to_string()).or_insert(0) += 1;
}

/// Establece un valor solo si la clave no existe
fn establecer_si_ausente(mapa: &mut HashMap<String, String>, clave: &str, valor: &str) -> bool {
    use std::collections::hash_map::Entry;
    
    match mapa.entry(clave.to_string()) {
        Entry::Vacant(e) => {
            e.insert(valor.to_string());
            true
        }
        Entry::Occupied(_) => false,
    }
}

/// Actualiza un valor si existe, de lo contrario inserta un default
fn actualizar_o_insertar<F>(
    mapa: &mut HashMap<String, i32>,
    clave: &str,
    default: i32,
    actualizar: F,
) where
    F: FnOnce(&mut i32),
{
    mapa.entry(clave.to_string())
        .and_modify(actualizar)
        .or_insert(default);
}

/// Agrega un elemento a la lista asociada a una clave
fn agregar_a_lista(mapa: &mut HashMap<String, Vec<String>>, clave: &str, valor: &str) {
    mapa.entry(clave.to_string())
        .or_default()
        .push(valor.to_string());
}

fn demo_entry_api() {
    println!("--- Ejercicio 2: Entry API ---");

    // Contador
    let mut contador: HashMap<String, i32> = HashMap::new();
    incrementar(&mut contador, "a");
    incrementar(&mut contador, "b");
    incrementar(&mut contador, "a");
    incrementar(&mut contador, "a");
    println!("Contador: {:?}", contador);

    // Establecer si ausente
    let mut config: HashMap<String, String> = HashMap::new();
    config.insert("tema".to_string(), "oscuro".to_string());
    
    let establecido1 = establecer_si_ausente(&mut config, "idioma", "es");
    let establecido2 = establecer_si_ausente(&mut config, "tema", "claro");
    println!("Idioma establecido: {} -> {:?}", establecido1, config.get("idioma"));
    println!("Tema establecido: {} -> {:?}", establecido2, config.get("tema"));

    // Actualizar o insertar
    let mut puntos: HashMap<String, i32> = HashMap::new();
    puntos.insert("alice".to_string(), 100);
    
    actualizar_o_insertar(&mut puntos, "alice", 0, |v| *v += 50);
    actualizar_o_insertar(&mut puntos, "bob", 25, |v| *v += 50);
    println!("Puntos: {:?}", puntos);

    // Agregar a lista
    let mut categorias: HashMap<String, Vec<String>> = HashMap::new();
    agregar_a_lista(&mut categorias, "frutas", "manzana");
    agregar_a_lista(&mut categorias, "frutas", "pera");
    agregar_a_lista(&mut categorias, "verduras", "zanahoria");
    println!("Categorías: {:?}", categorias);

    println!();
}

// ============================================================================
// EJERCICIO 3: Contador y Agrupación
// ============================================================================

/// Cuenta la frecuencia de cada elemento
fn contar_frecuencia<T>(elementos: &[T]) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut frecuencia = HashMap::new();
    for elemento in elementos {
        *frecuencia.entry(elemento.clone()).or_insert(0) += 1;
    }
    frecuencia
}

/// Encuentra el elemento más frecuente
fn mas_frecuente<T>(elementos: &[T]) -> Option<(T, usize)>
where
    T: Eq + std::hash::Hash + Clone,
{
    let frecuencia = contar_frecuencia(elementos);
    frecuencia
        .into_iter()
        .max_by_key(|(_, count)| *count)
}

/// Agrupa elementos por el resultado de una función
fn agrupar_por<T, K, F>(elementos: Vec<T>, clasificador: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut grupos: HashMap<K, Vec<T>> = HashMap::new();
    for elemento in elementos {
        let clave = clasificador(&elemento);
        grupos.entry(clave).or_default().push(elemento);
    }
    grupos
}

/// Cuenta palabras en un texto
fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut frecuencia = HashMap::new();
    for palabra in texto.split_whitespace() {
        let palabra_limpia = palabra
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();
        if !palabra_limpia.is_empty() {
            *frecuencia.entry(palabra_limpia).or_insert(0) += 1;
        }
    }
    frecuencia
}

/// Genera un histograma de texto
fn histograma(frecuencia: &HashMap<String, usize>) -> String {
    let mut resultado = String::new();
    let mut items: Vec<_> = frecuencia.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));

    for (palabra, count) in items.iter().take(5) {
        resultado.push_str(&format!("{}: {}\n", palabra, "#".repeat(**count)));
    }
    resultado
}

fn demo_contador_agrupacion() {
    println!("--- Ejercicio 3: Contador y Agrupación ---");

    // Frecuencia de números
    let numeros = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let freq = contar_frecuencia(&numeros);
    println!("Frecuencia: {:?}", freq);

    // Más frecuente
    if let Some((num, count)) = mas_frecuente(&numeros) {
        println!("Más frecuente: {} ({} veces)", num, count);
    }

    // Agrupar por paridad
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let por_paridad = agrupar_por(nums, |n| if n % 2 == 0 { "par" } else { "impar" });
    println!("Por paridad: {:?}", por_paridad);

    // Agrupar por longitud
    let palabras = vec!["hola", "mundo", "rust", "es", "genial", "a"];
    let por_longitud = agrupar_por(palabras, |s| s.len());
    println!("Por longitud: {:?}", por_longitud);

    // Contar palabras
    let texto = "Rust es seguro. Rust es rápido. Rust es genial.";
    let conteo = contar_palabras(texto);
    println!("\nConteo de palabras en: '{}'", texto);
    println!("{:?}", conteo);

    // Histograma
    println!("\nHistograma:");
    print!("{}", histograma(&conteo));

    println!();
}

// ============================================================================
// EJERCICIO 4: Agenda de Contactos
// ============================================================================

#[derive(Debug, Clone)]
struct Contacto {
    nombre: String,
    telefono: String,
    email: Option<String>,
    etiquetas: Vec<String>,
}

impl Contacto {
    fn new(nombre: &str, telefono: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            telefono: telefono.to_string(),
            email: None,
            etiquetas: Vec::new(),
        }
    }

    fn con_email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    fn con_etiqueta(mut self, etiqueta: &str) -> Self {
        self.etiquetas.push(etiqueta.to_string());
        self
    }
}

struct Agenda {
    contactos: HashMap<String, Contacto>,
}

impl Agenda {
    fn new() -> Self {
        Self {
            contactos: HashMap::new(),
        }
    }

    /// Agrega o actualiza un contacto
    fn agregar(&mut self, contacto: Contacto) {
        self.contactos.insert(contacto.nombre.clone(), contacto);
    }

    /// Busca un contacto por nombre exacto
    fn buscar(&self, nombre: &str) -> Option<&Contacto> {
        self.contactos.get(nombre)
    }

    /// Busca contactos que contengan el texto en el nombre
    fn buscar_parcial(&self, texto: &str) -> Vec<&Contacto> {
        let texto_lower = texto.to_lowercase();
        self.contactos
            .values()
            .filter(|c| c.nombre.to_lowercase().contains(&texto_lower))
            .collect()
    }

    /// Busca contactos por etiqueta
    fn buscar_por_etiqueta(&self, etiqueta: &str) -> Vec<&Contacto> {
        self.contactos
            .values()
            .filter(|c| c.etiquetas.iter().any(|e| e == etiqueta))
            .collect()
    }

    /// Elimina un contacto
    fn eliminar(&mut self, nombre: &str) -> Option<Contacto> {
        self.contactos.remove(nombre)
    }

    /// Lista todos los contactos ordenados por nombre
    fn listar_todos(&self) -> Vec<&Contacto> {
        let mut contactos: Vec<_> = self.contactos.values().collect();
        contactos.sort_by(|a, b| a.nombre.cmp(&b.nombre));
        contactos
    }

    /// Obtiene todas las etiquetas únicas
    fn etiquetas_unicas(&self) -> Vec<String> {
        let mut etiquetas: Vec<String> = self
            .contactos
            .values()
            .flat_map(|c| c.etiquetas.clone())
            .collect();
        etiquetas.sort();
        etiquetas.dedup();
        etiquetas
    }

    /// Cuenta contactos por etiqueta
    fn contar_por_etiqueta(&self) -> HashMap<String, usize> {
        let mut conteo = HashMap::new();
        for contacto in self.contactos.values() {
            for etiqueta in &contacto.etiquetas {
                *conteo.entry(etiqueta.clone()).or_insert(0) += 1;
            }
        }
        conteo
    }

    /// Exporta la agenda como texto
    fn exportar(&self) -> String {
        let mut resultado = String::new();
        for contacto in self.listar_todos() {
            resultado.push_str(&format!(
                "📇 {}\n   📞 {}\n",
                contacto.nombre, contacto.telefono
            ));
            if let Some(email) = &contacto.email {
                resultado.push_str(&format!("   📧 {}\n", email));
            }
            if !contacto.etiquetas.is_empty() {
                resultado.push_str(&format!("   🏷️  {}\n", contacto.etiquetas.join(", ")));
            }
            resultado.push('\n');
        }
        resultado
    }
}

fn demo_agenda() {
    println!("--- Ejercicio 4: Agenda de Contactos ---");

    let mut agenda = Agenda::new();

    // Agregar contactos
    agenda.agregar(
        Contacto::new("Ana García", "555-1234")
            .con_email("ana@email.com")
            .con_etiqueta("familia")
            .con_etiqueta("favoritos"),
    );

    agenda.agregar(
        Contacto::new("Bob Smith", "555-5678")
            .con_email("bob@work.com")
            .con_etiqueta("trabajo"),
    );

    agenda.agregar(
        Contacto::new("Carlos López", "555-9012")
            .con_etiqueta("familia"),
    );

    agenda.agregar(
        Contacto::new("Ana Martínez", "555-3456")
            .con_etiqueta("trabajo")
            .con_etiqueta("favoritos"),
    );

    // Buscar por nombre
    println!("Buscar 'Ana García':");
    if let Some(contacto) = agenda.buscar("Ana García") {
        println!("  {:?}", contacto);
    }

    // Buscar parcial
    println!("\nBuscar 'Ana' (parcial):");
    for contacto in agenda.buscar_parcial("Ana") {
        println!("  - {}", contacto.nombre);
    }

    // Buscar por etiqueta
    println!("\nContactos con etiqueta 'familia':");
    for contacto in agenda.buscar_por_etiqueta("familia") {
        println!("  - {}", contacto.nombre);
    }

    // Etiquetas únicas
    println!("\nEtiquetas en uso: {:?}", agenda.etiquetas_unicas());

    // Conteo por etiqueta
    println!("Conteo por etiqueta: {:?}", agenda.contar_por_etiqueta());

    // Exportar
    println!("\n--- Agenda Completa ---\n{}", agenda.exportar());
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_crear_desde_vecs() {
        let claves = vec!["a", "b"];
        let valores = vec![1, 2];
        let mapa = crear_desde_vecs(claves, valores);
        assert_eq!(mapa.get("a"), Some(&1));
        assert_eq!(mapa.get("b"), Some(&2));
    }

    #[test]
    fn test_obtener_o_default() {
        let mut mapa = HashMap::new();
        mapa.insert("existe".to_string(), 42);
        let default = 0;
        assert_eq!(*obtener_o_default(&mapa, "existe", &default), 42);
        assert_eq!(*obtener_o_default(&mapa, "no_existe", &default), 0);
    }

    #[test]
    fn test_eliminar_menores_que() {
        let mut mapa: HashMap<String, i32> = [
            ("a".to_string(), 5),
            ("b".to_string(), 15),
            ("c".to_string(), 10),
        ]
        .into_iter()
        .collect();
        eliminar_menores_que(&mut mapa, 10);
        assert_eq!(mapa.len(), 2);
        assert!(!mapa.contains_key("a"));
    }

    #[test]
    fn test_invertir_mapa() {
        let original: HashMap<String, String> = [
            ("clave".to_string(), "valor".to_string()),
        ]
        .into_iter()
        .collect();
        let invertido = invertir_mapa(&original);
        assert_eq!(invertido.get("valor"), Some(&"clave".to_string()));
    }

    #[test]
    fn test_fusionar_sumando() {
        let mapa1: HashMap<String, i32> = [("a".to_string(), 10)].into_iter().collect();
        let mapa2: HashMap<String, i32> = [("a".to_string(), 5), ("b".to_string(), 20)].into_iter().collect();
        let fusionado = fusionar_sumando(&mapa1, &mapa2);
        assert_eq!(fusionado.get("a"), Some(&15));
        assert_eq!(fusionado.get("b"), Some(&20));
    }

    // Tests Ejercicio 2
    #[test]
    fn test_incrementar() {
        let mut mapa = HashMap::new();
        incrementar(&mut mapa, "a");
        incrementar(&mut mapa, "a");
        assert_eq!(mapa.get("a"), Some(&2));
    }

    #[test]
    fn test_establecer_si_ausente() {
        let mut mapa = HashMap::new();
        mapa.insert("existe".to_string(), "valor1".to_string());
        
        assert!(establecer_si_ausente(&mut mapa, "nuevo", "valor2"));
        assert!(!establecer_si_ausente(&mut mapa, "existe", "valor3"));
        assert_eq!(mapa.get("existe"), Some(&"valor1".to_string()));
    }

    #[test]
    fn test_agregar_a_lista() {
        let mut mapa: HashMap<String, Vec<String>> = HashMap::new();
        agregar_a_lista(&mut mapa, "key", "val1");
        agregar_a_lista(&mut mapa, "key", "val2");
        assert_eq!(mapa.get("key").unwrap().len(), 2);
    }

    // Tests Ejercicio 3
    #[test]
    fn test_contar_frecuencia() {
        let items = vec![1, 2, 2, 3, 3, 3];
        let freq = contar_frecuencia(&items);
        assert_eq!(freq.get(&1), Some(&1));
        assert_eq!(freq.get(&2), Some(&2));
        assert_eq!(freq.get(&3), Some(&3));
    }

    #[test]
    fn test_mas_frecuente() {
        let items = vec!["a", "b", "a", "c", "a"];
        let resultado = mas_frecuente(&items);
        assert_eq!(resultado, Some(("a", 3)));
    }

    #[test]
    fn test_agrupar_por() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let grupos = agrupar_por(nums, |n| n % 2);
        assert_eq!(grupos.get(&0).unwrap(), &vec![2, 4, 6]);
        assert_eq!(grupos.get(&1).unwrap(), &vec![1, 3, 5]);
    }

    #[test]
    fn test_contar_palabras() {
        let texto = "hola mundo hola";
        let conteo = contar_palabras(texto);
        assert_eq!(conteo.get("hola"), Some(&2));
        assert_eq!(conteo.get("mundo"), Some(&1));
    }

    // Tests Ejercicio 4
    #[test]
    fn test_agenda_agregar_buscar() {
        let mut agenda = Agenda::new();
        agenda.agregar(Contacto::new("Test", "123"));
        assert!(agenda.buscar("Test").is_some());
        assert!(agenda.buscar("NoExiste").is_none());
    }

    #[test]
    fn test_agenda_buscar_parcial() {
        let mut agenda = Agenda::new();
        agenda.agregar(Contacto::new("Ana García", "123"));
        agenda.agregar(Contacto::new("Ana López", "456"));
        agenda.agregar(Contacto::new("Bob", "789"));

        let resultados = agenda.buscar_parcial("Ana");
        assert_eq!(resultados.len(), 2);
    }

    #[test]
    fn test_agenda_buscar_por_etiqueta() {
        let mut agenda = Agenda::new();
        agenda.agregar(Contacto::new("A", "1").con_etiqueta("trabajo"));
        agenda.agregar(Contacto::new("B", "2").con_etiqueta("familia"));
        agenda.agregar(Contacto::new("C", "3").con_etiqueta("trabajo"));

        let trabajo = agenda.buscar_por_etiqueta("trabajo");
        assert_eq!(trabajo.len(), 2);
    }

    #[test]
    fn test_agenda_eliminar() {
        let mut agenda = Agenda::new();
        agenda.agregar(Contacto::new("Test", "123"));
        
        let eliminado = agenda.eliminar("Test");
        assert!(eliminado.is_some());
        assert!(agenda.buscar("Test").is_none());
    }

    #[test]
    fn test_agenda_etiquetas_unicas() {
        let mut agenda = Agenda::new();
        agenda.agregar(Contacto::new("A", "1").con_etiqueta("x").con_etiqueta("y"));
        agenda.agregar(Contacto::new("B", "2").con_etiqueta("x").con_etiqueta("z"));

        let etiquetas = agenda.etiquetas_unicas();
        assert_eq!(etiquetas, vec!["x", "y", "z"]);
    }
}
