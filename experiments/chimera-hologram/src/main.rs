mod hologram;

use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use hologram::HolographicDna;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::{io, time::{Duration, Instant}};

struct App {
    dna: HolographicDna,
    vm: Option<ChimeraVM>,
    phase_shift: f64,
    running: bool,
    genome_size: usize,
    opcodes: Vec<OpCode>,
}

impl App {
    fn new() -> Self {
        let size = 64; // Power of 2 for FFT efficiency
        let dna = HolographicDna::new_random(size);
        let mut app = Self {
            dna,
            vm: None,
            phase_shift: 0.0,
            running: true,
            genome_size: size,
            opcodes: Vec::new(),
        };
        app.update_vm();
        app
    }

    fn update_vm(&mut self) {
        // Decode the hologram with current phase shift
        self.opcodes = self.dna.decode(self.phase_shift);

        // Create DNA structure for ChimeraVM
        // DNA = Helix { strands: [ Strand { genes: [ Gene { op, args: [] } ] } ] }
        let genes: Vec<Gene> = self.opcodes.iter()
            .map(|op| Gene {
                op: op.clone(),
                args: vec![] // For simplicity, no arguments or random arguments could be added later
            })
            .collect();

        let helix = Helix {
            strands: vec![Strand { genes }],
        };

        let dna_struct = Dna { helix };

        // Initialize VM
        self.vm = Some(ChimeraVM::new(dna_struct));
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key(key),
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Some(vm) = &mut self.vm {
                    // Step the VM
                    // We catch panics or errors if step returns Result
                    // chimera-lang VM usually has a step() method that might not return Result
                    // but let's assume it's safe or handle it.
                    // Based on docs: vm.step()
                    vm.step();
                }
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Left => {
                self.phase_shift -= 0.1;
                self.update_vm();
            }
            KeyCode::Right => {
                self.phase_shift += 0.1;
                self.update_vm();
            }
            KeyCode::Char('r') => {
                self.dna = HolographicDna::new_random(self.genome_size);
                self.update_vm();
            }
            _ => {}
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(10),   // Main Content
                Constraint::Length(3), // Status/Controls
            ])
            .split(f.area());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Hologram + Genome
                Constraint::Percentage(50), // VM State
            ])
            .split(chunks[1]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Hologram Spectrum
                Constraint::Percentage(50), // Decoded Genome
            ])
            .split(main_chunks[0]);

        // Title
        let title = Paragraph::new(" 🧬 CHIMERA HOLOGRAM - Subjective Code Execution ")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // 1. Hologram Spectrum (Frequency Domain)
        // We plot the magnitude of the coefficients
        // We need to construct data for BarChart: Vec<(&str, u64)>
        // Since we can't easily make str refs for indices, we'll just plot bars.
        // Actually BarChart needs labels.
        // We can convert indices to strings.

        let mut bar_data: Vec<(String, u64)> = Vec::new();
        // Since we have 64 coefficients, displaying all might be crowded.
        // Let's display first 32 or bin them.
        for (i, c) in self.dna.coefficients.iter().enumerate().take(32) {
            let mag = (c.norm() * 100.0) as u64;
            bar_data.push((format!("{}", i), mag));
        }

        let bar_refs: Vec<(&str, u64)> = bar_data.iter().map(|(s, v)| (s.as_str(), *v)).collect();

        let barchart = BarChart::default()
            .block(Block::default().title(" Hologram (Frequency Magnitude) ").borders(Borders::ALL))
            .data(&bar_refs)
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Magenta))
            .value_style(Style::default().fg(Color::White));
        f.render_widget(barchart, left_chunks[0]);

        // 2. Decoded Genome
        let items: Vec<ListItem> = self.opcodes.iter().enumerate().map(|(i, op)| {
            let content = format!("{:03}: {:?}", i, op);
            let style = if let Some(vm) = &self.vm {
                // Highlight current instruction
                // accessing vm.ip or similar?
                // ChimeraVM usually has `ip` (Instruction Pointer).
                // Let's assumes it has `ip.0` as strand index and `ip.1` as gene index.
                // We only have 1 strand.
                if vm.ip.1 == i {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                }
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        }).collect();

        let list = List::new(items)
            .block(Block::default().title(" Decoded Genome (Phenotype) ").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_widget(list, left_chunks[1]);

        // 3. VM State
        if let Some(vm) = &self.vm {
            // Let's manually format key fields since ChimeraVM might not derive Debug
            let mut lines = Vec::new();
            lines.push(Line::from(vec![
                Span::raw("Energy: "),
                Span::styled(format!("{}", vm.energy), Style::default().fg(Color::Green))
            ]));
            lines.push(Line::from(vec![
                Span::raw("IP: "),
                Span::styled(format!("{:?}", vm.ip), Style::default().fg(Color::Blue))
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from("Stack (Top 10):"));

            for (i, val) in vm.stack.iter().rev().take(10).enumerate() {
                lines.push(Line::from(format!(" {}: {}", i, val)));
            }

            lines.push(Line::from(""));
            lines.push(Line::from("Output Log (Last 5):"));
            for log in vm.output.iter().rev().take(5) {
                lines.push(Line::from(Span::styled(log, Style::default().fg(Color::Gray))));
            }

            let state_paragraph = Paragraph::new(lines)
                .block(Block::default().title(" VM State ").borders(Borders::ALL))
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(state_paragraph, main_chunks[1]);
        }

        // Controls
        let controls = Paragraph::new(format!(
            "Phase Shift: {:.2} rad | [Left/Right] Adjust Phase | [R]andomize | [Q]uit",
            self.phase_shift
        ))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(controls, chunks[2]);
    }
}

fn main() -> Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let mut app = App::new();
    let res = app.run(&mut terminal);

    // cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
