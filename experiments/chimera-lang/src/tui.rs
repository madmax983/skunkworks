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
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline},
    Terminal,
};
use std::io;

#[derive(Debug, PartialEq)]
pub(crate) enum ViewMode {
    Genome,
    Grid,
    #[cfg(feature = "biophysics")]
    Cortex,
}

enum InputMode {
    Normal,
    Editing,
}

pub(crate) struct AppState {
    pub(crate) view_mode: ViewMode,
    input_mode: InputMode,
    selected_strand: usize,
    selected_gene: usize,
    grid_cursor: (usize, usize),
    input_buffer: String,
    status_msg: String,
    #[cfg(feature = "biophysics")]
    pub(crate) voltage_history: Vec<u64>,
    #[cfg(feature = "biophysics")]
    pub(crate) selected_neuron_coords: Option<(usize, usize)>,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            view_mode: ViewMode::Genome,
            input_mode: InputMode::Normal,
            selected_strand: 0,
            selected_gene: 0,
            grid_cursor: (0, 0),
            input_buffer: String::new(),
            status_msg: String::new(),
            #[cfg(feature = "biophysics")]
            voltage_history: Vec::with_capacity(100),
            #[cfg(feature = "biophysics")]
            selected_neuron_coords: None,
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

fn parse_grid_value(s: &str) -> crate::vm::Value {
    if s.is_empty() {
        return crate::vm::Value::Int(0);
    }
    if let Ok(i) = s.parse::<i64>() {
        crate::vm::Value::Int(i)
    } else {
        crate::vm::Value::Str(s.to_string())
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        #[cfg(feature = "biophysics")]
        if let Some(coord) = app_state.selected_neuron_coords {
            if let Some(neuron) = vm.neurons.get(&coord) {
                // Push voltage (mapped to u64 for Sparkline)
                // V is approx -100 to +50. Shift by +100.
                let v_norm = (neuron.v + 100.0).clamp(0.0, 200.0) as u64;
                if app_state.voltage_history.len() >= 100 {
                    app_state.voltage_history.remove(0);
                }
                app_state.voltage_history.push(v_norm);
            }
        }

        terminal.draw(|f| {
            // Handle Cortex View
            #[cfg(feature = "biophysics")]
            if let ViewMode::Cortex = app_state.view_mode {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(f.area());

                let right_split = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);

                // Neuron List
                let mut neuron_items = Vec::new();
                let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                neurons_sorted.sort();

                for (i, coord) in neurons_sorted.iter().enumerate() {
                    let neuron = &vm.neurons[coord];
                    let is_selected = app_state.selected_neuron_coords == Some(**coord);

                    let style = if is_selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    // Auto-select first if none selected
                    if app_state.selected_neuron_coords.is_none() && i == 0 {
                        app_state.selected_neuron_coords = Some(**coord);
                    }

                    neuron_items.push(ListItem::new(Span::styled(
                        format!("({}, {}) - {:.2}mV", coord.1, coord.0, neuron.v),
                        style,
                    )));
                }

                let neuron_list = List::new(neuron_items).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Neurons (Cortex)"),
                );
                f.render_widget(neuron_list, chunks[0]);

                // Details & Oscilloscope
                if let Some(coord) = app_state.selected_neuron_coords {
                    if let Some(neuron) = vm.neurons.get(&coord) {
                        // Sparkline
                        let history = &app_state.voltage_history;
                        let sparkline = Sparkline::default()
                            .block(
                                Block::default()
                                    .title("Voltage Trace")
                                    .borders(Borders::ALL),
                            )
                            .data(history)
                            .style(Style::default().fg(Color::Cyan));
                        f.render_widget(sparkline, right_split[1]);

                        // Details
                        let details = vec![
                            Line::from(format!("Membrane Potential (v): {:.2} mV", neuron.v)),
                            Line::from(format!("Injected Current (i_inj): {:.2}", neuron.i_inj)),
                            Line::from(" "),
                            Line::from(format!("Na Activation (m): {:.4}", neuron.m)),
                            Line::from(format!("Na Inactivation (h): {:.4}", neuron.h)),
                            Line::from(format!("K Activation (n): {:.4}", neuron.n)),
                        ];
                        let info = Paragraph::new(details).block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(format!("Neuron Details [{}, {}]", coord.1, coord.0)),
                        );
                        f.render_widget(info, right_split[0]);
                    }
                }

                return; // Skip normal rendering
            }

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

                    // Logic for Editor Selection highlighting (Only if ViewMode::Genome)
                    if app_state.view_mode == ViewMode::Genome && s_idx == app_state.selected_strand && g_idx == app_state.selected_gene {
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

            let chaos_status = if vm.chaos_mode { "ON" } else { "OFF" };
            let mode_str = match app_state.view_mode {
                ViewMode::Genome => "GENOME",
                ViewMode::Grid => "GRID",
                #[cfg(feature = "biophysics")]
                ViewMode::Cortex => "CORTEX",
            };

            let title = match app_state.input_mode {
                InputMode::Normal => format!(
                    "{} (Tab: Switch View, Space: Step, M: Mutate, C: Chaos[{}], Arrows: Nav, Enter: Edit, Q: Quit)",
                    mode_str, chaos_status
                ),
                InputMode::Editing => format!(
                    "EDITING {} (Enter: Commit, Esc: Cancel) - {}",
                    mode_str, app_state.input_buffer
                ),
            };

            let genome_block = Block::default().borders(Borders::ALL).title("Genome");
            let genome_style = if app_state.view_mode == ViewMode::Genome {
                 Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                 Style::default().fg(Color::DarkGray)
            };

            // Highlight the active block borders/title
            let genome_list = List::new(strand_items).block(
                genome_block.border_style(genome_style).title(title.clone()) // Show controls in main title usually
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
                        crate::vm::Value::Junction(_, _) => (
                            "J".to_string(),
                            Style::default().fg(Color::Yellow),
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
                                "fire" => "F",
                                "water" => "W",
                                "earth" => "E",
                                "air" => "A",
                                "steam" => "S",
                                "lava" => "L",
                                "cloud" => "C",
                                "spirit" => "S",
                                "gold" => "G",
                                "lead" => "L",
                                _ => &s[0..1],
                            };
                            let style = match s.as_str() {
                                "fire" => Style::default().fg(Color::Red),
                                "water" => Style::default().fg(Color::Blue),
                                "earth" => Style::default().fg(Color::Yellow),
                                "air" => Style::default().fg(Color::Cyan),
                                "steam" => Style::default().fg(Color::White),
                                "lava" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                                "cloud" => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                                "spirit" => Style::default().fg(Color::Magenta),
                                "gold" => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                                "lead" => Style::default().fg(Color::DarkGray),
                                _ => Style::default().fg(Color::Cyan),
                            };
                            (symbol.to_string(), style)
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
                            if (r as u16 + g as u16 + b as u16) > 300 {
                                style = style.fg(Color::Black);
                            }
                        }

                    // Chromatophores (Nova)
                    let chroma = &vm.chroma_grid[y][x];
                    if let Some(c) = chroma.char {
                        char_rep = c.to_string();
                    }
                    if let Some((r, g, b)) = chroma.fg {
                        style = style.fg(Color::Rgb(r, g, b));
                    }

                        if vm.waste_grid[y][x] > 50 {
                            style = style.add_modifier(Modifier::CROSSED_OUT);
                            if vm.waste_grid[y][x] > 100 {
                                style = style.fg(Color::Red);
                            }
                        }

                        #[cfg(feature = "nova")]
                        if vm.mutagen_grid[y][x] > 20 {
                            // Purple haze for radiation
                            if vm.mutagen_grid[y][x] > 50 {
                                style = style.bg(Color::Magenta).fg(Color::White);
                            } else {
                                style = style.fg(Color::Magenta);
                            }
                        }

                        if let Some(organelle) =
                            vm.organelles.iter().find(|o| o.context_loc == (y, x))
                        {
                            let mut color = match organelle.kind {
                                crate::vm::nova::OrganelleType::Chloroplast => Color::Green,
                                crate::vm::nova::OrganelleType::Mitochondria => Color::Red,
                                crate::vm::nova::OrganelleType::Lysosome => Color::Magenta,
                                crate::vm::nova::OrganelleType::Ribosome => Color::Cyan,
                                crate::vm::nova::OrganelleType::Void => Color::DarkGray,
                                crate::vm::nova::OrganelleType::Alchemist => Color::Yellow,
                                crate::vm::nova::OrganelleType::Worker => Color::White,
                            };
                            let char_code = match organelle.kind {
                                crate::vm::nova::OrganelleType::Chloroplast => "C",
                                crate::vm::nova::OrganelleType::Mitochondria => "M",
                                crate::vm::nova::OrganelleType::Lysosome => "L",
                                crate::vm::nova::OrganelleType::Ribosome => "R",
                                crate::vm::nova::OrganelleType::Void => "Ø",
                                crate::vm::nova::OrganelleType::Alchemist => "A",
                                crate::vm::nova::OrganelleType::Worker => "O",
                            };

                            if organelle.ttl.is_some() {
                                color = Color::Yellow;
                            }

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

                        if (vm.membranes[y][x] & 2) != 0 {
                            style = style.add_modifier(Modifier::UNDERLINED);
                        }
                    }

                    // Highlight Cursor in Grid Mode
                    if app_state.view_mode == ViewMode::Grid && app_state.grid_cursor == (x, y) {
                        if let InputMode::Editing = app_state.input_mode {
                             style = style.bg(Color::Red).fg(Color::White);
                             // If editing, maybe show first char of input buffer?
                             // But input buffer might be long string "add".
                             // Let's just highlight the cell.
                        } else {
                             style = style.bg(Color::White).fg(Color::Black);
                        }
                    }

                    line_spans.push(Span::styled(char_rep, style));

                    #[allow(unused_mut)]
                    let mut spacer = " ";
                    #[cfg(feature = "nova")]
                    if (vm.membranes[y][x] & 4) != 0 {
                        spacer = "|";
                    }
                    line_spans.push(Span::raw(spacer));
                }
                grid_lines.push(Line::from(line_spans));
            }

            #[cfg(feature = "nova")]
            let topology_name = format!("{:?}", vm.topology);
            #[cfg(not(feature = "nova"))]
            let topology_name = "Classic";

            let grid_title = format!("Petri Dish (16x16) - {}", topology_name);
            let grid_style = if app_state.view_mode == ViewMode::Grid {
                 Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                 Style::default().fg(Color::DarkGray)
            };

            let grid_paragraph = Paragraph::new(grid_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(grid_title)
                    .border_style(grid_style),
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
            if let InputMode::Editing = app_state.input_mode {
                 if app_state.view_mode == ViewMode::Grid {
                      output_items.insert(0, ListItem::new(Span::styled(
                           format!("EDIT GRID [{},{}]: {}", app_state.grid_cursor.0, app_state.grid_cursor.1, app_state.input_buffer),
                           Style::default().fg(Color::Cyan)
                      )));
                 }
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
                            match app_state.view_mode {
                                ViewMode::Genome => {
                                    // Genome Editing Logic
                                    match ChimeraParser::parse(Rule::gene, &app_state.input_buffer)
                                    {
                                        Ok(mut pairs) => {
                                            let pair = pairs.next().unwrap();
                                            let gene = Gene::from_pair(pair);

                                            if app_state.selected_strand
                                                < vm.dna.helix.strands.len()
                                                && app_state.selected_gene
                                                    < vm.dna.helix.strands
                                                        [app_state.selected_strand]
                                                        .genes
                                                        .len()
                                            {
                                                vm.dna.helix.strands[app_state.selected_strand]
                                                    .genes[app_state.selected_gene] = gene;
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
                                ViewMode::Grid => {
                                    // Grid Editing Logic
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "biophysics")]
                                ViewMode::Cortex => {
                                    // No editing for Cortex view yet
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
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
                    if c != 'q' && c != ' ' && c != 'm' && c != 'c' && vm.handle_input(c) {
                        continue;
                    }
                }

                match key.code {
                    KeyCode::Tab => {
                        app_state.view_mode = match app_state.view_mode {
                            ViewMode::Genome => ViewMode::Grid,
                            ViewMode::Grid => {
                                #[cfg(feature = "biophysics")]
                                {
                                    ViewMode::Cortex
                                }
                                #[cfg(not(feature = "biophysics"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => ViewMode::Genome,
                        };
                    }
                    #[cfg(feature = "biophysics")]
                    KeyCode::Char('b') => app_state.view_mode = ViewMode::Cortex,
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => vm.step(),
                    KeyCode::Char('m') => vm.mutate(),
                    KeyCode::Char('c') => vm.chaos_mode = !vm.chaos_mode,
                    KeyCode::Down => match app_state.view_mode {
                        ViewMode::Genome => {
                            let s_len = vm.dna.helix.strands.len();
                            if s_len > 0 {
                                let g_len =
                                    vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                if app_state.selected_gene + 1 < g_len {
                                    app_state.selected_gene += 1;
                                } else if app_state.selected_strand + 1 < s_len {
                                    app_state.selected_strand += 1;
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {
                            let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                            neurons_sorted.sort();
                            if let Some(current) = app_state.selected_neuron_coords {
                                if let Some(pos) =
                                    neurons_sorted.iter().position(|&c| *c == current)
                                {
                                    if pos + 1 < neurons_sorted.len() {
                                        app_state.selected_neuron_coords =
                                            Some(*neurons_sorted[pos + 1]);
                                        app_state.voltage_history.clear(); // Reset history on switch
                                    }
                                }
                            } else if !neurons_sorted.is_empty() {
                                app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
                            }
                        }
                    },
                    KeyCode::Up => match app_state.view_mode {
                        ViewMode::Genome => {
                            if app_state.selected_gene > 0 {
                                app_state.selected_gene -= 1;
                            } else if app_state.selected_strand > 0 {
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
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {
                            let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                            neurons_sorted.sort();
                            if let Some(current) = app_state.selected_neuron_coords {
                                if let Some(pos) =
                                    neurons_sorted.iter().position(|&c| *c == current)
                                {
                                    if pos > 0 {
                                        app_state.selected_neuron_coords =
                                            Some(*neurons_sorted[pos - 1]);
                                        app_state.voltage_history.clear();
                                    }
                                }
                            } else if !neurons_sorted.is_empty() {
                                app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
                            }
                        }
                    },
                    KeyCode::Right => match app_state.view_mode {
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                    },
                    KeyCode::Left => match app_state.view_mode {
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                    },
                    KeyCode::Enter => {
                        app_state.input_mode = InputMode::Editing;
                        match app_state.view_mode {
                            ViewMode::Genome => {
                                if app_state.selected_strand < vm.dna.helix.strands.len() {
                                    let g_len =
                                        vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                    if app_state.selected_gene < g_len {
                                        let gene = &vm.dna.helix.strands[app_state.selected_strand]
                                            .genes[app_state.selected_gene];
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
                                                crate::ast::Nucleotide::Identifier(id) => {
                                                    s.push_str(id)
                                                }
                                                crate::ast::Nucleotide::Junction(t, vals) => {
                                                    let t_str = match t {
                                                        crate::ast::JunctionType::Any => "any",
                                                        crate::ast::JunctionType::All => "all",
                                                    };
                                                    s.push_str(t_str);
                                                    s.push('(');
                                                    for (k, v) in vals.iter().enumerate() {
                                                        if k > 0 {
                                                            s.push(' ');
                                                        }
                                                        match v {
                                                            crate::ast::Nucleotide::Number(n) => {
                                                                s.push_str(&n.to_string())
                                                            }
                                                            crate::ast::Nucleotide::String(
                                                                str_val,
                                                            ) => s.push_str(&format!(
                                                                "\"{}\"",
                                                                str_val
                                                            )),
                                                            crate::ast::Nucleotide::Identifier(
                                                                id,
                                                            ) => s.push_str(id),
                                                            crate::ast::Nucleotide::Junction(
                                                                _,
                                                                _,
                                                            ) => s.push_str("nested"),
                                                        }
                                                    }
                                                    s.push(')');
                                                }
                                            }
                                        }
                                        s.push(')');
                                        app_state.input_buffer = s;
                                    }
                                }
                            }
                            ViewMode::Grid => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                // Prevent entering edit mode for Cortex
                                app_state.input_mode = InputMode::Normal;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
