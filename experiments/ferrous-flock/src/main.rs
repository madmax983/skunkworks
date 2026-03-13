use ::rand::Rng;
use macroquad::prelude::*;

mod physics;
use physics::Flock;

#[macroquad::main("Ferrous Flock")]
async fn main() {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let mut flock = Flock::new(screen_w as f64, screen_h as f64);
    let mut rng = ::rand::thread_rng();

    for _ in 0..300 {
        let x = rng.gen_range(0.0..screen_w as f64);
        let y = rng.gen_range(0.0..screen_h as f64);
        // Randomly assign North (+1.0) or South (-1.0)
        let polarity = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        flock.add_particle(x, y, polarity);

        // Give an initial random velocity
        let idx = flock.particles.len() - 1;
        flock.particles[idx].vel.x = rng.gen_range(-1.0..1.0);
        flock.particles[idx].vel.y = rng.gen_range(-1.0..1.0);
    }

    loop {
        clear_background(BLACK);

        let dt = get_frame_time().min(0.05);
        flock.update((dt * 60.0) as f64); // Assuming 60fps base speed

        for p in &flock.particles {
            let color = if p.polarity > 0.0 { RED } else { BLUE };

            // Draw a small circle
            draw_circle(p.pos.x as f32, p.pos.y as f32, 3.0, color);

            // Draw heading vector
            if p.vel.magnitude_squared() > 0.001 {
                let dir = p.vel.normalize();
                draw_line(
                    p.pos.x as f32,
                    p.pos.y as f32,
                    (p.pos.x + dir.x * 6.0) as f32,
                    (p.pos.y + dir.y * 6.0) as f32,
                    1.0,
                    WHITE,
                );
            }
        }

        next_frame().await;
    }
}
