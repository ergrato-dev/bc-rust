//! # Inventory Management System
//!
//! Demonstration of using collections in Rust.

use proyecto_inventario::{Inventory, ReportGenerator};

fn main() {
    println!("🦀 Inventory Management System\n");
    println!("{}", "═".repeat(50));

    // Create inventory
    let mut inventory = create_demo_inventory();

    // Show initial state
    let reports = ReportGenerator::new(&inventory);
    println!("{}", reports.summary_report());

    // Perform some operations
    println!("\n📦 INVENTORY OPERATIONS");
    println!("{}", "─".repeat(50));

    // Stock entry
    println!("\n➕ Stock entry: 20 laptops");
    inventory.stock_entry(1, 20, "Order #1234");

    // Stock exit
    println!("➖ Stock exit: 5 laptops (sale)");
    inventory.stock_exit(1, 5, "Customer ABC sale");

    println!("➖ Stock exit: 10 mice (sale)");
    inventory.stock_exit(2, 10, "Wholesale sale");

    // Searches
    println!("\n🔍 SEARCHES");
    println!("{}", "─".repeat(50));

    println!("\nSearch 'lap':");
    for p in inventory.search_by_name("lap") {
        println!("   {}", p);
    }

    println!("\nCategory 'Electronics':");
    for p in inventory.search_by_category("Electronics") {
        println!("   {}", p);
    }

    // Add new product
    println!("\n➕ Adding new product...");
    let new_id = inventory.add_product(
        "HD Webcam",
        "1080p camera with microphone",
        79.99,
        "Electronics",
        25,
    );
    println!("   Product added with ID: {}", new_id);

    // Full report
    let reports = ReportGenerator::new(&inventory);
    println!("{}", reports.full_report());

    println!("\n✅ Demo completed");
}

fn create_demo_inventory() -> Inventory {
    let mut inv = Inventory::new();

    // Electronics
    inv.add_product(
        "Laptop Pro",
        "15\" i7 16GB RAM Laptop",
        1299.99,
        "Electronics",
        10,
    );
    inv.add_product(
        "Gaming Mouse",
        "RGB 16000 DPI Mouse",
        49.99,
        "Electronics",
        50,
    );
    inv.add_product(
        "Mechanical Keyboard",
        "Cherry MX Blue Keyboard",
        129.99,
        "Electronics",
        3, // Low stock
    );
    inv.add_product(
        "27\" Monitor",
        "4K IPS Monitor",
        399.99,
        "Electronics",
        8,
    );

    // Furniture
    inv.add_product(
        "Ergonomic Chair",
        "Chair with lumbar support",
        299.99,
        "Furniture",
        5,
    );
    inv.add_product(
        "Adjustable Desk",
        "Standing desk",
        449.99,
        "Furniture",
        2, // Low stock
    );

    // Office
    inv.add_product(
        "A4 Notebook",
        "Pack of 5 notebooks",
        9.99,
        "Office",
        100,
    );
    inv.add_product(
        "Pens",
        "Pack of 20 pens",
        7.99,
        "Office",
        0, // Out of stock!
    );

    inv
}
