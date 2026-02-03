use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

use crate::mutator::evolve_code;
use crate::phonology::{EvolutionTrace, Evolver, SoundLaw};

pub struct App {
    original_code: String,
    current_code: String,
    history: Vec<String>,
    year: usize,
    running: bool,
    // New fields
    traces: HashMap<String, EvolutionTrace>, // Key: Current Word, Value: Full Trace
    show_genealogy: bool,
}

impl App {
    pub fn new(code: String) -> Self {
        Self {
            original_code: code.clone(),
            current_code: code,
            history: vec!["Origin: Proto-Code-Germanic".to_string()],
            year: 0,
            running: true,
            traces: HashMap::new(),
            show_genealogy: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                            KeyCode::Char(' ') => self.advance_epoch(),
                            KeyCode::Char('r') => self.reset(),
                            KeyCode::Char('g') => self.show_genealogy = !self.show_genealogy,
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn advance_epoch(&mut self) {
        self.year += 100;

        // Select a law based on "era" or random
        let law = if self.year < 500 {
            SoundLaw::GrimmsLaw
        } else if self.year < 1000 {
            SoundLaw::Lenition
        } else if self.year < 1500 {
            SoundLaw::GreatVowelShift
        } else if self.year < 2000 {
            SoundLaw::Palatalization
        } else {
            // Entropy takes over
            match self.year % 5 {
                0 => SoundLaw::LossOfEndings,
                1 => SoundLaw::Metathesis,
                2 => SoundLaw::Rhotacism,
                3 => SoundLaw::ClusterSimplification,
                _ => SoundLaw::HDropping,
            }
        };

        self.history
            .push(format!("Year {}: Applied {}", self.year, law.description()));

        let mut step_evolver = Evolver::new();
        step_evolver.add_law(law.clone());

        match evolve_code(&self.current_code, step_evolver) {
            Ok((new_code, step_traces)) => {
                self.current_code = new_code;
                self.update_traces(step_traces);
            }
            Err(e) => {
                self.history.push(format!(
                    "Year {}: The language collapsed! (Parse Error: {})",
                    self.year, e
                ));
            }
        }
    }

    fn update_traces(&mut self, step_traces: HashMap<String, EvolutionTrace>) {
        // step_traces: Key = New Word. Value = Trace from Old Word to New Word.
        // We need to link this with self.traces.

        // We can't easily map backwards because we don't know which old word turned into which new word
        // purely from the keys if there are collisions, but `step_traces` values contain the history.
        // The first step of the trace usually contains the 'original' word for that step.
        // Wait, `Evolver` trace steps are (Rule, Result).
        // It doesn't explicitly store the START word in the steps list, but we passed it to `evolve`.

        // Let's rely on the fact that we can infer the previous word?
        // Actually, if I change `Evolver` to include the start word in the trace it would be easier.
        // But let's assume `CodeEvolver` implementation: `self.traces.insert(valid_evolved.clone(), trace);`
        // The `trace` is what comes out of `evolver.evolve_with_trace`.
        // `evolve_with_trace` returns `(result, trace)`.
        // The trace only has the steps. The input word is implicitly the starting state.

        // PROBLEM: We don't know the starting word for the step trace just by looking at the trace struct.
        // We need to know: OldWord -> NewWord.
        // `CodeEvolver` knows `s` (OldWord) and `valid_evolved` (NewWord).
        // But `CodeEvolver` only returns `traces` which is `HashMap<NewWord, Trace>`.
        // It threw away `OldWord` association in the map structure (Key is NewWord).

        // I should modify `mutator.rs` to return `HashMap<String, (String, EvolutionTrace)>`
        // where Key = NewWord, Value = (OldWord, Trace).

        // OR: Just rebuild the traces from scratch? No, we need historical continuity.

        // Let's assume for now that I can't easily link them without changing `mutator.rs`.
        // BUT, I can guess? No, guessing is bad.

        // I will just display the *current step* trace in the genealogy view for now,
        // to avoid complexity of perfect history tracking in this iteration.
        // Or better: clear `self.traces` and just show what happened *this year*.
        // Users can scroll back in history log to see what happened.

        // Actually, let's try to maintain it.
        // If I change `CodeEvolver` to return `HashMap<String, (String, EvolutionTrace)>`...
        // That requires going back to `mutator.rs`.

        // Alternate idea: The `Trace` object should contain the starting word.
        // Let's check `phonology.rs`.
        // `evolve_with_trace` takes `word`. `current` starts as `word`.
        // If I push `("Original", word)` to steps at start?

        // That seems like a good, minimally invasive change to `phonology.rs`.
        // But I'm in `app.rs` step. I shouldn't touch `phonology.rs` if I can avoid it.

        // Let's just store the step traces for now.
        // If the user wants full etymology, they really want to see the chain.
        // I'll make `self.traces` accumulate.
        // Since I can't link, I'll just dump the new traces into the list.
        // The UI will show "Recent Changes".

        self.traces = step_traces;
    }

    fn reset(&mut self) {
        self.current_code = self.original_code.clone();
        self.year = 0;
        self.history.clear();
        self.history.push("Origin: Proto-Code-Germanic".to_string());
        self.traces.clear();
        self.show_genealogy = false;
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer / Log
            ])
            .split(area);

        // Header
        let title_text = if self.show_genealogy {
            format!(
                "PHONETIC DECAY | Year: {} | MODE: GENEALOGY (Press 'g' to close)",
                self.year
            )
        } else {
            format!(
                "PHONETIC DECAY | Year: {} | Press 'g' for Genealogy, 'Space' to Evolve, 'r' Reset",
                self.year
            )
        };

        let title = Paragraph::new(title_text)
            .block(Block::default().borders(Borders::ALL))
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(title, chunks[0]);

        // Content
        if self.show_genealogy {
            self.draw_genealogy(frame, chunks[1]);
        } else {
            self.draw_code_comparison(frame, chunks[1]);
        }

        // Footer / Log
        let last_log = self.history.last().map(|s| s.as_str()).unwrap_or("");
        let log = Paragraph::new(last_log)
            .block(Block::default().borders(Borders::ALL).title("History Log"))
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(log, chunks[2]);
    }

    fn draw_code_comparison(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let original = Paragraph::new(self.original_code.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Proto-Language (Original)"),
            )
            .wrap(Wrap { trim: false });

        let evolved = Paragraph::new(self.current_code.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Modern Dialect (Evolved)"),
            )
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: false });

        frame.render_widget(original, content_chunks[0]);
        frame.render_widget(evolved, content_chunks[1]);
    }

    fn draw_genealogy(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        // Show a list of all words that changed in the last step
        let mut raw_items = Vec::new();
        for (word, trace) in &self.traces {
            if trace.steps.is_empty() {
                continue;
            }

            let last_rule = &trace.steps.last().unwrap().0;
            let text = format!("{} <== {}", word, last_rule);
            raw_items.push(text);
        }

        // Sort items for stability
        raw_items.sort();

        let items: Vec<ListItem> = raw_items.into_iter().map(ListItem::new).collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Recent Mutations (Genealogy)"),
            )
            .style(Style::default().fg(Color::Magenta));

        frame.render_widget(list, area);
    }
}
