use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{BlameOptions, Repository};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use rustfft::{FftPlanner, num_complex::Complex as FftComplex};
use std::{error::Error, io, path::Path, time::{Duration, SystemTime}};

/// "Spectral History"
/// Combines `chrontext` (git blame) with `hologram-text` (FFT rendering).

struct App {
    lines: Vec<LineData>,
    hologram_grid: Vec<Vec<f64>>,
    phase: f64,
}

#[derive(Clone)]
struct LineData {
    content: String,
    age_score: f64, // 0.0 (new) to 1.0 (old)
}

impl App {
    fn new() -> App {
        App {
            lines: Vec::new(),
            hologram_grid: vec![vec![0.0; 64]; 64],
            phase: 0.0,
        }
    }

    fn analyze_repo(&mut self, repo_path: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
        let repo = Repository::open(repo_path)?;
        let mut opts = BlameOptions::new();
        let blame = repo.blame_file(Path::new(file_path), Some(&mut opts))?;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let mut max_age = 1;

        let mut temp_lines = Vec::new();

        // Very basic file reading (could just read the raw file, but blame lines match it)
        let blob = repo.revparse_single(format!("HEAD:{}", file_path).as_str())?.peel_to_blob()?;
        let content = String::from_utf8_lossy(blob.content());
        let file_lines: Vec<&str> = content.lines().collect();

        for (i, line) in file_lines.iter().enumerate() {
            if let Some(hunk) = blame.get_line(i + 1) {
                let commit = repo.find_commit(hunk.final_commit_id())?;
                let time = commit.time().seconds();
                let age = now.saturating_sub(time as u64);
                if age > max_age {
                    max_age = age;
                }
                temp_lines.push(LineData {
                    content: line.to_string(),
                    age_score: age as f64,
                });
            } else {
                temp_lines.push(LineData {
                    content: line.to_string(),
                    age_score: 0.0,
                });
            }
        }

        for line in &mut temp_lines {
            line.age_score = line.age_score / (max_age as f64);
        }

        self.lines = temp_lines;
        Ok(())
    }

    fn update_hologram(&mut self) {
        self.phase += 0.1;
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(64);

        let mut buffer: Vec<FftComplex<f64>> = vec![FftComplex { re: 0.0, im: 0.0 }; 64];

        // Map age_score into a 1D signal to generate 2D interference
        let mut density = vec![0.0; 64];
        for (i, line) in self.lines.iter().enumerate().take(64) {
            density[i] = line.age_score;
        }

        for i in 0..64 {
            buffer[i] = FftComplex {
                re: density[i] * (self.phase + (i as f64) * 0.1).cos(),
                im: density[i] * (self.phase + (i as f64) * 0.1).sin(),
            };
        }

        fft.process(&mut buffer);

        for y in 0..64 {
            for x in 0..64 {
                // Generate 2D interference pattern from 1D FFT
                let val_x = buffer[x].norm();
                let val_y = buffer[y].norm();
                let interference = (val_x * val_y * (self.phase + (x * y) as f64 * 0.01).sin()).abs();
                self.hologram_grid[y][x] = interference;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    // Try to open this crate's own Cargo.toml or similar to visualize
    if let Err(_) = app.analyze_repo(".", "Cargo.toml") {
        // Fallback for demo
        app.lines.push(LineData { content: "No git repo found or Cargo.toml missing".into(), age_score: 1.0 });
    }

    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>>
where
    <B as Backend>::Error: 'static,
{
    loop {
        app.update_hologram();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.area());

            // Left side: Chrontext (Age-colored code)
            let mut chron_text = vec![];
            for line in &app.lines {
                // Color scale from blue (new) to red (old)
                let r = (line.age_score * 255.0) as u8;
                let b = ((1.0 - line.age_score) * 255.0) as u8;
                let style = Style::default().fg(Color::Rgb(r, 100, b));
                chron_text.push(ratatui::text::Line::from(Span::styled(line.content.clone(), style)));
            }

            let chron_block = Paragraph::new(chron_text)
                .block(Block::default().borders(Borders::ALL).title(" chrontext (Git Age) "));
            f.render_widget(chron_block, chunks[0]);

            // Right side: Hologram
            let mut hologram_lines = vec![];
            for y in 0..std::cmp::min(64, chunks[1].height as usize - 2) {
                let mut row_str = String::new();
                for x in 0..std::cmp::min(64, chunks[1].width as usize - 2) {
                    let intensity = app.hologram_grid[y][x];
                    let char = if intensity > 2.0 {
                        '#'
                    } else if intensity > 1.0 {
                        '+'
                    } else if intensity > 0.5 {
                        '.'
                    } else {
                        ' '
                    };
                    row_str.push(char);
                }
                hologram_lines.push(ratatui::text::Line::from(Span::raw(row_str)));
            }

            let hologram_block = Paragraph::new(hologram_lines)
                .block(Block::default().borders(Borders::ALL).title(" hologram-text (Spectral Pattern) "));
            f.render_widget(hologram_block, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}
