use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub mod game;
pub mod graph;

use crate::game::GameState;

struct App {
    game: GameState,
    running: bool,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            game: GameState::new(),
            running: true,
        })
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        let tick_rate = Duration::from_millis(50);
        let mut last_tick = Instant::now();

        while self.running {
            tui.terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.game.tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(f.area());

        let canvas = Canvas::default()
            .block(Block::default().title("Mnem-Chimera: Entomological Code Rot").borders(Borders::ALL))
            .paint(|ctx| {
                // Draw edges
                for edge in &self.game.graph.edges {
                    let from_node = &self.game.graph.nodes[edge.from];
                    let to_node = &self.game.graph.nodes[edge.to];
                    ctx.draw(&CanvasLine {
                        x1: from_node.pos.x,
                        y1: from_node.pos.y,
                        x2: to_node.pos.x,
                        y2: to_node.pos.y,
                        color: Color::DarkGray,
                    });
                }

                // Draw nodes
                for node in &self.game.graph.nodes {
                    let color = if node.entropy > 0.7 {
                        Color::Red
                    } else if node.entropy > 0.4 {
                        Color::Yellow
                    } else {
                        Color::Green
                    };
                    ctx.draw(&Points {
                        coords: &[(node.pos.x, node.pos.y)],
                        color,
                    });
                }

                // Draw agents
                for agent in &self.game.agents {
                    let node = &self.game.graph.nodes[agent.current_node];
                    ctx.print(node.pos.x, node.pos.y, Span::styled("🦠", Style::default().fg(Color::Magenta)));
                }
            })
            .x_bounds([-400.0, 400.0])
            .y_bounds([-300.0, 300.0]);

        f.render_widget(canvas, chunks[0]);

        let info = Paragraph::new(Line::from(vec![
            Span::raw(format!("Nodes: {} | ", self.game.graph.nodes.len())),
            Span::raw(format!("Agents: {} | ", self.game.agents.len())),
            Span::raw(format!("Ticks: {} | ", self.game.ticks)),
            Span::styled("Press 'q' to quit", Style::default().fg(Color::Yellow)),
        ]))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let mut app = App::new()?;
    let res = app.run(&mut tui);

    res
}
