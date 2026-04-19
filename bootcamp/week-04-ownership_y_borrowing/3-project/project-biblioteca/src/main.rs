// Project: Library System
// ================================
// A system that models the concept of borrowing in a real way

use std::fmt;

// ============================================
// Structures
// ============================================

#[derive(Debug, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
}

impl Book {
    pub fn new(isbn: &str, title: &str, author: &str) -> Self {
        Book {
            isbn: isbn.to_string(),
            title: title.to_string(),
            author: author.to_string(),
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" by {} ({})", self.title, self.author, self.isbn)
    }
}

#[derive(Debug)]
pub struct Loan {
    pub book: Book,
    pub user: String,
}

#[derive(Debug)]
pub struct Library {
    name: String,
    available: Vec<Book>,
    loaned: Vec<Loan>,
}

impl Library {
    /// Creates a new empty library
    pub fn new(name: &str) -> Self {
        Library {
            name: name.to_string(),
            available: Vec::new(),
            loaned: Vec::new(),
        }
    }
    
    /// Adds a book to the collection (takes ownership of the book)
    pub fn add_book(&mut self, book: Book) {
        self.available.push(book);
    }
    
    /// Lists available books (read only)
    pub fn list_available(&self) -> &[Book] {
        &self.available
    }
    
    /// Lists active loans (read only)
    pub fn list_loaned(&self) -> &[Loan] {
        &self.loaned
    }
    
    /// Searches for a book by ISBN (immutable reference)
    pub fn find_by_isbn(&self, isbn: &str) -> Option<&Book> {
        self.available.iter().find(|book| book.isbn == isbn)
    }
    
    /// Searches books by title (contains, case-insensitive)
    pub fn find_by_title(&self, title: &str) -> Vec<&Book> {
        let title_lower = title.to_lowercase();
        self.available
            .iter()
            .filter(|book| book.title.to_lowercase().contains(&title_lower))
            .collect()
    }
    
    /// Loans a book to a user (moves the book)
    pub fn loan(&mut self, isbn: &str, user: &str) -> Result<&Book, String> {
        // Find the book's index
        let pos = self.available
            .iter()
            .position(|book| book.isbn == isbn)
            .ok_or_else(|| format!("Book with ISBN '{}' not found or not available", isbn))?;
        
        // Move the book from available to loaned
        let book = self.available.remove(pos);
        
        self.loaned.push(Loan {
            book,
            user: user.to_string(),
        });
        
        // Return reference to the loaned book
        Ok(&self.loaned.last().unwrap().book)
    }
    
    /// Returns a loaned book (moves the book back)
    pub fn return_book(&mut self, isbn: &str) -> Result<(), String> {
        // Find the loan
        let pos = self.loaned
            .iter()
            .position(|l| l.book.isbn == isbn)
            .ok_or_else(|| format!("No active loan for ISBN '{}'", isbn))?;
        
        // Move the book from loaned to available
        let loan = self.loaned.remove(pos);
        self.available.push(loan.book);
        
        Ok(())
    }
    
    /// Counts available books
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
    
    /// Counts loaned books
    pub fn loaned_count(&self) -> usize {
        self.loaned.len()
    }
    
    /// Total books in the system
    pub fn total_books(&self) -> usize {
        self.available_count() + self.loaned_count()
    }
    
    /// Gets the library name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Checks if a book is available
    pub fn is_available(&self, isbn: &str) -> bool {
        self.available.iter().any(|b| b.isbn == isbn)
    }
    
    /// Gets who has a book loaned
    pub fn loaned_to(&self, isbn: &str) -> Option<&str> {
        self.loaned
            .iter()
            .find(|l| l.book.isbn == isbn)
            .map(|l| l.user.as_str())
    }
}

// ============================================
// Main Function
// ============================================

fn main() {
    println!("=== Library System ===\n");
    
    // Create library
    let mut library = Library::new("City Library");
    println!("📚 Created: {}\n", library.name());
    
    // Add books
    library.add_book(Book::new(
        "978-0-13-110362-7",
        "The C Programming Language",
        "Kernighan & Ritchie"
    ));
    library.add_book(Book::new(
        "978-1-59327-584-6",
        "The Linux Command Line",
        "William Shotts"
    ));
    library.add_book(Book::new(
        "978-1-718-50310-6",
        "The Rust Programming Language",
        "Klabnik & Nichols"
    ));
    
    println!("📖 Available books ({}):", library.available_count());
    for book in library.list_available() {
        println!("   - {}", book);
    }
    
    // Loan a book
    println!("\n🔄 Loaning 'The Rust Programming Language' to Ana...");
    match library.loan("978-1-718-50310-6", "Ana") {
        Ok(book) => println!("   ✅ Loaned: {}", book.title),
        Err(e) => println!("   ❌ Error: {}", e),
    }
    
    // Check status
    println!("\n📊 Current status:");
    println!("   Available: {}", library.available_count());
    println!("   Loaned: {}", library.loaned_count());
    
    // See who has the book
    if let Some(user) = library.loaned_to("978-1-718-50310-6") {
        println!("   'The Rust...' is with: {}", user);
    }
    
    // Try to loan the same book
    println!("\n🔄 Trying to loan the same book to Carlos...");
    match library.loan("978-1-718-50310-6", "Carlos") {
        Ok(_) => println!("   ✅ Loaned"),
        Err(e) => println!("   ❌ {}", e),
    }
    
    // Return book
    println!("\n🔄 Ana returns the book...");
    match library.return_book("978-1-718-50310-6") {
        Ok(()) => println!("   ✅ Returned successfully"),
        Err(e) => println!("   ❌ Error: {}", e),
    }
    
    // Search by title
    println!("\n🔍 Searching books with 'Programming':");
    for book in library.find_by_title("Programming") {
        println!("   - {}", book.title);
    }
    
    // Final status
    println!("\n📊 Final status:");
    println!("   Total: {} books", library.total_books());
    println!("   Available: {}", library.available_count());
    println!("   Loaned: {}", library.loaned_count());
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_library() -> Library {
        let mut lib = Library::new("Test");
        lib.add_book(Book::new("001", "Book A", "Author A"));
        lib.add_book(Book::new("002", "Book B", "Author B"));
        lib.add_book(Book::new("003", "Book C", "Author C"));
        lib
    }
    
    #[test]
    fn test_create_library() {
        let lib = Library::new("My Library");
        assert_eq!(lib.name(), "My Library");
        assert_eq!(lib.total_books(), 0);
    }
    
    #[test]
    fn test_add_books() {
        let lib = create_test_library();
        assert_eq!(lib.available_count(), 3);
        assert_eq!(lib.loaned_count(), 0);
    }
    
    #[test]
    fn test_loan_book() {
        let mut lib = create_test_library();
        
        let result = lib.loan("001", "User1");
        assert!(result.is_ok());
        
        assert_eq!(lib.available_count(), 2);
        assert_eq!(lib.loaned_count(), 1);
    }
    
    #[test]
    fn test_book_not_available_after_loan() {
        let mut lib = create_test_library();
        lib.loan("001", "User1").unwrap();
        
        assert!(!lib.is_available("001"));
        
        // Trying to loan again should fail
        let result = lib.loan("001", "User2");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_return_book() {
        let mut lib = create_test_library();
        lib.loan("001", "User1").unwrap();
        
        let result = lib.return_book("001");
        assert!(result.is_ok());
        
        assert!(lib.is_available("001"));
        assert_eq!(lib.available_count(), 3);
        assert_eq!(lib.loaned_count(), 0);
    }
    
    #[test]
    fn test_return_book_not_loaned() {
        let mut lib = create_test_library();
        
        let result = lib.return_book("001");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_find_by_isbn() {
        let lib = create_test_library();
        
        let found = lib.find_by_isbn("002");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Book B");
        
        let not_found = lib.find_by_isbn("999");
        assert!(not_found.is_none());
    }
    
    #[test]
    fn test_find_by_title() {
        let lib = create_test_library();
        
        let results = lib.find_by_title("book");
        assert_eq!(results.len(), 3); // All contain "Book"
        
        let results = lib.find_by_title("Book B");
        assert_eq!(results.len(), 1);
    }
    
    #[test]
    fn test_loaned_to() {
        let mut lib = create_test_library();
        
        assert!(lib.loaned_to("001").is_none());
        
        lib.loan("001", "Ana").unwrap();
        assert_eq!(lib.loaned_to("001"), Some("Ana"));
    }
    
    #[test]
    fn test_total_books_constant() {
        let mut lib = create_test_library();
        let initial_total = lib.total_books();
        
        lib.loan("001", "User").unwrap();
        assert_eq!(lib.total_books(), initial_total);
        
        lib.return_book("001").unwrap();
        assert_eq!(lib.total_books(), initial_total);
    }
    
    #[test]
    fn test_ownership_book_moves() {
        let mut lib = Library::new("Test");
        let book = Book::new("001", "My Book", "Author");
        
        // The book moves to the library
        lib.add_book(book);
        // book is no longer valid here
        
        assert_eq!(lib.available_count(), 1);
    }
}
