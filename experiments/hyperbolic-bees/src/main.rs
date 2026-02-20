use macroquad::prelude::*;
use num_complex::Complex;
use poincare_disk::{hyperbolic_dist, mobius_add, mobius_sub, Point};
use ::rand::Rng;

const DISK_RADIUS_SCREEN_FACTOR: f32 = 0.45;
// const HIVE_RADIUS_HYPERBOLIC: f64 = 0.2; // Radius in hyperbolic distance? No, in disk coords (0.2 is close to center)

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BeeState {
    Scouting,
    Returning,
    Dancing,
    Observing,
    Foraging,
}

#[derive(Clone, Copy, Debug)]
pub struct Bee {
    pub position: Point,
    pub state: BeeState,
    pub target_source_idx: Option<usize>,
    pub memory_quality: f64,
    pub dance_timer: f64,
    pub dance_angle: f64,
    pub dance_duration: f64,
    pub velocity_dir: f64, // Angle of movement for random walk
}

#[derive(Clone, Copy, Debug)]
pub struct Source {
    pub position: Point,
    pub quality: f64,
    pub radius: f64, // Detection radius in hyperbolic distance
}

#[derive(Clone, Debug)]
pub struct Dance {
    pub target_idx: usize,
    pub angle: f64,
    pub distance: f64,
    pub quality: f64,
}

pub struct World {
    pub bees: Vec<Bee>,
    pub sources: Vec<Source>,
}

impl World {
    pub fn new(num_bees: usize) -> Self {
        let mut bees = Vec::with_capacity(num_bees);
        let mut rng = ::rand::thread_rng();
        for _ in 0..num_bees {
            // Start near hive (origin)
            let r = rng.gen_range(0.0..0.1);
            let theta = rng.gen_range(0.0..std::f64::consts::TAU);
            bees.push(Bee {
                position: Complex::from_polar(r, theta),
                state: BeeState::Scouting,
                target_source_idx: None,
                memory_quality: 0.0,
                dance_timer: 0.0,
                dance_angle: 0.0,
                dance_duration: 0.0,
                velocity_dir: rng.gen_range(0.0..std::f64::consts::TAU),
            });
        }
        Self {
            bees,
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, r: f64, theta: f64, quality: f64) {
        self.sources.push(Source {
            position: Complex::from_polar(r, theta),
            quality,
            radius: 0.3, // Hyperbolic detection radius
        });
    }

    pub fn update(&mut self) {
        let mut rng = ::rand::thread_rng();
        let dt = 0.01; // Simulation step
        let bee_speed = 0.05; // Hyperbolic distance per tick

        // Collect active dances
        let dances: Vec<Dance> = self.bees.iter()
            .filter(|b| b.state == BeeState::Dancing && b.target_source_idx.is_some())
            .map(|b| {
                let source_pos = self.sources[b.target_source_idx.unwrap()].position;
                Dance {
                    target_idx: b.target_source_idx.unwrap(),
                    angle: source_pos.arg(), // Angle from hive (origin) to source
                    distance: hyperbolic_dist(Point::new(0.0, 0.0), source_pos),
                    quality: b.memory_quality,
                }
            })
            .collect();

        for bee in &mut self.bees {
            match bee.state {
                BeeState::Scouting => {
                    // Random walk
                    bee.velocity_dir += rng.gen_range(-1.0..1.0) * dt;
                    let step = Complex::from_polar(bee_speed * dt, bee.velocity_dir);
                    bee.position = mobius_add(step, bee.position);

                    // Check boundaries (keep within reasonable disk range to avoid edge artifacts)
                    if bee.position.norm() > 0.95 {
                         bee.velocity_dir += std::f64::consts::PI; // Turn around
                         // Bump back
                         bee.position = Complex::from_polar(0.94, bee.position.arg());
                    }

                    // Check for sources
                    for (i, source) in self.sources.iter().enumerate() {
                        if hyperbolic_dist(bee.position, source.position) < source.radius {
                            bee.target_source_idx = Some(i);
                            bee.memory_quality = source.quality;
                            bee.state = BeeState::Returning;
                            break;
                        }
                    }
                }
                BeeState::Returning => {
                    // Return to Hive (Origin)
                    // Direction is towards origin.
                    // To move towards origin, we use mobius_sub(pos, step) where step is in direction of pos?
                    // No, easier: local target is Origin (0,0).
                    // target_local = mobius_sub(0, pos) = -pos.
                    // So we move in direction of -pos.
                    let dist_to_hive = bee.position.norm(); // Euclidean norm is related to hyperbolic dist from origin
                    if dist_to_hive < 0.05 {
                        // Arrived
                        bee.state = BeeState::Dancing;
                        if let Some(idx) = bee.target_source_idx {
                             let source = &self.sources[idx];
                             bee.dance_angle = source.position.arg();
                             let dist = hyperbolic_dist(Point::new(0.0,0.0), source.position);
                             bee.dance_duration = dist * 100.0; // Dance longer for further sources? Or proportional?
                             // Von Frisch: Duration correlates with distance.
                             bee.dance_timer = bee.dance_duration;
                        } else {
                            bee.state = BeeState::Observing;
                        }
                    } else {
                         // Move towards origin
                         // In local frame of bee, where is origin?
                         // origin_local = mobius_sub(0, bee.pos) = -bee.pos / (1 - 0) = -bee.pos ?
                         // mobius_sub(z, a) = (z-a)/(1-a_bar z).
                         // mobius_sub(0, p) = (0-p)/(1-p_bar*0) = -p.
                         // So direction is argument of -p.
                         let dir = (-bee.position).arg();
                         let step = Complex::from_polar(bee_speed * dt, dir);
                         bee.position = mobius_add(step, bee.position);
                    }
                }
                BeeState::Dancing => {
                    bee.dance_timer -= 1.0;
                    if bee.dance_timer <= 0.0 {
                         // Done dancing.
                         if rng.gen_bool(0.5) {
                             bee.state = BeeState::Foraging; // Go back
                         } else {
                             bee.state = BeeState::Observing; // Rest
                         }
                    }
                    // Wiggle effect (visual only, handled in draw)
                }
                BeeState::Observing => {
                    // Drift near hive
                    let step = Complex::from_polar(bee_speed * 0.1 * dt, rng.gen_range(0.0..6.28));
                    bee.position = mobius_add(step, bee.position);
                    if bee.position.norm() > 0.1 {
                        bee.position = Complex::from_polar(0.09, bee.position.arg());
                    }

                    // Watch dances
                    if !dances.is_empty() {
                         let dance = &dances[rng.gen_range(0..dances.len())];
                         if rng.gen::<f64>() < dance.quality * 0.1 {
                             bee.target_source_idx = Some(dance.target_idx);
                             bee.state = BeeState::Foraging;
                         }
                    }
                }
                BeeState::Foraging => {
                    if let Some(idx) = bee.target_source_idx {
                        let source = self.sources[idx];
                        // Move towards source
                        let target_local = mobius_sub(source.position, bee.position);
                        let dir = target_local.arg();
                        let _dist = target_local.norm(); // Not hyperbolic dist, but monotonic

                        // Actually need to check hyperbolic dist for arrival
                        if hyperbolic_dist(bee.position, source.position) < source.radius * 0.5 {
                             // Arrived
                             bee.memory_quality = source.quality;
                             bee.state = BeeState::Returning;
                        } else {
                             // Move
                             let step = Complex::from_polar(bee_speed * dt, dir);
                             bee.position = mobius_add(step, bee.position);
                             // Jitter
                             let jitter = Complex::from_polar(bee_speed * 0.2 * dt, rng.gen_range(0.0..6.28));
                             bee.position = mobius_add(jitter, bee.position);
                        }
                    } else {
                        bee.state = BeeState::Returning;
                    }
                }
            }
        }
    }
}

#[macroquad::main("Hyperbolic Bees")]
async fn main() {
    let mut world = World::new(100);
    // Add some sources
    // r, theta, quality
    world.add_source(0.5, 0.0, 1.0); // Right
    world.add_source(0.7, 2.0, 0.5); // Top Left
    world.add_source(0.8, 4.0, 2.0); // Bottom Left (High quality)

    loop {
        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
             let (mx, my) = mouse_position();
             let sw = screen_width();
             let sh = screen_height();
             let sc = Vec2::new(sw/2.0, sh/2.0);
             let radius = sw.min(sh) * DISK_RADIUS_SCREEN_FACTOR;
             let dx = (mx - sc.x) / radius;
             let dy = -(my - sc.y) / radius; // Flip Y for complex plane
             let p = Complex::new(dx as f64, dy as f64);
             if p.norm() < 0.99 {
                  world.add_source(p.norm(), p.arg(), 1.5);
             }
        }

        world.update();

        clear_background(BLACK);

        let sw = screen_width();
        let sh = screen_height();
        let sc = Vec2::new(sw/2.0, sh/2.0);
        let radius = sw.min(sh) * DISK_RADIUS_SCREEN_FACTOR;

        // Draw Disk
        draw_circle(sc.x, sc.y, radius, Color::new(0.1, 0.1, 0.1, 1.0));
        draw_circle_lines(sc.x, sc.y, radius, 2.0, GRAY);

        // Draw Sources
        for source in &world.sources {
             let pos = to_screen(source.position, sc, radius);
             let _r = source.radius as f32 * 20.0; // Scale radius visual
             // Note: Hyperbolic circles appear as Euclidean circles but not centered on the representational point.
             // For simplicity, drawing Euclidean circle at point.
             let color = Color::new(0.2, 0.8 * source.quality as f32, 0.2, 1.0);
             draw_circle(pos.x, pos.y, 10.0 * source.quality as f32, color);
        }

        // Draw Hive
        draw_circle(sc.x, sc.y, 15.0, Color::new(0.6, 0.4, 0.1, 1.0));

        // Draw Bees
        for bee in &world.bees {
             let pos = to_screen(bee.position, sc, radius);
             let color = match bee.state {
                 BeeState::Scouting => WHITE,
                 BeeState::Returning => SKYBLUE,
                 BeeState::Dancing => GOLD,
                 BeeState::Observing => DARKGRAY,
                 BeeState::Foraging => ORANGE,
             };

             draw_circle(pos.x, pos.y, 3.0, color);

             if bee.state == BeeState::Dancing {
                 // Draw dance vector
                 // Angle points to source
                 let angle = bee.dance_angle;
                 // In Poincaré disk, geodesics from origin are straight lines.
                 // So we can draw a line in direction 'angle'.
                 let dance_vec = Vec2::new(angle.cos() as f32, angle.sin() as f32) * 20.0;
                 draw_line(pos.x, pos.y, pos.x + dance_vec.x, pos.y - dance_vec.y, 2.0, GOLD); // Flip Y again
             }
        }

        draw_text("Hyperbolic Bees", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Bees: {}", world.bees.len()), 20.0, 60.0, 20.0, GRAY);
        draw_text(&format!("Sources: {}", world.sources.len()), 20.0, 80.0, 20.0, GRAY);
        draw_text("Left Click to add Source", 20.0, 100.0, 20.0, GRAY);


        next_frame().await
    }
}

fn to_screen(p: Point, center: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        center.x + p.re as f32 * radius,
        center.y - p.im as f32 * radius,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_init() {
        let world = World::new(10);
        assert_eq!(world.bees.len(), 10);
        for bee in &world.bees {
            assert!(bee.position.norm() < 1.0);
            assert_eq!(bee.state, BeeState::Scouting);
        }
    }

    #[test]
    fn test_add_source() {
        let mut world = World::new(0);
        world.add_source(0.5, 0.0, 1.0);
        assert_eq!(world.sources.len(), 1);
        assert_eq!(world.sources[0].quality, 1.0);
    }
}
