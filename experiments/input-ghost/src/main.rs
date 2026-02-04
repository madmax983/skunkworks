use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::{fs, time::Duration};
use tui_shared::{
    event::{GhostRecorder, GhostReplayer},
    Tui,
};

#[derive(PartialEq)]
enum AppState {
    Idle,
    Recording,
    Replaying,
}

struct App {
    x: u16,
    y: u16,
    recorder: GhostRecorder,
    replayer: Option<GhostReplayer>,
    state: AppState,
    status_msg: String,
    event_count: usize,
}

impl App {
    fn new() -> Self {
        Self {
            x: 10,
            y: 10,
            recorder: GhostRecorder::new(),
            replayer: None,
            state: AppState::Idle,
            status_msg: "Press 'r' to record, 'p' to replay".to_string(),
            event_count: 0,
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        // In Replaying mode, we check for real input to abort, and poll replayer
        if app.state == AppState::Replaying {
            // Check for abort
            if event::poll(Duration::from_millis(5))? {
                let event = event::read()?;
                if let Event::Key(key) = event {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        app.state = AppState::Idle;
                        app.replayer = None;
                        app.status_msg = "Replay aborted".to_string();
                        continue;
                    }
                }
            }

            // Poll replayer
            if let Some(replayer) = &mut app.replayer {
                if let Some(event) = replayer.poll() {
                    process_event(&mut app, event, true);
                }
            }
        } else {
            // Normal mode
            if event::poll(Duration::from_millis(16))? {
                let event = event::read()?;

                if app.state == AppState::Recording {
                    app.recorder.record(event.clone());
                    app.event_count = app.recorder.events.len();
                }

                if !process_event(&mut app, event, false) {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn process_event(app: &mut App, event: Event, is_replay: bool) -> bool {
    if let Event::Key(key) = event {
        if !is_replay {
            match key.code {
                KeyCode::Char('q') => return false,
                KeyCode::Char('r') => {
                    if app.state == AppState::Idle {
                        app.state = AppState::Recording;
                        app.recorder.start();
                        app.status_msg = "RECORDING... (Press 'r' to stop)".to_string();
                    } else if app.state == AppState::Recording {
                        app.state = AppState::Idle;
                        app.status_msg = format!(
                            "Recorded {} events. Press 'p' to replay, 's' to save.",
                            app.recorder.events.len()
                        );
                    }
                    return true;
                }
                KeyCode::Char('p') => {
                    if app.state == AppState::Idle && !app.recorder.events.is_empty() {
                        app.state = AppState::Replaying;
                        let mut replayer = GhostReplayer::new(app.recorder.events.clone());
                        replayer.start();
                        app.replayer = Some(replayer);
                        app.status_msg = "REPLAYING... (Press 'q' to abort)".to_string();
                        // Reset state for replay
                        app.x = 10;
                        app.y = 10;
                        return true;
                    }
                }
                KeyCode::Char('s') => {
                    if !app.recorder.events.is_empty() {
                        if let Ok(json) = app.recorder.to_json() {
                            if fs::write("ghost_recording.json", json).is_ok() {
                                app.status_msg = "Saved to ghost_recording.json".to_string();
                            } else {
                                app.status_msg = "Failed to save".to_string();
                            }
                        }
                    }
                    return true;
                }
                KeyCode::Char('l') => {
                    if let Ok(json) = fs::read_to_string("ghost_recording.json") {
                        if let Ok(replayer) = GhostReplayer::from_json(&json) {
                            app.state = AppState::Replaying;
                            let mut replayer = replayer;
                            replayer.start();
                            app.replayer = Some(replayer);
                            app.status_msg = "REPLAYING FROM FILE...".to_string();
                            app.x = 10;
                            app.y = 10;
                        } else {
                            app.status_msg = "Failed to parse json".to_string();
                        }
                    } else {
                        app.status_msg = "File not found".to_string();
                    }
                    return true;
                }
                _ => {}
            }
        }

        // Movement logic (runs for both real and replay)
        match key.code {
            KeyCode::Left => app.x = app.x.saturating_sub(1),
            KeyCode::Right => app.x = app.x.saturating_add(1),
            KeyCode::Up => app.y = app.y.saturating_sub(1),
            KeyCode::Down => app.y = app.y.saturating_add(1),
            _ => {}
        }
    }
    true
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Draw canvas
    let canvas_block = Block::default().borders(Borders::ALL).title("Ghost Canvas");
    f.render_widget(canvas_block, chunks[0]);

    // Draw the "Player" manually on the buffer
    let buf = f.buffer_mut();
    // Offset by block border (1,1)
    let draw_x = chunks[0].x + 1 + app.x;
    let draw_y = chunks[0].y + 1 + app.y;

    if draw_x < chunks[0].right() - 1 && draw_y < chunks[0].bottom() - 1 {
        buf.cell_mut((draw_x, draw_y))
            .unwrap()
            .set_symbol("O")
            .set_fg(Color::Cyan);
    }

    // Status bar
    let status_style = match app.state {
        AppState::Idle => Style::default(),
        AppState::Recording => Style::default().fg(Color::Red),
        AppState::Replaying => Style::default().fg(Color::Green),
    };

    let p = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(
                "[{}] ",
                match app.state {
                    AppState::Idle => "IDLE",
                    AppState::Recording => "REC",
                    AppState::Replaying => "PLAY",
                }
            ),
            status_style,
        ),
        Span::raw(&app.status_msg),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[1]);
}
