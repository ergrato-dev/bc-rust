// Project: State Machine - Order System
// Week 05 - Enums and Pattern Matching

// ============================================
// Order States
// ============================================

#[derive(Debug, Clone)]
enum OrderStatus {
    Created { date: String },
    Paid { date: String, amount: f64 },
    Shipped { date: String, tracking: String },
    Delivered { date: String },
    Cancelled { date: String, reason: String },
    Returned { date: String, reason: String },
}

// ============================================
// Order Structure
// ============================================

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    customer: String,
    items: Vec<String>,
    status: OrderStatus,
}

// Helper function to get current date (simulated)
fn get_date() -> String {
    "2025-01-15".to_string()
}

impl Order {
    // TODO: Create a new order in Created status
    fn new(id: u32, customer: &str, items: Vec<String>) -> Self {
        todo!("Create order with Created status")
    }

    // TODO: Transition to Paid (only from Created)
    fn pay(&mut self, amount: f64) -> Result<(), &'static str> {
        todo!("Implement transition to Paid")
    }

    // TODO: Transition to Shipped (only from Paid)
    fn ship(&mut self, tracking: &str) -> Result<(), &'static str> {
        todo!("Implement transition to Shipped")
    }

    // TODO: Transition to Delivered (only from Shipped)
    fn deliver(&mut self) -> Result<(), &'static str> {
        todo!("Implement transition to Delivered")
    }

    // TODO: Transition to Cancelled (from Created or Paid)
    fn cancel(&mut self, reason: &str) -> Result<(), &'static str> {
        todo!("Implement transition to Cancelled")
    }

    // TODO: Transition to Returned (only from Delivered)
    fn return_order(&mut self, reason: &str) -> Result<(), &'static str> {
        todo!("Implement transition to Returned")
    }

    // TODO: Check if the order can be cancelled
    fn can_cancel(&self) -> bool {
        todo!("Use matches! or match")
    }

    // TODO: Return readable status description
    fn status_description(&self) -> String {
        todo!("Use match for each variant")
    }

    // TODO: Get the amount if paid or later
    fn get_amount(&self) -> Option<f64> {
        todo!("Return Some(amount) if applicable, None if not")
    }

    // TODO: Get tracking if shipped or delivered
    fn get_tracking(&self) -> Option<&str> {
        todo!("Return Some(&tracking) if applicable")
    }

    // TODO: Check if the order is in a final state
    fn is_final_state(&self) -> bool {
        todo!("Delivered, Cancelled or Returned are final")
    }
}

// ============================================
// Order Manager
// ============================================

#[derive(Debug, Default)]
struct OrderManager {
    orders: Vec<Order>,
    next_id: u32,
}

impl OrderManager {
    fn new() -> Self {
        OrderManager {
            orders: Vec::new(),
            next_id: 1,
        }
    }

    // TODO: Create and add a new order
    fn create_order(&mut self, customer: &str, items: Vec<String>) -> u32 {
        todo!("Create order, add to list, return ID")
    }

    // TODO: Find order by ID
    fn find(&self, id: u32) -> Option<&Order> {
        todo!("Use iter().find()")
    }

    // TODO: Find mutable order by ID
    fn find_mut(&mut self, id: u32) -> Option<&mut Order> {
        todo!("Use iter_mut().find()")
    }

    // TODO: List orders in a specific status
    fn list_by_status(&self, status_name: &str) -> Vec<&Order> {
        todo!("Filter using matches! or match")
        // status_name can be: "created", "paid", "shipped", etc.
    }

    // TODO: Count orders by status
    fn statistics(&self) -> Statistics {
        todo!("Count each status type")
    }

    // TODO: Get total sales (sum of paid amounts)
    fn total_sales(&self) -> f64 {
        todo!("Sum amounts of paid/shipped/delivered orders")
    }
}

#[derive(Debug, Default, PartialEq)]
struct Statistics {
    created: u32,
    paid: u32,
    shipped: u32,
    delivered: u32,
    cancelled: u32,
    returned: u32,
}

// ============================================
// Main Function
// ============================================

fn main() {
    let mut manager = OrderManager::new();

    // Create some orders
    let id1 = manager.create_order("Ana Garcia", vec![
        "Laptop".to_string(),
        "Mouse".to_string(),
    ]);

    let id2 = manager.create_order("Bob Lopez", vec![
        "Keyboard".to_string(),
    ]);

    println!("Orders created: {} and {}", id1, id2);

    // Process order 1
    if let Some(order) = manager.find_mut(id1) {
        println!("\n--- Processing order {} ---", id1);
        
        if let Err(e) = order.pay(1500.0) {
            println!("Error paying: {}", e);
        } else {
            println!("Order paid: {}", order.status_description());
        }

        if let Err(e) = order.ship("TRACK123456") {
            println!("Error shipping: {}", e);
        } else {
            println!("Order shipped: {}", order.status_description());
        }
    }

    // Cancel order 2
    if let Some(order) = manager.find_mut(id2) {
        println!("\n--- Cancelling order {} ---", id2);
        
        if order.can_cancel() {
            let _ = order.cancel("Customer changed their mind");
            println!("Order cancelled: {}", order.status_description());
        }
    }

    // Show statistics
    println!("\n--- Statistics ---");
    let stats = manager.statistics();
    println!("{:?}", stats);
    println!("Total sales: ${:.2}", manager.total_sales());
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Creation Tests ---

    #[test]
    fn test_create_order() {
        let order = Order::new(1, "Test", vec!["Item".to_string()]);
        assert_eq!(order.id, 1);
        assert_eq!(order.customer, "Test");
        assert!(matches!(order.status, OrderStatus::Created { .. }));
    }

    // --- Valid Transition Tests ---

    #[test]
    fn test_transition_created_to_paid() {
        let mut order = Order::new(1, "Test", vec![]);
        assert!(order.pay(100.0).is_ok());
        assert!(matches!(order.status, OrderStatus::Paid { amount, .. } if amount == 100.0));
    }

    #[test]
    fn test_transition_paid_to_shipped() {
        let mut order = Order::new(1, "Test", vec![]);
        order.pay(100.0).unwrap();
        assert!(order.ship("TRACK001").is_ok());
        assert!(matches!(order.status, OrderStatus::Shipped { .. }));
    }

    #[test]
    fn test_transition_shipped_to_delivered() {
        let mut order = Order::new(1, "Test", vec![]);
        order.pay(100.0).unwrap();
        order.ship("TRACK001").unwrap();
        assert!(order.deliver().is_ok());
        assert!(matches!(order.status, OrderStatus::Delivered { .. }));
    }

    #[test]
    fn test_transition_created_to_cancelled() {
        let mut order = Order::new(1, "Test", vec![]);
        assert!(order.cancel("Don't want it").is_ok());
        assert!(matches!(order.status, OrderStatus::Cancelled { .. }));
    }

    #[test]
    fn test_transition_delivered_to_returned() {
        let mut order = Order::new(1, "Test", vec![]);
        order.pay(100.0).unwrap();
        order.ship("TRACK001").unwrap();
        order.deliver().unwrap();
        assert!(order.return_order("Defective").is_ok());
        assert!(matches!(order.status, OrderStatus::Returned { .. }));
    }

    // --- Invalid Transition Tests ---

    #[test]
    fn test_cannot_pay_if_already_paid() {
        let mut order = Order::new(1, "Test", vec![]);
        order.pay(100.0).unwrap();
        assert!(order.pay(200.0).is_err());
    }

    #[test]
    fn test_cannot_ship_if_not_paid() {
        let mut order = Order::new(1, "Test", vec![]);
        assert!(order.ship("TRACK001").is_err());
    }

    #[test]
    fn test_cannot_cancel_if_shipped() {
        let mut order = Order::new(1, "Test", vec![]);
        order.pay(100.0).unwrap();
        order.ship("TRACK001").unwrap();
        assert!(!order.can_cancel());
        assert!(order.cancel("Reason").is_err());
    }

    // --- Query Tests ---

    #[test]
    fn test_get_amount() {
        let mut order = Order::new(1, "Test", vec![]);
        assert!(order.get_amount().is_none());
        
        order.pay(150.0).unwrap();
        assert_eq!(order.get_amount(), Some(150.0));
    }

    #[test]
    fn test_get_tracking() {
        let mut order = Order::new(1, "Test", vec![]);
        order.pay(100.0).unwrap();
        assert!(order.get_tracking().is_none());
        
        order.ship("ABC123").unwrap();
        assert_eq!(order.get_tracking(), Some("ABC123"));
    }

    #[test]
    fn test_is_final_state() {
        let mut order = Order::new(1, "Test", vec![]);
        assert!(!order.is_final_state());
        
        order.cancel("Test").unwrap();
        assert!(order.is_final_state());
    }

    // --- Manager Tests ---

    #[test]
    fn test_manager_create_order() {
        let mut manager = OrderManager::new();
        let id1 = manager.create_order("Ana", vec!["Item".to_string()]);
        let id2 = manager.create_order("Bob", vec!["Other".to_string()]);
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.orders.len(), 2);
    }

    #[test]
    fn test_manager_find() {
        let mut manager = OrderManager::new();
        let id = manager.create_order("Ana", vec![]);
        
        assert!(manager.find(id).is_some());
        assert!(manager.find(999).is_none());
    }

    #[test]
    fn test_manager_statistics() {
        let mut manager = OrderManager::new();
        manager.create_order("A", vec![]);
        manager.create_order("B", vec![]);
        
        if let Some(p) = manager.find_mut(1) {
            p.pay(100.0).unwrap();
        }
        
        let stats = manager.statistics();
        assert_eq!(stats.created, 1);
        assert_eq!(stats.paid, 1);
    }

    #[test]
    fn test_manager_total_sales() {
        let mut manager = OrderManager::new();
        manager.create_order("A", vec![]);
        manager.create_order("B", vec![]);
        
        if let Some(p) = manager.find_mut(1) {
            p.pay(100.0).unwrap();
        }
        if let Some(p) = manager.find_mut(2) {
            p.pay(250.0).unwrap();
        }
        
        assert_eq!(manager.total_sales(), 350.0);
    }
}
