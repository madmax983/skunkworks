mod topology;
mod fs;
mod renderer;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use glam::Vec3;
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};

use fs::{scan_directory, FsNode};
use renderer::{draw_nodes, draw_wireframe, Camera};

struct App {
    current_path: PathBuf,
    fs_nodes: Vec<FsNode>,
    selected_idx: usize,

    // Camera state
    camera_angle: f32,
    camera_height: f32,
    camera_radius: f32,

    should_quit: bool,
}

impl App {
    fn new() -> Result<Self> {
        let current_path = std::env::current_dir()?;
        let fs_nodes = scan_directory(&current_path)?;

        Ok(Self {
            current_path,
            fs_nodes,
            selected_idx: 0,
            camera_angle: 0.0,
            camera_height: 1.0,
            camera_radius: 6.0,
            should_quit: false,
        })
    }

    fn update_scan(&mut self) -> Result<()> {
        match scan_directory(&self.current_path) {
            Ok(nodes) => {
                self.fs_nodes = nodes;
                self.selected_idx = 0;
            }
            Err(e) => {
                // In a real app we'd show an error
                eprintln!("Error scanning: {}", e);
            }
        }
        Ok(())
    }

    fn on_tick(&mut self) {
        // Auto-rotate slowly if desired, or just smooth move
        if !self.fs_nodes.is_empty() {
             // Placeholder for auto-focus logic
             let _target_node = &self.fs_nodes[self.selected_idx];
        }
    }

    fn move_selection(&mut self, delta: i32) {
        if self.fs_nodes.is_empty() {
            return;
        }
        let len = self.fs_nodes.len();
        self.selected_idx = (self.selected_idx as i32 + delta).rem_euclid(len as i32) as usize;
    }

    fn enter_directory(&mut self) -> Result<()> {
        if self.fs_nodes.is_empty() {
            return Ok(());
        }
        let node = &self.fs_nodes[self.selected_idx];
        if node.is_dir {
            self.current_path = node.path.clone();
            self.update_scan()?;
        }
        Ok(())
    }

    fn go_up(&mut self) -> Result<()> {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.update_scan()?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;
    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Left => app.camera_angle -= 0.1,
                    KeyCode::Right => app.camera_angle += 0.1,
                    KeyCode::Up => app.move_selection(-1),
                    KeyCode::Down => app.move_selection(1),
                    KeyCode::Char('w') => app.camera_height += 0.5,
                    KeyCode::Char('s') => app.camera_height -= 0.5,
                    KeyCode::Enter => { app.enter_directory()?; },
                    KeyCode::Backspace => { app.go_up()?; },
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Info Bar
    let path_str = app.current_path.display().to_string();
    let selected_name = if !app.fs_nodes.is_empty() {
        &app.fs_nodes[app.selected_idx].name
    } else {
        "<empty>"
    };

    let info_text = vec![
        Line::from(vec![
            Span::styled("Path: ", Style::default().fg(Color::Cyan)),
            Span::raw(&path_str),
            Span::raw(" | Selected: "),
            Span::styled(selected_name, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Arrows: Rotate/Select | W/S: Height | Enter: Open | Backspace: Up | Q: Quit"),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Klein File System"));
    f.render_widget(info, chunks[1]);

    // 3D Canvas
    let camera_pos = Vec3::new(
        app.camera_radius * app.camera_angle.cos(),
        app.camera_height,
        app.camera_radius * app.camera_angle.sin(),
    );
    let camera = Camera::new(camera_pos, Vec3::ZERO);

    let canvas_area = chunks[0];
    let width = canvas_area.width as f32;
    let height = canvas_area.height as f32;
    // Aspect ratio of the viewport in characters.
    // Characters are roughly 1x2 pixels (or 0.5 aspect).
    // So 1 unit of X is 0.5 unit of Y visual width.
    // To make a circle look circular, we need to correct for this?
    // Ratatui Canvas (with Braille) is 2x4 dots per character.
    // The canvas coordinate system is arbitrary.
    // If we use square bounds [-1,1], [-1,1], it will look squashed if the viewport is not square.
    // We pass aspect to projection matrix to handle FOV.
    // aspect = width / height.
    // Note: Terminal cells are non-square. Usually 1 cell width ~ 0.5 cell height.
    // So visual aspect = (width * 0.5) / height?
    // Let's assume standard font aspect ratio of 0.5.
    let aspect = (width * 0.5) / height;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Figure-8 Immersion"))
        .x_bounds([-2.0, 2.0])
        .y_bounds([-1.5, 1.5])
        .paint(move |ctx| {
            draw_wireframe(ctx, &camera, aspect);
            draw_nodes(ctx, &camera, aspect, &app.fs_nodes, app.selected_idx);
        });

    f.render_widget(canvas, chunks[0]);
}
