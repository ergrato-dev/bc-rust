// ============================================
// Weekly Project: User System
// ============================================
// Week 03: Structs and Methods
// ============================================

// ============================================
// LEVEL 1: BASIC STRUCTS
// ============================================

/// Unique user ID (newtype pattern)
#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(u64);

impl UserId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    fn value(&self) -> u64 {
        self.0
    }
}

/// Represents a system user
struct User {
    id: UserId,
    name: String,
    email: String,
    age: u32,
    active: bool,
}

impl User {
    // -----------------------------------------
    // Constructors
    // -----------------------------------------
    
    /// Main constructor
    fn new(id: u64, name: String, email: String) -> Self {
        Self {
            id: UserId::new(id),
            name,
            email,
            age: 0,
            active: true,
        }
    }

    /// Constructor with all data
    fn complete(id: u64, name: String, email: String, age: u32) -> Self {
        Self {
            id: UserId::new(id),
            name,
            email,
            age,
            active: true,
        }
    }

    // -----------------------------------------
    // Read Methods
    // -----------------------------------------
    
    fn id(&self) -> UserId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn display(&self) {
        println!(
            "[{}] {} ({}) - {} years old - {}",
            self.id.value(),
            self.name,
            self.email,
            self.age,
            if self.active { "✓ Active" } else { "✗ Inactive" }
        );
    }

    // -----------------------------------------
    // Mutation Methods
    // -----------------------------------------
    
    fn have_birthday(&mut self) {
        self.age += 1;
    }

    fn change_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

// ============================================
// LEVEL 1: USER PROFILE
// ============================================

/// Additional profile information
struct Profile {
    bio: String,
    website: Option<String>,
    location: String,
}

impl Profile {
    fn new(bio: String, location: String) -> Self {
        Self {
            bio,
            website: None,
            location,
        }
    }

    fn with_website(bio: String, location: String, site: String) -> Self {
        Self {
            bio,
            website: Some(site),
            location,
        }
    }

    fn display(&self) {
        println!("  Bio: {}", self.bio);
        println!("  Location: {}", self.location);
        match &self.website {
            Some(url) => println!("  Web: {}", url),
            None => println!("  Web: Not specified"),
        }
    }

    fn set_website(&mut self, url: String) {
        self.website = Some(url);
    }
}

// ============================================
// LEVEL 1: COMPLETE USER
// ============================================

/// User with complete profile
struct CompleteUser {
    user: User,
    profile: Profile,
}

impl CompleteUser {
    fn new(user: User, profile: Profile) -> Self {
        Self { user, profile }
    }

    fn display(&self) {
        println!("╔══════════════════════════════╗");
        println!("║       COMPLETE USER          ║");
        println!("╠══════════════════════════════╣");
        self.user.display();
        self.profile.display();
        println!("╚══════════════════════════════╝");
    }

    // Delegated access
    fn name(&self) -> &str {
        self.user.name()
    }

    fn location(&self) -> &str {
        &self.profile.location
    }

    fn is_adult(&self) -> bool {
        self.user.is_adult()
    }
}

// ============================================
// LEVEL 2: ROLE SYSTEM
// ============================================

/// Available roles in the system
struct Role {
    name: String,
    can_edit: bool,
    can_delete: bool,
    is_admin: bool,
}

impl Role {
    fn user() -> Self {
        Self {
            name: String::from("User"),
            can_edit: false,
            can_delete: false,
            is_admin: false,
        }
    }

    fn editor() -> Self {
        Self {
            name: String::from("Editor"),
            can_edit: true,
            can_delete: false,
            is_admin: false,
        }
    }

    fn admin() -> Self {
        Self {
            name: String::from("Administrator"),
            can_edit: true,
            can_delete: true,
            is_admin: true,
        }
    }

    fn display(&self) {
        println!(
            "Role: {} [edit:{}, delete:{}, admin:{}]",
            self.name,
            self.can_edit,
            self.can_delete,
            self.is_admin
        );
    }
}

// ============================================
// LEVEL 3: USER MANAGER
// ============================================

/// Manager that contains multiple users
struct UserManager {
    users: Vec<User>,
    next_id: u64,
}

impl UserManager {
    fn new() -> Self {
        Self {
            users: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, name: String, email: String, age: u32) -> UserId {
        let id = self.next_id;
        self.next_id += 1;
        
        let user = User::complete(id, name, email, age);
        let user_id = user.id();
        self.users.push(user);
        
        user_id
    }

    fn count(&self) -> usize {
        self.users.len()
    }

    fn list(&self) {
        println!("\n=== User List ({}) ===", self.count());
        for user in &self.users {
            user.display();
        }
    }

    fn find_by_id(&self, id: UserId) -> Option<&User> {
        self.users.iter().find(|u| u.id() == id)
    }

    fn active_users(&self) -> Vec<&User> {
        self.users.iter().filter(|u| u.is_active()).collect()
    }

    fn adult_users(&self) -> Vec<&User> {
        self.users.iter().filter(|u| u.is_adult()).collect()
    }
}

// ============================================
// MAIN
// ============================================

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   👥 USER SYSTEM - RUST               ║");
    println!("║      Week 03 Project                  ║");
    println!("╚════════════════════════════════════════╝\n");

    // -----------------------------------------
    // Level 1: Basic users
    // -----------------------------------------
    println!("═══ LEVEL 1: BASIC USERS ═══\n");
    
    let mut user1 = User::new(
        1,
        String::from("Ana Garcia"),
        String::from("ana@email.com"),
    );
    user1.have_birthday();
    user1.have_birthday();
    
    let user2 = User::complete(
        2,
        String::from("Carlos Lopez"),
        String::from("carlos@email.com"),
        25,
    );

    println!("--- Created users ---");
    user1.display();
    user2.display();

    println!("\nIs Ana an adult?: {}", user1.is_adult());
    println!("Is Carlos an adult?: {}", user2.is_adult());

    // -----------------------------------------
    // Level 1: Complete user with profile
    // -----------------------------------------
    println!("\n═══ COMPLETE USER ═══\n");
    
    let user = User::complete(
        3,
        String::from("Maria Rodriguez"),
        String::from("maria@email.com"),
        30,
    );
    
    let profile = Profile::with_website(
        String::from("Passionate Rust developer"),
        String::from("Madrid, Spain"),
        String::from("https://maria.dev"),
    );

    let complete_user = CompleteUser::new(user, profile);
    complete_user.display();

    // -----------------------------------------
    // Level 2: Roles
    // -----------------------------------------
    println!("\n═══ LEVEL 2: ROLES ═══\n");
    
    let user_role = Role::user();
    let editor_role = Role::editor();
    let admin_role = Role::admin();

    user_role.display();
    editor_role.display();
    admin_role.display();

    // -----------------------------------------
    // Level 3: User manager
    // -----------------------------------------
    println!("\n═══ LEVEL 3: MANAGER ═══");
    
    let mut manager = UserManager::new();
    
    let id1 = manager.add(
        String::from("Pedro"),
        String::from("pedro@email.com"),
        17,
    );
    
    manager.add(
        String::from("Laura"),
        String::from("laura@email.com"),
        22,
    );
    
    manager.add(
        String::from("Miguel"),
        String::from("miguel@email.com"),
        35,
    );

    manager.list();

    println!("\n--- Search by ID ---");
    if let Some(user) = manager.find_by_id(id1) {
        print!("Found: ");
        user.display();
    }

    println!("\n--- Adult users ({}) ---", manager.adult_users().len());
    for u in manager.adult_users() {
        println!("  - {}", u.name());
    }

    println!("\n✅ Project completed");
}

// ============================================
// TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let u = User::new(1, String::from("Test"), String::from("t@t.com"));
        
        assert_eq!(u.id().value(), 1);
        assert_eq!(u.name(), "Test");
        assert!(u.is_active());
    }

    #[test]
    fn test_user_is_adult() {
        let minor = User::complete(1, String::from("A"), String::from("a@a"), 17);
        let adult = User::complete(2, String::from("B"), String::from("b@b"), 18);
        
        assert!(!minor.is_adult());
        assert!(adult.is_adult());
    }

    #[test]
    fn test_user_have_birthday() {
        let mut u = User::new(1, String::from("T"), String::from("t@t"));
        assert_eq!(u.age(), 0);
        
        u.have_birthday();
        assert_eq!(u.age(), 1);
    }

    #[test]
    fn test_user_deactivate() {
        let mut u = User::new(1, String::from("T"), String::from("t@t"));
        assert!(u.is_active());
        
        u.deactivate();
        assert!(!u.is_active());
    }

    #[test]
    fn test_profile_without_website() {
        let p = Profile::new(String::from("Bio"), String::from("City"));
        assert!(p.website.is_none());
    }

    #[test]
    fn test_profile_with_website() {
        let p = Profile::with_website(
            String::from("Bio"),
            String::from("City"),
            String::from("https://example.com"),
        );
        assert!(p.website.is_some());
    }

    #[test]
    fn test_role_admin() {
        let admin = Role::admin();
        assert!(admin.is_admin);
        assert!(admin.can_edit);
        assert!(admin.can_delete);
    }

    #[test]
    fn test_manager_add() {
        let mut m = UserManager::new();
        m.add(String::from("A"), String::from("a@a"), 20);
        m.add(String::from("B"), String::from("b@b"), 25);
        
        assert_eq!(m.count(), 2);
    }

    #[test]
    fn test_manager_find() {
        let mut m = UserManager::new();
        let id = m.add(String::from("Test"), String::from("t@t"), 20);
        
        let found = m.find_by_id(id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), "Test");
    }

    #[test]
    fn test_manager_adults() {
        let mut m = UserManager::new();
        m.add(String::from("Minor"), String::from("m@m"), 15);
        m.add(String::from("Adult"), String::from("a@a"), 25);
        
        assert_eq!(m.adult_users().len(), 1);
    }
}
