use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::env;
use std::time::{Duration, Instant};

mod app;
mod blame;
mod ui;

use app::App;
use blame::BlameAnalyzer;
use tui_shared::Tui;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: chron-flock <file-path>");
        return Ok(());
    }

    let file_path = &args[1];

    let mut tui = Tui::init()?;

    let analyzer = BlameAnalyzer::new(".");
    let blame_info = analyzer.analyze(std::path::Path::new(file_path))?;

    let mut app = App::new(file_path.clone(), blame_info)?;

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Char('k') => app.previous_line(),
                    KeyCode::Down | KeyCode::Char('j') => app.next_line(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
