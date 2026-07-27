use chimera_lang::prelude::*;
use macroquad::prelude::*;

const SERVER_RADIUS: f32 = 40.0;
const FIREWALL_RADIUS: f32 = 25.0;

#[derive(Clone)]
struct Firewall {
    x: f32,
    y: f32,
}

struct Packet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    dna: Dna,
    vm: ChimeraVM,
    alive: bool,
    age: f32,
    color: Color,
}

impl Packet {
    fn new(x: f32, y: f32, dna: Dna) -> Self {
        let vm = ChimeraVM::new(dna.clone());
        Self {
            x,
            y,
            vx: rand::gen_range(-1.0, 1.0),
            vy: rand::gen_range(-1.0, 1.0),
            dna,
            vm,
            alive: true,
            age: 0.0,
            color: GREEN,
        }
    }

    fn update(&mut self, dt: f32, tx: f32, ty: f32, firewalls: &[Firewall]) {
        if !self.alive {
            return;
        }

        self.age += dt;

        // Provide sensor input to VM
        // Vector to target
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.1);
        let ndx = dx / dist;
        let ndy = dy / dist;

        // Sensor inputs (pushing onto stack or updating a mock sensor state)
        // Here we just use the VM to generate a force output.
        // We'll run the VM for a few steps and use the top of the stack as steering.

        // Since Chimera VM only uses Int for numeric types, scale by 1000
        self.vm.stack.push(Value::Int((ndx * 1000.0) as i64));
        self.vm.stack.push(Value::Int((ndy * 1000.0) as i64));
        self.vm.step();

        // Decode steering from VM
        let mut steer_x = ndx;
        let mut steer_y = ndy;
        if let Some(Value::Int(i)) = self.vm.stack.pop() {
            steer_y = i as f32 / 1000.0;
        }
        if let Some(Value::Int(i)) = self.vm.stack.pop() {
            steer_x = i as f32 / 1000.0;
        }

        let steer_len = (steer_x * steer_x + steer_y * steer_y).sqrt().max(0.1);
        steer_x /= steer_len;
        steer_y /= steer_len;

        // Apply force
        self.vx += steer_x * 20.0 * dt;
        self.vy += steer_y * 20.0 * dt;

        // Speed limit
        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt().max(0.1);
        if speed > 100.0 {
            self.vx = (self.vx / speed) * 100.0;
            self.vy = (self.vy / speed) * 100.0;
        }

        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Collision with firewalls
        for fw in firewalls {
            let fdx = self.x - fw.x;
            let fdy = self.y - fw.y;
            let fdist = (fdx * fdx + fdy * fdy).sqrt();
            if fdist < FIREWALL_RADIUS {
                self.alive = false;
                break;
            }
        }

        // Screen bounds wrap
        if self.x < 0.0 {
            self.x += screen_width();
        }
        if self.x > screen_width() {
            self.x -= screen_width();
        }
        if self.y < 0.0 {
            self.y += screen_height();
        }
        if self.y > screen_height() {
            self.y -= screen_height();
        }
    }
}

fn random_dna() -> Dna {
    let mut genes = vec![];
    for _ in 0..10 {
        genes.push(Gene {
            op: match rand::gen_range(0, 4) {
                0 => OpCode::Add,
                1 => OpCode::Sub,
                2 => OpCode::Mul,
                _ => OpCode::Push,
            },
            args: vec![Nucleotide::Number(rand::gen_range(-10, 10))],
        });
    }
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

fn crossover(a: &Dna, b: &Dna) -> Dna {
    let mut child_genes = vec![];
    let a_genes = &a.helix.strands[0].genes;
    let b_genes = &b.helix.strands[0].genes;

    let len = a_genes.len().min(b_genes.len());
    let mid = rand::gen_range(0, len.max(1));

    for i in 0..len {
        if i < mid {
            child_genes.push(a_genes[i].clone());
        } else {
            child_genes.push(b_genes[i].clone());
        }
    }

    // Mutation
    if rand::gen_range(0.0, 1.0) < 0.1 {
        let idx = rand::gen_range(0, child_genes.len().max(1));
        if idx < child_genes.len() {
            child_genes[idx].args = vec![Nucleotide::Number(rand::gen_range(-10, 10))];
        }
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes: child_genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Chimera DDoS")]
async fn main() {
    let mut packets = vec![];
    for _ in 0..200 {
        packets.push(Packet::new(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
            random_dna(),
        ));
    }

    let mut firewalls = vec![];
    for _ in 0..10 {
        firewalls.push(Firewall {
            x: rand::gen_range(100.0, 700.0),
            y: rand::gen_range(100.0, 500.0),
        });
    }

    let server_x = screen_width() / 2.0;
    let server_y = screen_height() / 2.0;

    let mut generation = 0;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        // Update packets
        for packet in &mut packets {
            packet.update(dt, server_x, server_y, &firewalls);
        }

        // Check for success and reproduce
        let mut new_packets = vec![];
        let mut successful_dna = vec![];

        for packet in &mut packets {
            if !packet.alive {
                continue;
            }
            let dx = server_x - packet.x;
            let dy = server_y - packet.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < SERVER_RADIUS {
                packet.alive = false; // Successfully hit target
                successful_dna.push(packet.dna.clone());
            }
        }

        packets.retain(|p| p.alive);

        // Reproduction phase if we have successes
        if !successful_dna.is_empty() {
            while packets.len() + new_packets.len() < 200 {
                let parent_a = &successful_dna[rand::gen_range(0, successful_dna.len())];
                let parent_b = &successful_dna[rand::gen_range(0, successful_dna.len())];
                let child_dna = crossover(parent_a, parent_b);

                // Spawn near edges
                let (spawn_x, spawn_y) = if rand::gen_range(0, 2) == 0 {
                    (rand::gen_range(0.0, screen_width()), 0.0)
                } else {
                    (0.0, rand::gen_range(0.0, screen_height()))
                };

                new_packets.push(Packet::new(spawn_x, spawn_y, child_dna));
            }
            generation += 1;
        }

        packets.append(&mut new_packets);

        // Replenish dead packets with random DNA if all died
        if packets.is_empty() {
            for _ in 0..200 {
                packets.push(Packet::new(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                    random_dna(),
                ));
            }
        }

        // Render
        // Draw firewalls
        for fw in &firewalls {
            draw_circle(fw.x, fw.y, FIREWALL_RADIUS, RED);
        }

        // Draw Server
        draw_circle(server_x, server_y, SERVER_RADIUS, BLUE);

        // Draw packets
        for packet in &packets {
            draw_circle(packet.x, packet.y, 2.0, packet.color);
        }

        draw_text(
            &format!("Generation: {}", generation),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Alive Packets: {}", packets.len()),
            10.0,
            50.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}
