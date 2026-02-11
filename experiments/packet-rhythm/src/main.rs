#[cfg(feature = "audio")]
mod audio;
mod network;
mod visuals;

#[cfg(feature = "audio")]
use audio::Synth;
use network::{PingAgent, TargetId};
use visuals::{draw_system, PacketVisual, TargetVisual};

use crossbeam_channel::unbounded;
use macroquad::prelude::*;

const TARGETS_DATA: &[(&str, &str)] = &[
    ("Google", "8.8.8.8:53"),
    ("Cloudflare", "1.1.1.1:53"),
    ("GitHub", "github.com:443"),
    ("Localhost", "127.0.0.1:80"),
];

#[macroquad::main("Packet Rhythm")]
async fn main() {
    let (tx, rx) = unbounded();
    let agent = PingAgent::new(tx);

    #[cfg(feature = "audio")]
    let synth = Synth::new();

    #[cfg(feature = "audio")]
    if synth.is_none() {
        eprintln!("Warning: Audio initialization failed. Running in visual-only mode.");
    }

    let mut targets = Vec::new();
    for (i, (name, _addr)) in TARGETS_DATA.iter().enumerate() {
        targets.push(TargetVisual {
            id: i,
            name: name.to_string(),
            angle: i as f32 * std::f32::consts::PI / 2.0,
            radius: 150.0,
            last_rtt: None,
            flash: 0.0,
        });
    }

    let mut packets = Vec::new();
    let mut last_tick = get_time();
    let bpm = 120.0;
    let tick_interval = 60.0 / bpm;
    let mut beat_counter = 0;

    loop {
        clear_background(BLACK);
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        // Sequencer
        if get_time() - last_tick > tick_interval {
            last_tick = get_time();
            beat_counter += 1;

            // Kick (Target 0) - Every beat
            if beat_counter % 1 == 0 {
                agent.ping(TargetId(0), TARGETS_DATA[0].1.to_string());
                packets.push(PacketVisual { target_id: 0, progress: 0.0, return_trip: false });
            }

            // Snare (Target 1) - Every 2 beats, offset
            if beat_counter % 2 == 0 {
                 agent.ping(TargetId(1), TARGETS_DATA[1].1.to_string());
                 packets.push(PacketVisual { target_id: 1, progress: 0.0, return_trip: false });
            }

            // HiHat (Target 2) - Every beat
            if beat_counter % 1 == 0 {
                agent.ping(TargetId(2), TARGETS_DATA[2].1.to_string());
                packets.push(PacketVisual { target_id: 2, progress: 0.0, return_trip: false });
            }

            // Bass (Target 3) - Every 4 beats
            if beat_counter % 4 == 0 {
                agent.ping(TargetId(3), TARGETS_DATA[3].1.to_string());
                 packets.push(PacketVisual { target_id: 3, progress: 0.0, return_trip: false });
            }
        }

        // Process Network Events
        while let Ok((id, rtt)) = rx.try_recv() {
            // Find target
            if let Some(target) = targets.iter_mut().find(|t| t.id == id.0) {
                if let Some(ms) = rtt {
                    target.last_rtt = Some(ms);
                    target.flash = 1.0;

                    // Update Radius based on RTT
                    // Scale: 1ms = 0.5px. Base 100px.
                    target.radius = (100.0 + ms as f32 * 0.5).clamp(100.0, 600.0);

                    // Trigger Sound
                    #[cfg(feature = "audio")]
                    if let Some(synth) = &synth {
                        match id.0 {
                            0 => synth.play_kick(),
                            1 => synth.play_snare(),
                            2 => synth.play_hihat(),
                            3 => synth.play_bass(110.0),
                            _ => {}
                        }
                    }
                } else {
                    // Timeout / Failure
                    target.flash = -1.0; // Negative flash = Red?
                }
            }

            // Remove the oldest packet for this target
            if let Some(idx) = packets.iter().position(|p| p.target_id == id.0) {
                packets.remove(idx);
            }
        }

        // Update Visuals
        let dt = get_frame_time();

        // Update Packets
        let packet_speed = 300.0; // px/sec

        for packet in packets.iter_mut() {
            if let Some(target) = targets.iter().find(|t| t.id == packet.target_id) {
                let dist = target.radius;
                if dist > 0.0 {
                    packet.progress += (packet_speed * dt) / dist;
                    // Clamp to 1.0 (Wait at node until network response)
                    packet.progress = packet.progress.min(1.0);
                }
            }
        }

        // Update Targets
        for target in &mut targets {
            if target.flash > 0.0 {
                target.flash = (target.flash - dt * 5.0).max(0.0);
            } else if target.flash < 0.0 {
                 target.flash = (target.flash + dt * 2.0).min(0.0);
            }
            target.angle += dt * 0.2;
        }

        draw_system(center, &targets, &packets);

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, GRAY);
        draw_text("Packet Rhythm: Network Latency -> Sound", 20.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}
