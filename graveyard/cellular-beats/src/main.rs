use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

#[cfg(feature = "audio")]
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};

const GRID_SIZE: usize = 16;

// Pentatonic Scale (C Major Pentatonic)
// C4, D4, E4, G4, A4, C5, ...
const SCALES: [f32; 16] = [
    261.63,  // C4
    293.66,  // D4
    329.63,  // E4
    392.00,  // G4
    440.00,  // A4
    523.25,  // C5
    587.33,  // D5
    659.25,  // E5
    783.99,  // G5
    880.00,  // A5
    1046.50, // C6
    1174.66, // D6
    1318.51, // E6
    1567.98, // G6
    1760.00, // A6
    2093.00, // C7
];

struct Grid {
    cells: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    fn new() -> Self {
        Self {
            cells: [[false; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn toggle(&mut self, x: usize, y: usize) {
        if x < GRID_SIZE && y < GRID_SIZE {
            self.cells[y][x] = !self.cells[y][x];
        }
    }

    fn clear(&mut self) {
        self.cells = [[false; GRID_SIZE]; GRID_SIZE];
    }

    fn randomize(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                self.cells[y][x] = rng.gen_bool(0.2);
            }
        }
    }

    fn step(&mut self) {
        let mut next = [[false; GRID_SIZE]; GRID_SIZE];
        for (y, row) in next.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.cells[y][x];
                *cell = matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && nx < GRID_SIZE as isize
                    && ny >= 0
                    && ny < GRID_SIZE as isize
                    && self.cells[ny as usize][nx as usize]
                {
                    count += 1;
                }
            }
        }
        count
    }
}

struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<OutputStreamHandle>,
    enabled: bool,
}

impl AudioEngine {
    fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Self {
                    _stream: Some(stream),
                    stream_handle: Some(handle),
                    enabled: true,
                },
                Err(_) => Self {
                    _stream: None,
                    stream_handle: None,
                    enabled: false,
                },
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            Self { enabled: false }
        }
    }

    fn play_notes(&self, _notes: Vec<f32>) {
        if self.enabled {
            #[cfg(feature = "audio")]
            if let Some(handle) = &self.stream_handle {
                for freq in _notes {
                    if let Ok(sink) = Sink::try_new(handle) {
                        // Short sine wave envelope
                        let source = rodio::source::SineWave::new(freq)
                            .take_duration(Duration::from_millis(150))
                            .amplify(0.10)
                            .fade_in(Duration::from_millis(10))
                            .fade_out(Duration::from_millis(140)); // Quick fade out to avoid clicks
                        sink.append(source);
                        sink.detach();
                    }
                }
            }
        }
    }
}

struct App {
    grid: Grid,
    audio: AudioEngine,
    scan_pos: usize,
    sim_running: bool,
    playback_running: bool,
    cursor: (usize, usize), // x, y
    exit: bool,
    tempo_ms: u64,
}

impl App {
    fn new() -> Self {
        let mut grid = Grid::new();
        // Start with a glider
        grid.toggle(1, 0);
        grid.toggle(2, 1);
        grid.toggle(0, 2);
        grid.toggle(1, 2);
        grid.toggle(2, 2);

        Self {
            grid,
            audio: AudioEngine::new(),
            scan_pos: 0,
            sim_running: false,
            playback_running: true,
            cursor: (GRID_SIZE / 2, GRID_SIZE / 2),
            exit: false,
            tempo_ms: 150,
        }
    }

    fn on_tick(&mut self) {
        if self.playback_running {
            // Play notes at current scan_pos
            let mut notes = Vec::new();
            for y in 0..GRID_SIZE {
                if self.grid.cells[y][self.scan_pos] {
                    // Index mapping: Bottom (y=15) is Low Pitch (index 0)?
                    // Or Top (y=0) is High Pitch?
                    // Let's make Top (0) = High Pitch (15).
                    let index = (GRID_SIZE - 1).saturating_sub(y);
                    if index < SCALES.len() {
                        notes.push(SCALES[index]);
                    }
                }
            }
            if !notes.is_empty() {
                self.audio.play_notes(notes);
            }

            // Move scanline
            self.scan_pos = (self.scan_pos + 1) % GRID_SIZE;

            // If we wrapped, step simulation if enabled
            if self.scan_pos == 0 && self.sim_running {
                self.grid.step();
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let mut tick_rate = Duration::from_millis(app.tempo_ms);

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.exit = true,
                            KeyCode::Char(' ') => app.playback_running = !app.playback_running,
                            KeyCode::Enter => app.sim_running = !app.sim_running,
                            KeyCode::Char('c') => app.grid.clear(),
                            KeyCode::Char('r') => app.grid.randomize(),
                            KeyCode::Up => app.cursor.1 = app.cursor.1.saturating_sub(1),
                            KeyCode::Down => app.cursor.1 = (app.cursor.1 + 1).min(GRID_SIZE - 1),
                            KeyCode::Left => app.cursor.0 = app.cursor.0.saturating_sub(1),
                            KeyCode::Right => app.cursor.0 = (app.cursor.0 + 1).min(GRID_SIZE - 1),
                            KeyCode::Char('x') | KeyCode::Char('z') => {
                                app.grid.toggle(app.cursor.0, app.cursor.1);
                            }
                            KeyCode::Char('+') => {
                                app.tempo_ms = app.tempo_ms.saturating_sub(10).max(50);
                                tick_rate = Duration::from_millis(app.tempo_ms);
                            }
                            KeyCode::Char('-') => {
                                app.tempo_ms = app.tempo_ms.saturating_add(10).min(1000);
                                tick_rate = Duration::from_millis(app.tempo_ms);
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(_mouse) => {
                    // Ignored
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
            // Reset tick rate in case tempo changed
            tick_rate = Duration::from_millis(app.tempo_ms);
        }

        if app.exit {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.area());

    // Left: Grid
    let grid_block = Block::default()
        .borders(Borders::ALL)
        .title("Cellular Beats");

    let area = grid_block.inner(chunks[0]);
    f.render_widget(grid_block, chunks[0]);

    let mut rows = Vec::new();
    for y in 0..GRID_SIZE {
        let mut spans = Vec::new();
        for x in 0..GRID_SIZE {
            let is_alive = app.grid.cells[y][x];
            let is_cursor = x == app.cursor.0 && y == app.cursor.1;
            let is_scan = x == app.scan_pos;

            let char_sym = if is_alive { "██" } else { "  " };

            let mut style = Style::default();

            if is_alive {
                style = style.fg(Color::Green);
            } else {
                style = style.bg(Color::Reset);
            }

            if is_scan {
                style = style.bg(Color::DarkGray);
                if is_alive {
                    style = style.fg(Color::LightGreen);
                }
            }

            if is_cursor {
                style = style.add_modifier(Modifier::REVERSED); // Invert cursor
            }

            spans.push(Span::styled(char_sym, style));
        }
        rows.push(Line::from(spans));
    }

    let grid_widget = Paragraph::new(rows);
    f.render_widget(grid_widget, area);

    // Right: Controls & Info
    let info_text = vec![
        Line::from("Controls:"),
        Line::from(" [Arrows] Move Cursor"),
        Line::from(" [z/x]    Toggle Cell"),
        Line::from(" [Space]  Play/Pause Scanline"),
        Line::from(" [Enter]  Enable/Disable Evolution"),
        Line::from(" [c]      Clear Grid"),
        Line::from(" [r]      Randomize Grid"),
        Line::from(" [+/-]    Tempo"),
        Line::from(" [q]      Quit"),
        Line::from(""),
        Line::from(format!("Tempo: {} ms", app.tempo_ms)),
        Line::from(format!(
            "Scanline: {}",
            if app.playback_running {
                "PLAYING"
            } else {
                "PAUSED"
            }
        )),
        Line::from(format!(
            "Evolution: {}",
            if app.sim_running { "ACTIVE" } else { "MANUAL" }
        )),
        Line::from(format!(
            "Audio: {}",
            if app.audio.enabled { "ON" } else { "OFF" }
        )),
        Line::from(""),
        Line::from("Concept:"),
        Line::from("Active cells play a note when scanned."),
        Line::from("Vertical Position = Pitch (Pentatonic)."),
        Line::from("Evolution happens when scanline wraps."),
    ];

    let info_block =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(info_block, chunks[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_step_blinker() {
        let mut grid = Grid::new();
        // Blinker (vertical)
        grid.toggle(1, 0);
        grid.toggle(1, 1);
        grid.toggle(1, 2);

        grid.step();

        // Should be horizontal
        assert!(!grid.cells[0][1]);
        assert!(grid.cells[1][0]);
        assert!(grid.cells[1][1]);
        assert!(grid.cells[1][2]);
        assert!(!grid.cells[2][1]);

        grid.step();

        // Should be vertical again
        assert!(grid.cells[0][1]);
        assert!(grid.cells[1][1]);
        assert!(grid.cells[2][1]);
    }

    #[test]
    fn test_grid_step_block() {
        let mut grid = Grid::new();
        // Block (still life)
        grid.toggle(0, 0);
        grid.toggle(0, 1);
        grid.toggle(1, 0);
        grid.toggle(1, 1);

        grid.step();

        assert!(grid.cells[0][0]);
        assert!(grid.cells[0][1]);
        assert!(grid.cells[1][0]);
        assert!(grid.cells[1][1]);
    }
}
