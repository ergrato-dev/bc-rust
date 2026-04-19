// Practice 04: Advanced Patterns
// Week 05 - Enums and Pattern Matching

// ============================================
// Exercise 1: if let and while let
// ============================================

#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
    Empty,
}

// TODO: Print only if Text, using if let
fn print_if_text(msg: &Message) {
    todo!("Use if let")
}

// TODO: Process a stack until empty, using while let
fn process_stack(stack: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    todo!("Use while let with pop()")
    // Returns the sum of all elements
}

// TODO: Extract all texts from a vector of messages
fn extract_texts(messages: Vec<Message>) -> Vec<String> {
    let mut texts = Vec::new();
    todo!("Use for with if let or filter_map")
}

// ============================================
// Exercise 2: let else (Early Return)
// ============================================

#[derive(Debug)]
struct Config {
    port: Option<u16>,
    host: Option<String>,
    timeout: Option<u32>,
}

// TODO: Use let else to validate config
// Return error string if any field is None
fn validate_config(config: &Config) -> Result<String, &'static str> {
    todo!("Use let else to extract each field or return error")
    // If everything is ok, return Ok("Valid config: host:port, timeout=X")
}

// TODO: Get the first element greater than 10
fn first_greater_than_ten(numbers: &[i32]) -> Option<i32> {
    todo!("Use let else or ? operator")
}

// ============================================
// Exercise 3: Binding with @
// ============================================

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Scroll(f64),
}

// TODO: Classify the event using @ to capture values in ranges
fn describe_event(event: &Event) -> String {
    // Click in zone (0-100, 0-100): "Click in top left corner at (x, y)"
    // Click in other zone: "Click at (x, y)"
    // Key lowercase letter: "Key letter: X"
    // Key uppercase letter: "Key UPPERCASE: X"
    // Key other: "Special key: X"
    // Scroll positive: "Scroll up: X"
    // Scroll negative: "Scroll down: X"
    todo!("Use @ to capture values while checking patterns")
}

// TODO: Classify age using @ binding
fn classify_age(age: u8) -> &'static str {
    todo!("Use age @ 0..=2 => ... etc")
    // 0-2: "baby"
    // 3-12: "child"
    // 13-19: "teenager"
    // 20-64: "adult"
    // 65+: "senior"
}

// ============================================
// Exercise 4: matches! Macro
// ============================================

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Pending,
    Error(String),
}

// TODO: Use matches! to check if active
fn is_active(status: &Status) -> bool {
    todo!("Use matches!")
}

// TODO: Use matches! to check if error
fn is_error(status: &Status) -> bool {
    todo!("Use matches! with pattern")
}

// TODO: Count how many elements are active
fn count_active(statuses: &[Status]) -> usize {
    todo!("Use filter with matches!")
}

// TODO: Check if a number is in valid range
fn in_valid_range(n: i32) -> bool {
    todo!("Use matches! with range 1..=100")
}

fn main() {
    // if let demo
    let msg = Message::Text("Hello".to_string());
    print_if_text(&msg);

    // while let demo
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("Sum: {}", process_stack(&mut stack));

    // let else demo
    let config = Config {
        port: Some(8080),
        host: Some("localhost".to_string()),
        timeout: Some(30),
    };
    println!("{:?}", validate_config(&config));

    // @ binding demo
    let event = Event::Click { x: 50, y: 50 };
    println!("{}", describe_event(&event));

    // matches! demo
    let status = Status::Active;
    println!("Active? {}", is_active(&status));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_stack() {
        let mut stack = vec![1, 2, 3, 4, 5];
        assert_eq!(process_stack(&mut stack), 15);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_extract_texts() {
        let messages = vec![
            Message::Text("Hello".to_string()),
            Message::Number(42),
            Message::Text("World".to_string()),
            Message::Empty,
        ];
        let texts = extract_texts(messages);
        assert_eq!(texts, vec!["Hello", "World"]);
    }

    #[test]
    fn test_validate_config_ok() {
        let config = Config {
            port: Some(8080),
            host: Some("localhost".to_string()),
            timeout: Some(30),
        };
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_missing_port() {
        let config = Config {
            port: None,
            host: Some("localhost".to_string()),
            timeout: Some(30),
        };
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_first_greater_than_ten() {
        assert_eq!(first_greater_than_ten(&[5, 8, 15, 3]), Some(15));
        assert_eq!(first_greater_than_ten(&[1, 2, 3]), None);
    }

    #[test]
    fn test_classify_age() {
        assert_eq!(classify_age(1), "baby");
        assert_eq!(classify_age(8), "child");
        assert_eq!(classify_age(16), "teenager");
        assert_eq!(classify_age(30), "adult");
        assert_eq!(classify_age(70), "senior");
    }

    #[test]
    fn test_is_active() {
        assert!(is_active(&Status::Active));
        assert!(!is_active(&Status::Inactive));
    }

    #[test]
    fn test_is_error() {
        assert!(is_error(&Status::Error("failure".to_string())));
        assert!(!is_error(&Status::Active));
    }

    #[test]
    fn test_count_active() {
        let statuses = vec![
            Status::Active,
            Status::Inactive,
            Status::Active,
            Status::Pending,
        ];
        assert_eq!(count_active(&statuses), 2);
    }

    #[test]
    fn test_in_valid_range() {
        assert!(in_valid_range(50));
        assert!(!in_valid_range(0));
        assert!(!in_valid_range(101));
    }
}
