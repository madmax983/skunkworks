use macroquad::prelude::*;

mod mechanism;
use mechanism::{Differential, Integrator};

const DISC_RADIUS: f32 = 60.0;
const GEAR_COLOR: Color = GOLD;
const SHAFT_COLOR: Color = LIGHTGRAY;

struct Machine {
    // Damped Harmonic Oscillator: y'' + c*y' + k*y = 0
    // y'' = -(c*y' + k*y)

    // Components
    int_1: Integrator,  // Computes y' from y''
    int_2: Integrator,  // Computes y from y'
    diff: Differential, // Sums c*y' and k*y

    // Parameters
    damping: f32,   // c
    stiffness: f32, // k

    // State
    t: f32,
    y: f32,
    y_prime: f32,
    y_double_prime: f32,

    // Visual State
    disc_angle: f32,

    // History
    history: Vec<(f32, f32)>,
    paused: bool,
}

impl Machine {
    fn new() -> Self {
        let mut m = Self {
            int_1: Integrator::new(),
            int_2: Integrator::new(),
            diff: Differential::new(),
            damping: 0.1,
            stiffness: 1.0,
            t: 0.0,
            y: 1.0,
            y_prime: 0.0,
            y_double_prime: -1.0, // Initial acceleration (-k*y)
            disc_angle: 0.0,
            history: Vec::new(),
            paused: false,
        };

        // Initialize Differential state based on initial conditions
        // y'' = -(k*y + c*y')
        // diff output = (k*y + c*y')/2
        // y=1, y'=0, k=1, c=0.1 -> sum=1 -> diff=0.5
        let initial_sum = m.stiffness * m.y + m.damping * m.y_prime;
        m.diff.output_angle = initial_sum / 2.0;

        m.update_mechanisms();
        m
    }

    fn update_mechanisms(&mut self) {
        // Set carriage positions based on current state
        // Int 1 integrates y''. Carriage = y''.
        self.int_1.carriage_pos = self.y_double_prime;

        // Int 2 integrates y'. Carriage = y'.
        self.int_2.carriage_pos = self.y_prime;
    }

    fn step(&mut self, dt: f32) {
        if self.paused {
            return;
        }

        // 1. Drive Input Shafts (Time)
        // Assume constant angular velocity for discs = 1.0 rad/s
        let disc_delta = dt;
        self.disc_angle += disc_delta;

        // 2. Update Integrators
        // Int 1 output change (dy')
        let dy_prime = self.int_1.update(disc_delta);

        // Int 2 output change (dy)
        let dy = self.int_2.update(disc_delta);

        // 3. Update State variables
        self.y_prime += dy_prime;
        self.y += dy;
        self.t += dt;

        // 4. Update Differential (Feedback Logic)
        // We need to compute the new y''.
        // Mechanism:
        // Shaft A: y (scaled by stiffness k) -> rotation delta is dy * k
        // Shaft B: y' (scaled by damping c) -> rotation delta is dy_prime * c
        // Diff Output: (dy*k + dy_prime*c) / 2

        // Note: dy and dy_prime are deltas (velocities * dt).
        // But the Differential usually sums Positions (Angles).
        // Here we sum Deltas to get Delta Output.
        // Or we sum Absolute Values?
        // Let's stick to the algebraic constraint: y'' = -(k*y + c*y').
        // The mechanical differential sums inputs continuously.

        // Let's treat the inputs to the differential as the values y and y' themselves.
        // But the Differential update takes 'deltas' in my implementation?
        // Check mechanism.rs: pub fn update(&mut self, delta_a: f32, delta_b: f32) -> f32
        // Yes, it returns output delta.
        // But y'' is a value, not a delta of something (well, delta of y').
        // Actually, in the mechanical integrator, the position of the carriage is driven by a shaft.
        // So we need the VALUE of the output shaft to set the carriage position.

        // Let's feed the Absolute Values into a "Position Differential" if we had one.
        // Or just use the deltas to update a virtual shaft for y''.

        let term_k_delta = dy * self.stiffness;
        let term_c_delta = dy_prime * self.damping;
        let _diff_delta = self.diff.update(term_k_delta, term_c_delta);

        // diff_delta is change in (k*y + c*y')/2.
        // We want the value -(k*y + c*y').
        // So we can accumulate diff_delta.

        // Let's reconstruct y'' from the accumulated differential output.
        // The differential's output_angle stores sum/2.
        let sum_halved = self.diff.output_angle;
        let sum = sum_halved * 2.0;

        // y'' = -sum
        self.y_double_prime = -sum;

        // 5. Update Carriages for next step
        self.update_mechanisms();

        // History
        if self.history.len() > 2000 {
            self.history.remove(0);
        }
        self.history.push((self.t, self.y));
    }

    fn draw(&self) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;

        // Layout
        let int1_pos = vec2(cx - 200.0, cy - 50.0);
        let int2_pos = vec2(cx + 200.0, cy - 50.0);
        let diff_pos = vec2(cx, cy + 100.0);

        // Draw Connections
        // Y'' -> Int 1 Carriage
        draw_line(
            diff_pos.x,
            diff_pos.y + 20.0,
            diff_pos.x,
            diff_pos.y + 60.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            diff_pos.x,
            diff_pos.y + 60.0,
            int1_pos.x,
            diff_pos.y + 60.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            int1_pos.x,
            diff_pos.y + 60.0,
            int1_pos.x,
            int1_pos.y + DISC_RADIUS + 10.0,
            2.0,
            SHAFT_COLOR,
        );

        // Int 1 Output (y') -> Int 2 Carriage AND Differential Input B
        let int1_out = vec2(int1_pos.x - 40.0, int1_pos.y - DISC_RADIUS - 10.0);
        // Path to Int 2 Carriage
        draw_line(
            int1_out.x,
            int1_out.y,
            int1_out.x,
            int1_out.y - 40.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            int1_out.x,
            int1_out.y - 40.0,
            int2_pos.x,
            int1_out.y - 40.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            int2_pos.x,
            int1_out.y - 40.0,
            int2_pos.x,
            int2_pos.y + DISC_RADIUS + 10.0,
            2.0,
            SHAFT_COLOR,
        );
        // Path to Differential B
        draw_line(
            int1_out.x,
            int1_out.y - 40.0,
            diff_pos.x + 30.0,
            int1_out.y - 40.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            diff_pos.x + 30.0,
            int1_out.y - 40.0,
            diff_pos.x + 30.0,
            diff_pos.y - 20.0,
            2.0,
            SHAFT_COLOR,
        );

        // Int 2 Output (y) -> Differential Input A
        let int2_out = vec2(int2_pos.x - 40.0, int2_pos.y - DISC_RADIUS - 10.0);
        draw_line(
            int2_out.x,
            int2_out.y,
            int2_out.x,
            int2_out.y - 60.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            int2_out.x,
            int2_out.y - 60.0,
            diff_pos.x - 30.0,
            int2_out.y - 60.0,
            2.0,
            SHAFT_COLOR,
        );
        draw_line(
            diff_pos.x - 30.0,
            int2_out.y - 60.0,
            diff_pos.x - 30.0,
            diff_pos.y - 20.0,
            2.0,
            SHAFT_COLOR,
        );

        // Draw Components
        self.draw_integrator(int1_pos, &self.int_1, "Int 1: y' = integral(y'')");
        self.draw_integrator(int2_pos, &self.int_2, "Int 2: y = integral(y')");
        self.draw_differential(diff_pos, "Diff: -(y + 0.1y')");

        // Draw Plot
        self.draw_plot(cx, cy + 250.0);

        // UI
        draw_text("MECHANICAL INTEGRATOR", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Damping (UP/DOWN): {:.2}", self.damping),
            20.0,
            60.0,
            20.0,
            YELLOW,
        );
        draw_text("Space to Pause", 20.0, 80.0, 20.0, GRAY);

        if self.paused {
            draw_text("PAUSED", cx - 50.0, cy, 40.0, RED);
        }
    }

    fn draw_integrator(&self, pos: Vec2, integrator: &Integrator, label: &str) {
        let x = pos.x;
        let y = pos.y;

        // Disc
        draw_circle(x, y, DISC_RADIUS, DARKGRAY);
        draw_circle_lines(x, y, DISC_RADIUS, 2.0, WHITE);

        // Rotation indicator
        let dx = self.disc_angle.cos() * DISC_RADIUS;
        let dy = self.disc_angle.sin() * DISC_RADIUS;
        draw_line(x, y, x + dx, y + dy, 2.0, LIGHTGRAY);

        // Carriage Rail
        draw_line(x - DISC_RADIUS, y, x + DISC_RADIUS, y, 1.0, GRAY);

        // Carriage
        let scale = 40.0;
        let cx_clamped = (x + integrator.carriage_pos * scale)
            .clamp(x - DISC_RADIUS + 5.0, x + DISC_RADIUS - 5.0);

        draw_rectangle(cx_clamped - 8.0, y - 12.0, 16.0, 24.0, RED);
        draw_circle(cx_clamped, y, 4.0, BLACK);

        // Output cylinder (friction wheel)
        // Positioned relative to carriage but fixed in y?
        // No, in Kelvin's integrator, the output cylinder rests on the disc and moves radially.
        // Or the disc moves under a fixed cylinder.
        // Let's assume the Carriage holds the Output Cylinder.
        // So the output cylinder IS at cx_clamped.

        // Visualize output rotation
        // We need the accumulated angle for this integrator to spin it visually.
        let out_angle = integrator.output_angle;
        let ox = cx_clamped;
        let oy = y; // On the disc

        // Draw a small wheel on the carriage
        let wheel_r = 10.0;
        draw_circle_lines(ox, oy, wheel_r, 2.0, GREEN);
        let wx = out_angle.cos() * wheel_r;
        let wy = out_angle.sin() * wheel_r;
        draw_line(ox, oy, ox + wx, oy + wy, 2.0, WHITE);

        draw_text(label, x - 60.0, y + DISC_RADIUS + 25.0, 15.0, WHITE);
    }

    fn draw_differential(&self, pos: Vec2, label: &str) {
        let x = pos.x;
        let y = pos.y;
        let w = 80.0;
        let h = 40.0;

        // Box
        draw_rectangle(x - w / 2.0, y - h / 2.0, w, h, GEAR_COLOR);
        draw_rectangle_lines(x - w / 2.0, y - h / 2.0, w, h, 2.0, WHITE);

        // Inputs
        draw_circle(x - 30.0, y - h / 2.0, 5.0, DARKGRAY); // Input A
        draw_circle(x + 30.0, y - h / 2.0, 5.0, DARKGRAY); // Input B

        // Output
        draw_circle(x, y + h / 2.0, 5.0, DARKGRAY); // Output

        // Internal gears (symbolic)
        draw_circle_lines(x, y, 15.0, 1.0, BLACK);
        draw_line(x - 15.0, y, x + 15.0, y, 1.0, BLACK);
        draw_line(x, y - 15.0, x, y + 15.0, 1.0, BLACK);

        draw_text(label, x - 40.0, y + h / 2.0 + 20.0, 15.0, WHITE);
    }

    fn draw_plot(&self, x: f32, y: f32) {
        let w = 500.0;
        let h = 150.0;
        let left = x - w / 2.0;
        let bottom = y + h / 2.0;
        let top = y - h / 2.0;

        draw_rectangle(left, top, w, h, Color::new(0.1, 0.1, 0.1, 1.0));
        draw_rectangle_lines(left, top, w, h, 2.0, WHITE);

        // Zero line
        let zero_y = y;
        draw_line(left, zero_y, left + w, zero_y, 1.0, GRAY);

        if self.history.len() < 2 {
            return;
        }

        let t_start = self.history.first().unwrap().0;
        let t_end = self.history.last().unwrap().0;
        let t_range = t_end - t_start;
        if t_range < 0.001 {
            return;
        }

        let prev_pt = self.history.first().unwrap();
        // Map y from [-2, 2] to [bottom, top]
        let map_y = |val: f32| -> f32 {
            let norm = (val + 2.0) / 4.0; // 0..1
            bottom - norm * h
        };

        let mut px = left;
        let mut py = map_y(prev_pt.1);

        for (t, val) in self.history.iter().skip(1) {
            let tx = left + ((t - t_start) / t_range) * w;
            let ty = map_y(*val);

            draw_line(px, py, tx, ty, 2.0, GREEN);
            px = tx;
            py = ty;
        }
    }
}

#[macroquad::main("Mechanical Integrator")]
async fn main() {
    let mut machine = Machine::new();

    loop {
        clear_background(BLACK);

        // Input
        if is_key_pressed(KeyCode::Space) {
            machine.paused = !machine.paused;
        }
        if is_key_down(KeyCode::Up) {
            machine.damping += 0.001;
        }
        if is_key_down(KeyCode::Down) {
            machine.damping -= 0.001;
            if machine.damping < 0.0 {
                machine.damping = 0.0;
            }
        }

        // Update
        // Step size logic
        let dt = 0.05; // Simulation speed
        for _ in 0..4 {
            // Multiple substeps for smoothness
            machine.step(dt / 4.0);
        }

        // Draw
        machine.draw();

        next_frame().await
    }
}
