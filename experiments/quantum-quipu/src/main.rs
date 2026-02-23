use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use quipu::{Cord, Knot};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod simulation;
use simulation::StateVector;

const NUM_QUBITS: usize = 3;
const CIRCUIT_DEPTH: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq)]
enum QuantumGate {
    H,
    X,
    Z,
    CNOT, // Target is next qubit (wrapping)
    Measure,
    Identity,
}

impl QuantumGate {
    fn from_knot(k: &Knot) -> Self {
        match k {
            Knot::Simple => QuantumGate::H,
            Knot::Long(1) => QuantumGate::X,
            Knot::Long(2) => QuantumGate::Z,
            Knot::FigureEight => QuantumGate::CNOT,
            Knot::Long(9) => QuantumGate::Measure,
            _ => QuantumGate::Identity,
        }
    }

    fn to_knot(&self) -> Knot {
        match self {
            QuantumGate::H => Knot::Simple,
            QuantumGate::X => Knot::Long(1),
            QuantumGate::Z => Knot::Long(2),
            QuantumGate::CNOT => Knot::FigureEight,
            QuantumGate::Measure => Knot::Long(9),
            QuantumGate::Identity => Knot::Long(0),
        }
    }
}

struct AppState {
    cords: Vec<Cord>,
    static_probs: Vec<Vec<f64>>,
    dynamic_state: StateVector,
    playhead_y: f32, // 0.0 (Top) to 1.0 (Bottom)
    speed: f32,
    playing: bool,
    last_update: Instant,
    measurements: Vec<Option<bool>>,
    measured_at_step: Vec<Option<usize>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            cords: Vec::new(),
            static_probs: Vec::new(),
            dynamic_state: StateVector::new(NUM_QUBITS),
            playhead_y: 0.0,
            speed: 0.1,
            playing: true,
            last_update: Instant::now(),
            measurements: vec![None; NUM_QUBITS],
            measured_at_step: vec![None; NUM_QUBITS],
        }
    }

    fn regenerate(&mut self) {
        self.cords.clear();

        for _ in 0..NUM_QUBITS {
            let mut clusters = Vec::new();
            let mut steps = Vec::new();
            for _ in 0..CIRCUIT_DEPTH {
                let r: u8 = rand::random::<u8>() % 20;
                let gate = match r {
                    0..=2 => QuantumGate::H,
                    3..=4 => QuantumGate::X,
                    5 => QuantumGate::Z,
                    6..=8 => QuantumGate::CNOT,
                    9 => QuantumGate::Measure,
                    _ => QuantumGate::Identity,
                };
                steps.push(gate);
            }

            for gate in steps.iter().rev() {
                if *gate == QuantumGate::Identity {
                    clusters.push(Vec::new());
                } else {
                    clusters.push(vec![gate.to_knot()]);
                }
            }

            self.cords.push(Cord {
                clusters,
                subsidiaries: Vec::new(),
                color: quipu::Color::Natural,
            });
        }

        self.recalc_static_probs();
        self.reset_dynamic();
    }

    fn reset_dynamic(&mut self) {
        self.dynamic_state = StateVector::new(NUM_QUBITS);
        self.playhead_y = 0.0;
        self.measurements = vec![None; NUM_QUBITS];
        self.measured_at_step = vec![None; NUM_QUBITS];
    }

    fn recalc_static_probs(&mut self) {
        self.static_probs = vec![Vec::new(); NUM_QUBITS];
        let mut state = StateVector::new(NUM_QUBITS);

        // Initial state (Step 0)
        for q in 0..NUM_QUBITS {
            self.static_probs[q].push(state.get_prob(q));
        }

        for step in 0..CIRCUIT_DEPTH {
            let cord_idx = (CIRCUIT_DEPTH - 1).saturating_sub(step);
            let mut gates_to_apply = Vec::new();

            for q in 0..NUM_QUBITS {
                if let Some(cluster) = self.cords[q].clusters.get(cord_idx) {
                    if let Some(knot) = cluster.first() {
                        gates_to_apply.push((q, QuantumGate::from_knot(knot)));
                    }
                }
            }

            for (q, gate) in &gates_to_apply {
                match gate {
                    QuantumGate::H => state.apply_hadamard(*q),
                    QuantumGate::X => state.apply_x(*q),
                    QuantumGate::Z => state.apply_z(*q),
                    _ => {}
                }
            }

            for (q, gate) in &gates_to_apply {
                if *gate == QuantumGate::CNOT {
                    let target = (q + 1) % NUM_QUBITS;
                    state.apply_cnot(*q, target);
                }
            }

            for q in 0..NUM_QUBITS {
                self.static_probs[q].push(state.get_prob(q));
            }
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut state = AppState::new();
    state.regenerate();

    let mut last_processed_step: isize = -1;

    loop {
        if state.playing {
            let now = Instant::now();
            let dt = now.duration_since(state.last_update).as_secs_f32();
            state.last_update = now;

            state.playhead_y += state.speed * dt;
            if state.playhead_y >= 1.0 {
                state.playhead_y = 0.0;
                state.reset_dynamic();
                last_processed_step = -1;
            }

            let total_steps = CIRCUIT_DEPTH as f32;
            let current_step = (state.playhead_y * total_steps).floor() as isize;

            if current_step > last_processed_step {
                let step = current_step as usize;
                if step < CIRCUIT_DEPTH {
                    let cord_idx = (CIRCUIT_DEPTH - 1).saturating_sub(step);
                    let mut gates_to_apply = Vec::new();

                    for q in 0..NUM_QUBITS {
                        if let Some(cluster) = state.cords[q].clusters.get(cord_idx) {
                            if let Some(knot) = cluster.first() {
                                gates_to_apply.push((q, QuantumGate::from_knot(knot)));
                            }
                        }
                    }

                    for (q, gate) in &gates_to_apply {
                        match gate {
                            QuantumGate::H => state.dynamic_state.apply_hadamard(*q),
                            QuantumGate::X => state.dynamic_state.apply_x(*q),
                            QuantumGate::Z => state.dynamic_state.apply_z(*q),
                            QuantumGate::Measure => {
                                let res = state.dynamic_state.measure(*q);
                                state.measurements[*q] = Some(res);
                                state.measured_at_step[*q] = Some(step);
                            }
                            _ => {}
                        }
                    }

                    for (q, gate) in &gates_to_apply {
                        if *gate == QuantumGate::CNOT {
                            let target = (q + 1) % NUM_QUBITS;
                            state.dynamic_state.apply_cnot(*q, target);
                        }
                    }
                }
                last_processed_step = current_step;
            }
        } else {
            state.last_update = Instant::now();
        }

        tui.terminal.draw(|f| ui(f, &state))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => state.playing = !state.playing,
                    KeyCode::Char('r') => {
                        state.regenerate();
                        state.playhead_y = 0.0;
                        state.reset_dynamic();
                        last_processed_step = -1;
                    }
                    KeyCode::Up => state.speed += 0.05,
                    KeyCode::Down => state.speed = (state.speed - 0.05).max(0.01),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Quantum Quipu: Entangled Knots"),
        )
        .x_bounds([0.0, NUM_QUBITS as f64])
        .y_bounds([0.0, (CIRCUIT_DEPTH + 1) as f64])
        .paint(|ctx| {
            // Draw Playhead
            // playhead_y (0..1) -> Canvas Y (Depth+1 .. 0)
            // 0 -> Top (Depth+1). 1 -> Bottom (0).
            let ph_canvas_y = (1.0 - state.playhead_y) * ((CIRCUIT_DEPTH + 1) as f32);
            ctx.draw(&Line {
                x1: 0.0,
                y1: ph_canvas_y as f64,
                x2: NUM_QUBITS as f64,
                y2: ph_canvas_y as f64,
                color: Color::White,
            });

            for q in 0..NUM_QUBITS {
                let x_center = q as f64 + 0.5;

                // Draw Segments 0 to DEPTH
                for step in 0..=CIRCUIT_DEPTH {
                    // Step 0 (Top) -> Canvas Y: Depth+1 to Depth
                    // Step i -> Canvas Y: (Depth+1 - i) to (Depth - i)
                    let y_start = (CIRCUIT_DEPTH + 1 - step) as f64;
                    let y_end = (CIRCUIT_DEPTH - step) as f64;

                    let prob = if let Some(m_val) = state.measurements[q] {
                        if let Some(m_step) = state.measured_at_step[q] {
                            if step > m_step {
                                if m_val {
                                    1.0
                                } else {
                                    0.0
                                }
                            } else {
                                // Safely access static_probs
                                *state.static_probs[q].get(step).unwrap_or(&0.0)
                            }
                        } else {
                            *state.static_probs[q].get(step).unwrap_or(&0.0)
                        }
                    } else {
                        *state.static_probs[q].get(step).unwrap_or(&0.0)
                    };

                    let color = if prob < 0.1 {
                        Color::Blue
                    } else if prob > 0.9 {
                        Color::Red
                    } else {
                        Color::Magenta
                    };

                    ctx.draw(&Line {
                        x1: x_center,
                        y1: y_start,
                        x2: x_center,
                        y2: y_end,
                        color,
                    });

                    // Draw Gate at bottom of segment (except last segment)
                    if step < CIRCUIT_DEPTH {
                        let cord_idx = (CIRCUIT_DEPTH - 1).saturating_sub(step);
                        if let Some(cluster) = state.cords[q].clusters.get(cord_idx) {
                            if let Some(knot) = cluster.first() {
                                let gate = QuantumGate::from_knot(knot);
                                let symbol_color = Color::Yellow;

                                // Draw symbol
                                ctx.draw(&Rectangle {
                                    x: x_center - 0.1,
                                    y: y_end + 0.2, // Near top of NEXT segment / Bottom of current
                                    width: 0.2,
                                    height: 0.2,
                                    color: symbol_color,
                                });

                                if gate == QuantumGate::CNOT {
                                    let target = (q + 1) % NUM_QUBITS;
                                    let tx_center = target as f64 + 0.5;
                                    // Link to target
                                    ctx.draw(&Line {
                                        x1: x_center,
                                        y1: y_end + 0.3,
                                        x2: tx_center,
                                        y2: y_end + 0.3,
                                        color: Color::Green,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Info
    let probs_text: String = state
        .static_probs
        .iter()
        .enumerate()
        .map(|(i, p)| format!("Q{}: {:.2}", i, p.last().unwrap_or(&0.0)))
        .collect::<Vec<_>>()
        .join(" | ");

    let info_text = format!(
        "Playhead: {:.2} | Measurements: {:?} | Final Probs: {}",
        state.playhead_y,
        state
            .measurements
            .iter()
            .map(|m| m.map(|b| if b { 1 } else { 0 }))
            .collect::<Vec<_>>(),
        probs_text
    );
    let info = Paragraph::new(info_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}
