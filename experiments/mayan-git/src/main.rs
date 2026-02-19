use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use git_associates::GitModel;
use ratatui::{prelude::*, widgets::*};
use std::{io, time::Duration};

mod mayan;
use mayan::LongCount;

struct App {
    commits: Vec<git_associates::Commit>,
    selected_index: usize,
    should_quit: bool,
}

impl App {
    fn new() -> Result<Self> {
        // Try to open git repo from current directory, which should discover the root
        let git_model = GitModel::open(".");
        let commits = match git_model {
            Ok(model) => model.history(100).unwrap_or_default(),
            Err(_) => {
                // Fallback for testing/CI if no git repo found (though we are in a repo)
                vec![]
            }
        };

        Ok(Self {
            commits,
            selected_index: 0,
            should_quit: false,
        })
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.commits.is_empty() && self.selected_index < self.commits.len() - 1 {
                    self.selected_index += 1;
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_result = App::new();

    if let Err(e) = app_result {
         disable_raw_mode()?;
         execute!(io::stdout(), LeaveAlternateScreen)?;
         eprintln!("Error initializing app: {}", e);
         return Err(e);
    }
    let mut app = app_result.unwrap();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.area());

    render_commit_list(f, chunks[0], app);
    render_stela(f, chunks[1], app);
}

fn render_commit_list(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app.commits
        .iter()
        .map(|c| {
            let lc = LongCount::from_timestamp(c.timestamp);
            let content = Line::from(vec![
                Span::styled(format!("{:<15}", lc.to_string()), Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled(format!("{}", c.short_hash), Style::default().fg(Color::Cyan)),
                Span::raw(" | "),
                Span::raw(c.message.lines().next().unwrap_or("")),
            ]);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("The Long Count of History").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    if !app.commits.is_empty() {
        state.select(Some(app.selected_index));
    }
    f.render_stateful_widget(list, area, &mut state);
}

fn render_stela(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().title("Stela").borders(Borders::ALL);

    if app.commits.is_empty() {
        let p = Paragraph::new("No commits found (or not a git repo).")
            .block(block);
        f.render_widget(p, area);
        return;
    }

    let commit = &app.commits[app.selected_index];
    let lc = LongCount::from_timestamp(commit.timestamp);
    let stela_art = lc.to_stela();

    let mut lines = vec![
        Line::from(vec![
            Span::styled("Date: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", commit.timestamp)),
        ]),
        Line::from(vec![
            Span::styled("Mayan: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}", lc), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
    ];

    // Add stela art lines
    for line in stela_art.lines() {
        lines.push(Line::from(Span::styled(line.to_string(), Style::default().fg(Color::Green))));
    }

    let p = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(p, area);
}
