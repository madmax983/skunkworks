use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use crate::phonology::PhonologyEngine;
use crate::obfuscator::{DefinitionFinder, Obfuscator};
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use std::collections::HashMap;

pub struct App {
    original_code: String,
    evolved_code: String,
    history: Vec<String>,
    engine: PhonologyEngine,
    ast: syn::File,
    identifiers: Vec<String>,
    current_mapping: HashMap<String, String>,
}

impl App {
    pub fn new(code: &str) -> Self {
        let ast = syn::parse_file(code).expect("Failed to parse code");
        let mut finder = DefinitionFinder::new();
        finder.visit_file(&ast);

        let mut current_mapping = HashMap::new();
        for id in &finder.identifiers {
            current_mapping.insert(id.clone(), id.clone());
        }

        Self {
            original_code: code.to_string(),
            evolved_code: code.to_string(),
            history: vec!["Initial State".to_string()],
            engine: PhonologyEngine::grimms_law(),
            ast,
            identifiers: finder.identifiers.into_iter().collect(),
            current_mapping,
        }
    }

    pub fn on_tick(&mut self) {
    }

    pub fn evolve(&mut self) {
        let mut changes = Vec::new();
        let mut changed_count = 0;

        // Apply engine to CURRENT values
        let mut new_mapping = self.current_mapping.clone();
        for (original, current) in self.current_mapping.iter() {
            let new_val = self.engine.evolve(current);
            if new_val != *current {
                new_mapping.insert(original.clone(), new_val.clone());
                changes.push(format!("{} -> {}", current, new_val));
                changed_count += 1;
            }
        }
        self.current_mapping = new_mapping;

        if changed_count > 0 {
            self.history.push(format!("Generation {}: {} mutations", self.history.len(), changed_count));
            if changes.len() < 5 {
                for c in changes {
                    self.history.push(format!("  - {}", c));
                }
            } else {
                 self.history.push(format!("  - (and {} more)", changes.len() - 5));
            }

            // Re-render
            let mut new_ast = self.ast.clone();
            let mut obfuscator = Obfuscator { mapping: self.current_mapping.clone() };
            obfuscator.visit_file_mut(&mut new_ast);

            self.evolved_code = prettyplease::unparse(&new_ast);
        } else {
            self.history.push("No mutations occurred.".to_string());
        }
    }

    pub fn reset(&mut self) {
        // Reset mapping to identity
        let keys: Vec<String> = self.current_mapping.keys().cloned().collect();
        for k in keys {
            self.current_mapping.insert(k.clone(), k);
        }

        self.evolved_code = self.original_code.clone();
        self.history.push("Reset to original state.".to_string());
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> anyhow::Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => app.evolve(),
                    KeyCode::Char('r') => app.reset(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.area());

    let code_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let left_block = Block::default().title("Original Code").borders(Borders::ALL);
    let right_block = Block::default().title("Evolved Dialect").borders(Borders::ALL);

    let left_text = Paragraph::new(app.original_code.as_str())
        .block(left_block)
        .wrap(Wrap { trim: false });
    f.render_widget(left_text, code_chunks[0]);

    let right_text = Paragraph::new(app.evolved_code.as_str())
        .block(right_block)
        .wrap(Wrap { trim: false });
    f.render_widget(right_text, code_chunks[1]);

    let history_block = Block::default().title("Evolution Log (Space: Evolve, R: Reset, Q: Quit)").borders(Borders::ALL);
    let history_lines: Vec<Line> = app.history.iter().rev().take(10).rev().map(|s| Line::from(s.as_str())).collect();
    let history_text = Paragraph::new(history_lines).block(history_block);
    f.render_widget(history_text, chunks[1]);
}
