use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::process::Command;
use tui_shared::Tui;

pub mod metadata;

struct App {
    all_packages: Vec<cargo_metadata::Package>,
    filtered_packages: Vec<cargo_metadata::Package>,
    list_state: ListState,
    filter_query: String,
}

impl App {
    fn new(packages: Vec<cargo_metadata::Package>) -> Self {
        let mut list_state = ListState::default();
        if !packages.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            all_packages: packages.clone(),
            filtered_packages: packages,
            list_state,
            filter_query: String::new(),
        }
    }

    fn update_filter(&mut self) {
        if self.filter_query.is_empty() {
            self.filtered_packages = self.all_packages.clone();
        } else {
            let query = self.filter_query.to_lowercase();
            self.filtered_packages = self
                .all_packages
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }

        if !self.filtered_packages.is_empty() {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    fn next(&mut self) {
        if self.filtered_packages.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_packages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.filtered_packages.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_packages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

fn main() -> Result<()> {
    let packages = metadata::fetch_packages()?;
    let mut app = App::new(packages);

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui, &mut app);
    tui.exit()?;

    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        app.filter_query.push(c);
                        app.update_filter();
                    }
                    KeyCode::Backspace => {
                        app.filter_query.pop();
                        app.update_filter();
                    }
                    KeyCode::Esc => {
                        if app.filter_query.is_empty() {
                            return Ok(());
                        } else {
                            app.filter_query.clear();
                            app.update_filter();
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(i) = app.list_state.selected() {
                            if let Some(pkg) = app.filtered_packages.get(i) {
                                let pkg_name = pkg.name.clone();

                                // Exit TUI
                                tui.exit()?;

                                println!("🚀 Launching {}...", pkg_name);
                                println!("---------------------------------------------------");

                                let status = Command::new("cargo")
                                    .args(["run", "-p", &pkg_name])
                                    .status();

                                match status {
                                    Ok(s) => println!("\n---------------------------------------------------\nProcess exited with {}", s),
                                    Err(e) => println!("\n---------------------------------------------------\nFailed to start process: {}", e),
                                }

                                println!("\nPress <Enter> to return to Compass...");
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input)?;

                                // Restore TUI
                                *tui = Tui::init()?;
                            }
                        }
                    }
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_chunks[1]);

    // Search bar
    let search_text = format!("Filter: {}_", app.filter_query);
    let search_bar =
        Paragraph::new(search_text).block(Block::default().borders(Borders::ALL).title(" Search "));
    f.render_widget(search_bar, main_chunks[0]);

    // List
    let items: Vec<ListItem> = app
        .filtered_packages
        .iter()
        .map(|p| {
            ListItem::new(Line::from(vec![Span::styled(
                p.name.clone(),
                Style::default().fg(Color::White),
            )]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Packages "))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, content_chunks[0], &mut app.list_state);

    // Details view
    if let Some(selected_index) = app.list_state.selected() {
        if let Some(pkg) = app.filtered_packages.get(selected_index) {
            let deps_count = pkg.dependencies.len();
            let description = pkg.description.as_deref().unwrap_or("No description");

            let details = vec![
                Line::from(vec![
                    Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&pkg.name),
                ]),
                Line::from(vec![
                    Span::styled("Version: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(pkg.version.to_string()),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Description: ",
                    Style::default().add_modifier(Modifier::BOLD),
                )]),
                Line::from(Span::raw(description)),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        "Dependencies: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(deps_count.to_string()),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Path: ",
                    Style::default().add_modifier(Modifier::BOLD),
                )]),
                Line::from(Span::raw(pkg.manifest_path.as_str())),
                Line::from(""),
                Line::from(Span::styled(
                    "Press <Enter> to Run | <Esc> to Clear/Quit",
                    Style::default().fg(Color::Yellow),
                )),
            ];

            let p = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title(" Details "))
                .wrap(Wrap { trim: true });
            f.render_widget(p, content_chunks[1]);
        }
    }
}
