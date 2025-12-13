// src/sales/cart.rs
// Shopping cart

use crate::products::inventory::Product;
use crate::products::pricing;

/// Item in the cart
#[derive(Debug, Clone)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}

impl CartItem {
    pub fn subtotal(&self) -> f64 {
        self.product.base_price() * self.quantity as f64
    }

    pub fn total_with_tax(&self) -> f64 {
        pricing::calculate_final_price(&self.product, self.quantity)
    }
}

/// Shopping cart
#[derive(Debug, Default)]
pub struct Cart {
    items: Vec<CartItem>,
}

impl Cart {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, product: Product, quantity: u32) {
        // Check if product already exists in cart
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += quantity;
        } else {
            self.items.push(CartItem { product, quantity });
        }
    }

    pub fn remove(&mut self, product_id: u32) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.product.id == product_id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn items(&self) -> &[CartItem] {
        &self.items
    }

    pub fn item_count(&self) -> usize {
        self.items.iter().map(|i| i.quantity as usize).sum()
    }

    pub fn subtotal(&self) -> f64 {
        self.items.iter().map(|i| i.subtotal()).sum()
    }

    pub fn total_tax(&self) -> f64 {
        pricing::calculate_tax(self.subtotal())
    }

    pub fn total(&self) -> f64 {
        self.items.iter().map(|i| i.total_with_tax()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Shopping Cart:\n");
        summary.push_str("--------------\n");
        
        for item in &self.items {
            summary.push_str(&format!(
                "  {} x {} = {:.2}€\n",
                item.quantity,
                item.product.name,
                item.subtotal()
            ));
        }
        
        summary.push_str(&format!("--------------\n"));
        summary.push_str(&format!("Subtotal: {:.2}€\n", self.subtotal()));
        summary.push_str(&format!("Tax ({}%): {:.2}€\n", (pricing::TAX * 100.0) as u32, self.total_tax()));
        summary.push_str(&format!("Total: {:.2}€", self.total()));
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_product(id: u32, name: &str, price: f64) -> Product {
        Product::new(id, name, price)
    }

    #[test]
    fn test_cart_new() {
        let cart = Cart::new();
        assert!(cart.is_empty());
        assert_eq!(cart.item_count(), 0);
    }

    #[test]
    fn test_cart_add() {
        let mut cart = Cart::new();
        let product = create_product(1, "Test", 100.0);
        cart.add(product, 2);
        
        assert!(!cart.is_empty());
        assert_eq!(cart.item_count(), 2);
    }

    #[test]
    fn test_cart_add_same_product() {
        let mut cart = Cart::new();
        let product = create_product(1, "Test", 100.0);
        cart.add(product.clone(), 2);
        cart.add(product, 3);
        
        assert_eq!(cart.items().len(), 1);
        assert_eq!(cart.item_count(), 5);
    }

    #[test]
    fn test_cart_remove() {
        let mut cart = Cart::new();
        let product = create_product(1, "Test", 100.0);
        cart.add(product, 2);
        
        assert!(cart.remove(1));
        assert!(cart.is_empty());
    }

    #[test]
    fn test_cart_subtotal() {
        let mut cart = Cart::new();
        let product = create_product(1, "Test", 100.0);
        cart.add(product, 2);
        
        assert_eq!(cart.subtotal(), 200.0);
    }
}
