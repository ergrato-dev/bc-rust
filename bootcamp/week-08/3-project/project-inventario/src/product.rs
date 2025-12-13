//! Product module

use std::fmt;

/// Represents a product in the inventory
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub stock: u32,
}

impl Product {
    /// Creates a new product
    pub fn new(
        id: u32,
        name: impl Into<String>,
        description: impl Into<String>,
        price: f64,
        category: impl Into<String>,
        stock: u32,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            price,
            category: category.into(),
            stock,
        }
    }

    /// Calculates total inventory value
    pub fn inventory_value(&self) -> f64 {
        self.price * self.stock as f64
    }

    /// Checks if stock is below threshold
    pub fn low_stock(&self, threshold: u32) -> bool {
        self.stock < threshold
    }

    /// Updates stock (entry)
    pub fn add_stock(&mut self, quantity: u32) {
        self.stock += quantity;
    }

    /// Updates stock (exit), returns true if successful
    pub fn remove_stock(&mut self, quantity: u32) -> bool {
        if self.stock >= quantity {
            self.stock -= quantity;
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} - ${:.2} (Stock: {}) [{}]",
            self.id, self.name, self.price, self.stock, self.category
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_product() {
        let p = Product::new(1, "Test", "Desc", 10.0, "Cat", 5);
        assert_eq!(p.id, 1);
        assert_eq!(p.name, "Test");
        assert_eq!(p.stock, 5);
    }

    #[test]
    fn test_inventory_value() {
        let p = Product::new(1, "Test", "Desc", 10.0, "Cat", 5);
        assert_eq!(p.inventory_value(), 50.0);
    }

    #[test]
    fn test_low_stock() {
        let p = Product::new(1, "Test", "Desc", 10.0, "Cat", 3);
        assert!(p.low_stock(5));
        assert!(!p.low_stock(2));
    }

    #[test]
    fn test_add_stock() {
        let mut p = Product::new(1, "Test", "Desc", 10.0, "Cat", 5);
        p.add_stock(10);
        assert_eq!(p.stock, 15);
    }

    #[test]
    fn test_remove_stock_success() {
        let mut p = Product::new(1, "Test", "Desc", 10.0, "Cat", 10);
        assert!(p.remove_stock(5));
        assert_eq!(p.stock, 5);
    }

    #[test]
    fn test_remove_stock_insufficient() {
        let mut p = Product::new(1, "Test", "Desc", 10.0, "Cat", 3);
        assert!(!p.remove_stock(5));
        assert_eq!(p.stock, 3);
    }
}
