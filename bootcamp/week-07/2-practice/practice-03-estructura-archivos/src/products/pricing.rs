// src/products/pricing.rs
// Price and tax calculations

use super::inventory::Product;

/// Tax rate (21%)
pub const TAX: f64 = 0.21;

/// Calculates final price including tax
pub fn calculate_final_price(product: &Product, quantity: u32) -> f64 {
    let subtotal = product.base_price() * quantity as f64;
    subtotal * (1.0 + TAX)
}

/// Applies a discount percentage to the price
pub fn apply_discount(price: f64, percentage: f64) -> f64 {
    let discount = price * (percentage / 100.0);
    price - discount
}

/// Calculates subtotal without taxes
pub fn calculate_subtotal(product: &Product, quantity: u32) -> f64 {
    product.base_price() * quantity as f64
}

/// Calculates tax amount only
pub fn calculate_tax(subtotal: f64) -> f64 {
    subtotal * TAX
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_product() -> Product {
        Product::new(1, "Test", 100.0)
    }

    #[test]
    fn test_final_price_one_unit() {
        let p = test_product();
        let price = calculate_final_price(&p, 1);
        assert!((price - 121.0).abs() < 0.01); // 100 * 1.21
    }

    #[test]
    fn test_final_price_multiple_units() {
        let p = test_product();
        let price = calculate_final_price(&p, 3);
        assert!((price - 363.0).abs() < 0.01); // 300 * 1.21
    }

    #[test]
    fn test_apply_discount_10() {
        let price = apply_discount(100.0, 10.0);
        assert!((price - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_apply_discount_25() {
        let price = apply_discount(100.0, 25.0);
        assert!((price - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_subtotal() {
        let p = test_product();
        let subtotal = calculate_subtotal(&p, 3);
        assert_eq!(subtotal, 300.0);
    }

    #[test]
    fn test_calculate_tax() {
        let tax = calculate_tax(100.0);
        assert!((tax - 21.0).abs() < 0.01);
    }
}
