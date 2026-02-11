use macroquad::prelude::*;

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: f32 = 12.0;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Terrain {
    Empty,
    Solid,
    Gap,
    Bridge, // Formed by ants
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum PacketState {
    Walking,
    Waiting,
    Bridging,
}

struct Packet {
    pos: Vec2,
    state: PacketState,
    load: f32, // 0.0 to 1.0, affects speed
    patience: f32, // Decreases while waiting
}

struct World {
    terrain: Vec<Terrain>,
    original_terrain: Vec<Terrain>, // To restore after bridge dissolves
    packets: Vec<Packet>,
    width: usize,
    height: usize,
    spawn_timer: f32,
    spawn_rate: f32,

    // Interaction
    bridge_usage: Vec<f32>, // Decay timer for bridge segments
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let mut terrain = vec![Terrain::Empty; width * height];
        let bridge_usage = vec![0.0; width * height];

        // Setup simple map: Solid ground with a Gap in the middle
        let mid_y = height / 2;

        for y in 0..height {
            for x in 0..width {
                // Ground
                if y >= mid_y - 2 && y <= mid_y + 2 {
                     terrain[y * width + x] = Terrain::Solid;
                }

                // Gap
                if x > width / 3 && x < 2 * width / 3 {
                    if terrain[y * width + x] == Terrain::Solid {
                         terrain[y * width + x] = Terrain::Gap;
                    }
                }
            }
        }

        Self {
            original_terrain: terrain.clone(),
            terrain,
            packets: Vec::new(),
            width,
            height,
            spawn_timer: 0.0,
            spawn_rate: 0.05, // Fast spawn
            bridge_usage,
        }
    }

    fn get_terrain(&self, x: i32, y: i32) -> Terrain {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return Terrain::Empty;
        }
        self.terrain[(y as usize) * self.width + (x as usize)]
    }

    fn set_terrain(&mut self, x: i32, y: i32, t: Terrain) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }
        let idx = (y as usize) * self.width + (x as usize);
        self.terrain[idx] = t;
        // If user sets terrain, update original too, unless it's a bridge forming automatically (which calls set_terrain too).
        // Wait, bridge formation calls set_terrain(Bridge).
        // We need to distinguish user action from logic.
        // For now, assume set_terrain is called for Bridge formation primarily.
        // If t is NOT Bridge, update original.
        if t != Terrain::Bridge {
            self.original_terrain[idx] = t;
        }
    }

    fn update(&mut self, dt: f32) {
        self.spawn_timer += dt;

        // Spawn packets
        if self.spawn_timer > self.spawn_rate {
            self.spawn_timer = 0.0;
            let start_y = (self.height / 2) as f32;
            self.packets.push(Packet {
                pos: vec2(2.0, start_y + rand::gen_range(-1.0, 1.0)),
                state: PacketState::Walking,
                load: rand::gen_range(0.1, 1.0),
                patience: 1.0,
            });
        }

        // Bridge Decay
        for i in 0..self.bridge_usage.len() {
            if self.bridge_usage[i] > 0.0 {
                self.bridge_usage[i] -= dt;
            }
        }

        let mut new_bridges = Vec::new();
        let mut dissolved_bridges = Vec::new();

        let target = vec2(self.width as f32 - 2.0, (self.height / 2) as f32);
        let width = self.width;
        let height = self.height;
        let terrain = &self.terrain;
        let bridge_usage = &mut self.bridge_usage;

        // Helper closure to avoid borrowing self
        let get_terrain = |x: i32, y: i32| -> Terrain {
            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                return Terrain::Empty;
            }
            terrain[(y as usize) * width + (x as usize)]
        };

        for packet in &mut self.packets {
            match packet.state {
                PacketState::Walking => {
                    let dir = (target - packet.pos).normalize_or_zero();
                    let speed = 20.0 * (1.5 - packet.load); // Heavier = Slower

                    // Simple steering: if blocked, try up or down
                    let move_vec = dir * speed * dt;
                    let next_pos = packet.pos + move_vec;

                    let gx = next_pos.x as i32;
                    let gy = next_pos.y as i32;
                    let t = get_terrain(gx, gy);

                    if t == Terrain::Solid || t == Terrain::Bridge {
                        packet.pos = next_pos;

                        // If walking on bridge, refresh usage
                        if t == Terrain::Bridge {
                             let idx = (gy as usize) * width + (gx as usize);
                             if idx < bridge_usage.len() {
                                 bridge_usage[idx] = 2.0; // Reset usage timer
                             }
                        }
                    } else if t == Terrain::Gap {
                        // Hit a gap
                        packet.state = PacketState::Waiting;
                        packet.patience = 0.5; // Short patience
                    } else {
                        // Empty space (falling?)
                        // For this simulation, they just stick to the lane height roughly
                        // But let's allow them to move if it's close to center
                        if (gy - (height as i32 / 2)).abs() < 10 {
                             // Treat empty as walkable-ish but maybe they fall if too far?
                             // No, let's keep it simple: Only walk on Solid/Bridge.
                             // But we need to allow them to "step onto" a bridge forming next to them.

                             // Look for neighbor bridge
                             let mut found = false;
                             for dy in -1..=1 {
                                 let ny = gy + dy;
                                 if get_terrain(gx, ny) == Terrain::Bridge {
                                     packet.pos.y = ny as f32;
                                     packet.pos.x = next_pos.x;
                                     found = true;
                                     break;
                                 }
                             }
                             if !found {
                                 packet.state = PacketState::Waiting;
                             }
                        }
                    }
                }
                PacketState::Waiting => {
                    packet.patience -= dt;

                    // If patience runs out, become a bridge
                    if packet.patience <= 0.0 {
                        // Check if we are in a valid spot to bridge (Gap or Empty near valid)
                        // Snap to grid
                        let gx = packet.pos.x.round() as i32;
                        let gy = packet.pos.y.round() as i32;

                        // Only bridge if we are actually in a gap or empty space that needs bridging
                        let t = get_terrain(gx, gy);
                        if t == Terrain::Gap || t == Terrain::Empty {
                             new_bridges.push((gx, gy));
                             packet.state = PacketState::Bridging;
                             packet.pos = vec2(gx as f32, gy as f32); // Snap

                             // Initial usage
                             let idx = (gy as usize) * width + (gx as usize);
                             if idx < bridge_usage.len() {
                                 bridge_usage[idx] = 5.0; // Initial grace period
                             }
                        } else {
                            // If we are waiting on solid ground, maybe try to move around?
                            // Jitter
                            packet.pos.y += rand::gen_range(-0.1, 0.1);
                            packet.state = PacketState::Walking; // Retry
                        }
                    }
                }
                PacketState::Bridging => {
                    let gx = packet.pos.x as i32;
                    let gy = packet.pos.y as i32;
                    let idx = (gy as usize) * width + (gx as usize);

                    // Check if usage is 0
                    if idx < bridge_usage.len() && bridge_usage[idx] <= 0.0 {
                        // Dissolve!
                        dissolved_bridges.push((gx, gy));
                        packet.state = PacketState::Walking; // Free!
                        packet.patience = 1.0; // Reset patience
                    }
                }
            }
        }

        for (bx, by) in new_bridges {
             self.set_terrain(bx, by, Terrain::Bridge);
        }

        for (bx, by) in dissolved_bridges {
            // Restore from original terrain
            let idx = (by as usize) * self.width + (bx as usize);
            if idx < self.original_terrain.len() {
                // We directly set terrain because we want to restore, not update original again
                // Actually set_terrain logic handles non-Bridge correctly
                // But here we want to restore specifically what was there.
                self.terrain[idx] = self.original_terrain[idx];
            } else {
                self.set_terrain(bx, by, Terrain::Empty);
            }
        }

        // Cleanup
        self.packets.retain(|p| p.pos.x < self.width as f32);
    }
}

#[macroquad::main("Bridge Balancer")]
async fn main() {
    let mut world = World::new(GRID_WIDTH, GRID_HEIGHT);

    loop {
        world.update(get_frame_time());

        if is_key_down(KeyCode::Space) {
            world.spawn_rate = 0.005; // Surge
        } else {
            world.spawn_rate = 0.05; // Normal
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / CELL_SIZE) as i32;
            let gy = (my / CELL_SIZE) as i32;
            world.set_terrain(gx, gy, Terrain::Gap);
        }
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let gx = (mx / CELL_SIZE) as i32;
            let gy = (my / CELL_SIZE) as i32;
            world.set_terrain(gx, gy, Terrain::Solid);
        }

        clear_background(BLACK);

        // Draw Terrain
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let t = world.get_terrain(x as i32, y as i32);
                match t {
                    Terrain::Solid => {
                        draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, GRAY);
                    },
                    Terrain::Gap => {
                         // Gap is dark
                    },
                    Terrain::Bridge => {
                        // Color based on usage?
                        let idx = y * GRID_WIDTH + x;
                        let usage = world.bridge_usage[idx];
                        let intensity = (usage / 2.0).min(1.0);
                        let color = Color::new(0.0, 0.5 + intensity * 0.5, 1.0 - intensity, 1.0);
                        draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
                    },
                    Terrain::Empty => {}
                };
            }
        }

        // Draw Packets
        for p in &world.packets {
            if p.state != PacketState::Bridging {
                if p.state == PacketState::Waiting {
                    // Glow for latency/congestion
                    draw_circle(p.pos.x * CELL_SIZE + CELL_SIZE/2.0, p.pos.y * CELL_SIZE + CELL_SIZE/2.0, 6.0, Color::new(1.0, 0.0, 0.0, 0.3));
                    draw_circle(p.pos.x * CELL_SIZE + CELL_SIZE/2.0, p.pos.y * CELL_SIZE + CELL_SIZE/2.0, 3.0, RED);
                } else {
                    draw_circle(p.pos.x * CELL_SIZE + CELL_SIZE/2.0, p.pos.y * CELL_SIZE + CELL_SIZE/2.0, 3.0, WHITE);
                }
            }
        }

        draw_text("Space: Surge Traffic | L-Click: Dig Gap | R-Click: Fill", 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Packets: {}", world.packets.len()), 10.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}
