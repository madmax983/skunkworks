mod sexagesimal;
mod tablet;
mod agent;
mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_shared::Tui;
use simulation::Simulation;
// use sexagesimal::Sexagesimal;

struct App {
    sim: Simulation,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            sim: Simulation::new(60, 30, 100), // 60x30 grid, 100 agents
            exit: false,
        }
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        while !self.exit {
            tui.terminal.draw(|f| self.ui(f))?;
            self.handle_events()?;
            self.sim.step();
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(f.area());

        self.render_tablet(f, chunks[0]);
        self.render_status(f, chunks[1]);
    }

    fn render_tablet(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Chimera Cuneiform Tablet");

        let inner_area = block.inner(area);

        let mut lines = Vec::new();
        // Limit render to area height to prevent panic
        let render_height = (inner_area.height as usize).min(self.sim.tablet.height);
        let render_width = (inner_area.width as usize / 3).min(self.sim.tablet.width); // 3 chars per cell

        for y in 0..render_height {
            let mut spans = Vec::new();
            for x in 0..render_width {
                if let Some(cell) = self.sim.tablet.get(x, y) {
                    let char_str = if cell.value == 0 {
                        " .".to_string()
                    } else {
                        format!("{:02}", cell.value)
                    };

                    let mut style = if cell.is_immutable() {
                        Style::default().fg(Color::DarkGray) // Hardened (Stone)
                    } else if cell.hardness > 0.5 {
                        Style::default().fg(Color::Gray) // Drying
                    } else {
                        Style::default().fg(Color::Yellow) // Wet Clay
                    };

                    // Highlight if an agent is here
                    if self.sim.agents.iter().any(|a| a.pos == (x, y)) {
                        style = style.bg(Color::Blue).fg(Color::White);
                    }

                    spans.push(Span::styled(format!("{} ", char_str), style));
                }
            }
            lines.push(Line::from(spans));
        }

        let p = Paragraph::new(lines).block(block);
        f.render_widget(p, area);
    }

    fn render_status(&self, f: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("Status");
        let cycle_prog = self.sim.ticks % self.sim.flood_cycle;
        let text = format!(
            "Ticks: {} | Saros Cycle: {}/{} | Agents: {} | Survivors: ?",
            self.sim.ticks, cycle_prog, self.sim.flood_cycle, self.sim.agents.len()
        );
        let p = Paragraph::new(text).block(block);
        f.render_widget(p, area);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = app.run(&mut tui);
    tui.exit()?;
    res
}
