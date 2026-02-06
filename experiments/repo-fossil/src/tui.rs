use crate::entropy::Fossil;
use crate::git::Commit;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct AppState {
    pub commits: Vec<Commit>,
    pub commit_list_state: ListState,

    pub files: Vec<String>,
    pub file_list_state: ListState,

    pub current_fossil: Option<Fossil>,
    // TODO: Add user input buffer for restoration mechanics
    pub mode: Mode,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    CommitSelect,
    FileSelect,
    Excavation,
}

impl AppState {
    pub fn new(commits: Vec<Commit>) -> Self {
        let mut state = ListState::default();
        if !commits.is_empty() {
            state.select(Some(0));
        }

        Self {
            commits,
            commit_list_state: state,
            files: vec![],
            file_list_state: ListState::default(),
            current_fossil: None,
            mode: Mode::CommitSelect,
        }
    }

    pub fn next_commit(&mut self) {
        if self.commits.is_empty() {
            return;
        }
        let i = match self.commit_list_state.selected() {
            Some(i) => {
                if i >= self.commits.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.commit_list_state.select(Some(i));
    }

    pub fn previous_commit(&mut self) {
        if self.commits.is_empty() {
            return;
        }
        let i = match self.commit_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.commits.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.commit_list_state.select(Some(i));
    }

    pub fn next_file(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.file_list_state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.file_list_state.select(Some(i));
    }

    pub fn previous_file(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.file_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.file_list_state.select(Some(i));
    }
}

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Commit List
    let items: Vec<ListItem> = state
        .commits
        .iter()
        .map(|c| {
            let hash = if c.hash.len() > 7 {
                &c.hash[..7]
            } else {
                &c.hash
            };
            // Simple truncation of message
            let msg = if c.message.len() > 20 {
                format!("{}...", &c.message[..20])
            } else {
                c.message.clone()
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", hash), Style::default().fg(Color::Yellow)),
                Span::raw(msg),
            ]))
        })
        .collect();

    let commit_border_style = if state.mode == Mode::CommitSelect {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let commits_block = Block::default()
        .borders(Borders::ALL)
        .title("Strata (Commits)")
        .border_style(commit_border_style);

    let commits_list = List::new(items)
        .block(commits_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(commits_list, left_chunks[0], &mut state.commit_list_state);

    // File List
    let file_items: Vec<ListItem> = state
        .files
        .iter()
        .map(|file| ListItem::new(file.clone()))
        .collect();

    let file_border_style = if state.mode == Mode::FileSelect {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let files_block = Block::default()
        .borders(Borders::ALL)
        .title("Artifacts (Files)")
        .border_style(file_border_style);

    let files_list = List::new(file_items)
        .block(files_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(files_list, left_chunks[1], &mut state.file_list_state);

    // Excavation View
    let right_area = chunks[1];
    let title = if let Some(idx) = state.file_list_state.selected() {
        if idx < state.files.len() {
            format!("Excavation: {}", state.files[idx])
        } else {
            "Excavation Site".to_string()
        }
    } else {
        "Excavation Site".to_string()
    };

    let exc_border_style = if state.mode == Mode::Excavation {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(exc_border_style);

    if let Some(fossil) = &state.current_fossil {
        let mut lines = Vec::new();
        let mut current_line = Vec::new();

        for (i, c) in fossil.displayed_text.chars().enumerate() {
            let style = if fossil.mask.get(i).copied().unwrap_or(true) {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(Color::Red) // Corrupted
            };

            if c == '\n' {
                lines.push(Line::from(current_line));
                current_line = Vec::new();
            } else {
                current_line.push(Span::styled(c.to_string(), style));
            }
        }
        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        let paragraph = Paragraph::new(lines).block(block).scroll((0, 0)); // TODO: Add scrolling state

        f.render_widget(paragraph, right_area);
    } else {
        let text = Paragraph::new(
            "Select a commit, then a file to begin excavation.\nPress <Enter> to load.",
        )
        .block(block);
        f.render_widget(text, right_area);
    }
}
