//! 🧬 Splice: Cross git-associates × platter
//!
//! **Concept**: Git History Heatmap.
//!
//! **Lineage**:
//! - Parent A (git-associates): Provides native Git repository parsing, returning structured metadata on commits and file changes.
//! - Parent B (platter): Provides a 2D scalar field to visualize continuous decay and saturation.
//!
//! **Novel trait**: Projecting discrete file modifications across a codebase's history into a continuous 2D scalar field. Codebase modifications dynamically heat and cool the field.
//!
//! **Predicted Phenotype**: An evolving heatmap showing areas of high codebase churn.

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git_associates::GitModel;
use platter::Platter;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    env,
    hash::{Hash, Hasher},
    io::stdout,
    time::{Duration, Instant},
};

fn hash_path(path: &str, width: usize, height: usize) -> (usize, usize) {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut hasher);
    let hash = hasher.finish();
    let x = (hash % width as u64) as usize;
    let y = ((hash / width as u64) % height as u64) as usize;
    (x, y)
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
    let current_dir = env::current_dir()?;
    let git_model = GitModel::open(current_dir)?;
    let history = git_model.history_with_diffs(500)?;

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let width = 80;
    let height = 40;
    let mut platter = Platter::new(width, height);

    // We will iterate from the oldest to the newest to show evolution.
    let mut commit_idx = history.len();

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let mut lines = Vec::new();
            for y in 0..platter.height() {
                let mut row_spans = Vec::new();
                for x in 0..platter.width() {
                    let val = platter.get(x, y);
                    let symbol = map_value_to_char(val);

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

            let title = if commit_idx > 0 && commit_idx <= history.len() {
                let commit = &history[commit_idx - 1];
                format!(" Git Platter: {} [{}] ", commit.message.lines().next().unwrap_or(""), commit.short_hash)
            } else {
                " Git Platter (Finished) ".to_string()
            };

            let p = Paragraph::new(lines).block(
                Block::default()
                    .title(title)
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
            platter.decay(0.90);

            if commit_idx > 0 {
                commit_idx -= 1;
                let commit = &history[commit_idx];

                for file in &commit.files {
                    let (x, y) = hash_path(&file.path, width, height);
                    let churn = file.insertions + file.deletions;
                    let heat = (churn as f64 / 20.0).min(1.0); // max saturation per commit per file

                    if heat > 0.0 {
                        platter.saturate(x, y, heat);
                        // Spread slightly to make it visible
                        if x > 0 { platter.saturate(x - 1, y, heat * 0.5); }
                        if x < width - 1 { platter.saturate(x + 1, y, heat * 0.5); }
                        if y > 0 { platter.saturate(x, y - 1, heat * 0.5); }
                        if y < height - 1 { platter.saturate(x, y + 1, heat * 0.5); }
                    }
                }
            }
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
