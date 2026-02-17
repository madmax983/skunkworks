use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod simulation;
mod stego;

use simulation::World;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the image file (optional). If not provided, a default cover is generated.
    #[arg(short, long)]
    image: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize TUI
    let mut tui = Tui::init()?;

    // Load or Generate Image
    let img = if let Some(path) = args.image {
        image::open(path)?.to_rgba8()
    } else {
        // Generate default cover
        let payload = b"THE TRUTH IS IN THE SPORES. BIOLOGICAL DECRYPTION ACTIVE.";
        // 160x100 to match TUI scale approx, or larger?
        // stego-cartridge uses 512x512. Let's use 200x120 for a nice TUI fit.
        let width = 200;
        let height = 120;
        let mut img = stego::generate_cover(payload, width, height);
        stego::embed(&mut img, payload).expect("Failed to embed payload");
        img
    };

    let width = img.width() as usize;
    let height = img.height() as usize;

    // Initialize Simulation
    // 5000 agents for density
    let (mut world, mut agents) = World::from_image(&img, 5000);

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);

    loop {
        // Collect trails for rendering
        trails_low.clear();
        trails_med.clear();
        trails_high.clear();

        for y in 0..height {
            for x in 0..width {
                let val = world.get_trail(x, y);
                if val > 50.0 {
                    trails_high.push((x as f64, y as f64));
                } else if val > 20.0 {
                    trails_med.push((x as f64, y as f64));
                } else if val > 5.0 {
                    trails_low.push((x as f64, y as f64));
                }
            }
        }

        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Myco-Stego: Biological Steganalysis"),
                )
                .marker(ratatui::symbols::Marker::Block)
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: &trails_low,
                        color: Color::DarkGray,
                    });
                    ctx.draw(&Points {
                        coords: &trails_med,
                        color: Color::Green,
                    });
                    ctx.draw(&Points {
                        coords: &trails_high,
                        color: Color::Cyan, // High pheromone = Data?
                    });
                });

            f.render_widget(canvas, chunks[0]);

            let status = Line::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" to quit. Agents: "),
                Span::styled(format!("{}", agents.len()), Style::default().fg(Color::Cyan)),
                Span::raw(format!(" | Resolution: {}x{}", width, height)),
            ]);
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update_agents_parallel(&mut agents);
            world.diffuse_and_decay();

            // Re-seed data?
            // If we want the agents to find the static LSBs continuously, we should add the LSB values back.
            // But we don't keep the LSB map in World struct in this version.
            // The initial trails were seeded.
            // The agents deposit pheromone.
            // If the LSBs are "Food", they should regenerate.
            // But I didn't save the LSB map.
            // So currently, the LSBs are an "Initial Condition" that shapes the initial swarm.
            // The swarm will then maintain the pattern if they loop.
            // If the pattern is a line, they will loop along the line.
            // This is acceptable for version 1.

            last_tick = Instant::now();
        }
    }

    Ok(())
}
