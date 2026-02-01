use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{seq::SliceRandom, Rng};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};

use syntax_invaders::game::Game;
use syntax_invaders::scanner::scan_words;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    game: Game,
    available_words: Vec<String>,
    running: bool,
}

impl App {
    fn new(width: u16, height: u16) -> Self {
        // Scan for words in the current directory
        let path = std::path::Path::new(".");
        let mut words = scan_words(path).unwrap_or_default();

        // Filter out very short words or boring ones if needed
        words.retain(|w| w.len() > 3);

        if words.is_empty() {
            words = vec![
                "struct".into(),
                "enum".into(),
                "impl".into(),
                "trait".into(),
                "type".into(),
                "const".into(),
                "crate".into(),
                "match".into(),
            ];
        }

        Self {
            game: Game::new(width, height),
            available_words: words,
            running: true,
        }
    }

    fn on_tick(&mut self) {
        if self.game.game_over {
            return;
        }

        self.game.tick();

        // Randomly spawn words
        // Rate depends on score? Harder as you go.
        let spawn_rate = 0.02 + (self.game.score as f64 * 0.0001).min(0.1);

        let mut rng = rand::thread_rng();
        if rng.gen_bool(spawn_rate) {
            if let Some(word) = self.available_words.choose(&mut rng) {
                // Check if we already have this word on screen to avoid duplicates (optional, but good for gameplay)
                if !self.game.words.iter().any(|w| w.text == *word) {
                    self.game.spawn_word(word.clone());
                }
            }
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let size = terminal.size()?;
    // We use the terminal size directly for the game world
    let mut app = App::new(size.width, size.height - 2); // Reserve 2 lines for status bar

    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => app.running = false,
                        KeyCode::Char(c) => {
                            // Ctrl+C to quit
                            if key.modifiers == crossterm::event::KeyModifiers::CONTROL && c == 'c'
                            {
                                app.running = false;
                            } else {
                                app.game.input_char(c);
                            }
                        }
                        KeyCode::Backspace => app.game.backspace(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3), // Status bar + Input
        ])
        .split(f.area());

    // Game Area (Canvas)
    // We use Canvas to draw words at specific coordinates
    // Note: Canvas coordinates are usually bottom-left origin (0,0), but game logic might be top-left.
    // Let's assume game logic is (0,0) top-left.
    // Canvas y-axis: 0 is bottom. So we need to invert Y.
    // game.y increases downwards. Canvas y = height - game.y.

    let canvas_height = app.game.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Syntax Invaders "),
        )
        .x_bounds([0.0, app.game.width as f64])
        .y_bounds([0.0, canvas_height])
        .paint(|ctx| {
            if app.game.game_over {
                ctx.print(
                    app.game.width as f64 / 2.0 - 5.0,
                    canvas_height / 2.0,
                    Span::styled(
                        "GAME OVER",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                );
                return;
            }

            for word in &app.game.words {
                // Invert Y for rendering
                let render_y = canvas_height - word.y;

                // Styling
                let is_targeted =
                    word.text.starts_with(&app.game.input) && !app.game.input.is_empty();

                if is_targeted {
                    // Split into matched and unmatched parts
                    let matched_len = app.game.input.len();
                    let matched_part = word.text[..matched_len].to_string();
                    let rest_part = word.text[matched_len..].to_string();

                    let span = Line::from(vec![
                        Span::styled(
                            matched_part,
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(rest_part, Style::default().fg(Color::Green)),
                    ]);

                    ctx.print(word.x, render_y, span);
                } else {
                    ctx.print(
                        word.x,
                        render_y,
                        Span::styled(word.text.clone(), Style::default().fg(Color::Green)),
                    );
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let input_style = if app.game.game_over {
        Style::default().fg(Color::Gray)
    } else {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    };

    let status_text = vec![Line::from(vec![
        Span::raw("Score: "),
        Span::styled(
            format!("{}", app.game.score),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(" | Input: "),
        Span::styled(&app.game.input, input_style),
        Span::raw(" | "),
        Span::raw("Type the falling words! ESC to quit."),
    ])];

    let p = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[1]);
}
