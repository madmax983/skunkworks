use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use neuro_sim::Network;
use platter::Platter;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

struct NeuroPlatter {
    network: Network,
    platter: Platter,
    // Store positions of neurons in the grid
    positions: Vec<(usize, usize)>,
}

impl NeuroPlatter {
    fn new(width: usize, height: usize, num_neurons: usize) -> Self {
        let mut network = Network::new();
        let platter = Platter::new(width, height);
        let mut positions = Vec::with_capacity(num_neurons);

        // Distribute neurons evenly in a circle in the grid
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let radius = (width.min(height) as f64 / 2.0) * 0.8;

        for i in 0..num_neurons {
            network.add_neuron();
            let angle = (i as f64 / num_neurons as f64) * std::f64::consts::TAU;
            let x = (cx + radius * angle.cos()) as usize;
            let y = (cy + radius * angle.sin() * 0.5) as usize; // squish Y to account for term char aspect ratio
            positions.push((x, y));

            // Connect in a ring with random varying weights
            let next = (i + 1) % num_neurons;
            network.add_synapse(i, next, 25.0);

            // Connect across for complexity
            if i % 3 == 0 {
                let across = (i + num_neurons / 2) % num_neurons;
                network.add_synapse(i, across, 10.0);
            }
        }

        Self {
            network,
            platter,
            positions,
        }
    }

    fn update(&mut self) {
        // Step the neural network
        let mut inputs = vec![0.0; self.positions.len()];

        // Feed 10.0 continuous current to the 0th neuron to keep the network active
        inputs[0] = 10.0;

        self.network.step(&inputs);

        // Decay the field
        self.platter.decay(0.85);

        // Map spikes to the platter field
        for (i, &(x, y)) in self.positions.iter().enumerate() {
            if self.network.spikes[i] {
                // When a neuron spikes, it "saturates" the field in its local area
                self.platter.saturate(x, y, 1.0);

                // Spread slightly
                if x > 0 {
                    self.platter.saturate(x - 1, y, 0.5);
                }
                if x < self.platter.width() - 1 {
                    self.platter.saturate(x + 1, y, 0.5);
                }
                if y > 0 {
                    self.platter.saturate(x, y - 1, 0.5);
                }
                if y < self.platter.height() - 1 {
                    self.platter.saturate(x, y + 1, 0.5);
                }
            }
        }
    }
}

fn map_value_to_char(val: f64) -> &'static str {
    if val < 0.1 {
        " "
    } else if val < 0.3 {
        "░"
    } else if val < 0.6 {
        "▒"
    } else if val < 0.9 {
        "▓"
    } else {
        "█"
    }
}

fn run_app() -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 80x40 is a reasonable default terminal size.
    let mut brain_scan = NeuroPlatter::new(80, 40, 20);

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let mut lines = Vec::new();
            for y in 0..brain_scan.platter.height() {
                let mut row_spans = Vec::new();
                for x in 0..brain_scan.platter.width() {
                    let val = brain_scan.platter.get(x, y);
                    let symbol = map_value_to_char(val);

                    // Map value to color gradient
                    let color = if val > 0.8 {
                        Color::Red
                    } else if val > 0.5 {
                        Color::Yellow
                    } else if val > 0.2 {
                        Color::Magenta
                    } else {
                        Color::DarkGray
                    };

                    row_spans.push(Span::styled(symbol, Style::default().fg(color)));
                }
                lines.push(Line::from(row_spans));
            }

            let p = Paragraph::new(lines).block(
                Block::default()
                    .title(" Neuro Platter (neuro-platter) ")
                    .borders(Borders::ALL),
            );
            f.render_widget(p, size);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            brain_scan.update();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() {
    if let Err(err) = run_app() {
        eprintln!("Error: {:?}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuro_platter_initialization() {
        let np = NeuroPlatter::new(10, 10, 5);
        assert_eq!(np.platter.width(), 10);
        assert_eq!(np.platter.height(), 10);
        assert_eq!(np.positions.len(), 5);
    }
}
