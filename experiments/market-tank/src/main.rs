use anyhow::Result;
use crossbeam_channel::bounded;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use market_sim::{Grid, Particle};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::{io, thread, time::Duration};

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;

struct App {
    market: Grid,
    cmd_tx: crossbeam_channel::Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<resonance_audio::audio::AudioSnapshot>,
    current_pressure: Vec<f32>,
    status_msg: String,
    next_id: usize,
}

impl App {
    fn new() -> Self {
        let (cmd_tx, cmd_rx) = bounded(1024);
        let (snap_tx, snap_rx) = bounded(2);

        // Run wave tank audio simulation in background thread
        thread::spawn(move || {
            let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);
            let mut dummy_buffer = vec![0.0; 1024];
            loop {
                model.process(&mut dummy_buffer);
                thread::sleep(Duration::from_millis(16));
            }
        });

        Self {
            market: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            cmd_tx,
            snap_rx,
            current_pressure: vec![0.0; GRID_WIDTH * GRID_HEIGHT],
            status_msg: "Acoustic Liquidity. Trades inject sound. Sound moves orders.".into(),
            next_id: 1,
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut rng = rand::thread_rng();

        loop {
            // Drain newest pressure snapshot
            while let Ok(snap) = self.snap_rx.try_recv() {
                self.current_pressure = snap.pressure;
            }

            // Periodically inject new Bids (buyers at bottom) and Asks (sellers at top)
            if rng.gen_bool(0.3) {
                let x = rng.gen_range(0..GRID_WIDTH);
                // Bids start at bottom (high Y index -> low price)
                if self.market.get(x, GRID_HEIGHT - 1) == Particle::Empty {
                    self.market
                        .set(x, GRID_HEIGHT - 1, Particle::Bid(self.next_id));
                    self.next_id += 1;
                }
            }
            if rng.gen_bool(0.3) {
                let x = rng.gen_range(0..GRID_WIDTH);
                // Asks start at top (low Y index -> high price)
                if self.market.get(x, 0) == Particle::Empty {
                    self.market.set(x, 0, Particle::Ask(self.next_id));
                    self.next_id += 1;
                }
            }

            // Step the market simulation
            let trades = self.market.update();

            // When a trade happens, it Plucks the wave tank at the execution location
            for trade in trades {
                // Determine collision Y coordinate from the trade price
                // price = GRID_HEIGHT - 1 - Y => Y = GRID_HEIGHT - 1 - price
                let y = (GRID_HEIGHT as f32 - 1.0 - trade.price) as usize;
                // Find approximate x by scanning for the Trade particle (the market doesn't emit X natively)
                let mut trade_x = GRID_WIDTH / 2;
                for x in 0..GRID_WIDTH {
                    if let Particle::Trade { .. } = self.market.get(x, y) {
                        trade_x = x;
                        break;
                    }
                }

                // Send acoustic pluck!
                let _ = self.cmd_tx.send(AudioCommand::Pluck {
                    x: trade_x,
                    y,
                    strength: 1.5,
                });
            }

            // Apply Acoustic Volatility: The physical pressure waves push the Bids and Asks laterally
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    let idx = y * GRID_WIDTH + x;
                    let pressure = self.current_pressure[idx];

                    if pressure.abs() > 0.5 && rng.gen_bool(0.1) {
                        let particle = self.market.get(x, y);
                        match particle {
                            Particle::Bid(_) | Particle::Ask(_) => {
                                // Advect left or right based on pressure gradient
                                let move_dir = if pressure > 0.0 { 1 } else { -1 };
                                let nx = x as isize + move_dir;
                                if nx >= 0 && nx < GRID_WIDTH as isize {
                                    let nx = nx as usize;
                                    if self.market.get(nx, y) == Particle::Empty {
                                        self.market.set(nx, y, particle);
                                        self.market.set(x, y, Particle::Empty);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            terminal
                .draw(|f| self.ui(f))
                .map_err(|e| io::Error::other(e.to_string()))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('c') => {
                                let _ = self.cmd_tx.send(AudioCommand::ClearWaves);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let size = f.area();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                " 📈 Market-Tank: Acoustic Liquidity ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().fg(Color::White).bg(Color::Black));
        let inner_area = block.inner(size);
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(inner_area);

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Acoustic Order Book "),
            )
            .marker(ratatui::symbols::Marker::Block)
            .x_bounds([0.0, GRID_WIDTH as f64])
            .y_bounds([0.0, GRID_HEIGHT as f64])
            .paint(|ctx| {
                let mut bid_points = Vec::new();
                let mut ask_points = Vec::new();
                let mut trade_points = Vec::new();
                let mut pos_wave_points = Vec::new();
                let mut neg_wave_points = Vec::new();

                for y in 0..GRID_HEIGHT {
                    for x in 0..GRID_WIDTH {
                        let y_flipped = (GRID_HEIGHT - 1 - y) as f64;
                        let x_f64 = x as f64;

                        // Render wave tank pressure background
                        let pressure = self.current_pressure[y * GRID_WIDTH + x];
                        if pressure > 0.2 {
                            pos_wave_points.push((x_f64, y_flipped));
                        } else if pressure < -0.2 {
                            neg_wave_points.push((x_f64, y_flipped));
                        }

                        // Render market particles foreground
                        match self.market.get(x, y) {
                            Particle::Bid(_) => bid_points.push((x_f64, y_flipped)),
                            Particle::Ask(_) => ask_points.push((x_f64, y_flipped)),
                            Particle::Trade { .. } => trade_points.push((x_f64, y_flipped)),
                            _ => {}
                        }
                    }
                }

                ctx.draw(&Points {
                    coords: &pos_wave_points,
                    color: Color::DarkGray,
                });
                ctx.draw(&Points {
                    coords: &neg_wave_points,
                    color: Color::Gray,
                });

                ctx.draw(&Points {
                    coords: &bid_points,
                    color: Color::Green,
                });
                ctx.draw(&Points {
                    coords: &ask_points,
                    color: Color::Red,
                });
                ctx.draw(&Points {
                    coords: &trade_points,
                    color: Color::Yellow,
                });
            });
        f.render_widget(canvas, chunks[0]);

        let status = Paragraph::new(format!(
            "{} | Bids: {} | Asks: {} | Trades Last Tick: {} | CoM Price: {:.2} | [Q/Esc] Quit",
            self.status_msg,
            self.market.total_bids,
            self.market.total_asks,
            self.market.trade_count,
            self.market.center_of_mass
        ))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

        f.render_widget(status, chunks[1]);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
