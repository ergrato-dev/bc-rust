// ============================================
// Práctica 01: Struct Básico
// ============================================
// Objetivo: Definir structs y crear instancias
// ============================================

// -----------------------------------------
// PARTE 1: Definir Structs
// -----------------------------------------

// TODO: Define el struct Libro
// Campos: titulo (String), autor (String), paginas (u32), disponible (bool)
struct Libro {
    titulo: String,
    autor: String,
    paginas: u32,
    disponible: bool,
}

// TODO: Define el struct Producto
// Campos: id (u64), nombre (String), precio (f64), stock (i32)
// struct Producto { ... }

fn main() {
    println!("=== Práctica 01: Struct Básico ===\n");

    // -----------------------------------------
    // PARTE 2: Crear Instancias
    // -----------------------------------------
    println!("--- Instancias de Libro ---");
    
    let libro1 = Libro {
        titulo: String::from("El Quijote"),
        autor: String::from("Cervantes"),
        paginas: 1200,
        disponible: true,
    };

    // TODO: Crea libro2 con otros datos
    // let libro2 = Libro { ... };

    // Acceder a campos
    println!("Título: {}", libro1.titulo);
    println!("Autor: {}", libro1.autor);
    println!("Páginas: {}", libro1.paginas);
    println!("Disponible: {}", libro1.disponible);

    // TODO: Imprime los datos de libro2
    
    // -----------------------------------------
    // PARTE 3: Instancias Mutables
    // -----------------------------------------
    println!("\n--- Modificar Campos ---");
    
    let mut libro_mutable = Libro {
        titulo: String::from("Rust Programming"),
        autor: String::from("Steve Klabnik"),
        paginas: 500,
        disponible: true,
    };

    println!("Antes: disponible = {}", libro_mutable.disponible);
    
    // Modificar campo
    libro_mutable.disponible = false;
    libro_mutable.paginas = 552;
    
    println!("Después: disponible = {}", libro_mutable.disponible);
    println!("Páginas actualizadas: {}", libro_mutable.paginas);

    // -----------------------------------------
    // PARTE 4: Field Init Shorthand
    // -----------------------------------------
    println!("\n--- Field Init Shorthand ---");
    
    let titulo = String::from("Clean Code");
    let autor = String::from("Robert Martin");
    let paginas = 450;
    let disponible = true;

    // Forma larga
    let _libro_largo = Libro {
        titulo: titulo.clone(),
        autor: autor.clone(),
        paginas: paginas,
        disponible: disponible,
    };

    // TODO: Usa field init shorthand (cuando variable = campo)
    // let libro_corto = Libro { titulo, autor, paginas, disponible };

    println!("✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_libro() {
        let libro = Libro {
            titulo: String::from("Test"),
            autor: String::from("Autor"),
            paginas: 100,
            disponible: true,
        };

        assert_eq!(libro.titulo, "Test");
        assert_eq!(libro.paginas, 100);
        assert!(libro.disponible);
    }

    #[test]
    fn test_modificar_libro() {
        let mut libro = Libro {
            titulo: String::from("Test"),
            autor: String::from("Autor"),
            paginas: 100,
            disponible: true,
        };

        libro.disponible = false;
        assert!(!libro.disponible);
    }

    #[test]
    fn test_field_init_shorthand() {
        let titulo = String::from("Libro");
        let autor = String::from("Autor");
        let paginas = 200;
        let disponible = false;

        let libro = Libro { titulo, autor, paginas, disponible };
        
        assert_eq!(libro.titulo, "Libro");
        assert_eq!(libro.paginas, 200);
    }
}
