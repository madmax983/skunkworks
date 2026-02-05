use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pest::Parser;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

enum InputMode {
    Normal,
    Editing,
}

struct AppState {
    input_mode: InputMode,
    selected_strand: usize,
    selected_gene: usize,
    input_buffer: String,
    status_msg: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            selected_strand: 0,
            selected_gene: 0,
            input_buffer: String::new(),
            status_msg: String::new(),
        }
    }
}

pub fn run_tui(mut vm: ChimeraVM) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();

    let res = run_app(&mut terminal, &mut vm, &mut app_state);

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
    app_state: &mut AppState,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| {
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.area());

            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(main_chunks[0]);

            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(main_chunks[1]);

            // Genome View
            let helix = &vm.dna.helix;
            let mut strand_items = Vec::new();

            for (s_idx, strand) in helix.strands.iter().enumerate() {
                #[cfg(feature = "cortex")]
                {
                    let mut header = format!("Strand {}", s_idx);
                    if s_idx < vm.activation_levels.len() {
                        header.push_str(&format!(" ⚡{}", vm.activation_levels[s_idx]));
                    }
                    if s_idx < vm.synapse_map.len() && !vm.synapse_map[s_idx].is_empty() {
                        header.push_str(&format!(" -> {:?}", vm.synapse_map[s_idx]));
                    }
                    strand_items.push(ListItem::new(Span::styled(
                        header,
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    )));
                }

                #[cfg(not(feature = "cortex"))]
                {
                    strand_items.push(ListItem::new(Span::styled(
                         format!("Strand {}", s_idx),
                         Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    )));
                }

                for (g_idx, gene) in strand.genes.iter().enumerate() {
                    let content = format!("{}({:?})", gene.op, gene.args);
                    let mut style = Style::default();
                    let mut prefix = "  ";

                    // Logic for execution highlighting
                    #[cfg(feature = "nova")]
                    if vm.epigenome.contains(&(s_idx, g_idx)) {
                        style = style.fg(Color::Blue);
                    }

                    if s_idx == vm.ip.0 && g_idx == vm.ip.1 {
                        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                        #[cfg(feature = "nova")]
                        if vm.epigenome.contains(&(s_idx, g_idx)) {
                            style = style.bg(Color::Blue);
                        }
                        prefix = "> ";
                    } else if (s_idx < vm.ip.0 || (s_idx == vm.ip.0 && g_idx < vm.ip.1))
                        && style.fg != Some(Color::Blue)
                    {
                        style = style.fg(Color::DarkGray);
                    }

                    // Logic for Editor Selection highlighting
                    if s_idx == app_state.selected_strand && g_idx == app_state.selected_gene {
                         if let InputMode::Editing = app_state.input_mode {
                              style = style.bg(Color::Red).fg(Color::White);
                              prefix = "E ";
                         } else {
                              style = style.bg(Color::White).fg(Color::Black);
                              prefix = "* ";
                         }
                    }

                    strand_items.push(ListItem::new(format!("{}{}", prefix, content)).style(style));
                }
                strand_items.push(ListItem::new("-------------------"));
            }

            // Add a placeholder for adding new genes at the end of strand?
            // For now, keep it simple. Only editing existing genes.

            let chaos_status = if vm.chaos_mode { "ON" } else { "OFF" };
            let title = match app_state.input_mode {
                InputMode::Normal => format!(
                    "Genome (Space: Step, M: Mutate, C: Chaos[{}], Arrows: Nav, Enter: Edit, Q: Quit)",
                    chaos_status
                ),
                InputMode::Editing => format!(
                    "Editing {}:{} (Enter: Commit, Esc: Cancel) - {}",
                    app_state.selected_strand, app_state.selected_gene, app_state.input_buffer
                ),
            };

            let genome_list = List::new(strand_items).block(
                Block::default().borders(Borders::ALL).title(title),
            );
            f.render_widget(genome_list, left_chunks[0]);

            // Petri Dish (Grid)
            let mut grid_lines = Vec::new();

            for y in 0..16 {
                let mut line_spans = Vec::new();
                for x in 0..16 {
                    let val = &vm.grid[y][x];

                    #[allow(unused_mut)]
                    let (mut char_rep, mut style) = match val {
                        crate::vm::Value::Int(0) => {
                            (".".to_string(), Style::default().fg(Color::DarkGray))
                        }
                        crate::vm::Value::Int(n) => (
                            format!("{}", (n.abs() % 10)),
                            Style::default().fg(Color::Green),
                        ),
                        crate::vm::Value::Str(s) => {
                            let symbol = match s.as_str() {
                                "virus" => "V",
                                "incubate" => "I",
                                "push" => "^",
                                "add" => "+",
                                "sub" => "-",
                                "mul" => "*",
                                "div" => "/",
                                "jump" | "jump_s" => "J",
                                "brz" | "brz_s" => "?",
                                "photosynthesize" => "P",
                                "consume" => "C",
                                "g_read" => "R",
                                "g_write" => "W",
                                "mitosis" => "M",
                                "apoptosis" => "X",
                                _ => &s[0..1],
                            };
                            (symbol.to_string(), Style::default().fg(Color::Cyan))
                        }
                    };

                    #[cfg(feature = "nova")]
                    {
                        let h = vm.hormone_grid[y][x];
                        let r = h[0].clamp(0, 255) as u8;
                        let g = h[1].clamp(0, 255) as u8;
                        let b = h[2].clamp(0, 255) as u8;
                        if r > 0 || g > 0 || b > 0 {
                            style = style.bg(Color::Rgb(r, g, b));
                            // Ensure foreground is visible if background is bright
                            // Simple heuristic: if sum > 300, use black fg
                            if (r as u16 + g as u16 + b as u16) > 300 {
                                style = style.fg(Color::Black);
                            }
                        }

                        if vm.waste_grid[y][x] > 50 {
                            style = style.add_modifier(Modifier::CROSSED_OUT);
                            if vm.waste_grid[y][x] > 100 {
                                style = style.fg(Color::Red);
                            }
                        }

                        if let Some(organelle) =
                            vm.organelles.iter().find(|o| o.context_loc == (y, x))
                        {
                            let (color, char_code) = match organelle.kind {
                                crate::vm::nova::OrganelleType::Chloroplast => (Color::Green, "C"),
                                crate::vm::nova::OrganelleType::Mitochondria => (Color::Red, "M"),
                                crate::vm::nova::OrganelleType::Lysosome => (Color::Magenta, "L"),
                                crate::vm::nova::OrganelleType::Ribosome => (Color::Cyan, "R"),
                                crate::vm::nova::OrganelleType::Worker => (Color::White, "O"),
                            };

                            style = style
                                .bg(color)
                                .fg(Color::Black)
                                .add_modifier(Modifier::BOLD);
                            if char_rep == "." {
                                char_rep = char_code.to_string();
                            }
                        }

                        if let Some(target) = vm.sonar_target {
                            if target == (y, x) {
                                style = style
                                    .bg(Color::Yellow)
                                    .fg(Color::Black)
                                    .add_modifier(Modifier::SLOW_BLINK);
                            }
                        }

                        if vm.portals.contains_key(&(y, x)) {
                            char_rep = "@".to_string();
                            style = style
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
                        }
                    }

                    line_spans.push(Span::styled(char_rep, style));
                    line_spans.push(Span::raw(" ")); // Spacing
                }
                grid_lines.push(Line::from(line_spans));
            }

            #[cfg(feature = "nova")]
            let topology_name = format!("{:?}", vm.topology);
            #[cfg(not(feature = "nova"))]
            let topology_name = "Classic";

            let grid_title = format!("Petri Dish (16x16) - {}", topology_name);

            let grid_paragraph = Paragraph::new(grid_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(grid_title),
            );
            f.render_widget(grid_paragraph, left_chunks[1]);

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
                    .title(format!("Cytoplasm (Stack) - Energy: {}", vm.energy)),
            );
            f.render_widget(stack_list, right_chunks[0]);

            // Output
            let mut output_items: Vec<ListItem> = vm
                .output
                .iter()
                .rev()
                .map(|val| ListItem::new(val.clone()))
                .collect();

            if !app_state.status_msg.is_empty() {
                output_items.insert(0, ListItem::new(Span::styled(
                    format!("STATUS: {}", app_state.status_msg),
                    Style::default().fg(Color::Yellow)
                )));
            }

            let output_list = List::new(output_items)
                .block(Block::default().borders(Borders::ALL).title("Output"));
            f.render_widget(output_list, right_chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Handle Editing Mode
                if let InputMode::Editing = app_state.input_mode {
                    match key.code {
                        KeyCode::Enter => {
                            // Parse and commit
                            match ChimeraParser::parse(Rule::gene, &app_state.input_buffer) {
                                Ok(mut pairs) => {
                                    let pair = pairs.next().unwrap();
                                    // We need to convert pair to Gene.
                                    // Gene::from_pair is available in crate::ast::Gene
                                    let gene = Gene::from_pair(pair);

                                    // Update VM
                                    if app_state.selected_strand < vm.dna.helix.strands.len()
                                        && app_state.selected_gene
                                            < vm.dna.helix.strands[app_state.selected_strand]
                                                .genes
                                                .len()
                                    {
                                        vm.dna.helix.strands[app_state.selected_strand].genes
                                            [app_state.selected_gene] = gene;
                                        app_state.status_msg =
                                            "Gene updated successfully".to_string();
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                Err(e) => {
                                    app_state.status_msg = format!("Parse Error: {}", e);
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Char(c) => {
                            app_state.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.input_buffer.pop();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Normal Mode
                #[cfg(feature = "nova")]
                if let KeyCode::Char(c) = key.code {
                    // Only handle receptor input if not a control key
                    if c != 'q' && c != ' ' && c != 'm' && c != 'c' && vm.handle_input(c) {
                        continue;
                    }
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => vm.step(),
                    KeyCode::Char('m') => vm.mutate(),
                    KeyCode::Char('c') => vm.chaos_mode = !vm.chaos_mode,
                    KeyCode::Down => {
                        // Move to next gene or next strand
                        let s_len = vm.dna.helix.strands.len();
                        if s_len > 0 {
                            let g_len = vm.dna.helix.strands[app_state.selected_strand].genes.len();
                            if app_state.selected_gene + 1 < g_len {
                                app_state.selected_gene += 1;
                            } else {
                                // Next strand
                                if app_state.selected_strand + 1 < s_len {
                                    app_state.selected_strand += 1;
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                    }
                    KeyCode::Up => {
                        if app_state.selected_gene > 0 {
                            app_state.selected_gene -= 1;
                        } else {
                            // Prev strand
                            if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                                let g_len =
                                    vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                if g_len > 0 {
                                    app_state.selected_gene = g_len - 1;
                                } else {
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                    }
                    KeyCode::Right => {
                        // Maybe jump strands? For now just same as down/up or maybe nothing
                    }
                    KeyCode::Left => {
                        // Same
                    }
                    KeyCode::Enter => {
                        // Start Editing
                        if app_state.selected_strand < vm.dna.helix.strands.len() {
                            let g_len = vm.dna.helix.strands[app_state.selected_strand].genes.len();
                            if app_state.selected_gene < g_len {
                                app_state.input_mode = InputMode::Editing;
                                // Pre-fill buffer with current gene?
                                let gene = &vm.dna.helix.strands[app_state.selected_strand].genes
                                    [app_state.selected_gene];
                                // We don't have a gene to string converter easily accessible that matches parser format perfecty
                                // But we can format it manually.
                                // gene.op is Display, args are Debug.
                                // Let's rely on user typing from scratch or empty buffer for now,
                                // or try to reconstruct.
                                // Format: name(arg1 arg2)
                                let mut s = format!("{}(", gene.op);
                                for (i, arg) in gene.args.iter().enumerate() {
                                    if i > 0 {
                                        s.push(' ');
                                    }
                                    match arg {
                                        crate::ast::Nucleotide::Number(n) => {
                                            s.push_str(&n.to_string())
                                        }
                                        crate::ast::Nucleotide::String(str_val) => {
                                            s.push_str(&format!("\"{}\"", str_val))
                                        }
                                        crate::ast::Nucleotide::Identifier(id) => s.push_str(id),
                                    }
                                }
                                s.push(')');
                                app_state.input_buffer = s;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
