use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{bounded, Sender};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

use crate::audio::{AudioCommand, AudioModel};

pub mod physics;
pub mod audio;

fn main() -> Result<()> {
    // Audio Setup
    let host = cpal::default_host();
    // Use default device or fail gracefully if none (e.g. CI)
    // If we fail here, we might want to run without audio?
    // But the prompt says "Preferred Stack: cpal".
    // I'll assume audio is available or fail.
    let device = match host.default_output_device() {
        Some(d) => d,
        None => {
             eprintln!("No audio device found. Running in visual-only mode (mocking audio thread).");
             // For visual only, we need to spawn a thread that simulates the audio callback loop.
             // This is good for CI/Cloud too.
             return run_visual_only();
        }
    };

    let config = device.default_output_config()?;

    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(2);

    let width = 60;
    let height = 30;

    let mut model = AudioModel::new(width, height, cmd_rx, snap_tx);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream_result = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    model.process(data);
                },
                err_fn,
                None,
            )
        }
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported for this demo")),
    };

    let stream = stream_result?;
    stream.play()?;

    // TUI Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, width, height, cmd_tx, snap_rx);

    // Restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

// Fallback for no audio device
fn run_visual_only() -> Result<()> {
    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(2);
    let width = 60;
    let height = 30;

    // Spawn mock audio thread
    let mut model = AudioModel::new(width, height, cmd_rx, snap_tx);
    std::thread::spawn(move || {
        let mut buffer = vec![0.0; 1024];
        loop {
            model.process(&mut buffer);
            std::thread::sleep(Duration::from_millis(20)); // Approx 50Hz
        }
    });

    // TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, width, height, cmd_tx, snap_rx);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res.map_err(|e| anyhow::anyhow!(e))
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    width: usize,
    height: usize,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<Vec<f32>>,
) -> io::Result<()> {
    let mut cursor_x = width / 2;
    let mut cursor_y = height / 2;
    let mut listener_x = width / 2;
    let mut listener_y = height / 2;

    let mut grid_u = vec![0.0; width * height];
    let mut walls = vec![false; width * height];

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            grid_u = snap;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let area = chunks[0];

            let mut lines = Vec::new();
            for y in 0..height {
                let mut spans = Vec::new();
                for x in 0..width {
                    if x >= width || y >= height { continue; }
                    let idx = y * width + x;
                    let u = grid_u.get(idx).unwrap_or(&0.0);
                    let val = *u;

                    let mut style = Style::default();
                    let mut ch = ' ';

                    if x == cursor_x && y == cursor_y {
                        style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                        ch = 'X';
                    } else if x == listener_x && y == listener_y {
                        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                        ch = 'L';
                    } else if walls.get(idx).cloned().unwrap_or(false) {
                        style = style.fg(Color::White);
                        ch = '#';
                    } else {
                        if val > 0.05 {
                             style = style.fg(Color::Green);
                             if val > 0.5 { ch = '@'; }
                             else if val > 0.2 { ch = 'O'; }
                             else { ch = '.'; }
                        } else if val < -0.05 {
                            style = style.fg(Color::Red);
                             if val < -0.5 { ch = '@'; }
                             else if val < -0.2 { ch = 'O'; }
                             else { ch = '.'; }
                        } else if val.abs() > 0.01 {
                            style = style.fg(Color::DarkGray);
                            ch = '·';
                        }
                    };

                    spans.push(Span::styled(String::from(ch), style));
                }
                lines.push(Line::from(spans));
            }

            let grid_widget = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Resonance Chamber"));
            f.render_widget(grid_widget, area);

            let info = Paragraph::new("Arrows: Move | Space: Pluck | m: Move Listener | w: Wall | q: Quit")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                 match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => cursor_y = cursor_y.saturating_sub(1),
                    KeyCode::Down => if cursor_y < height - 1 { cursor_y += 1; },
                    KeyCode::Left => cursor_x = cursor_x.saturating_sub(1),
                    KeyCode::Right => if cursor_x < width - 1 { cursor_x += 1; },
                    KeyCode::Char(' ') => {
                        let _ = cmd_tx.send(AudioCommand::Pluck { x: cursor_x, y: cursor_y, strength: 1.0 });
                    },
                    KeyCode::Char('w') => {
                        walls[cursor_y * width + cursor_x] = true;
                         let _ = cmd_tx.send(AudioCommand::AddWall { x: cursor_x, y: cursor_y });
                    },
                    KeyCode::Char('m') => {
                        listener_x = cursor_x;
                        listener_y = cursor_y;
                         let _ = cmd_tx.send(AudioCommand::MoveListener { x: cursor_x, y: cursor_y });
                    },
                    _ => {}
                }
            }
        }
    }
}
