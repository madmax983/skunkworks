use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

pub fn run_tui(mut vm: ChimeraVM) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut vm);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    vm: &mut ChimeraVM,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.area()); // Changed f.size() to f.area() for newer ratatui

            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30), // Stack
                    Constraint::Percentage(40), // Petri Dish
                    Constraint::Percentage(30), // Output
                ].as_ref())
                .split(chunks[1]);

            // Genome View
            let helix = &vm.dna.helix;
            let mut strand_items = Vec::new();

            for (s_idx, strand) in helix.strands.iter().enumerate() {
                for (g_idx, gene) in strand.genes.iter().enumerate() {
                    let content = format!("{}({:?})", gene.name, gene.args);
                    let mut style = Style::default();

                    if s_idx == vm.ip.0 && g_idx == vm.ip.1 {
                        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                        strand_items.push(ListItem::new(format!("> {}", content)).style(style));
                    } else if s_idx < vm.ip.0 || (s_idx == vm.ip.0 && g_idx < vm.ip.1) {
                        style = style.fg(Color::DarkGray); // Changed Gray to DarkGray
                        strand_items.push(ListItem::new(format!("  {}", content)).style(style));
                    } else {
                        strand_items.push(ListItem::new(format!("  {}", content)).style(style));
                    }
                }
                strand_items.push(ListItem::new("-------------------"));
            }

            let genome_list = List::new(strand_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Genome (Space: Step, M: Mutate, X: Recombine, Q: Quit)"),
            );
            f.render_widget(genome_list, chunks[0]);

            // Cytoplasm (Stack)
            let stack_items: Vec<ListItem> = vm
                .stack
                .iter()
                .rev()
                .map(|val| ListItem::new(format!("{}", val)))
                .collect();

            let stack_list = List::new(stack_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Cytoplasm (Stack)"),
            );
            f.render_widget(stack_list, right_chunks[0]);

            // Petri Dish (Grid)
            let mut grid_str = String::new();
            for row in &vm.grid {
                for cell in row {
                    let char_rep = match cell {
                        crate::vm::Value::Int(0) => '.',
                        crate::vm::Value::Int(n) => {
                             if *n > 0 && *n < 10 {
                                 char::from_digit(*n as u32, 10).unwrap()
                             } else {
                                 '#'
                             }
                        },
                        _ => '?',
                    };
                    grid_str.push(char_rep);
                    grid_str.push(' ');
                }
                grid_str.push('\n');
            }
            let petri_dish = Paragraph::new(grid_str)
                .block(Block::default().borders(Borders::ALL).title("Petri Dish"));
             f.render_widget(petri_dish, right_chunks[1]);

            // Output
            let output_items: Vec<ListItem> = vm
                .output
                .iter()
                .rev()
                .map(|val| ListItem::new(val.clone()))
                .collect();

            let output_list = List::new(output_items)
                .block(Block::default().borders(Borders::ALL).title("Output"));
            f.render_widget(output_list, right_chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => vm.step(),
                    KeyCode::Char('m') => vm.mutate(),
                    KeyCode::Char('x') => vm.recombine(),
                    _ => {}
                }
            }
        }
    }
}
