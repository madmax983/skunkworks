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
/// # Examples
///
/// ```no_run
/// use hyper_system::monitor::SystemMonitor;
///
/// let mut monitor = SystemMonitor::new();
/// // Call update inside your main loop
/// monitor.update_with_time(0.016, 100.0);
/// println!("CPU Usage: {:.2}%", monitor.cpu_usage * 100.0);
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
