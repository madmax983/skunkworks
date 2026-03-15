use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{BlameOptions, Repository};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{env, fs, io, path::Path, time::Duration};

#[derive(Debug, Clone)]
struct LineInfo {
    line_number: usize,
    commit_hash: String,
    author: String,
    date: DateTime<Utc>,
    message: String,
    age_score: f64, // 0.0 (Oldest) to 1.0 (Newest)
}

struct App {
    path: String,
    content: Vec<String>,
    blame_info: Vec<LineInfo>,
    scroll: usize,
    selected_line: usize,
}

impl App {
    fn new(path: String, blame_info: Vec<LineInfo>) -> Result<Self> {
        let content_str = fs::read_to_string(&path)?;
        let content: Vec<String> = content_str.lines().map(|s| s.to_string()).collect();

        Ok(Self {
            path,
            content,
            blame_info,
            scroll: 0,
            selected_line: 0,
        })
    }

    fn next_line(&mut self) {
        if self.selected_line < self.content.len().saturating_sub(1) {
            self.selected_line += 1;
        }
    }

    fn previous_line(&mut self) {
        if self.selected_line > 0 {
            self.selected_line -= 1;
        }
    }

    fn update_scroll(&mut self, height: usize) {
        if height == 0 {
            return;
        }
        if self.selected_line >= self.scroll + height {
            self.scroll = self.selected_line + 1 - height;
        } else if self.selected_line < self.scroll {
            self.scroll = self.selected_line;
        }
    }
}

fn analyze_blame(file_path: &Path, start_path: &str) -> Result<Vec<LineInfo>> {
    let discover_path = if file_path.is_absolute() {
        file_path
    } else {
        Path::new(start_path)
    };

    let repo = Repository::discover(discover_path).context("Failed to discover git repository")?;

    let abs_file_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        std::fs::canonicalize(file_path)
            .context(format!("Failed to canonicalize path: {:?}", file_path))?
    };

    let workdir = repo.workdir().context("Repository has no workdir")?;
    let workdir = std::fs::canonicalize(workdir).unwrap_or(workdir.to_path_buf());

    let path_relative = abs_file_path.strip_prefix(&workdir).context(format!(
        "File {:?} is not in repository {:?}",
        abs_file_path, workdir
    ))?;

    let mut opts = BlameOptions::new();
    let blame = repo
        .blame_file(path_relative, Some(&mut opts))
        .context(format!("Failed to blame file: {:?}", path_relative))?;

    let mut lines = Vec::new();
    let mut min_time = i64::MAX;
    let mut max_time = i64::MIN;
    let now = Utc::now().timestamp();

    for hunk in blame.iter() {
        let commit_id = hunk.final_commit_id();
        let time = if commit_id.is_zero() {
            now
        } else if let Ok(commit) = repo.find_commit(commit_id) {
            commit.time().seconds()
        } else {
            now
        };

        if time < min_time {
            min_time = time;
        }
        if time > max_time {
            max_time = time;
        }
    }

    if min_time > max_time {
        min_time = now;
        max_time = now;
    }

    if min_time == max_time {
        min_time -= 1;
    }

    let range = (max_time - min_time) as f64;

    for hunk in blame.iter() {
        let commit_id = hunk.final_commit_id();

        let (author_name, message, time, date, hash_str) = if commit_id.is_zero() {
            (
                "You (Uncommitted)".to_string(),
                "Uncommitted changes".to_string(),
                now,
                Utc::now(),
                "00000000".to_string(),
            )
        } else if let Ok(commit) = repo.find_commit(commit_id) {
            let author = commit.author();
            let author_name = author.name().unwrap_or("Unknown").to_string();
            let message = commit.summary().unwrap_or("").to_string();
            let time = commit.time().seconds();
            let date = Utc.timestamp_opt(time, 0).single().unwrap_or_default();
            (
                author_name,
                message,
                time,
                date,
                commit_id.to_string()[..8].to_string(),
            )
        } else {
            (
                "Unknown".to_string(),
                "Unknown commit".to_string(),
                now,
                Utc::now(),
                "????????".to_string(),
            )
        };

        let age_score = (time - min_time) as f64 / range;
        let start_line = hunk.final_start_line();
        let count = hunk.lines_in_hunk();

        for i in 0..count {
            lines.push(LineInfo {
                line_number: start_line + i,
                commit_hash: hash_str.clone(),
                author: author_name.clone(),
                date,
                message: message.clone(),
                age_score,
            });
        }
    }

    Ok(lines)
}

fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    draw_content(f, app, chunks[0]);
    draw_status(f, app, chunks[1]);
}

fn draw_content(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Chrontext: {} ", app.path));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let height = inner_area.height as usize;
    let start_line = app.scroll;
    let end_line = (start_line + height).min(app.content.len());

    for (i, line_idx) in (start_line..end_line).enumerate() {
        let line_num = line_idx + 1;
        let content = &app.content[line_idx];

        let info = app.blame_info.iter().find(|l| l.line_number == line_num);

        let color = if let Some(info) = info {
            let s = info.age_score;
            let val = (s * 255.0) as u8;
            Color::Rgb(val, val, 255)
        } else {
            Color::Gray
        };

        let style = if line_idx == app.selected_line {
            Style::default()
                .fg(Color::Black)
                .bg(color)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        let line_widget = Line::from(vec![
            Span::styled(
                format!("{:4} ", line_num),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(content, style),
        ]);

        let y = inner_area.y + i as u16;
        if y < inner_area.bottom() {
            f.render_widget(
                Paragraph::new(line_widget),
                Rect::new(inner_area.x, y, inner_area.width, 1),
            );
        }
    }
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Info ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let line_num = app.selected_line + 1;
    let info = app.blame_info.iter().find(|l| l.line_number == line_num);

    let text = if let Some(info) = info {
        format!(
            "Hash: {} | Author: {} | Date: {} | Age: {:.2} | Msg: {}",
            info.commit_hash,
            info.author,
            info.date.format("%Y-%m-%d %H:%M"),
            info.age_score,
            info.message
        )
    } else {
        "No blame info".to_string()
    };

    f.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::White)),
        inner,
    );
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        let size = terminal.size()?;
        let view_height = (size.height as usize).saturating_sub(5); // -3 for status, -2 for borders
        app.update_scroll(view_height);

        terminal.draw(|f| draw(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next_line(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_line(),
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chrontext <file_path>");
        return Ok(());
    }
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);

    println!("Analyzing {}...", file_path_str);
    let blame_info = analyze_blame(file_path, ".")?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(file_path_str.to_string(), blame_info)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_age_score_normalization() {
        let min_time = 1000;
        let max_time = 2000;
        let range = (max_time - min_time) as f64;

        let time = 1500;
        let score = (time - min_time) as f64 / range;

        assert_eq!(score, 0.5);
    }
}
