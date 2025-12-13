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
fn create_from_vecs<K, V>(keys: Vec<K>, values: Vec<V>) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    keys.into_iter().zip(values).collect()
}

/// Obtiene un valor o un default si no existe
fn get_or_default<'a>(map: &'a HashMap<String, i32>, key: &str, default: &'a i32) -> &'a i32 {
    map.get(key).unwrap_or(default)
}

/// Elimina entradas con valor menor a un umbral
fn remove_less_than(map: &mut HashMap<String, i32>, threshold: i32) {
    map.retain(|_, v| *v >= threshold);
}

/// Invierte un mapa (valor -> clave)
fn invert_map(map: &HashMap<String, String>) -> HashMap<String, String> {
    map.iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect()
}

/// Fusiona dos mapas, sumando valores para claves duplicadas
fn merge_adding(
    map1: &HashMap<String, i32>,
    map2: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut result = map1.clone();
    for (key, value) in map2 {
        *result.entry(key.clone()).or_insert(0) += value;
    }
    result
}

fn demo_operaciones_basicas() {
    println!("--- Ejercicio 1: Operaciones Básicas ---");

    // Crear desde vectores
    let keys = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let values = vec![1, 2, 3];
    let map = create_from_vecs(keys, values);
    println!("Mapa desde vecs: {:?}", map);

    // Obtener o default
    let default = 0;
    println!("Valor de 'a': {}", get_or_default(&map, "a", &default));
    println!("Valor de 'z': {}", get_or_default(&map, "z", &default));

    // Eliminar menores
    let mut map2: HashMap<String, i32> = [
        ("bajo".to_string(), 5),
        ("medio".to_string(), 15),
        ("alto".to_string(), 25),
    ]
    .into_iter()
    .collect();
    remove_less_than(&mut map2, 10);
    println!("Después de eliminar < 10: {:?}", map2);

    // Invertir
    let translations: HashMap<String, String> = [
        ("hola".to_string(), "hello".to_string()),
        ("mundo".to_string(), "world".to_string()),
    ]
    .into_iter()
    .collect();
    let inverted = invert_map(&translations);
    println!("Invertido: {:?}", inverted);

    // Fusionar
    let map_a: HashMap<String, i32> =
        [("x".to_string(), 10), ("y".to_string(), 20)].into_iter().collect();
    let map_b: HashMap<String, i32> =
        [("y".to_string(), 5), ("z".to_string(), 15)].into_iter().collect();
    let merged = merge_adding(&map_a, &map_b);
    println!("Fusionado (sumando): {:?}", merged);

    println!();
}

// ============================================================================
// EJERCICIO 2: Entry API
// ============================================================================

/// Incrementa el contador para una clave
fn increment(map: &mut HashMap<String, i32>, key: &str) {
    *map.entry(key.to_string()).or_insert(0) += 1;
}

/// Establece un valor solo si la clave no existe
fn set_if_absent(map: &mut HashMap<String, String>, key: &str, value: &str) -> bool {
    use std::collections::hash_map::Entry;
    
    match map.entry(key.to_string()) {
        Entry::Vacant(e) => {
            e.insert(value.to_string());
            true
        }
        Entry::Occupied(_) => false,
    }
}

/// Actualiza un valor si existe, de lo contrario inserta un default
fn update_or_insert<F>(
    map: &mut HashMap<String, i32>,
    key: &str,
    default: i32,
    update: F,
) where
    F: FnOnce(&mut i32),
{
    map.entry(key.to_string())
        .and_modify(update)
        .or_insert(default);
}

/// Agrega un elemento a la lista asociada a una clave
fn add_to_list(map: &mut HashMap<String, Vec<String>>, key: &str, value: &str) {
    map.entry(key.to_string())
        .or_default()
        .push(value.to_string());
}

fn demo_entry_api() {
    println!("--- Ejercicio 2: Entry API ---");

    // Contador
    let mut counter: HashMap<String, i32> = HashMap::new();
    increment(&mut counter, "a");
    increment(&mut counter, "b");
    increment(&mut counter, "a");
    increment(&mut counter, "a");
    println!("Contador: {:?}", counter);

    // Establecer si ausente
    let mut config: HashMap<String, String> = HashMap::new();
    config.insert("tema".to_string(), "oscuro".to_string());
    
    let set1 = set_if_absent(&mut config, "idioma", "es");
    let set2 = set_if_absent(&mut config, "tema", "claro");
    println!("Idioma establecido: {} -> {:?}", set1, config.get("idioma"));
    println!("Tema establecido: {} -> {:?}", set2, config.get("tema"));

    // Actualizar o insertar
    let mut points: HashMap<String, i32> = HashMap::new();
    points.insert("alice".to_string(), 100);
    
    update_or_insert(&mut points, "alice", 0, |v| *v += 50);
    update_or_insert(&mut points, "bob", 25, |v| *v += 50);
    println!("Puntos: {:?}", points);

    // Agregar a lista
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();
    add_to_list(&mut categories, "frutas", "manzana");
    add_to_list(&mut categories, "frutas", "pera");
    add_to_list(&mut categories, "verduras", "zanahoria");
    println!("Categorías: {:?}", categories);

    println!();
}

// ============================================================================
// EJERCICIO 3: Contador y Agrupación
// ============================================================================

/// Cuenta la frecuencia de cada elemento
fn count_frequency<T>(elements: &[T]) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut frequency = HashMap::new();
    for element in elements {
        *frequency.entry(element.clone()).or_insert(0) += 1;
    }
    frequency
}

/// Encuentra el elemento más frecuente
fn most_frequent<T>(elements: &[T]) -> Option<(T, usize)>
where
    T: Eq + std::hash::Hash + Clone,
{
    let frequency = count_frequency(elements);
    frequency
        .into_iter()
        .max_by_key(|(_, count)| *count)
}

/// Agrupa elementos por el resultado de una función
fn group_by<T, K, F>(elements: Vec<T>, classifier: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for element in elements {
        let key = classifier(&element);
        groups.entry(key).or_default().push(element);
    }
    groups
}

/// Cuenta palabras en un texto
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut frequency = HashMap::new();
    for word in text.split_whitespace() {
        let clean_word = word
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();
        if !clean_word.is_empty() {
            *frequency.entry(clean_word).or_insert(0) += 1;
        }
    }
    frequency
}

/// Genera un histograma de texto
fn histogram(frequency: &HashMap<String, usize>) -> String {
    let mut result = String::new();
    let mut items: Vec<_> = frequency.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));

    for (word, count) in items.iter().take(5) {
        result.push_str(&format!("{}: {}\n", word, "#".repeat(**count)));
    }
    result
}

fn demo_contador_agrupacion() {
    println!("--- Ejercicio 3: Contador y Agrupación ---");

    // Frecuencia de números
    let numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let freq = count_frequency(&numbers);
    println!("Frecuencia: {:?}", freq);

    // Más frecuente
    if let Some((num, count)) = most_frequent(&numbers) {
        println!("Más frecuente: {} ({} veces)", num, count);
    }

    // Agrupar por paridad
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let by_parity = group_by(nums, |n| if n % 2 == 0 { "par" } else { "impar" });
    println!("Por paridad: {:?}", by_parity);

    // Agrupar por longitud
    let words = vec!["hola", "mundo", "rust", "es", "genial", "a"];
    let by_length = group_by(words, |s| s.len());
    println!("Por longitud: {:?}", by_length);

    // Contar palabras
    let text = "Rust es seguro. Rust es rápido. Rust es genial.";
    let word_count = count_words(text);
    println!("\nConteo de palabras en: '{}'", text);
    println!("{:?}", word_count);

    // Histograma
    println!("\nHistograma:");
    print!("{}", histogram(&word_count));

    println!();
}

// ============================================================================
// EJERCICIO 4: Agenda de Contactos
// ============================================================================

#[derive(Debug, Clone)]
struct Contact {
    name: String,
    phone: String,
    email: Option<String>,
    tags: Vec<String>,
}

impl Contact {
    fn new(name: &str, phone: &str) -> Self {
        Self {
            name: name.to_string(),
            phone: phone.to_string(),
            email: None,
            tags: Vec::new(),
        }
    }

    fn with_email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

struct AddressBook {
    contacts: HashMap<String, Contact>,
}

impl AddressBook {
    fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }

    /// Agrega o actualiza un contacto
    fn add(&mut self, contact: Contact) {
        self.contacts.insert(contact.name.clone(), contact);
    }

    /// Busca un contacto por nombre exacto
    fn find(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }

    /// Busca contactos que contengan el texto en el nombre
    fn find_partial(&self, text: &str) -> Vec<&Contact> {
        let text_lower = text.to_lowercase();
        self.contacts
            .values()
            .filter(|c| c.name.to_lowercase().contains(&text_lower))
            .collect()
    }

    /// Busca contactos por etiqueta
    fn find_by_tag(&self, tag: &str) -> Vec<&Contact> {
        self.contacts
            .values()
            .filter(|c| c.tags.iter().any(|e| e == tag))
            .collect()
    }

    /// Elimina un contacto
    fn delete(&mut self, name: &str) -> Option<Contact> {
        self.contacts.remove(name)
    }

    /// Lista todos los contactos ordenados por nombre
    fn list_all(&self) -> Vec<&Contact> {
        let mut contacts: Vec<_> = self.contacts.values().collect();
        contacts.sort_by(|a, b| a.name.cmp(&b.name));
        contacts
    }

    /// Obtiene todas las etiquetas únicas
    fn unique_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .contacts
            .values()
            .flat_map(|c| c.tags.clone())
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }

    /// Cuenta contactos por etiqueta
    fn count_by_tag(&self) -> HashMap<String, usize> {
        let mut count = HashMap::new();
        for contact in self.contacts.values() {
            for tag in &contact.tags {
                *count.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        count
    }

    /// Exporta la agenda como texto
    fn export(&self) -> String {
        let mut result = String::new();
        for contact in self.list_all() {
            result.push_str(&format!(
                "📇 {}\n   📞 {}\n",
                contact.name, contact.phone
            ));
            if let Some(email) = &contact.email {
                result.push_str(&format!("   📧 {}\n", email));
            }
            if !contact.tags.is_empty() {
                result.push_str(&format!("   🏷️  {}\n", contact.tags.join(", ")));
            }
            result.push('\n');
        }
        result
    }
}

fn demo_agenda() {
    println!("--- Ejercicio 4: Agenda de Contactos ---");

    let mut address_book = AddressBook::new();

    // Agregar contactos
    address_book.add(
        Contact::new("Ana García", "555-1234")
            .with_email("ana@email.com")
            .with_tag("familia")
            .with_tag("favoritos"),
    );

    address_book.add(
        Contact::new("Bob Smith", "555-5678")
            .with_email("bob@work.com")
            .with_tag("trabajo"),
    );

    address_book.add(
        Contact::new("Carlos López", "555-9012")
            .with_tag("familia"),
    );

    address_book.add(
        Contact::new("Ana Martínez", "555-3456")
            .with_tag("trabajo")
            .with_tag("favoritos"),
    );

    // Buscar por nombre
    println!("Buscar 'Ana García':");
    if let Some(contact) = address_book.find("Ana García") {
        println!("  {:?}", contact);
    }

    // Buscar parcial
    println!("\nBuscar 'Ana' (parcial):");
    for contact in address_book.find_partial("Ana") {
        println!("  - {}", contact.name);
    }

    // Buscar por etiqueta
    println!("\nContactos con etiqueta 'familia':");
    for contact in address_book.find_by_tag("familia") {
        println!("  - {}", contact.name);
    }

    // Etiquetas únicas
    println!("\nEtiquetas en uso: {:?}", address_book.unique_tags());

    // Conteo por etiqueta
    println!("Conteo por etiqueta: {:?}", address_book.count_by_tag());

    // Exportar
    println!("\n--- Agenda Completa ---\n{}", address_book.export());
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_create_from_vecs() {
        let keys = vec!["a", "b"];
        let values = vec![1, 2];
        let map = create_from_vecs(keys, values);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_get_or_default() {
        let mut map = HashMap::new();
        map.insert("existe".to_string(), 42);
        let default = 0;
        assert_eq!(*get_or_default(&map, "existe", &default), 42);
        assert_eq!(*get_or_default(&map, "no_existe", &default), 0);
    }

    #[test]
    fn test_remove_less_than() {
        let mut map: HashMap<String, i32> = [
            ("a".to_string(), 5),
            ("b".to_string(), 15),
            ("c".to_string(), 10),
        ]
        .into_iter()
        .collect();
        remove_less_than(&mut map, 10);
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key("a"));
    }

    #[test]
    fn test_invert_map() {
        let original: HashMap<String, String> = [
            ("clave".to_string(), "valor".to_string()),
        ]
        .into_iter()
        .collect();
        let inverted = invert_map(&original);
        assert_eq!(inverted.get("valor"), Some(&"clave".to_string()));
    }

    #[test]
    fn test_merge_adding() {
        let map1: HashMap<String, i32> = [("a".to_string(), 10)].into_iter().collect();
        let map2: HashMap<String, i32> = [("a".to_string(), 5), ("b".to_string(), 20)].into_iter().collect();
        let merged = merge_adding(&map1, &map2);
        assert_eq!(merged.get("a"), Some(&15));
        assert_eq!(merged.get("b"), Some(&20));
    }

    // Tests Ejercicio 2
    #[test]
    fn test_increment() {
        let mut map = HashMap::new();
        increment(&mut map, "a");
        increment(&mut map, "a");
        assert_eq!(map.get("a"), Some(&2));
    }

    #[test]
    fn test_set_if_absent() {
        let mut map = HashMap::new();
        map.insert("existe".to_string(), "valor1".to_string());
        
        assert!(set_if_absent(&mut map, "nuevo", "valor2"));
        assert!(!set_if_absent(&mut map, "existe", "valor3"));
        assert_eq!(map.get("existe"), Some(&"valor1".to_string()));
    }

    #[test]
    fn test_add_to_list() {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        add_to_list(&mut map, "key", "val1");
        add_to_list(&mut map, "key", "val2");
        assert_eq!(map.get("key").unwrap().len(), 2);
    }

    // Tests Ejercicio 3
    #[test]
    fn test_count_frequency() {
        let items = vec![1, 2, 2, 3, 3, 3];
        let freq = count_frequency(&items);
        assert_eq!(freq.get(&1), Some(&1));
        assert_eq!(freq.get(&2), Some(&2));
        assert_eq!(freq.get(&3), Some(&3));
    }

    #[test]
    fn test_most_frequent() {
        let items = vec!["a", "b", "a", "c", "a"];
        let result = most_frequent(&items);
        assert_eq!(result, Some(("a", 3)));
    }

    #[test]
    fn test_group_by() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let groups = group_by(nums, |n| n % 2);
        assert_eq!(groups.get(&0).unwrap(), &vec![2, 4, 6]);
        assert_eq!(groups.get(&1).unwrap(), &vec![1, 3, 5]);
    }

    #[test]
    fn test_count_words() {
        let text = "hola mundo hola";
        let word_count = count_words(text);
        assert_eq!(word_count.get("hola"), Some(&2));
        assert_eq!(word_count.get("mundo"), Some(&1));
    }

    // Tests Ejercicio 4
    #[test]
    fn test_address_book_add_find() {
        let mut address_book = AddressBook::new();
        address_book.add(Contact::new("Test", "123"));
        assert!(address_book.find("Test").is_some());
        assert!(address_book.find("NoExiste").is_none());
    }

    #[test]
    fn test_address_book_find_partial() {
        let mut address_book = AddressBook::new();
        address_book.add(Contact::new("Ana García", "123"));
        address_book.add(Contact::new("Ana López", "456"));
        address_book.add(Contact::new("Bob", "789"));

        let results = address_book.find_partial("Ana");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_address_book_find_by_tag() {
        let mut address_book = AddressBook::new();
        address_book.add(Contact::new("A", "1").with_tag("trabajo"));
        address_book.add(Contact::new("B", "2").with_tag("familia"));
        address_book.add(Contact::new("C", "3").with_tag("trabajo"));

        let work = address_book.find_by_tag("trabajo");
        assert_eq!(work.len(), 2);
    }

    #[test]
    fn test_address_book_delete() {
        let mut address_book = AddressBook::new();
        address_book.add(Contact::new("Test", "123"));
        
        let deleted = address_book.delete("Test");
        assert!(deleted.is_some());
        assert!(address_book.find("Test").is_none());
    }

    #[test]
    fn test_address_book_unique_tags() {
        let mut address_book = AddressBook::new();
        address_book.add(Contact::new("A", "1").with_tag("x").with_tag("y"));
        address_book.add(Contact::new("B", "2").with_tag("x").with_tag("z"));

        let tags = address_book.unique_tags();
        assert_eq!(tags, vec!["x", "y", "z"]);
    }
}
