use std::time::Duration;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Circle, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use tui_shared::Tui;
use rand::Rng;

pub mod simulation;
use simulation::{Node, NodeState, Ring, World};

fn main() -> Result<()> {
    // Initialize TUI
    let mut tui = Tui::init()?;

    // Initialize World
    let width = 200.0;
    let height = 200.0;
    let mut world = World::new(width, height);

    // Seed nodes
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let mut node = Node::new(
            rng.gen_range(10.0..width-10.0),
            rng.gen_range(10.0..height-10.0),
        );
        node.reset_timer(&mut rng);
        world.nodes.push(node);
    }

    // Start a few leaders manually to kickstart (optional, but good for demo)
    if let Some(node) = world.nodes.first_mut() {
        node.state = NodeState::Leader;
        world.rings.push(Ring::new(node.x, node.y));
    }

    loop {
        tui.terminal.draw(|f| {
            ui(f, &world);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        world.update(&mut rng);
    }

    Ok(())
}

fn ui(frame: &mut Frame, world: &World) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(frame.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Fairy Consensus: Fungal Raft"))
        .x_bounds([0.0, world.width])
        .y_bounds([0.0, world.height])
        .paint(|ctx| {
            // Draw Rings
            for ring in &world.rings {
                ctx.draw(&Circle {
                    x: ring.x,
                    y: ring.y,
                    radius: ring.radius,
                    color: Color::Magenta,
                });
            }

            // Draw Nodes
            for node in &world.nodes {
                let color = match node.state {
                    NodeState::Follower => Color::Gray,
                    NodeState::Candidate => Color::Yellow,
                    NodeState::Leader => Color::Red,
                    NodeState::Committed => Color::Green,
                };

                ctx.draw(&Rectangle {
                    x: node.x - 2.0,
                    y: node.y - 2.0,
                    width: 4.0,
                    height: 4.0,
                    color,
                });
            }
        });

    frame.render_widget(canvas, chunks[0]);

    let footer = Paragraph::new("Q: Quit")
        .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(footer, chunks[1]);
}
