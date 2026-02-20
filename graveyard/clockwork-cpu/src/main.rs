mod physics;
use physics::Simulation;
use macroquad::prelude::*;

const TEETH_COUNT: usize = 15;

#[macroquad::main("Clockwork CPU")]
async fn main() {
    let mut sim = Simulation::new();
    sim.teeth = TEETH_COUNT;

    // Oscilloscope buffer
    let mut history = Vec::new();
    let max_history = 500;

    loop {
        // Input
        if is_key_down(KeyCode::Up) {
            sim.voltage += 10.0;
        }
        if is_key_down(KeyCode::Down) {
            sim.voltage -= 10.0;
        }
        sim.voltage = sim.voltage.max(0.0); // No negative voltage

        // Physics Sub-stepping
        let dt = get_frame_time();
        // Limit dt to avoid spiral of death
        let dt = dt.min(0.05);

        let sub_steps = 20; // High resolution for stiff springs
        let sub_dt = dt / sub_steps as f32;

        for _ in 0..sub_steps {
            sim.step(sub_dt);
        }

        // Record history
        history.push(sim.verge.velocity);
        if history.len() > max_history {
            history.remove(0);
        }

        // Render
        clear_background(BLACK);

        // Use camera to center 0,0
        // Zoom: smaller is bigger? No, zoom is scale.
        // Screen width ~800. We want radius 50 to be visible.
        // Zoom 1.0 means coordinate 1.0 is edge? No.
        // Default coordinate system: [0, width], [0, height].
        // Camera coordinate system: [-1, 1] usually?
        // Let's use standard coordinates and translate manually or use Camera2D.
        // Camera2D zoom: (scale_x, scale_y).
        // If we want 1 unit = 1 pixel, zoom = (2/width, 2/height).
        // If we want 0,0 at center.

        let zoom = 1.0 / 300.0; // View range +/- 300 units
        let aspect = screen_width() / screen_height();

        set_camera(&Camera2D {
            zoom: vec2(zoom, -zoom * aspect), // Flip Y because physics usually Y-up, but screen Y-down.
            target: vec2(0.0, 0.0),
            ..Default::default()
        });

        draw_simulation(&sim);

        // Reset camera for UI
        set_default_camera();
        draw_ui(&sim, &history);

        next_frame().await
    }
}

fn draw_simulation(sim: &Simulation) {
    let radius = 100.0;

    // Draw Crown Wheel
    draw_circle_lines(0.0, 0.0, radius, 2.0, GOLD);
    draw_circle_lines(0.0, 0.0, radius * 0.9, 1.0, BROWN);

    // Draw Teeth
    let step = 2.0 * std::f32::consts::PI / sim.teeth as f32;
    for i in 0..sim.teeth {
        let theta = sim.crown.angle + i as f32 * step;

        // Tooth triangle
        let tooth_h = 20.0;
        // Base on circle
        let p1 = vec2(theta.cos() * radius, theta.sin() * radius);
        // Tip (angled forward slightly for saw shape)
        let tip_angle = theta + 0.1;
        let p2 = vec2(tip_angle.cos() * (radius + tooth_h), tip_angle.sin() * (radius + tooth_h));
        // Back base
        let back_angle = theta - 0.1;
        let p3 = vec2(back_angle.cos() * radius, back_angle.sin() * radius);

        draw_triangle_lines(p1, p2, p3, 2.0, GOLD);
    }

    // Draw Verge Axis (Vertical line visually? No, it's a point in this projection if top down)
    // Wait, if I'm projecting top down:
    // Crown is a circle.
    // Verge axis is a line across the circle (Diameter).
    // Pallets engage at top and bottom of circle.
    // The Foliot swings perpendicular to Verge Axis?
    // Verge Axis is vertical (Z). Foliot is horizontal (XY).
    // Crown Axis is Horizontal (X). Crown is Vertical (YZ).
    // My previous physics derivation assumed interaction at PI/2 and 3PI/2.
    // This implies looking at the Crown Wheel "Face on" (XY plane).
    // So the Verge Axis is a Vertical Line (Y axis).
    // The Foliot is a horizontal bar swinging in Z (depth)?
    // Or Foliot is attached to the Verge Axis and swings in the XY plane?
    // Let's visualize the Foliot as a bar crossing the Crown Wheel, swinging.

    let foliot_len = 250.0;
    let va = sim.verge.angle;

    // Draw Foliot Bar
    // Center at (0,0) (Verge Axis crosses Crown Center - rare but simple)
    // Let's offset it to be distinct.
    // Actually, in a Verge escapement, the verge is tangent.
    // But let's keep it centered for symmetry.

    let p1 = vec2(va.cos() * foliot_len, va.sin() * foliot_len);
    let p2 = vec2(-va.cos() * foliot_len, -va.sin() * foliot_len);

    draw_line(p1.x, p1.y, p2.x, p2.y, 8.0, LIGHTGRAY);
    draw_circle(p1.x, p1.y, 10.0, RED);
    draw_circle(p2.x, p2.y, 10.0, RED);
    draw_circle(0.0, 0.0, 15.0, LIGHTGRAY); // Pivot

    // Draw Pallets interaction points
    // Top Pallet Interaction (PI/2 -> (0, Radius))
    // Bottom Pallet Interaction (3PI/2 -> (0, -Radius))

    // Visualize Pallet State
    // Top Pallet Active: Verge > -0.2 && < Escape
    let escape_angle = 0.6;
    let top_active = sim.verge.angle > -0.2 && sim.verge.angle < escape_angle;
    let bot_active = sim.verge.angle < 0.2 && sim.verge.angle > -escape_angle;

    let top_pos = vec2(0.0, radius);
    let bot_pos = vec2(0.0, -radius);

    draw_circle(top_pos.x, top_pos.y, 12.0, if top_active { GREEN } else { DARKGRAY });
    draw_circle(bot_pos.x, bot_pos.y, 12.0, if bot_active { GREEN } else { DARKGRAY });

    // Draw "Sparks" or Force indicators?
    // Maybe later.
}

fn draw_ui(sim: &Simulation, history: &[f32]) {
    let font_size = 30.0;
    draw_text(&format!("Voltage: {:.0}", sim.voltage), 20.0, 40.0, font_size, WHITE);
    draw_text("Controls: UP/DOWN Arrows", 20.0, 80.0, 20.0, LIGHTGRAY);

    // Oscilloscope
    let w = screen_width();
    let h = 200.0;
    let base_y = screen_height() - h/2.0;

    // Background for scope
    draw_rectangle(0.0, screen_height() - h, w, h, Color::new(0.1, 0.1, 0.1, 1.0));
    draw_line(0.0, base_y, w, base_y, 1.0, GRAY);

    let step_x = w / history.len().max(1) as f32;
    let scale_y = 5.0;

    for i in 0..history.len().saturating_sub(1) {
        let x1 = i as f32 * step_x;
        let y1 = base_y - history[i] * scale_y;
        let x2 = (i + 1) as f32 * step_x;
        let y2 = base_y - history[i+1] * scale_y;

        draw_line(x1, y1, x2, y2, 2.0, GREEN);
    }

    draw_text("CPU CLOCK SIGNAL", w - 200.0, screen_height() - h + 30.0, 20.0, GREEN);
}
