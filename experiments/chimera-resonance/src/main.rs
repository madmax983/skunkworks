use anyhow::Result;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{bounded, Sender};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};
use tui_shared::Tui;

use crate::audio::{AudioCommand, AudioModel};

pub mod audio;
pub mod physics;

fn main() -> Result<()> {
    #[cfg(feature = "audio")]
    {
        // Audio Setup
        let host = cpal::default_host();
        let device = match host.default_output_device() {
            Some(d) => d,
            None => return run_visual_only(),
        };

        let config = match device.default_output_config() {
            Ok(c) => c,
            Err(_) => return run_visual_only(),
        };

        let (cmd_tx, cmd_rx) = bounded(100);
        let (snap_tx, snap_rx) = bounded(2);

        let width = 60;
        let height = 30;

        let mut model = AudioModel::new(width, height, cmd_rx, snap_tx);
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    model.process(data);
                },
                err_fn,
                None,
            ),
            _ => return run_visual_only(),
        }?;

        stream.play()?;

        // TUI Setup
        let mut tui = Tui::init()?;
        let res = run_app(&mut tui.terminal, width, height, cmd_tx, snap_rx);

        if let Err(err) = &res {
            tui.exit()?;
            println!("{:?}", err);
        } else {
            tui.exit()?;
        }

        Ok(())
    }

    #[cfg(not(feature = "audio"))]
    {
        run_visual_only()
    }
}

fn run_visual_only() -> Result<()> {
    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(2);
    let width = 60;
    let height = 30;

    let mut model = AudioModel::new(width, height, cmd_rx, snap_tx);
    std::thread::spawn(move || {
        let mut buffer = vec![0.0; 1024];
        loop {
            model.process(&mut buffer);
            std::thread::sleep(Duration::from_millis(20));
        }
    });

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal, width, height, cmd_tx, snap_rx);
    tui.exit()?;
    res.map_err(|e| anyhow::anyhow!(e))
}

fn make_dna() -> Dna {
    // Strand 0: Spawner
    // [ Push(0) Push(1) Spawn ] -> Spawn Worker on Strand 1
    // [ Push(0) Push(2) Spawn ] -> Spawn Worker on Strand 2
    // [ Push(1000) Consume ] -> Eat heavily to fuel spawns
    // [ Halt ]
    let strand0 = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Spawn, args: vec![] }, // Worker on Strand 1
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Spawn, args: vec![] }, // Worker on Strand 2
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1000)] },
            Gene { op: OpCode::Consume, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(10)] }, // Loop or halt
        ],
    };

    // Strand 1: Circular Walker
    // Right, Down, Left, Up
    let strand1 = Strand {
        genes: vec![
            // Move Right (0, 1)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Migrate, args: vec![] },
            // Move Down (1, 0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Migrate, args: vec![] },
            // Move Left (0, -1)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Migrate, args: vec![] },
            // Move Up (-1, 0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] },
            Gene { op: OpCode::Migrate, args: vec![] },
            // Loop
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ],
    };

    // Strand 2: Diagonal Bouncer
    let strand2 = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Migrate, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] },
            Gene { op: OpCode::Migrate, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ],
    };

    Dna {
        helix: Helix {
            strands: vec![strand0, strand1, strand2],
        },
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    width: usize,
    height: usize,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<Vec<f32>>,
) -> io::Result<()> {
    // Setup VM
    let mut vm = ChimeraVM::new(make_dna());
    vm.energy = 5000; // Give plenty of energy
    vm.topology = chimera_lang::vm::Topology::Torus; // Wrap around

    // Physics Grid State
    let mut grid_u = vec![0.0; width * height];

    // VM Offset (Center 16x16 in 60x30)
    let offset_x = (width.saturating_sub(16)) / 2;
    let offset_y = (height.saturating_sub(16)) / 2;

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            grid_u = snap;
        }

        // TUI Render
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let area = chunks[0];

            let mut lines = Vec::new();
            for y in 0..height {
                let mut spans = Vec::new();
                for x in 0..width {
                    if x >= width || y >= height { continue; }
                    let idx = y * width + x;
                    let u = grid_u.get(idx).unwrap_or(&0.0);
                    let val = *u;

                    // Base style from Physics
                    let mut style = Style::default();
                    let mut ch = ' ';

                    if val > 0.05 {
                        style = style.fg(Color::Cyan);
                        ch = '~';
                    } else if val < -0.05 {
                        style = style.fg(Color::Blue);
                        ch = '~';
                    } else if val.abs() > 0.01 {
                        style = style.fg(Color::DarkGray);
                        ch = '·';
                    }

                    // Overlay VM
                    // Check if (x, y) is inside VM Grid
                    if x >= offset_x && x < offset_x + 16 && y >= offset_y && y < offset_y + 16 {
                        let vm_x = x - offset_x;
                        let vm_y = y - offset_y;

                        // Check organelles
                        let mut has_organelle = false;
                        for org in &vm.organelles {
                            if org.context_loc == (vm_y, vm_x) {
                                style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                                ch = match org.kind {
                                    chimera_lang::vm::nova::OrganelleType::Worker => 'W',
                                    chimera_lang::vm::nova::OrganelleType::Chloroplast => 'C',
                                    chimera_lang::vm::nova::OrganelleType::Mitochondria => 'M',
                                    chimera_lang::vm::nova::OrganelleType::Ribosome => 'R',
                                    _ => 'O',
                                };
                                has_organelle = true;
                                break;
                            }
                        }

                        if !has_organelle {
                            // Check grid value
                            let cell = &vm.grid[vm_y][vm_x];
                            match cell {
                                Value::Int(n) if *n != 0 => {
                                    style = style.fg(Color::White);
                                    ch = '#';
                                }
                                Value::Str(s) => {
                                    style = style.fg(Color::Green);
                                    ch = s.chars().next().unwrap_or('?');
                                }
                                _ => {}
                            }
                        }

                        // Border visual hint
                        if !has_organelle && ch == ' ' && (val.abs() <= 0.05) {
                             style = style.fg(Color::DarkGray);
                             ch = '.';
                        }
                    }

                    spans.push(Span::styled(String::from(ch), style));
                }
                lines.push(Line::from(spans));
            }

            let grid_widget = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Chimera Resonance 🧬🔊"),
            );
            f.render_widget(grid_widget, area);

            let info_text = format!(
                "Energy: {} | Organelles: {} | Spores: {} | Press 'q' to quit",
                vm.energy,
                vm.organelles.len(),
                vm.spores.len()
            );
            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        // Logic Step
        vm.step();

        // ACOUSTIC COUPLING: Pluck physics grid based on Organelle movement
        // We can't easily detect movement without tracking prev state.
        // Instead, we'll just pluck at current position every N ticks or if energy consumed?
        // Let's pluck at current position with low intensity every tick.
        // Or better: Pluck if they are active (not halted).

        for org in &vm.organelles {
            if !org.halted {
                let (oy, ox) = org.context_loc;
                let px = offset_x + ox;
                let py = offset_y + oy;
                // Send Pluck
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: px,
                    y: py,
                    strength: 0.2, // Small pluck
                });
            }
        }

        // Also pluck for main VM context
        let (cy, cx) = vm.context_loc;
        let _ = cmd_tx.send(AudioCommand::Pluck {
            x: offset_x + cx,
            y: offset_y + cy,
            strength: 0.1,
        });

        // Input Handling
        if event::poll(Duration::from_millis(16))? { // ~60 FPS
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                // Pass input to VM
                if let KeyCode::Char(c) = key.code {
                    vm.handle_input(c);
                }
            }
        }
    }
}
