//! Inventory transaction module

use std::fmt;

/// Inventory transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Entry,
    Exit,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Entry => write!(f, "Entry"),
            TransactionType::Exit => write!(f, "Exit"),
        }
    }
}

/// Represents an inventory transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u32,
    pub product_id: u32,
    pub transaction_type: TransactionType,
    pub quantity: u32,
    pub date: String,
    pub note: Option<String>,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(
        id: u32,
        product_id: u32,
        transaction_type: TransactionType,
        quantity: u32,
        date: impl Into<String>,
    ) -> Self {
        Self {
            id,
            product_id,
            transaction_type,
            quantity,
            date: date.into(),
            note: None,
        }
    }

    /// Adds a note to the transaction
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Creates an entry transaction
    pub fn entry(id: u32, product_id: u32, quantity: u32, date: impl Into<String>) -> Self {
        Self::new(id, product_id, TransactionType::Entry, quantity, date)
    }

    /// Creates an exit transaction
    pub fn exit(id: u32, product_id: u32, quantity: u32, date: impl Into<String>) -> Self {
        Self::new(id, product_id, TransactionType::Exit, quantity, date)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.transaction_type {
            TransactionType::Entry => "+",
            TransactionType::Exit => "-",
        };
        write!(
            f,
            "[{}] {} Product #{}: {}{} units",
            self.date, self.transaction_type, self.product_id, symbol, self.quantity
        )?;
        if let Some(note) = &self.note {
            write!(f, " ({})", note)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let t = Transaction::new(1, 100, TransactionType::Entry, 50, "2025-01-15");
        assert_eq!(t.id, 1);
        assert_eq!(t.product_id, 100);
        assert_eq!(t.quantity, 50);
        assert!(t.note.is_none());
    }

    #[test]
    fn test_transaction_with_note() {
        let t = Transaction::entry(1, 100, 50, "2025-01-15")
            .with_note("Monthly restocking");
        assert_eq!(t.note, Some("Monthly restocking".to_string()));
    }

    #[test]
    fn test_entry_and_exit() {
        let entry = Transaction::entry(1, 100, 50, "2025-01-15");
        let exit = Transaction::exit(2, 100, 10, "2025-01-16");
        
        assert_eq!(entry.transaction_type, TransactionType::Entry);
        assert_eq!(exit.transaction_type, TransactionType::Exit);
    }
}
