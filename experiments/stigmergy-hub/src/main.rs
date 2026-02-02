use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tui_shared::Tui;

#[derive(Debug, Clone)]
struct GuestbookEntry {
    location: String,
    level: String,
    origin: String,
    status: String,
}

struct App {
    experiments: Vec<String>,
    guestbook: Vec<GuestbookEntry>,
    list_state: ListState,
    should_quit: bool,
    launch_target: Option<String>,
}

impl App {
    fn new(experiments: Vec<String>, guestbook: Vec<GuestbookEntry>) -> Self {
        let mut list_state = ListState::default();
        if !experiments.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            experiments,
            guestbook,
            list_state,
            should_quit: false,
            launch_target: None,
        }
    }

    fn select_next(&mut self) {
        if self.experiments.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.experiments.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn select_previous(&mut self) {
        if self.experiments.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.experiments.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

fn find_root() -> Result<PathBuf> {
    let paths = [".", "..", "../.."];
    for p in paths {
        if Path::new(p).join("GUESTBOOK.md").exists() {
            return Ok(PathBuf::from(p));
        }
    }
    anyhow::bail!("Could not find project root containing GUESTBOOK.md");
}

fn parse_guestbook(path: &Path) -> Result<Vec<GuestbookEntry>> {
    let content = fs::read_to_string(path).context("Failed to read GUESTBOOK.md")?;
    let mut entries = Vec::new();

    let mut current_location = String::new();
    let mut current_level = String::new();
    let mut current_origin = String::new();
    let mut current_status = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("### [Concentration Level:") {
            if !current_location.is_empty() {
                entries.push(GuestbookEntry {
                    location: current_location.clone(),
                    level: current_level.clone(),
                    origin: current_origin.clone(),
                    status: current_status.clone(),
                });
                current_origin.clear();
                current_status.clear();
            }

            if let Some(start_level) = line.find("Level: ") {
                if let Some(end_level) = line.find("]") {
                   current_level = line[start_level + 7..end_level].to_string();
                }
            }
            if let Some(start_loc) = line.find("Location: ") {
                current_location = line[start_loc + 10..].to_string();
            }

        } else if line.starts_with("- **Scent Origin:**") {
            current_origin = line["- **Scent Origin:**".len()..].trim().to_string();
        } else if line.starts_with("- **Status:**") {
            current_status = line["- **Status:**".len()..].trim().to_string();
        }
    }

    if !current_location.is_empty() {
        entries.push(GuestbookEntry {
            location: current_location,
            level: current_level,
            origin: current_origin,
            status: current_status,
        });
    }

    Ok(entries)
}

fn scan_experiments(root: &Path) -> Result<Vec<String>> {
    let experiments_dir = if root.join("experiments").exists() {
        root.join("experiments")
    } else {
        anyhow::bail!("Could not find experiments directory at {:?}", root.join("experiments"));
    };

    let mut names = Vec::new();
    for entry in fs::read_dir(experiments_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.join("Cargo.toml").exists() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name != "stigmergy-hub" {
                     names.push(name.to_string());
                }
            }
        }
    }
    names.sort();
    Ok(names)
}

fn run_tui(app: &mut App) -> Result<()> {
    let mut tui = Tui::init()?;

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                        break;
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
                    KeyCode::Enter => {
                         if let Some(i) = app.list_state.selected() {
                             app.launch_target = Some(app.experiments[i].clone());
                         }
                         break;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_ui_render() {
        let experiments = vec!["exp1".to_string(), "exp2".to_string()];
        let guestbook = vec![
            GuestbookEntry {
                location: "exp1".to_string(),
                level: "HIGH".to_string(),
                origin: "Tester".to_string(),
                status: "Testing".to_string(),
            }
        ];
        let mut app = App::new(experiments, guestbook);

        let backend = TestBackend::new(100, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| ui(f, &mut app)).unwrap();

        // Basic assertions could go here, but for now we just verify no panic
        // and that we can interact with the app state
        assert_eq!(app.list_state.selected(), Some(0));
        app.select_next();
        assert_eq!(app.list_state.selected(), Some(1));
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(f.area());

    // List of experiments
    let items: Vec<ListItem> = app
        .experiments
        .iter()
        .map(|name| {
            // Check if we have guestbook info
            // Guestbook location might be "experiments/git_galaxy" or just "git_galaxy"?
            // Parsed location is like "experiments/git_galaxy"
            let entry = app.guestbook.iter().find(|e| e.location.ends_with(name));
            let style = if let Some(e) = entry {
                match e.level.as_str() {
                    "HIGH" => Style::default().fg(Color::Red),
                    "STABLE TRAIL" => Style::default().fg(Color::Green),
                    "EVAPORATING" => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                }
            } else {
                Style::default()
            };

            let prefix = if entry.is_some() { "🍄 " } else { "   " };

            ListItem::new(format!("{}{}", prefix, name)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Experiments (Stigmergy Hub)"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    // Details view
    if let Some(selected_index) = app.list_state.selected() {
        let name = &app.experiments[selected_index];
        let entry = app.guestbook.iter().find(|e| e.location.ends_with(name));

        let text = if let Some(e) = entry {
            vec![
                Line::from(vec![
                    Span::styled("Concentration Level: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(e.level.clone(), Style::default().fg(Color::Yellow)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Scent Origin: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::from(e.origin.clone()),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                ]),
                Line::from(e.status.clone()),
                Line::from(""),
                Line::from(Span::styled("Press <Enter> to launch", Style::default().fg(Color::Gray))),
            ]
        } else {
             vec![
                 Line::from("No Guestbook entry found for this experiment."),
                 Line::from(""),
                 Line::from(Span::styled("Press <Enter> to launch", Style::default().fg(Color::Gray))),
             ]
        };

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[1]);
    }
}

fn main() -> Result<()> {
    let root = find_root()?;
    let guestbook_path = root.join("GUESTBOOK.md");
    let guestbook = parse_guestbook(&guestbook_path)?;
    let experiments = scan_experiments(&root)?;

    let mut app = App::new(experiments, guestbook);

    loop {
        run_tui(&mut app)?;

        if app.should_quit {
            break;
        }

        if let Some(target) = app.launch_target.take() {
            println!("Launching {}...", target);

            let mut args = vec!["run", "-p", &target];
            // Fix for neuro-terminal
            if target == "neuro-terminal" {
                 args.push("--bin");
                 args.push("neuro-terminal");
            }

            let status = Command::new("cargo")
                .args(&args)
                .status();

            match status {
                Ok(s) => {
                    println!("\nExperiment finished with status: {}", s);
                }
                Err(e) => {
                    println!("\nFailed to run experiment: {}", e);
                }
            }

            println!("Press Enter to return to Hub...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }
    }

    Ok(())
}
