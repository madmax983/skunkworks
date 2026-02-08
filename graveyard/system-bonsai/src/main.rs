use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::canvas::Canvas,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use sysinfo::System;
use tui_shared::Tui;

mod process_tree;
mod renderer;

use process_tree::ProcessTree;
use renderer::BonsaiRenderer;

struct App {
    system: System,
    tree: ProcessTree,
    renderer: BonsaiRenderer,
    last_refresh: Instant,
    refresh_rate: Duration,
    exit: bool,
}

impl App {
    fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let tree = ProcessTree::new(&system);

        Self {
            system,
            tree,
            renderer: BonsaiRenderer::new(),
            last_refresh: Instant::now(),
            refresh_rate: Duration::from_secs(2), // Refresh every 2s
            exit: false,
        }
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        while !self.exit {
            tui.terminal.draw(|f| self.ui(f))?;
            self.handle_events()?;

            if self.last_refresh.elapsed() >= self.refresh_rate {
                self.system.refresh_all();
                self.tree = ProcessTree::new(&self.system);
                self.last_refresh = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        KeyCode::Char('r') => {
                            self.system.refresh_all();
                            self.tree = ProcessTree::new(&self.system);
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => self.renderer.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => self.renderer.zoom /= 1.1,
                        KeyCode::Left => self.renderer.pan_x -= 10.0 / self.renderer.zoom,
                        KeyCode::Right => self.renderer.pan_x += 10.0 / self.renderer.zoom,
                        KeyCode::Up => self.renderer.pan_y += 10.0 / self.renderer.zoom,
                        KeyCode::Down => self.renderer.pan_y -= 10.0 / self.renderer.zoom,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        // Canvas
        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("System Bonsai"),
            )
            .x_bounds([
                -100.0 / self.renderer.zoom + self.renderer.pan_x,
                100.0 / self.renderer.zoom + self.renderer.pan_x,
            ])
            .y_bounds([
                -20.0 / self.renderer.zoom + self.renderer.pan_y,
                180.0 / self.renderer.zoom + self.renderer.pan_y,
            ]) // Shifted y-bounds
            .paint(|ctx| {
                self.renderer.render(ctx, &self.tree);
            });

        frame.render_widget(canvas, chunks[0]);

        // Info
        let info_text = vec![
            Span::raw("System Bonsai 🌳 | "),
            Span::styled(
                format!("Processes: {}", self.tree.total_processes),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" | "),
            Span::styled(
                "Controls: [q] Quit, [r] Refresh, [+/-] Zoom, [Arrows] Pan",
                Style::default().fg(Color::Yellow),
            ),
        ];

        let info =
            Paragraph::new(Line::from(info_text)).block(Block::default().borders(Borders::ALL));

        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = app.run(&mut tui);

    if let Err(e) = res {
        tui.exit()?;
        eprintln!("Error: {:?}", e);
    } else {
        tui.exit()?;
    }

    Ok(())
}
