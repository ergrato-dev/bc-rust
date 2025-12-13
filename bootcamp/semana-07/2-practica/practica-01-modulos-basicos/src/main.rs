// Practice 01: Basic Modules
// Week 07 - Modules and Crates

// =============================================================================
// EXERCISE 1: Inline Modules - Calculator
// =============================================================================

mod operations {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub mod advanced {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub fn divide(a: i32, b: i32) -> Option<i32> {
            if b == 0 { None } else { Some(a / b) }
        }
    }
}

// =============================================================================
// EXERCISE 2: Navigation with super and self
// =============================================================================

mod library {
    pub const NAME: &str = "Rust Library";

    pub mod books {
        pub fn full_title(title: &str) -> String {
            // TODO: Use super:: to access NAME from parent module
            // Return format: "NAME: title"
            format!("{}: {}", super::NAME, title)
        }

        mod internal {
            pub fn process(title: &str) -> String {
                // TODO: Use super:: to call full_title
                format!("[Processed] {}", super::full_title(title))
            }
        }

        pub fn process_public(title: &str) -> String {
            // TODO: Use self:: to call the internal module
            self::internal::process(title)
        }
    }
}

// =============================================================================
// EXERCISE 3: Module Tree - Vehicles
// =============================================================================

mod vehicles {
    pub mod ground {
        pub mod car {
            pub struct Car {
                brand: String,
            }

            impl Car {
                pub fn new(brand: &str) -> Self {
                    Self {
                        brand: brand.to_string(),
                    }
                }

                pub fn describe(&self) -> String {
                    format!("Car brand: {}", self.brand)
                }
            }
        }

        pub mod motorcycle {
            pub struct Motorcycle {
                brand: String,
            }

            impl Motorcycle {
                pub fn new(brand: &str) -> Self {
                    Self {
                        brand: brand.to_string(),
                    }
                }

                pub fn describe(&self) -> String {
                    format!("Motorcycle brand: {}", self.brand)
                }
            }
        }
    }

    pub mod aerial {
        pub mod airplane {
            pub struct Airplane {
                brand: String,
            }

            impl Airplane {
                pub fn new(brand: &str) -> Self {
                    Self {
                        brand: brand.to_string(),
                    }
                }

                pub fn describe(&self) -> String {
                    format!("Airplane brand: {}", self.brand)
                }
            }
        }
    }
}

// =============================================================================
// MAIN FUNCTION
// =============================================================================

fn main() {
    println!("=== Practice 01: Basic Modules ===\n");

    // Exercise 1: Operations
    println!("--- Exercise 1: Calculator ---");
    let sum = crate::operations::add(10, 5);
    let diff = crate::operations::subtract(10, 5);
    let product = crate::operations::advanced::multiply(3, 4);
    let quotient = crate::operations::advanced::divide(20, 4);
    let division_zero = crate::operations::advanced::divide(10, 0);

    println!("10 + 5 = {}", sum);
    println!("10 - 5 = {}", diff);
    println!("3 * 4 = {}", product);
    println!("20 / 4 = {:?}", quotient);
    println!("10 / 0 = {:?}", division_zero);

    // Exercise 2: Library
    println!("\n--- Exercise 2: Library ---");
    let title = library::books::full_title("Don Quixote");
    let processed = library::books::process_public("Don Quixote");

    println!("Title: {}", title);
    println!("Processed: {}", processed);

    // Exercise 3: Vehicles
    println!("\n--- Exercise 3: Vehicles ---");
    use vehicles::ground::car::Car;
    use vehicles::ground::motorcycle::Motorcycle;
    use vehicles::aerial::airplane::Airplane;

    let car = Car::new("Toyota");
    let motorcycle = Motorcycle::new("Honda");
    let airplane = Airplane::new("Boeing");

    println!("{}", car.describe());
    println!("{}", motorcycle.describe());
    println!("{}", airplane.describe());
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(operations::add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(operations::subtract(10, 4), 6);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(operations::advanced::multiply(3, 4), 12);
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(operations::advanced::divide(10, 2), Some(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(operations::advanced::divide(10, 0), None);
    }

    #[test]
    fn test_full_title() {
        let title = library::books::full_title("Test Book");
        assert!(title.contains("Rust Library"));
        assert!(title.contains("Test Book"));
    }

    #[test]
    fn test_process_public() {
        let processed = library::books::process_public("Test");
        assert!(processed.contains("[Processed]"));
    }

    #[test]
    fn test_car_describe() {
        let car = vehicles::ground::car::Car::new("Ford");
        assert!(car.describe().contains("Ford"));
    }
}
