use macroquad::prelude::*;
use ::rand::Rng;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

// --- Vec4 Math (from tesseract-ops) ---

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self {
        Self {
            x: self.x * sx,
            y: self.y * sy,
            z: self.z * sz,
            w: self.w * sw,
        }
    }

    pub fn rotate_xw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x * c - self.w * s,
            y: self.y,
            z: self.z,
            w: self.x * s + self.w * c,
        }
    }

    pub fn rotate_yw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y * c - self.w * s,
            z: self.z,
            w: self.y * s + self.w * c,
        }
    }

    pub fn rotate_zw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y,
            z: self.z * c - self.w * s,
            w: self.z * s + self.w * c,
        }
    }

    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        let w_dist = camera_w - self.w;
        let scale = 2.0 / w_dist.max(0.1);
        vec3(self.x * scale, self.y * scale, self.z * scale)
    }

    pub fn distance(&self, other: &Vec4) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dw = self.w - other.w;
        (dx * dx + dy * dy + dz * dz + dw * dw).sqrt()
    }
}

// --- System Monitor (from tesseract-ops) ---

struct SystemMonitor {
    sys: System,
    last_update: f64,
    cpu_usage: f32,
    mem_usage: f32,
    swap_usage: f32,
    load_avg: f32,
    target_cpu: f32,
    target_mem: f32,
    target_swap: f32,
    target_load: f32,
}

impl SystemMonitor {
    fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            last_update: 0.0,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            swap_usage: 0.0,
            load_avg: 0.0,
            target_cpu: 0.0,
            target_mem: 0.0,
            target_swap: 0.0,
            target_load: 0.0,
        }
    }

    fn update(&mut self) {
        let now = get_time();
        if now - self.last_update > 1.0 {
            self.sys.refresh_cpu();
            self.sys.refresh_memory();
            self.last_update = now;

            self.target_cpu = self.sys.global_cpu_info().cpu_usage() / 100.0;
            let total_mem = self.sys.total_memory() as f32;
            let used_mem = self.sys.used_memory() as f32;
            self.target_mem = if total_mem > 0.0 { used_mem / total_mem } else { 0.0 };

            let total_swap = self.sys.total_swap() as f32;
            let used_swap = self.sys.used_swap() as f32;
            self.target_swap = if total_swap > 0.0 { used_swap / total_swap } else { 0.0 };

            let load = System::load_average();
            self.target_load = (load.one as f32 / 4.0).clamp(0.0, 1.0);
        }

        let dt = get_frame_time();
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let speed = 2.0 * dt;

        self.cpu_usage = lerp(self.cpu_usage, self.target_cpu, speed);
        self.mem_usage = lerp(self.mem_usage, self.target_mem, speed);
        self.swap_usage = lerp(self.swap_usage, self.target_swap, speed);
        self.load_avg = lerp(self.load_avg, self.target_load, speed);
    }
}

// --- Hyper Bridge Logic ---

#[derive(Clone, Copy, PartialEq)]
enum EdgeType {
    Base,
    Bridge,
}

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    ty: EdgeType,
    age: f32,
}

struct Ant {
    edge_idx: usize,
    progress: f32, // 0.0 to 1.0
    direction: f32, // 1.0 or -1.0
    carrying: bool,
}

struct World {
    vertices: Vec<Vec4>,
    edges: Vec<Edge>,
    ants: Vec<Ant>,
    monitor: SystemMonitor,
}

impl World {
    fn new() -> Self {
        let (verts, base_edges) = generate_tesseract_base();
        let mut edges = Vec::new();
        for (u, v) in base_edges {
            edges.push(Edge { u, v, ty: EdgeType::Base, age: 0.0 });
        }

        // Spawn ants
        let mut ants = Vec::new();
        for _ in 0..100 {
            ants.push(Ant {
                edge_idx: ::rand::thread_rng().gen_range(0..edges.len()),
                progress: ::rand::thread_rng().gen_range(0.0..1.0),
                direction: if ::rand::thread_rng().gen_bool(0.5) { 1.0 } else { -1.0 },
                carrying: false,
            });
        }

        Self {
            vertices: verts,
            edges,
            ants,
            monitor: SystemMonitor::new(),
        }
    }

    fn update(&mut self) {
        self.monitor.update();
        let dt = get_frame_time();

        // 1. Tesseract Distortion Parameters
        let sx = 1.0 + self.monitor.cpu_usage * 0.5;
        let sy = 1.0 + self.monitor.mem_usage * 0.5;
        let sz = 1.0 + self.monitor.swap_usage * 0.5;
        let sw = 1.0 + (get_time() as f32 * (1.0 + self.monitor.load_avg * 5.0)).sin() * 0.1;

        // Helper to get current projected position of a vertex
        // Note: We don't modify self.vertices in place because they are the "Base" shape (usually -1 to 1).
        // But for distance checks, we need the distorted shape.
        let get_distorted_pos = |idx: usize| -> Vec4 {
            let v = self.vertices[idx];
             v.scale_dim(sx, sy, sz, 1.0 + sw)
        };

        // 2. Update Ants
        let mut new_bridges = Vec::new();
        let mut ants_to_respawn = Vec::new();

        for (i, ant) in self.ants.iter_mut().enumerate() {
            // Check if edge exists (it might have decayed)
            if ant.edge_idx >= self.edges.len() {
                // Respawn
                ants_to_respawn.push(i);
                continue;
            }

            // Move
            let speed = 0.5 * (1.0 + self.monitor.load_avg * 2.0); // Move faster under load
            ant.progress += ant.direction * speed * dt;

            // Check bounds
            if ant.progress >= 1.0 || ant.progress <= 0.0 {
                // Reached a node
                let current_edge = &self.edges[ant.edge_idx];
                let current_node = if ant.progress >= 1.0 { current_edge.v } else { current_edge.u };

                // Decide next move
                let mut connected_edges = Vec::new();
                for (ei, e) in self.edges.iter().enumerate() {
                    if e.u == current_node {
                        connected_edges.push((ei, 1.0)); // 1.0 means moving u -> v
                    } else if e.v == current_node {
                        connected_edges.push((ei, -1.0)); // -1.0 means moving v -> u
                    }
                }

                if connected_edges.is_empty() {
                    // Trapped? Respawn
                     ants_to_respawn.push(i);
                     continue;
                }

                // Bridge Building Logic
                // If system load is high, chance to build a bridge to a nearby (in 4D) but unconnected node
                let bridge_chance = 0.01 + self.monitor.load_avg * 0.05;
                let mut bridged = false;

                if ::rand::thread_rng().gen::<f32>() < bridge_chance {
                    let pos_a = get_distorted_pos(current_node);

                    // Find candidate
                    let mut best_candidate = None;
                    let mut min_dist = f32::MAX;

                    for (vi, _) in self.vertices.iter().enumerate() {
                        if vi == current_node { continue; }

                        // Check if already connected (simple check)
                        let mut connected = false;
                        for (ei, dir) in &connected_edges {
                            let e = &self.edges[*ei];
                            let neighbor = if *dir > 0.0 { e.v } else { e.u };
                            if neighbor == vi {
                                connected = true;
                                break;
                            }
                        }

                        if !connected {
                            let pos_b = get_distorted_pos(vi);
                            let d = pos_a.distance(&pos_b);
                            // Only bridge if reasonably close
                            if d < 2.5 && d < min_dist {
                                min_dist = d;
                                best_candidate = Some(vi);
                            }
                        }
                    }

                    if let Some(target) = best_candidate {
                        // Build Bridge!
                        new_bridges.push(Edge {
                            u: current_node,
                            v: target,
                            ty: EdgeType::Bridge,
                            age: 0.0,
                        });

                        // Ant takes the new bridge immediately
                        // The new bridge will be at self.edges.len() + offset
                        // We can't set it yet because we haven't pushed to self.edges
                        // So we just mark "bridged" and handle logic next frame?
                        // Or just pick a random existing edge for now.
                        bridged = true;
                    }
                }

                if !bridged {
                    let (next_edge, dir) = connected_edges[::rand::thread_rng().gen_range(0..connected_edges.len())];
                    ant.edge_idx = next_edge;
                    ant.direction = dir; // 1.0 or -1.0
                    ant.progress = if dir > 0.0 { 0.0 } else { 1.0 };
                } else {
                     // If bridged, the ant effectively waits/builds this frame.
                     // Next frame it will be at the node again and might pick the new bridge.
                     ant.progress = ant.progress.clamp(0.0, 1.0);
                     // Flip direction to stay at node?
                     // Just Clamp.
                }
            }
        }

        // Add new bridges
        self.edges.append(&mut new_bridges);

        // Respawn ants
        for i in ants_to_respawn {
            if !self.edges.is_empty() {
                self.ants[i].edge_idx = ::rand::thread_rng().gen_range(0..self.edges.len());
                self.ants[i].progress = ::rand::thread_rng().gen_range(0.0..1.0);
            }
        }

        // 3. Decay Bridges
        // Bridges decay over time. High load accelerates decay (stress).
        let decay_rate = dt * (0.1 + self.monitor.cpu_usage * 0.5);
        let max_age = 5.0; // Seconds

        let mut edges_to_keep = Vec::new();
        // Map old indices to new indices to update ants
        let mut index_map = vec![0; self.edges.len()];
        let mut current_idx = 0;

        for (i, edge) in self.edges.iter_mut().enumerate() {
            if edge.ty == EdgeType::Base {
                edges_to_keep.push(Edge { u: edge.u, v: edge.v, ty: edge.ty, age: 0.0 });
                index_map[i] = current_idx;
                current_idx += 1;
            } else {
                edge.age += decay_rate;
                // Also break if stretched too far?
                let pos_u = get_distorted_pos(edge.u);
                let pos_v = get_distorted_pos(edge.v);
                let dist = pos_u.distance(&pos_v);

                if edge.age < max_age && dist < 4.0 {
                    edges_to_keep.push(Edge { u: edge.u, v: edge.v, ty: edge.ty, age: edge.age });
                    index_map[i] = current_idx;
                    current_idx += 1;
                } else {
                    index_map[i] = usize::MAX; // Deleted
                }
            }
        }

        self.edges = edges_to_keep;

        // Fix ant indices
        for ant in &mut self.ants {
            if ant.edge_idx < index_map.len() {
                let new_idx = index_map[ant.edge_idx];
                if new_idx == usize::MAX {
                    // Edge deleted under ant's feet!
                    // Ant falls? Or respawns?
                    // Let's make it fall to a random base edge
                    ant.edge_idx = ::rand::thread_rng().gen_range(0..32.min(self.edges.len()));
                    ant.progress = 0.5;
                } else {
                    ant.edge_idx = new_idx;
                }
            }
        }
    }
}

fn generate_tesseract_base() -> (Vec<Vec4>, Vec<(usize, usize)>) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { 1.0 } else { -1.0 };
        let y = if i & 2 != 0 { 1.0 } else { -1.0 };
        let z = if i & 4 != 0 { 1.0 } else { -1.0 };
        let w = if i & 8 != 0 { 1.0 } else { -1.0 };
        verts.push(Vec4::new(x, y, z, w));
    }

    let mut edges = Vec::new();
    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                edges.push((i, j));
            }
        }
    }
    (verts, edges)
}

#[macroquad::main("Hyper Bridge")]
async fn main() {
    let mut world = World::new();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 6.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        world.update();
        let dt = get_frame_time();
        let monitor = &world.monitor;

        // Auto Rotate based on load
        let base_speed = 0.2;
        let speed_mult = 1.0 + monitor.load_avg * 2.0;
        angle_xw += dt * base_speed * speed_mult;
        angle_yw += dt * base_speed * 0.7 * speed_mult;
        angle_zw += dt * base_speed * 0.3;

        // Input Camera
        if is_key_down(KeyCode::Left) { cam_angle_y += 2.0 * dt; }
        if is_key_down(KeyCode::Right) { cam_angle_y -= 2.0 * dt; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 2.0 * dt; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 2.0 * dt; }
        if is_key_down(KeyCode::W) { cam_dist -= 5.0 * dt; }
        if is_key_down(KeyCode::S) { cam_dist += 5.0 * dt; }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Rendering Transform
        let sx = 1.0 + monitor.cpu_usage * 0.5;
        let sy = 1.0 + monitor.mem_usage * 0.5;
        let sz = 1.0 + monitor.swap_usage * 0.5;
        let sw = 1.0 + (get_time() as f32 * (1.0 + monitor.load_avg * 5.0)).sin() * 0.1;

        let transform = |v: Vec4| -> Vec3 {
            let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            v.project_to_3d(3.0)
        };

        // Pre-calculate projected vertices
        let projected_verts: Vec<Vec3> = world.vertices.iter().map(|v| transform(*v)).collect();

        // Draw Edges
        for edge in &world.edges {
            let p1 = projected_verts[edge.u];
            let p2 = projected_verts[edge.v];

            let color = match edge.ty {
                EdgeType::Base => {
                    let r = monitor.cpu_usage;
                    let g = 0.2;
                    let b = monitor.mem_usage;
                    Color::new(r, g, b, 0.4)
                }
                EdgeType::Bridge => {
                    // Bridges are Cyan, alpha fades with age
                    let alpha = (1.0 - edge.age / 5.0).clamp(0.0, 1.0);
                    Color::new(0.0, 1.0, 1.0, alpha)
                }
            };

            draw_line_3d(p1, p2, color);
        }

        // Draw Vertices
        for p in &projected_verts {
            let size = 0.05 + monitor.load_avg * 0.1;
            draw_sphere(*p, size, None, WHITE);
        }

        // Draw Ants
        for ant in &world.ants {
            if ant.edge_idx < world.edges.len() {
                let edge = &world.edges[ant.edge_idx];
                let p1 = projected_verts[edge.u];
                let p2 = projected_verts[edge.v];

                // Lerp
                let t = ant.progress;
                let pos = vec3(
                    p1.x + (p2.x - p1.x) * t,
                    p1.y + (p2.y - p1.y) * t,
                    p1.z + (p2.z - p1.z) * t,
                );

                draw_sphere(pos, 0.03, None, YELLOW);
            }
        }

        set_default_camera();
        draw_text("Hyper-Bridge", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Ants: {}", world.ants.len()), 10.0, 50.0, 20.0, YELLOW);
        draw_text(&format!("Bridges: {}", world.edges.len() - 32), 10.0, 70.0, 20.0, Color::new(0.0, 1.0, 1.0, 1.0));
        draw_text(&format!("CPU: {:.0}%", monitor.cpu_usage * 100.0), 10.0, 90.0, 20.0, RED);

        next_frame().await
    }
}
