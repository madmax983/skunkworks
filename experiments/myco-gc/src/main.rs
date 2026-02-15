use macroquad::prelude::*;
use std::collections::HashSet;

const WORLD_WIDTH: f32 = 1200.0;
const WORLD_HEIGHT: f32 = 800.0;
const CELL_SIZE: f32 = 10.0;
const COLS: usize = (WORLD_WIDTH / CELL_SIZE) as usize;
const ROWS: usize = (WORLD_HEIGHT / CELL_SIZE) as usize;

#[derive(Clone, Copy, PartialEq)]
enum ObjectState {
    Live,
    Dead,
    Rotting,
}

struct Object {
    #[allow(dead_code)]
    id: usize,
    pos: Vec2,
    radius: f32,
    refs: Vec<usize>,
    state: ObjectState,
    rot_level: f32,
}

struct Hypha {
    pos: Vec2,
    vel: Vec2,
    energy: f32,
    age: f32,
    active: bool,
}

struct World {
    objects: Vec<Object>,
    hyphae: Vec<Hypha>,
    roots: Vec<usize>,
    pheromone_grid: Vec<f32>,
    trail_grid: Vec<f32>,
    next_id: usize,
}

impl World {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
            hyphae: Vec::new(),
            roots: Vec::new(),
            pheromone_grid: vec![0.0; COLS * ROWS],
            trail_grid: vec![0.0; COLS * ROWS],
            next_id: 0,
        }
    }

    fn spawn_object(&mut self, pos: Vec2, is_root: bool) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.push(Object {
            id,
            pos,
            radius: rand::gen_range(5.0, 10.0),
            refs: Vec::new(),
            state: ObjectState::Live,
            rot_level: 0.0,
        });
        if is_root {
            self.roots.push(id);
        }
        id
    }

    fn add_reference(&mut self, from_idx: usize, to_idx: usize) {
        if from_idx < self.objects.len() && to_idx < self.objects.len()
            && !self.objects[from_idx].refs.contains(&to_idx) {
                self.objects[from_idx].refs.push(to_idx);
            }
    }

    fn mark_phase(&mut self) {
        let mut reachable = HashSet::new();
        let mut queue = Vec::new();

        for &root_idx in &self.roots {
            // Note: In a real GC we'd trace strictly by ID, but here indices are stable enough for prototype
            // provided we don't use swap_remove.
            if root_idx < self.objects.len() {
                // Simple bounds check
                queue.push(root_idx);
                reachable.insert(root_idx);
            }
        }

        while let Some(curr_idx) = queue.pop() {
            if curr_idx >= self.objects.len() {
                continue;
            }

            let refs = self.objects[curr_idx].refs.clone();
            for ref_idx in refs {
                if !reachable.contains(&ref_idx) && ref_idx < self.objects.len() {
                    reachable.insert(ref_idx);
                    queue.push(ref_idx);
                }
            }
        }

        for (i, obj) in self.objects.iter_mut().enumerate() {
            // Don't kill roots
            if self.roots.contains(&i) {
                obj.state = ObjectState::Live;
                obj.rot_level = 0.0;
                continue;
            }

            if reachable.contains(&i) {
                obj.state = ObjectState::Live;
                obj.rot_level = 0.0;
            } else if obj.state == ObjectState::Live {
                obj.state = ObjectState::Dead;
            }
        }
    }

    fn update_pheromones(&mut self) {
        for p in self.pheromone_grid.iter_mut() {
            *p *= 0.95;
        }
        for t in self.trail_grid.iter_mut() {
            *t *= 0.98;
        }

        for obj in &mut self.objects {
            if obj.state == ObjectState::Dead || obj.state == ObjectState::Rotting {
                obj.rot_level += 0.01;
                if obj.rot_level > 1.0 {
                    obj.state = ObjectState::Rotting;
                }

                if obj.state == ObjectState::Rotting {
                    let cx = (obj.pos.x / CELL_SIZE) as i32;
                    let cy = (obj.pos.y / CELL_SIZE) as i32;
                    for dy in -2..=2 {
                        for dx in -2..=2 {
                            let x = cx + dx;
                            let y = cy + dy;
                            if x >= 0 && x < COLS as i32 && y >= 0 && y < ROWS as i32 {
                                let idx = (y as usize) * COLS + (x as usize);
                                self.pheromone_grid[idx] =
                                    (self.pheromone_grid[idx] + 0.1).min(1.0);
                            }
                        }
                    }
                }
            }
        }
    }

    fn spawn_spore(&mut self, pos: Vec2) {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        self.hyphae.push(Hypha {
            pos,
            vel: Vec2::new(angle.cos(), angle.sin()) * 2.0,
            energy: 1.0,
            age: 0.0,
            active: true,
        });
    }

    fn update_fungus(&mut self) {
        let mut new_hyphae = Vec::new();

        {
            // Split borrows
            let pheromones = &self.pheromone_grid;
            let trail = &mut self.trail_grid;
            let objects = &mut self.objects;

            for hypha in self.hyphae.iter_mut() {
                if !hypha.active {
                    continue;
                }

                // 1. Sense
                let angle_step = 0.5;
                let sensor_dist = 15.0;
                let mut best_angle = 0.0;
                let mut max_val = -1.0;

                let angle = hypha.vel.y.atan2(hypha.vel.x);

                for i in -1..=1 {
                    let check_angle = angle + (i as f32) * angle_step;
                    let sensor_pos =
                        hypha.pos + Vec2::new(check_angle.cos(), check_angle.sin()) * sensor_dist;

                    let cx = (sensor_pos.x / CELL_SIZE) as i32;
                    let cy = (sensor_pos.y / CELL_SIZE) as i32;

                    if cx >= 0 && cx < COLS as i32 && cy >= 0 && cy < ROWS as i32 {
                        let idx = (cy as usize) * COLS + (cx as usize);
                        let val = pheromones[idx];
                        if val > max_val {
                            max_val = val;
                            best_angle = check_angle;
                        }
                    }
                }

                // 2. Move
                if max_val > 0.01 {
                    let desired_vel = Vec2::new(best_angle.cos(), best_angle.sin()) * 2.0;
                    hypha.vel = hypha.vel.lerp(desired_vel, 0.1);
                } else {
                    let wiggle = rand::gen_range(-0.2, 0.2);
                    let current_angle = hypha.vel.y.atan2(hypha.vel.x) + wiggle;
                    hypha.vel = Vec2::new(current_angle.cos(), current_angle.sin()) * 2.0;
                }

                hypha.pos += hypha.vel;
                hypha.energy -= 0.005;
                hypha.age += 1.0;

                // Bounds
                if hypha.pos.x < 0.0
                    || hypha.pos.x > WORLD_WIDTH
                    || hypha.pos.y < 0.0
                    || hypha.pos.y > WORLD_HEIGHT
                {
                    hypha.active = false;
                }
                if hypha.energy <= 0.0 {
                    hypha.active = false;
                }

                // 3. Trail
                let cx = (hypha.pos.x / CELL_SIZE) as usize;
                let cy = (hypha.pos.y / CELL_SIZE) as usize;
                if cx < COLS && cy < ROWS {
                    let idx = cy * COLS + cx;
                    trail[idx] = (trail[idx] + 0.5).min(2.0);
                }

                // 4. Interact (Eat Garbage)
                for obj in objects.iter_mut() {
                    if obj.state == ObjectState::Rotting
                        && hypha.pos.distance(obj.pos) < obj.radius + 5.0 {
                            obj.radius -= 0.5;
                            hypha.energy += 0.5;
                            if obj.radius <= 0.0 {
                                obj.state = ObjectState::Dead;
                                obj.pos = Vec2::new(-9999.0, -9999.0);
                            }

                            if rand::gen_range(0.0, 1.0) < 0.1 {
                                let branch_angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
                                new_hyphae.push(Hypha {
                                    pos: hypha.pos,
                                    vel: Vec2::new(branch_angle.cos(), branch_angle.sin()) * 2.0,
                                    energy: 0.8,
                                    age: 0.0,
                                    active: true,
                                });
                            }
                        }
                }
            }
        }

        self.hyphae.append(&mut new_hyphae);
        self.hyphae.retain(|h| h.active);
    }
}

#[macroquad::main("Myco-GC")]
async fn main() {
    let mut world = World::new();

    let root_pos = Vec2::new(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
    world.spawn_object(root_pos, true);

    for _ in 0..50 {
        world.spawn_spore(Vec2::new(
            rand::gen_range(0.0, WORLD_WIDTH),
            rand::gen_range(0.0, WORLD_HEIGHT),
        ));
    }

    loop {
        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let id = world.spawn_object(Vec2::new(mpos.0, mpos.1), false);
            // Connect to root or random
            world.add_reference(world.roots[0], id);
        }

        if is_key_pressed(KeyCode::Space) {
            // Spawn Cluster
            let center = Vec2::new(
                rand::gen_range(100.0, WORLD_WIDTH - 100.0),
                rand::gen_range(100.0, WORLD_HEIGHT - 100.0),
            );
            let mut ids = Vec::new();
            for _ in 0..10 {
                let offset = Vec2::new(rand::gen_range(-50.0, 50.0), rand::gen_range(-50.0, 50.0));
                ids.push(world.spawn_object(center + offset, false));
            }
            // Link them
            for i in 0..ids.len() {
                if i > 0 {
                    world.add_reference(ids[i - 1], ids[i]);
                }
            }
            // Link first to world root
            world.add_reference(world.roots[0], ids[0]);
        }

        if is_key_pressed(KeyCode::G) {
            // Create Garbage (Cut links)
            if world.objects.len() > 5 {
                // Cut connection from root to some child
                let _target = rand::gen_range(1, world.objects.len()); // skip root 0
                // Find who points to target and remove ref?
                // Easier: clear refs of a random node, making its children garbage
                let victim = rand::gen_range(1, world.objects.len());
                world.objects[victim].refs.clear();
            }
        }

        if is_key_pressed(KeyCode::R) {
            world = World::new();
            let root_pos = Vec2::new(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
            world.spawn_object(root_pos, true);
        }

        // Update
        world.mark_phase();
        world.update_pheromones();
        world.update_fungus();

        // Render
        clear_background(BLACK);

        // Draw Background Grid (Memory Addresses)
        for x in (0..WORLD_WIDTH as i32).step_by(100) {
            draw_line(
                x as f32,
                0.0,
                x as f32,
                WORLD_HEIGHT,
                1.0,
                Color::new(0.1, 0.1, 0.1, 1.0),
            );
        }
        for y in (0..WORLD_HEIGHT as i32).step_by(100) {
            draw_line(
                0.0,
                y as f32,
                WORLD_WIDTH,
                y as f32,
                1.0,
                Color::new(0.1, 0.1, 0.1, 1.0),
            );
        }

        // Trails
        for y in (0..ROWS).step_by(2) {
            for x in (0..COLS).step_by(2) {
                let idx = y * COLS + x;
                let val = world.trail_grid[idx];
                if val > 0.1 {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE * 2.0,
                        CELL_SIZE * 2.0,
                        Color::new(0.8, 0.8, 0.6, val.min(0.5)),
                    );
                }
            }
        }

        // References
        for obj in &world.objects {
            if obj.radius <= 0.0 {
                continue;
            }
            for &ref_idx in &obj.refs {
                if ref_idx < world.objects.len() {
                    let target = &world.objects[ref_idx];
                    if target.radius > 0.0 {
                        draw_line(
                            obj.pos.x,
                            obj.pos.y,
                            target.pos.x,
                            target.pos.y,
                            1.0,
                            DARKGRAY,
                        );
                    }
                }
            }
        }

        // Objects
        for obj in &world.objects {
            if obj.radius <= 0.0 {
                continue;
            }
            let color = match obj.state {
                ObjectState::Live => GREEN,
                ObjectState::Dead => BROWN,
                ObjectState::Rotting => RED,
            };
            draw_circle(obj.pos.x, obj.pos.y, obj.radius, color);
            if obj.state == ObjectState::Live {
                draw_circle_lines(obj.pos.x, obj.pos.y, obj.radius + 2.0, 1.0, LIME);
            }
        }

        // Hyphae
        for h in &world.hyphae {
            draw_circle(h.pos.x, h.pos.y, 1.0, WHITE);
        }

        draw_text(
            format!(
                "Objects: {}",
                world.objects.iter().filter(|o| o.radius > 0.0).count()
            )
            .as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Hyphae: {}", world.hyphae.len()).as_str(),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Space: Spawn Cluster | Click: Add Node | G: Create Garbage | R: Reset",
            10.0,
            WORLD_HEIGHT - 10.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
