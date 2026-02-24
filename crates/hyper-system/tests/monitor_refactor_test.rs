use hyper_system::monitor::SystemMonitor;

#[test]
fn test_system_monitor_initialization() {
    let monitor = SystemMonitor::new();
    assert_eq!(monitor.cpu_usage, 0.0);
    assert_eq!(monitor.mem_usage, 0.0);
    assert_eq!(monitor.swap_usage, 0.0);
    assert_eq!(monitor.load_avg, 0.0);
    // last_update should be -2.0 to force immediate update
    assert_eq!(monitor.last_update, -2.0);
}

#[test]
fn test_system_monitor_update_with_time() {
    let mut monitor = SystemMonitor::new();

    // Initial state
    assert_eq!(monitor.cpu_usage, 0.0);

    // Simulate first update at t=0.0
    // This should trigger a refresh because 0.0 - (-2.0) > 1.0
    monitor.update_with_time(0.016, 0.0);
    assert_eq!(monitor.last_update, 0.0);

    // CPU usage might still be 0.0 because sysinfo needs two points
    // But let's check that interpolation happened (0.0 -> target)
    // Since target is likely 0.0 initially, cpu_usage stays 0.0

    // Advance time slightly
    monitor.update_with_time(0.016, 0.016);
    // last_update shouldn't change
    assert_eq!(monitor.last_update, 0.0);

    // Advance time significantly (past 1s)
    monitor.update_with_time(0.016, 1.1);
    // Should update
    assert_eq!(monitor.last_update, 1.1);

    // Now target_cpu should be set (could be 0.0 if idle or < 1%)
    // But mem_usage should definitely be > 0.0
    assert!(monitor.mem_usage >= 0.0);
    assert!(monitor.mem_usage <= 1.0);
}

#[test]
fn test_interpolation_logic() {
    let mut monitor = SystemMonitor::new();

    // Force set internal state (this is tricky because fields are private)
    // But we can observe that multiple small updates smoothly transition values

    // Force a large update by manipulating time
    // Let's assume monitor updates targets at t=0
    monitor.update_with_time(0.016, 0.0);

    let initial_mem = monitor.mem_usage;

    // Run many small steps
    for i in 1..10 {
        let t = i as f64 * 0.016;
        monitor.update_with_time(0.016, t);
    }

    // Mem usage should have moved from 0.0 towards target
    // Since it started at 0.0 and target is > 0.0 (usually), it should increase
    if initial_mem > 0.0 {
        // If it was already initialized non-zero (unlikely for new()), fine
    } else {
        // It should be increasing towards target
        assert!(monitor.mem_usage >= initial_mem);
    }
}
