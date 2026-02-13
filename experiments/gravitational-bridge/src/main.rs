mod audio;
mod physics;

use crate::audio::AudioEngine;
use crate::physics::{update, Body, G};
use macroquad::prelude::*;
use std::f32::consts::PI;

const STAR_MASS: f32 = 50000.0;
const PLANET_MASS: f32 = 1000.0;
const BRIDGE_THRESHOLD: f32 = 250.0;
const BRIDGE_BREAK_DIST: f32 = 450.0;
const BRIDGE_STRENGTH: f32 = 0.5;

#[derive(Clone, Copy, PartialEq)]
enum AntState {
    Foraging,
    Bridging,
}

struct Ant {
    planet_idx: usize,
    angle: f32, // Angle on planet
    speed: f32,
    state: AntState,
    bridge_idx: Option<usize>, // If bridging, which bridge index
    bridge_progress: f32,      // 0.0 to 1.0 along bridge
}

struct Bridge {
    planet_a: usize,
    planet_b: usize,
}

#[macroquad::main("Gravitational Bridge")]
async fn main() {
    let audio = AudioEngine::new().await;
    let mut bodies = Vec::new();
    let mut ants = Vec::new();
    let mut bridges: Vec<Bridge> = Vec::new();

    // Create Star
    bodies.push(Body::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        STAR_MASS,
        30.0,
        YELLOW,
    ));

    // Create Initial Planets
    let colors = [RED, GREEN, BLUE, PURPLE, ORANGE, SKYBLUE];
    for i in 0..5 {
        let angle = i as f32 * 2.0 * PI / 5.0;
        let dist = 300.0 + rand::gen_range(-50.0, 50.0);
        let pos = Vec2::new(angle.cos() * dist, angle.sin() * dist);

        let v_mag = (G * STAR_MASS / dist).sqrt();
        let v_dir = Vec2::new(-pos.y, pos.x).normalize();
        let vel = v_dir * v_mag;

        bodies.push(Body::new(
            pos,
            vel,
            PLANET_MASS,
            15.0,
            colors[i % colors.len()],
        ));

        // Spawn Ants
        for _ in 0..20 {
            ants.push(Ant {
                planet_idx: i + 1,
                angle: rand::gen_range(0.0, 2.0 * PI),
                speed: rand::gen_range(0.5, 2.0)
                    * (if rand::gen_range(0, 2) == 0 {
                        1.0
                    } else {
                        -1.0
                    }),
                state: AntState::Foraging,
                bridge_idx: None,
                bridge_progress: 0.0,
            });
        }
    }

    loop {
        let dt = get_frame_time().min(0.05);

        // Input: Spawn Planet
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
            // Mouse is in screen coords (0,0 top left). Camera is centered.
            // We need to inverse transform.
            // Simplified: Assume camera zoom 1.0 effectively for mouse logic relative to center
            // Actually let's just use screen center offset
            let zoom = 1.0;
            let pos = (Vec2::new(mpos.0, mpos.1) - screen_center) / zoom;

            // Just spawn static for now or calculate orbital vel
            let dist = pos.length();
            let v_mag = if dist > 10.0 {
                (G * STAR_MASS / dist).sqrt()
            } else {
                0.0
            };
            let v_dir = if dist > 10.0 {
                Vec2::new(-pos.y, pos.x).normalize()
            } else {
                Vec2::ZERO
            };
            let vel = v_dir * v_mag;

            bodies.push(Body::new(pos, vel, PLANET_MASS, 15.0, WHITE));

            let new_idx = bodies.len() - 1;
            for _ in 0..15 {
                ants.push(Ant {
                    planet_idx: new_idx,
                    angle: rand::gen_range(0.0, 2.0 * PI),
                    speed: rand::gen_range(0.5, 2.0),
                    state: AntState::Foraging,
                    bridge_idx: None,
                    bridge_progress: 0.0,
                });
            }
        }

        // 1. Physics Update (Planets)
        // Apply Bridge Forces first
        let mut bridge_forces = vec![Vec2::ZERO; bodies.len()];
        for bridge in &bridges {
            let p_a = bodies[bridge.planet_a].pos;
            let p_b = bodies[bridge.planet_b].pos;
            let dist = p_a.distance(p_b);
            if dist > 0.1 {
                let dir = (p_b - p_a).normalize();
                // Pull together
                let f = dir * BRIDGE_STRENGTH * 10.0;
                bridge_forces[bridge.planet_a] += f;
                bridge_forces[bridge.planet_b] -= f;
            }
        }

        for (i, f) in bridge_forces.iter().enumerate() {
            if i > 0 {
                // Don't move star
                bodies[i].vel += *f * dt; // F=ma, assuming m=1 for simplicity or scale force
            }
        }

        update(&mut bodies, dt);

        // 2. Logic Update (Bridges & Ants)

        // Check for broken bridges
        let mut broken_indices = Vec::new();
        for (i, bridge) in bridges.iter().enumerate() {
            let dist = bodies[bridge.planet_a]
                .pos
                .distance(bodies[bridge.planet_b].pos);
            if dist > BRIDGE_BREAK_DIST {
                broken_indices.push(i);
                audio.play_freq(100.0 + rand::gen_range(0.0, 50.0));
            }
        }

        // Remove broken bridges (reverse to keep indices valid)
        for &idx in broken_indices.iter().rev() {
            bridges.remove(idx);

            // Reset ants on this bridge
            for ant in ants.iter_mut() {
                if ant.state == AntState::Bridging {
                    if ant.bridge_idx == Some(idx) {
                        ant.state = AntState::Foraging;
                        ant.bridge_idx = None;
                        // Keep planet_idx (they fall back to one of the planets, maybe randomly based on progress)
                        // Simple: already have planet_idx
                    } else if let Some(b_idx) = ant.bridge_idx {
                        if b_idx > idx {
                            ant.bridge_idx = Some(b_idx - 1);
                        }
                    }
                }
            }
        }

        // Form new bridges
        // Only check if we don't have too many?
        for i in 1..bodies.len() {
            for j in (i + 1)..bodies.len() {
                let dist = bodies[i].pos.distance(bodies[j].pos);
                if dist < BRIDGE_THRESHOLD {
                    // Check if exists
                    if !bridges.iter().any(|b| {
                        (b.planet_a == i && b.planet_b == j) || (b.planet_a == j && b.planet_b == i)
                    }) {
                        // Create
                        let bridge_idx = bridges.len();
                        bridges.push(Bridge {
                            planet_a: i,
                            planet_b: j,
                        });
                        audio.play_freq(400.0 + dist);

                        // Recruit ants
                        let mut recruited = 0;
                        for ant in ants.iter_mut() {
                            if ant.state == AntState::Foraging
                                && (ant.planet_idx == i || ant.planet_idx == j)
                            {
                                ant.state = AntState::Bridging;
                                ant.bridge_idx = Some(bridge_idx);
                                ant.bridge_progress = if ant.planet_idx == i { 0.0 } else { 1.0 };
                                recruited += 1;
                                if recruited > 5 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Update Ants
        for ant in ants.iter_mut() {
            match ant.state {
                AntState::Foraging => {
                    ant.angle += ant.speed * dt;
                }
                AntState::Bridging => {
                    if let Some(b_idx) = ant.bridge_idx {
                        if b_idx < bridges.len() {
                            let bridge = &bridges[b_idx];
                            // Move along bridge
                            // speed dictates direction?
                            let direction = if ant.planet_idx == bridge.planet_a {
                                1.0
                            } else {
                                -1.0
                            };
                            ant.bridge_progress += ant.speed * 0.1 * direction * dt;

                            // Reached other side?
                            if ant.bridge_progress > 1.0 {
                                ant.planet_idx = bridge.planet_b;
                                ant.state = AntState::Foraging;
                                ant.bridge_idx = None;
                            } else if ant.bridge_progress < 0.0 {
                                ant.planet_idx = bridge.planet_a;
                                ant.state = AntState::Foraging;
                                ant.bridge_idx = None;
                            }
                        } else {
                            // Bridge gone? Should have been handled.
                            ant.state = AntState::Foraging;
                        }
                    }
                }
            }
        }

        // 3. Render
        clear_background(BLACK);

        // Camera setup
        set_camera(&Camera2D {
            target: Vec2::ZERO,
            zoom: Vec2::new(1.0 / (screen_width() * 0.5), 1.0 / (screen_height() * 0.5)),
            ..Default::default()
        });

        // Draw Bridges
        for bridge in &bridges {
            let p_a = bodies[bridge.planet_a].pos;
            let p_b = bodies[bridge.planet_b].pos;
            draw_line(p_a.x, p_a.y, p_b.x, p_b.y, 2.0, SKYBLUE);
        }

        // Draw Bodies
        for body in &bodies {
            draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
        }

        // Draw Ants
        for ant in &ants {
            match ant.state {
                AntState::Foraging => {
                    let body = &bodies[ant.planet_idx];
                    let r = body.radius + 2.0;
                    let pos = body.pos + Vec2::new(ant.angle.cos(), ant.angle.sin()) * r;
                    draw_circle(pos.x, pos.y, 2.0, WHITE);
                }
                AntState::Bridging => {
                    if let Some(b_idx) = ant.bridge_idx {
                        if b_idx < bridges.len() {
                            let bridge = &bridges[b_idx];
                            let p_a = bodies[bridge.planet_a].pos;
                            let p_b = bodies[bridge.planet_b].pos;
                            let pos = p_a + (p_b - p_a) * ant.bridge_progress;
                            draw_circle(pos.x, pos.y, 2.0, RED);
                        }
                    }
                }
            }
        }

        set_default_camera();
        draw_text("Left Click: Spawn Planet", 10.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("Bridges: {}", bridges.len()),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("Ants: {}", ants.len()), 10.0, 60.0, 20.0, WHITE);

        next_frame().await;
    }
}
