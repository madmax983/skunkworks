use ::rand::Rng;
use macroquad::prelude::*;
use physics_pbd::PbdSystem;
use git2::Repository;


struct Node {
    path: String,
    entropy: f32, // 0.0 (healthy) to 1.0 (rotten)
    particle_idx: usize,
}

struct TissueGraph {
    system: PbdSystem,
    nodes: Vec<Node>,
    links: Vec<(usize, usize)>, // Indices into `nodes`
}

impl TissueGraph {
    fn new(repo_path: &str) -> Self {
        let mut system = PbdSystem::new();
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        // Let's actually parse real git data using git2
        let repo = match Repository::discover(repo_path) {
            Ok(r) => r,
            Err(_) => {
                println!("Failed to find git repo, falling back to mock graph");
                return Self::mock();
            }
        };

        // We will just do a simple scan of the HEAD tree
        let head = match repo.head().and_then(|h| h.peel_to_tree()) {
            Ok(t) => t,
            Err(_) => return Self::mock(),
        };

        let mut files = Vec::new();
        head.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if let Some(name) = entry.name() {
                if entry.kind() == Some(git2::ObjectType::Blob) {
                    files.push(format!("{}{}", root, name));
                }
            }
            git2::TreeWalkResult::Ok
        }).unwrap_or(());

        // Limit to max 100 files for physics performance
        let num_nodes = files.len().min(100);
        let files = &files[0..num_nodes];

        if num_nodes == 0 {
            return Self::mock();
        }

        let radius = 300.0;
        let center = vec3(400.0, 400.0, 0.0);

        for (i, file) in files.iter().enumerate() {
            let angle = (i as f32 / num_nodes as f32) * std::f32::consts::PI * 2.0;
            let pos = center + vec3(angle.cos() * radius, angle.sin() * radius, 0.0);

            let p_idx = system.add_particle(pos, 1.0);

            nodes.push(Node {
                path: file.clone(),
                entropy: 0.0,
                particle_idx: p_idx,
            });
        }

        // Create links based on directory similarity
        for i in 0..num_nodes {
            let next = (i + 1) % num_nodes;
            links.push((i, next));

            let dist1 = system.particles[nodes[i].particle_idx].pos.distance(system.particles[nodes[next].particle_idx].pos);
            system.add_distance_constraint(nodes[i].particle_idx, nodes[next].particle_idx, dist1);

            // Connect files in the same directory
            for j in (i + 1)..num_nodes {
                let dir_i = nodes[i].path.split('/').next().unwrap_or("");
                let dir_j = nodes[j].path.split('/').next().unwrap_or("");
                if dir_i == dir_j && !dir_i.is_empty() && i != j && ::rand::thread_rng().gen_bool(0.3) {
                    links.push((i, j));
                    let dist = system.particles[nodes[i].particle_idx].pos.distance(system.particles[nodes[j].particle_idx].pos);
                    system.add_distance_constraint(nodes[i].particle_idx, nodes[j].particle_idx, dist);
                }
            }
        }

        Self { system, nodes, links }
    }

    fn mock() -> Self {
        let mut system = PbdSystem::new();
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        let num_nodes = 30;
        let radius = 200.0;
        let center = vec3(400.0, 400.0, 0.0);

        for i in 0..num_nodes {
            let angle = (i as f32 / num_nodes as f32) * std::f32::consts::PI * 2.0;
            let pos = center + vec3(angle.cos() * radius, angle.sin() * radius, 0.0);

            let p_idx = system.add_particle(pos, 1.0);

            nodes.push(Node {
                path: format!("file_{}.rs", i),
                entropy: 0.0,
                particle_idx: p_idx,
            });
        }

        for i in 0..num_nodes {
            let next = (i + 1) % num_nodes;
            links.push((i, next));

            let cross = (i + num_nodes / 2) % num_nodes;
            links.push((i, cross));

            let dist1 = system.particles[nodes[i].particle_idx].pos.distance(system.particles[nodes[next].particle_idx].pos);
            system.add_distance_constraint(nodes[i].particle_idx, nodes[next].particle_idx, dist1);

            let dist2 = system.particles[nodes[i].particle_idx].pos.distance(system.particles[nodes[cross].particle_idx].pos);
            system.add_distance_constraint(nodes[i].particle_idx, nodes[cross].particle_idx, dist2);
        }

        Self { system, nodes, links }
    }

    fn update(&mut self, dt: f32) {
        let mut rng = ::rand::thread_rng();

        for node in &mut self.nodes {
            if rng.gen_bool(0.01) { // Slowly rot
                node.entropy = (node.entropy + 0.05).min(1.0);
            }
            if rng.gen_bool(0.005) { // Maintenance
                node.entropy = 0.0;
            }
        }

        for node in &self.nodes {
            if node.entropy > 0.0 {
                let force = vec3(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    0.0
                ) * node.entropy * 200.0; // Violent shaking for rotting tissue

                self.system.particles[node.particle_idx].vel += force * dt;
            }
        }

        let center = vec3(400.0, 400.0, 0.0);
        for p in &mut self.system.particles {
            let dir = center - p.pos;
            if dir.length() > 20.0 {
                // Return to center
                p.vel += dir.normalize() * 50.0 * dt;
            }
            // Add some damping
            p.vel *= 0.95;
        }

        self.system.step(dt, 5);
    }

    fn draw(&self) {
        clear_background(BLACK);

        for &(i, j) in &self.links {
            let p1 = self.system.particles[self.nodes[i].particle_idx].pos;
            let p2 = self.system.particles[self.nodes[j].particle_idx].pos;

            let avg_entropy = (self.nodes[i].entropy + self.nodes[j].entropy) / 2.0;
            let color = if avg_entropy > 0.5 {
                Color::new(0.6, 0.1, 0.1, 0.6)
            } else {
                Color::new(0.2, 0.6, 0.2, 0.4)
            };

            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, color);
        }

        for node in &self.nodes {
            let pos = self.system.particles[node.particle_idx].pos;

            let color = Color::new(node.entropy, 1.0 - node.entropy, 0.0, 1.0);
            // Tissue swells!
            let radius = 6.0 + (node.entropy * 15.0);

            draw_circle(pos.x, pos.y, radius, color);

            // Only draw text if it's rotting to reduce visual noise, or draw small
            if node.entropy > 0.3 {
                draw_text(&node.path, pos.x + radius + 2.0, pos.y, 16.0, WHITE);
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Mnem Tissue".to_string(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let repo_path = if args.len() > 1 { &args[1] } else { "." };

    let mut tissue = TissueGraph::new(repo_path);

    loop {
        tissue.update(get_frame_time().min(0.1));
        tissue.draw();
        next_frame().await;
    }
}
