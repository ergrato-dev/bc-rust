// Practice 03: Option and Methods
// Week 05 - Enums and Pattern Matching

// ============================================
// Exercise 1: Search with Option
// ============================================

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

// TODO: Find a user by ID
// Returns Some(&User) if exists, None if not
fn find_user(users: &[User], id: u32) -> Option<&User> {
    todo!("Implement search")
}

// TODO: Find an active user by ID
fn find_active_user(users: &[User], id: u32) -> Option<&User> {
    todo!("Use find_user and filter by active")
}

// ============================================
// Exercise 2: Option Methods
// ============================================

// TODO: Get the user name or "Anonymous"
fn get_name(users: &[User], id: u32) -> String {
    todo!("Use map and unwrap_or")
}

// TODO: Get the name in uppercase if exists
fn get_name_uppercase(users: &[User], id: u32) -> Option<String> {
    todo!("Use map to transform")
}

// TODO: Find a user and then find their friend
fn find_friend(users: &[User], id: u32, friends: &[(u32, u32)]) -> Option<&User> {
    // friends is a list of (user_id, friend_id)
    todo!("Use and_then to chain searches")
}

// ============================================
// Exercise 3: Option in Structs
// ============================================

#[derive(Debug)]
struct Profile {
    name: String,
    email: Option<String>,
    phone: Option<String>,
    age: Option<u8>,
}

impl Profile {
    fn new(name: &str) -> Self {
        Profile {
            name: name.to_string(),
            email: None,
            phone: None,
            age: None,
        }
    }

    // TODO: Setter that returns Self for chaining
    fn with_email(mut self, email: &str) -> Self {
        todo!("Set email and return self")
    }

    fn with_phone(mut self, phone: &str) -> Self {
        todo!("Set phone and return self")
    }

    fn with_age(mut self, age: u8) -> Self {
        todo!("Set age and return self")
    }

    // TODO: Get email or a default value
    fn email_or_default(&self) -> &str {
        todo!("Use as_ref and unwrap_or")
    }

    // TODO: Check if the profile is complete
    fn is_complete(&self) -> bool {
        todo!("Check that all Options are Some")
    }

    // TODO: Get contact information
    fn contact_info(&self) -> String {
        todo!("Combine available email and phone")
    }
}

// ============================================
// Exercise 4: Chaining with ?
// ============================================

// TODO: Get the first character of the email
fn first_email_char(profile: &Profile) -> Option<char> {
    todo!("Use ? to propagate None")
}

fn main() {
    let users = vec![
        User { id: 1, name: "Ana".to_string(), active: true },
        User { id: 2, name: "Bob".to_string(), active: false },
        User { id: 3, name: "Carlos".to_string(), active: true },
    ];

    // Basic search
    if let Some(u) = find_user(&users, 1) {
        println!("Found: {:?}", u);
    }

    // Name with fallback
    println!("User 1: {}", get_name(&users, 1));
    println!("User 99: {}", get_name(&users, 99));

    // Profile with builder pattern
    let profile = Profile::new("Diana")
        .with_email("diana@email.com")
        .with_age(28);

    println!("Email: {}", profile.email_or_default());
    println!("Complete: {}", profile.is_complete());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_users() -> Vec<User> {
        vec![
            User { id: 1, name: "Ana".to_string(), active: true },
            User { id: 2, name: "Bob".to_string(), active: false },
            User { id: 3, name: "Carlos".to_string(), active: true },
        ]
    }

    #[test]
    fn test_find_user_existing() {
        let users = create_users();
        let result = find_user(&users, 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Ana");
    }

    #[test]
    fn test_find_user_not_existing() {
        let users = create_users();
        assert!(find_user(&users, 99).is_none());
    }

    #[test]
    fn test_find_active_user() {
        let users = create_users();
        assert!(find_active_user(&users, 1).is_some());
        assert!(find_active_user(&users, 2).is_none()); // Bob is not active
    }

    #[test]
    fn test_get_name_existing() {
        let users = create_users();
        assert_eq!(get_name(&users, 1), "Ana");
    }

    #[test]
    fn test_get_name_not_existing() {
        let users = create_users();
        assert_eq!(get_name(&users, 99), "Anonymous");
    }

    #[test]
    fn test_get_name_uppercase() {
        let users = create_users();
        assert_eq!(get_name_uppercase(&users, 1), Some("ANA".to_string()));
        assert!(get_name_uppercase(&users, 99).is_none());
    }

    #[test]
    fn test_profile_builder() {
        let profile = Profile::new("Test")
            .with_email("test@test.com")
            .with_phone("123456");
        
        assert_eq!(profile.email, Some("test@test.com".to_string()));
        assert_eq!(profile.phone, Some("123456".to_string()));
    }

    #[test]
    fn test_profile_email_default() {
        let profile = Profile::new("Test");
        assert_eq!(profile.email_or_default(), "no email");
    }

    #[test]
    fn test_profile_complete() {
        let incomplete = Profile::new("Test").with_email("a@b.com");
        let complete = Profile::new("Test")
            .with_email("a@b.com")
            .with_phone("123")
            .with_age(25);
        
        assert!(!incomplete.is_complete());
        assert!(complete.is_complete());
    }

    #[test]
    fn test_first_email_char() {
        let with_email = Profile::new("Test").with_email("abc@test.com");
        let without_email = Profile::new("Test");
        
        assert_eq!(first_email_char(&with_email), Some('a'));
        assert!(first_email_char(&without_email).is_none());
    }
}
