use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Gauge, Paragraph},
};
use std::{
    io::{self, BufRead},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tui_semantic::{PropValue, Snapshot};

struct State {
    snapshots: Vec<Snapshot>,
    play_head: usize,
    playing: bool,
}

pub fn run_player() -> Result<()> {
    // Shared state
    let state = Arc::new(Mutex::new(State {
        snapshots: Vec::new(),
        play_head: 0,
        playing: true,
    }));

    // Spawn reader thread
    let state_clone = state.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines().map_while(Result::ok) {
            if let Ok(snapshot) = serde_json::from_str::<Snapshot>(&line) {
                if let Ok(mut s) = state_clone.lock() {
                    s.snapshots.push(snapshot);
                }
            }
        }
    });

    // Init TUI
    let mut tui = tui_shared::Tui::init()?;
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        {
            let mut s = state.lock().unwrap();

            // Auto-play logic
            if s.playing && !s.snapshots.is_empty() {
                if s.play_head < s.snapshots.len() - 1 {
                    s.play_head += 1;
                } else {
                    // Loop
                    s.play_head = 0;
                }
            }

            // Draw
            tui.terminal.draw(|f| {
                let size = f.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(0),
                        Constraint::Length(3),
                        Constraint::Length(1),
                    ])
                    .split(size);

                let canvas_area = chunks[0];
                let timeline_area = chunks[1];
                let status_area = chunks[2];

                if s.snapshots.is_empty() {
                    let p = Paragraph::new("Waiting for data... (Pipe JSON snapshots to stdin)")
                        .block(Block::default().borders(Borders::ALL));
                    f.render_widget(p, canvas_area);
                    return;
                }

                let snap = &s.snapshots[s.play_head];
                let (vp_w, vp_h) = snap.viewport.unwrap_or((100, 50));

                // Canvas
                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title(format!(
                        " {} (Frame {}) ",
                        snap.app,
                        snap.frame.unwrap_or(0)
                    )))
                    .x_bounds([0.0, vp_w as f64])
                    .y_bounds([0.0, vp_h as f64])
                    .paint(|ctx| {
                        for entity in &snap.entities {
                            if let Some(pos) = entity.position {
                                let color = match entity.props.get("color") {
                                    Some(PropValue::Text(c)) => match c.as_str() {
                                        "red" => Color::Red,
                                        "blue" => Color::Blue,
                                        "green" => Color::Green,
                                        "cyan" => Color::Cyan,
                                        "yellow" => Color::Yellow,
                                        "magenta" => Color::Magenta,
                                        _ => Color::White,
                                    },
                                    _ => Color::White,
                                };

                                let symbol =
                                    entity.display.clone().unwrap_or_else(|| "?".to_string());
                                // Flip Y for TUI canvas (0,0 is bottom-left, but snapshot y=0 is top)
                                let y = vp_h as f64 - pos.y;
                                ctx.print(
                                    pos.x,
                                    y,
                                    Span::styled(symbol, Style::default().fg(color)),
                                );
                            }
                        }
                    });
                f.render_widget(canvas, canvas_area);

                // Timeline
                let progress = if s.snapshots.len() > 1 {
                    s.play_head as f64 / (s.snapshots.len() - 1) as f64
                } else {
                    0.0
                };
                let gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title(" Timeline "))
                    .gauge_style(Style::default().fg(Color::Yellow))
                    .ratio(progress)
                    .label(format!("{}/{}", s.play_head, s.snapshots.len()));
                f.render_widget(gauge, timeline_area);

                // Status
                let status = Paragraph::new(" [Space] Pause/Play | [Left/Right] Seek | [Q] Quit ");
                f.render_widget(status, status_area);
            })?;
        }

        // Handle Events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let mut s = state.lock().unwrap();
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => s.playing = !s.playing,
                        KeyCode::Left => {
                            s.playing = false;
                            if s.play_head > 0 {
                                s.play_head -= 1;
                            }
                        }
                        KeyCode::Right => {
                            s.playing = false;
                            if !s.snapshots.is_empty() && s.play_head < s.snapshots.len() - 1 {
                                s.play_head += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
