use crossbeam_channel::bounded;
use hyper_system::SystemMonitor;
use macroquad::prelude::*;
use resonance_audio::{AudioCommand, AudioModel};

#[macroquad::main("Hyper Resonance")]
async fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode: bypassing UI.");
        return;
    }

    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, snap_rx) = bounded(1);

    // Spawn the audio thread
    std::thread::spawn(move || {
        let mut model = AudioModel::new(100, 100, cmd_rx, snap_tx, None);
        let mut dummy_buffer = vec![0.0; 512];
        loop {
            model.process(&mut dummy_buffer);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    });

    let mut monitor = SystemMonitor::new();

    loop {
        monitor.update();
        let cpu = monitor.cpu_usage;

        if cpu > 0.1 && macroquad::rand::gen_range(0.0, 1.0) < cpu * 0.1 {
            let x = macroquad::rand::gen_range(10, 90);
            let y = macroquad::rand::gen_range(10, 90);
            let _ = cmd_tx.try_send(AudioCommand::Pluck { x, y, strength: cpu });
        }

        clear_background(BLACK);

        if let Ok(snapshot) = snap_rx.try_recv() {
            let w = screen_width() / 100.0;
            let h = screen_height() / 100.0;
            for y in 0..100 {
                for x in 0..100 {
                    let pressure = snapshot.pressure[y * 100 + x];
                    if pressure.abs() > 0.01 {
                        let c = (pressure.abs() * 5.0).clamp(0.0, 1.0);
                        draw_rectangle(x as f32 * w, y as f32 * h, w, h, Color::new(c, c, c, 1.0));
                    }
                }
            }
        }

        draw_text(format!("CPU Stress: {:.2}%", cpu * 100.0).as_str(), 10.0, 20.0, 20.0, RED);
        next_frame().await;
    }
}
