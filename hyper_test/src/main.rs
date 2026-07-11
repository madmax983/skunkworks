use hyper_system::SystemMonitor;

fn main() {
    let mut monitor = SystemMonitor::new();

    // In your game loop:
    // monitor.update(); // If using macroquad
    // OR
    // monitor.update_with_time(dt, current_time);

    println!("CPU Stress: {:.2}%", monitor.cpu_usage * 100.0);
}
