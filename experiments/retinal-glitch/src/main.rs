use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::Duration;
use tui_shared::Tui;

pub mod input;
pub mod neuron;
pub mod retina;

use input::InputGenerator;
use retina::Retina;

struct App {
    retina: Retina,
    input_gen: InputGenerator,
    running: bool,
    width: usize,
    height: usize,
    steps_per_frame: usize,
    glitch_mode: bool,

    // Simulation buffers
    input_buffer: Vec<f32>,

    // Visualization buffers
    display_input_points: Vec<(f64, f64)>,
    display_spike_points: Vec<(f64, f64)>,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        Self {
            retina: Retina::new(width, height),
            input_gen: InputGenerator::new(width, height),
            running: true,
            width,
            height,
            steps_per_frame: 10,
            glitch_mode: false,
            input_buffer: vec![0.0; width * height],
            display_input_points: Vec::new(),
            display_spike_points: Vec::new(),
        }
    }

    fn update(&mut self) {
        self.display_spike_points.clear();
        self.display_input_points.clear();

        // Run physics multiple times per frame
        for step in 0..self.steps_per_frame {
            self.input_gen.update(1.0, &mut self.input_buffer); // dt=1.0ms

            // If glitch mode is on, randomize neuron parameters randomly
            if self.glitch_mode {
                self.apply_glitch();
            }

            self.retina.update(&self.input_buffer);

            // Collect data for visualization
            // For input: only capture on the last step to save perf
            if step == self.steps_per_frame - 1 {
                for (i, &val) in self.input_buffer.iter().enumerate() {
                    if val > 5.0 { // Threshold for visualization
                        let y = (i / self.width) as f64;
                        let x = (i % self.width) as f64;
                        self.display_input_points.push((x, (self.height as f64 - 1.0) - y));
                    }
                }
            }

            // For spikes: accumulate ALL spikes
            for (i, neuron) in self.retina.neurons.iter().enumerate() {
                if neuron.spiked {
                    let y = (i / self.width) as f64;
                    let x = (i % self.width) as f64;
                    self.display_spike_points.push((x, (self.height as f64 - 1.0) - y));
                }
            }
        }
    }

    fn apply_glitch(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        // Pick a random neuron and mess it up
        let idx = rng.gen_range(0..self.retina.neurons.len());
        if rng.gen_bool(0.1) {
            self.retina.neurons[idx].a = rng.gen_range(0.02..0.1);
            self.retina.neurons[idx].d = rng.gen_range(2.0..10.0);
        }
        // Occasional reset
        if rng.gen_bool(0.001) {
             self.retina.neurons[idx] = neuron::Neuron::regular_spiking();
        }
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                KeyCode::Char(' ') => self.input_gen.next_pattern(),
                KeyCode::Up => self.retina.inhibition_weight += 2.0,
                KeyCode::Down => self.retina.inhibition_weight -= 2.0,
                KeyCode::Char('g') => self.glitch_mode = !self.glitch_mode,
                _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // 64x32 fits nicely in most terminals
    let mut app = App::new(64, 32);

    while app.running {
        if event::poll(Duration::from_millis(16))? {
            app.handle_event(event::read()?);
        }

        app.update();

        tui.terminal.draw(|f| ui(f, &app))?;
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Draw Input
    let input_canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Input Signal (Retina Layer)"))
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, app.height as f64])
        .marker(ratatui::symbols::Marker::Block)
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &app.display_input_points,
                color: Color::Cyan,
            });
        });

    f.render_widget(input_canvas, main_chunks[0]);

    // Draw Output (Spikes)
    let output_canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Visual Cortex (Spikes)"))
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, app.height as f64])
        .marker(ratatui::symbols::Marker::Block)
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &app.display_spike_points,
                color: Color::Red, // Spikes are red!
            });
        });

    f.render_widget(output_canvas, main_chunks[1]);

    // Status Bar
    let status_text = format!(
        "Pattern: {:?} | Inhibition: {:.1} | Glitch: {} | Press <SPACE> Pattern | <UP/DOWN> Inhibition | <G> Glitch | <Q> Quit",
        app.input_gen.pattern,
        app.retina.inhibition_weight,
        if app.glitch_mode { "ON" } else { "OFF" }
    );

    let info = Paragraph::new(Span::styled(status_text, Style::default().fg(Color::Yellow)))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}
