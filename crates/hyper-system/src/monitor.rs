//! System monitoring utilities for real-time visualization.
//!
//! This module provides the [`SystemMonitor`] struct, which gathers system metrics
//! (CPU, RAM, Swap) and interpolates them over time. This is particularly useful
//! for "Hyper" series experiments where simulation parameters are driven by
//! system load.

#[cfg(feature = "macroquad")]
use macroquad::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

/// A real-time system resource monitor.
///
/// `SystemMonitor` tracks CPU, memory, swap usage, and load average.
/// It uses linear interpolation (lerp) to smooth out sudden spikes, providing
/// values suitable for driving fluid visual animations.
///
/// # Metrics
/// All public metric fields (`cpu_usage`, `mem_usage`, etc.) are normalized
/// to a range of **0.0 to 1.0**, where 1.0 represents 100% usage.
///
/// # Behavior
///
/// - **Polling Frequency**: The underlying system metrics are refreshed at most
///   once every **1.0 second**. This prevents excessive overhead from system calls.
/// - **Smoothing**: Between polls, the public values are interpolated towards the
///   latest system values using an exponential moving average. The smoothing speed
///   is proportional to `2.0 * dt`.
/// - **Load Average**: The load average is normalized against a baseline of **4 cores**.
///   A load average of 4.0 results in a value of 1.0. This is a heuristic for
///   visualization purposes and may saturate on machines with more cores.
///
/// # Examples
///
/// ```no_run
/// use hyper_system::monitor::SystemMonitor;
/// use std::thread;
/// use std::time::Duration;
///
/// let mut monitor = SystemMonitor::new();
///
/// // Simulate a loop
/// let dt: f32 = 0.016; // 60 FPS
/// let mut time: f64 = 0.0;
///
/// for _ in 0..100 {
///     time += dt as f64;
///     // Update the monitor with the current frame time
///     monitor.update_with_time(dt, time);
///
///     // Use the smoothed values for visualization
///     println!("Smoothed CPU: {:.2}%", monitor.cpu_usage * 100.0);
///
///     // In a real app, you would sleep or wait for vsync here
///     // thread::sleep(Duration::from_secs_f32(dt));
/// }
/// ```
pub struct SystemMonitor {
    pub sys: System,
    /// Timestamp of the last successful system poll (in seconds).
    pub last_update: f64,
    /// Current interpolated CPU usage (0.0 - 1.0).
    pub cpu_usage: f32,
    /// Current interpolated memory usage (0.0 - 1.0).
    pub mem_usage: f32,
    /// Current interpolated swap usage (0.0 - 1.0).
    pub swap_usage: f32,
    /// Current interpolated load average (0.0 - 1.0), normalized against a load of 4.0.
    pub load_avg: f32,

    // Target metrics for interpolation
    target_cpu: f32,
    target_mem: f32,
    target_swap: f32,
    target_load: f32,
}

impl SystemMonitor {
    /// Creates a new `SystemMonitor` instance.
    ///
    /// Initializes the underlying system information gatherer.
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            last_update: 0.0,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            swap_usage: 0.0,
            load_avg: 0.0,
            target_cpu: 0.0,
            target_mem: 0.0,
            target_swap: 0.0,
            target_load: 0.0,
        }
    }

    /// Updates the system metrics using explicitly provided time values.
    ///
    /// Use this method if you are not using macroquad or want manual control over timing.
    ///
    /// # Arguments
    ///
    /// * `dt` - The time elapsed since the last frame (in seconds).
    /// * `now` - The current timestamp (in seconds).
    ///
    /// # Implementation Details
    ///
    /// This method performs two tasks:
    /// 1. **Poll**: Checks if 1 second has passed since `last_update`. If so, it refreshes
    ///    system stats via `sysinfo` and updates the internal "target" values.
    /// 2. **Interpolate**: Smoothly moves the public fields (`cpu_usage`, etc.) towards
    ///    the target values using the formula: `current = lerp(current, target, 2.0 * dt)`.
    pub fn update_with_time(&mut self, dt: f32, now: f64) {
        if now - self.last_update > 1.0 {
            self.sys.refresh_cpu();
            self.sys.refresh_memory();
            self.last_update = now;

            self.target_cpu = self.sys.global_cpu_info().cpu_usage() / 100.0;

            let total_mem = self.sys.total_memory() as f32;
            let used_mem = self.sys.used_memory() as f32;
            self.target_mem = if total_mem > 0.0 {
                used_mem / total_mem
            } else {
                0.0
            };

            let total_swap = self.sys.total_swap() as f32;
            let used_swap = self.sys.used_swap() as f32;
            self.target_swap = if total_swap > 0.0 {
                used_swap / total_swap
            } else {
                0.0
            };

            let load = System::load_average();
            // Normalize load average assuming 4 cores is "full load" for visual purposes
            self.target_load = (load.one as f32 / 4.0).clamp(0.0, 1.0);
        }

        // Interpolate
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let speed = 2.0 * dt;

        self.cpu_usage = lerp(self.cpu_usage, self.target_cpu, speed);
        self.mem_usage = lerp(self.mem_usage, self.target_mem, speed);
        self.swap_usage = lerp(self.swap_usage, self.target_swap, speed);
        self.load_avg = lerp(self.load_avg, self.target_load, speed);
    }

    /// Updates the system metrics using macroquad's time functions.
    ///
    /// This method is only available when the `macroquad` feature is enabled.
    /// It automatically calls [`get_frame_time()`] and [`get_time()`].
    #[cfg(feature = "macroquad")]
    pub fn update(&mut self) {
        self.update_with_time(get_frame_time(), get_time());
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}
