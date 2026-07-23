use hyper_system::SystemMonitor;

#[test]
fn test_system_monitor_negative_dt() {
    let mut monitor = SystemMonitor::new();
    monitor.update_with_time(0.016, 0.0);
    monitor.cpu_usage = 0.5;
    // Negative dt should be ignored
    monitor.update_with_time(-0.1, 0.016);
    assert_eq!(monitor.cpu_usage, 0.5);
}

#[test]
fn test_system_monitor_nan_now() {
    let mut monitor = SystemMonitor::new();
    monitor.last_update = 0.0;
    // NaN now should be ignored
    monitor.update_with_time(0.1, f64::NAN);
    assert_eq!(monitor.last_update, 0.0);
}
