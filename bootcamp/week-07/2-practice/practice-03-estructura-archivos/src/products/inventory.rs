// src/products/inventory.rs
// Product inventory management

/// Represents a product in the inventory
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    price: f64,
    pub quantity: u32,
}

impl Product {
    /// Creates a new product
    pub fn new(id: u32, name: &str, price: f64) -> Self {
        Product {
            id,
            name: name.to_string(),
            price,
            quantity: 0,
        }
    }

    /// Returns the base price
    pub fn base_price(&self) -> f64 {
        self.price
    }

    /// Adds stock to the product
    pub fn add_stock(&mut self, quantity: u32) {
        self.quantity += quantity;
    }

    /// Removes stock from the product, returns true if successful
    pub fn reduce_stock(&mut self, quantity: u32) -> bool {
        if self.quantity >= quantity {
            self.quantity -= quantity;
            true
        } else {
            false
        }
    }

    /// Returns the total value of the stock
    pub fn stock_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}
