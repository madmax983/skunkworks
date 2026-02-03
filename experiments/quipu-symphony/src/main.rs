#![allow(dead_code, unused_imports)]

mod audio;
mod quipu;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};
use tui_shared::Tui;

use crate::audio::{AudioEngine, AudioEvent};
use crate::quipu::{Cord, Knot};

struct CordTrack {
    cord: Cord,
    name: String,
    color: Color,
    base_freq: f32, // For Pluck
    sound_type: TrackSound,
}

#[derive(Clone, Copy)]
enum TrackSound {
    Percussion, // Kick (Simples), Snare (Longs), HiHat (Figure8)
    Melodic,    // Pluck based on knot value
}

struct AppState {
    tracks: Vec<CordTrack>,
    playhead_y: f32, // 0.0 (Top) to 1.0 (Bottom)
    speed: f32,      // Screens per second
    playing: bool,
    last_update: Instant,
    triggered_clusters: HashSet<(usize, usize)>, // (track_idx, cluster_idx)
}

impl AppState {
    fn new() -> Self {
        Self {
            tracks: Vec::new(),
            playhead_y: 0.0,
            speed: 0.2, // 5 seconds per loop
            playing: true,
            last_update: Instant::now(),
            triggered_clusters: HashSet::new(),
        }
    }

    fn regenerate(&mut self) {
        self.tracks.clear();

        // Track 1: Rhythm (Percussion)
        // A cord with random clusters
        let mut cord1 = Cord::new();
        for _ in 0..4 {
            let val = rand::random::<u8>() % 3;
            if val == 0 {
                cord1.clusters.push(vec![Knot::Simple]);
            } else if val == 1 {
                cord1.clusters.push(vec![Knot::Long(3)]);
            } else {
                cord1.clusters.push(vec![Knot::FigureEight]);
            }
        }
        // Pad with empty to ensure spacing
        let mut final_clusters_1 = Vec::new();
        for c in cord1.clusters {
            final_clusters_1.push(c);
            final_clusters_1.push(Vec::new()); // Gap
        }
        cord1.clusters = final_clusters_1;

        self.tracks.push(CordTrack {
            cord: cord1,
            name: "Rhythm".to_string(),
            color: Color::Red,
            base_freq: 0.0,
            sound_type: TrackSound::Percussion,
        });

        // Track 2: Melody (Bass)
        let mut cord2 = Cord::new();
        for _ in 0..8 {
            let val = rand::random::<u8>() % 10;
            if val > 0 {
                cord2.clusters.push(vec![Knot::Long(val)]);
            } else {
                cord2.clusters.push(Vec::new());
            }
        }
        self.tracks.push(CordTrack {
            cord: cord2,
            name: "Bass".to_string(),
            color: Color::Blue,
            base_freq: 110.0,
            sound_type: TrackSound::Melodic,
        });

        // Track 3: Melody (Lead)
        let mut cord3 = Cord::new();
        for _ in 0..8 {
            let val = rand::random::<u8>() % 10;
            if val > 0 {
                cord3.clusters.push(vec![Knot::Long(val)]);
            } else {
                cord3.clusters.push(Vec::new());
            }
        }
        self.tracks.push(CordTrack {
            cord: cord3,
            name: "Lead".to_string(),
            color: Color::Yellow,
            base_freq: 440.0,
            sound_type: TrackSound::Melodic,
        });
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let audio = AudioEngine::new()?;
    let tx = audio.get_sender();

    let mut state = AppState::new();
    state.regenerate();

    loop {
        // Update Logic
        if state.playing {
            let now = Instant::now();
            let dt = now.duration_since(state.last_update).as_secs_f32();
            state.last_update = now;

            let old_y = state.playhead_y;
            state.playhead_y += state.speed * dt;

            if state.playhead_y >= 1.0 {
                state.playhead_y = 0.0;
                state.triggered_clusters.clear();
            }

            // Check triggers
            for (t_idx, track) in state.tracks.iter().enumerate() {
                // Determine layout mapping
                // Assuming Cord is rendered from Top (0.0) to Bottom (1.0)
                // We have `track.cord.clusters.len()` slots.
                // But wait, `Cord` struct: index 0 is Units (Bottom).
                // So visually, index 0 is at Y ~ 1.0. Index N is at Y ~ 0.0.

                let num_clusters = track.cord.clusters.len();
                if num_clusters == 0 { continue; }

                let spacing = 1.0 / (num_clusters as f32 + 1.0);

                for (c_idx, cluster) in track.cord.clusters.iter().enumerate() {
                    if cluster.is_empty() { continue; }

                    // Calculate Y position of this cluster
                    // Rev index: Top cluster has index `len-1`.
                    // We want Top cluster at Y = spacing.
                    // Bottom cluster (idx 0) at Y = 1.0 - spacing.

                    // Visual position (0.0 is top)
                    // let visual_idx = (num_clusters - 1) - c_idx; // 0 for top
                    // let cluster_y = (visual_idx as f32 + 1.0) * spacing;

                    // Actually, let's just map index directly to Y for simplicity in generic cords
                    // Let's say index 0 is top. (Reverse of Quipu logic but easier for sequencer).
                    // No, let's stick to Quipu logic: Index 0 is Bottom (Units).

                    let cluster_y = 1.0 - ((c_idx as f32 + 1.0) * spacing);

                    // Check if playhead crossed this Y
                    // Since playhead moves down (increasing Y), we check if old < y <= new
                    // Or if we wrapped around

                    let hit = if state.playhead_y < old_y {
                        // Wrapped
                        (cluster_y > old_y && cluster_y <= 1.0) || (cluster_y >= 0.0 && cluster_y <= state.playhead_y)
                    } else {
                        cluster_y > old_y && cluster_y <= state.playhead_y
                    };

                    if hit {
                        if !state.triggered_clusters.contains(&(t_idx, c_idx)) {
                            state.triggered_clusters.insert((t_idx, c_idx));

                            // Trigger Sound
                            match track.sound_type {
                                TrackSound::Percussion => {
                                    // Use first knot type to decide
                                    if let Some(k) = cluster.first() {
                                        match k {
                                            Knot::Simple => { let _ = tx.send(AudioEvent::Kick); },
                                            Knot::Long(_) => { let _ = tx.send(AudioEvent::Snare); },
                                            Knot::FigureEight => { let _ = tx.send(AudioEvent::HiHat); },
                                        }
                                    }
                                },
                                TrackSound::Melodic => {
                                    // Sum values for pitch offset
                                    let mut val = 0;
                                    for k in cluster {
                                        val += k.value();
                                    }
                                    // scale: pentatonic?
                                    // simple chromatic for now: base * 2^(val/12)
                                    let pitch = track.base_freq * 2.0_f32.powf(val as f32 / 12.0);
                                    let _ = tx.send(AudioEvent::Pluck(pitch));
                                }
                            }
                        }
                    }
                }
            }
        } else {
             state.last_update = Instant::now();
        }

        // Render
        tui.terminal.draw(|f| {
            ui(f, &state);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => state.playing = !state.playing,
                    KeyCode::Char('r') => {
                        state.regenerate();
                        state.playhead_y = 0.0;
                        state.triggered_clusters.clear();
                    },
                    KeyCode::Up => state.speed += 0.05,
                    KeyCode::Down => state.speed = (state.speed - 0.05).max(0.01),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("🎼 QUIPU SYMPHONY: Ancient Data Sequencer")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Canvas for Cords
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Cords"))
        .x_bounds([0.0, state.tracks.len() as f64])
        .y_bounds([0.0, 1.0]) // 0.0 bottom, 1.0 top in Ratatui Canvas usually?
        // Wait, Ratatui Canvas:
        // x: left to right
        // y: bottom to top usually.
        // My logic for playhead was 0.0 Top to 1.0 Bottom.
        // So I need to invert Y when drawing.
        .paint(|ctx| {
            // Draw Playhead
            // If playhead_y is 0.0 (top), Canvas Y should be 1.0.
            let playhead_canvas_y = 1.0 - state.playhead_y as f64;

            ctx.draw(&Line {
                x1: 0.0,
                y1: playhead_canvas_y,
                x2: state.tracks.len() as f64,
                y2: playhead_canvas_y,
                color: Color::White,
            });

            // Draw Tracks
            for (t_idx, track) in state.tracks.iter().enumerate() {
                let x_center = t_idx as f64 + 0.5;

                // Draw Cord Line
                ctx.draw(&Line {
                    x1: x_center,
                    y1: 0.0,
                    x2: x_center,
                    y2: 1.0,
                    color: Color::DarkGray,
                });

                // Draw Knots
                let num_clusters = track.cord.clusters.len();
                let spacing = 1.0 / (num_clusters as f32 + 1.0);

                for (c_idx, cluster) in track.cord.clusters.iter().enumerate() {
                     if cluster.is_empty() { continue; }

                     // Same Y logic as main loop
                     // cluster_y (0-1, 0 is top) = 1.0 - ((c_idx + 1) * spacing)
                     // Canvas Y (0 bottom, 1 top) = 1.0 - cluster_y
                     //                            = ((c_idx + 1) * spacing)

                     let canvas_y = ((c_idx as f32 + 1.0) * spacing) as f64;

                     // Draw Cluster
                     // Simple circle or rect
                     let color = if state.triggered_clusters.contains(&(t_idx, c_idx)) &&
                                    (state.playhead_y as f64 - (1.0 - canvas_y)).abs() < 0.05 {
                         Color::White // Flash
                     } else {
                         track.color
                     };

                     // Draw each knot in cluster stacked horizontally?
                     // Or just one big blob?
                     // Let's draw a blob.

                     let mut radius = 0.02;
                     for k in cluster {
                         match k {
                            Knot::Simple => radius += 0.01,
                            Knot::Long(v) => radius += 0.01 * *v as f64,
                            Knot::FigureEight => radius += 0.03,
                         }
                     }

                     ctx.draw(&Rectangle {
                         x: x_center - radius,
                         y: canvas_y - 0.02,
                         width: radius * 2.0,
                         height: 0.04,
                         color,
                     });

                     // Label value?
                     // Can't draw text in Canvas easily without newer Ratatui features or custom painter
                     // Just use shape.
                }

                // Draw Track Name at the bottom of the column?
                // Canvas paint doesn't support text easily.
                // We'll leave it for now.
            }
        });
    f.render_widget(canvas, chunks[1]);

    let instructions = Paragraph::new("Space: Play/Pause | R: Regenerate | Up/Down: Speed | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}
