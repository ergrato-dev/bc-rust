// Practice 01: Basic Enums
// Week 05 - Enums and Pattern Matching

// TODO: Define the Weekday enum with all 7 days
// Derive: Debug, Clone, Copy, PartialEq
#[derive(Debug, Clone, Copy, PartialEq)]
enum Weekday {
    // Complete the variants
}

// TODO: Implement this function
// Returns true if it's a business day (Monday to Friday)
fn is_business_day(day: Weekday) -> bool {
    todo!("Implement using match")
}

// TODO: Implement methods for Weekday
impl Weekday {
    // Returns the next day (Sunday -> Monday)
    fn next(&self) -> Weekday {
        todo!("Implement day cycle")
    }
    
    // Returns the day name as &str
    fn name(&self) -> &str {
        todo!("Implement with match")
    }
}

fn main() {
    let today = Weekday::Wednesday;
    
    println!("Today is: {:?}", today);
    println!("Is business day?: {}", is_business_day(today));
    println!("Tomorrow is: {:?}", today.next());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_business_day_monday() {
        assert!(is_business_day(Weekday::Monday));
    }

    #[test]
    fn test_is_business_day_friday() {
        assert!(is_business_day(Weekday::Friday));
    }

    #[test]
    fn test_not_business_day_saturday() {
        assert!(!is_business_day(Weekday::Saturday));
    }

    #[test]
    fn test_not_business_day_sunday() {
        assert!(!is_business_day(Weekday::Sunday));
    }

    #[test]
    fn test_next_monday() {
        assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
    }

    #[test]
    fn test_next_sunday() {
        assert_eq!(Weekday::Sunday.next(), Weekday::Monday);
    }

    #[test]
    fn test_name_wednesday() {
        assert_eq!(Weekday::Wednesday.name(), "Wednesday");
    }
}
