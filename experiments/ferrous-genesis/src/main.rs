use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod physics;
mod platter;

use physics::{Body, Universe};

fn create_homeostasis_dna() -> Dna {
    // Strand 0: Start & Check Hot
    let s0 = Strand {
        genes: vec![
            // Stack: [Local, Self] (Top: Self) -> Wait, scan pushes [Local, Self]
            // Let's assume input is pushed as: stack.push(Local); stack.push(Self);
            // So Top is Self. Bottom is Local.

            // 1. Check LocalMag (Under Self)
            Gene {
                op: OpCode::Swap,
                args: vec![],
            }, // [Self, Local]
            Gene {
                op: OpCode::Dup,
                args: vec![],
            }, // [Self, Local, Local]
            // 2. Is Local > 60?
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(60)],
            }, // [Self, Local, Local, 60]
            Gene {
                op: OpCode::Gt,
                args: vec![],
            }, // [Self, Local, IsHot]
            // 3. If NOT Hot (0), Jump to Strand 1 (Check Cold)
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(1)],
            },
            // 4. If Hot (1), We are here. Cool down (20).
            Gene {
                op: OpCode::Drop,
                args: vec![],
            }, // [Self, Local] -> [Self]
            Gene {
                op: OpCode::Drop,
                args: vec![],
            }, // [Self] -> []
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Ret,
                args: vec![],
            },
        ],
    };

    // Strand 1: Check Cold
    let s1 = Strand {
        genes: vec![
            // Stack: [Self, Local]
            Gene {
                op: OpCode::Dup,
                args: vec![],
            }, // [Self, Local, Local]
            // Is Local < 40?
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(40)],
            },
            Gene {
                op: OpCode::Lt,
                args: vec![],
            }, // [Self, Local, IsCold]
            // If NOT Cold (0), Jump to Strand 2 (Stay)
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(2)],
            },
            // If Cold (1), Heat up (80).
            Gene {
                op: OpCode::Drop,
                args: vec![],
            },
            Gene {
                op: OpCode::Drop,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(80)],
            },
            Gene {
                op: OpCode::Ret,
                args: vec![],
            },
        ],
    };

    // Strand 2: Stay
    let s2 = Strand {
        genes: vec![
            // Stack: [Self, Local]
            Gene {
                op: OpCode::Drop,
                args: vec![],
            }, // [Self]
            Gene {
                op: OpCode::Ret,
                args: vec![],
            },
        ],
    };

    Dna {
        helix: Helix {
            strands: vec![s0, s1, s2],
        },
        evolution_config: None,
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    let mut tui = Tui::init()?;

    // Run Logic
    let res = run_app(&mut tui);

    // Restore Terminal
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    // Initialize Universe
    let mut universe = Universe::new();
    let dna = create_homeostasis_dna();

    // Spawn 50 Particles
    for _ in 0..50 {
        let x = (rand::random::<f64>() - 0.5) * 100.0;
        let y = (rand::random::<f64>() - 0.5) * 100.0;
        let mass = 10.0 + rand::random::<f64>() * 20.0;
        let radius = mass.sqrt();
        let color = Color::Cyan; // Initial color

        let body = Body::new(x, y, mass, radius, color, dna.clone());
        universe.add_body(body);
    }

    let mut zoom = 1.0;
    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();
    let mut paused = false;

    loop {
        tui.terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(size);

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Ferrous Genesis: Amorphous CA"),
                )
                .x_bounds([-100.0 * zoom, 100.0 * zoom])
                .y_bounds([-100.0 * zoom, 100.0 * zoom])
                .paint(|ctx| {
                    // Render Platter Heatmap (Sparsely)
                    let step = 4;
                    for y in (0..universe.platter.height).step_by(step) {
                        for x in (0..universe.platter.width).step_by(step) {
                            let mag = universe.platter.get_magnetism(x, y);
                            if mag > 0.1 {
                                let px = (x as f64) - 100.0;
                                let py = (y as f64) - 100.0;

                                let color = match mag {
                                    m if m > 0.8 => Color::Red,
                                    m if m > 0.5 => Color::Magenta,
                                    m if m > 0.2 => Color::Blue,
                                    _ => Color::DarkGray,
                                };
                                ctx.print(px, py, Span::styled("·", Style::default().fg(color)));
                            }
                        }
                    }

                    // Render Bodies
                    for body in &universe.bodies {
                        // Trail
                        for i in 0..body.trail.len().saturating_sub(1) {
                            ctx.draw(&CanvasLine {
                                x1: body.trail[i].x,
                                y1: body.trail[i].y,
                                x2: body.trail[i + 1].x,
                                y2: body.trail[i + 1].y,
                                color: Color::DarkGray,
                            });
                        }

                        // Body
                        let symbol = if body.mass > 20.0 { "O" } else { "o" };
                        // Color is updated in physics step based on magnetism
                        ctx.print(
                            body.pos.x,
                            body.pos.y,
                            Span::styled(symbol, Style::default().fg(body.color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let status = if paused { "PAUSED" } else { "RUNNING" };
            let footer = Paragraph::new(format!(
                "Controls: [Q] Quit | [Space] Pause | [+/-] Zoom | Status: {}",
                status
            ))
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));
            f.render_widget(footer, chunks[1]);
        })?;

        // Input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => paused = !paused,
                        KeyCode::Char('+') => zoom *= 0.9,
                        KeyCode::Char('-') => zoom *= 1.1,
                        _ => {}
                    }
                }
            }
        }

        if !paused && last_tick.elapsed() >= tick_rate {
            universe.step(0.1);
            last_tick = Instant::now();
        }
    }
}
