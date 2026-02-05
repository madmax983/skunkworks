use anyhow::Result;
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
use std::{io, time::{Duration, Instant}};
use tui_shared::Tui;

use crate::audio::{AudioCommand, AudioModel};

// Chimera Imports
use chimera_lang::{
    ast::{Dna, Helix, Strand, Gene, Nucleotide},
    opcode::OpCode,
    vm::ChimeraVM,
};
use rand::Rng;

pub mod audio;
pub mod physics;

struct Agent {
    vm: ChimeraVM,
    x: usize,
    y: usize,
    color: Color,
    last_action: Instant,
}

impl Agent {
    fn new(x: usize, y: usize, dna: Dna, color: Color) -> Self {
        Self {
            vm: ChimeraVM::new(dna),
            x,
            y,
            color,
            last_action: Instant::now(),
        }
    }
}

fn make_plucker_dna() -> Dna {
    // Loop: PUSH 100, PHOTOSYNTHESIZE, JUMP 0
    // The "100" will be interpreted as a Pluck strength
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Photosynthesize, // Acts as a delay/energy gather
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    let strand = Strand { genes };
    Dna { helix: Helix { strands: vec![strand] } }
}

fn make_fast_plucker_dna() -> Dna {
    // Loop: PUSH 50, JUMP 0
    // Faster beat
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    let strand = Strand { genes };
    Dna { helix: Helix { strands: vec![strand] } }
}

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

        let stream_result = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    model.process(data);
                },
                err_fn,
                None,
            ),
            _ => return run_visual_only(),
        };

        let stream = match stream_result {
            Ok(s) => s,
            Err(_) => return run_visual_only(),
        };

        if let Err(_) = stream.play() {
            return run_visual_only();
        }

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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    width: usize,
    height: usize,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<Vec<f32>>,
) -> io::Result<()> {
    let mut cursor_x = width / 2;
    let mut cursor_y = height / 2;

    // Initialize Agents
    let mut rng = rand::thread_rng();
    let mut agents = Vec::new();

    // Create a few random agents
    for _ in 0..5 {
        agents.push(Agent::new(
            rng.gen_range(5..width-5),
            rng.gen_range(5..height-5),
            make_plucker_dna(),
            Color::Green,
        ));
    }
    // One fast agent
    agents.push(Agent::new(
        rng.gen_range(5..width-5),
        rng.gen_range(5..height-5),
        make_fast_plucker_dna(),
        Color::Magenta,
    ));

    let mut grid_u = vec![0.0; width * height];

    // Send initial positions as walls or just visuals?
    // Let's not make them walls, but maybe they pluck.

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            grid_u = snap;
        }

        // Agent Logic
        for agent in &mut agents {
            // Tick VM
            // We limit execution speed slightly so they don't run millions of ops per frame
            // But ChimeraVM is synchronous.
            // Let's execute one step per frame.
            if !agent.vm.halted {
                agent.vm.step();
            }

            // Check output (Stack)
            // If stack has value, interpret as Pluck Strength
            if let Some(val) = agent.vm.stack.last() {
                // If the value is a Number and > 0, pluck
                match val {
                    chimera_lang::vm::Value::Int(n) => {
                         if *n > 0 {
                             // Pluck!
                             let strength = (*n as f32).min(100.0) / 100.0; // Normalize 0-100 -> 0.0-1.0
                             let _ = cmd_tx.send(AudioCommand::Pluck {
                                 x: agent.x,
                                 y: agent.y,
                                 strength,
                             });
                             // Pop the value so we don't pluck forever on same value
                             agent.vm.stack.pop();

                             // Move randomly on pluck?
                             let dx = rng.gen_range(0..3) as i32 - 1;
                             let dy = rng.gen_range(0..3) as i32 - 1;
                             let new_x = (agent.x as i32 + dx).clamp(0, width as i32 - 1) as usize;
                             let new_y = (agent.y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                             agent.x = new_x;
                             agent.y = new_y;
                         }
                    }
                    _ => {
                        agent.vm.stack.pop(); // Clear garbage
                    }
                }
            }
        }

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
                    if x >= width || y >= height {
                        continue;
                    }
                    let idx = y * width + x;
                    let u = grid_u.get(idx).unwrap_or(&0.0);
                    let val = *u;

                    let mut style = Style::default();
                    let mut ch = ' ';

                    // Check for agent
                    let mut agent_here = None;
                    for agent in &agents {
                        if agent.x == x && agent.y == y {
                            agent_here = Some(agent);
                            break;
                        }
                    }

                    if x == cursor_x && y == cursor_y {
                        style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                        ch = 'X';
                    } else if let Some(agent) = agent_here {
                        style = style.fg(agent.color).add_modifier(Modifier::BOLD);
                        ch = 'A'; // Agent
                    } else {
                        if val > 0.05 {
                            style = style.fg(Color::Green);
                            if val > 0.5 {
                                ch = '@';
                            } else if val > 0.2 {
                                ch = 'O';
                            } else {
                                ch = '.';
                            }
                        } else if val < -0.05 {
                            style = style.fg(Color::Red);
                            if val < -0.5 {
                                ch = '@';
                            } else if val < -0.2 {
                                ch = 'O';
                            } else {
                                ch = '.';
                            }
                        } else if val.abs() > 0.01 {
                            style = style.fg(Color::DarkGray);
                            ch = '·';
                        }
                    };

                    spans.push(Span::styled(String::from(ch), style));
                }
                lines.push(Line::from(spans));
            }

            let grid_widget = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Chimera Resonance (Agents Plucking Strings)"),
            );
            f.render_widget(grid_widget, area);

            let info = Paragraph::new(
                "Agents (A) are executing DNA and plucking the grid. | q: Quit",
            )
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => cursor_y = cursor_y.saturating_sub(1),
                    KeyCode::Down => {
                        if cursor_y < height - 1 {
                            cursor_y += 1;
                        }
                    }
                    KeyCode::Left => cursor_x = cursor_x.saturating_sub(1),
                    KeyCode::Right => {
                        if cursor_x < width - 1 {
                            cursor_x += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
