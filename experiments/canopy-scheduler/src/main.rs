use macroquad::prelude::*;

mod simulation;

use simulation::{ProcessState, ProcessTree, Scheduler, SchedulingAlgorithm};

#[macroquad::main("Canopy Scheduler")]
async fn main() {
    let mut scheduler = Scheduler::new();
    let mut running = true;

    // Initial processes
    for _ in 0..5 {
        add_random_process(&mut scheduler);
    }

    loop {
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
        }

        // Logic
        if running {
            let dt = get_frame_time();
            scheduler.update(dt, get_time());
        }

        // Rendering
        clear_background(SKYBLUE);

        // Ground
        draw_rectangle(
            0.0,
            screen_height() - 50.0,
            screen_width(),
            50.0,
            Color::new(0.4, 0.26, 0.13, 1.0), // Dirt
        );

        // Sun (Scheduler)
        draw_circle(scheduler.sun_pos, 50.0, 30.0, YELLOW);
        // Sun Rays
        draw_line(
            scheduler.sun_pos,
            50.0,
            scheduler.sun_pos,
            screen_height(),
            2.0,
            Color::new(1.0, 1.0, 0.0, 0.2),
        );

        // Trees
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
    let width = rand::gen_range(20.0, 50.0);
    // Find a spot? For now, just random placement.
    let pos = rand::gen_range(50.0, screen_width() - 50.0);
    let priority = rand::gen_range(0, 255) as u8;
    let cpu_needed = rand::gen_range(100.0, 400.0); // Height

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

    // Outline for target height (ghost of potential)
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
        // Priority determines canopy color vibrancy? No, ProcessTree handles color.

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

    draw_text("Canopy Scheduler", 20.0, 30.0, 30.0, BLACK);
    draw_text(
        &format!("Algorithm: {} (S)", algo_name),
        20.0,
        60.0,
        20.0,
        BLACK,
    );
    draw_text(
        &format!("Processes: {}", scheduler.processes.len()),
        20.0,
        80.0,
        20.0,
        BLACK,
    );

    if let Some(idx) = scheduler.current_process_idx {
        if idx < scheduler.processes.len() {
             let p = &scheduler.processes[idx];
             draw_text(
                &format!("Running: PID {} | Prio {} | Rem {:.1}", p.id, p.priority, scheduler.time_left),
                20.0, 100.0, 20.0, BLACK
            );
        }
    } else {
        draw_text("Running: Idle", 20.0, 100.0, 20.0, BLACK);
    }

    if !running {
        draw_text("PAUSED", screen_width() - 150.0, 30.0, 30.0, RED);
    }

    draw_text(
        "[Space] Pause | [A] Add | [K] Kill | [R] Reset",
        20.0,
        screen_height() - 20.0,
        20.0,
        BLACK,
    );
}
