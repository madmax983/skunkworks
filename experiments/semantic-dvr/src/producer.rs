use anyhow::Result;
use rand::Rng;
use std::{thread, time::Duration};
use tui_semantic::{Entity, Snapshot, Vec2};

struct Particle {
    id: usize,
    pos: Vec2,
    vel: Vec2,
    color: String,
}

pub fn run_producer() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut particles = Vec::new();
    let width = 100.0;
    let height = 50.0;

    for i in 0..10 {
        particles.push(Particle {
            id: i,
            pos: Vec2::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height)),
            vel: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            color: if i % 2 == 0 {
                "red".to_string()
            } else {
                "cyan".to_string()
            },
        });
    }

    let mut frame = 0;
    loop {
        update_particles(&mut particles, width, height);

        // Snapshot
        let mut snap = Snapshot::new("semantic-dvr-demo")
            .with_frame(frame)
            .with_viewport(width as u16, height as u16)
            .with_metric("particle_count", particles.len());

        for p in &particles {
            snap = snap.with_entity(
                Entity::new("particle")
                    .with_id(p.id.to_string())
                    .at(p.pos.x, p.pos.y)
                    .moving(p.vel.x, p.vel.y)
                    .with_prop("color", p.color.clone())
                    .display("O"),
            );
        }

        println!("{}", snap.to_json());
        frame += 1;
        thread::sleep(Duration::from_millis(33)); // ~30 FPS
    }
}

fn update_particles(particles: &mut [Particle], width: f64, height: f64) {
    for p in particles {
        p.pos.x += p.vel.x;
        p.pos.y += p.vel.y;

        if p.pos.x <= 0.0 || p.pos.x >= width {
            p.vel.x *= -1.0;
            p.pos.x = p.pos.x.clamp(0.0, width);
        }
        if p.pos.y <= 0.0 || p.pos.y >= height {
            p.vel.y *= -1.0;
            p.pos.y = p.pos.y.clamp(0.0, height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_bounds() {
        let mut particles = vec![Particle {
            id: 0,
            pos: Vec2::new(101.0, 50.0),
            vel: Vec2::new(1.0, 0.0),
            color: "red".to_string(),
        }];
        update_particles(&mut particles, 100.0, 50.0);
        assert!(particles[0].pos.x <= 100.0);
        assert_eq!(particles[0].vel.x, -1.0); // Should bounce
    }
}
