use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Rectangle},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod garden;
mod quantum;

use garden::Garden;
use quantum::Gate;

struct App {
    garden: Garden,
    control_index: Option<usize>, // If Some, we are selecting target for CNOT
    status_msg: String,
}

impl App {
    fn new() -> Self {
        Self {
            garden: Garden::new(5),
            control_index: None,
            status_msg: "Welcome to the Quantum Garden. Arrow Keys to select, H/X/Z to tend."
                .into(),
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                if self.control_index.is_some() {
                                    self.control_index = None;
                                    self.status_msg = "Cancelled Entanglement.".into();
                                } else {
                                    return Ok(());
                                }
                            }
                            KeyCode::Left => {
                                if self.garden.selected_index > 0 {
                                    self.garden.selected_index -= 1;
                                }
                            }
                            KeyCode::Right => {
                                if self.garden.selected_index < self.garden.system.num_qubits - 1 {
                                    self.garden.selected_index += 1;
                                }
                            }
                            KeyCode::Char('h') => {
                                self.garden.apply_gate(Gate::H)?;
                                self.status_msg = format!(
                                    "Watered (Hadamard) Plant {}",
                                    self.garden.selected_index
                                );
                            }
                            KeyCode::Char('x') => {
                                self.garden.apply_gate(Gate::X)?;
                                self.status_msg = format!(
                                    "Fertilized (Pauli-X) Plant {}",
                                    self.garden.selected_index
                                );
                            }
                            KeyCode::Char('z') => {
                                self.garden.apply_gate(Gate::Z)?;
                                self.status_msg = format!(
                                    "Pruned (Pauli-Z) Plant {}",
                                    self.garden.selected_index
                                );
                            }
                            KeyCode::Char('c') => {
                                if self.control_index.is_none() {
                                    self.control_index = Some(self.garden.selected_index);
                                    self.status_msg = format!(
                                        "Plant {} selected as Control. Select Target and press Enter.",
                                        self.garden.selected_index
                                    );
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(control) = self.control_index {
                                    let target = self.garden.selected_index;
                                    if control == target {
                                        self.status_msg =
                                            "Self-entanglement impossible! Choose another target."
                                                .into();
                                    } else {
                                        self.garden.system.apply_cnot(control, target)?;

                                        // Update visuals
                                        self.garden.plants[control].is_entangled = true;
                                        self.garden.plants[target].is_entangled = true;
                                        self.garden.plants[control].entangled_with.push(target);
                                        self.garden.plants[target].entangled_with.push(control);
                                        self.garden.update();

                                        self.status_msg =
                                            format!("Entangled {} with {}", control, target);
                                        self.control_index = None;
                                    }
                                }
                            }
                            KeyCode::Char('m') => {
                                self.garden.measure();
                                self.status_msg = "Harvested (Measured) the Garden.".into();
                            }
                            KeyCode::Char('r') => {
                                self.garden = Garden::new(5);
                                self.control_index = None;
                                self.status_msg = "Garden Reset.".into();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(12),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Header
        let title = Paragraph::new(" QUANTUM GARDEN ⚛ ")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Garden Canvas
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(" The Garden "))
            .x_bounds([0.0, (self.garden.system.num_qubits as f64) * 20.0])
            .y_bounds([0.0, 100.0])
            .paint(|ctx| {
                for (i, plant) in self.garden.plants.iter().enumerate() {
                    let x = (i as f64) * 20.0 + 10.0;
                    let h = plant.height * 80.0; // Max height 80

                    // Draw stem
                    ctx.draw(&CanvasLine {
                        x1: x,
                        y1: 0.0,
                        x2: x,
                        y2: h,
                        color: Color::Green,
                    });

                    // Draw flower head
                    let bloom_size = 2.0 + plant.height * 5.0;
                    ctx.draw(&Rectangle {
                        x: x - bloom_size,
                        y: h,
                        width: bloom_size * 2.0,
                        height: bloom_size * 2.0,
                        color: plant.phase_color,
                    });

                    // Draw selection marker
                    if i == self.garden.selected_index {
                        ctx.draw(&Rectangle {
                            x: x - 5.0,
                            y: -5.0,
                            width: 10.0,
                            height: 5.0,
                            color: Color::Yellow,
                        });
                    }

                    // Draw Control marker if active
                    if let Some(c) = self.control_index {
                        if i == c {
                            ctx.draw(&Rectangle {
                                x: x - 3.0,
                                y: -3.0,
                                width: 6.0,
                                height: 6.0,
                                color: Color::Red,
                            });
                        }
                    }

                    // Draw entanglement vines
                    for &target in &plant.entangled_with {
                        if target > i {
                            // Draw only once per pair
                            let tx = (target as f64) * 20.0 + 10.0;
                            // Arc
                            let mid_x = (x + tx) / 2.0;
                            let mid_y = 50.0;

                            ctx.draw(&CanvasLine {
                                x1: x,
                                y1: 10.0,
                                x2: mid_x,
                                y2: mid_y,
                                color: Color::Magenta,
                            });
                            ctx.draw(&CanvasLine {
                                x1: mid_x,
                                y1: mid_y,
                                x2: tx,
                                y2: 10.0,
                                color: Color::Magenta,
                            });
                        }
                    }
                }
            });
        f.render_widget(canvas, chunks[1]);

        // State Vector Panel
        let mut state_lines = Vec::new();
        state_lines.push(Line::from("System State Vector (|psi>):"));

        let mut count = 0;
        for (i, amp) in self.garden.system.state.iter().enumerate() {
            let mag = amp.norm_sqr();
            if mag > 0.001 {
                if count > 8 {
                    state_lines.push(Line::from("..."));
                    break;
                }
                // Convert index to binary string representation
                let binary = format!("{:0width$b}", i, width = self.garden.system.num_qubits);
                let s = format!("|{}> : {:.3} (prob: {:.3})", binary, amp, mag);
                state_lines.push(Line::from(s));
                count += 1;
            }
        }

        let state_block = Paragraph::new(state_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hilbert Space "),
        );
        f.render_widget(state_block, chunks[2]);

        // Status / Help
        let status = Paragraph::new(format!("{} | [Arrow] Move | [H] Water | [X] Feed | [Z] Prune | [C] Entangle | [M] Harvest | [R] Reset", self.status_msg))
             .style(Style::default().fg(Color::White))
             .block(Block::default().borders(Borders::ALL).title(" Status "));
        f.render_widget(status, chunks[3]);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
