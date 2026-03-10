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
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub(crate) const GRIMOIRE_TEXT: &str = include_str!("../../GRIMOIRE.md");

pub mod state;
pub use state::*;
pub(crate) mod views;
use views::*;
pub(crate) fn panel_block<'a>(title: &'a str, active: bool) -> Block<'a> {
    let border_style = if active {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };

    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title,
            if active {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            },
        ))
        .border_style(border_style)
}

pub fn run_tui(
    mut vm: ChimeraVM,
    initial_view: Option<ViewMode>,
    source_path: Option<std::path::PathBuf>,
) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new(initial_view, source_path);

    // Initialize Evolution Engine if config is present
    if let Some(config) = &vm.dna.evolution_config {
        if let Some(strand) = vm.dna.helix.strands.first() {
            app_state.evolution_state.engine = Some(
                crate::vm::evolution::EvolutionEngine::from_config(strand.clone(), config.clone()),
            );
            app_state.evolution_state.challenge =
                crate::vm::evolution::Challenge::Custom(config.clone());
            app_state.view_mode = ViewMode::Evolution; // Auto-switch to view
        }
    }

    let res = run_app(&mut terminal, &mut vm, &mut app_state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

pub(crate) fn parse_grid_value(s: &str) -> crate::vm::Value {
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
        // Hot Reload Check
        app_state.last_check_tick = app_state.last_check_tick.wrapping_add(1);
        if app_state.last_check_tick.is_multiple_of(10) {
            if let Some(path) = &app_state.source_path {
                if let Ok(metadata) = std::fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        let should_reload = match app_state.last_modified {
                            Some(last) => modified > last,
                            None => true,
                        };

                        if should_reload {
                            let mut src = String::new();
                            if std::fs::File::open(path).and_then(|mut f| {
                                use std::io::Read;
                                f.take(10 * 1024 * 1024).read_to_string(&mut src)
                            }).is_ok() {
                                // Default to ChimeraScript for hot reload for now
                                // Ideally we check extension, but compile() handles imports
                                let parent = path.parent();
                                if let Ok(new_dna) = crate::compiler::compile(&src, parent) {
                                    vm.patch_dna(new_dna);
                                    app_state.last_modified = Some(modified);
                                    app_state.status_msg = "Hot Reloaded!".to_string();
                                    app_state.screen_shake = 5.0;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Process TuiEvents
        for event in vm.tui_events.drain(..) {
            match event {
                crate::vm::TuiEvent::Glitch(v) => vm.glitch_level = v,
                crate::vm::TuiEvent::Shake(v) => app_state.screen_shake = v,
                crate::vm::TuiEvent::Message(s) => app_state.status_msg = s,
            }
        }

        // Decay Shake
        if app_state.screen_shake > 0.0 {
            app_state.screen_shake *= 0.9;
            if app_state.screen_shake < 0.1 {
                app_state.screen_shake = 0.0;
            }
        }

        let size = terminal.size()?;
        app_state.matrix_rain.update(size.width, size.height);

        if let ViewMode::Evolution = app_state.view_mode {
            if app_state.evolution_state.auto_run {
                if let Some(engine) = &mut app_state.evolution_state.engine {
                    engine.step(vm);
                }
            }
        }

        if let ViewMode::Sequencer = app_state.view_mode {
            if app_state.sequencer_state.playing {
                app_state.sequencer_state.tick += 1;

                #[cfg(feature = "resonance")]
                {
                    if let Some(tx) = &vm.audio_tx {
                        let tick = app_state.sequencer_state.tick;
                        for strand in &vm.dna.helix.strands {
                            if tick < strand.genes.len() {
                                let gene = &strand.genes[tick];
                                let freq = match gene.op {
                                    crate::opcode::OpCode::Push => 110.0,
                                    crate::opcode::OpCode::Add => 220.0,
                                    crate::opcode::OpCode::Sub => 440.0,
                                    crate::opcode::OpCode::Jump => 55.0,
                                    _ => gene.op.to_string().len() as f32 * 50.0 + 200.0,
                                };

                                use resonance_audio::audio::AudioCommand;
                                let _ = tx.send(AudioCommand::Tone {
                                    x: 0,
                                    y: 0,
                                    frequency: freq,
                                    strength: 0.5,
                                    duration_ms: 100,
                                });
                            }
                        }
                    }
                }
            }
        }

        #[cfg(feature = "nova")]
        if let ViewMode::Fishing = app_state.view_mode {
            if app_state.fishing_cast {
                // Bobber float animation
                app_state.fishing_bobber_y += (rand::random::<f64>() - 0.5) * 0.5;

                // Random Hook
                if !app_state.fishing_hooked && rand::random::<f64>() < 0.01 {
                    app_state.fishing_hooked = true;
                    app_state.status_msg = "FISH HOOKED! REEL IT IN!".to_string();
                }

                if app_state.fishing_hooked {
                    // Fish fights back (Randomly pulls)
                    if rand::random::<f64>() < 0.2 {
                        app_state.fishing_tension += 0.02;
                    } else {
                        // Passive decay when not being pulled
                        app_state.fishing_tension -= 0.002;
                    }
                    app_state.fishing_fish_y =
                        app_state.fishing_bobber_y + (rand::random::<f64>() - 0.5) * 2.0;
                } else {
                    app_state.fishing_tension -= 0.01;
                }

                // Clamp tension
                app_state.fishing_tension = app_state.fishing_tension.clamp(0.0, 1.1); // Allow slight over for snap check

                if app_state.fishing_tension >= 1.0 {
                    app_state.status_msg = "SNAP! Line broke.".to_string();
                    app_state.fishing_cast = false;
                    app_state.fishing_hooked = false;
                    app_state.fishing_tension = 0.0;
                    app_state.screen_shake = 2.0;
                }
            }
        }

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
            #[cfg(feature = "nova")]
            if matches!(app_state.view_mode, ViewMode::Terminal | ViewMode::Void) {
                let area = app_state.get_render_area(f.area());
                app_state.matrix_rain.render(f.buffer_mut(), area);
            }

            if let ViewMode::Microscope = app_state.view_mode {
                render_microscope(f, vm, app_state);
                return;
            }

            #[cfg(feature = "biophysics")]
            if let ViewMode::Cortex = app_state.view_mode {
                render_cortex(f, vm, app_state);
                return;
            }

            #[cfg(feature = "resonance")]
            if let ViewMode::Resonance = app_state.view_mode {
                render_resonance(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Grimoire = app_state.view_mode {
                render_grimoire(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Topology = app_state.view_mode {
                render_topology(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Laboratory = app_state.view_mode {
                render_laboratory(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Graveyard = app_state.view_mode {
                render_graveyard(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Retina = app_state.view_mode {
                render_retina(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Quantum = app_state.view_mode {
                render_quantum(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Dream = app_state.view_mode {
                render_dream(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Phylogeny = app_state.view_mode {
                render_phylogeny(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Alchemy = app_state.view_mode {
                render_alchemy(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::PianoRoll = app_state.view_mode {
                render_piano_roll(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Memetics = app_state.view_mode {
                render_memetics(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Egregore = app_state.view_mode {
                render_egregore(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Bestiary = app_state.view_mode {
                render_bestiary(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Kaleidoscope = app_state.view_mode {
                render_kaleidoscope(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Void = app_state.view_mode {
                render_void(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Signals = app_state.view_mode {
                render_signals(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Sovereignty = app_state.view_mode {
                render_sovereignty(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Spectrogram = app_state.view_mode {
                render_spectrogram(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Market = app_state.view_mode {
                render_market(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Ballistics = app_state.view_mode {
                render_ballistics(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Scent = app_state.view_mode {
                render_scent(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Fishing = app_state.view_mode {
                render_fishing(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Arena = app_state.view_mode {
                render_arena(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Garden = app_state.view_mode {
                render_garden(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Orca = app_state.view_mode {
                render_orca(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Babel = app_state.view_mode {
                render_babel(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Strings = app_state.view_mode {
                render_strings(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Quipu = app_state.view_mode {
                render_quipu(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Hydra = app_state.view_mode {
                render_hydra(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Chronos = app_state.view_mode {
                render_chronos(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Logos = app_state.view_mode {
                render_logos(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Pandemonium = app_state.view_mode {
                render_pandemonium(f, vm, app_state);
                return;
            }

            if let ViewMode::BioticChaos = app_state.view_mode {
                render_biotic_chaos(f, vm, app_state);
                return;
            }

            if let ViewMode::Catalyst = app_state.view_mode {
                render_catalyst(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Hyperspace = app_state.view_mode {
                render_hyperspace(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Hologram = app_state.view_mode {
                render_hologram(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Weaver = app_state.view_mode {
                render_weaver(f, vm, app_state);
                return;
            }

            if let ViewMode::Heatmap = app_state.view_mode {
                render_heatmap(f, vm, app_state);
                return;
            }

            #[cfg(feature = "silicon")]
            if let ViewMode::Schematic = app_state.view_mode {
                render_schematic(f, vm, app_state);
                return;
            }

            #[cfg(feature = "silicon")]
            if let ViewMode::Foundry = app_state.view_mode {
                render_foundry(f, vm, app_state);
                return;
            }

            #[cfg(feature = "elektra")]
            if let ViewMode::Elektra = app_state.view_mode {
                render_elektra(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Terminal = app_state.view_mode {
                render_terminal(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Attractor = app_state.view_mode {
                render_attractor(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Virology = app_state.view_mode {
                render_virology(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::BioMesh = app_state.view_mode {
                render_biomesh(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Crispr = app_state.view_mode {
                render_crispr(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Reactor = app_state.view_mode {
                render_reactor(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Biolum = app_state.view_mode {
                render_biolum(f, vm, app_state);
                return;
            }

            if let ViewMode::Evolution = app_state.view_mode {
                render_evolution(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Ecology = app_state.view_mode {
                render_ecology(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::LifeCycle = app_state.view_mode {
                render_lifecycle(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Semiotics = app_state.view_mode {
                render_semiotics(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Fractal = app_state.view_mode {
                render_fractal(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Metazoa = app_state.view_mode {
                render_metazoa(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Genesis = app_state.view_mode {
                render_genesis(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Cambrian = app_state.view_mode {
                render_cambrian(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Savant = app_state.view_mode {
                render_savant(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Akashic = app_state.view_mode {
                render_akashic(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Prologue = app_state.view_mode {
                render_prologue(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Lexicon = app_state.view_mode {
                render_lexicon(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Narrative = app_state.view_mode {
                render_narrative(f, vm, app_state);
                return;
            }

            if let ViewMode::Sequencer = app_state.view_mode {
                render_sequencer(f, vm, app_state);
                return;
            }

            if let ViewMode::Mutagen = app_state.view_mode {
                render_mutagen(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Forge = app_state.view_mode {
                render_forge(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Tesseract = app_state.view_mode {
                render_tesseract(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Choir = app_state.view_mode {
                render_choir(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Paradox = app_state.view_mode {
                render_paradox(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Codex = app_state.view_mode {
                render_codex(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Verbum = app_state.view_mode {
                render_verbum(f, vm, app_state);
                return;
            }

            render_genome_and_grid(f, vm, app_state);

            if vm.glitch_level > 0.01 {
                apply_glitch_fx(f.buffer_mut(), vm.glitch_level);
            }

            if vm.glitch_level > 0.8 {
                let area = app_state.get_render_area(f.area());
                let warning_area = ratatui::layout::Rect {
                    x: area.width.saturating_sub(20) / 2,
                    y: 0,
                    width: 20,
                    height: 1,
                };
                let warning = Paragraph::new("ENTROPY STORM")
                    .style(
                        Style::default()
                            .bg(Color::Red)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                    )
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(warning, warning_area);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app_state.show_view_selector {
                    let views = get_all_views();
                    let mut list_state = app_state.view_selector_state.borrow_mut();
                    let selected = list_state.selected().unwrap_or(0);

                    match key.code {
                        KeyCode::Esc => app_state.show_view_selector = false,
                        KeyCode::Up => {
                            if selected > 0 {
                                list_state.select(Some(selected - 1));
                            } else {
                                list_state.select(Some(views.len().saturating_sub(1)));
                            }
                        }
                        KeyCode::Down => {
                            if selected + 1 < views.len() {
                                list_state.select(Some(selected + 1));
                            } else {
                                list_state.select(Some(0));
                            }
                        }
                        KeyCode::Enter => {
                            if selected < views.len() {
                                app_state.view_mode = views[selected].0;
                            }
                            app_state.show_view_selector = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Verbum = app_state.view_mode {
                    // Reuse alchemy_strand_idx as the selected word ID
                    // Clone basic info to avoid holding borrow on VM
                    let mut words: Vec<(usize, String)> = vm
                        .verbum_forge
                        .words
                        .values()
                        .map(|w| (w.id, w.name.clone()))
                        .collect();
                    words.sort_by_key(|w| w.0);

                    match key.code {
                        KeyCode::Up => {
                            // Find current index
                            if let Some(pos) = words
                                .iter()
                                .position(|w| w.0 == app_state.alchemy_strand_idx)
                            {
                                if pos > 0 {
                                    app_state.alchemy_strand_idx = words[pos - 1].0;
                                }
                            } else if !words.is_empty() {
                                app_state.alchemy_strand_idx = words[0].0;
                            }
                        }
                        KeyCode::Down => {
                            if let Some(pos) = words
                                .iter()
                                .position(|w| w.0 == app_state.alchemy_strand_idx)
                            {
                                if pos + 1 < words.len() {
                                    app_state.alchemy_strand_idx = words[pos + 1].0;
                                }
                            } else if !words.is_empty() {
                                app_state.alchemy_strand_idx = words[0].0;
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(word) =
                                words.iter().find(|w| w.0 == app_state.alchemy_strand_idx)
                            {
                                let name = word.1.clone();
                                // Get word data first (immut borrow)
                                let word_data = vm.verbum_forge.get_word_data(&name);

                                if let Some((genes, cost)) = word_data {
                                    if vm.energy >= cost {
                                        vm.energy -= cost;
                                        // Execute manually
                                        for gene in genes {
                                            let _ = vm.execute_gene_inner(gene.op, &gene.args);
                                        }
                                        app_state.status_msg = format!("Invoked Word: {}", name);
                                        app_state.screen_shake = 1.0;
                                    } else {
                                        app_state.status_msg =
                                            format!("Not enough energy to speak '{}'", name);
                                    }
                                } else {
                                    app_state.status_msg = format!("Word '{}' fading...", name);
                                }
                            }
                        }
                        KeyCode::Char('F') => {
                            // Forge from current strand (simple random name for now)
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let s_idx = app_state.selected_strand;
                            if s_idx < vm.dna.helix.strands.len() {
                                let genes = vm.dna.helix.strands[s_idx].genes.clone();
                                let suffix = rng.gen_range(100..999);
                                let name = format!("Word{}", suffix);
                                match vm.verbum_forge.forge(name.clone(), genes, vec![]) {
                                    Ok(_) => {
                                        app_state.status_msg = format!("Forged: {}", name);
                                        app_state.screen_shake = 2.0;
                                    }
                                    Err(e) => {
                                        app_state.status_msg = format!("Forge Error: {}", e);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                if app_state.palette_open {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('p') => app_state.palette_open = false,
                        KeyCode::Up => {
                            if app_state.palette_idx >= 4 {
                                app_state.palette_idx -= 4;
                            }
                        }
                        KeyCode::Down => {
                            if app_state.palette_idx + 4 < 16 {
                                app_state.palette_idx += 4;
                            }
                        }
                        KeyCode::Left => {
                            if app_state.palette_idx > 0 {
                                app_state.palette_idx -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app_state.palette_idx + 1 < 16 {
                                app_state.palette_idx += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let chars = [
                                '*', 'o', 'x', '^', 'v', '<', '>', '+', '-', '/', '%', '!', '=',
                                ':', ';', '?',
                            ];
                            if app_state.palette_idx < chars.len() {
                                app_state.palette_char = Some(chars[app_state.palette_idx]);
                            }
                            app_state.palette_open = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Grimoire = app_state.view_mode {
                    match key.code {
                        KeyCode::Up => {
                            app_state.grimoire_scroll = app_state.grimoire_scroll.saturating_sub(1);
                            continue;
                        }
                        KeyCode::Down => {
                            app_state.grimoire_scroll = app_state.grimoire_scroll.saturating_add(1);
                            continue;
                        }
                        _ => {}
                    }
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Terminal = app_state.view_mode {
                    match key.code {
                        KeyCode::Enter => {
                            let input = app_state.terminal_input.clone();
                            if !input.is_empty() {
                                app_state.terminal_history.push(input.clone());
                                app_state.terminal_history_idx = app_state.terminal_history.len();
                                app_state.terminal_input.clear();

                                vm.output.push(format!("> {}", input));

                                if input.trim_start().starts_with('(') {
                                    // Lisp Mode
                                    match crate::lisp::compile_fragment(&input) {
                                        Ok(genes) => {
                                            for gene in genes {
                                                let _ = vm.execute_gene_inner(gene.op, &gene.args);
                                            }
                                            vm.output.push("LISP: Executed.".to_string());
                                        }
                                        Err(e) => {
                                            vm.output.push(format!("Lisp Error: {}", e));
                                        }
                                    }
                                } else {
                                    // ChimeraScript Mode
                                    let src = format!("strand terminal_input {{ {} }}", input);
                                    match crate::compiler::compile(&src, None) {
                                        Ok(dna) => {
                                            if let Some(strand) = dna.helix.strands.first() {
                                                // Execute immediately to mimic REPL
                                                for gene in &strand.genes {
                                                    let _ = vm.execute_gene_inner(
                                                        gene.op.clone(),
                                                        &gene.args,
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            vm.output.push(format!("Error: {}", e));
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Up => {
                            if app_state.terminal_history_idx > 0 {
                                app_state.terminal_history_idx -= 1;
                                app_state.terminal_input = app_state.terminal_history
                                    [app_state.terminal_history_idx]
                                    .clone();
                            }
                        }
                        KeyCode::Down => {
                            if app_state.terminal_history_idx + 1 < app_state.terminal_history.len()
                            {
                                app_state.terminal_history_idx += 1;
                                app_state.terminal_input = app_state.terminal_history
                                    [app_state.terminal_history_idx]
                                    .clone();
                            } else {
                                app_state.terminal_history_idx = app_state.terminal_history.len();
                                app_state.terminal_input.clear();
                            }
                        }
                        KeyCode::Char(c) => {
                            app_state.terminal_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.terminal_input.pop();
                        }
                        _ => {}
                    }
                    continue;
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Codex = app_state.view_mode {
                    match key.code {
                        KeyCode::Up => {
                            if app_state.codex_selected_spell > 0 {
                                app_state.codex_selected_spell -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app_state.codex_selected_spell + 1 < vm.codex.spells.len() {
                                app_state.codex_selected_spell += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(spell) = vm.codex.get_spell(app_state.codex_selected_spell)
                            {
                                crate::vm::codex::exec_spell(vm, &spell);
                                app_state.screen_shake = 2.0;
                                app_state.status_msg = format!("Cast Spell: {}", spell.name);
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                if let ViewMode::Mutagen = app_state.view_mode {
                    let mut handled = true;
                    match key.code {
                        KeyCode::Up => {
                            app_state.selected_gene = app_state.selected_gene.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            if let Some(strand) =
                                vm.dna.helix.strands.get(app_state.selected_strand)
                            {
                                if app_state.selected_gene + 1 < strand.genes.len() {
                                    app_state.selected_gene += 1;
                                }
                            }
                        }
                        KeyCode::Left => {
                            app_state.selected_strand = app_state.selected_strand.saturating_sub(1);
                            app_state.selected_gene = 0;
                        }
                        KeyCode::Right => {
                            if app_state.selected_strand + 1 < vm.dna.helix.strands.len() {
                                app_state.selected_strand += 1;
                                app_state.selected_gene = 0;
                            }
                        }
                        KeyCode::Char('M') => {
                            crate::vm::pandemonium::apply_mutation(
                                vm,
                                app_state.selected_strand,
                                app_state.selected_gene,
                            );
                            app_state.status_msg = "Mutated!".to_string();
                            app_state.screen_shake = 1.0;
                        }
                        _ => {
                            handled = false;
                        }
                    }
                    if handled {
                        continue;
                    }
                }

                if let ViewMode::Sequencer = app_state.view_mode {
                    let mut handled = true;
                    match key.code {
                        KeyCode::Char(' ') => {
                            app_state.sequencer_state.playing = !app_state.sequencer_state.playing;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app_state.sequencer_state.bpm =
                                app_state.sequencer_state.bpm.saturating_add(10);
                        }
                        KeyCode::Char('-') => {
                            app_state.sequencer_state.bpm =
                                app_state.sequencer_state.bpm.saturating_sub(10).max(10);
                        }
                        KeyCode::Left => {
                            app_state.sequencer_state.tick =
                                app_state.sequencer_state.tick.saturating_sub(1);
                            if app_state.sequencer_state.tick < app_state.sequencer_state.scroll_x {
                                app_state.sequencer_state.scroll_x = app_state.sequencer_state.tick;
                            }
                        }
                        KeyCode::Right => {
                            app_state.sequencer_state.tick += 1;
                            if app_state.sequencer_state.tick
                                > app_state.sequencer_state.scroll_x + 80
                            {
                                app_state.sequencer_state.scroll_x =
                                    app_state.sequencer_state.tick - 80;
                            }
                        }
                        KeyCode::Char('s') => {
                            crate::vm::pandemonium::apply_scramble(
                                vm,
                                0,
                                app_state.sequencer_state.tick,
                                5.0,
                            );
                            app_state.status_msg = "Sequencer: Scrambled!".to_string();
                        }
                        KeyCode::Char('m') => {
                            crate::vm::pandemonium::apply_mutation(
                                vm,
                                0,
                                app_state.sequencer_state.tick,
                            );
                            app_state.status_msg = "Sequencer: Mutated!".to_string();
                        }
                        _ => {
                            handled = false;
                        }
                    }
                    if handled {
                        continue;
                    }
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Virology = app_state.view_mode {
                    let mut handled = true;
                    match key.code {
                        KeyCode::Tab => {
                            app_state.virus_design_focus = (app_state.virus_design_focus + 1) % 5;
                        }
                        KeyCode::Enter => {
                            app_state.input_buffer = match app_state.virus_design_focus {
                                0 => app_state.virus_design_name.clone(),
                                1 => app_state.virus_design_pattern.clone(),
                                2 => app_state.virus_design_rate.to_string(),
                                3 => app_state.virus_design_payload.to_string(),
                                _ => String::new(),
                            };

                            if app_state.virus_design_focus != 4 {
                                app_state.input_mode = InputMode::Editing;
                            }
                        }
                        KeyCode::Up => {
                            if app_state.virus_design_focus == 4 && app_state.virus_design_mode > 0
                            {
                                app_state.virus_design_mode -= 1;
                            }
                            #[cfg(feature = "nova")]
                            if let ViewMode::Forge = app_state.view_mode {
                                if app_state.forge_focus == 0 {
                                    // Move selection up
                                    let keys: Vec<_> = vm
                                        .prologue_state
                                        .logos_engine
                                        .rules
                                        .keys()
                                        .cloned()
                                        .collect();
                                    let mut sorted_keys = keys;
                                    sorted_keys.sort();
                                    if let Some(pos) = sorted_keys
                                        .iter()
                                        .position(|k| *k == app_state.forge_selected_rule)
                                    {
                                        if pos > 0 {
                                            app_state.forge_selected_rule =
                                                sorted_keys[pos - 1].clone();
                                        }
                                    } else if !sorted_keys.is_empty() {
                                        app_state.forge_selected_rule = sorted_keys[0].clone();
                                    }
                                }
                            }
                        }
                        KeyCode::Down => {
                            if app_state.virus_design_focus == 4 && app_state.virus_design_mode < 2
                            {
                                app_state.virus_design_mode += 1;
                            }
                            #[cfg(feature = "nova")]
                            if let ViewMode::Forge = app_state.view_mode {
                                if app_state.forge_focus == 0 {
                                    // Move selection down
                                    let keys: Vec<_> = vm
                                        .prologue_state
                                        .logos_engine
                                        .rules
                                        .keys()
                                        .cloned()
                                        .collect();
                                    let mut sorted_keys = keys;
                                    sorted_keys.sort();
                                    if let Some(pos) = sorted_keys
                                        .iter()
                                        .position(|k| *k == app_state.forge_selected_rule)
                                    {
                                        if pos + 1 < sorted_keys.len() {
                                            app_state.forge_selected_rule =
                                                sorted_keys[pos + 1].clone();
                                        }
                                    } else if !sorted_keys.is_empty() {
                                        app_state.forge_selected_rule = sorted_keys[0].clone();
                                    }
                                }
                            }
                        }
                        KeyCode::Char('S') => {
                            let mode = match app_state.virus_design_mode {
                                1 => crate::vm::memetics::VirusMode::RewriteGrid,
                                2 => crate::vm::memetics::VirusMode::RewriteDNA,
                                _ => crate::vm::memetics::VirusMode::Overwrite,
                            };
                            let payload = if app_state.virus_design_payload >= 0 {
                                Some(app_state.virus_design_payload as usize)
                            } else {
                                None
                            };

                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let color = (
                                rng.gen_range(50..255),
                                rng.gen_range(50..255),
                                rng.gen_range(50..255),
                            );

                            let virus = crate::vm::memetics::Virus {
                                name: app_state.virus_design_name.clone(),
                                color,
                                pattern: app_state.virus_design_pattern.clone(),
                                mutation_rate: app_state.virus_design_rate,
                                payload,
                                grammar: None,
                                quorum_threshold: 0,
                                quorum_action: None,
                                mode,
                            };
                            vm.virus_library.push(virus);
                            app_state.status_msg = "Virus Synthesized!".to_string();
                        }
                        KeyCode::Char('I') => {
                            if !vm.virus_library.is_empty() {
                                let v_id = vm.virus_library.len() - 1;
                                let (x, y) = app_state.grid_cursor;
                                vm.viral_grid[y][x] = Some(crate::vm::memetics::ViralState {
                                    infection_level: 100,
                                    virus_id: v_id,
                                });
                                app_state.status_msg =
                                    format!("Injected Virus ID {} at {},{}", v_id, x, y);
                            } else {
                                app_state.status_msg =
                                    "Library Empty! Synthesize (S) first.".to_string();
                            }
                        }
                        KeyCode::Char(' ') => {
                            crate::vm::memetics::exec_memetics_op(
                                vm,
                                crate::opcode::OpCode::Outbreak,
                                &[],
                            );
                            app_state.status_msg = "Outbreak Simulated.".to_string();
                        }
                        _ => {
                            handled = false;
                        }
                    }
                    if handled {
                        continue;
                    }
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Pandemonium = app_state.view_mode {
                    match key.code {
                        KeyCode::Left => app_state.pandemonium_cursor.0 -= 1.0,
                        KeyCode::Right => app_state.pandemonium_cursor.0 += 1.0,
                        KeyCode::Up => app_state.pandemonium_cursor.1 -= 1.0,
                        KeyCode::Down => app_state.pandemonium_cursor.1 += 1.0,
                        KeyCode::Char('[') => {
                            app_state.pandemonium_radius =
                                (app_state.pandemonium_radius - 0.5).max(0.5);
                        }
                        KeyCode::Char(']') => app_state.pandemonium_radius += 0.5,
                        KeyCode::Char('1') => app_state.pandemonium_selected_tool = 0,
                        KeyCode::Char('2') => app_state.pandemonium_selected_tool = 1,
                        KeyCode::Char('3') => app_state.pandemonium_selected_tool = 2,
                        KeyCode::Char('4') => app_state.pandemonium_selected_tool = 3,
                        KeyCode::Char('5') => app_state.pandemonium_selected_tool = 4,
                        KeyCode::Char(' ') => {
                            let (cx, cy) = app_state.pandemonium_cursor;
                            let radius = app_state.pandemonium_radius;
                            let mut linear_idx = 0;
                            let mut hits = Vec::new();

                            for (s_idx, strand) in vm.dna.helix.strands.iter().enumerate() {
                                for (g_idx, _gene) in strand.genes.iter().enumerate() {
                                    let theta = (linear_idx as f64) * 0.1;
                                    let r = theta * 0.5;
                                    let x = r * theta.cos();
                                    let y = r * theta.sin();

                                    let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                                    if dist <= radius {
                                        hits.push((s_idx, g_idx));
                                    }
                                    linear_idx += 1;
                                }
                            }

                            // Sort hits descending to avoid index shift issues on deletion
                            hits.sort_by(|a, b| b.cmp(a));

                            for (s, g) in hits {
                                match app_state.pandemonium_selected_tool {
                                    0 => crate::vm::pandemonium::apply_mutation(vm, s, g),
                                    1 => crate::vm::pandemonium::apply_scramble(vm, s, g, radius),
                                    2 => crate::vm::pandemonium::apply_purge(vm, s, g, radius),
                                    3 => crate::vm::pandemonium::apply_duplicate(vm, s, g),
                                    4 => crate::vm::pandemonium::apply_storm(vm, s, g, radius),
                                    _ => {}
                                }
                            }
                            app_state.status_msg = "Pandemonium Unleashed!".to_string();
                            app_state.screen_shake = 2.0;
                        }
                        _ => {}
                    }
                    continue;
                }

                #[cfg(feature = "nova")]
                if let ViewMode::Weaver = app_state.view_mode {
                    let mut handled = true;
                    match key.code {
                        KeyCode::Char('S') => {
                            // Stitch: Weave Strands
                            // Stack Args: [A, B, Pattern]
                            vm.stack
                                .push(crate::vm::Value::Int(app_state.lab_parent_a as i64));
                            vm.stack
                                .push(crate::vm::Value::Int(app_state.lab_parent_b as i64));
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.input_buffer.clone()));

                            // Execute Weave OpCode
                            let _ = vm.execute_gene_inner(crate::opcode::OpCode::Weave, &[]);

                            // Check result (Weave pushes new strand index or error)
                            if let Some(res) = vm.stack.pop() {
                                app_state.status_msg = format!("Weaver Result: {}", res);
                                // If successful (Int), switch to it?
                                if let crate::vm::Value::Int(idx) = res {
                                    app_state.selected_strand = idx as usize;
                                }
                            }
                            app_state.screen_shake = 1.0;
                        }
                        KeyCode::Char('R') => {
                            // Randomize Pattern
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let len = rng.gen_range(8..32);
                            let chars = ['A', 'B', 'X', '0'];
                            let pattern: String =
                                (0..len).map(|_| chars[rng.gen_range(0..4)]).collect();
                            app_state.input_buffer = pattern;
                            app_state.status_msg = "Weaver: Chaos Pattern Generated".to_string();
                        }
                        _ => {
                            handled = false;
                        }
                    }
                    if handled {
                        continue;
                    }
                }

                #[cfg(feature = "oracle")]
                if app_state.query_mode {
                    match key.code {
                        KeyCode::Enter => {
                            let query_str = &app_state.query_input;
                            match ChimeraParser::parse(Rule::gene, query_str) {
                                Ok(mut pairs) => {
                                    let pair = pairs.next().unwrap();
                                    match Gene::try_from_pair(pair) {
                                        Ok(gene) => {
                                            let op_name = gene.op.to_string();
                                            let mut terms = vec![crate::vm::Value::Str(op_name)];

                                            fn nuc_to_val(
                                                n: &crate::ast::Nucleotide,
                                            ) -> crate::vm::Value
                                            {
                                                match n {
                                                    crate::ast::Nucleotide::Number(i) => {
                                                        crate::vm::Value::Int(*i)
                                                    }
                                                    crate::ast::Nucleotide::String(s) => {
                                                        crate::vm::Value::Str(s.clone())
                                                    }
                                                    crate::ast::Nucleotide::Identifier(s) => {
                                                        crate::vm::Value::Str(s.clone())
                                                    }
                                                    crate::ast::Nucleotide::Junction(t, args) => {
                                                        crate::vm::Value::Junction(
                                                            *t,
                                                            args.iter().map(nuc_to_val).collect(),
                                                        )
                                                    }
                                                }
                                            }

                                            for arg in gene.args {
                                                terms.push(nuc_to_val(&arg));
                                            }

                                            let goal = crate::vm::Value::Junction(
                                                crate::ast::JunctionType::Any,
                                                terms,
                                            );
                                            let mut solutions = Vec::new();
                                            crate::vm::oracle::solve(
                                                &[goal],
                                                std::collections::HashMap::new(),
                                                &vm.knowledge_base,
                                                vm,
                                                &mut solutions,
                                                0,
                                            );

                                            app_state.query_results.clear();
                                            if solutions.is_empty() {
                                                app_state.query_results.push("No.".to_string());
                                            } else {
                                                app_state.query_results.push(format!(
                                                    "Yes ({} solutions):",
                                                    solutions.len()
                                                ));
                                                for (i, sol) in solutions.iter().enumerate() {
                                                    let mut s = format!("{}: ", i + 1);
                                                    for (k, v) in sol {
                                                        s.push_str(&format!("{}={} ", k, v));
                                                    }
                                                    if sol.is_empty() {
                                                        s.push_str("true");
                                                    }
                                                    app_state.query_results.push(s);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            app_state.query_results.clear();
                                            app_state
                                                .query_results
                                                .push(format!("Parse Error: {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    app_state.query_results.clear();
                                    app_state.query_results.push(format!("Syntax Error: {}", e));
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app_state.query_mode = false;
                            app_state.query_input.clear();
                            app_state.query_results.clear();
                        }
                        KeyCode::Char(c) => {
                            app_state.query_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.query_input.pop();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Spirit Request
                #[cfg(feature = "nova")]
                if vm.spirit_request {
                    match key.code {
                        KeyCode::Enter => {
                            let val = parse_grid_value(&app_state.input_buffer);
                            vm.spirit_value = Some(val);
                            vm.step(); // Resume
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Esc => {
                            vm.spirit_value = Some(crate::vm::Value::Int(0));
                            vm.step();
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

                // Handle Injection Mode
                if let InputMode::Injection = app_state.input_mode {
                    match key.code {
                        KeyCode::Enter => {
                            let src = format!("strand injection {{ {} }}", app_state.input_buffer);
                            match crate::compiler::compile(&src, None) {
                                Ok(dna) => {
                                    if let Some(strand) = dna.helix.strands.first() {
                                        vm.inject_genes(strand.genes.clone());
                                        app_state.status_msg = "Injection Successful".to_string();
                                    } else {
                                        app_state.status_msg =
                                            "Injection Failed: No genes".to_string();
                                    }
                                }
                                Err(e) => {
                                    app_state.status_msg = format!("Injection Error: {}", e);
                                }
                            }
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
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

                // Handle Editing Mode
                if let InputMode::Editing = app_state.input_mode {
                    match key.code {
                        KeyCode::Enter => {
                            match app_state.view_mode {
                                #[cfg(feature = "nova")]
                                ViewMode::Paradox => {
                                    if !app_state.paradox_editor_buffer.is_empty() {
                                        match vm
                                            .paradox
                                            .parse_rule(&app_state.paradox_editor_buffer)
                                        {
                                            Ok(_) => {
                                                app_state.status_msg =
                                                    "Paradox Rule Compiled.".to_string();
                                                app_state.paradox_editor_buffer.clear();
                                            }
                                            Err(e) => {
                                                app_state.status_msg = format!("Error: {}", e);
                                            }
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                }
                                ViewMode::Genome => {
                                    // Genome Editing Logic
                                    match ChimeraParser::parse(Rule::gene, &app_state.input_buffer)
                                    {
                                        Ok(mut pairs) => {
                                            let pair = pairs.next().unwrap();
                                            match Gene::try_from_pair(pair) {
                                                Ok(gene) => {
                                                    if app_state.selected_strand
                                                        < vm.dna.helix.strands.len()
                                                        && app_state.selected_gene
                                                            < vm.dna.helix.strands
                                                                [app_state.selected_strand]
                                                                .genes
                                                                .len()
                                                    {
                                                        vm.dna.helix.strands
                                                            [app_state.selected_strand]
                                                            .genes[app_state.selected_gene] = gene;
                                                        app_state.status_msg =
                                                            "Gene updated successfully".to_string();
                                                    }
                                                    app_state.input_mode = InputMode::Normal;
                                                    app_state.input_buffer.clear();
                                                }
                                                Err(e) => {
                                                    app_state.status_msg =
                                                        format!("Parse Error: {}", e);
                                                }
                                            }
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
                                #[cfg(feature = "nova")]
                                ViewMode::Crispr => {
                                    // Execute CRISPR Logic
                                    let guide_tokens: Vec<&str> =
                                        app_state.crispr_guide.split_whitespace().collect();
                                    let replace_tokens: Vec<&str> =
                                        app_state.crispr_replace.split_whitespace().collect();
                                    use std::str::FromStr;

                                    let mut guide_ops = Vec::new();
                                    for t in &guide_tokens {
                                        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
                                            guide_ops.push(op);
                                        }
                                    }
                                    let mut replace_genes = Vec::new();
                                    for t in &replace_tokens {
                                        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
                                            replace_genes
                                                .push(crate::ast::Gene { op, args: vec![] });
                                        }
                                    }

                                    if guide_ops.is_empty() {
                                        app_state.crispr_result =
                                            "Error: Empty Guide Pattern".to_string();
                                    } else {
                                        let s_idx = app_state.crispr_target_strand;
                                        if s_idx < vm.dna.helix.strands.len() {
                                            let strand = &mut vm.dna.helix.strands[s_idx];
                                            let mut new_genes = Vec::new();
                                            let mut i = 0;
                                            let mut matches = 0;
                                            while i < strand.genes.len() {
                                                let mut matched = true;
                                                for (j, op) in guide_ops.iter().enumerate() {
                                                    if i + j >= strand.genes.len()
                                                        || strand.genes[i + j].op != *op
                                                    {
                                                        matched = false;
                                                        break;
                                                    }
                                                }
                                                if matched {
                                                    new_genes.extend(replace_genes.clone());
                                                    i += guide_ops.len();
                                                    matches += 1;
                                                } else {
                                                    new_genes.push(strand.genes[i].clone());
                                                    i += 1;
                                                }
                                            }
                                            strand.genes = new_genes;
                                            app_state.crispr_result = format!(
                                                "CRISPR: Replaced {} occurrences.",
                                                matches
                                            );
                                        } else {
                                            app_state.crispr_result =
                                                "Error: Invalid Strand".to_string();
                                        }
                                    }
                                    // Stay in Editing mode
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Chronos => {
                                    // Enable editing grid from Chronos view
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Logos => {
                                    // Enable editing grid from Logos view
                                    let (x, y) = app_state.grid_cursor;
                                    // Should parse as String usually for Atoms
                                    // parse_grid_value handles numbers.
                                    let val = if app_state.input_buffer.starts_with('?') {
                                        // Variable
                                        crate::vm::Value::Str(app_state.input_buffer.clone())
                                    } else {
                                        parse_grid_value(&app_state.input_buffer)
                                    };
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Pandemonium => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Orca => {
                                    // Grid Editing Logic
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Microscope => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "biophysics")]
                                ViewMode::Cortex => {
                                    // No editing for Cortex view yet
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "resonance")]
                                ViewMode::Resonance => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Grimoire => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Laboratory => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Topology => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Graveyard => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::PianoRoll => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Retina => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Quantum => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Heatmap => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "silicon")]
                                ViewMode::Schematic => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Dream => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Phylogeny => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Alchemy => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Memetics => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Egregore => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Bestiary => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Kaleidoscope => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Void => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Signals => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Sovereignty => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Spectrogram => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Market => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Ballistics => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Scent => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Fishing => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Garden => {
                                    // Enable editing for Garden (Sowing rules)
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "elektra")]
                                ViewMode::Elektra => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Arena => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Babel => {
                                    // In Babel, Enter in Normal mode enters Editing mode.
                                    // Editing happens directly on the strings, no buffer commit needed here.
                                    // But we use input_buffer as scratchpad in other modes.
                                    // Here we edit in place.
                                    // So we just clear buffer and exit?
                                    // Wait, if we are in Editing mode, keys append to buffer.
                                    // We need to implement custom handling for Babel in Editing mode loop.
                                    // See below.
                                    app_state.input_mode = InputMode::Normal;
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Strings => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Quipu => {
                                    // Edit cord value?
                                    // Let's allow setting value of active cord
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    if let crate::vm::Value::Int(n) = val {
                                        if let Some(cord) =
                                            vm.quipu.cords.get_mut(vm.quipu.active_cord)
                                        {
                                            *cord = n;
                                            app_state.status_msg = format!(
                                                "Cord {} set to {}",
                                                vm.quipu.active_cord, n
                                            );
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hydra => {
                                    // Enable editing grid from Hydra view
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "silicon")]
                                ViewMode::Foundry => {
                                    // Same as Schematic/Grid?
                                    // Allow editing grid in Foundry
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::BioticChaos => {
                                    // Allow editing Chaos Grid?
                                    // Parse buffer as float
                                    if let Ok(v) = app_state.input_buffer.parse::<f64>() {
                                        let (x, y) = app_state.grid_cursor;
                                        vm.chaos_struct.grid[y][x] = v.clamp(0.0, 1.0);
                                        app_state.status_msg =
                                            format!("Chaos Grid updated at {},{}", x, y);
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Catalyst => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hyperspace => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hologram => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Weaver => {
                                    // Use input buffer as pattern
                                    app_state.input_mode = InputMode::Normal;
                                    // Don't clear buffer, keep it for preview
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Terminal => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Attractor => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Virology => {
                                    match app_state.virus_design_focus {
                                        0 => {
                                            app_state.virus_design_name =
                                                app_state.input_buffer.clone()
                                        }
                                        1 => {
                                            app_state.virus_design_pattern =
                                                app_state.input_buffer.clone()
                                        }
                                        2 => {
                                            if let Ok(n) = app_state.input_buffer.parse::<u8>() {
                                                app_state.virus_design_rate = n.clamp(0, 100);
                                            }
                                        }
                                        3 => {
                                            if let Ok(n) = app_state.input_buffer.parse::<i64>() {
                                                app_state.virus_design_payload = n;
                                            }
                                        }
                                        _ => {}
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Prologue => {
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Lexicon => {
                                    let val = if app_state.input_buffer.len() == 1 {
                                        crate::vm::Value::Str(app_state.input_buffer.clone())
                                    } else {
                                        parse_grid_value(&app_state.input_buffer)
                                    };
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Evolution => {
                                    if let Ok(val) = app_state.input_buffer.parse::<i64>() {
                                        app_state.evolution_state.challenge =
                                            crate::vm::evolution::Challenge::Target(val);
                                        if let Some(engine) = &mut app_state.evolution_state.engine
                                        {
                                            engine.challenge =
                                                crate::vm::evolution::Challenge::Target(val);
                                        }
                                        app_state.status_msg = format!("Target set to {}", val);
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Ecology => {
                                    // Inject Gene into Selected Organelle
                                    let gene_src = app_state.input_buffer.clone();
                                    if !gene_src.is_empty() {
                                        // 1. Compile gene
                                        // We use a hack: wrap in strand to compile, then extract gene
                                        let src = format!("strand injection {{ {} }}", gene_src);
                                        match crate::compiler::compile(&src, None) {
                                            Ok(dna) => {
                                                if let Some(strand) = dna.helix.strands.first() {
                                                    // 2. Inject into selected organelle
                                                    let mut found = false;
                                                    let (cx, cy) = app_state.grid_cursor;
                                                    for org in vm.organelles.iter_mut() {
                                                        if org.context_loc == (cy, cx) {
                                                            // Push to stack or execute immediately?
                                                            // Let's append to their current strand? No, shared DNA.
                                                            // Let's force execute immediately (Interrupt)
                                                            // Or push to their stack?

                                                            // "Mad Science" Injection: Modify the Organelle's IP to a new ephemeral strand?
                                                            // Complicated.
                                                            // Let's just try to execute the genes on the organelle's stack context?
                                                            // VM doesn't support executing genes on organelle directly easily without setting IP.

                                                            // Simplest: Add genes to the end of the Helix, and Jump the organelle there.
                                                            vm.dna
                                                                .helix
                                                                .strands
                                                                .push(strand.clone());
                                                            let new_idx =
                                                                vm.dna.helix.strands.len() - 1;

                                                            // Save current IP to call stack
                                                            org.call_stack.push(org.ip);
                                                            org.ip = (new_idx, 0);

                                                            found = true;
                                                            app_state.status_msg = format!(
                                                                "Injected code into {}",
                                                                org.name
                                                            );
                                                            break;
                                                        }
                                                    }
                                                    if !found {
                                                        app_state.status_msg =
                                                            "No organelle at cursor.".to_string();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                app_state.status_msg =
                                                    format!("Compilation Error: {}", e);
                                            }
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::BioMesh => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Reactor => {
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Biolum => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Fractal => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Metazoa => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Genesis => {
                                    // Commit change based on focus
                                    if app_state.genesis_focus == 0 {
                                        // Compile Editor Code
                                        let src = format!(
                                            "strand genesis {{ {} }}",
                                            app_state.genesis_editor_buffer
                                        );
                                        match crate::compiler::compile(&src, None) {
                                            Ok(dna) => {
                                                if let Some(strand) = dna.helix.strands.first() {
                                                    // Execute immediately
                                                    for gene in &strand.genes {
                                                        vm.execute_gene_inner(
                                                            gene.op.clone(),
                                                            &gene.args,
                                                        );
                                                    }
                                                    app_state.status_msg =
                                                        "Genesis: Executed.".to_string();
                                                }
                                            }
                                            Err(e) => {
                                                app_state.status_msg =
                                                    format!("Compile Error: {}", e)
                                            }
                                        }
                                        // Clear buffer? Maybe keep it for repeated editing.
                                        app_state.input_mode = InputMode::Normal;
                                    } else if app_state.genesis_focus == 1 {
                                        // Update Grammar
                                        match crate::lisp::parse(&app_state.genesis_grammar_buffer)
                                        {
                                            Ok(exprs) => {
                                                // Take the first expression as the grammar
                                                if let Some(expr) = exprs.first() {
                                                    match crate::lisp::sexpr_to_value(expr) {
                                                        Ok(grammar) => {
                                                            vm.active_grammar = grammar;
                                                            app_state.status_msg =
                                                                "Genesis: Grammar Updated."
                                                                    .to_string();
                                                        }
                                                        Err(e) => {
                                                            app_state.status_msg = format!(
                                                                "Value Conversion Error: {}",
                                                                e
                                                            )
                                                        }
                                                    }
                                                } else {
                                                    app_state.status_msg =
                                                        "Error: Empty Grammar".to_string();
                                                }
                                                app_state.input_mode = InputMode::Normal;
                                            }
                                            Err(e) => {
                                                app_state.status_msg = format!("Lisp Error: {}", e)
                                            }
                                        }
                                    } else {
                                        // Grid
                                        let val = parse_grid_value(&app_state.input_buffer);
                                        let (x, y) = app_state.grid_cursor;
                                        vm.grid[y][x] = val;
                                        app_state.input_mode = InputMode::Normal;
                                        app_state.input_buffer.clear();
                                    }
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Forge => {
                                    if app_state.forge_focus == 1 {
                                        // Define Rule
                                        if !app_state.forge_selected_rule.is_empty() {
                                            vm.prologue_state.logos_engine.define_rule(
                                                &app_state.forge_selected_rule,
                                                &app_state.forge_editor_buffer,
                                            );
                                            app_state.status_msg = format!(
                                                "Forge: Rule '{}' updated.",
                                                app_state.forge_selected_rule
                                            );
                                        }
                                    } else if app_state.forge_focus == 2 {
                                        // Test Rule
                                        if !app_state.forge_selected_rule.is_empty() {
                                            match vm.prologue_state.logos_engine.parse_input(
                                                &app_state.forge_selected_rule,
                                                &app_state.forge_test_input,
                                            ) {
                                                Ok(val) => {
                                                    app_state.forge_test_output =
                                                        format!("Success: {}", val);
                                                }
                                                Err(e) => {
                                                    app_state.forge_test_output =
                                                        format!("Error: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Tab =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Babel = app_state.view_mode {
                                app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                app_state.crispr_focus = (app_state.crispr_focus + 1) % 3;
                            }
                        }
                        KeyCode::Esc => {
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Char(c) =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Paradox = app_state.view_mode {
                                app_state.paradox_editor_buffer.push(c);
                            } else if let ViewMode::Forge = app_state.view_mode {
                                if app_state.forge_focus == 1 {
                                    app_state.forge_editor_buffer.push(c);
                                } else if app_state.forge_focus == 2 {
                                    app_state.forge_test_input.push(c);
                                } else if app_state.forge_focus == 0 {
                                    app_state.forge_selected_rule.push(c);
                                }
                            } else if let ViewMode::Genesis = app_state.view_mode {
                                if app_state.genesis_focus == 0 {
                                    app_state.genesis_editor_buffer.push(c);
                                } else if app_state.genesis_focus == 1 {
                                    app_state.genesis_grammar_buffer.push(c);
                                } else {
                                    app_state.input_buffer.push(c);
                                }
                            } else if let ViewMode::Babel = app_state.view_mode {
                                let target = if app_state.babel_focus == 0 {
                                    &mut app_state.babel_pattern
                                } else {
                                    &mut app_state.babel_input
                                };
                                target.push(c);
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                if app_state.crispr_focus == 1 {
                                    app_state.crispr_guide.push(c);
                                } else if app_state.crispr_focus == 2 {
                                    app_state.crispr_replace.push(c);
                                }
                            } else {
                                app_state.input_buffer.push(c);
                            }
                        }
                        KeyCode::Backspace =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Paradox = app_state.view_mode {
                                app_state.paradox_editor_buffer.pop();
                            } else if let ViewMode::Forge = app_state.view_mode {
                                if app_state.forge_focus == 1 {
                                    app_state.forge_editor_buffer.pop();
                                } else if app_state.forge_focus == 2 {
                                    app_state.forge_test_input.pop();
                                } else if app_state.forge_focus == 0 {
                                    app_state.forge_selected_rule.pop();
                                }
                            } else if let ViewMode::Genesis = app_state.view_mode {
                                if app_state.genesis_focus == 0 {
                                    app_state.genesis_editor_buffer.pop();
                                } else if app_state.genesis_focus == 1 {
                                    app_state.genesis_grammar_buffer.pop();
                                } else {
                                    app_state.input_buffer.pop();
                                }
                            } else if let ViewMode::Babel = app_state.view_mode {
                                let target = if app_state.babel_focus == 0 {
                                    &mut app_state.babel_pattern
                                } else {
                                    &mut app_state.babel_input
                                };
                                target.pop();
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                if app_state.crispr_focus == 1 {
                                    app_state.crispr_guide.pop();
                                } else if app_state.crispr_focus == 2 {
                                    app_state.crispr_replace.pop();
                                }
                            } else {
                                app_state.input_buffer.pop();
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Normal Mode
                #[cfg(feature = "nova")]
                if let KeyCode::Char(c) = key.code {
                    if app_state.view_mode == ViewMode::Orca
                        || app_state.view_mode == ViewMode::Prologue
                    {
                        if c == ' ' {
                            // Let Space fall through
                        } else if c.is_ascii_graphic() {
                            let (x, y) = app_state.grid_cursor;
                            vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            continue;
                        }
                    }

                    if c != 'q'
                        && c != ' '
                        && c != 'm'
                        && c != 'c'
                        && c != 'C'
                        && c != 'K'
                        && vm.handle_input(c)
                    {
                        continue;
                    }
                }

                match key.code {
                    KeyCode::Char('K') => {
                        #[cfg(feature = "nova")]
                        {
                            if let ViewMode::Ecology = app_state.view_mode {
                                vm.organelles.clear();
                                app_state.status_msg = "Extinction Event.".to_string();
                            } else {
                                app_state.view_mode = ViewMode::Choir;
                                app_state.status_msg = "Switched to Choir View".to_string();
                            }
                        }
                    }
                    KeyCode::Char('C') => {
                        app_state.chaos_mode = !app_state.chaos_mode;
                        app_state.status_msg = format!("Chaos Mode: {}", app_state.chaos_mode);
                    }
                    KeyCode::Tab => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            use crate::vm::evolution::Challenge;
                            app_state.evolution_state.challenge =
                                match app_state.evolution_state.challenge {
                                    Challenge::Target(_) => Challenge::Doubler,
                                    Challenge::Doubler => Challenge::Adder,
                                    Challenge::Adder => Challenge::Fibonacci,
                                    Challenge::Fibonacci => Challenge::Target(42),
                                    Challenge::Custom(_) => Challenge::Target(42), // Fallback/Cycle
                                };
                            if let Some(engine) = &mut app_state.evolution_state.engine {
                                engine.challenge = app_state.evolution_state.challenge.clone();
                            }
                            app_state.status_msg =
                                format!("Challenge set to {}", app_state.evolution_state.challenge);
                            return Ok(());
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Genesis = app_state.view_mode {
                            app_state.genesis_focus = (app_state.genesis_focus + 1) % 3;
                            return Ok(());
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Forge = app_state.view_mode {
                            app_state.forge_focus = (app_state.forge_focus + 1) % 3;
                            return Ok(());
                        }

                        app_state.view_mode = match app_state.view_mode {
                            ViewMode::Genome => ViewMode::Grid,
                            ViewMode::Grid => ViewMode::Microscope,
                            ViewMode::Microscope => {
                                #[cfg(feature = "biophysics")]
                                {
                                    ViewMode::Cortex
                                }
                                #[cfg(not(feature = "biophysics"))]
                                {
                                    #[cfg(feature = "resonance")]
                                    {
                                        ViewMode::Resonance
                                    }
                                    #[cfg(not(feature = "resonance"))]
                                    {
                                        #[cfg(feature = "nova")]
                                        {
                                            ViewMode::Grimoire
                                        }
                                        #[cfg(not(feature = "nova"))]
                                        {
                                            ViewMode::Heatmap
                                        }
                                    }
                                }
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                #[cfg(feature = "resonance")]
                                {
                                    ViewMode::Resonance
                                }
                                #[cfg(not(feature = "resonance"))]
                                {
                                    #[cfg(feature = "nova")]
                                    {
                                        ViewMode::Grimoire
                                    }
                                    #[cfg(not(feature = "nova"))]
                                    {
                                        ViewMode::Heatmap
                                    }
                                }
                            }
                            #[cfg(feature = "resonance")]
                            ViewMode::Resonance => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Grimoire
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Heatmap
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Grimoire => ViewMode::Topology,
                            #[cfg(feature = "nova")]
                            ViewMode::Topology => ViewMode::Graveyard,
                            #[cfg(feature = "nova")]
                            ViewMode::Graveyard => ViewMode::PianoRoll,
                            #[cfg(feature = "nova")]
                            ViewMode::PianoRoll => ViewMode::Retina,
                            #[cfg(feature = "nova")]
                            ViewMode::Retina => ViewMode::Quantum,
                            #[cfg(feature = "nova")]
                            ViewMode::Quantum => ViewMode::Dream,
                            #[cfg(feature = "nova")]
                            ViewMode::Dream => ViewMode::Phylogeny,
                            #[cfg(feature = "nova")]
                            ViewMode::Phylogeny => ViewMode::Alchemy,
                            #[cfg(feature = "nova")]
                            ViewMode::Alchemy => ViewMode::Memetics,
                            #[cfg(feature = "nova")]
                            ViewMode::Memetics => ViewMode::Egregore,
                            #[cfg(feature = "nova")]
                            ViewMode::Egregore => ViewMode::Bestiary,
                            #[cfg(feature = "nova")]
                            ViewMode::Bestiary => ViewMode::Kaleidoscope,
                            #[cfg(feature = "nova")]
                            ViewMode::Kaleidoscope => ViewMode::Void,
                            #[cfg(feature = "nova")]
                            ViewMode::Void => ViewMode::Signals,
                            #[cfg(feature = "nova")]
                            ViewMode::Signals => ViewMode::Sovereignty,
                            #[cfg(feature = "nova")]
                            ViewMode::Sovereignty => ViewMode::Spectrogram,
                            #[cfg(feature = "nova")]
                            ViewMode::Spectrogram => ViewMode::Market,
                            #[cfg(feature = "nova")]
                            ViewMode::Market => ViewMode::Ballistics,
                            #[cfg(feature = "nova")]
                            ViewMode::Ballistics => ViewMode::Scent,
                            #[cfg(feature = "nova")]
                            ViewMode::Scent => ViewMode::Fishing,
                            #[cfg(feature = "nova")]
                            ViewMode::Fishing => ViewMode::Arena,
                            #[cfg(feature = "nova")]
                            ViewMode::Arena => ViewMode::Garden,
                            #[cfg(feature = "nova")]
                            ViewMode::Garden => ViewMode::Orca,
                            #[cfg(feature = "nova")]
                            ViewMode::Orca => ViewMode::Babel,
                            #[cfg(feature = "nova")]
                            ViewMode::Babel => ViewMode::Strings,
                            #[cfg(feature = "nova")]
                            ViewMode::Strings => ViewMode::Quipu,
                            #[cfg(feature = "nova")]
                            ViewMode::Quipu => ViewMode::Hydra,
                            #[cfg(feature = "nova")]
                            ViewMode::Hydra => ViewMode::Chronos,
                            #[cfg(feature = "nova")]
                            ViewMode::Chronos => ViewMode::Logos,
                            #[cfg(feature = "nova")]
                            ViewMode::Logos => ViewMode::Pandemonium,
                            #[cfg(feature = "nova")]
                            ViewMode::Pandemonium => ViewMode::BioticChaos,
                            ViewMode::BioticChaos => ViewMode::Catalyst,
                            ViewMode::Catalyst => ViewMode::Heatmap,
                            ViewMode::Heatmap => {
                                #[cfg(feature = "silicon")]
                                {
                                    ViewMode::Schematic
                                }
                                #[cfg(not(feature = "silicon"))]
                                {
                                    #[cfg(feature = "elektra")]
                                    {
                                        ViewMode::Elektra
                                    }
                                    #[cfg(not(feature = "elektra"))]
                                    {
                                        #[cfg(feature = "nova")]
                                        {
                                            ViewMode::Laboratory
                                        }
                                        #[cfg(not(feature = "nova"))]
                                        {
                                            ViewMode::Genome
                                        }
                                    }
                                }
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => ViewMode::Foundry,
                            #[cfg(feature = "silicon")]
                            ViewMode::Foundry => {
                                #[cfg(feature = "elektra")]
                                {
                                    ViewMode::Elektra
                                }
                                #[cfg(not(feature = "elektra"))]
                                {
                                    #[cfg(feature = "nova")]
                                    {
                                        ViewMode::Laboratory
                                    }
                                    #[cfg(not(feature = "nova"))]
                                    {
                                        ViewMode::Genome
                                    }
                                }
                            }
                            #[cfg(feature = "elektra")]
                            ViewMode::Elektra => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Laboratory
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Laboratory => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Hyperspace => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Hologram => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Weaver => ViewMode::Terminal,
                            #[cfg(feature = "nova")]
                            ViewMode::Terminal => ViewMode::Attractor,
                            #[cfg(feature = "nova")]
                            ViewMode::Attractor => ViewMode::Virology,
                            #[cfg(feature = "nova")]
                            ViewMode::Virology => ViewMode::BioMesh,
                            #[cfg(feature = "nova")]
                            ViewMode::BioMesh => ViewMode::Crispr,
                            #[cfg(feature = "nova")]
                            ViewMode::Crispr => ViewMode::Reactor,
                            #[cfg(feature = "nova")]
                            ViewMode::Reactor => ViewMode::Biolum,
                            #[cfg(feature = "nova")]
                            ViewMode::Biolum => ViewMode::Evolution,
                            ViewMode::Evolution => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Ecology
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Ecology => ViewMode::LifeCycle,
                            #[cfg(feature = "nova")]
                            ViewMode::Semiotics => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Fractal => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::LifeCycle => ViewMode::Metazoa,
                            #[cfg(feature = "nova")]
                            ViewMode::Metazoa => ViewMode::Genesis,
                            #[cfg(feature = "nova")]
                            ViewMode::Genesis => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Cambrian => ViewMode::Savant,
                            #[cfg(feature = "nova")]
                            ViewMode::Savant => ViewMode::Akashic,
                            #[cfg(feature = "nova")]
                            ViewMode::Akashic => ViewMode::Prologue,
                            #[cfg(feature = "nova")]
                            ViewMode::Prologue => ViewMode::Lexicon,
                            #[cfg(feature = "nova")]
                            ViewMode::Lexicon => ViewMode::Narrative,
                            #[cfg(feature = "nova")]
                            ViewMode::Narrative => ViewMode::Sequencer,
                            ViewMode::Sequencer => ViewMode::Mutagen,
                            #[cfg(feature = "nova")]
                            ViewMode::Mutagen => ViewMode::Forge,
                            ViewMode::Forge => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Tesseract
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Tesseract => ViewMode::Genome,
                            #[cfg(not(feature = "nova"))]
                            ViewMode::Tesseract => ViewMode::Choir,
                            #[cfg(feature = "nova")]
                            ViewMode::Choir => ViewMode::Paradox,
                            #[cfg(feature = "nova")]
                            ViewMode::Paradox => ViewMode::Codex,
                            #[cfg(feature = "nova")]
                            ViewMode::Codex => ViewMode::Verbum,
                            ViewMode::Verbum => ViewMode::Genome,
                        };
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('^') => app_state.view_mode = ViewMode::Cambrian,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('&') => app_state.view_mode = ViewMode::Semiotics,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('*') => app_state.view_mode = ViewMode::Fractal,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('y') => app_state.view_mode = ViewMode::LifeCycle,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('|') => app_state.view_mode = ViewMode::Savant,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('#') => app_state.view_mode = ViewMode::Akashic,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('\\') => app_state.view_mode = ViewMode::Prologue,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('6') => app_state.view_mode = ViewMode::Lexicon,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('N') => app_state.view_mode = ViewMode::Narrative,
                    KeyCode::Char('h') => app_state.view_mode = ViewMode::Heatmap,
                    #[cfg(feature = "silicon")]
                    KeyCode::Char('F') => app_state.view_mode = ViewMode::Foundry,
                    #[cfg(feature = "elektra")]
                    KeyCode::Char('E') => app_state.view_mode = ViewMode::Elektra,
                    KeyCode::Char('p') => {
                        if let ViewMode::Grid = app_state.view_mode {
                            app_state.palette_open = !app_state.palette_open;
                        } else {
                            #[cfg(feature = "nova")]
                            {
                                app_state.view_mode = ViewMode::PianoRoll;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('5') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 4;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('z') => app_state.view_mode = ViewMode::Bestiary,
                    KeyCode::Char('i') => {
                        app_state.input_mode = InputMode::Injection;
                        app_state.input_buffer.clear();
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('r') => {
                        if let ViewMode::Graveyard = app_state.view_mode {
                            match vm.resurrect_from_graveyard(app_state.selected_graveyard_strand) {
                                Ok(idx) => {
                                    app_state.status_msg = format!("Resurrected strand {}!", idx);
                                    if app_state.selected_graveyard_strand >= vm.graveyard.len()
                                        && !vm.graveyard.is_empty()
                                    {
                                        app_state.selected_graveyard_strand =
                                            vm.graveyard.len() - 1;
                                    }
                                }
                                Err(e) => app_state.status_msg = format!("Error: {}", e),
                            }
                        }
                    }
                    #[cfg(feature = "biophysics")]
                    KeyCode::Char('b') => app_state.view_mode = ViewMode::Cortex,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('a') => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            app_state.evolution_state.auto_run =
                                !app_state.evolution_state.auto_run;
                        } else if let ViewMode::Alchemy = app_state.view_mode {
                            // Add to Crucible
                            match app_state.alchemy_selection {
                                0 => {
                                    // Shelf
                                    let elements = [
                                        "Fire", "Water", "Earth", "Air", "Life", "Death", "Lead",
                                        "Energy",
                                    ];
                                    if app_state.alchemy_shelf_idx < elements.len() {
                                        vm.crucible.add(crate::vm::Value::Str(
                                            elements[app_state.alchemy_shelf_idx].to_string(),
                                        ));
                                    }
                                }
                                1 => {
                                    // Strands
                                    if app_state.alchemy_strand_idx < vm.dna.helix.strands.len() {
                                        vm.crucible.add(crate::vm::Value::Int(
                                            app_state.alchemy_strand_idx as i64,
                                        ));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('x') => {
                        if let ViewMode::Graveyard = app_state.view_mode {
                            if app_state.selected_graveyard_strand < vm.graveyard.len() {
                                vm.graveyard.remove(app_state.selected_graveyard_strand);
                                app_state.status_msg = "Exterminated strand.".to_string();
                                if app_state.selected_graveyard_strand >= vm.graveyard.len()
                                    && !vm.graveyard.is_empty()
                                {
                                    app_state.selected_graveyard_strand = vm.graveyard.len() - 1;
                                }
                            }
                        } else if let ViewMode::Alchemy = app_state.view_mode {
                            vm.crucible.clear();
                            app_state.status_msg = "Crucible emptied.".to_string();
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('t') => {
                        if let ViewMode::Alchemy = app_state.view_mode {
                            crate::vm::alchemy::transmute_crucible(vm);
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('k') => app_state.view_mode = ViewMode::Kaleidoscope,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('$') => app_state.view_mode = ViewMode::Market,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('!') => app_state.view_mode = ViewMode::Ballistics,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('~') => app_state.view_mode = ViewMode::Scent,
                    KeyCode::Char('e') => {
                        if let ViewMode::Genome = app_state.view_mode {
                            if let Some(strand) =
                                vm.dna.helix.strands.get(app_state.selected_strand)
                            {
                                let engine = crate::vm::evolution::EvolutionEngine::new(
                                    strand.clone(),
                                    20, // Population
                                    app_state.evolution_state.challenge.clone(),
                                );
                                app_state.evolution_state.engine = Some(engine);
                                app_state.view_mode = ViewMode::Evolution;
                                app_state.status_msg = "Evolution Initialized".to_string();
                            }
                        }
                    }
                    KeyCode::Char('f') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Ecology = app_state.view_mode {
                            crate::vm::nova_ecology::spawn_food(vm);
                            app_state.status_msg = "Food spawned.".to_string();
                            continue;
                        }

                        #[cfg(feature = "silicon")]
                        if let ViewMode::Foundry = app_state.view_mode {
                            // Fabricate current strand
                            let (x, y) = app_state.grid_cursor;
                            let s_idx = app_state.selected_strand;
                            vm.stack.push(crate::vm::Value::Int(s_idx as i64));
                            vm.stack.push(crate::vm::Value::Int(y as i64));
                            vm.stack.push(crate::vm::Value::Int(x as i64));
                            crate::vm::silicon::exec_silicon_op(
                                vm,
                                crate::opcode::OpCode::Fabricate,
                                &[],
                            );
                            app_state.status_msg =
                                format!("Fabricated strand {} at {},{}", s_idx, x, y);
                            continue;
                        }

                        #[cfg(feature = "nova")]
                        {
                            app_state.view_mode = ViewMode::Fishing;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('V') => app_state.view_mode = ViewMode::Arena,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('G') => {
                        if let ViewMode::Babel = app_state.view_mode {
                            if let Some(ast) = &app_state.babel_ast {
                                let s = crate::vm::babel::generate_string(ast);
                                app_state.babel_result = s;
                            } else {
                                app_state.status_msg = "No Grammar to Generate from".to_string();
                            }
                        } else {
                            app_state.view_mode = ViewMode::Garden;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('l') => app_state.view_mode = ViewMode::Biolum,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('L') => app_state.view_mode = ViewMode::Babel,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('=') => app_state.view_mode = ViewMode::Strings,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('Y') => app_state.view_mode = ViewMode::Hydra,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('T') => {
                        if let ViewMode::Babel = app_state.view_mode {
                            if let Some(ast) = &app_state.babel_ast {
                                vm.stack.push(ast.clone());
                                vm.stack
                                    .push(crate::vm::Value::Str(app_state.babel_input.clone()));
                                crate::vm::babel::exec_babel_op(
                                    vm,
                                    crate::opcode::OpCode::Tongue,
                                    &[],
                                );
                                if let Some(res) = vm.stack.pop() {
                                    app_state.babel_result = format!("{}", res);
                                }
                            }
                        } else {
                            app_state.view_mode = ViewMode::Chronos;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('U') => app_state.view_mode = ViewMode::Logos,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('P') => app_state.view_mode = ViewMode::Pandemonium,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('H') => app_state.view_mode = ViewMode::Hyperspace,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('W') => app_state.view_mode = ViewMode::Weaver,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('`') => app_state.view_mode = ViewMode::Terminal,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('A') => app_state.view_mode = ViewMode::Attractor,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('v') => app_state.view_mode = ViewMode::Virology,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('B') => app_state.view_mode = ViewMode::BioMesh,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('X') => app_state.view_mode = ViewMode::Reactor,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('I') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            // Interfere (DNA -> Hologram)
                            let idx = app_state.selected_strand;
                            vm.stack.push(crate::vm::Value::Int(idx as i64));
                            crate::vm::nova_hologram::exec_interfere(
                                vm,
                                crate::opcode::OpCode::Interfere,
                                &[],
                            );
                            app_state.status_msg = format!("Interfered strand {}", idx);
                        } else if let ViewMode::Ecology = app_state.view_mode {
                            app_state.input_mode = InputMode::Editing;
                            app_state.input_buffer.clear();
                            app_state.status_msg = "Injecting Gene... (Type & Enter)".to_string();
                        } else {
                            app_state.view_mode = ViewMode::Hologram;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('O') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            // Refract (Hologram -> DNA)
                            crate::vm::nova_hologram::exec_refract(
                                vm,
                                crate::opcode::OpCode::Refract,
                                &[],
                            );
                        } else {
                            app_state.view_mode = ViewMode::Orca;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('+') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            let (x, y) = app_state.grid_cursor;
                            vm.hologram_grid[y][x].0 += 0.1;
                            vm.hologram_grid[y][x].1 += 0.1;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('-') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            let (x, y) = app_state.grid_cursor;
                            vm.hologram_grid[y][x].0 -= 0.1;
                            vm.hologram_grid[y][x].1 -= 0.1;
                        }
                    }
                    #[cfg(all(feature = "oracle", feature = "nova"))]
                    KeyCode::Char('/') => {
                        if let ViewMode::Grimoire = app_state.view_mode {
                            app_state.query_mode = true;
                            app_state.query_input.clear();
                            app_state.query_results.clear();
                        }
                    }
                    KeyCode::Char('?') => {
                        app_state.show_view_selector = !app_state.show_view_selector;
                        // Reset index when opening
                        if app_state.show_view_selector {
                            app_state.view_selector_state.borrow_mut().select(Some(0));
                        }
                    }
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            if let Some(engine) = &mut app_state.evolution_state.engine {
                                engine.step(vm);
                            }
                            continue;
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Babel = app_state.view_mode {
                            // Run Parse
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.babel_pattern.clone()));
                            let _ = crate::vm::babel::exec_babel_op(
                                vm,
                                crate::opcode::OpCode::ParserRegex,
                                &[],
                            );
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.babel_input.clone()));
                            let _ = crate::vm::babel::exec_babel_op(
                                vm,
                                crate::opcode::OpCode::Parse,
                                &[],
                            );

                            if let Some(res) = vm.stack.pop() {
                                app_state.babel_result = format!("{}", res);
                            } else {
                                app_state.babel_result = "Stack Empty/Error".to_string();
                            }
                            continue;
                        }

                        if let ViewMode::Grid = app_state.view_mode {
                            if let Some(c) = app_state.palette_char {
                                let (x, y) = app_state.grid_cursor;
                                vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            } else {
                                vm.step();
                            }
                        } else {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Arena = app_state.view_mode {
                                if let Some(arena) = &mut vm.arena {
                                    arena.tick();
                                }
                            } else if let ViewMode::Pandemonium = app_state.view_mode {
                                let cx = app_state.pandemonium_cursor.0;
                                let cy = app_state.pandemonium_cursor.1;

                                let mut best_dist = 1.0;
                                let mut target = None;

                                let mut linear_idx = 0;
                                for (s_idx, strand) in vm.dna.helix.strands.iter().enumerate() {
                                    for (g_idx, _) in strand.genes.iter().enumerate() {
                                        let t = (linear_idx as f64) * 0.1;
                                        let gr = t * 0.5;
                                        let gx = gr * t.cos();
                                        let gy = gr * t.sin();

                                        let dist = ((gx - cx).powi(2) + (gy - cy).powi(2)).sqrt();
                                        if dist < best_dist {
                                            best_dist = dist;
                                            target = Some((s_idx, g_idx));
                                        }
                                        linear_idx += 1;
                                    }
                                }

                                if let Some((s, g)) = target {
                                    match app_state.pandemonium_selected_tool {
                                        0 => crate::vm::pandemonium::apply_mutation(vm, s, g),
                                        1 => crate::vm::pandemonium::apply_scramble(
                                            vm,
                                            s,
                                            g,
                                            app_state.pandemonium_radius,
                                        ),
                                        2 => crate::vm::pandemonium::apply_purge(
                                            vm,
                                            s,
                                            g,
                                            app_state.pandemonium_radius,
                                        ),
                                        3 => crate::vm::pandemonium::apply_duplicate(vm, s, g),
                                        4 => crate::vm::pandemonium::apply_storm(
                                            vm,
                                            s,
                                            g,
                                            app_state.pandemonium_radius,
                                        ),
                                        _ => {}
                                    }
                                    app_state.status_msg =
                                        format!("Pandemonium applied at {},{}", s, g);
                                }
                            } else if let ViewMode::Fishing = app_state.view_mode {
                                if app_state.fishing_cast {
                                    // Reel
                                    if app_state.fishing_hooked {
                                        app_state.fishing_bobber_y += 4.0;
                                        app_state.fishing_tension += 0.05; // Reeling increases tension

                                        if app_state.fishing_bobber_y > 90.0 {
                                            // Caught!
                                            app_state.status_msg = "CAUGHT A FISH!".to_string();
                                            app_state.fishing_cast = false;
                                            app_state.fishing_hooked = false;
                                            app_state.fishing_tension = 0.0;
                                            // Maybe give energy?
                                            vm.energy += 10;
                                        }
                                    } else {
                                        // Just pull empty line
                                        app_state.fishing_cast = false;
                                        app_state.status_msg = "Reeled in empty.".to_string();
                                    }
                                } else {
                                    // Cast
                                    app_state.fishing_cast = true;
                                    app_state.fishing_bobber_y = 50.0;
                                    app_state.fishing_tension = 0.0;
                                    app_state.status_msg = "Casted line...".to_string();
                                }
                            } else if let ViewMode::Kaleidoscope = app_state.view_mode {
                                // Paint
                                let (x, y) = app_state.grid_cursor;
                                let r = match app_state.kaleidoscope_hue_idx {
                                    0 => 255,
                                    1 => 255,
                                    2 => 0,
                                    3 => 0,
                                    4 => 0,
                                    5 => 255,
                                    _ => 255,
                                };
                                let g = match app_state.kaleidoscope_hue_idx {
                                    0 => 0,
                                    1 => 255,
                                    2 => 255,
                                    3 => 255,
                                    4 => 0,
                                    5 => 0,
                                    _ => 255,
                                };
                                let b = match app_state.kaleidoscope_hue_idx {
                                    0 => 0,
                                    1 => 0,
                                    2 => 0,
                                    3 => 255,
                                    4 => 255,
                                    5 => 255,
                                    _ => 255,
                                };

                                // Adjust for lightness (Light=0, Normal=1, Dark=2)
                                let (r, g, b) = match app_state.kaleidoscope_light_idx {
                                    0 => (r + (255 - r) / 2, g + (255 - g) / 2, b + (255 - b) / 2), // Light
                                    2 => (r / 2, g / 2, b / 2), // Dark
                                    _ => (r, g, b),             // Normal
                                };

                                vm.chroma_grid[y][x].fg = Some((r as u8, g as u8, b as u8));
                            } else {
                                vm.step();
                            }
                            #[cfg(not(feature = "nova"))]
                            vm.step();
                        }
                    }
                    KeyCode::Char('s') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Ecology = app_state.view_mode {
                            crate::vm::nova_ecology::spawn_random_ecology(vm, 10);
                            app_state.status_msg = "Spawned 10 organisms.".to_string();
                            continue;
                        } else if let ViewMode::Kaleidoscope = app_state.view_mode {
                            // Step Piet
                            if vm.piet_state.is_none() {
                                vm.piet_state = Some(crate::vm::piet::init_piet(vm));
                            }
                            if let Some(mut state) = vm.piet_state.take() {
                                crate::vm::piet::step_piet_once(vm, &mut state);
                                vm.piet_state = Some(state);
                            }
                            continue;
                        }

                        #[cfg(feature = "silicon")]
                        {
                            app_state.view_mode = ViewMode::Schematic;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('R') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            vm.piet_state = None;
                            app_state.status_msg = "Piet State Reset".to_string();
                        } else if let ViewMode::Arena = app_state.view_mode {
                            if let Some(arena) = &mut vm.arena {
                                arena.reset();
                                app_state.status_msg = "Arena Reset".to_string();
                            }
                        } else if let ViewMode::Babel = app_state.view_mode {
                            // Seed
                            app_state.babel_ast = Some(crate::vm::Value::Junction(
                                crate::ast::JunctionType::Any,
                                vec![
                                    crate::vm::Value::Str("Seq".to_string()),
                                    crate::vm::Value::Junction(
                                        crate::ast::JunctionType::Any,
                                        vec![
                                            crate::vm::Value::Str("Match".to_string()),
                                            crate::vm::Value::Str("Hello".to_string()),
                                        ],
                                    ),
                                    crate::vm::Value::Junction(
                                        crate::ast::JunctionType::Any,
                                        vec![
                                            crate::vm::Value::Str("Match".to_string()),
                                            crate::vm::Value::Str("World".to_string()),
                                        ],
                                    ),
                                ],
                            ));
                            app_state.status_msg = "Grammar Reset".to_string();
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('M') => {
                        if let ViewMode::Babel = app_state.view_mode {
                            // Initialize if needed
                            if app_state.babel_ast.is_none() {
                                // Seed: Seq(Match("A"), Match("B"))
                                app_state.babel_ast = Some(crate::vm::Value::Junction(
                                    crate::ast::JunctionType::Any,
                                    vec![
                                        crate::vm::Value::Str("Seq".to_string()),
                                        crate::vm::Value::Junction(
                                            crate::ast::JunctionType::Any,
                                            vec![
                                                crate::vm::Value::Str("Match".to_string()),
                                                crate::vm::Value::Str("A".to_string()),
                                            ],
                                        ),
                                        crate::vm::Value::Junction(
                                            crate::ast::JunctionType::Any,
                                            vec![
                                                crate::vm::Value::Str("Match".to_string()),
                                                crate::vm::Value::Str("B".to_string()),
                                            ],
                                        ),
                                    ],
                                ));
                            }

                            if let Some(ast) = &app_state.babel_ast {
                                let new_ast = crate::vm::babel::mutate_grammar(ast, 0.2); // 20% rate
                                app_state.babel_ast = Some(new_ast);
                                app_state.status_msg = "Grammar Mutated".to_string();
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('S') => {
                        if let ViewMode::Arena = app_state.view_mode {
                            if let Some(arena) = &mut vm.arena {
                                // Add random gladiators if empty
                                if arena.combatants.is_empty() {
                                    // Use some existing strands or random
                                    let mut rng = rand::thread_rng();
                                    use rand::Rng;
                                    if !vm.dna.helix.strands.is_empty() {
                                        let s1 = vm.dna.helix.strands
                                            [rng.gen_range(0..vm.dna.helix.strands.len())]
                                        .clone();
                                        let s2 = vm.dna.helix.strands
                                            [rng.gen_range(0..vm.dna.helix.strands.len())]
                                        .clone();
                                        arena.add_gladiator(s1, rng.gen());
                                        arena.add_gladiator(s2, rng.gen());
                                    }
                                }
                                arena.start();
                                app_state.status_msg = "Arena Started!".to_string();
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('1') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 0;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('2') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 1;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('3') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 2;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('4') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 3;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('[') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            if app_state.kaleidoscope_hue_idx > 0 {
                                app_state.kaleidoscope_hue_idx -= 1;
                            } else {
                                app_state.kaleidoscope_hue_idx = 5;
                            }
                        } else if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_radius =
                                (app_state.pandemonium_radius - 1.0).max(1.0);
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char(']') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            app_state.kaleidoscope_hue_idx =
                                (app_state.kaleidoscope_hue_idx + 1) % 6;
                        } else if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_radius += 1.0;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('{') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            if app_state.kaleidoscope_light_idx > 0 {
                                app_state.kaleidoscope_light_idx -= 1;
                            } else {
                                app_state.kaleidoscope_light_idx = 2;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('}') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            app_state.kaleidoscope_light_idx =
                                (app_state.kaleidoscope_light_idx + 1) % 3;
                        }
                    }
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
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Genesis => {
                            if app_state.genesis_focus == 2 && app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Metazoa => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        ViewMode::Catalyst => {
                            if !vm.catalysts.is_empty()
                                && app_state.catalyst_scroll + 1 < vm.catalysts.len()
                            {
                                app_state.catalyst_scroll += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.1 += 1.0;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {
                            app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {
                            if app_state.selected_graveyard_strand + 1 < vm.graveyard.len() {
                                app_state.selected_graveyard_strand += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {
                            if app_state.selected_dream_trace + 1 < vm.dream_traces.len() {
                                app_state.selected_dream_trace += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            if app_state.alchemy_selection == 0 {
                                if app_state.alchemy_shelf_idx < 7 {
                                    // 8 items
                                    app_state.alchemy_shelf_idx += 1;
                                }
                            } else if app_state.alchemy_strand_idx + 1 < vm.dna.helix.strands.len()
                            {
                                app_state.alchemy_strand_idx += 1;
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
                            match app_state.selected_strand {
                                // 0=A, 1=B, 2=Method/Pattern
                                0 => {
                                    if app_state.lab_parent_a > 0 {
                                        app_state.lab_parent_a -= 1;
                                    }
                                }
                                1 => {
                                    if app_state.lab_parent_b > 0 {
                                        app_state.lab_parent_b -= 1;
                                    }
                                }
                                2 => {
                                    if let ViewMode::Laboratory = app_state.view_mode {
                                        if app_state.lab_method > 0 {
                                            app_state.lab_method -= 1;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {
                            if app_state.selected_sigil_index + 1 < vm.sigil_registry.len() {
                                app_state.selected_sigil_index += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {
                            if !vm.organelles.is_empty()
                                && app_state.selected_organelle_index + 1 < vm.organelles.len()
                            {
                                app_state.selected_organelle_index += 1;
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
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        _ => {}
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
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.1 -= 1.0;
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {
                            if app_state.selected_sigil_index > 0 {
                                app_state.selected_sigil_index -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
                            let max_strand = vm.dna.helix.strands.len().saturating_sub(1);
                            match app_state.selected_strand {
                                // 0=A, 1=B, 2=Method
                                0 => {
                                    if app_state.lab_parent_a < max_strand {
                                        app_state.lab_parent_a += 1;
                                    }
                                }
                                1 => {
                                    if app_state.lab_parent_b < max_strand {
                                        app_state.lab_parent_b += 1;
                                    }
                                }
                                2 => {
                                    if let ViewMode::Laboratory = app_state.view_mode {
                                        if app_state.lab_method < 3 {
                                            app_state.lab_method += 1;
                                        }
                                    }
                                }
                                _ => {}
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
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {
                            if app_state.selected_graveyard_strand > 0 {
                                app_state.selected_graveyard_strand -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {
                            if app_state.selected_dream_trace > 0 {
                                app_state.selected_dream_trace -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            if app_state.alchemy_selection == 0 {
                                if app_state.alchemy_shelf_idx > 0 {
                                    app_state.alchemy_shelf_idx -= 1;
                                }
                            } else if app_state.alchemy_strand_idx > 0 {
                                app_state.alchemy_strand_idx -= 1;
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {
                            if app_state.selected_organelle_index > 0 {
                                app_state.selected_organelle_index -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {
                            if app_state.babel_focus > 0 {
                                app_state.babel_focus -= 1;
                            } else {
                                app_state.babel_focus = 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::Catalyst => {
                            if app_state.catalyst_scroll > 0 {
                                app_state.catalyst_scroll -= 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Right => match app_state.view_mode {
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.0 += 1.0;
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {}
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Catalyst => {}
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
                            if app_state.selected_strand < 2 {
                                app_state.selected_strand += 1;
                            } else {
                                app_state.selected_strand = 0;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            app_state.alchemy_selection = 1;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {
                            if vm.quipu.active_cord + 1 < vm.quipu.cords.len() {
                                vm.quipu.active_cord += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Left => match app_state.view_mode {
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.0 -= 1.0;
                        }
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Catalyst => {}
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {}
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
                            if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                            } else {
                                app_state.selected_strand = 2;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            app_state.alchemy_selection = 0;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {
                            if vm.quipu.active_cord > 0 {
                                vm.quipu.active_cord -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Enter => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Reactor = app_state.view_mode {
                            app_state.input_mode = InputMode::Editing;
                            app_state.input_buffer.clear();
                        }

                        if let ViewMode::Evolution = app_state.view_mode {
                            app_state.input_mode = InputMode::Editing;
                            app_state.input_buffer.clear();
                        }

                        #[cfg(feature = "silicon")]
                        if let ViewMode::Foundry = app_state.view_mode {
                            // Trace current circuit
                            let (x, y) = app_state.grid_cursor;
                            vm.stack.push(crate::vm::Value::Int(y as i64));
                            vm.stack.push(crate::vm::Value::Int(x as i64));
                            crate::vm::silicon::exec_silicon_op(
                                vm,
                                crate::opcode::OpCode::Trace,
                                &[],
                            );
                            if let Some(crate::vm::Value::Int(idx)) = vm.stack.last() {
                                app_state.status_msg = format!("Traced circuit to strand {}", idx);
                                app_state.selected_strand = *idx as usize;
                            }
                            continue;
                        }

                        app_state.input_mode = InputMode::Editing;
                        match app_state.view_mode {
                            #[cfg(feature = "nova")]
                            ViewMode::Phylogeny => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Alchemy => {
                                // Prevent entering edit mode for Alchemy (uses keys instead)
                                app_state.input_mode = InputMode::Normal;
                            }
                            ViewMode::Catalyst => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Biolum => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Reactor => {}
                            #[cfg(feature = "nova")]
                            ViewMode::BioMesh => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Crispr => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Hyperspace => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Hologram => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Weaver => {
                                // Allow editing mode for Pattern entry
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Terminal => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Attractor => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Virology => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            ViewMode::Evolution => {}
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
                                                        crate::ast::JunctionType::Dish => "dish",
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
                            ViewMode::BioticChaos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = vm.chaos_struct.grid[y][x];
                                app_state.input_buffer = format!("{:.4}", val);
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
                            #[cfg(feature = "nova")]
                            ViewMode::Chronos => {
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
                            #[cfg(feature = "nova")]
                            ViewMode::Logos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                app_state.input_buffer = format!("{}", val);
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Pandemonium => {
                                app_state.input_mode = InputMode::Normal; // No editing mode for now
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Foundry => {
                                // Prepare input buffer for editing
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
                            #[cfg(feature = "nova")]
                            ViewMode::Orca => {
                                // Enable editing grid from Orca view
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
                            ViewMode::Microscope => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                // Prevent entering edit mode for Cortex
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "resonance")]
                            ViewMode::Resonance => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Grimoire => {
                                app_state.input_mode = InputMode::Normal;
                                let mut registry: Vec<_> =
                                    vm.sigil_registry.keys().cloned().collect();
                                registry.sort();
                                if app_state.selected_sigil_index < registry.len() {
                                    let key = &registry[app_state.selected_sigil_index];
                                    if let Some(sigil) = vm.sigil_registry.get_mut(key) {
                                        sigil.auto_cast = !sigil.auto_cast;
                                        let status = if sigil.auto_cast {
                                            "ENABLED"
                                        } else {
                                            "DISABLED"
                                        };
                                        app_state.status_msg =
                                            format!("{} Auto-Cast: {}", key, status);
                                    }
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Laboratory => {
                                app_state.input_mode = InputMode::Normal;
                                let splice_op = crate::opcode::OpCode::Splice;
                                let args = vec![
                                    crate::ast::Nucleotide::Number(app_state.lab_parent_a as i64),
                                    crate::ast::Nucleotide::Number(app_state.lab_parent_b as i64),
                                    crate::ast::Nucleotide::Number(app_state.lab_method as i64),
                                ];
                                vm.execute_gene_inner(splice_op, &args);
                                app_state.status_msg = format!(
                                    "Spliced {} & {} (Method {})",
                                    app_state.lab_parent_a,
                                    app_state.lab_parent_b,
                                    app_state.lab_method
                                );
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Topology => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Graveyard => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::PianoRoll => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Retina => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Quantum => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Dream => {
                                app_state.input_mode = InputMode::Normal;
                                if app_state.selected_dream_trace < vm.dream_traces.len() {
                                    let (target_idx, mutated_strand) = {
                                        let trace =
                                            &vm.dream_traces[app_state.selected_dream_trace];
                                        (trace.strand_idx, trace.mutated_strand.clone())
                                    };

                                    if let Some(strand) = mutated_strand {
                                        // Lucid Dreaming: Inject the strand
                                        if target_idx < vm.dna.helix.strands.len() {
                                            vm.dna.helix.strands[target_idx] = strand;
                                            app_state.status_msg = format!(
                                                "LUCID DREAM: Realized mutations for strand {}",
                                                target_idx
                                            );
                                        }
                                    }
                                }
                            }
                            ViewMode::Heatmap => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Memetics => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Egregore => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Bestiary => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Kaleidoscope => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Void => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Signals => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Sovereignty => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Spectrogram => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Market => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Ballistics => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Scent => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Fishing => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Arena => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Garden => {
                                // Enable editing grid from Garden view
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
                            #[cfg(feature = "nova")]
                            ViewMode::Babel => {
                                // No buffer prep needed, editing in place
                            }
                            #[cfg(feature = "elektra")]
                            ViewMode::Elektra => {
                                // Enable editing grid from Elektra view (like Grid view)
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
                            #[cfg(feature = "nova")]
                            ViewMode::Ecology => {
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
                            #[cfg(feature = "nova")]
                            ViewMode::Strings => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Quipu => {
                                // Prepare buffer with current value
                                if vm.quipu.active_cord < vm.quipu.cords.len() {
                                    let val = vm.quipu.cords[vm.quipu.active_cord];
                                    app_state.input_buffer = val.to_string();
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Hydra => {
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
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn apply_glitch_fx(buffer: &mut ratatui::buffer::Buffer, intensity: f32) {
    let area = *buffer.area();
    let mut rng = rand::thread_rng();
    use rand::Rng;

    for y in area.y..area.height {
        for x in area.x..area.width {
            if rng.gen::<f32>() < intensity {
                let cell = &mut buffer[(x, y)];
                match rng.gen_range(0..4) {
                    0 => {
                        let chars = ['@', '#', '$', '%', '&', '!', '?', 'X', '.', ':', ';', '~'];
                        cell.set_char(chars[rng.gen_range(0..chars.len())]);
                    }
                    1 => {
                        std::mem::swap(&mut cell.fg, &mut cell.bg);
                    }
                    2 => {
                        cell.fg = Color::DarkGray;
                    }
                    3 => {
                        cell.set_char('?');
                        cell.set_style(Style::default().fg(Color::Red).bg(Color::Black));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(feature = "nova")]
#[cfg(feature = "oracle")]
#[allow(dead_code)]

pub(crate) fn get_all_views() -> Vec<(ViewMode, &'static str, &'static str)> {
    let mut views = vec![
        (ViewMode::Genome, "Genome", "Tab"),
        (ViewMode::Grid, "Grid", "Tab"),
        (ViewMode::Microscope, "Microscope", "Tab"),
        (ViewMode::Heatmap, "Heatmap", "h"),
    ];

    #[cfg(feature = "biophysics")]
    views.push((ViewMode::Cortex, "Cortex", "b"));

    #[cfg(feature = "resonance")]
    views.push((ViewMode::Resonance, "Resonance", "Tab"));

    #[cfg(feature = "elektra")]
    views.push((ViewMode::Elektra, "Elektra", "E"));

    #[cfg(feature = "silicon")]
    {
        views.push((ViewMode::Schematic, "Schematic", "s"));
        views.push((ViewMode::Foundry, "Foundry", "F"));
    }

    #[cfg(feature = "nova")]
    {
        views.push((ViewMode::Grimoire, "Grimoire", "Tab"));
        views.push((ViewMode::Laboratory, "Laboratory", "Tab"));
        views.push((ViewMode::Topology, "Topology", "Tab"));
        views.push((ViewMode::Graveyard, "Graveyard", "Tab"));
        views.push((ViewMode::PianoRoll, "Piano Roll", "p"));
        views.push((ViewMode::Retina, "Retina", "Tab"));
        views.push((ViewMode::Quantum, "Quantum", "Tab"));
        views.push((ViewMode::Dream, "Dream Catcher", "Tab"));
        views.push((ViewMode::Phylogeny, "Phylogeny", "Tab"));
        views.push((ViewMode::Alchemy, "Alchemy", "Tab"));
        views.push((ViewMode::Memetics, "Memetics", "Tab"));
        views.push((ViewMode::Egregore, "Egregore", "Tab"));
        views.push((ViewMode::Bestiary, "Bestiary", "z"));
        views.push((ViewMode::Kaleidoscope, "Kaleidoscope", "k"));
        views.push((ViewMode::Void, "Void", "Tab"));
        views.push((ViewMode::Signals, "Signals", "Tab"));
        views.push((ViewMode::Sovereignty, "Sovereignty", "Tab"));
        views.push((ViewMode::Spectrogram, "Spectrogram", "Tab"));
        views.push((ViewMode::Market, "Market", "$"));
        views.push((ViewMode::Ballistics, "Ballistics", "!"));
        views.push((ViewMode::Scent, "Scent", "~"));
        views.push((ViewMode::Fishing, "Fishing", "f"));
        views.push((ViewMode::Arena, "Arena", "V"));
        views.push((ViewMode::Garden, "Garden", "G"));
        views.push((ViewMode::Orca, "Orca", "O"));
        views.push((ViewMode::Babel, "Babel", "L"));
        views.push((ViewMode::Strings, "Strings", "="));
        views.push((ViewMode::Quipu, "Quipu", "Tab"));
        views.push((ViewMode::Hydra, "Hydra", "Y"));
        views.push((ViewMode::Chronos, "Chronos", "T"));
        views.push((ViewMode::Logos, "Logos", "U"));
        views.push((ViewMode::Pandemonium, "Pandemonium", "P"));
        views.push((ViewMode::BioticChaos, "Biotic Chaos", "Tab"));
        views.push((ViewMode::Catalyst, "Catalyst Chamber", "Tab"));
        views.push((ViewMode::Hyperspace, "Hyperspace", "H"));
        views.push((ViewMode::Hologram, "Hologram", "I"));
        views.push((ViewMode::Weaver, "The Weaver", "W"));
        views.push((ViewMode::Terminal, "Terminal", "`"));
        views.push((ViewMode::Attractor, "Attractor", "A"));
        views.push((ViewMode::Virology, "Virology", "v"));
        views.push((ViewMode::BioMesh, "BioMesh", "N"));
        views.push((ViewMode::Reactor, "Reactor", "X"));
        views.push((ViewMode::Evolution, "Evolution", "E"));
        views.push((ViewMode::Ecology, "Genetic Ecology", "Shift+E"));
        views.push((ViewMode::Savant, "Savant", "Shift+\\"));
    }
    views
}

#[cfg(feature = "nova")]
pub(crate) fn layout_tree_node(
    node_id: usize,
    depth: f64,
    current_y: &mut f64,
    positions: &mut std::collections::HashMap<usize, (f64, f64)>,
    vm: &ChimeraVM,
    max_depth: &mut f64,
) -> f64 {
    if depth > *max_depth {
        *max_depth = depth;
    }

    let mut my_y = *current_y;

    if let Some(node) = vm.cladistics.nodes.get(&node_id) {
        if node.children.is_empty() {
            *current_y += 1.0;
        } else {
            let mut sum_y = 0.0;
            let count = node.children.len() as f64;
            for child_id in &node.children {
                sum_y +=
                    layout_tree_node(*child_id, depth + 1.0, current_y, positions, vm, max_depth);
            }
            my_y = sum_y / count;
        }
        positions.insert(node_id, (depth, my_y));
    }
    my_y
}
