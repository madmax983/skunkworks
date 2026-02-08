use anyhow::Result;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod audio;
use audio::{AudioEngine, AudioEvent};

struct App {
    vm: ChimeraVM,
    audio_tx: crossbeam_channel::Sender<AudioEvent>,
    paused: bool,
    gravity_timer: Instant,
    gravity_interval: Duration,
    last_pc: (usize, usize),
}

impl App {
    fn new(audio_tx: crossbeam_channel::Sender<AudioEvent>) -> Self {
        let dna = Self::generate_dna();
        Self {
            vm: ChimeraVM::new(dna),
            audio_tx,
            paused: false,
            gravity_timer: Instant::now(),
            gravity_interval: Duration::from_millis(150), // Speed of gravity
            last_pc: (0, 0),
        }
    }

    fn generate_dna() -> Dna {
        // Create a simple program:
        // Strand 0: Counter logic
        //   Push 0 (Counter)
        //   Dup
        //   Push 1
        //   Add
        //   Dup
        //   Push 10
        //   Sub
        //   Brz 1 (Jump to Strand 1 if 10)
        //   Jump 0 (Loop)

        // Strand 1: Finish
        //   Push 999
        //   Print
        //   Jump 0 (Reset)

        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Dup,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Add,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Dup,
                    args: vec![],
                }, // Stack: [..., N, N]
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(10)],
                },
                Gene {
                    op: OpCode::Sub,
                    args: vec![],
                }, // Stack: [..., N, N-10]
                Gene {
                    op: OpCode::Brz,
                    args: vec![Nucleotide::Number(1)],
                }, // If 0, jump to strand 1
                Gene {
                    op: OpCode::Drop,
                    args: vec![],
                }, // Drop the difference
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                }, // Loop
            ],
        };

        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(999)],
                },
                Gene {
                    op: OpCode::Print,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Drop,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Drop,
                    args: vec![],
                }, // Clean up
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                }, // Restart
            ],
        };

        Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        if self.gravity_timer.elapsed() >= self.gravity_interval {
            self.gravity_timer = Instant::now();
            self.last_pc = self.vm.ip; // Store previous PC for visuals

            // Execute one step
            self.vm.step();

            // Trigger audio
            self.trigger_audio();
        }
    }

    fn trigger_audio(&self) {
        // Determine sound based on the executed instruction (or just generic)
        // We can look at the instruction at last_pc (if valid)
        // Or just map based on the opcode type.

        let (s_idx, g_idx) = self.last_pc;
        if let Some(strand) = self.vm.dna.helix.strands.get(s_idx) {
            if let Some(gene) = strand.genes.get(g_idx) {
                match gene.op {
                    OpCode::Push | OpCode::Dup => {
                        let _ = self.audio_tx.send(AudioEvent::Pluck(220.0));
                    }
                    OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                        let _ = self.audio_tx.send(AudioEvent::Pluck(440.0));
                    }
                    OpCode::Jump | OpCode::Brz => {
                        let _ = self.audio_tx.send(AudioEvent::Kick);
                    }
                    OpCode::Print => {
                        let _ = self.audio_tx.send(AudioEvent::Snare);
                    }
                    _ => {
                        let _ = self.audio_tx.send(AudioEvent::HiHat);
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    // Initialize Audio
    let audio = AudioEngine::new()?;
    let tx = audio.get_sender();

    // Initialize TUI
    let mut tui = Tui::init()?;
    let mut app = App::new(tx);

    loop {
        app.update();

        tui.terminal.draw(|f| {
            ui(f, &app);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('r') => {
                            app.vm = ChimeraVM::new(App::generate_dna());
                            app.last_pc = (0, 0);
                        }
                        KeyCode::Up => {
                            if app.gravity_interval.as_millis() > 10 {
                                app.gravity_interval = app
                                    .gravity_interval
                                    .saturating_sub(Duration::from_millis(10));
                            }
                        }
                        KeyCode::Down => {
                            app.gravity_interval += Duration::from_millis(10);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("🧬 QUIPU CHIMERA: Genetic Code Knots 🧬")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    // Canvas for Quipu (DNA)
    // We render strands as vertical lines.
    // Genes are knots on the lines.

    let strand_count = app.vm.dna.helix.strands.len();
    let max_genes = app
        .vm
        .dna
        .helix
        .strands
        .iter()
        .map(|s| s.genes.len())
        .max()
        .unwrap_or(0);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Quipu Code "))
        .x_bounds([0.0, strand_count as f64])
        .y_bounds([0.0, (max_genes + 2) as f64]) // +2 for padding
        .paint(|ctx| {
            for (s_idx, strand) in app.vm.dna.helix.strands.iter().enumerate() {
                let x = s_idx as f64 + 0.5;

                // Draw Cord
                ctx.draw(&CanvasLine {
                    x1: x,
                    y1: 0.0,
                    x2: x,
                    y2: (max_genes + 1) as f64,
                    color: Color::DarkGray,
                });

                // Draw Knots (Genes)
                // We draw from Top (High Y) to Bottom (Low Y) or vice versa?
                // Typically Quipu hangs down. So index 0 is at Top.
                // In Canvas, Y increases upwards.
                // So index 0 should be at High Y.

                for (g_idx, gene) in strand.genes.iter().enumerate() {
                    let y = (max_genes as f64 - g_idx as f64) + 0.5;

                    // Determine Knot Style based on OpCode
                    let (color, size) = match gene.op {
                        OpCode::Push => (Color::Blue, 0.4),
                        OpCode::Add | OpCode::Sub => (Color::Red, 0.3),
                        OpCode::Jump | OpCode::Brz => (Color::Yellow, 0.5),
                        OpCode::Print => (Color::Green, 0.4),
                        _ => (Color::White, 0.2),
                    };

                    // Draw Knot
                    ctx.draw(&Rectangle {
                        x: x - size / 2.0,
                        y: y - size / 2.0,
                        width: size,
                        height: size,
                        color,
                    });

                    // Highlight PC (Playhead)
                    if app.vm.ip == (s_idx, g_idx) {
                        ctx.draw(&Rectangle {
                            x: x - 0.3,
                            y: y - 0.3,
                            width: 0.6,
                            height: 0.6,
                            color: Color::Magenta,
                        });
                    }
                }
            }
        });
    f.render_widget(canvas, main_chunks[0]);

    // Stack and Stats
    let mut stats_text = vec![
        Line::from(Span::styled(
            "Instruction Pointer (IP):",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(format!("{:?}", app.vm.ip)),
        Line::from(""),
        Line::from(Span::styled("Stack:", Style::default().fg(Color::Green))),
    ];

    for (i, val) in app.vm.stack.iter().rev().take(10).enumerate() {
        stats_text.push(Line::from(format!(
            "{}: {:?}",
            app.vm.stack.len() - 1 - i,
            val
        )));
    }

    stats_text.push(Line::from(""));
    stats_text.push(Line::from(format!("Energy: {}", app.vm.energy)));
    stats_text.push(Line::from(format!(
        "Speed: {}ms",
        app.gravity_interval.as_millis()
    )));

    let stats_block =
        Paragraph::new(stats_text).block(Block::default().borders(Borders::ALL).title(" State "));
    f.render_widget(stats_block, main_chunks[1]);

    let footer = Paragraph::new("Space: Pause | Up/Down: Speed | R: Reset | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
