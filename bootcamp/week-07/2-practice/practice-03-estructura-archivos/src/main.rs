// Practice 03: File Structure
// Week 07 - Modules and Crates
//
// This file is the entry point (crate root)
// Declares external modules

mod products;
mod sales;

// Re-import for easier use
use products::Product;
use sales::cart::Cart;
use sales::invoice::Invoice;

fn main() {
    println!("=== Practice 03: File Structure ===\n");

    // Create products
    let mut laptop = Product::new(1, "Gaming Laptop", 999.99);
    let mut mouse = Product::new(2, "Wireless Mouse", 29.99);
    let mut keyboard = Product::new(3, "Mechanical Keyboard", 79.99);

    // Add stock
    laptop.add_stock(10);
    mouse.add_stock(50);
    keyboard.add_stock(25);

    println!("--- Product Catalog ---");
    println!("{:?}", laptop);
    println!("{:?}", mouse);
    println!("{:?}", keyboard);

    // Create cart and add products
    println!("\n--- Shopping Cart ---");
    let mut cart = Cart::new();

    cart.add(laptop.clone(), 1);
    cart.add(mouse.clone(), 2);
    cart.add(keyboard.clone(), 1);

    println!("{}", cart.summary());

    // Calculate price with pricing module
    println!("\n--- Price Calculation ---");
    let laptop_price = products::calculate_final_price(&laptop, 1);
    println!(
        "Laptop (with {}% TAX): {:.2}€",
        products::pricing::TAX * 100.0,
        laptop_price
    );

    let discounted_price = products::pricing::apply_discount(laptop_price, 10.0);
    println!("Laptop (with 10% discount): {:.2}€", discounted_price);

    // Generate invoice
    println!("\n--- Invoice ---");
    let invoice = Invoice::from_cart(&cart, "Example Customer");
    println!("{}", invoice.generate());

    // Reduce stock after sale
    println!("\n--- Updating Stock ---");
    laptop.reduce_stock(1);
    mouse.reduce_stock(2);
    keyboard.reduce_stock(1);

    println!("Laptop stock: {}", laptop.quantity);
    println!("Mouse stock: {}", mouse.quantity);
    println!("Keyboard stock: {}", keyboard.quantity);
}
