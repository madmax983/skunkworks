use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::str::FromStr;

mod entropy;
mod git;

use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::ChimeraVM,
};
use entropy::{fossilize, Fossil};
use git::{get_file_content, get_files_in_commit, load_history, Commit};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List fossil history (Git Log) in a table
    #[arg(short, long)]
    list: bool,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FossilMode {
    CommitSelect,
    FileSelect,
    Excavation,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ActivePane {
    Fossil,
    Chimera,
}

pub struct AppState {
    // Fossil State
    pub commits: Vec<Commit>,
    pub commit_list_state: ListState,
    pub files: Vec<String>,
    pub file_list_state: ListState,
    pub current_fossil: Option<Fossil>,
    pub fossil_mode: FossilMode,

    // Chimera State
    pub vm: Option<ChimeraVM>,

    // Global State
    pub active_pane: ActivePane,
    pub log_messages: Vec<String>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let commits = load_history().unwrap_or_default();
        let mut commit_state = ListState::default();
        if !commits.is_empty() {
            commit_state.select(Some(0));
        }

        Ok(Self {
            commits,
            commit_list_state: commit_state,
            files: vec![],
            file_list_state: ListState::default(),
            current_fossil: None,
            fossil_mode: FossilMode::CommitSelect,
            vm: None,
            active_pane: ActivePane::Fossil,
            log_messages: vec!["Welcome to the Jurassic Code Park.".to_string()],
        })
    }

    fn log(&mut self, msg: &str) {
        self.log_messages.push(msg.to_string());
        if self.log_messages.len() > 10 {
            self.log_messages.remove(0);
        }
    }

    fn next_commit(&mut self) {
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
        self.refresh_files();
    }

    fn previous_commit(&mut self) {
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
        self.refresh_files();
    }

    fn refresh_files(&mut self) {
        if let Some(idx) = self.commit_list_state.selected() {
            if let Some(commit) = self.commits.get(idx) {
                if let Ok(files) = get_files_in_commit(&commit.hash) {
                    self.files = files;
                    self.file_list_state.select(Some(0));
                    self.current_fossil = None;
                }
            }
        }
    }

    fn next_file(&mut self) {
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

    fn previous_file(&mut self) {
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

    fn load_fossil(&mut self) {
        if let Some(c_idx) = self.commit_list_state.selected() {
            if let Some(f_idx) = self.file_list_state.selected() {
                let commit = &self.commits[c_idx];
                if let Some(path) = self.files.get(f_idx) {
                    if let Ok(content) = get_file_content(&commit.hash, path) {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        commit.hash.hash(&mut hasher);
                        let seed = hasher.finish();
                        // Age factor: index / 50.0 (older commits = more corruption)
                        let age_factor = (c_idx as f64) / 50.0;
                        let fossil = fossilize(&content, age_factor, seed);
                        self.current_fossil = Some(fossil);
                        self.log(&format!("Excavated fossil from {}", commit.hash));
                    }
                }
            }
        }
    }

    fn splice_dna(&mut self) {
        if let Some(fossil) = &self.current_fossil {
            let dna = splice_text_to_dna(&fossil.displayed_text);
            if dna.helix.strands.is_empty() || dna.helix.strands[0].genes.is_empty() {
                self.log("Splicing failed: No viable genetic material found.");
            } else {
                let genes_count = dna
                    .helix
                    .strands
                    .iter()
                    .map(|s| s.genes.len())
                    .sum::<usize>();
                self.log(&format!(
                    "Resurrection successful! Spliced {} genes.",
                    genes_count
                ));
                self.vm = Some(ChimeraVM::new(dna));
                self.active_pane = ActivePane::Chimera;
            }
        } else {
            self.log("No fossil loaded to splice.");
        }
    }
}

fn splice_text_to_dna(text: &str) -> Dna {
    // Improve splicing by treating punctuation as separators
    let clean_text = text.replace(
        &['(', ')', '[', ']', '{', '}', ',', ';', '.', ':', '!', '?'][..],
        " ",
    );
    let tokens: Vec<&str> = clean_text.split_whitespace().collect();
    let mut genes = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens[i];
        // Clean token of punctuation potentially? For now strict match.
        // OpCode::from_str is usually case sensitive.
        // We try to match token to OpCode.

        // Remove trailing punctuation for better matching (already done by replace above mostly)
        let clean_token: String = token
            .chars()
            .filter(|c| c.is_alphabetic() || *c == '_')
            .collect();

        if let Ok(op) = OpCode::from_str(&clean_token) {
            let mut args = Vec::new();

            // Check if op needs args
            // This is a heuristic as OpCode doesn't expose `arity` publicly usually?
            // Wait, we can infer from `read_args` in compiler or just guess.
            // Common ops: Push(1), Jump(1), Brz(1), GRead(2), GWrite(3), etc.
            // Actually, ChimeraVM takes args as part of the gene struct.
            // If the *next* token is a number or string, we take it as an arg.

            if i + 1 < tokens.len() {
                let next_token = tokens[i + 1];
                if let Ok(n) = next_token.parse::<i64>() {
                    args.push(Nucleotide::Number(n));
                    i += 1;
                } else if next_token.starts_with('"') && next_token.ends_with('"') {
                    // String literal
                    let s = next_token.trim_matches('"').to_string();
                    args.push(Nucleotide::String(s));
                    i += 1;
                }
            }

            genes.push(Gene { op, args });
        }
        i += 1;
    }

    // Split large gene sequence into strands of random length?
    // Or just one big strand. One big strand is safer.
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list {
        run_cli_list()?;
        return Ok(());
    }

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new()?;
    // Initial load
    app_state.refresh_files();

    let res = run_app(&mut terminal, &mut app_state);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_cli_list() -> Result<()> {
    let commits = load_history()?;
    let mut table = comfy_table::Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_header(vec![
            "Hash", "Date", "Author", "Message", "Entropy", "Verified",
        ]);

    for commit in commits {
        // Simulate a "Verified" status based on commit hash or message properties
        // For visual demonstration of "True" as Green
        let is_verified = commit.hash.chars().next().unwrap_or('0').is_numeric(); // Arbitrary check
        let verified_str = if is_verified { "✔" } else { "✘" };

        let verified_cell = if is_verified {
            comfy_table::Cell::new(verified_str).fg(comfy_table::Color::Green)
        } else {
            comfy_table::Cell::new(verified_str).fg(comfy_table::Color::Red)
        };

        // Calculate Entropy (Mock)
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        commit.hash.hash(&mut hasher);
        let entropy_val = (hasher.finish() % 100) as u8;

        let entropy_cell = comfy_table::Cell::new(format!("{}%", entropy_val));
        let entropy_cell = if entropy_val > 70 {
            entropy_cell.fg(comfy_table::Color::Red)
        } else if entropy_val > 30 {
            entropy_cell.fg(comfy_table::Color::Yellow)
        } else {
            entropy_cell.fg(comfy_table::Color::Green)
        };

        table.add_row(vec![
            comfy_table::Cell::new(&commit.hash[0..7]),
            comfy_table::Cell::new(commit.date.to_string()),
            comfy_table::Cell::new(&commit.author),
            comfy_table::Cell::new(&commit.message),
            entropy_cell,
            verified_cell,
        ]);
    }

    println!("{table}");
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> Result<()> {
    loop {
        terminal.draw(|f| draw(f, state))?;

        if let Event::Key(key) = event::read()? {
            match state.active_pane {
                ActivePane::Fossil => handle_fossil_input(key, state)?,
                ActivePane::Chimera => handle_chimera_input(key, state)?,
            }

            // Global keys
            match key.code {
                KeyCode::Tab => {
                    state.active_pane = match state.active_pane {
                        ActivePane::Fossil => ActivePane::Chimera,
                        ActivePane::Chimera => ActivePane::Fossil,
                    };
                }
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn handle_fossil_input(key: event::KeyEvent, state: &mut AppState) -> Result<()> {
    match state.fossil_mode {
        FossilMode::CommitSelect => match key.code {
            KeyCode::Down | KeyCode::Char('j') => state.next_commit(),
            KeyCode::Up | KeyCode::Char('k') => state.previous_commit(),
            KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                state.fossil_mode = FossilMode::FileSelect
            }
            _ => {}
        },
        FossilMode::FileSelect => match key.code {
            KeyCode::Left | KeyCode::Char('h') => state.fossil_mode = FossilMode::CommitSelect,
            KeyCode::Down | KeyCode::Char('j') => state.next_file(),
            KeyCode::Up | KeyCode::Char('k') => state.previous_file(),
            KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                state.load_fossil();
                state.fossil_mode = FossilMode::Excavation;
            }
            _ => {}
        },
        FossilMode::Excavation => match key.code {
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Esc => {
                state.fossil_mode = FossilMode::FileSelect
            }
            KeyCode::Enter => state.splice_dna(),
            _ => {}
        },
    }
    Ok(())
}

fn handle_chimera_input(key: event::KeyEvent, state: &mut AppState) -> Result<()> {
    if let Some(vm) = &mut state.vm {
        match key.code {
            KeyCode::Char(' ') => vm.step(),
            KeyCode::Char('m') => vm.mutate(),
            _ => {}
        }
    }
    Ok(())
}

fn draw(f: &mut Frame, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    draw_fossil_view(f, state, chunks[0]);
    draw_chimera_view(f, state, chunks[1]);
}

fn draw_fossil_view(f: &mut Frame, state: &mut AppState, area: ratatui::layout::Rect) {
    let active = state.active_pane == ActivePane::Fossil;
    let block_style = if active {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let container = Block::default()
        .borders(Borders::ALL)
        .title("Fossil Record")
        .border_style(block_style);
    f.render_widget(container.clone(), area);

    let inner_area = container.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(inner_area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[0]);

    // Commits
    let commits: Vec<ListItem> = state
        .commits
        .iter()
        .map(|c| ListItem::new(format!("{} - {}", &c.hash[0..7], c.message)))
        .collect();

    let commit_style = if state.fossil_mode == FossilMode::CommitSelect {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let commit_list = List::new(commits)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Strata")
                .border_style(commit_style),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );
    f.render_stateful_widget(commit_list, top_chunks[0], &mut state.commit_list_state);

    // Files
    let files: Vec<ListItem> = state
        .files
        .iter()
        .map(|s| ListItem::new(s.clone()))
        .collect();
    let file_style = if state.fossil_mode == FossilMode::FileSelect {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let file_list = List::new(files)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Artifacts")
                .border_style(file_style),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );
    f.render_stateful_widget(file_list, top_chunks[1], &mut state.file_list_state);

    // Content
    let content_style = if state.fossil_mode == FossilMode::Excavation {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title("Excavation Site")
        .border_style(content_style);

    if let Some(fossil) = &state.current_fossil {
        let mut spans = Vec::new();
        for (i, c) in fossil.displayed_text.chars().enumerate() {
            let style = if fossil.mask.get(i).copied().unwrap_or(true) {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(Color::Red)
            };
            spans.push(Span::styled(c.to_string(), style));
        }

        // Wrap somewhat simply
        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let width = chunks[1].width as usize - 2;

        for span in spans {
            if span.content == "\n" || current_line.len() >= width {
                lines.push(Line::from(current_line));
                current_line = Vec::new();
            }
            if span.content != "\n" {
                current_line.push(span);
            }
        }
        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        let p = Paragraph::new(lines).block(content_block);
        f.render_widget(p, chunks[1]);
    } else {
        let p = Paragraph::new("Select a file to excavate...").block(content_block);
        f.render_widget(p, chunks[1]);
    }
}

fn draw_chimera_view(f: &mut Frame, state: &mut AppState, area: ratatui::layout::Rect) {
    let active = state.active_pane == ActivePane::Chimera;
    let block_style = if active {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let container = Block::default()
        .borders(Borders::ALL)
        .title("Reanimation Chamber")
        .border_style(block_style);
    f.render_widget(container.clone(), area);
    let inner_area = container.inner(area);

    if let Some(vm) = &state.vm {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(inner_area);

        // Grid (Petri Dish)
        let mut grid_lines = Vec::new();
        for y in 0..16 {
            let mut spans = Vec::new();
            for x in 0..16 {
                let val = &vm.grid[y][x];
                let (s, style) = match val {
                    chimera_lang::vm::Value::Int(0) => (".", Style::default().fg(Color::DarkGray)),
                    chimera_lang::vm::Value::Int(_) => ("#", Style::default().fg(Color::Green)),
                    _ => ("?", Style::default().fg(Color::Yellow)),
                };
                spans.push(Span::styled(s, style));
                spans.push(Span::raw(" "));
            }
            grid_lines.push(Line::from(spans));
        }
        let grid =
            Paragraph::new(grid_lines).block(Block::default().borders(Borders::ALL).title("Grid"));
        f.render_widget(grid, chunks[0]);

        // Stack & Info
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        let stack_items: Vec<ListItem> = vm
            .stack
            .iter()
            .rev()
            .map(|v| ListItem::new(format!("{}", v)))
            .collect();
        let stack =
            List::new(stack_items).block(Block::default().borders(Borders::ALL).title("Stack"));
        f.render_widget(stack, bottom_chunks[0]);

        let info_text = vec![
            Line::from(format!("Energy: {}", vm.energy)),
            Line::from(format!("IP: {:?}", vm.ip)),
            Line::from(format!("Strands: {}", vm.dna.helix.strands.len())),
        ];
        let info = Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Vital Signs"));
        f.render_widget(info, bottom_chunks[1]);
    } else {
        let p = Paragraph::new("Awaiting genetic material...").block(Block::default());
        f.render_widget(p, inner_area);
    }

    // Log overlay at bottom
    if !state.log_messages.is_empty() {
        let log_height = 3;
        let log_area = ratatui::layout::Rect {
            x: area.x,
            y: area.y + area.height - log_height,
            width: area.width,
            height: log_height,
        };
        // Clear background
        f.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            log_area,
        );

        let msg = state.log_messages.last().unwrap();
        let p = Paragraph::new(Span::styled(msg, Style::default().fg(Color::Yellow)))
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(p, log_area);
    }
}
