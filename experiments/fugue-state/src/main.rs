pub mod parser;
pub mod synth;
pub mod app;
pub mod ui;

use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use app::App;
use parser::CodeParser;
use synth::generate_wav;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input Rust source file
    #[arg(short, long, default_value = "experiments/fugue-state/src/main.rs")]
    input: PathBuf,

    /// Output WAV file
    #[arg(short, long, default_value = "output.wav")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Read and Parse
    let code = match fs::read_to_string(&cli.input) {
        Ok(c) => c,
        Err(e) => {
             // Fallback to absolute path or just fail nicely
             eprintln!("Error reading file {:?}: {}", cli.input, e);
             return Ok(());
        }
    };

    let mut parser = CodeParser::new();
    if let Err(e) = parser.parse(&code) {
        eprintln!("Error parsing code: {}", e);
        return Ok(());
    }

    if parser.events.is_empty() {
        println!("No musical events found in source.");
        return Ok(());
    }

    // 2. Synthesize
    println!("Generating audio...");
    generate_wav(&parser.events, cli.output.to_str().unwrap())?;
    println!("Generated {}", cli.output.display());
    println!("Starting visualization... (Press 'q' to quit)");

    // Wait a bit to let user read
    std::thread::sleep(Duration::from_secs(1));

    // 3. TUI
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(parser.events);
    app.start();

    let tick_rate = Duration::from_millis(30);

    let res = run_app(&mut terminal, &mut app, tick_rate);

    // Cleanup
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

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> std::io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

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
