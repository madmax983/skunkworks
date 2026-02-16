use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Circle, Context},
        Block, Borders, Paragraph,
    },
};
use std::f64::consts::PI;
use std::time::{Duration, Instant};

mod audio;
mod git;
mod rhythm;

use git::GitSource;
use rhythm::{generate_pattern, Instrument};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file in the git repo
    #[arg(default_value = "README.md")]
    file: String,

    /// First revision (default: HEAD)
    #[arg(long, default_value = "HEAD")]
    rev1: String,

    /// Second revision (default: HEAD~1)
    #[arg(long, default_value = "HEAD~1")]
    rev2: String,

    /// BPM
    #[arg(long, default_value_t = 120)]
    bpm: u32,

    /// Pattern length in steps
    #[arg(long, default_value_t = 64)]
    length: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Git Data
    let source = GitSource::open(".")?;
    let data1 = source.read_file_at_revision(&args.file, &args.rev1)?;
    let data2 = source.read_file_at_revision(&args.file, &args.rev2)?;

    // 2. Rhythm Generation
    let pattern1 = generate_pattern(&data1, args.length);
    let pattern2 = generate_pattern(&data2, args.length);

    // 3. TUI Setup
    let mut tui = tui_shared::Tui::init()?;

    // 4. State
    let mut speed_ratio = 1.0;
    let mut playing = true;
    let mut phase1 = 0.0; // Position in steps (float)
    let mut phase2 = 0.0;
    let mut last_tick = Instant::now();
    let mut status_msg = String::from("Ready");

    let app_result = run_app(
        &mut tui.terminal,
        &args,
        &pattern1,
        &pattern2,
        &mut speed_ratio,
        &mut playing,
        &mut phase1,
        &mut phase2,
        &mut last_tick,
        &mut status_msg,
    );

    // Teardown is automatic via Drop, but we can call exit if needed.
    // However, if run_app returns error, tui is dropped and restored.
    // If run_app returns Ok, tui is dropped and restored.

    app_result
}

fn run_app(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    args: &Args,
    pattern1: &[Instrument],
    pattern2: &[Instrument],
    speed_ratio: &mut f32,
    playing: &mut bool,
    phase1: &mut f32,
    phase2: &mut f32,
    last_tick: &mut Instant,
    status_msg: &mut String,
) -> Result<()> {
    loop {
        // Time Delta
        let now = Instant::now();
        let delta = now.duration_since(*last_tick).as_secs_f32();
        *last_tick = now;

        if *playing {
            let steps_per_sec = (args.bpm as f32 * 4.0) / 60.0;
            *phase1 += steps_per_sec * delta;
            *phase2 += steps_per_sec * delta * *speed_ratio;

            if *phase1 >= args.length as f32 {
                *phase1 -= args.length as f32;
            }
            if *phase2 >= args.length as f32 {
                *phase2 -= args.length as f32;
            }
        }

        // Draw
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" ⚛️ Git Phase ⚛️ "))
                .x_bounds([-100.0, 100.0])
                .y_bounds([-100.0, 100.0])
                .paint(|ctx| {
                    draw_rings(ctx, pattern1, pattern2, *phase1, *phase2);
                });

            f.render_widget(canvas, chunks[0]);

            let info = format!(
                "File: {} | {} vs {} | BPM: {} | Ratio: {:.4}\n[Space] Pause [Arrows] Phase [R] Render [Q] Quit | {}",
                args.file, args.rev1, args.rev2, args.bpm, speed_ratio, status_msg
            );
            f.render_widget(Paragraph::new(info).block(Block::default().borders(Borders::ALL)), chunks[1]);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => *playing = !*playing,
                        KeyCode::Left => *speed_ratio -= 0.001,
                        KeyCode::Right => *speed_ratio += 0.001,
                        KeyCode::Char('r') => {
                            *status_msg = "Rendering output.wav...".to_string();
                            // Render (120 beats duration)
                            match audio::render_wav(
                                "output.wav",
                                pattern1,
                                pattern2,
                                args.bpm,
                                120,
                                *speed_ratio,
                            ) {
                                Ok(_) => *status_msg = "Rendered output.wav".to_string(),
                                Err(e) => *status_msg = format!("Error: {}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn draw_rings(ctx: &mut Context, p1: &[Instrument], p2: &[Instrument], phase1: f32, phase2: f32) {
    let center_x = 0.0;
    let center_y = 0.0;
    let radius1 = 80.0;
    let radius2 = 60.0;

    // Outer Ring (P1)
    ctx.draw(&Circle {
        x: center_x,
        y: center_y,
        radius: radius1,
        color: Color::DarkGray,
    });
    for (i, instr) in p1.iter().enumerate() {
        let angle = (i as f64 / p1.len() as f64) * 2.0 * PI - PI / 2.0;
        let x = center_x + radius1 * angle.cos();
        let y = center_y + radius1 * angle.sin();
        let color = match instr {
            Instrument::Kick => Color::Red,
            Instrument::Snare => Color::Cyan,
            Instrument::Hat => Color::Yellow,
            Instrument::Rest => Color::DarkGray,
        };

        let r = if *instr == Instrument::Rest { 1.0 } else { 2.0 };
        ctx.draw(&Circle {
            x,
            y,
            radius: r,
            color,
        });
    }

    // Cursor P1
    let angle1 = (phase1 as f64 / p1.len() as f64) * 2.0 * PI - PI / 2.0;
    let x1 = center_x + radius1 * angle1.cos();
    let y1 = center_y + radius1 * angle1.sin();
    ctx.draw(&Circle {
        x: x1,
        y: y1,
        radius: 3.0,
        color: Color::White,
    });

    // Inner Ring (P2)
    ctx.draw(&Circle {
        x: center_x,
        y: center_y,
        radius: radius2,
        color: Color::DarkGray,
    });
    for (i, instr) in p2.iter().enumerate() {
        let angle = (i as f64 / p2.len() as f64) * 2.0 * PI - PI / 2.0;
        let x = center_x + radius2 * angle.cos();
        let y = center_y + radius2 * angle.sin();
        let color = match instr {
            Instrument::Kick => Color::Red,
            Instrument::Snare => Color::Cyan,
            Instrument::Hat => Color::Yellow,
            Instrument::Rest => Color::DarkGray,
        };
        let r = if *instr == Instrument::Rest { 1.0 } else { 2.0 };
        ctx.draw(&Circle {
            x,
            y,
            radius: r,
            color,
        });
    }

    // Cursor P2
    let angle2 = (phase2 as f64 / p2.len() as f64) * 2.0 * PI - PI / 2.0;
    let x2 = center_x + radius2 * angle2.cos();
    let y2 = center_y + radius2 * angle2.sin();
    ctx.draw(&Circle {
        x: x2,
        y: y2,
        radius: 3.0,
        color: Color::White,
    });
}
