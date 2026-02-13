use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::time::Duration;

use crate::decay::DecayEngine;
use crate::platter::Platter;

pub struct App {
    pub platter: Platter,
    pub decay_engine: DecayEngine,
    pub cursor: usize,
    pub exit: bool,
}

impl App {
    pub fn new(path: &str) -> Self {
        // Initial arbitrary size, but platter resizes to capacity anyway
        let width = 128;
        let height = 64;
        let mut platter = Platter::new(width, height);
        // Load files from path. If fails, just use empty platter.
        let _ = platter.load_directory(path);

        Self {
            platter,
            decay_engine: DecayEngine::new(),
            cursor: 0,
            exit: false,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()>
    where
        <B as Backend>::Error: Send + Sync + 'static,
    {
        while !self.exit {
            terminal.draw(|f| self.ui(f))?;
            self.handle_events()?;
            self.decay_engine.apply_entropy(&mut self.platter);
        }
        Ok(())
    }

    fn ui(&mut self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(f.area());

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        self.render_platter_view(f, top_chunks[0]);
        self.render_sector_view(f, top_chunks[1]);
        self.render_stats(f, chunks[1]);
    }

    fn render_platter_view(&self, f: &mut ratatui::Frame, area: Rect) {
        let display_width = area.width as usize;
        let display_height = area.height.saturating_sub(2) as usize; // account for borders

        let mut text = String::new();
        // Start showing from cursor position roughly, or just from 0?
        // Let's show from 0 for "Surface View" context.
        // Actually, maybe show a window around cursor?
        // Let's just show from 0 for now to visualize the whole "disk" (part of it).

        for y in 0..display_height {
            for x in 0..display_width {
                let idx = y * display_width + x;
                if idx < self.platter.data.len() {
                    let b = self.platter.data[idx];
                    // Replace non-printable with a dot or block
                    let c = if b.is_ascii_graphic() {
                        b as char
                    } else {
                        '·'
                    };
                    text.push(c);
                }
            }
            text.push('\n');
        }

        let p = Paragraph::new(text).block(
            Block::default()
                .title("Surface View (Map)")
                .borders(Borders::ALL),
        );
        f.render_widget(p, area);
    }

    fn render_sector_view(&self, f: &mut ratatui::Frame, area: Rect) {
        let bytes_per_row = 16;
        let rows = area.height.saturating_sub(2) as usize;
        let start_idx = self.cursor;

        let mut lines = Vec::new();
        for i in 0..rows {
            let offset = start_idx + i * bytes_per_row;
            if offset >= self.platter.data.len() {
                break;
            }

            let mut hex_part = String::new();
            let mut ascii_part = String::new();

            for j in 0..bytes_per_row {
                let idx = offset + j;
                if idx < self.platter.data.len() {
                    let b = self.platter.data[idx];
                    hex_part.push_str(&format!("{:02x} ", b));
                    ascii_part.push(if b.is_ascii_graphic() { b as char } else { '.' });
                } else {
                    hex_part.push_str("   ");
                    ascii_part.push(' ');
                }
            }

            lines.push(Line::from(format!(
                "{:08x}  {}  |{}",
                offset, hex_part, ascii_part
            )));
        }

        let p = Paragraph::new(lines).block(
            Block::default()
                .title("Sector View (Hex)")
                .borders(Borders::ALL),
        );
        f.render_widget(p, area);
    }

    fn render_stats(&self, f: &mut ratatui::Frame, area: Rect) {
        let stats = format!(
            "Temp: {:.5} (Up/Down) | Cursor: 0x{:08x} (Left/Right) | Bytes: {} | Quit: 'q'",
            self.decay_engine.temperature,
            self.cursor,
            self.platter.data.len()
        );
        let p =
            Paragraph::new(stats).block(Block::default().title("Controls").borders(Borders::ALL));
        f.render_widget(p, area);
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Up => self.decay_engine.temperature += 0.0001,
                    KeyCode::Down => {
                        self.decay_engine.temperature =
                            (self.decay_engine.temperature - 0.0001).max(0.0)
                    }
                    KeyCode::Right => {
                        self.cursor =
                            (self.cursor + 16).min(self.platter.data.len().saturating_sub(1))
                    }
                    KeyCode::Left => self.cursor = self.cursor.saturating_sub(16),
                    KeyCode::PageDown => {
                        self.cursor =
                            (self.cursor + 256).min(self.platter.data.len().saturating_sub(1))
                    }
                    KeyCode::PageUp => self.cursor = self.cursor.saturating_sub(256),
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
