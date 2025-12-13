// ============================================
// Practice 01: Basic Struct
// ============================================
// Objective: Define structs and create instances
// ============================================

// -----------------------------------------
// PART 1: Define Structs
// -----------------------------------------

// TODO: Define the Book struct
// Fields: title (String), author (String), pages (u32), available (bool)
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

// TODO: Define the Product struct
// Fields: id (u64), name (String), price (f64), stock (i32)
// struct Product { ... }

fn main() {
    println!("=== Practice 01: Basic Struct ===\n");

    // -----------------------------------------
    // PART 2: Create Instances
    // -----------------------------------------
    println!("--- Book Instances ---");
    
    let book1 = Book {
        title: String::from("Don Quixote"),
        author: String::from("Cervantes"),
        pages: 1200,
        available: true,
    };

    // TODO: Create book2 with different data
    // let book2 = Book { ... };

    // Access fields
    println!("Title: {}", book1.title);
    println!("Author: {}", book1.author);
    println!("Pages: {}", book1.pages);
    println!("Available: {}", book1.available);

    // TODO: Print book2 data
    
    // -----------------------------------------
    // PART 3: Mutable Instances
    // -----------------------------------------
    println!("\n--- Modify Fields ---");
    
    let mut mutable_book = Book {
        title: String::from("Rust Programming"),
        author: String::from("Steve Klabnik"),
        pages: 500,
        available: true,
    };

    println!("Before: available = {}", mutable_book.available);
    
    // Modify field
    mutable_book.available = false;
    mutable_book.pages = 552;
    
    println!("After: available = {}", mutable_book.available);
    println!("Updated pages: {}", mutable_book.pages);

    // -----------------------------------------
    // PART 4: Field Init Shorthand
    // -----------------------------------------
    println!("\n--- Field Init Shorthand ---");
    
    let title = String::from("Clean Code");
    let author = String::from("Robert Martin");
    let pages = 450;
    let available = true;

    // Long form
    let _long_book = Book {
        title: title.clone(),
        author: author.clone(),
        pages: pages,
        available: available,
    };

    // TODO: Use field init shorthand (when variable = field)
    // let short_book = Book { title, author, pages, available };

    println!("✅ Practice completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_book() {
        let book = Book {
            title: String::from("Test"),
            author: String::from("Author"),
            pages: 100,
            available: true,
        };

        assert_eq!(book.title, "Test");
        assert_eq!(book.pages, 100);
        assert!(book.available);
    }

    #[test]
    fn test_modify_book() {
        let mut book = Book {
            title: String::from("Test"),
            author: String::from("Author"),
            pages: 100,
            available: true,
        };

        book.available = false;
        book.pages = 150;

        assert!(!book.available);
        assert_eq!(book.pages, 150);
    }

    #[test]
    fn test_field_init_shorthand() {
        let title = String::from("Test");
        let author = String::from("Author");
        let pages = 100;
        let available = true;

        let book = Book { title, author, pages, available };

        assert_eq!(book.title, "Test");
        assert_eq!(book.pages, 100);
    }
}
