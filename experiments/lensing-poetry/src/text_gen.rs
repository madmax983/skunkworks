use ::rand::Rng;
use macroquad::prelude::*;

pub fn generate_poetry_target(width: u32, height: u32) -> RenderTarget {
    let target = render_target(width, height);
    target.texture.set_filter(FilterMode::Linear);

    // Setup camera to draw to texture in screen coordinates (0,0 top-left)
    let cam = Camera2D {
        zoom: vec2(2.0 / width as f32, -2.0 / height as f32),
        target: vec2(width as f32 / 2.0, height as f32 / 2.0),
        render_target: Some(target.clone()),
        ..Default::default()
    };

    set_camera(&cam);

    clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Dark blue-ish

    let lines = [
        "In the beginning, there was gravity.",
        "A single point of infinite density.",
        "Then, expansion.",
        "Galaxies spiral in the void.",
        "Light bends around the dark matter.",
        "Space-time is a fabric, woven by mass.",
        "We are stardust, observing itself.",
        "Entropy increases, always.",
        "Orbits decay, stars collide.",
        "The dance of the spheres continues.",
        "Until the heat death of the universe.",
        "Silence.",
        "E = mc^2",
        "F = G m1 m2 / r^2",
        "R_uv - 1/2 R g_uv = 8 pi G T_uv",
        "d/dt (dL/dv) - dL/dx = 0",
        "H |psi> = E |psi>",
        "i h_bar d/dt |psi> = H |psi>",
        "ds^2 = -(1-2M/r)dt^2 + (1-2M/r)^-1 dr^2",
        "nabla . B = 0",
        "nabla x E = -dB/dt",
    ];

    let mut rng = ::rand::thread_rng();

    for _ in 0..200 {
        let text = lines[rng.gen_range(0..lines.len())];
        let x = rng.gen_range(0.0..width as f32);
        let y = rng.gen_range(0.0..height as f32);
        let size = rng.gen_range(15.0..40.0);
        let alpha = rng.gen_range(0.2..0.6);
        let color = Color::new(0.7, 0.8, 1.0, alpha);

        draw_text(text, x, y, size, color);
    }

    set_default_camera();

    target
}
