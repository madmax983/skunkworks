use crate::tui::state::AppState;
use crate::vm::ChimeraVM;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[cfg(feature = "nova")]
pub(crate) fn render_grimoire(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Grimoire Text (Manual)
    let grimoire_text = format!(
        "{}\n\n{}",
        crate::tui::GRIMOIRE_TEXT,
        render_hermetic_rules(vm)
    );

    let grimoire_widget = Paragraph::new(grimoire_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("The Grimoire (Manual) - Scroll with Up/Down"),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((app_state.grimoire_scroll, 0));
    f.render_widget(grimoire_widget, chunks[0]);

    // Right: Utilities (Oracle & Sigils)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Oracle (KB)
    #[cfg(feature = "oracle")]
    {
        let mut kb_items: Vec<ListItem> = vm
            .knowledge_base
            .iter()
            .take(20)
            .map(|fact| ListItem::new(format!("{}", fact)))
            .collect();

        if !app_state.query_results.is_empty() {
            kb_items.push(
                ListItem::new("--- Query Results ---").style(Style::default().fg(Color::Yellow)),
            );
            for res in &app_state.query_results {
                kb_items.push(ListItem::new(res.clone()).style(Style::default().fg(Color::Cyan)));
            }
        }

        let title = if app_state.query_mode {
            format!("Oracle (Query: {})", app_state.query_input)
        } else {
            "Oracle (Press '/')".to_string()
        };

        let border_style = if app_state.query_mode {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let oracle_list = List::new(kb_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        );
        f.render_widget(oracle_list, right_chunks[0]);
    }

    // Sigil Registry (Spells)
    {
        let mut registry: Vec<_> = vm.sigil_registry.iter().collect();
        registry.sort_by_key(|(k, _)| *k);

        let sigil_items: Vec<ListItem> = registry
            .iter()
            .enumerate()
            .map(|(i, (name, sigil))| {
                let status = if sigil.auto_cast {
                    "⚡ AUTO"
                } else {
                    "○ MANU"
                };
                let style = if i == app_state.selected_sigil_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!(
                    "{} | {} ({} cells) -> Strand {}",
                    status,
                    name,
                    sigil.pattern.len(),
                    sigil.strand_idx
                ))
                .style(style)
            })
            .collect();

        let sigil_list = List::new(sigil_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("The Grimoire (Select & Enter to Toggle Auto-Cast)"),
        );
        f.render_widget(sigil_list, chunks[2]);
    }

    // Bard (Score)
    let score_text: String = crate::vm::bard::score_to_abc(&vm.score);
    let bard_paragraph = Paragraph::new(score_text)
        .block(Block::default().borders(Borders::ALL).title("Bard (Score)"));
    f.render_widget(bard_paragraph, chunks[3]);
}

fn render_hermetic_rules(vm: &ChimeraVM) -> String {
    if vm.prologue_state.alchemy_book.is_empty() {
        return String::from("## Hermetic Alchemy\n\n(No dynamic rules defined)");
    }

    let mut s = String::from("## Hermetic Alchemy\n\n");
    for (i, rule) in vm.prologue_state.alchemy_book.iter().enumerate() {
        let ingredients: Vec<String> = rule.ingredients.iter().map(|v| format!("{}", v)).collect();
        s.push_str(&format!(
            "{}. {} -> {}\n",
            i + 1,
            ingredients.join(" + "),
            rule.result
        ));
    }
    s
}

#[cfg(feature = "nova")]
pub(crate) fn render_alchemy(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20), // Shelf
                Constraint::Percentage(40), // Crucible
                Constraint::Percentage(40), // Strands
            ]
            .as_ref(),
        )
        .split(app_state.get_render_area(f.area()));

    // Shelf
    let elements = [
        "Fire", "Water", "Earth", "Air", "Life", "Death", "Lead", "Energy",
    ];
    let mut shelf_items = Vec::new();
    for (i, elem) in elements.iter().enumerate() {
        let mut style = Style::default().fg(Color::Cyan);
        if app_state.alchemy_selection == 0 && i == app_state.alchemy_shelf_idx {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        shelf_items.push(ListItem::new(Span::styled(*elem, style)));
    }

    let shelf_border_style = if app_state.alchemy_selection == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let shelf_list = List::new(shelf_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Reagent Shelf")
            .border_style(shelf_border_style),
    );
    f.render_widget(shelf_list, chunks[0]);

    // Crucible
    let crucible_items: Vec<ListItem> = vm
        .crucible
        .contents
        .iter()
        .map(|v| ListItem::new(format!("{}", v)).style(Style::default().fg(Color::Magenta)))
        .collect();

    let crucible_list =
        List::new(crucible_items).block(Block::default().borders(Borders::ALL).title(
            "Crucible (A: Add, X: Clear, T: Transmute) [Recipes: Fire/Water/Void/Life + Strand]",
        ));
    f.render_widget(crucible_list, chunks[1]);

    // Strands
    let mut strand_items = Vec::new();
    for (i, strand) in vm.dna.helix.strands.iter().enumerate() {
        let mut style = Style::default().fg(Color::White);
        if app_state.alchemy_selection == 1 && i == app_state.alchemy_strand_idx {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        strand_items.push(
            ListItem::new(format!("Strand {} ({} genes)", i, strand.genes.len())).style(style),
        );
    }

    let strand_border_style = if app_state.alchemy_selection == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let strand_list = List::new(strand_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("DNA Inventory")
            .border_style(strand_border_style),
    );
    f.render_widget(strand_list, chunks[2]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_graveyard(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[0]);

    // Graveyard List
    let mut grave_items = Vec::new();
    if vm.graveyard.is_empty() {
        grave_items.push(ListItem::new("The Graveyard is empty."));
    } else {
        for (i, strand) in vm.graveyard.iter().enumerate() {
            let is_selected = i == app_state.selected_graveyard_strand;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            grave_items.push(
                ListItem::new(format!("Strand {} (Len: {})", i, strand.genes.len())).style(style),
            );
        }
    }
    let grave_list = List::new(grave_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Graveyard (Necropolis)"),
    );
    f.render_widget(grave_list, top_chunks[0]);

    // Strand Preview
    let mut gene_items = Vec::new();
    if !vm.graveyard.is_empty() && app_state.selected_graveyard_strand < vm.graveyard.len() {
        let strand = &vm.graveyard[app_state.selected_graveyard_strand];
        for gene in &strand.genes {
            gene_items.push(
                ListItem::new(format!("{}", gene.op)).style(Style::default().fg(Color::Cyan)),
            );
        }
    } else if !vm.graveyard.is_empty() {
        gene_items.push(ListItem::new("Invalid Selection"));
    } else {
        gene_items.push(ListItem::new("No souls to display."));
    }
    let preview_list = List::new(gene_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Genome of the Departed"),
    );
    f.render_widget(preview_list, top_chunks[1]);

    // Help / Status
    let help_text = "Controls:\n↑/↓: Navigate\nR: Resurrect (Exhume to Helix)\nX: Exterminate (Permanent Deletion)\nTab: Switch View";
    let help_para =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Necromancy"));
    f.render_widget(help_para, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_codex(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Spell List
    let mut items = Vec::new();
    for (i, spell) in vm.codex.spells.iter().enumerate() {
        let style = if i == app_state.codex_selected_spell {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        items.push(
            ListItem::new(format!("{}: {} ({} Energy)", i, spell.name, spell.cost)).style(style),
        );
    }

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("The Codex"));
    f.render_widget(list, chunks[0]);

    // Details
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    if let Some(spell) = vm.codex.spells.get(app_state.codex_selected_spell) {
        let details = vec![
            Line::from(vec![Span::styled(
                format!("Spell: {}", spell.name),
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(format!("Cost: {}", spell.cost)),
            Line::from(""),
            Line::from("Description:"),
            Line::from(spell.description.as_str()),
        ];
        let p = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Incantation"));
        f.render_widget(p, right_chunks[0]);
    }

    // Help
    let help = vec![
        Line::from("Controls:"),
        Line::from("  Up/Down: Select Spell"),
        Line::from("  Enter: Cast Spell"),
        Line::from(" "),
        Line::from("Warning: Spells consume Energy and may have unpredictable effects."),
    ];
    let help_p = Paragraph::new(help).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Grimoire Guide"),
    );
    f.render_widget(help_p, right_chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_verbum(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Word List
    let mut items = Vec::new();
    let mut words: Vec<_> = vm.verbum_forge.words.values().collect();
    words.sort_by_key(|w| w.id);

    for word in &words {
        let style = if word.id == app_state.alchemy_strand_idx {
            // Reuse alchemy index for selection
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            match word.rarity {
                crate::vm::verbum::Rarity::Common => Style::default().fg(Color::White),
                crate::vm::verbum::Rarity::Uncommon => Style::default().fg(Color::Green),
                crate::vm::verbum::Rarity::Rare => Style::default().fg(Color::Blue),
                crate::vm::verbum::Rarity::Epic => Style::default().fg(Color::Magenta),
                crate::vm::verbum::Rarity::Legendary => Style::default().fg(Color::Yellow),
                crate::vm::verbum::Rarity::Mythic => Style::default().fg(Color::Red),
            }
        };
        items.push(ListItem::new(format!("{}: {}", word.id, word.name)).style(style));
    }

    if items.is_empty() {
        items.push(ListItem::new("The Lexicon is empty."));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("The Verbum Forge"),
    );
    f.render_widget(list, chunks[0]);

    // Details
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(chunks[1]);

    if let Some(word) = words.iter().find(|w| w.id == app_state.alchemy_strand_idx) {
        let mut details = vec![
            Line::from(vec![Span::styled(
                format!("Word: {}", word.name),
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(format!("Rarity: {:?}", word.rarity)),
            Line::from(format!("Power Cost: {}", word.cost)),
            Line::from(""),
            Line::from("Meaning (Genes):"),
        ];

        for gene in &word.genes {
            details.push(Line::from(format!("  {}", gene.op)));
        }

        let p = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Etymology"));
        f.render_widget(p, right_chunks[0]);
    }

    // Help
    let help = vec![
        Line::from("Controls:"),
        Line::from("  Up/Down: Select Word"),
        Line::from("  Enter: Invoke Word"),
        Line::from("  F: Forge new word from current Strand"),
        Line::from(" "),
        Line::from("Use 'Forge(name, strand)' op to create programmatically."),
    ];
    let help_p = Paragraph::new(help).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Lexical Guide"),
    );
    f.render_widget(help_p, right_chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_savant(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Knowledge Base (Facts & Rules)
    #[cfg(feature = "oracle")]
    let kb_items: Vec<ListItem> = vm
        .knowledge_base
        .iter()
        .map(|val| ListItem::new(format!("{}", val)).style(Style::default().fg(Color::Cyan)))
        .collect();

    #[cfg(not(feature = "oracle"))]
    let kb_items: Vec<ListItem> = vec![ListItem::new("Oracle feature disabled.")];

    let kb_list = List::new(kb_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Savant Knowledge Base"),
    );
    f.render_widget(kb_list, chunks[0]);

    // Right: Savant Organelles & Logs
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Top Right: Active Savants
    let mut savant_items = Vec::new();
    for org in &vm.organelles {
        if matches!(org.kind, crate::vm::nova::OrganelleType::Savant) {
            savant_items.push(
                ListItem::new(format!(
                    "ID {}: {} @ {},{}",
                    org.id, org.name, org.context_loc.1, org.context_loc.0
                ))
                .style(Style::default().fg(Color::Yellow)),
            );
        }
    }

    if savant_items.is_empty() {
        savant_items
            .push(ListItem::new("No active Savants.").style(Style::default().fg(Color::DarkGray)));
    }

    let savant_list = List::new(savant_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Active Agents"),
    );
    f.render_widget(savant_list, right_chunks[0]);

    // Bottom Right: Logic Logs (Filtered Output)
    let log_items: Vec<ListItem> = vm
        .output
        .iter()
        .filter(|s| s.starts_with("SAVANT") || s.starts_with("ORACLE"))
        .rev()
        .take(20)
        .map(|s| ListItem::new(s.clone()).style(Style::default().fg(Color::Green)))
        .collect();

    let log_list = List::new(log_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Inference Log"),
    );
    f.render_widget(log_list, right_chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_akashic(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left Panel: Split into Records (Top) and Memories (Bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Top-Left: Records (Key-Value)
    let mut items = Vec::new();
    for (k, v) in &vm.akashic.storage {
        items.push(ListItem::new(format!("{}: {}", k, v)).style(Style::default().fg(Color::Cyan)));
    }
    if items.is_empty() {
        items.push(
            ListItem::new("Akashic Records Empty.").style(Style::default().fg(Color::DarkGray)),
        );
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Persistent Storage (KV)"),
    );
    f.render_widget(list, left_chunks[0]);

    // Bottom-Left: Memories (Snapshots)
    let mut mem_items = Vec::new();
    for k in vm.akashic.memories.keys() {
        mem_items.push(
            ListItem::new(format!("Memory: {}", k)).style(Style::default().fg(Color::Magenta)),
        );
    }
    if mem_items.is_empty() {
        mem_items
            .push(ListItem::new("No Memories Saved.").style(Style::default().fg(Color::DarkGray)));
    }

    let mem_list = List::new(mem_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Saved Memories (Time Travel)"),
    );
    f.render_widget(mem_list, left_chunks[1]);

    // Right: Karma & Miracles
    let mut info = Vec::new();
    info.push(Line::from(format!("Karma: {}", vm.akashic.karma)));
    info.push(Line::from(" "));
    info.push(Line::from("Miracles (Cost):"));
    info.push(Line::from("  0: Resurrection (1000)"));
    info.push(Line::from("  1: Terraform (5000)"));
    info.push(Line::from("  2: Wealth (2000)"));
    info.push(Line::from("  3: Cleanse (500)"));
    info.push(Line::from("  4: Ascension (10000)"));
    info.push(Line::from(" "));
    info.push(Line::from("Opcodes:"));
    info.push(Line::from("  Miracle(id)"));
    info.push(Line::from("  AkashicSave(key)"));
    info.push(Line::from("  AkashicLoad(key)"));

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Karma & Destiny"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_prologue(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();
            let val = &vm.grid[y][x];
            let mut s = match val {
                crate::vm::Value::Str(s) => s.clone(),
                crate::vm::Value::Int(n) => n.to_string(),
                crate::vm::Value::Color(r, g, b) => {
                    style = style.fg(Color::Rgb(*r, *g, *b));
                    "█".to_string()
                }
                _ => ".".to_string(),
            };

            // Reality Mode Coloring
            let mode = vm.prologue_state.reality_state.get_mode(y, x);
            match mode {
                crate::vm::prologue::weave_reality::RealityMode::Orca => {
                    style = style.bg(Color::Rgb(0, 0, 50)); // Dark Blue for Orca
                }
                crate::vm::prologue::weave_reality::RealityMode::Silicon => {
                    style = style.bg(Color::Rgb(50, 40, 30)); // Copper/Dark Grey for Silicon
                }
                crate::vm::prologue::weave_reality::RealityMode::Life => {
                    style = style.bg(Color::Rgb(0, 50, 0)); // Dark Green for Life
                }
                crate::vm::prologue::weave_reality::RealityMode::Prologue => {
                    // Default Black
                }
            }

            if app_state.chaos_mode {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                if s == "." || s == "0" {
                    if rng.gen_bool(0.05) {
                        let glitches = ['░', '▒', '▓', '█', '!', '?', '*', '#', '@', '§', '¶'];
                        s = glitches[rng.gen_range(0..glitches.len())].to_string();
                        style = style.fg(Color::DarkGray);
                    }
                } else if rng.gen_bool(0.1) {
                    style = style.add_modifier(Modifier::RAPID_BLINK);
                }
            }

            if let crate::vm::Value::Superposition(_) = val {
                style = style.add_modifier(Modifier::RAPID_BLINK).fg(Color::Yellow);
                s = "Ψ".to_string();
            }

            use crate::vm::prologue::epigenetics::EpigeneticMark;
            let epi = vm.prologue_state.epigenetic_grid[y][x];
            match epi {
                EpigeneticMark::Methylated => {
                    style = style
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::CROSSED_OUT);
                }
                EpigeneticMark::Phosphorylated => {
                    style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                }
                EpigeneticMark::Rotting => {
                    style = style.fg(Color::Rgb(139, 69, 19)); // SaddleBrown
                    if s == "." {
                        s = "☣".to_string();
                    }
                }
                _ => {}
            }

            if vm.prologue_state.signal_grid[y][x].is_some() {
                style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
            } else if vm.prologue_state.mycelium_network.contains(&(y, x)) {
                // Mycelium Substrate
                style = style.bg(Color::Rgb(101, 67, 33)); // Dark Brown
            }

            if vm.prologue_state.runes.contains(&(y, x)) {
                // Colorize Runes
                match s.as_str() {
                    "$" => style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    "M" => style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    "O" => style = style.fg(Color::Blue).add_modifier(Modifier::BOLD),
                    "%" => style = style.fg(Color::White).add_modifier(Modifier::BOLD),
                    "^" | "J" => style = style.fg(Color::Red).add_modifier(Modifier::BOLD),
                    "@" => style = style.fg(Color::Red).add_modifier(Modifier::BOLD),
                    "•" => style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    "°" => style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    "⇝" => style = style.fg(Color::Red).add_modifier(Modifier::BOLD),
                    "G" => style = style.fg(Color::Green).add_modifier(Modifier::BOLD),
                    "D" | "N" | "S" | "E" | "W" => {
                        style = style.fg(Color::Blue).add_modifier(Modifier::BOLD)
                    }
                    "A" | "B" | "P" | "Q" | "C" => {
                        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    }
                    "(" => style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    "=" | ">" | "<" => style = style.fg(Color::White).add_modifier(Modifier::BOLD),
                    "I" => style = style.fg(Color::Red).add_modifier(Modifier::BOLD),
                    "Y" | "L" => style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    "K" => {
                        style = style
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                    }
                    "R" => style = style.fg(Color::White).add_modifier(Modifier::BOLD),
                    "X" => style = style.fg(Color::Green).add_modifier(Modifier::BOLD),
                    "[" | "]" | "U" | "V" | "F" | "T" => {
                        style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD)
                    }
                    "q" | "m" => {
                        style = style
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                    }
                    "{" | "}" => {
                        style = style
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                    }
                    "k" => {
                        style = style
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK)
                    }
                    "z" => style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    "h" => style = style.fg(Color::Red).add_modifier(Modifier::BOLD),
                    "." | ":" | "," => {
                        style = style.fg(Color::LightCyan).add_modifier(Modifier::BOLD)
                    }
                    "⚡" => {
                        style = style
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK)
                    }
                    "≡" => style = style.fg(Color::Blue).add_modifier(Modifier::BOLD),
                    "∿" => style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    "¶" => style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    "λ" => {
                        style = style
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                    }
                    "♻" => {
                        style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
                    }
                    "☣" => {
                        style = style.fg(Color::Rgb(139, 69, 19)); // SaddleBrown
                    }
                    "₣" => {
                        style = style.fg(Color::White).add_modifier(Modifier::BOLD);
                    }
                    "⏱️" | "🥁" | "🎹" | "🎚️" => {
                        style = style.fg(Color::LightCyan).add_modifier(Modifier::BOLD);
                    }
                    "🍄" => style = style.fg(Color::Red).add_modifier(Modifier::BOLD),
                    "📥" | "📤" | "🦋" => {
                        style = style.fg(Color::LightGreen).add_modifier(Modifier::BOLD)
                    }
                    "🌀" => {
                        style = style
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                    }
                    "🌐" => {
                        style = style
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                    }
                    _ => style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD),
                }
            } else if !vm.prologue_state.mycelium_network.contains(&(y, x)) {
                // Dim runes that are not active/registered
                if !matches!(s.as_str(), "." | "0") {
                    style = style.fg(Color::DarkGray);
                } else {
                    style = style.fg(Color::DarkGray);
                }
            }

            if app_state.grid_cursor == (x, y) {
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
            }

            // Truncate to 3 chars
            let display = format!("{:^3.3}", s);
            line_spans.push(Span::styled(display, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let status = if vm.prologue_state.active {
        "ACTIVE"
    } else {
        "INACTIVE"
    };
    let color = if vm.prologue_state.active {
        Color::Green
    } else {
        Color::Red
    };

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default().borders(Borders::ALL).title(Span::styled(
            format!("PROLOGUE GRID ({})", status),
            Style::default().fg(color),
        )),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Rules
    let mut info = Vec::new();
    info.push(Line::from("Logic Engine Status:"));
    info.push(Line::from(format!(
        "Active Agents: {}",
        vm.prologue_state.agents.len()
    )));

    // Debug Signals
    let mut signal_count = 0;
    for row in &vm.prologue_state.signal_grid {
        for cell in row {
            if cell.is_some() {
                signal_count += 1;
            }
        }
    }
    info.push(Line::from(format!("Active Signals: {}", signal_count)));

    // Rhythm Status
    info.push(Line::from(format!(
        "Rhythm: {} BPM (Int: {})",
        vm.prologue_state.rhythm_state.bpm, vm.prologue_state.rhythm_state.beat_interval
    )));

    // Mycelium Status
    info.push(Line::from(format!(
        "Mycelium Nodes: {}",
        vm.prologue_state.mycelium_network.len()
    )));
    info.push(Line::from(format!(
        "Buffer: {} items",
        vm.prologue_state.mycelium_buffer.len()
    )));
    if let Some(val) = vm.prologue_state.mycelium_buffer.front() {
        info.push(Line::from(format!("  Head: {}", val)));
    }

    // Check for Agent at Cursor
    let (cx, cy) = app_state.grid_cursor;
    if let Some(agent) = vm
        .prologue_state
        .agents
        .iter()
        .find(|a| a.x == cx && a.y == cy)
    {
        info.push(Line::from(" "));
        info.push(Line::from(Span::styled(
            "SELECTED AGENT",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));

        let type_str = if let crate::vm::Value::Str(s) = &vm.grid[cy][cx] {
            s.clone()
        } else {
            "?".to_string()
        };

        info.push(Line::from(format!("Type: {}", type_str)));

        if type_str == "₣" {
            info.push(Line::from("Forth Stack:"));
            for (i, val) in agent.stack.iter().rev().take(10).enumerate() {
                info.push(Line::from(format!(" {}: {}", i, val)));
            }
            if agent.stack.len() > 10 {
                info.push(Line::from(" ..."));
            }
            if agent.stack.is_empty() {
                info.push(Line::from(" (Empty)"));
            }
        } else {
            info.push(Line::from(format!("State: {}", agent.state)));
        }
    }

    // Check Reality Mode at Cursor
    let mode = vm.prologue_state.reality_state.get_mode(cy, cx);
    info.push(Line::from(" "));
    info.push(Line::from(format!("Reality: {:?}", mode)));

    info.push(Line::from(" "));
    info.push(Line::from("Rules:"));
    info.push(Line::from("  ! Source (Emits North)"));
    info.push(Line::from("  ? Sink (Reads South)"));
    info.push(Line::from("  ~ Wire, & AND, | OR"));
    info.push(Line::from("  @ Agent (Moves to Signal)"));
    info.push(Line::from("  $ Scribe (W->S), % Mod"));
    info.push(Line::from("  M Mutate, O Organelle"));
    info.push(Line::from("  ^ Jump, J Jumper, ( Warp"));
    info.push(Line::from("  A/B/P/Q/C Arithmetic/Time"));
    info.push(Line::from("  N/S/E/W Directional"));
    info.push(Line::from("  =/</> Compare, I If"));
    info.push(Line::from("  Y/L Ether (Yell/Listen)"));
    info.push(Line::from("  K Chaos, R Register, X Cross"));
    info.push(Line::from("  [ ] Collect/Scatter"));
    info.push(Line::from("  U/V Head/Tail, F/T Filter/Take"));
    info.push(Line::from("  k Chaos Src, z Glitch, h Havoc"));
    info.push(Line::from("  ⏱️ Clock, 🥁 Drum, 🎹 Key, 🎚️ Fader"));
    info.push(Line::from("  🌐 World (W=Rad, N=Mode: 1=Orca, 2=Silicon)"));

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Logic Engine"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_forge(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Rule List
    let mut items = Vec::new();
    let mut keys: Vec<_> = vm.prologue_state.logos_engine.rules.keys().collect();
    keys.sort();

    for key in keys {
        let style = if *key == app_state.forge_selected_rule {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        items.push(ListItem::new(key.clone()).style(style));
    }

    if items.is_empty() {
        items.push(ListItem::new("No Rules defined."));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title("Grammar Rules")
        .border_style(if app_state.forge_focus == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    f.render_widget(List::new(items).block(list_block), chunks[0]);

    // Right: Editor & Testbed
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(chunks[1]);

    // Editor
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .title("Rule Definition (Enter to Commit)")
        .border_style(if app_state.forge_focus == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let editor_content = if app_state.forge_editor_buffer.is_empty() {
        // Show current definition if empty buffer?
        if let Some(rule) = vm
            .prologue_state
            .logos_engine
            .rules
            .get(&app_state.forge_selected_rule)
        {
            format!("{:?}", rule)
        } else {
            "Select or Create Rule...".to_string()
        }
    } else {
        app_state.forge_editor_buffer.clone()
    };

    f.render_widget(
        Paragraph::new(editor_content)
            .block(editor_block)
            .wrap(ratatui::widgets::Wrap { trim: false }),
        right_chunks[0],
    );

    // Testbed
    let test_block = Block::default()
        .borders(Borders::ALL)
        .title("Test Input (Enter to Test)")
        .border_style(if app_state.forge_focus == 2 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let test_text = vec![
        Line::from(format!("Input: {}", app_state.forge_test_input)),
        Line::from("---"),
        Line::from(format!("Output: {}", app_state.forge_test_output)),
    ];

    f.render_widget(Paragraph::new(test_text).block(test_block), right_chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_dream(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Trace List
    let mut trace_items = Vec::new();
    if vm.dream_traces.is_empty() {
        trace_items.push(ListItem::new("No dreams recorded."));
    } else {
        for (i, trace) in vm.dream_traces.iter().enumerate() {
            let is_selected = i == app_state.selected_dream_trace;
            let mut style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            if trace.accepted {
                style = style.fg(Color::Green);
            } else {
                style = style.fg(Color::Magenta); // Discarded dreams
            }

            let mut label_suffix = "";
            if trace.is_nightmare {
                style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
                label_suffix = " (NIGHTMARE)";
            }

            if is_selected {
                style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            }

            let icon = if trace.accepted { "✔" } else { "✖" };
            trace_items.push(
                ListItem::new(format!(
                    "{} Dream #{} (Strand {}) - {} Ticks{}",
                    icon, i, trace.strand_idx, trace.duration, label_suffix
                ))
                .style(style),
            );
        }
    }

    let trace_list =
        List::new(trace_items).block(Block::default().borders(Borders::ALL).title("Dream Log"));
    f.render_widget(trace_list, chunks[0]);

    // Details
    if !vm.dream_traces.is_empty() && app_state.selected_dream_trace < vm.dream_traces.len() {
        let trace = &vm.dream_traces[app_state.selected_dream_trace];

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
            .split(chunks[1]);

        let mut info_text = vec![
            Line::from(format!("Mutation: {}", trace.mutation_desc)),
            Line::from(format!(
                "Energy: {} -> {} (Cost: {})",
                trace.result_energy + trace.energy_cost, // Approx start
                trace.result_energy,
                trace.energy_cost
            )),
            Line::from(format!(
                "Status: {}",
                if trace.status == 1 { "Alive" } else { "Dead" }
            )),
            Line::from(format!("Accepted: {}", trace.accepted)),
        ];

        if trace.is_nightmare {
            info_text.push(Line::from(Span::styled(
                "TYPE: NIGHTMARE (FORCED)",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
        }

        info_text.push(Line::from(""));
        info_text.push(Line::from(Span::styled(
            "Press ENTER to Realize (Lucid Dreaming)",
            Style::default().fg(Color::Cyan),
        )));

        let info = Paragraph::new(info_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dream Details"),
        );
        f.render_widget(info, right_chunks[0]);

        // Output Log
        let log_items: Vec<ListItem> = trace
            .output_log
            .iter()
            .map(|s| ListItem::new(s.clone()).style(Style::default().fg(Color::DarkGray)))
            .collect();

        let log_list = List::new(log_items)
            .block(Block::default().borders(Borders::ALL).title("Dream Output"));
        f.render_widget(log_list, right_chunks[1]);
    } else {
        let info = Paragraph::new("Select a dream to view details.")
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(info, chunks[1]);
    }
}

#[cfg(feature = "nova")]
pub(crate) fn render_kaleidoscope(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Grid (Piet Canvas)
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();

            // Background Color from Chroma
            let chroma = &vm.chroma_grid[y][x];
            if let Some((r, g, b)) = chroma.fg {
                style = style.bg(Color::Rgb(r, g, b));
            } else {
                style = style.bg(Color::White); // Default white canvas
            }

            // Cursor
            let mut ch = "  ".to_string();
            if app_state.grid_cursor == (x, y) {
                ch = "[]".to_string();
                style = style.fg(Color::Black).add_modifier(Modifier::BOLD);
            }

            // Piet DP/CC if active
            if let Some(state) = &vm.piet_state {
                if state.y == y && state.x == x {
                    let arrow = match state.dp {
                        crate::vm::piet::Direction::Right => ">",
                        crate::vm::piet::Direction::Down => "v",
                        crate::vm::piet::Direction::Left => "<",
                        crate::vm::piet::Direction::Up => "^",
                    };
                    ch = format!(
                        "{}{}",
                        arrow,
                        if state.cc == crate::vm::piet::CodelChooser::Left {
                            "L"
                        } else {
                            "R"
                        }
                    );
                    style = style.fg(Color::Black).add_modifier(Modifier::BOLD);
                }
            }

            line_spans.push(Span::styled(ch, style));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Kaleidoscope (Piet Canvas)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Palette & State
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    // Palette
    let mut palette_lines = Vec::new();
    let hues = ["Red", "Yellow", "Green", "Cyan", "Blue", "Magenta"];
    let lights = ["Light", "Normal", "Dark"];

    for (l_idx, light) in lights.iter().enumerate() {
        let mut spans = Vec::new();
        spans.push(Span::raw(format!("{:<8}", light)));

        for (h_idx, _hue) in hues.iter().enumerate() {
            let r = match h_idx {
                0 => 255,
                1 => 255,
                2 => 0,
                3 => 0,
                4 => 0,
                5 => 255,
                _ => 0,
            };
            let g = match h_idx {
                0 => 0,
                1 => 255,
                2 => 255,
                3 => 255,
                4 => 0,
                5 => 0,
                _ => 0,
            };
            let b = match h_idx {
                0 => 0,
                1 => 0,
                2 => 0,
                3 => 255,
                4 => 255,
                5 => 255,
                _ => 0,
            };

            let (r, g, b) = match l_idx {
                0 => (r + (255 - r) / 2, g + (255 - g) / 2, b + (255 - b) / 2),
                2 => (r / 2, g / 2, b / 2),
                _ => (r, g, b),
            };

            let mut style = Style::default().bg(Color::Rgb(r as u8, g as u8, b as u8));
            let mut text = "  ".to_string();

            if app_state.kaleidoscope_hue_idx == h_idx && app_state.kaleidoscope_light_idx == l_idx
            {
                style = style.fg(Color::Black).add_modifier(Modifier::BOLD);
                text = "XX".to_string();
            }

            spans.push(Span::styled(text, style));
            spans.push(Span::raw(" "));
        }
        palette_lines.push(Line::from(spans));
    }

    // Black & White
    let mut bw_spans = Vec::new();
    bw_spans.push(Span::raw("Special:  "));
    bw_spans.push(Span::styled("  ", Style::default().bg(Color::White))); // White
    bw_spans.push(Span::raw(" "));
    bw_spans.push(Span::styled("  ", Style::default().bg(Color::Black))); // Black
    palette_lines.push(Line::from(bw_spans));

    let palette_widget = Paragraph::new(palette_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Palette (Space: Paint, []: Hue, {}: Light)"),
    );
    f.render_widget(palette_widget, right_chunks[0]);

    // State Info
    let mut info_lines = Vec::new();
    info_lines.push(Line::from("Controls: S: Step, R: Reset, Arrows: Move"));

    if let Some(state) = &vm.piet_state {
        info_lines.push(Line::from(""));
        info_lines.push(Line::from(format!("Steps: {}", state.steps)));
        info_lines.push(Line::from(format!(
            "DP: {:?} | CC: {:?}",
            state.dp, state.cc
        )));
        info_lines.push(Line::from(format!("Pos: {},{}", state.x, state.y)));

        info_lines.push(Line::from(""));
        info_lines.push(Line::from("Stack (Top):"));
        for val in state.stack.iter().rev().take(10) {
            info_lines.push(Line::from(format!("  {}", val)));
        }
    } else {
        info_lines.push(Line::from(""));
        info_lines.push(Line::from("Piet Interpreter Inactive. Press 'S' to start."));
    }

    let info_widget = Paragraph::new(info_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Interpreter State"),
    );
    f.render_widget(info_widget, right_chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_bestiary(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Organelle List
    let mut items = Vec::new();
    if vm.organelles.is_empty() {
        items.push(ListItem::new("No active organelles."));
    } else {
        for (i, org) in vm.organelles.iter().enumerate() {
            let style = if i == app_state.selected_organelle_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            items.push(ListItem::new(format!("{} [{:?}]", org.name, org.kind)).style(style));
        }
    }
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Bestiary (Active Agents)"),
    );
    f.render_widget(list, chunks[0]);

    // Details
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    if !vm.organelles.is_empty() && app_state.selected_organelle_index < vm.organelles.len() {
        let org = &vm.organelles[app_state.selected_organelle_index];

        // Face
        let face_lines = crate::vm::nova_bestiary::generate_face(org.genome_id, &org.traits);
        let mut face_text = Vec::new();
        for line in face_lines {
            face_text.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Cyan),
            )));
        }
        let face_widget = Paragraph::new(face_text)
            .block(Block::default().borders(Borders::ALL).title("Portrait"));
        f.render_widget(face_widget, right_chunks[0]);

        // Stats
        let stats = vec![
            Line::from(format!("Name: {}", org.name)),
            Line::from(format!("Type: {:?}", org.kind)),
            Line::from(format!("Genome ID: {:x}", org.genome_id)),
            Line::from(format!("Traits: {:?}", org.traits)),
            Line::from(format!("Location: {:?}", org.context_loc)),
            Line::from(format!("Stack Depth: {}", org.stack.len())),
            Line::from(format!("IP: {:?}", org.ip)),
            Line::from(format!("Direction: {:?}", org.direction)),
        ];

        let stats_widget =
            Paragraph::new(stats).block(Block::default().borders(Borders::ALL).title("Vitals"));
        f.render_widget(stats_widget, right_chunks[1]);
    } else {
        let info = Paragraph::new("Select an organelle to inspect.")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(info, chunks[1]);
    }
}
