// src/sales/invoice.rs
// Invoice generation

use super::cart::Cart;
use crate::products::pricing;

/// Represents a generated invoice
#[derive(Debug)]
pub struct Invoice {
    pub number: u64,
    pub customer: String,
    pub lines: Vec<InvoiceLine>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
}

/// An invoice line
#[derive(Debug)]
pub struct InvoiceLine {
    pub description: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub amount: f64,
}

impl Invoice {
    /// Creates an invoice from a cart
    pub fn from_cart(cart: &Cart, customer: &str) -> Self {
        static mut INVOICE_NUMBER: u64 = 1000;

        let number = unsafe {
            INVOICE_NUMBER += 1;
            INVOICE_NUMBER
        };

        let lines: Vec<InvoiceLine> = cart
            .items()
            .iter()
            .map(|item| InvoiceLine {
                description: item.product.name.clone(),
                quantity: item.quantity,
                unit_price: item.product.base_price(),
                amount: item.subtotal(),
            })
            .collect();

        let subtotal = cart.subtotal();
        let tax = pricing::calculate_tax(subtotal);
        let total = subtotal + tax;

        Self {
            number,
            customer: customer.to_string(),
            lines,
            subtotal,
            tax,
            total,
        }
    }

    /// Generates invoice text
    pub fn generate(&self) -> String {
        let mut invoice = String::new();

        invoice.push_str("╔══════════════════════════════════════════╗\n");
        invoice.push_str("║              I N V O I C E               ║\n");
        invoice.push_str("╠══════════════════════════════════════════╣\n");
        invoice.push_str(&format!("║ No: {:>36} ║\n", self.number));
        invoice.push_str(&format!("║ Customer: {:<30} ║\n", self.customer));
        invoice.push_str("╠══════════════════════════════════════════╣\n");
        invoice.push_str("║ Description          Qty.  Price  Amount ║\n");
        invoice.push_str("╠══════════════════════════════════════════╣\n");

        for line in &self.lines {
            let desc = if line.description.len() > 18 {
                format!("{}...", &line.description[..15])
            } else {
                format!("{:<18}", line.description)
            };

            invoice.push_str(&format!(
                "║ {} {:>4} {:>7.2} {:>7.2}║\n",
                desc, line.quantity, line.unit_price, line.amount
            ));
        }

        invoice.push_str("╠══════════════════════════════════════════╣\n");
        invoice.push_str(&format!("║ Subtotal:                   {:>12.2}€║\n", self.subtotal));
        invoice.push_str(&format!(
            "║ TAX ({:>2}%):                  {:>12.2}€║\n",
            (pricing::TAX * 100.0) as u32,
            self.tax
        ));
        invoice.push_str("╠══════════════════════════════════════════╣\n");
        invoice.push_str(&format!("║ TOTAL:                      {:>12.2}€║\n", self.total));
        invoice.push_str("╚══════════════════════════════════════════╝");

        invoice
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::products::Product;

    fn create_cart() -> Cart {
        let mut cart = Cart::new();
        let product = Product::new(1, "Test Product", 100.0);
        cart.add(product, 2);
        cart
    }

    #[test]
    fn test_invoice_from_cart() {
        let cart = create_cart();
        let invoice = Invoice::from_cart(&cart, "Test Customer");

        assert_eq!(invoice.customer, "Test Customer");
        assert_eq!(invoice.lines.len(), 1);
        assert_eq!(invoice.subtotal, 200.0);
    }

    #[test]
    fn test_invoice_number_increments() {
        let cart = create_cart();
        let invoice1 = Invoice::from_cart(&cart, "Customer 1");
        let invoice2 = Invoice::from_cart(&cart, "Customer 2");

        assert!(invoice2.number > invoice1.number);
    }

    #[test]
    fn test_invoice_generate() {
        let cart = create_cart();
        let invoice = Invoice::from_cart(&cart, "Test");
        let text = invoice.generate();

        assert!(text.contains("INVOICE"));
        assert!(text.contains("Test"));
    }
}
