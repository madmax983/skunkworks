use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use neuro_sim::Network;
use rand::{thread_rng, Rng};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

/// 🧬 Splice: Cross `neuro-sim` × `gray-scott`
/// Concept: Neural Morphogenesis.
/// A Spiking Neural Network where the neurons are spatially embedded into a
/// Gray-Scott reaction-diffusion grid.
/// The continuous 'U' chemical feeds into the neurons as input current.
/// When the neurons spike, they inject the discrete 'V' chemical into the grid.
/// This creates a bidirectional feedback loop between continuous spatial morphogenesis
/// and discrete neural spiking.

struct NeuroDiffusionApp {
    gs: GrayScott,
    net: Network,
    neuron_positions: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl NeuroDiffusionApp {
    fn new(width: usize, height: usize) -> Self {
        let mut gs = GrayScott::new(width, height);
        let mut net = Network::new();
        let mut rng = thread_rng();

        // Let's create a sparse grid of neurons over the space
        let spacing = 4;
        let mut neuron_positions = Vec::new();

        for y in (0..height).step_by(spacing) {
            for x in (0..width).step_by(spacing) {
                let _id = net.add_neuron();
                neuron_positions.push((x, y));
            }
        }

        // Add some random connections between neurons
        let num_neurons = neuron_positions.len();
        for i in 0..num_neurons {
            for _ in 0..3 {
                let target = rng.gen_range(0..num_neurons);
                if target != i {
                    // mostly excitatory, some inhibitory
                    let weight = if rng.gen_bool(0.8) { 15.0 } else { -10.0 };
                    let delay = rng.gen_range(1..=10);
                    net.add_synapse_with_delay(i, target, weight, delay);
                }
            }
        }

        // Initial seed of V chemical to get the Gray-Scott going
        gs.add_chemical(width / 2, height / 2, 1.0);

        Self {
            gs,
            net,
            neuron_positions,
            width,
            height,
        }
    }

    fn update(&mut self) {
        let mut inputs = vec![0.0; self.net.neurons.len()];

        // 1. Read U from GrayScott as input current
        for (i, &(x, y)) in self.neuron_positions.iter().enumerate() {
            let idx = self.gs.get_index(x, y);
            let u_concentration = self.gs.u()[idx];
            // Scale U concentration to a decent input current (e.g. 0.0 to 10.0)
            inputs[i] = u_concentration * 10.0;
        }

        // 2. Step Neural Network
        self.net.step(&inputs);

        // 3. Inject V where neurons spiked
        for (i, &(x, y)) in self.neuron_positions.iter().enumerate() {
            if self.net.is_spiking(i) {
                // Large injection of kill chemical when spiking
                self.gs.add_chemical(x, y, 1.0);
            }
        }

        // 4. Step GrayScott
        // Use standard "Spots" or "Mitosis" parameters
        let f = 0.0367; // Mitosis feed
        let k = 0.0649; // Mitosis kill
        let dt = 1.0;
        self.gs.update(f, k, dt);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(16);
    let width = 80;
    let height = 40;
    let mut app = NeuroDiffusionApp::new(width, height);

    let res = run_app(&mut terminal, &mut app, tick_rate);

    // restore terminal
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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut NeuroDiffusionApp,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> where <B as Backend>::Error: Send + Sync + std::error::Error + 'static, {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &NeuroDiffusionApp) {
    let size = f.area();
    let render_width = app.width.min(size.width as usize);
    let render_height = app.height.min(size.height as usize);

    // Create a buffer for the TUI rendering
    let mut grid_display = String::with_capacity(render_width * render_height * 2);

    for y in 0..render_height {
        for x in 0..render_width {
            let idx = app.gs.get_index(x, y);
            let v_val = app.gs.v()[idx];

            // Check if a neuron is here and if it spiked
            let mut spiked = false;
            let mut is_neuron = false;
            if let Some(n_idx) = app.neuron_positions.iter().position(|pos| *pos == (x, y)) {
                is_neuron = true;
                if app.net.is_spiking(n_idx) {
                    spiked = true;
                }
            }

            let ch = if spiked {
                '*'
            } else if is_neuron {
                '.'
            } else {
                let chars = [' ', '░', '▒', '▓', '█'];
                let char_idx = (v_val * 4.99).floor() as usize;
                chars[char_idx.clamp(0, 4)]
            };

            grid_display.push(ch);
        }
        grid_display.push('\n');
    }

    let p = Paragraph::new(grid_display).block(
        Block::default()
            .title(" Neural Morphogenesis (neuro-diffusion) | 'q' to quit ")
            .borders(Borders::ALL),
    );

    f.render_widget(p, size);
}
