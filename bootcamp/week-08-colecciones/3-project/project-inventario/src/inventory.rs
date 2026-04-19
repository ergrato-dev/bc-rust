//! Main inventory module

use std::collections::HashMap;
use crate::product::Product;
use crate::transaction::{Transaction, TransactionType};

/// Inventory management system
#[derive(Debug)]
pub struct Inventory {
    products: HashMap<u32, Product>,
    transactions: Vec<Transaction>,
    next_product_id: u32,
    next_transaction_id: u32,
}

impl Inventory {
    /// Creates a new empty inventory
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            transactions: Vec::new(),
            next_product_id: 1,
            next_transaction_id: 1,
        }
    }

    // ========== Product CRUD ==========

    /// Adds a new product and returns its ID
    pub fn add_product(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        price: f64,
        category: impl Into<String>,
        initial_stock: u32,
    ) -> u32 {
        let id = self.next_product_id;
        self.next_product_id += 1;

        let product = Product::new(id, name, description, price, category, initial_stock);
        self.products.insert(id, product);

        // Register initial entry if there is stock
        if initial_stock > 0 {
            self.register_transaction(id, TransactionType::Entry, initial_stock, "Initial stock");
        }

        id
    }

    /// Gets a product by ID
    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    /// Gets a mutable product by ID
    pub fn get_product_mut(&mut self, id: u32) -> Option<&mut Product> {
        self.products.get_mut(&id)
    }

    /// Updates the price of a product
    pub fn update_price(&mut self, id: u32, new_price: f64) -> bool {
        if let Some(product) = self.products.get_mut(&id) {
            product.price = new_price;
            true
        } else {
            false
        }
    }

    /// Deletes a product
    pub fn delete_product(&mut self, id: u32) -> Option<Product> {
        self.products.remove(&id)
    }

    /// Returns all products
    pub fn list_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    /// Returns products sorted by name
    pub fn products_sorted_by_name(&self) -> Vec<&Product> {
        let mut products: Vec<_> = self.products.values().collect();
        products.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        products
    }

    // ========== Searches ==========

    /// Searches products by name (partial match)
    pub fn search_by_name(&self, text: &str) -> Vec<&Product> {
        let text_lower = text.to_lowercase();
        self.products
            .values()
            .filter(|p| p.name.to_lowercase().contains(&text_lower))
            .collect()
    }

    /// Searches products by exact category
    pub fn search_by_category(&self, category: &str) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Searches products with low stock
    pub fn low_stock_products(&self, threshold: u32) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| p.low_stock(threshold))
            .collect()
    }

    /// Searches products without stock
    pub fn out_of_stock_products(&self) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| p.stock == 0)
            .collect()
    }

    // ========== Categories ==========

    /// Gets all unique categories
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.products
            .values()
            .map(|p| p.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Counts products by category
    pub fn count_by_category(&self) -> HashMap<String, usize> {
        let mut count: HashMap<String, usize> = HashMap::new();
        for product in self.products.values() {
            *count.entry(product.category.clone()).or_insert(0) += 1;
        }
        count
    }

    // ========== Stock Transactions ==========

    /// Registers a stock entry
    pub fn stock_entry(&mut self, product_id: u32, quantity: u32, note: &str) -> bool {
        if let Some(product) = self.products.get_mut(&product_id) {
            product.add_stock(quantity);
            self.register_transaction(product_id, TransactionType::Entry, quantity, note);
            true
        } else {
            false
        }
    }

    /// Registers a stock exit
    pub fn stock_exit(&mut self, product_id: u32, quantity: u32, note: &str) -> bool {
        if let Some(product) = self.products.get_mut(&product_id) {
            if product.remove_stock(quantity) {
                self.register_transaction(product_id, TransactionType::Exit, quantity, note);
                true
            } else {
                false // Insufficient stock
            }
        } else {
            false // Product not found
        }
    }

    /// Registers an internal transaction
    fn register_transaction(
        &mut self,
        product_id: u32,
        transaction_type: TransactionType,
        quantity: u32,
        note: &str,
    ) {
        let id = self.next_transaction_id;
        self.next_transaction_id += 1;

        let date = get_current_date();
        let mut transaction = Transaction::new(id, product_id, transaction_type, quantity, date);
        if !note.is_empty() {
            transaction = transaction.with_note(note);
        }
        self.transactions.push(transaction);
    }

    /// Gets the transaction history
    pub fn transaction_history(&self) -> &[Transaction] {
        &self.transactions
    }

    /// Gets transactions for a specific product
    pub fn product_transactions(&self, product_id: u32) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.product_id == product_id)
            .collect()
    }

    // ========== Statistics ==========

    /// Total number of products
    pub fn total_products(&self) -> usize {
        self.products.len()
    }

    /// Total units in stock
    pub fn total_units(&self) -> u32 {
        self.products.values().map(|p| p.stock).sum()
    }

    /// Total inventory value
    pub fn total_value(&self) -> f64 {
        self.products.values().map(|p| p.inventory_value()).sum()
    }

    /// Average product price
    pub fn average_price(&self) -> f64 {
        if self.products.is_empty() {
            return 0.0;
        }
        let total: f64 = self.products.values().map(|p| p.price).sum();
        total / self.products.len() as f64
    }

    /// Most expensive product
    pub fn most_expensive_product(&self) -> Option<&Product> {
        self.products
            .values()
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    }

    /// Cheapest product
    pub fn cheapest_product(&self) -> Option<&Product> {
        self.products
            .values()
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    }

    /// Top N products by inventory value
    pub fn top_products_by_value(&self, n: usize) -> Vec<&Product> {
        let mut products: Vec<_> = self.products.values().collect();
        products.sort_by(|a, b| {
            b.inventory_value()
                .partial_cmp(&a.inventory_value())
                .unwrap()
        });
        products.into_iter().take(n).collect()
    }

    /// Value by category
    pub fn value_by_category(&self) -> HashMap<String, f64> {
        let mut values: HashMap<String, f64> = HashMap::new();
        for product in self.products.values() {
            *values.entry(product.category.clone()).or_insert(0.0) 
                += product.inventory_value();
        }
        values
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

/// Gets the current date as string (simplified)
fn get_current_date() -> String {
    // In a real project we would use chrono
    "2025-01-15".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_inventory() -> Inventory {
        let mut inv = Inventory::new();
        inv.add_product("Laptop", "15\" Laptop", 999.99, "Electronics", 10);
        inv.add_product("Mouse", "Wireless mouse", 29.99, "Electronics", 50);
        inv.add_product("Chair", "Ergonomic chair", 199.99, "Furniture", 5);
        inv
    }

    #[test]
    fn test_add_product() {
        let mut inv = Inventory::new();
        let id = inv.add_product("Test", "Desc", 10.0, "Cat", 5);
        assert_eq!(id, 1);
        assert!(inv.get_product(1).is_some());
    }

    #[test]
    fn test_delete_product() {
        let mut inv = create_test_inventory();
        let deleted = inv.delete_product(1);
        assert!(deleted.is_some());
        assert!(inv.get_product(1).is_none());
    }

    #[test]
    fn test_search_by_name() {
        let inv = create_test_inventory();
        let results = inv.search_by_name("lap");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_search_by_category() {
        let inv = create_test_inventory();
        let results = inv.search_by_category("Electronics");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_categories() {
        let inv = create_test_inventory();
        let cats = inv.categories();
        assert_eq!(cats.len(), 2);
        assert!(cats.contains(&"Electronics".to_string()));
        assert!(cats.contains(&"Furniture".to_string()));
    }

    #[test]
    fn test_stock_entry() {
        let mut inv = create_test_inventory();
        let stock_before = inv.get_product(1).unwrap().stock;
        
        assert!(inv.stock_entry(1, 5, "Restocking"));
        
        let stock_after = inv.get_product(1).unwrap().stock;
        assert_eq!(stock_after, stock_before + 5);
    }

    #[test]
    fn test_stock_exit_success() {
        let mut inv = create_test_inventory();
        let stock_before = inv.get_product(1).unwrap().stock;
        
        assert!(inv.stock_exit(1, 3, "Sale"));
        
        let stock_after = inv.get_product(1).unwrap().stock;
        assert_eq!(stock_after, stock_before - 3);
    }

    #[test]
    fn test_stock_exit_insufficient() {
        let mut inv = create_test_inventory();
        let stock_before = inv.get_product(1).unwrap().stock;
        
        assert!(!inv.stock_exit(1, 100, "Large sale"));
        
        let stock_after = inv.get_product(1).unwrap().stock;
        assert_eq!(stock_after, stock_before); // No change
    }

    #[test]
    fn test_transaction_history() {
        let mut inv = Inventory::new();
        inv.add_product("Test", "Desc", 10.0, "Cat", 10);
        inv.stock_entry(1, 5, "Entry");
        inv.stock_exit(1, 3, "Exit");
        
        let history = inv.transaction_history();
        // 1 initial entry + 1 entry + 1 exit = 3
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_total_value() {
        let inv = create_test_inventory();
        // Laptop: 999.99 * 10 = 9999.90
        // Mouse: 29.99 * 50 = 1499.50
        // Chair: 199.99 * 5 = 999.95
        // Total: 12499.35
        let value = inv.total_value();
        assert!((value - 12499.35).abs() < 0.01);
    }

    #[test]
    fn test_most_expensive_product() {
        let inv = create_test_inventory();
        let most_expensive = inv.most_expensive_product().unwrap();
        assert_eq!(most_expensive.name, "Laptop");
    }

    #[test]
    fn test_top_products_by_value() {
        let inv = create_test_inventory();
        let top = inv.top_products_by_value(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].name, "Laptop"); // Highest value: 9999.90
    }

    #[test]
    fn test_low_stock_products() {
        let inv = create_test_inventory();
        let low = inv.low_stock_products(10);
        assert_eq!(low.len(), 1); // Only Chair (5 units)
    }
}
