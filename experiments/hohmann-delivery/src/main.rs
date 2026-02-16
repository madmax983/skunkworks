use macroquad::prelude::*;

mod guidance;
mod physics;
mod ship;

use physics::{symplectic_euler, Body};
use ship::{Ship, ShipEvent, ShipState};

const G: f32 = 1.0;

#[macroquad::main("Hohmann Delivery")]
async fn main() {
    // Initialize bodies
    let mut bodies = vec![
        // Sun (Index 0)
        Body::new(vec2(0.0, 0.0), vec2(0.0, 0.0), 10000.0, 20.0, YELLOW),
        // Earth (Index 1) - r=200
        Body::new(
            vec2(200.0, 0.0),
            vec2(0.0, (G * 10000.0 / 200.0).sqrt()),
            100.0,
            8.0,
            BLUE,
        ),
        // Mars (Index 2) - r=300
        Body::new(
            vec2(0.0, 300.0),
            vec2(-(G * 10000.0 / 300.0).sqrt(), 0.0),
            80.0,
            6.0,
            RED,
        ),
    ];

    // Initialize Ship at Earth (Index 1)
    let mut ship = Ship::new(1, &bodies[1]);

    // Schedule transfer to Mars (Index 2)
    // GM = G * M_sun = 1.0 * 10000.0
    let gm = G * bodies[0].mass;
    ship.schedule_transfer(&bodies, 2, gm);

    let dt = 0.1;
    let mut time_scale = 1.0;

    loop {
        // Handle Input
        if is_key_down(KeyCode::Right) {
            time_scale *= 1.02;
        }
        if is_key_down(KeyCode::Left) {
            time_scale *= 0.98;
        }
        if is_key_pressed(KeyCode::Space) {
            time_scale = if time_scale == 0.0 { 1.0 } else { 0.0 };
        }

        // Auto-schedule return trip?
        if let ShipState::Arrived { body_index } = ship.state {
            if is_mouse_button_pressed(MouseButton::Left) {
                // Schedule to other planet
                let target = if body_index == 1 { 2 } else { 1 };
                ship.schedule_transfer(&bodies, target, gm);
            }
        }

        let step_dt = dt * time_scale;

        // Physics
        if time_scale > 0.0 {
            symplectic_euler(&mut bodies, step_dt, G);

            match ship.update_logic(&bodies, step_dt) {
                ShipEvent::Launched => {
                    // Could play sound
                }
                ShipEvent::Arrived(idx) => {
                    // Transfer resources
                    if idx == 2 {
                        // Delivered to Mars
                        bodies[idx].resources += 10;
                        ship.cargo = 0;
                    } else if idx == 1 {
                        // Back at Earth, reload
                        ship.cargo = 10;
                    }
                }
                ShipEvent::None => {}
            }

            ship.update_physics(&bodies, step_dt, gm);
        }

        // Render
        clear_background(BLACK);

        let zoom = 1.0 / 600.0;
        set_camera(&Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        });

        // Draw bodies
        for body in &bodies {
            draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
            // Draw resources
            if body.resources > 0 {
                draw_text_ex(
                    format!("{}", body.resources).as_str(),
                    body.pos.x + 15.0,
                    body.pos.y,
                    TextParams {
                        font_size: 20,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        // Draw Ship
        draw_circle(ship.pos.x, ship.pos.y, 3.0, ship.color);

        // Draw Transfer Orbit (Ellipse) if Waiting
        if let ShipState::WaitingForWindow {
            body_index,
            target_index,
            ..
        } = &ship.state
        {
            let pos1 = bodies[*body_index].pos;
            let r1 = pos1.length();
            let r2 = bodies[*target_index].pos.length(); // Approx target radius
            let a = (r1 + r2) / 2.0;
            let c = (r1 - a).abs();
            let b = (a * a - c * c).sqrt();

            // Center is at distance (r1 - a) from Sun along pos1 direction
            let angle = pos1.y.atan2(pos1.x);
            let center_dist = r1 - a;
            let center = pos1.normalize() * center_dist;

            // Draw ellipse
            draw_ellipse_orbit(center, a, b, angle, DARKGRAY);

            // Draw line to target (phase visualization)
            let target = &bodies[*target_index];
            draw_line(
                ship.pos.x,
                ship.pos.y,
                target.pos.x,
                target.pos.y,
                2.0,
                DARKGRAY,
            );
        }

        // Draw UI
        set_default_camera();
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            format!("Time Scale: {:.2}x", time_scale).as_str(),
            10.0,
            50.0,
            30.0,
            WHITE,
        );

        let status = match &ship.state {
            ShipState::Docked { body_index } => format!("Docked at Planet {}", body_index),
            ShipState::WaitingForWindow { .. } => "Waiting for Window".to_string(),
            ShipState::InTransit { .. } => "In Transit".to_string(),
            ShipState::Arrived { body_index } => format!("Arrived at Planet {}", body_index),
        };
        draw_text(
            format!("Ship Status: {}", status).as_str(),
            10.0,
            80.0,
            30.0,
            WHITE,
        );
        draw_text(
            format!("Fuel: {:.1}", ship.fuel).as_str(),
            10.0,
            110.0,
            30.0,
            WHITE,
        );

        if let ShipState::WaitingForWindow {
            body_index,
            target_index,
            plan,
        } = &ship.state
        {
            let pos1 = bodies[*body_index].pos;
            let pos2 = bodies[*target_index].pos;
            let angle1 = pos1.y.atan2(pos1.x);
            let angle2 = pos2.y.atan2(pos2.x);
            let mut current_phase = angle2 - angle1;
            while current_phase > std::f32::consts::PI {
                current_phase -= 2.0 * std::f32::consts::PI;
            }
            while current_phase <= -std::f32::consts::PI {
                current_phase += 2.0 * std::f32::consts::PI;
            }

            draw_text(
                format!("Phase: {:.2} rad", current_phase).as_str(),
                10.0,
                140.0,
                30.0,
                LIGHTGRAY,
            );
            draw_text(
                format!("Target: {:.2} rad", plan.phase_angle_required).as_str(),
                10.0,
                170.0,
                30.0,
                LIGHTGRAY,
            );
        }

        if let ShipState::Arrived { .. } = ship.state {
            draw_text("Click to Launch Return Trip", 10.0, 200.0, 30.0, GREEN);
        }

        next_frame().await
    }
}

fn draw_ellipse_orbit(center: Vec2, a: f32, b: f32, rotation: f32, color: Color) {
    let steps = 64;
    for i in 0..steps {
        let t1 = (i as f32) * 2.0 * std::f32::consts::PI / (steps as f32);
        let t2 = ((i + 1) as f32) * 2.0 * std::f32::consts::PI / (steps as f32);

        let p1 =
            center + vec2(a * t1.cos(), b * t1.sin()).rotate(vec2(rotation.cos(), rotation.sin()));
        let p2 =
            center + vec2(a * t2.cos(), b * t2.sin()).rotate(vec2(rotation.cos(), rotation.sin()));

        draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, color);
    }
}
