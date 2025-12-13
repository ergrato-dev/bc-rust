//! # Project: Inventory Management System
//!
//! This project demonstrates the use of collections in Rust:
//! - `Vec<T>` for lists of products and transactions
//! - `String` for names, descriptions and categories
//! - `HashMap<K, V>` for indexes and fast lookups
//! - Iterators for data processing

mod product;
mod transaction;
mod inventory;
mod reports;

pub use product::Product;
pub use transaction::{Transaction, TransactionType};
pub use inventory::Inventory;
pub use reports::ReportGenerator;
