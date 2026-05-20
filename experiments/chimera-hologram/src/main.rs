use anyhow::Result;
use chimera_lang::{
    prologue_compiler,
    vm::{ChimeraVM, Value},
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph, Row, Table,
    },
    Terminal,
};
use std::io::Read;
use std::{fs, io, path::Path, time::Duration};

mod fft;
use fft::Hologram;

struct App {
    vm: ChimeraVM,
    status_msg: String,
}

impl App {
    fn new(input_path: &Path) -> Result<Self> {
        let file = fs::File::open(input_path)?;
        let mut unparsed_file = String::new();
        let limit = 1024 * 1024; // 1MB limit
        let bytes_read = file.take(limit + 1).read_to_string(&mut unparsed_file)?;

        if bytes_read as u64 > limit {
            anyhow::bail!("File {:?} exceeds 1MB limit", input_path);
        }
        let prog = prologue_compiler::compile(&unparsed_file, input_path.parent())?;

        let mut vm = ChimeraVM::new(prog.dna);
        if let Some(g) = prog.grid {
            vm.grid = g;
        }

        vm.prologue_state.active = true;
        if let Some(mode) = prog.orca_mode {
            vm.prologue_state.orca_mode = mode;
        }
        vm.prologue_state.custom_runes = prog.custom_runes;
        vm.prologue_state.alchemy_book = prog.alchemy_book;

        Ok(Self {
            vm,
            status_msg:
                "Running Spectral Genetic Logic. ESC to quit. Space to step, P to pause/play."
                    .into(),
        })
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut playing = true;
        loop {
            terminal
                .draw(|f| self.ui(f))
                .map_err(|e| io::Error::other(e.to_string()))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('p') => {
                                playing = !playing;
                                self.status_msg = if playing {
                                    "Playing".into()
                                } else {
                                    "Paused".into()
                                };
                            }
                            KeyCode::Char(' ') => {
                                if !self.vm.halted {
                                    self.vm.step();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else if playing && !self.vm.halted {
                self.vm.step();
            }
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let title = Paragraph::new(" Chimera Hologram: Spectral Genetic Logic ")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Left Panel: Spatial Grid
        let mut rows = vec![];
        for row in &self.vm.grid {
            let cells = row.iter().map(|v| {
                let s = match v {
                    Value::Int(n) => {
                        if *n == 0 {
                            ".".to_string()
                        } else {
                            n.to_string()
                        }
                    }
                    Value::Str(s) => s.clone(),
                    _ => "?".to_string(),
                };
                ratatui::widgets::Cell::from(s).style(Style::default().fg(Color::Yellow))
            });
            rows.push(Row::new(cells));
        }

        let widths = vec![Constraint::Length(2); 16];
        let grid_table = Table::new(rows, widths).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Spatial Grid "),
        );

        f.render_widget(grid_table, main_chunks[0]);

        // Right Panel: Spectral FFT Grid
        let mut float_grid = vec![0.0; 16 * 16];
        for (y, row) in self.vm.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                float_grid[y * 16 + x] = match cell {
                    Value::Int(n) => *n as f64,
                    Value::Str(s) => {
                        if s == "." || s.is_empty() {
                            0.0
                        } else {
                            // Hash string to float or just use a constant for active runes
                            10.0
                        }
                    }
                    _ => 0.0,
                };
            }
        }

        let hologram = Hologram::from_grid(&float_grid, 16, 16);
        let hologram_mag = hologram.get_magnitude();
        let max_mag = hologram_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_hologram = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Hologram (Frequency Domain) "),
            )
            .marker(ratatui::symbols::Marker::Block)
            .x_bounds([0.0, 16.0])
            .y_bounds([0.0, 16.0])
            .paint(|ctx| {
                let threshold = max_mag * 0.1;
                let mut points = Vec::new();
                for (i, &val) in hologram_mag.iter().enumerate() {
                    if val > threshold {
                        let x = (i % 16) as f64;
                        let y = (i / 16) as f64;
                        // Flip Y to match standard Cartesian visualization or just keep it
                        let y_flipped = 16.0 - y;
                        points.push((x, y_flipped));
                    }
                }
                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Blue,
                });
            });
        f.render_widget(canvas_hologram, main_chunks[1]);

        let status = Paragraph::new(self.status_msg.clone())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status, chunks[2]);
    }
}

fn main() -> Result<()> {
    // Provide a default pro file path for the experiment using manifest dir to avoid pwd issues
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_path =
        std::path::PathBuf::from(manifest_dir).join("../chimera-lang/examples/hello_world.pro");

    // Initialize app BEFORE enabling raw mode to avoid terminal corruption on error
    let mut app = App::new(&input_path)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
