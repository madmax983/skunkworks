use crate::tui::panel_block;
use crate::tui::state::AppState;
use crate::tui::state::{InputMode, ViewMode};
use crate::tui::views::{render_palette, render_view_selector};
use crate::vm::ChimeraVM;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub(crate) fn render_genome_and_grid(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        for (g_idx, gene) in strand.genes.iter().enumerate() {
            let op_str = format!("{}", gene.op);
            let args_str = format!("{:?}", gene.args);
            let mut style = Style::default();

            // Helix Visualization
            let helix_phase = g_idx % 4;
            let (h_prefix, h_suffix) = match helix_phase {
                0 => (" /--[", "]--\\ "),
                1 => ("|    ", "    |"),
                2 => (" \\--[", "]--/ "),
                3 => ("     ", "     "),
                _ => ("", ""),
            };

            let display_content = format!("{}{:^12}{}{}", h_prefix, op_str, h_suffix, args_str);
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
            if app_state.view_mode == ViewMode::Genome
                && s_idx == app_state.selected_strand
                && g_idx == app_state.selected_gene
            {
                if let InputMode::Editing = app_state.input_mode {
                    style = style.bg(Color::Red).fg(Color::White);
                    prefix = "E ";
                } else {
                    style = style.bg(Color::White).fg(Color::Black);
                    prefix = "* ";
                }
            }

            strand_items.push(ListItem::new(format!("{}{}", prefix, display_content)).style(style));
        }
        strand_items.push(ListItem::new("-------------------"));
    }

    let chaos_status = if vm.chaos_mode { "ON" } else { "OFF" };
    let mode_str = match app_state.view_mode {
        ViewMode::Genome => "GENOME",
        ViewMode::Grid => "GRID",
        ViewMode::Microscope => "MICROSCOPE",
        #[cfg(feature = "biophysics")]
        ViewMode::Cortex => "CORTEX",
        #[cfg(feature = "resonance")]
        ViewMode::Resonance => "RESONANCE",
        #[cfg(feature = "nova")]
        ViewMode::Grimoire => "GRIMOIRE",
        #[cfg(feature = "nova")]
        ViewMode::Laboratory => "LABORATORY",
        #[cfg(feature = "nova")]
        ViewMode::Topology => "TOPOLOGY",
        #[cfg(feature = "nova")]
        ViewMode::Graveyard => "GRAVEYARD",
        #[cfg(feature = "nova")]
        ViewMode::PianoRoll => "PIANO ROLL",
        #[cfg(feature = "nova")]
        ViewMode::Retina => "RETINA",
        #[cfg(feature = "nova")]
        ViewMode::Quantum => "QUANTUM",
        #[cfg(feature = "nova")]
        ViewMode::Dream => "DREAM CATCHER",
        #[cfg(feature = "nova")]
        ViewMode::Phylogeny => "PHYLOGENY",
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => "THE ALCHEMIST'S TABLE",
        #[cfg(feature = "nova")]
        ViewMode::Memetics => "MEMETICS",
        #[cfg(feature = "nova")]
        ViewMode::Egregore => "THE EGREGORE",
        #[cfg(feature = "nova")]
        ViewMode::Bestiary => "BESTIARY",
        #[cfg(feature = "nova")]
        ViewMode::Kaleidoscope => "KALEIDOSCOPE",
        #[cfg(feature = "nova")]
        ViewMode::Void => "VOID (ENTROPY)",
        #[cfg(feature = "nova")]
        ViewMode::Signals => "SIGNALS & TRAILS",
        #[cfg(feature = "nova")]
        ViewMode::Sovereignty => "SOVEREIGNTY (TERRITORY)",
        #[cfg(feature = "nova")]
        ViewMode::Spectrogram => "SPECTROGRAM (RESONANCE)",
        #[cfg(feature = "nova")]
        ViewMode::Market => "MARKET (EXCHANGE)",
        #[cfg(feature = "nova")]
        ViewMode::Ballistics => "BALLISTICS (TRAJECTORY)",
        #[cfg(feature = "nova")]
        ViewMode::Scent => "SCENT (OLFACTORY)",
        #[cfg(feature = "nova")]
        ViewMode::Fishing => "FISHING (MINIGAME)",
        #[cfg(feature = "nova")]
        ViewMode::Arena => "ARENA (COLOSSEUM)",
        #[cfg(feature = "nova")]
        ViewMode::Garden => "THE GARDEN OF EDEN (Cellular Automata)",
        #[cfg(feature = "nova")]
        ViewMode::Orca => "ORCA (SIGNAL GRID)",
        ViewMode::Heatmap => "HEATMAP",
        #[cfg(feature = "silicon")]
        ViewMode::Schematic => "SCHEMATIC",
        #[cfg(feature = "elektra")]
        ViewMode::Elektra => "ELEKTRA (ANALOG SIMULATION)",
        #[cfg(feature = "nova")]
        ViewMode::Babel => "BABEL (REGEX LAB)",
        #[cfg(feature = "nova")]
        ViewMode::Strings => "COSMIC STRINGS (VIBRATION)",
        #[cfg(feature = "nova")]
        ViewMode::Quipu => "QUIPU (TOPOLOGICAL MEMORY)",
        #[cfg(feature = "nova")]
        ViewMode::Hydra => "HYDRA (FLUIDIC LOGIC)",
        #[cfg(feature = "nova")]
        ViewMode::Chronos => "CHRONOS (TIME DILATION & HISTORY)",
        #[cfg(feature = "nova")]
        ViewMode::Logos => "LOGOS (LOGIC CHEMISTRY)",
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => "PANDEMONIUM REACTOR (GENOMIC CHAOS)",
        ViewMode::BioticChaos => "BIOTIC CHAOS (COUPLED MAP LATTICE)",
        ViewMode::Catalyst => "CATALYST CHAMBER (DIRECTED EVOLUTION)",
        #[cfg(feature = "nova")]
        ViewMode::Hyperspace => "HYPERSPACE (RECURSION TUNNEL)",
        #[cfg(feature = "nova")]
        ViewMode::Hologram => "HOLOGRAPHIC PLATE (INTERFERENCE)",
        #[cfg(feature = "nova")]
        ViewMode::Weaver => "THE WEAVER",
        #[cfg(feature = "nova")]
        ViewMode::Terminal => "CHIMERIC TERMINAL",
        #[cfg(feature = "silicon")]
        ViewMode::Foundry => "FOUNDRY (GENETIC CIRCUITRY)",
        #[cfg(feature = "nova")]
        ViewMode::Attractor => "STRANGE ATTRACTOR (DYNAMICS)",
        #[cfg(feature = "nova")]
        ViewMode::Virology => "VIROLOGY LAB",
        #[cfg(feature = "nova")]
        ViewMode::BioMesh => "BIOMESH",
        #[cfg(feature = "nova")]
        ViewMode::Crispr => "CRISPR EDITOR",
        #[cfg(feature = "nova")]
        ViewMode::Reactor => "REACTOR CHAMBER",
        #[cfg(feature = "nova")]
        ViewMode::Biolum => "BIOLUMINESCENCE",
        ViewMode::Evolution => "EVOLUTION CHAMBER",
        #[cfg(feature = "nova")]
        ViewMode::Ecology => "GENETIC ECOLOGY",
        _ => "UNKNOWN MODE",
    };

    let title = match app_state.input_mode {
                InputMode::Normal => format!(
                    "{} (Tab: Switch View, Space: Step, M: Mutate, C: Chaos[{}], Madness[{:.2}], I: Inject, Arrows: Nav, Enter: Edit, Q: Quit)",
                    mode_str, chaos_status, vm.glitch_level
                ),
                InputMode::Editing => format!(
                    "EDITING {} (Enter: Commit, Esc: Cancel) - {}",
                    mode_str, app_state.input_buffer
                ),
                InputMode::Injection => "INJECTION (Enter: Splice, Esc: Cancel)".to_string(),
            };

    // Highlight the active block borders/title
    let genome_list =
        List::new(strand_items).block(panel_block(&title, app_state.view_mode == ViewMode::Genome));
    f.render_widget(genome_list, left_chunks[0]);

    // Petri Dish (Grid)
    let mut grid_lines = Vec::new();

    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];

            #[allow(unused_mut)]
            let (mut char_rep, mut style) = match val {
                crate::vm::Value::Int(0) => (".".to_string(), Style::default().fg(Color::DarkGray)),
                crate::vm::Value::Symbol(id) => {
                    (format!("§{:x}", id), Style::default().fg(Color::Magenta))
                }
                crate::vm::Value::Color(r, g, b) => {
                    ("█".to_string(), Style::default().fg(Color::Rgb(*r, *g, *b)))
                }
                crate::vm::Value::Int(n) => {
                    #[cfg(feature = "silicon")]
                    if vm.silicon_mode {
                        match n {
                            1 => ("#".to_string(), Style::default().fg(Color::Yellow)), // Conductor
                            2 => (
                                "@".to_string(),
                                Style::default().fg(Color::White).bg(Color::Cyan),
                            ), // Head
                            3 => ("~".to_string(), Style::default().fg(Color::Red)),    // Tail
                            _ => (
                                format!("{}", (n.abs() % 10)),
                                Style::default().fg(Color::Green),
                            ),
                        }
                    } else {
                        (
                            format!("{}", (n.abs() % 10)),
                            Style::default().fg(Color::Green),
                        )
                    }
                    #[cfg(not(feature = "silicon"))]
                    (
                        format!("{}", (n.abs() % 10)),
                        Style::default().fg(Color::Green),
                    )
                }
                crate::vm::Value::Junction(_, _) => {
                    ("J".to_string(), Style::default().fg(Color::Yellow))
                }
                crate::vm::Value::Superposition(_) => (
                    "Ψ".to_string(),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                crate::vm::Value::Str(s) => {
                    let mut symbol = if s.starts_with("G:") {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() >= 2 {
                            match parts[1] {
                                "AND" => "&",
                                "OR" => "|",
                                "XOR" => "^",
                                "NAND" => "!",
                                "NOT" => "~",
                                _ => "G",
                            }
                        } else {
                            "G"
                        }
                    } else {
                        match s.as_str() {
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
                        }
                    };
                    let style = if s.starts_with("G:") {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        match s.as_str() {
                            "fire" => Style::default().fg(Color::Red),
                            "water" => Style::default().fg(Color::Blue),
                            "earth" => Style::default().fg(Color::Yellow),
                            "air" => Style::default().fg(Color::Cyan),
                            "steam" => Style::default().fg(Color::White),
                            "lava" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            "cloud" => Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                            "spirit" => Style::default().fg(Color::Magenta),
                            "gold" => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            "lead" => Style::default().fg(Color::DarkGray),
                            "PIN:IN" => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            "PIN:OUT" => Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                            s if s.starts_with("EMIT:") => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            s if s.starts_with("RECV:") => Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::BOLD),
                            _ => Style::default().fg(Color::Cyan),
                        }
                    };
                    if s == "PIN:IN" {
                        symbol = "I";
                    } else if s == "PIN:OUT" {
                        symbol = "O";
                    } else if s.starts_with("EMIT:") {
                        symbol = "E";
                    } else if s.starts_with("RECV:") {
                        symbol = "R";
                    }
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

                if vm.signal_grid[y][x] > 0 {
                    // Signal active!
                    style = style
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
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

                if let Some(organelle) = vm.organelles.iter().find(|o| o.context_loc == (y, x)) {
                    let mut color = match organelle.kind {
                        crate::vm::nova::OrganelleType::Chloroplast => Color::Green,
                        crate::vm::nova::OrganelleType::Mitochondria => Color::Red,
                        crate::vm::nova::OrganelleType::Lysosome => Color::Magenta,
                        crate::vm::nova::OrganelleType::Ribosome => Color::Cyan,
                        crate::vm::nova::OrganelleType::Void => Color::DarkGray,
                        crate::vm::nova::OrganelleType::Alchemist => Color::Yellow,
                        crate::vm::nova::OrganelleType::Seed => Color::Green,
                        crate::vm::nova::OrganelleType::Choir => Color::Blue,
                        crate::vm::nova::OrganelleType::Wisp => Color::Yellow,
                        crate::vm::nova::OrganelleType::MadScientist => Color::Magenta,
                        crate::vm::nova::OrganelleType::Phage => Color::Red,
                        crate::vm::nova::OrganelleType::Savant => Color::Cyan,
                        crate::vm::nova::OrganelleType::Metazoan => Color::Green,
                        crate::vm::nova::OrganelleType::Worker => Color::White,
                    };
                    let char_code = match organelle.kind {
                        crate::vm::nova::OrganelleType::Chloroplast => "C",
                        crate::vm::nova::OrganelleType::Mitochondria => "M",
                        crate::vm::nova::OrganelleType::Lysosome => "L",
                        crate::vm::nova::OrganelleType::Ribosome => "R",
                        crate::vm::nova::OrganelleType::Void => "Ø",
                        crate::vm::nova::OrganelleType::Alchemist => "A",
                        crate::vm::nova::OrganelleType::Seed => "S",
                        crate::vm::nova::OrganelleType::Choir => "♫",
                        crate::vm::nova::OrganelleType::Wisp => "*",
                        crate::vm::nova::OrganelleType::MadScientist => "⚛",
                        crate::vm::nova::OrganelleType::Phage => "P",
                        crate::vm::nova::OrganelleType::Savant => "S",
                        crate::vm::nova::OrganelleType::Metazoan => "M",
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

                // Viral Infection
                if let Some(state) = &vm.viral_grid[y][x] {
                    if state.virus_id < vm.virus_library.len() {
                        let virus = &vm.virus_library[state.virus_id];
                        let (r, g, b) = virus.color;
                        style = style.bg(Color::Rgb(r, g, b)).fg(Color::Black);

                        if state.infection_level > 150 {
                            let chars = ['@', '#', '$', '%', '&', '!', '?', 'X'];
                            let idx = (x + y + vm.tick_counter as usize) % chars.len();
                            char_rep = chars[idx].to_string();
                            style = style.add_modifier(Modifier::RAPID_BLINK);
                        }
                    }
                }

                // Projectiles (Top Layer)
                for p in &vm.projectiles {
                    if (p.x as usize) == x && (p.y as usize) == y {
                        style = style
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
                        if p.vx.abs() > p.vy.abs() {
                            if p.vx > 0.0 {
                                char_rep = "→".to_string();
                            } else {
                                char_rep = "←".to_string();
                            }
                        } else if p.vy > 0.0 {
                            char_rep = "↓".to_string();
                        } else {
                            char_rep = "↑".to_string();
                        }
                    }
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
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
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
        .map(|val| {
            // Apply Babel Glitch
            #[cfg(feature = "nova")]
            let content = if vm.babel_state.integrity < 0.9 {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(1.0 - vm.babel_state.integrity) {
                    val.chars()
                        .map(|c| {
                            if rng.gen_bool(0.3) {
                                let glitch_chars =
                                    ['!', '@', '#', '$', '%', '^', '&', '*', '?', '¿', '¡'];
                                glitch_chars[rng.gen_range(0..glitch_chars.len())]
                            } else {
                                c
                            }
                        })
                        .collect()
                } else {
                    val.clone()
                }
            } else {
                val.clone()
            };
            #[cfg(not(feature = "nova"))]
            let content = val.clone();

            ListItem::new(content)
        })
        .collect();

    if !app_state.status_msg.is_empty() {
        output_items.insert(
            0,
            ListItem::new(Span::styled(
                format!("STATUS: {}", app_state.status_msg),
                Style::default().fg(Color::Yellow),
            )),
        );
    }
    if let InputMode::Editing = app_state.input_mode {
        if app_state.view_mode == ViewMode::Grid {
            output_items.insert(
                0,
                ListItem::new(Span::styled(
                    format!(
                        "EDIT GRID [{},{}]: {}",
                        app_state.grid_cursor.0, app_state.grid_cursor.1, app_state.input_buffer
                    ),
                    Style::default().fg(Color::Cyan),
                )),
            );
        }
    }

    let output_list =
        List::new(output_items).block(Block::default().borders(Borders::ALL).title("Output"));
    f.render_widget(output_list, right_chunks[1]);

    // Draw Injection Popup
    if let InputMode::Injection = app_state.input_mode {
        let area = app_state.get_render_area(f.area());
        let popup_area = ratatui::layout::Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        f.render_widget(ratatui::widgets::Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Viral Injection Vector (ChimeraScript)")
            .style(Style::default().fg(Color::Green));
        let text = Paragraph::new(app_state.input_buffer.clone())
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(text, popup_area);
    }

    // Draw Spirit Popup on top
    #[cfg(feature = "nova")]
    if vm.spirit_request {
        let area = app_state.get_render_area(f.area());
        let popup_area = ratatui::layout::Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        f.render_widget(ratatui::widgets::Clear, popup_area);

        let prompt = vm.spirit_message.as_deref().unwrap_or("SPIRIT SUMMONING");
        let text = format!("{}\n\n> {}", prompt, app_state.input_buffer);
        let popup = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Spirit Communication")
                .style(Style::default().fg(Color::Cyan)),
        );
        f.render_widget(popup, popup_area);
    }

    if app_state.palette_open {
        render_palette(f, app_state);
    }

    if app_state.show_view_selector {
        render_view_selector(f, app_state);
    }
}

#[cfg(feature = "nova")]
pub(crate) fn render_terminal(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Show last N lines, oldest first (standard terminal log)
    let log_start = vm.output.len().saturating_sub(30);
    let log_items: Vec<ListItem> = vm
        .output
        .iter()
        .skip(log_start)
        .map(|s| ListItem::new(s.clone()).style(Style::default().fg(Color::Green)))
        .collect();

    let log_list = List::new(log_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Chimeric Console (Type '?' for help)"),
    );
    f.render_widget(log_list, chunks[0]);

    // Input Line
    let input_text = format!("> {}_", app_state.terminal_input);
    let input_widget = Paragraph::new(input_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(input_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_void(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Void Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let entropy = vm.entropy_grid[y][x];
            let mut style = Style::default();

            // Entropy visualization
            // 0-10: Space
            // 10-30: Light Shade
            // 30-60: Medium Shade
            // 60-80: Dark Shade
            // 80+: Full Block
            let ch = if entropy < 10 {
                " ".to_string()
            } else if entropy < 30 {
                "░".to_string()
            } else if entropy < 60 {
                "▒".to_string()
            } else if entropy < 80 {
                "▓".to_string()
            } else {
                "█".to_string()
            };

            // Color: Dark Gray to White to Red
            if entropy < 30 {
                style = style.fg(Color::DarkGray);
            } else if entropy < 60 {
                style = style.fg(Color::Gray);
            } else if entropy < 80 {
                style = style.fg(Color::White);
            } else {
                style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
            }

            // Overlay Void Organelles
            let mut is_void = false;
            let mut is_wisp = false;
            if let Some(org) = vm.organelles.iter().find(|o| o.context_loc == (y, x)) {
                if org.kind == crate::vm::nova::OrganelleType::Void {
                    is_void = true;
                } else if org.kind == crate::vm::nova::OrganelleType::Wisp {
                    is_wisp = true;
                }
            }

            if is_void {
                style = style
                    .bg(Color::Red)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
                line_spans.push(Span::styled("Ø", style));
            } else if is_wisp {
                style = style
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
                line_spans.push(Span::styled("*", style));
            } else {
                // If cursor
                if app_state.grid_cursor == (x, y) {
                    style = style.bg(Color::White).fg(Color::Black);
                }
                line_spans.push(Span::styled(ch, style));
            }
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines)
        .block(Block::default().borders(Borders::ALL).title("Entropy Grid"));
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let (cx, cy) = app_state.grid_cursor;
    let local_entropy = vm.entropy_grid[cy][cx];

    let mut info_text = vec![
        Line::from("THE VOID"),
        Line::from(" "),
        Line::from(format!("Local Entropy: {} / 100", local_entropy)),
        Line::from(" "),
        Line::from("Mechanics:"),
        Line::from("  - Entropy > 50 causes Reality Decay (Glitches)"),
        Line::from("  - Void Organelles (Ø) generate Entropy"),
        Line::from("  - stabilize(n) reduces Entropy"),
        Line::from("  - disintegrate(y, x) creates Entropy"),
        Line::from(" "),
        Line::from("Void Buffer (LIFO):"),
    ];

    if vm.prologue_state.void_buffer.is_empty() {
        info_text.push(Line::from("  (Empty)"));
    } else {
        for (i, val) in vm
            .prologue_state
            .void_buffer
            .iter()
            .rev()
            .take(10)
            .enumerate()
        {
            info_text.push(Line::from(format!("  [{}] {}", i, val)));
        }
        if vm.prologue_state.void_buffer.len() > 10 {
            info_text.push(Line::from("  ..."));
        }
    }

    let info_widget =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(info_widget, chunks[1]);
}
