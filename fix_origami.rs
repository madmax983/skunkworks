use std::env;

fn main() {
    if env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        println!("Headless environment detected, skipping execution.");
        return;
    }
    // Launch macroquad
}
