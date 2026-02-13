pub mod color;
pub mod parser;
pub mod sentiment;
pub mod ui;

use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use image::{ImageBuffer, Rgb};
use ratatui::{backend::CrosstermBackend, style::Color, Terminal};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use tui_shared::Tui;

use crate::color::ColorMapper;
use crate::parser::LogEntry;
use crate::parser::LogParser;
use crate::sentiment::SentimentAnalyzer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input log file (optional, defaults to stdin if not provided or "-" is used)
    input: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct MappedLog {
    pub entry: LogEntry,
    pub color: Color,
    pub rgb: palette::Srgb<u8>,
    pub sentiment: f32,
}

#[derive(PartialEq)]
pub enum ViewMode {
    Text,
    Spectrum,
}

pub struct App {
    pub logs: Vec<MappedLog>,
    pub scroll: usize,
    pub mode: ViewMode,
    pub should_quit: bool,
}

impl App {
    pub fn new(logs: Vec<MappedLog>) -> Self {
        Self {
            logs,
            scroll: 0,
            mode: ViewMode::Text,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        // Animation updates if any
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Read Input
    let lines = if let Some(path) = args.input {
        if path.to_str() == Some("-") {
            read_stdin()?
        } else {
            read_file(path)?
        }
    } else {
        // Check if stdin is a TTY. If it is, maybe use a demo log?
        // For now, assume piped input or just wait.
        // Actually, reading stdin in blocking mode before TUI init is safest for simple apps.
        if !atty::is(atty::Stream::Stdin) {
            read_stdin()?
        } else {
            // Demo Mode
            generate_demo_logs()
        }
    };

    // 2. Process Logs
    let parser = LogParser::new();
    let sentiment = SentimentAnalyzer::new();
    let color_mapper = ColorMapper::new();

    let mapped_logs: Vec<MappedLog> = lines
        .into_iter()
        .map(|line| {
            let entry = parser.parse(&line);
            let score = sentiment.score(&entry.message);
            let (color, rgb) = color_mapper.map(&entry, score);
            MappedLog {
                entry,
                color,
                rgb,
                sentiment: score,
            }
        })
        .collect();

    let mut app = App::new(mapped_logs);

    // 3. Setup TUI
    let mut tui = Tui::init()?;

    // 4. Run Loop
    let res = run_app(&mut tui.terminal, &mut app);

    // 5. Cleanup
    tui.exit()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn read_stdin() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn read_file(path: PathBuf) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn generate_demo_logs() -> Vec<String> {
    vec![
        "[2023-10-27 10:00:00] [INFO] [System] Booting up...".into(),
        "[2023-10-27 10:00:01] [INFO] [Kernel] CPU check passed".into(),
        "[2023-10-27 10:00:02] [WARN] [Memory] Usage slightly high".into(),
        "[2023-10-27 10:00:03] [ERROR] [Network] Connection refused to 192.168.1.1".into(),
        "[2023-10-27 10:00:04] [INFO] [Network] Retrying connection...".into(),
        "[2023-10-27 10:00:05] [INFO] [Network] Connected successfully!".into(),
        "[2023-10-27 10:00:06] [DEBUG] [Auth] User 'admin' attempting login".into(),
        "[2023-10-27 10:00:07] [ERROR] [Auth] Invalid password".into(),
        "[2023-10-27 10:00:08] [CRITICAL] [Security] Multiple failed login attempts detected!"
            .into(),
        "[2023-10-27 10:00:09] [INFO] [Security] IP 10.0.0.5 banned".into(),
    ]
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            ui::draw(f, app);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => {
                            app.mode = match app.mode {
                                ViewMode::Text => ViewMode::Spectrum,
                                ViewMode::Spectrum => ViewMode::Text,
                            };
                        }
                        KeyCode::Char('s') => {
                            if let Err(_e) = save_palette_image(&app.logs) {
                                // In a real app we'd show a popup
                                // We can't print to stderr easily in TUI mode without breaking layout
                                // But maybe just log it?
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.scroll < app.logs.len().saturating_sub(1) {
                                app.scroll += 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.scroll > 0 {
                                app.scroll -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        app.on_tick();

        if app.should_quit {
            return Ok(());
        }
    }
}
fn save_palette_image(logs: &[MappedLog]) -> Result<()> {
    if logs.is_empty() {
        return Ok(());
    }

    let width = logs.len() as u32;
    let height = 100;

    let mut img = ImageBuffer::new(width, height);

    for (x, log) in logs.iter().enumerate() {
        // Correct way to access sRGB components from palette 0.7
        // log.rgb is palette::Srgb<u8>
        let pixel = Rgb([log.rgb.red, log.rgb.green, log.rgb.blue]);

        for y in 0..height {
            img.put_pixel(x as u32, y, pixel);
        }
    }

    img.save("palette.png")?;
    Ok(())
}
