// Practice 04: Ownership in Functions
// ====================================

fn main() {
    println!("=== Practice 04: Ownership in Functions ===\n");
    
    exercise2();
    exercise3();
    exercise4();
}

// ============================================
// Exercise 2: Implement Functions
// ============================================

// a) Count characters - only needs to read
fn count_characters(s: &str) -> usize {
    s.chars().count()
}

// b) Convert to uppercase in-place
fn to_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

// c) Create greeting - returns new String
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// d) Take first element - consumes the vector
fn take_first<T>(mut vec: Vec<T>) -> Option<T> {
    if vec.is_empty() {
        None
    } else {
        Some(vec.remove(0))
    }
}

fn exercise2() {
    println!("--- Exercise 2: Implement Functions ---");
    
    // a) Count characters
    let text = String::from("Rust 🦀");
    let chars = count_characters(&text);
    println!("  '{}' has {} characters", text, chars);
    
    // b) To uppercase
    let mut message = String::from("hello world");
    to_uppercase(&mut message);
    println!("  Uppercase: {}", message);
    
    // c) Create greeting
    let greeting = create_greeting("Ferris");
    println!("  {}", greeting);
    
    // d) Take first
    let numbers = vec![10, 20, 30];
    if let Some(first) = take_first(numbers) {
        println!("  First element: {}", first);
    }
    // numbers is no longer valid here (was consumed)
    
    println!();
}

// ============================================
// Exercise 3: Refactor (without unnecessary clones)
// ============================================

// BEFORE (with unnecessary clones):
// fn process_data(data: Vec<i32>) -> i32 {
//     let copy = data.clone();
//     let sum: i32 = copy.iter().sum();
//     let max = copy.iter().max().unwrap_or(&0);
//     sum + max
// }

// AFTER (using reference):
fn process_data(data: &[i32]) -> i32 {
    let sum: i32 = data.iter().sum();
    let max = *data.iter().max().unwrap_or(&0);
    sum + max
}

fn exercise3() {
    println!("--- Exercise 3: Refactor ---");
    
    let my_data = vec![1, 2, 3, 4, 5];
    
    // Now we only pass reference, no cloning
    let result = process_data(&my_data);
    
    println!("  Data: {:?}", my_data);  // ✅ still valid
    println!("  Result: {} (sum + max)", result);
    println!();
}

// ============================================
// Exercise 4: Struct API
// ============================================

struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    // Constructor - doesn't need self
    fn new() -> Self {
        Inventory { items: Vec::new() }
    }
    
    // Add - needs to modify → &mut self
    fn add(&mut self, item: &str) {
        self.items.push(item.to_string());
    }
    
    // List - read only → &self
    fn list(&self) -> &[String] {
        &self.items
    }
    
    // Contains - read only → &self
    fn contains(&self, item: &str) -> bool {
        self.items.iter().any(|i| i == item)
    }
    
    // Remove - needs to modify → &mut self
    fn remove(&mut self, item: &str) -> Option<String> {
        if let Some(pos) = self.items.iter().position(|i| i == item) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
    
    // Count - read only → &self
    fn count(&self) -> usize {
        self.items.len()
    }
}

fn exercise4() {
    println!("--- Exercise 4: Struct API ---");
    
    let mut inv = Inventory::new();
    
    // Add items
    inv.add("Sword");
    inv.add("Shield");
    inv.add("Potion");
    
    println!("  Items: {:?}", inv.list());
    println!("  Count: {}", inv.count());
    
    // Search
    println!("  Has Sword? {}", inv.contains("Sword"));
    println!("  Has Bow? {}", inv.contains("Bow"));
    
    // Remove
    if let Some(item) = inv.remove("Shield") {
        println!("  Removed: {}", item);
    }
    
    println!("  Final items: {:?}", inv.list());
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_characters() {
        assert_eq!(count_characters("hello"), 5);
        assert_eq!(count_characters("🦀"), 1);
    }
    
    #[test]
    fn test_to_uppercase() {
        let mut s = String::from("rust");
        to_uppercase(&mut s);
        assert_eq!(s, "RUST");
    }
    
    #[test]
    fn test_create_greeting() {
        let greeting = create_greeting("World");
        assert_eq!(greeting, "Hello, World!");
    }
    
    #[test]
    fn test_take_first() {
        let vec = vec![1, 2, 3];
        assert_eq!(take_first(vec), Some(1));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(take_first(empty), None);
    }
    
    #[test]
    fn test_process_data() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(process_data(&data), 20); // 15 + 5
    }
    
    #[test]
    fn test_inventory_add_remove() {
        let mut inv = Inventory::new();
        inv.add("Item1");
        inv.add("Item2");
        
        assert_eq!(inv.count(), 2);
        assert!(inv.contains("Item1"));
        
        inv.remove("Item1");
        assert_eq!(inv.count(), 1);
        assert!(!inv.contains("Item1"));
    }
}
