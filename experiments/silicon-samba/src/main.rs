mod audio;
mod euclidean;
mod monitor;
mod tui;

use anyhow::Result;
use crossbeam::channel::unbounded;
use std::time::{Duration, Instant};

use crate::audio::{AudioEngine, AudioEvent};
use crate::euclidean::EuclideanGenerator;
use crate::monitor::SystemMonitor;
use crate::tui::AppTui;

fn main() -> Result<()> {
    // 1. Setup Audio
    let (audio_tx, audio_rx) = unbounded();
    let _audio_engine = AudioEngine::new(audio_rx)?;

    // 2. Setup Monitor
    let mut monitor = SystemMonitor::new();

    // 3. Setup Generators
    // 0: Kick, 1: Snare, 2: HiHat, 3: Perc
    let mut generators = vec![
        EuclideanGenerator::new(16, 4), // Kick (Default 4 on floor)
        EuclideanGenerator::new(16, 2), // Snare (Backbeat)
        EuclideanGenerator::new(16, 8), // HiHat (Eighths)
        EuclideanGenerator::new(16, 3), // Perc
    ];

    // Set initial rotations for musicality
    generators[1].set_rotation(4); // Snare on 5, 13 (if 4/16)

    // 4. Setup TUI
    let mut tui = AppTui::new()?;

    // 5. Main Loop State
    let mut current_step = 0;
    let mut last_step_time = Instant::now();
    let mut bpm = 120.0;
    let mut last_monitor_update = Instant::now();

    // Run loop
    loop {
        // Handle Input
        if tui.should_quit()? {
            break;
        }

        // Update Monitor & Parameters (every 100ms)
        if last_monitor_update.elapsed() >= Duration::from_millis(100) {
            monitor.update();
            last_monitor_update = Instant::now();

            let cores = monitor.get_cpu_usage_per_core();
            let global_cpu = monitor.get_global_cpu_usage();
            let memory = monitor.get_memory_usage();

            // Map Parameters
            // Tempo: 80 - 180 BPM based on global load
            bpm = 80.0 + (global_cpu * 100.0).clamp(0.0, 100.0);

            // Kick: Core 0
            if let Some(&c0) = cores.get(0) {
                // Map 0.0-1.0 to 1-16 pulses
                let pulses = (c0 * 16.0).round() as usize;
                generators[0].set_params(16, pulses.max(1));
            }

            // Snare: Core 1
            if let Some(&c1) = cores.get(1) {
                let pulses = (c1 * 8.0).round() as usize;
                generators[1].set_params(16, pulses.max(1));
            }

            // HiHat: Memory (Steady if low, chaotic if high)
            let hh_pulses = (memory * 16.0).round() as usize;
            generators[2].set_params(16, hh_pulses.max(4));

            // Perc: Core 2 (or Global if single core)
            if let Some(&c2) = cores.get(2).or(cores.get(0)) {
                let pulses = (c2 * 12.0).round() as usize;
                generators[3].set_params(16, pulses.max(2));
            }
        }

        // Sequencer Step
        // 16th notes: (60 / BPM) / 4 seconds per step
        let step_duration_secs = (60.0 / bpm) / 4.0;
        let step_duration = Duration::from_secs_f32(step_duration_secs);

        if last_step_time.elapsed() >= step_duration {
            last_step_time = Instant::now();

            // Trigger Sounds
            if generators[0].get_beat_at(current_step) {
                let _ = audio_tx.send(AudioEvent::Kick);
            }
            if generators[1].get_beat_at(current_step) {
                let _ = audio_tx.send(AudioEvent::Snare);
            }
            if generators[2].get_beat_at(current_step) {
                let _ = audio_tx.send(AudioEvent::HiHat);
            }
            if generators[3].get_beat_at(current_step) {
                // Modulate pitch based on step
                let pitch = 400.0 + (current_step as f32 * 50.0);
                let _ = audio_tx.send(AudioEvent::Perc(pitch));
            }

            current_step += 1;
        }

        // Draw TUI
        tui.draw(&monitor, &generators, current_step)?;

        // Sleep briefly to yield (aim for 60 FPS update rate for TUI, logic runs faster if needed)
        // Actually, we just spin/sleep lightly.
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
