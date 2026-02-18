use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel, AudioSnapshot};

mod simulation;
use simulation::{Scheduler, ProcessTree, SchedulerEvent, SchedulingAlgorithm, ProcessState};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam_channel::Sender;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;

#[macroquad::main("Ripple Scheduler")]
async fn main() {
    // 1. Audio / Simulation Setup
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    #[cfg(feature = "audio")]
    let _stream = match init_audio(cmd_rx, snap_tx.clone()) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            None
        }
    };

    // If audio is not active, we need to drive the simulation manually
    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx);

    // 2. Scheduler Setup
    let mut scheduler = Scheduler::new();
    let mut running = true;

    // Add initial processes
    for _ in 0..5 {
        add_random_process(&mut scheduler);
    }

    // 3. Visual Setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    // Listener is the "Sun" / CPU Core
    let mut listener_pos = (GRID_WIDTH / 2, GRID_HEIGHT / 2);

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        #[cfg(not(feature = "audio"))]
        {
            // Simulate steps for this frame
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000); // Limit

            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        // Logic
        if running {
            let dt = get_frame_time();
            let events = scheduler.update(dt, get_time());

            // Update Listener Position based on Scheduler Sun
            let screen_w = screen_width();
            let sun_grid_x = ((scheduler.sun_pos / screen_w) * GRID_WIDTH as f32) as usize;
            let sun_grid_x = sun_grid_x.clamp(0, GRID_WIDTH - 1);

            if listener_pos.0 != sun_grid_x {
                listener_pos.0 = sun_grid_x;
                // Update listener in AudioModel
                 let _ = cmd_tx.send(AudioCommand::MoveListener {
                    x: listener_pos.0,
                    y: listener_pos.1,
                });
            }

            for event in events {
                match event {
                    SchedulerEvent::ContextSwitch { .. } => {
                    }
                    SchedulerEvent::ProcessTick { pid: _, pos, priority } => {
                        let grid_x = ((pos / screen_w) * GRID_WIDTH as f32) as usize;
                        let grid_x = grid_x.clamp(0, GRID_WIDTH - 1);

                        let center_y = GRID_HEIGHT / 2;
                        let offset = (255 - priority as i32) as f32 / 4.0; // 0..64
                        let grid_y = if (get_time() * 10.0) as u32 % 2 == 0 {
                            center_y as usize + offset as usize
                        } else {
                            center_y as usize - offset as usize
                        };
                        let grid_y = grid_y.clamp(0, GRID_HEIGHT - 1);

                        let _ = cmd_tx.send(AudioCommand::Pluck {
                            x: grid_x,
                            y: grid_y,
                            strength: 0.5,
                        });
                    }
                }
            }
        }

        // Input Handling
        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }
        if is_key_pressed(KeyCode::A) {
            add_random_process(&mut scheduler);
        }
        if is_key_pressed(KeyCode::S) {
            // Cycle algorithm
            scheduler.algorithm = match scheduler.algorithm {
                SchedulingAlgorithm::RoundRobin => SchedulingAlgorithm::FCFS,
                SchedulingAlgorithm::FCFS => SchedulingAlgorithm::Priority,
                SchedulingAlgorithm::Priority => SchedulingAlgorithm::ShortestJobFirst,
                SchedulingAlgorithm::ShortestJobFirst => SchedulingAlgorithm::RoundRobin,
            };
        }
        if is_key_pressed(KeyCode::K) {
            scheduler.kill_current();
        }
        if is_key_pressed(KeyCode::R) {
             scheduler = Scheduler::new();
             for _ in 0..5 {
                add_random_process(&mut scheduler);
            }
             let _ = cmd_tx.send(AudioCommand::ClearWaves);
        }

        // Update texture
        for (i, &val) in current_snapshot.iter().enumerate() {
            let x = (i % GRID_WIDTH) as u32;
            let y = (i / GRID_WIDTH) as u32;

            let color = if val > 0.0 {
                Color::new(val.min(1.0), 0.0, 0.0, 1.0)
            } else {
                Color::new(0.0, 0.0, (-val).min(1.0), 1.0)
            };
            image.set_pixel(x, y, color);
        }
        texture.update(&image);

        // Draw Background (Ripple Tank)
        clear_background(BLACK);

        let scale = (screen_width() / GRID_WIDTH as f32).min(screen_height() / GRID_HEIGHT as f32);
        let draw_width = GRID_WIDTH as f32 * scale;
        let draw_height = GRID_HEIGHT as f32 * scale;
        let offset_x = (screen_width() - draw_width) / 2.0;
        let offset_y = (screen_height() - draw_height) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                ..Default::default()
            },
        );

        // Draw Listener
        let lx = offset_x + listener_pos.0 as f32 * scale;
        let ly = offset_y + listener_pos.1 as f32 * scale;
        draw_circle(lx + scale / 2.0, ly + scale / 2.0, scale / 2.0, GREEN);

        // Draw Ground
        draw_rectangle(
            0.0,
            screen_height() - 50.0,
            screen_width(),
            50.0,
            Color::new(0.4, 0.26, 0.13, 1.0), // Dirt
        );

        for process in &scheduler.processes {
            draw_tree(process);
        }

        // UI
        draw_ui(&scheduler, running);

        next_frame().await
    }
}

fn add_random_process(scheduler: &mut Scheduler) {
    let id = scheduler.processes.len();
    let width = macroquad::rand::gen_range(20.0, 50.0);
    // Find a spot? For now, just random placement.
    let pos = macroquad::rand::gen_range(50.0, screen_width() - 50.0);
    let priority = macroquad::rand::gen_range(0, 255) as u8;
    let cpu_needed = macroquad::rand::gen_range(100.0, 400.0); // Height

    scheduler.add_process(ProcessTree::new(id, pos, width, priority, cpu_needed, get_time()));
}

fn draw_tree(process: &ProcessTree) {
    let bottom_y = screen_height() - 50.0;
    let height = process.progress;
    let target_height = process.cpu_needed;

    // Trunk
    let trunk_color = if process.state == ProcessState::Zombie {
        GRAY // Dead wood
    } else if process.state == ProcessState::Running {
        // Glowing trunk
        Color::new(
            process.color.r * 1.5,
            process.color.g * 1.5,
            process.color.b * 1.5,
            1.0,
        )
    } else {
        process.color
    };

    draw_rectangle(
        process.pos,
        bottom_y - height,
        process.width,
        height,
        trunk_color,
    );

    // Outline for target height
    draw_rectangle_lines(
        process.pos,
        bottom_y - target_height,
        process.width,
        target_height,
        2.0,
        Color::new(trunk_color.r, trunk_color.g, trunk_color.b, 0.3),
    );

    // Leaves / Canopy
    if process.state != ProcessState::Zombie {
        let canopy_y = bottom_y - height;
        let canopy_size = process.width * 1.5;
        draw_circle(
            process.pos + process.width / 2.0,
            canopy_y,
            canopy_size / 2.0,
            Color::new(0.0, 0.6, 0.0, 0.8),
        );
    }

    // Status Indicator
    let status_color = match process.state {
        ProcessState::Running => GREEN,
        ProcessState::Ready => YELLOW,
        ProcessState::Blocked => RED,
        ProcessState::Zombie => DARKGRAY,
    };

    draw_circle(
        process.pos + process.width / 2.0,
        bottom_y + 25.0, // In the ground (root node)
        5.0,
        status_color,
    );
}

fn draw_ui(scheduler: &Scheduler, running: bool) {
    let algo_name = match scheduler.algorithm {
        SchedulingAlgorithm::RoundRobin => "Round Robin",
        SchedulingAlgorithm::FCFS => "FCFS",
        SchedulingAlgorithm::Priority => "Priority",
        SchedulingAlgorithm::ShortestJobFirst => "SJF",
    };

    draw_text("Ripple Scheduler", 20.0, 30.0, 30.0, WHITE);
    draw_text(
        &format!("Algorithm: {} (S)", algo_name),
        20.0,
        60.0,
        20.0,
        WHITE,
    );
    draw_text(
        &format!("Processes: {}", scheduler.processes.len()),
        20.0,
        80.0,
        20.0,
        WHITE,
    );

    if let Some(idx) = scheduler.current_process_idx {
        if idx < scheduler.processes.len() {
             let p = &scheduler.processes[idx];
             draw_text(
                &format!("Running: PID {} | Prio {} | Rem {:.1}", p.id, p.priority, scheduler.time_left),
                20.0, 100.0, 20.0, WHITE
            );
        }
    } else {
        draw_text("Running: Idle", 20.0, 100.0, 20.0, WHITE);
    }

    if !running {
        draw_text("PAUSED", screen_width() - 150.0, 30.0, 30.0, RED);
    }

    draw_text(
        "[Space] Pause | [A] Add | [K] Kill | [R] Reset",
        20.0,
        screen_height() - 20.0,
        20.0,
        WHITE,
    );
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<AudioSnapshot>,
) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported")),
    };

    stream.play()?;
    Ok(stream)
}
