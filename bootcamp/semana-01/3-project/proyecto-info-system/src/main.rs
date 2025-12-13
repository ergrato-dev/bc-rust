// ============================================
// 🦀 Bootcamp Rust: Zero to Hero
// Week 01 - Project: Information System
// ============================================
//
// OBJECTIVE:
// Create a program that displays personal
// and bootcamp information in an organized way.
//
// RUN:
// $ cargo run
//
// ============================================

fn main() {
    show_banner();
    show_personal_info();
    show_bootcamp_info();
    show_statistics();
    show_footer();
}

/// Shows the welcome banner
fn show_banner() {
    println!("╔══════════════════════════════════════════╗");
    println!("║    🦀 BOOTCAMP RUST: ZERO TO HERO 🦀    ║");
    println!("╠══════════════════════════════════════════╣");
}

/// Shows student's personal information
fn show_personal_info() {
    // TODO: Customize with your information
    let name = "Your Name";
    let role = "Developer";
    let location = "Your City";

    println!("║  Student: {:<30}║", name);
    println!("║  Role: {:<33}║", role);
    println!("║  Location: {:<29}║", location);
    println!("╠══════════════════════════════════════════╣");
}

/// Shows bootcamp progress
fn show_bootcamp_info() {
    let current_week = 1;
    let total_weeks = 17;
    let topic = "Introduction to Rust";

    // Calculate progress
    let progress = (current_week * 100) / total_weeks;

    // Create progress bar
    let bar = create_progress_bar(progress, 16);

    println!("║  📅 Week: {:02}/{}                         ║", current_week, total_weeks);
    println!("║  📚 Topic: {:<29}║", topic);
    println!("║  📊 Progress: {} {:>3}%       ║", bar, progress);
    println!("╠══════════════════════════════════════════╣");
}

/// Shows daily statistics
fn show_statistics() {
    let energy = 80;
    let planned_hours = 4;

    let energy_bar = create_progress_bar(energy, 10);

    println!("║  ⚡ Energy today: {} {:>3}%         ║", energy_bar, energy);
    println!("║  ⏰ Planned hours: {}                     ║", planned_hours);
}

/// Shows the footer
fn show_footer() {
    println!("╚══════════════════════════════════════════╝");
    println!();
    println!("  Let's code! 🚀");
}

/// Creates a visual progress bar
fn create_progress_bar(percentage: i32, length: i32) -> String {
    let filled = (percentage * length) / 100;
    let empty = length - filled;

    let mut bar = String::new();

    for _ in 0..filled {
        bar.push('▓');
    }

    for _ in 0..empty {
        bar.push('░');
    }

    bar
}
