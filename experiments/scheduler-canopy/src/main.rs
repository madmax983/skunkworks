use macroquad::prelude::*;

mod cpu;
mod memory;
mod process;

use cpu::Scheduler;
use memory::MemoryGrid;
use process::{ProcessState, ProcessTree};

#[macroquad::main("Scheduler Canopy")]
async fn main() {
    let mut scheduler = Scheduler::new();

    // Grid settings
    // 100x50 grid for memory? Or pixel based?
    // Let's do 128x64 grid.
    let grid_w = 128;
    let grid_h = 64;
    let mut memory = MemoryGrid::new(grid_w, grid_h);

    let mut processes: Vec<ProcessTree> = Vec::new();
    let mut next_pid = 1;

    loop {
        // --- Input ---
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();
            let ground_y = sh * 0.5;

            // Only spawn if clicked in valid area (near ground?)
            if my < ground_y + 50.0 && my > ground_y - 50.0 {
                let mut p = ProcessTree::new(next_pid, vec2(mx, ground_y), sw);
                p.init_roots(&mut memory);
                if p.state != ProcessState::Zombie {
                    processes.push(p);
                    next_pid += 1;
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            processes.clear();
            memory = MemoryGrid::new(grid_w, grid_h);
            next_pid = 1;
        }

        // --- Update ---
        let dt = get_frame_time();
        scheduler.update(dt);

        for p in &mut processes {
            p.update(&scheduler, &mut memory, dt);
        }

        // Remove dead processes?
        // Or keep zombies as withered trees?
        // Let's keep them but maybe color them gray.

        // --- Draw ---
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0)); // Dark Blue Sky

        let sw = screen_width();
        let sh = screen_height();
        let ground_y = sh * 0.5;

        // Draw Sun Beam
        // Map sun angle (0..PI) to screen X
        // Sun Angle 0 = Left, PI = Right
        let sun_x = (scheduler.sun_angle / std::f32::consts::PI) * sw;
        let beam_w_px = (scheduler.beam_width / std::f32::consts::PI) * sw;

        draw_rectangle(
            sun_x - beam_w_px / 2.0,
            0.0,
            beam_w_px,
            sh,
            Color::new(1.0, 1.0, 0.8, 0.1) // Faint yellow beam
        );

        // Draw Sun Orb
        draw_circle(sun_x, 50.0, 30.0, YELLOW);

        // Draw Ground Line
        draw_line(0.0, ground_y, sw, ground_y, 2.0, BROWN);

        // Draw Memory Grid (Roots)
        // Map grid to bottom half
        let cell_w = sw / grid_w as f32;
        let cell_h = (sh - ground_y) / grid_h as f32;

        for y in 0..grid_h {
            for x in 0..grid_w {
                if let Some(pid) = memory.get(x, y) {
                    let color = get_pid_color(pid);
                    draw_rectangle(
                        x as f32 * cell_w,
                        ground_y + y as f32 * cell_h,
                        cell_w,
                        cell_h,
                        color,
                    );
                }
            }
        }

        // Draw Trees (Canopy)
        for p in &processes {
            let color = if p.state == ProcessState::Zombie {
                GRAY
            } else if p.state == ProcessState::Running {
                // Flash if running?
                if get_time() % 0.2 < 0.1 { WHITE } else { get_pid_color(p.pid) }
            } else {
                get_pid_color(p.pid)
            };

            // Draw Trunk
            draw_circle(p.position.x, p.position.y, 5.0, color);

            // Draw Branches
            for branch in &p.branches {
                draw_line(
                    branch.start.x, branch.start.y,
                    branch.end.x, branch.end.y,
                    branch.thickness,
                    color
                );
            }
        }

        // UI
        draw_text("Scheduler Canopy", 20.0, 30.0, 30.0, WHITE);
        draw_text("Click near horizon to plant process.", 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Processes: {}", processes.len()), 20.0, 70.0, 20.0, WHITE);

        next_frame().await
    }
}

fn get_pid_color(pid: usize) -> Color {
    // Generate distinct colors based on PID
    let hue = (pid * 137) as f32 % 360.0;
    hsl_to_rgb(hue / 360.0, 0.8, 0.5)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    Color::new(r, g, b, 1.0)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 { t += 1.0; }
    if t > 1.0 { t -= 1.0; }
    if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0 / 2.0 { return q; }
    if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    p
}
