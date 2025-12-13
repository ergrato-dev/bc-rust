// src/products.rs
// Main products module - declares submodules

pub mod inventory;
pub mod pricing;

// Re-export important items for simpler use
pub use inventory::Product;
pub use pricing::calculate_final_price;
