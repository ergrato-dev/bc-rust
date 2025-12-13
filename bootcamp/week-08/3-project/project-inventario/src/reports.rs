//! Report generation module

use crate::inventory::Inventory;

/// Report generator for the inventory
pub struct ReportGenerator<'a> {
    inventory: &'a Inventory,
}

impl<'a> ReportGenerator<'a> {
    /// Creates a new report generator
    pub fn new(inventory: &'a Inventory) -> Self {
        Self { inventory }
    }

    /// Generates a summary report of the inventory
    pub fn summary_report(&self) -> String {
        let mut report = String::new();

        report.push_str("╔══════════════════════════════════════════════════════╗\n");
        report.push_str("║         📊 INVENTORY REPORT                          ║\n");
        report.push_str("╠══════════════════════════════════════════════════════╣\n");

        // General statistics
        report.push_str(&format!(
            "║ Total products:      {:>10}                     ║\n",
            self.inventory.total_products()
        ));
        report.push_str(&format!(
            "║ Total units:         {:>10}                     ║\n",
            self.inventory.total_units()
        ));
        report.push_str(&format!(
            "║ Total value:         ${:>9.2}                     ║\n",
            self.inventory.total_value()
        ));
        report.push_str(&format!(
            "║ Average price:       ${:>9.2}                     ║\n",
            self.inventory.average_price()
        ));

        report.push_str("╚══════════════════════════════════════════════════════╝\n");

        report
    }

    /// Generates a report of products by category
    pub fn category_report(&self) -> String {
        let mut report = String::new();

        report.push_str("\n📁 PRODUCTS BY CATEGORY\n");
        report.push_str("─".repeat(50).as_str());
        report.push('\n');

        for category in self.inventory.categories() {
            let products = self.inventory.search_by_category(&category);
            let value: f64 = products.iter().map(|p| p.inventory_value()).sum();

            report.push_str(&format!("\n🏷️  {} ({} products, value: ${:.2})\n",
                category, products.len(), value));

            for product in products {
                report.push_str(&format!(
                    "   • {} - ${:.2} (Stock: {})\n",
                    product.name, product.price, product.stock
                ));
            }
        }

        report
    }

    /// Generates a stock alerts report
    pub fn alerts_report(&self, low_threshold: u32) -> String {
        let mut report = String::new();

        report.push_str("\n⚠️  STOCK ALERTS\n");
        report.push_str("─".repeat(50).as_str());
        report.push('\n');

        // Out of stock
        let out_of_stock = self.inventory.out_of_stock_products();
        if !out_of_stock.is_empty() {
            report.push_str("\n🔴 OUT OF STOCK:\n");
            for product in &out_of_stock {
                report.push_str(&format!("   • {} [{}]\n", product.name, product.category));
            }
        }

        // Low stock
        let low_stock: Vec<_> = self.inventory.low_stock_products(low_threshold)
            .into_iter()
            .filter(|p| p.stock > 0)
            .collect();

        if !low_stock.is_empty() {
            report.push_str(&format!("\n🟡 LOW STOCK (< {} units):\n", low_threshold));
            for product in low_stock {
                report.push_str(&format!(
                    "   • {} - {} units [{}]\n",
                    product.name, product.stock, product.category
                ));
            }
        }

        if out_of_stock.is_empty() && self.inventory.low_stock_products(low_threshold).is_empty() {
            report.push_str("\n✅ No stock alerts\n");
        }

        report
    }

    /// Generates a top products by value report
    pub fn top_value_report(&self, n: usize) -> String {
        let mut report = String::new();

        report.push_str(&format!("\n🏆 TOP {} PRODUCTS BY VALUE\n", n));
        report.push_str("─".repeat(50).as_str());
        report.push('\n');

        let top = self.inventory.top_products_by_value(n);

        for (i, product) in top.iter().enumerate() {
            report.push_str(&format!(
                "   {}. {} - ${:.2} ({}×${:.2})\n",
                i + 1,
                product.name,
                product.inventory_value(),
                product.stock,
                product.price
            ));
        }

        report
    }

    /// Generates a transaction history report
    pub fn transactions_report(&self) -> String {
        let mut report = String::new();

        report.push_str("\n📋 TRANSACTION HISTORY\n");
        report.push_str("─".repeat(50).as_str());
        report.push('\n');

        let transactions = self.inventory.transaction_history();

        if transactions.is_empty() {
            report.push_str("\nNo transactions recorded.\n");
        } else {
            for transaction in transactions.iter().rev().take(10) {
                let product_name = self.inventory
                    .get_product(transaction.product_id)
                    .map(|p| p.name.as_str())
                    .unwrap_or("Deleted product");

                report.push_str(&format!(
                    "\n   {} | {} {} units of {}\n",
                    transaction.date,
                    transaction.transaction_type,
                    transaction.quantity,
                    product_name
                ));

                if let Some(note) = &transaction.note {
                    report.push_str(&format!("      📝 {}\n", note));
                }
            }

            if transactions.len() > 10 {
                report.push_str(&format!(
                    "\n   ... and {} more transactions\n",
                    transactions.len() - 10
                ));
            }
        }

        report
    }

    /// Generates a full report
    pub fn full_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&self.summary_report());
        report.push_str(&self.category_report());
        report.push_str(&self.alerts_report(10));
        report.push_str(&self.top_value_report(5));
        report.push_str(&self.transactions_report());

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_inventory() -> Inventory {
        let mut inv = Inventory::new();
        inv.add_product("Laptop", "Portable", 999.99, "Electronics", 10);
        inv.add_product("Mouse", "Wireless", 29.99, "Electronics", 50);
        inv.add_product("Chair", "Ergonomic", 199.99, "Furniture", 3);
        inv
    }

    #[test]
    fn test_summary_report() {
        let inv = create_test_inventory();
        let generator = ReportGenerator::new(&inv);
        let report = generator.summary_report();
        
        assert!(report.contains("INVENTORY REPORT"));
        assert!(report.contains("Total products"));
    }

    #[test]
    fn test_category_report() {
        let inv = create_test_inventory();
        let generator = ReportGenerator::new(&inv);
        let report = generator.category_report();
        
        assert!(report.contains("Electronics"));
        assert!(report.contains("Furniture"));
    }

    #[test]
    fn test_alerts_report() {
        let inv = create_test_inventory();
        let generator = ReportGenerator::new(&inv);
        let report = generator.alerts_report(5);
        
        assert!(report.contains("STOCK ALERTS"));
        assert!(report.contains("Chair")); // Stock = 3 < 5
    }
}
