use macroquad::prelude::*;
use market_sim::{Grid, Particle};
use origami::{generate_miura_grid, MiuraParams, Orientation};

fn window_conf() -> Conf {
    Conf {
        window_title: "Origami Market".to_owned(),
        ..Default::default()
    }
}

async fn run_sim() {
    let grid_size = 20;
    let mut market = Grid::new(grid_size, grid_size);

    let mut camera = Camera3D {
        position: vec3(0., 30., 40.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let mut base_gamma = 1.0;

    loop {
        clear_background(BLACK);

        // Advance market
        let trades = market.update();
        if !trades.is_empty() {
            base_gamma = (base_gamma - 0.05 * trades.len() as f32).max(0.2);
        } else {
            base_gamma = (base_gamma + 0.01).min(1.0);
        }

        // Draw market mapped onto Miura-ori mesh
        set_camera(&camera);

        let params = MiuraParams {
            a: 2.0,
            b: 2.0,
            gamma: base_gamma,
            orientation: Orientation::Horizontal,
        };

        // Note: generate_miura_grid returns points for a grid of size (cols+1) x (rows+1)
        // grid_size items implies we use size-1 cells to get size points
        let mesh_grid = generate_miura_grid(params, (grid_size - 1, grid_size - 1), 0.5);

        for y in 0..grid_size {
            for x in 0..grid_size {
                let cell = market.get(x, y);
                let p = mesh_grid[y * grid_size + x];

                let color = match cell {
                    Particle::Empty => None,
                    Particle::Bid(_) => Some(GREEN),
                    Particle::Ask(_) => Some(RED),
                    Particle::Trade { .. } => Some(YELLOW),
                    Particle::Wall => Some(DARKGRAY),
                };

                if let Some(c) = color {
                    let p_mq = vec3(p.x, p.y, p.z);
                    draw_sphere(p_mq, 0.5, None, c);
                }
            }
        }

        // Draw structural connections for the soft body mesh visualization
        for y in 0..grid_size - 1 {
            for x in 0..grid_size - 1 {
                let p0 = mesh_grid[y * grid_size + x];
                let p1 = mesh_grid[y * grid_size + x + 1];
                let p2 = mesh_grid[(y + 1) * grid_size + x];

                let p0_mq = vec3(p0.x, p0.y, p0.z);
                let p1_mq = vec3(p1.x, p1.y, p1.z);
                let p2_mq = vec3(p2.x, p2.y, p2.z);
                draw_line_3d(p0_mq, p1_mq, WHITE);
                draw_line_3d(p0_mq, p2_mq, WHITE);
            }
        }

        // Camera movement
        if is_key_down(KeyCode::Left) {
            camera.position.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            camera.position.x += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            camera.position.y += 1.0;
        }
        if is_key_down(KeyCode::Down) {
            camera.position.y -= 1.0;
        }

        set_default_camera();

        draw_text(
            &format!("Bids: {}", market.total_bids),
            10.0,
            20.0,
            30.0,
            GREEN,
        );
        draw_text(
            &format!("Asks: {}", market.total_asks),
            10.0,
            50.0,
            30.0,
            RED,
        );
        draw_text(
            &format!("Trades: {}", market.trade_count),
            10.0,
            80.0,
            30.0,
            YELLOW,
        );
        draw_text(
            &format!("Gamma Tension: {:.2}", base_gamma),
            10.0,
            110.0,
            30.0,
            WHITE,
        );

        next_frame().await
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("--headless")) {
        println!("Headless mode: exiting early to prevent CI timeouts.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_sim());
}
