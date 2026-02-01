use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use schrodingers_text::{PatternExtractor, WaveFunction};
use std::{error::Error, io};

#[derive(Debug, PartialEq)]
enum AppState {
    Splash,
    Running,
}

struct App {
    state: AppState,
    wave: WaveFunction,
    paused: bool,
    tick_count: usize,
}

impl App {
    fn new() -> Self {
        // Sample text with repeating patterns for WFC
        // Using simple patterns that can tile and combine
        let text = "░▒▓█▓▒░\n\
                    ▒▓█ █▓▒\n\
                    ▓█ ⚛ █▓\n\
                    █ ⚛ ⚛ █\n\
                    ▓█ ⚛ █▓\n\
                    ▒▓█ █▓▒\n\
                    ░▒▓█▓▒░\n\
                    \n\
                    .:~^~:.\n\
                    :~^*^~:\n\
                    ~^*⚛*^~\n\
                    ^*⚛⚛⚛*^\n\
                    ~^*⚛*^~\n\
                    :~^*^~:\n\
                    .:~^~:.";
        let rules = PatternExtractor::from_text(text);
        let wave = WaveFunction::new(80, 24, rules);

        Self {
            state: AppState::Splash,
            wave,
            paused: false,
            tick_count: 0,
        }
    }

    fn on_tick(&mut self) {
        if self.state == AppState::Running {
            self.tick_count = self.tick_count.wrapping_add(1);
            if !self.paused {
                for _ in 0..5 {
                    if !self.wave.collapse() {
                        // Done
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Run loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B::Error: From<io::Error> + Into<io::Error>,
{
    let tick_rate = std::time::Duration::from_millis(16);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, app)).map_err(|e| e.into())?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match app.state {
                AppState::Splash => match key.code {
                    KeyCode::Enter => app.state = AppState::Running,
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                AppState::Running => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('r') => {
                            *app = App::new();
                            app.state = AppState::Running; // Skip splash on reset
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }
    }
}

struct GridWidget<'a> {
    wave: &'a WaveFunction,
    tick: usize,
}

impl<'a> Widget for GridWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the grid in the area
        let grid_width = self.wave.width as u16;
        let grid_height = self.wave.height as u16;

        let start_x = area.x + (area.width.saturating_sub(grid_width)) / 2;
        let start_y = area.y + (area.height.saturating_sub(grid_height)) / 2;

        for y in 0..self.wave.height {
            for x in 0..self.wave.width {
                let cell = &self.wave.grid[y][x];
                let bx = start_x + x as u16;
                let by = start_y + y as u16;

                if bx >= area.x + area.width || by >= area.y + area.height {
                    continue;
                }

                let (char, style) = if cell.is_collapsed() {
                    if let Some(c) = cell.possibilities.first() {
                        (*c, Style::default().fg(Color::White))
                    } else {
                        (
                            'X',
                            Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::RAPID_BLINK),
                        ) // Contradiction
                    }
                } else {
                    let entropy = cell.entropy();
                    if entropy == 0 {
                        (
                            '!',
                            Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::RAPID_BLINK),
                        )
                    } else {
                        // "Quantum Shimmer"
                        // Pick char based on tick and position to look random but deterministic per frame
                        let idx = (self.tick + x + y * 13) % entropy;
                        let c = cell.possibilities[idx];

                        // Color based on entropy
                        // High entropy = Red, Low = Green/Blue
                        let color = if entropy > 20 {
                            Color::Red
                        } else if entropy > 10 {
                            Color::Yellow
                        } else if entropy > 5 {
                            Color::Green
                        } else {
                            Color::Cyan
                        };

                        (c, Style::default().fg(color).add_modifier(Modifier::DIM))
                    }
                };

                if let Some(cell) = buf.cell_mut((bx, by)) {
                    cell.set_char(char).set_style(style);
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    match app.state {
        AppState::Splash => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ])
                .split(f.area());

            let title = Paragraph::new("⚛️  SCHRÖDINGER'S TEXT ⚛️")
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );

            let subtitle = Paragraph::new(
                "Wave Function Collapse Text Generation\n\nPress [ENTER] to Observe",
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));

            f.render_widget(title, chunks[1]);
            f.render_widget(subtitle, chunks[2]); // actually 2 is bottom part, let's put it there
        }
        AppState::Running => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.area());

            // Title
            let title = Paragraph::new(
                "Schrödinger's Text (WFC) | [Space] Pause/Play | [R] Reset | [Q] Quit",
            )
            .block(Block::default().borders(Borders::ALL).title(" ⚛️ Genesis "));
            f.render_widget(title, chunks[0]);

            // Grid
            let grid = GridWidget {
                wave: &app.wave,
                tick: app.tick_count,
            };

            let grid_block = Block::default()
                .borders(Borders::ALL)
                .title(" Observation Window ");
            f.render_widget(grid_block, chunks[1]);

            // We need to render grid INSIDE the block's inner area
            let inner_area = chunks[1].inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 1,
            });
            f.render_widget(grid, inner_area);
        }
    }
}
