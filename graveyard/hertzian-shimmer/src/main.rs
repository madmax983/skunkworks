use anyhow::Result;
use crossbeam_channel::{bounded, Sender};
use crossterm::event::{self, Event, KeyCode, MouseEvent, MouseEventKind};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Borders, Paragraph, Widget},
    Terminal,
};
use std::{io, path::PathBuf, thread, time::Duration};
use tui_shared::Tui;

use resonance_audio::audio::{AudioCommand, AudioModel};

mod map;
use map::{build_tree, generate_layout};

const SIM_WIDTH: usize = 80;
const SIM_HEIGHT: usize = 40;

fn main() -> Result<()> {
    // 1. Generate Map
    let root = build_tree(&PathBuf::from(".")); // Scan current dir
    let (walls, _rooms) = generate_layout(&root, SIM_WIDTH, SIM_HEIGHT);

    // 2. Setup Audio/Physics
    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(2);

    // Initialize Physics with walls
    #[cfg(feature = "audio")]
    let _audio_stream = setup_audio(cmd_rx, snap_tx);

    #[cfg(not(feature = "audio"))]
    let _audio_thread = setup_mock_audio(cmd_rx, snap_tx);

    // Send initial walls
    for (i, &is_wall) in walls.iter().enumerate() {
        if is_wall {
            let x = i % SIM_WIDTH;
            let y = i / SIM_WIDTH;
            let _ = cmd_tx.send(AudioCommand::AddWall { x, y });
        }
    }

    // 3. TUI Setup
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal, cmd_tx, snap_rx, walls);

    if let Err(err) = &res {
        tui.exit()?;
        eprintln!("{:?}", err);
    } else {
        tui.exit()?;
    }

    Ok(())
}

fn setup_mock_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: Sender<Vec<f32>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut model = AudioModel::new(SIM_WIDTH, SIM_HEIGHT, cmd_rx, snap_tx);
        let mut buffer = vec![0.0; 1024];
        loop {
            model.process(&mut buffer);
            thread::sleep(Duration::from_millis(20));
        }
    })
}

#[cfg(feature = "audio")]
fn setup_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: Sender<Vec<f32>>,
) -> Option<cpal::Stream> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let host = cpal::default_host();
    let device = host.default_output_device();

    if let Some(device) = device {
        if let Ok(config) = device.default_output_config() {
            let cmd_rx_clone = cmd_rx.clone();
            let snap_tx_clone = snap_tx.clone();

            let mut model = AudioModel::new(SIM_WIDTH, SIM_HEIGHT, cmd_rx_clone, snap_tx_clone);
            let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

            let stream_res = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        model.process(data);
                    },
                    err_fn,
                    None,
                ),
                _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
            };

            if let Ok(stream) = stream_res {
                if stream.play().is_ok() {
                    return Some(stream);
                }
            }
        }
    }

    // Fallback if device fails or config fails
    eprintln!("Audio initialization failed or no device found. Falling back to mock.");
    setup_mock_audio(cmd_rx, snap_tx);
    None
}

struct HeatmapWidget<'a> {
    data: &'a [f32],
    walls: &'a [bool],
    width: usize,
    height: usize,
}

impl<'a> Widget for HeatmapWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if x >= area.width as usize || y >= area.height as usize {
                    continue;
                }

                let idx = y * self.width + x;
                let val = self.data.get(idx).unwrap_or(&0.0);
                let is_wall = self.walls.get(idx).copied().unwrap_or(false);

                let (char, color) = if is_wall {
                    ('█', Color::White)
                } else {
                    let v = *val;
                    if v > 0.5 {
                        ('@', Color::Red)
                    } else if v > 0.2 {
                        ('%', Color::LightRed)
                    } else if v > 0.05 {
                        ('.', Color::DarkGray)
                    } else if v < -0.5 {
                        ('@', Color::Blue)
                    } else if v < -0.2 {
                        ('%', Color::LightBlue)
                    } else if v < -0.05 {
                        ('.', Color::DarkGray)
                    } else {
                        (' ', Color::Black)
                    }
                };

                if let Some(cell) = buf.cell_mut((area.left() + x as u16, area.top() + y as u16)) {
                    cell.set_char(char).set_fg(color);
                }
            }
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<Vec<f32>>,
    walls: Vec<bool>,
) -> io::Result<()> {
    let mut grid_u = vec![0.0; SIM_WIDTH * SIM_HEIGHT];

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            grid_u = snap;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let grid_area = chunks[0];

            // Center the simulation in the viewport
            let render_w = (grid_area.width as usize).min(SIM_WIDTH);
            let render_h = (grid_area.height as usize).min(SIM_HEIGHT);

            let centered_area = Rect::new(
                grid_area.x + (grid_area.width.saturating_sub(render_w as u16)) / 2,
                grid_area.y + (grid_area.height.saturating_sub(render_h as u16)) / 2,
                render_w as u16,
                render_h as u16,
            );

            let heatmap = HeatmapWidget {
                data: &grid_u,
                walls: &walls,
                width: SIM_WIDTH,
                height: SIM_HEIGHT,
            };

            f.render_widget(heatmap, centered_area);

            let info = Paragraph::new(
                "Hertzian Shimmer: Codebase Acoustics. Click/Space to Pluck. 'q' to Quit.",
            )
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                    if key.code == KeyCode::Char(' ') {
                        // Random pluck
                        let x = SIM_WIDTH / 2;
                        let y = SIM_HEIGHT / 2;
                        let _ = cmd_tx.send(AudioCommand::Pluck {
                            x,
                            y,
                            strength: 1.0,
                        });
                    }
                }
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Down(_),
                    ..
                }) => {
                    // Try to map mouse to grid
                    // This is hard without knowing the calculated 'centered_area' from the draw loop.
                    // But we can approximate or just ignore mouse for MVP.
                    // Let's ignore mouse precision for now to save complexity, Space is enough.
                    // Or, just pluck at center.
                    let x = SIM_WIDTH / 2;
                    let y = SIM_HEIGHT / 2;
                    let _ = cmd_tx.send(AudioCommand::Pluck {
                        x,
                        y,
                        strength: 1.0,
                    });
                }
                _ => {}
            }
        }
    }
}
