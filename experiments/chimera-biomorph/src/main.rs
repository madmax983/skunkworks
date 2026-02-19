use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use glam::Vec3;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::ChimeraVM,
};

mod physics;
use physics::{GeneticString, GeneticNode};

struct BioChimera {
    vm: ChimeraVM,
    string: GeneticString,
}

impl BioChimera {
    fn new() -> Self {
        // Create a simple genome: A loop that pushes numbers and consumes energy
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Add, args: vec![] },
            Gene { op: OpCode::Dup, args: vec![] },
            Gene { op: OpCode::Print, args: vec![] }, // Output visible
            Gene { op: OpCode::Photosynthesize, args: vec![] }, // Gain energy
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Jump, args: vec![] }, // Loop
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: genes.clone() }],
            },
        };

        let vm = ChimeraVM::new(dna);

        // Initialize physics string
        // Center it
        let start = Vec3::new(-60.0, 0.0, 0.0);
        let end = Vec3::new(60.0, 0.0, 0.0);
        let string = GeneticString::new(genes, start, end, 20.0, 0.5);

        Self { vm, string }
    }

    fn update(&mut self, dt: f32) {
        // 1. Update Physics
        self.string.update(dt);

        // 2. Check for Kinetic Mutagenesis
        if self.string.check_mutation_trigger() {
            // Trigger mutation in VM
            // We use the VM's built-in mutate() function which handles logic
            // But we might want to force it more aggressively based on energy
            self.vm.mutate();

            // Sync physics nodes to new DNA
            // This is the tricky part. The VM mutation modifies self.vm.dna
            // We need to reflect that in self.string.nodes
            // Assuming length hasn't changed (mutate mostly changes Op/Arg)
            // If length changes (e.g. transposon), we might need to rebuild the string
            // For now, let's just update Ops
            self.sync_physics_to_dna();
        }

        // 3. Step VM
        // Only step if not halted
        if !self.vm.halted {
             self.vm.step();
        }

        // 4. Update "Active" node based on IP
        // Reset all active flags
        for node in &mut self.string.nodes {
            node.active = false;
        }
        // Set current IP active
        let (strand_idx, gene_idx) = self.vm.ip;
        if strand_idx == 0 && gene_idx < self.string.nodes.len() {
            self.string.nodes[gene_idx].active = true;
        }
    }

    fn sync_physics_to_dna(&mut self) {
        // Assuming we are visualizing Strand 0
        if let Some(strand) = self.vm.dna.helix.strands.get(0) {
            // If lengths match, update in place
            if strand.genes.len() == self.string.nodes.len() {
                for (i, gene) in strand.genes.iter().enumerate() {
                    // Check if OpCode changed
                    let node = &mut self.string.nodes[i];
                    // Compare opcodes roughly (to avoid deep cloning check if not needed)
                    // Actually, just overwrite gene info and check if we need to flag mutation
                    // We assume physics.mutate() or vm.mutate() handles the flag,
                    // but here we are detecting VM-side changes.
                    // Let's just update the gene.
                    // If OpCode type changed, mass might change.

                    // Simple check: to_string comparison?
                    let old_op = node.gene.op.to_string();
                    let new_op = gene.op.to_string();

                    if old_op != new_op {
                        node.mutated = true;
                        // Update mass based on new op
                         node.mass = match gene.op {
                            OpCode::Push | OpCode::Dup | OpCode::Swap => 1.0,
                            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => 2.0,
                            OpCode::Jump | OpCode::Brz => 0.5,
                            OpCode::GRead | OpCode::GWrite => 3.0,
                            OpCode::Photosynthesize | OpCode::Consume => 1.5,
                            _ => 1.0,
                        };
                    }
                    node.gene = gene.clone();
                }
            } else {
                // Length mismatch (Transposon insertion/deletion?)
                // Rebuild string?
                // For this MVP, let's handle simple length mismatch by truncating or padding?
                // Or just rebuild positions (might snap visually).
                // Let's rebuild but try to keep positions if possible.
                // Actually, let's just re-init the string if length changes, it's easier.
                // But we lose velocity.
                // Let's just ignore length changes for now or crash gracefully.
                // Or:
                // self.string = GeneticString::new(strand.genes.clone(), ...);
                // self.string.pluck(); // Add chaos on rebuild
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut bio = BioChimera::new();
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.area());

            // Left: Physics Canvas
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Chimera Biomorph 🧬 (Space: Agitate, Enter: Reset, Q: Quit)"),
                )
                .x_bounds([-80.0, 80.0])
                .y_bounds([-40.0, 40.0])
                .paint(|ctx| {
                    let nodes = &bio.string.nodes;
                    if nodes.is_empty() { return; }

                    // Draw connections
                    for i in 0..nodes.len() - 1 {
                        let n1 = &nodes[i];
                        let n2 = &nodes[i + 1];
                        ctx.draw(&CanvasLine {
                            x1: n1.pos.x as f64,
                            y1: n1.pos.y as f64,
                            x2: n2.pos.x as f64,
                            y2: n2.pos.y as f64,
                            color: Color::DarkGray,
                        });
                    }

                    // Draw nodes
                    for node in nodes {
                        let color = if node.active {
                            Color::Yellow
                        } else if node.mutated {
                            Color::Red
                        } else {
                            match node.gene.op {
                                OpCode::Push => Color::Blue,
                                OpCode::Add | OpCode::Sub => Color::Green,
                                OpCode::Jump | OpCode::Brz => Color::Magenta,
                                _ => Color::Cyan,
                            }
                        };

                        let label = node.gene.op.to_string();
                        // Truncate label if too long
                        let short_label = if label.len() > 4 { &label[0..4] } else { &label };

                        ctx.print(
                            node.pos.x as f64,
                            node.pos.y as f64,
                            Span::styled(short_label.to_string(), Style::default().fg(color)),
                        );
                    }
                });
            f.render_widget(canvas, chunks[0]);

            // Right: VM State
            let vm = &bio.vm;
            let status_text = vec![
                Line::from(Span::styled("VM State", Style::default().add_modifier(Modifier::BOLD))),
                Line::from(format!("Energy: {}", vm.energy)),
                Line::from(format!("IP: {:?}", vm.ip)),
                Line::from(format!("Stack: {:?}", vm.stack)),
                Line::from(""),
                Line::from(Span::styled("Physics", Style::default().add_modifier(Modifier::BOLD))),
                Line::from(format!("Kinetic Energy: {:.2}", bio.string.kinetic_energy)),
                Line::from(format!("Mutation Threshold: {:.2}", bio.string.mutation_threshold)),
                Line::from(""),
                Line::from(Span::styled("Output Log", Style::default().add_modifier(Modifier::BOLD))),
            ];

            // Add last 10 log lines
            let start_log = vm.output.len().saturating_sub(10);
            let logs: Vec<Line> = vm.output[start_log..].iter().map(|s| Line::from(Span::raw(s))).collect();

            let mut full_text = status_text;
            full_text.extend(logs);

            let status = Paragraph::new(full_text)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => {
                        bio = BioChimera::new();
                    }
                    KeyCode::Char(' ') => {
                        bio.string.pluck();
                    }
                    KeyCode::Char('m') => {
                        // Force mutation
                        bio.vm.mutate();
                        bio.sync_physics_to_dna();
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = 0.016;
            bio.update(dt);
            bio.string.reset_mutation_flags();
            last_tick = Instant::now();
        }
    }
}
