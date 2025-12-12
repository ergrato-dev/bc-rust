// Proyecto: Sistema de Biblioteca
// ================================
// Un sistema que modela el concepto de préstamo (borrowing) de forma real

use std::fmt;

// ============================================
// Estructuras
// ============================================

#[derive(Debug, Clone)]
pub struct Libro {
    pub isbn: String,
    pub titulo: String,
    pub autor: String,
}

impl Libro {
    pub fn new(isbn: &str, titulo: &str, autor: &str) -> Self {
        Libro {
            isbn: isbn.to_string(),
            titulo: titulo.to_string(),
            autor: autor.to_string(),
        }
    }
}

impl fmt::Display for Libro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" por {} ({})", self.titulo, self.autor, self.isbn)
    }
}

#[derive(Debug)]
pub struct Prestamo {
    pub libro: Libro,
    pub usuario: String,
}

#[derive(Debug)]
pub struct Biblioteca {
    nombre: String,
    disponibles: Vec<Libro>,
    prestados: Vec<Prestamo>,
}

impl Biblioteca {
    /// Crea una nueva biblioteca vacía
    pub fn new(nombre: &str) -> Self {
        Biblioteca {
            nombre: nombre.to_string(),
            disponibles: Vec::new(),
            prestados: Vec::new(),
        }
    }
    
    /// Agrega un libro a la colección (toma ownership del libro)
    pub fn agregar_libro(&mut self, libro: Libro) {
        self.disponibles.push(libro);
    }
    
    /// Lista los libros disponibles (solo lectura)
    pub fn listar_disponibles(&self) -> &[Libro] {
        &self.disponibles
    }
    
    /// Lista los préstamos activos (solo lectura)
    pub fn listar_prestados(&self) -> &[Prestamo] {
        &self.prestados
    }
    
    /// Busca un libro por ISBN (referencia inmutable)
    pub fn buscar_por_isbn(&self, isbn: &str) -> Option<&Libro> {
        self.disponibles.iter().find(|libro| libro.isbn == isbn)
    }
    
    /// Busca libros por título (contiene, case-insensitive)
    pub fn buscar_por_titulo(&self, titulo: &str) -> Vec<&Libro> {
        let titulo_lower = titulo.to_lowercase();
        self.disponibles
            .iter()
            .filter(|libro| libro.titulo.to_lowercase().contains(&titulo_lower))
            .collect()
    }
    
    /// Presta un libro a un usuario (mueve el libro)
    pub fn prestar(&mut self, isbn: &str, usuario: &str) -> Result<&Libro, String> {
        // Buscar el índice del libro
        let pos = self.disponibles
            .iter()
            .position(|libro| libro.isbn == isbn)
            .ok_or_else(|| format!("Libro con ISBN '{}' no encontrado o no disponible", isbn))?;
        
        // Mover el libro de disponibles a prestados
        let libro = self.disponibles.remove(pos);
        
        self.prestados.push(Prestamo {
            libro,
            usuario: usuario.to_string(),
        });
        
        // Retornar referencia al libro prestado
        Ok(&self.prestados.last().unwrap().libro)
    }
    
    /// Devuelve un libro prestado (mueve el libro de vuelta)
    pub fn devolver(&mut self, isbn: &str) -> Result<(), String> {
        // Buscar el préstamo
        let pos = self.prestados
            .iter()
            .position(|p| p.libro.isbn == isbn)
            .ok_or_else(|| format!("No hay préstamo activo para ISBN '{}'", isbn))?;
        
        // Mover el libro de prestados a disponibles
        let prestamo = self.prestados.remove(pos);
        self.disponibles.push(prestamo.libro);
        
        Ok(())
    }
    
    /// Cuenta libros disponibles
    pub fn cantidad_disponibles(&self) -> usize {
        self.disponibles.len()
    }
    
    /// Cuenta libros prestados
    pub fn cantidad_prestados(&self) -> usize {
        self.prestados.len()
    }
    
    /// Total de libros en el sistema
    pub fn total_libros(&self) -> usize {
        self.cantidad_disponibles() + self.cantidad_prestados()
    }
    
    /// Obtiene el nombre de la biblioteca
    pub fn nombre(&self) -> &str {
        &self.nombre
    }
    
    /// Verifica si un libro está disponible
    pub fn esta_disponible(&self, isbn: &str) -> bool {
        self.disponibles.iter().any(|l| l.isbn == isbn)
    }
    
    /// Obtiene quién tiene prestado un libro
    pub fn prestado_a(&self, isbn: &str) -> Option<&str> {
        self.prestados
            .iter()
            .find(|p| p.libro.isbn == isbn)
            .map(|p| p.usuario.as_str())
    }
}

// ============================================
// Función Principal
// ============================================

fn main() {
    println!("=== Sistema de Biblioteca ===\n");
    
    // Crear biblioteca
    let mut biblioteca = Biblioteca::new("Biblioteca Municipal");
    println!("📚 Creada: {}\n", biblioteca.nombre());
    
    // Agregar libros
    biblioteca.agregar_libro(Libro::new(
        "978-0-13-110362-7",
        "The C Programming Language",
        "Kernighan & Ritchie"
    ));
    biblioteca.agregar_libro(Libro::new(
        "978-1-59327-584-6",
        "The Linux Command Line",
        "William Shotts"
    ));
    biblioteca.agregar_libro(Libro::new(
        "978-1-718-50310-6",
        "The Rust Programming Language",
        "Klabnik & Nichols"
    ));
    
    println!("📖 Libros disponibles ({}):", biblioteca.cantidad_disponibles());
    for libro in biblioteca.listar_disponibles() {
        println!("   - {}", libro);
    }
    
    // Prestar un libro
    println!("\n🔄 Prestando 'The Rust Programming Language' a Ana...");
    match biblioteca.prestar("978-1-718-50310-6", "Ana") {
        Ok(libro) => println!("   ✅ Prestado: {}", libro.titulo),
        Err(e) => println!("   ❌ Error: {}", e),
    }
    
    // Verificar estado
    println!("\n📊 Estado actual:");
    println!("   Disponibles: {}", biblioteca.cantidad_disponibles());
    println!("   Prestados: {}", biblioteca.cantidad_prestados());
    
    // Ver quién tiene el libro
    if let Some(usuario) = biblioteca.prestado_a("978-1-718-50310-6") {
        println!("   'The Rust...' está con: {}", usuario);
    }
    
    // Intentar prestar el mismo libro
    println!("\n🔄 Intentando prestar el mismo libro a Carlos...");
    match biblioteca.prestar("978-1-718-50310-6", "Carlos") {
        Ok(_) => println!("   ✅ Prestado"),
        Err(e) => println!("   ❌ {}", e),
    }
    
    // Devolver libro
    println!("\n🔄 Ana devuelve el libro...");
    match biblioteca.devolver("978-1-718-50310-6") {
        Ok(()) => println!("   ✅ Devuelto correctamente"),
        Err(e) => println!("   ❌ Error: {}", e),
    }
    
    // Buscar por título
    println!("\n🔍 Buscando libros con 'Programming':");
    for libro in biblioteca.buscar_por_titulo("Programming") {
        println!("   - {}", libro.titulo);
    }
    
    // Estado final
    println!("\n📊 Estado final:");
    println!("   Total: {} libros", biblioteca.total_libros());
    println!("   Disponibles: {}", biblioteca.cantidad_disponibles());
    println!("   Prestados: {}", biblioteca.cantidad_prestados());
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn crear_biblioteca_test() -> Biblioteca {
        let mut bib = Biblioteca::new("Test");
        bib.agregar_libro(Libro::new("001", "Libro A", "Autor A"));
        bib.agregar_libro(Libro::new("002", "Libro B", "Autor B"));
        bib.agregar_libro(Libro::new("003", "Libro C", "Autor C"));
        bib
    }
    
    #[test]
    fn test_crear_biblioteca() {
        let bib = Biblioteca::new("Mi Biblioteca");
        assert_eq!(bib.nombre(), "Mi Biblioteca");
        assert_eq!(bib.total_libros(), 0);
    }
    
    #[test]
    fn test_agregar_libros() {
        let bib = crear_biblioteca_test();
        assert_eq!(bib.cantidad_disponibles(), 3);
        assert_eq!(bib.cantidad_prestados(), 0);
    }
    
    #[test]
    fn test_prestar_libro() {
        let mut bib = crear_biblioteca_test();
        
        let resultado = bib.prestar("001", "Usuario1");
        assert!(resultado.is_ok());
        
        assert_eq!(bib.cantidad_disponibles(), 2);
        assert_eq!(bib.cantidad_prestados(), 1);
    }
    
    #[test]
    fn test_libro_no_disponible_despues_de_prestar() {
        let mut bib = crear_biblioteca_test();
        bib.prestar("001", "Usuario1").unwrap();
        
        assert!(!bib.esta_disponible("001"));
        
        // Intentar prestar de nuevo debe fallar
        let resultado = bib.prestar("001", "Usuario2");
        assert!(resultado.is_err());
    }
    
    #[test]
    fn test_devolver_libro() {
        let mut bib = crear_biblioteca_test();
        bib.prestar("001", "Usuario1").unwrap();
        
        let resultado = bib.devolver("001");
        assert!(resultado.is_ok());
        
        assert!(bib.esta_disponible("001"));
        assert_eq!(bib.cantidad_disponibles(), 3);
        assert_eq!(bib.cantidad_prestados(), 0);
    }
    
    #[test]
    fn test_devolver_libro_no_prestado() {
        let mut bib = crear_biblioteca_test();
        
        let resultado = bib.devolver("001");
        assert!(resultado.is_err());
    }
    
    #[test]
    fn test_buscar_por_isbn() {
        let bib = crear_biblioteca_test();
        
        let encontrado = bib.buscar_por_isbn("002");
        assert!(encontrado.is_some());
        assert_eq!(encontrado.unwrap().titulo, "Libro B");
        
        let no_encontrado = bib.buscar_por_isbn("999");
        assert!(no_encontrado.is_none());
    }
    
    #[test]
    fn test_buscar_por_titulo() {
        let bib = crear_biblioteca_test();
        
        let resultados = bib.buscar_por_titulo("libro");
        assert_eq!(resultados.len(), 3); // Todos contienen "Libro"
        
        let resultados = bib.buscar_por_titulo("B");
        assert_eq!(resultados.len(), 1);
    }
    
    #[test]
    fn test_prestado_a() {
        let mut bib = crear_biblioteca_test();
        
        assert!(bib.prestado_a("001").is_none());
        
        bib.prestar("001", "Ana").unwrap();
        assert_eq!(bib.prestado_a("001"), Some("Ana"));
    }
    
    #[test]
    fn test_total_libros_constante() {
        let mut bib = crear_biblioteca_test();
        let total_inicial = bib.total_libros();
        
        bib.prestar("001", "Usuario").unwrap();
        assert_eq!(bib.total_libros(), total_inicial);
        
        bib.devolver("001").unwrap();
        assert_eq!(bib.total_libros(), total_inicial);
    }
    
    #[test]
    fn test_ownership_libro_se_mueve() {
        let mut bib = Biblioteca::new("Test");
        let libro = Libro::new("001", "Mi Libro", "Autor");
        
        // El libro se mueve a la biblioteca
        bib.agregar_libro(libro);
        // libro ya no es válido aquí
        
        assert_eq!(bib.cantidad_disponibles(), 1);
    }
}
