mod creature;
mod ik;
mod silk;
mod world;

use crossterm::event::{self, Event, KeyCode};
use glam::Vec2;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line as TextLine, Span},
    widgets::{
        canvas::{Canvas, Circle, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::io;
use std::time::{Duration, Instant};
use tui_shared::Tui;

use creature::Creature;
use world::World;

struct App {
    world: World,
    creature: Creature,
    silk: silk::Silk,
}

impl App {
    fn new() -> Self {
        let world = World::new();
        let start_pos = Vec2::ZERO;
        Self {
            world,
            creature: Creature::new(start_pos),
            silk: silk::Silk::new(),
        }
    }

    fn on_tick(&mut self, dt: f32) {
        if !self.world.nodes.is_empty() {
            if self.world.selected_index >= self.world.nodes.len() {
                self.world.selected_index = 0;
            }
            // Target is slightly offset so we don't sit *on* the text
            let target_node = &self.world.nodes[self.world.selected_index];
            self.creature.target_pos = target_node.position;
        }

        self.creature.update(&self.world, dt);
        self.silk.add_strand(self.creature.body_pos);
        self.silk.update(dt);
    }
}

fn main() -> io::Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Up | KeyCode::Char('k') => {
                        if app.world.selected_index > 0 {
                            app.world.selected_index -= 1;
                        } else {
                            app.world.selected_index = app.world.nodes.len().saturating_sub(1);
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Down | KeyCode::Char('j') => {
                        if app.world.selected_index < app.world.nodes.len().saturating_sub(1) {
                            app.world.selected_index += 1;
                        } else {
                            app.world.selected_index = 0;
                        }
                    }
                    KeyCode::Enter => {
                        if !app.world.nodes.is_empty() {
                            let node = &app.world.nodes[app.world.selected_index];
                            if node.is_dir {
                                let path = node.path.clone();
                                app.world.scan(&path);
                                // Teleport creature for now, or let it walk?
                                // Teleporting is less confusing for a complete context switch
                                app.creature.body_pos = Vec2::ZERO;
                                app.creature.target_pos = Vec2::ZERO;
                                app.silk.clear();
                                // Reset legs
                                app.creature = Creature::new(Vec2::ZERO);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(parent) = app.world.current_path.parent() {
                            let p = parent.to_path_buf();
                            if p.exists() {
                                app.world.scan(&p);
                                app.silk.clear();
                                app.creature = Creature::new(Vec2::ZERO);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32();
        last_tick = now;

        app.on_tick(dt);
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    // Camera window
    let cam_x = app.creature.body_pos.x as f64;
    let cam_y = app.creature.body_pos.y as f64;
    let view_width = 100.0;
    let view_height = 80.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Code Crawler 🕷️"),
        )
        .x_bounds([cam_x - view_width / 2.0, cam_x + view_width / 2.0])
        .y_bounds([cam_y - view_height / 2.0, cam_y + view_height / 2.0])
        .paint(|ctx| {
            // Draw Silk
            for strand in &app.silk.strands {
                ctx.draw(&Line {
                    x1: strand.start.x as f64,
                    y1: strand.start.y as f64,
                    x2: strand.end.x as f64,
                    y2: strand.end.y as f64,
                    color: Color::DarkGray,
                });
            }

            // Draw Nodes
            for (i, node) in app.world.nodes.iter().enumerate() {
                let color = if i == app.world.selected_index {
                    Color::Yellow
                } else if node.is_dir {
                    Color::Blue
                } else {
                    Color::Green
                };

                // Draw node circle
                ctx.draw(&Circle {
                    x: node.position.x as f64,
                    y: node.position.y as f64,
                    radius: node.size as f64,
                    color,
                });

                // Label (simplified, only selected or nearby)
                if i == app.world.selected_index {
                    ctx.print(
                        node.position.x as f64,
                        node.position.y as f64 + node.size as f64 + 2.0,
                        Span::styled(node.name.clone(), Style::default().fg(Color::Yellow)),
                    );
                } else if node.position.distance(app.creature.body_pos) < 30.0 {
                    // Only show nearby labels to reduce clutter
                    ctx.print(
                        node.position.x as f64,
                        node.position.y as f64 + node.size as f64 + 2.0,
                        Span::styled(node.name.clone(), Style::default().fg(Color::DarkGray)),
                    );
                }
            }

            // Draw Creature
            let body = &app.creature;

            // Draw Body
            ctx.draw(&Circle {
                x: body.body_pos.x as f64,
                y: body.body_pos.y as f64,
                radius: 4.0,
                color: Color::Red,
            });

            // Draw Legs
            for leg in &body.legs {
                // Joints
                let mut prev = body.body_pos + leg.offset;
                for joint in &leg.chain.joints {
                    ctx.draw(&Line {
                        x1: prev.x as f64,
                        y1: prev.y as f64,
                        x2: joint.x as f64,
                        y2: joint.y as f64,
                        color: Color::Magenta,
                    });
                    prev = *joint;
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Footer
    let current_node_name = if !app.world.nodes.is_empty() {
        &app.world.nodes[app.world.selected_index].name
    } else {
        "Empty"
    };

    let info = TextLine::from(vec![
        Span::raw(" Path: "),
        Span::styled(
            app.world.current_path.to_string_lossy(),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(" | Selected: "),
        Span::styled(current_node_name, Style::default().fg(Color::Yellow)),
        Span::raw(" | Controls: Arrows/Vim to move, Enter to open, Backspace to go up, Q to quit"),
    ]);

    f.render_widget(Paragraph::new(info), chunks[1]);
}
