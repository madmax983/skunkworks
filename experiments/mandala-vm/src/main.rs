mod cipher;
mod vm;

use macroquad::prelude::*;
use cipher::{encode, compile, Jewel, Shape};
use vm::VM;

enum AppState {
    Editor,
    Running,
}

struct App {
    state: AppState,
    source_code: String,
    bytecode: Vec<u8>,
    mandala: Vec<Jewel>,
    vm: VM,
    vm_timer: f32,
    vm_speed: f32,
    scroll_y: f32,
}

impl App {
    fn new() -> Self {
        let source = "PUSH 1\nPUSH 1\nDUP\nprint\nADD\nDUP\nprint\nADD\nDUP\nprint\nJMP 2".to_string(); // Infinite Fib? No, JMP 2 jumps to index 2 (DUP).
        let bytecode = compile(&source);
        let mandala = encode(&bytecode);

        Self {
            state: AppState::Editor,
            source_code: source,
            bytecode,
            mandala,
            vm: VM::new(),
            vm_timer: 0.0,
            vm_speed: 0.1,
            scroll_y: 0.0,
        }
    }

    fn update_mandala(&mut self) {
        self.bytecode = compile(&self.source_code);
        self.mandala = encode(&self.bytecode);
    }
}

#[macroquad::main("Mandala VM")]
async fn main() {
    let mut app = App::new();

    // Initial compile
    app.update_mandala();

    loop {
        clear_background(BLACK);

        match app.state {
            AppState::Editor => {
                update_editor(&mut app);
                draw_editor(&app);
            }
            AppState::Running => {
                update_running(&mut app);
                draw_running(&app);
            }
        }

        draw_mandala(&app);

        // Global UI
        draw_text("TAB: Switch Mode", 20.0, screen_height() - 20.0, 20.0, DARKGRAY);

        if is_key_pressed(KeyCode::Tab) {
            match app.state {
                AppState::Editor => {
                    app.state = AppState::Running;
                    app.vm = VM::new(); // Reset VM
                    // app.vm.execute(&app.bytecode); // No, step by step
                }
                AppState::Running => {
                    app.state = AppState::Editor;
                }
            }
        }

        next_frame().await
    }
}

fn update_editor(app: &mut App) {
    let mut changed = false;
    while let Some(c) = get_char_pressed() {
        if c.is_ascii_graphic() || c == ' ' || c == '\n' {
            app.source_code.push(c);
            changed = true;
        }
    }

    if is_key_pressed(KeyCode::Backspace) {
        app.source_code.pop();
        changed = true;
    }

    if is_key_pressed(KeyCode::Enter) {
        app.source_code.push('\n');
        changed = true;
    }

    if changed {
        app.update_mandala();
    }
}

fn draw_editor(app: &App) {
    // Draw Text Editor on the Left
    draw_rectangle(0.0, 0.0, screen_width() * 0.3, screen_height(), Color::new(0.1, 0.1, 0.1, 1.0));

    let lines: Vec<&str> = app.source_code.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        draw_text(line, 10.0, 30.0 + i as f32 * 20.0, 20.0, WHITE);
    }

    draw_text("EDITOR MODE", 10.0, screen_height() - 50.0, 30.0, GREEN);
}

fn update_running(app: &mut App) {
    if app.vm.halted {
        return;
    }

    app.vm_timer += get_frame_time();
    if app.vm_timer > app.vm_speed {
        app.vm_timer = 0.0;
        app.vm.step(&app.bytecode);
    }
}

fn draw_running(app: &App) {
    // Draw VM State on the Left
    draw_rectangle(0.0, 0.0, screen_width() * 0.3, screen_height(), Color::new(0.1, 0.0, 0.0, 1.0));

    draw_text("RUNNING...", 10.0, 30.0, 30.0, RED);
    draw_text(&format!("PC: {}", app.vm.pc), 10.0, 60.0, 20.0, WHITE);
    draw_text(&format!("STACK: {:?}", app.vm.stack), 10.0, 90.0, 20.0, WHITE);
    draw_text("OUTPUT:", 10.0, 120.0, 20.0, GRAY);
    draw_text(&app.vm.output, 10.0, 140.0, 20.0, YELLOW);

    if app.vm.halted {
        draw_text("HALTED", 10.0, screen_height() - 80.0, 40.0, RED);
    }
}

fn draw_mandala(app: &App) {
    let center = vec2(screen_width() * 0.65, screen_height() / 2.0);

    let mut theta = 0.0f32;
    let mut radius = 20.0f32;

    let jewel_size = 15.0;

    for (i, jewel) in app.mandala.iter().enumerate() {
        let pos = center + vec2(theta.cos(), theta.sin()) * radius;

        let color = match jewel.color {
            cipher::Color::Red => RED,
            cipher::Color::Green => GREEN,
            cipher::Color::Blue => BLUE,
            cipher::Color::Yellow => YELLOW,
            cipher::Color::Purple => PURPLE,
            cipher::Color::Cyan => SKYBLUE,
            cipher::Color::White => WHITE,
            cipher::Color::Black => GRAY,
        };

        // Highlight logic
        let mut is_active = false;
        if let AppState::Running = app.state {
             // PC is in bytes. Each byte is 2 jewels.
             // Jewel index i corresponds to Byte index i/2.
             // If app.vm.pc == i / 2, this is the Active Instruction.
             // Wait, PC points to the NEXT instruction byte to read.
             // When executing, we read at PC.
             if i / 2 == app.vm.pc {
                 is_active = true;
             }
        }

        if is_active {
            draw_circle(pos.x, pos.y, jewel_size * 1.5, WHITE); // Glow
        }

        match jewel.shape {
            Shape::Circle => draw_circle(pos.x, pos.y, jewel_size, color),
            Shape::Square => draw_rectangle(pos.x - jewel_size, pos.y - jewel_size, jewel_size * 2.0, jewel_size * 2.0, color),
            Shape::Triangle => draw_poly(pos.x, pos.y, 3, jewel_size, theta.to_degrees(), color),
            Shape::Diamond => draw_poly(pos.x, pos.y, 4, jewel_size, theta.to_degrees() + 45.0, color),
            Shape::Hexagon => draw_poly(pos.x, pos.y, 6, jewel_size, theta.to_degrees(), color),
            Shape::Star => draw_poly(pos.x, pos.y, 5, jewel_size, theta.to_degrees() + 18.0, color), // Star via poly? Just pentagon for now
            Shape::Pentagram => draw_poly(pos.x, pos.y, 5, jewel_size, theta.to_degrees(), color),
            Shape::Octagon => draw_poly(pos.x, pos.y, 8, jewel_size, theta.to_degrees(), color),
        }

        // Advance spiral
        // Circumference at radius r is 2 * pi * r.
        // We advance by jewel_size * 2.0 (padding).
        // Fraction of circumference = (jewel_size * 2.0) / (2 * pi * r)
        // Delta theta = (jewel_size * 1.5) / r

        theta += (jewel_size * 1.8) / radius;
        // Simple Archimedean spiral: r = a + b * theta
        radius = 20.0 + 5.0 * theta;
    }

    // Draw "Read Head" Line to Center
    if let AppState::Running = app.state {
        // Calculate pos of current PC
        // Re-simulate loop? Expensive.
        // Store positions?
        // Or just draw line from center to... where?
        // Let's just draw a line to the *active* jewels.
        // (Handled by highlight above)
    }
}
