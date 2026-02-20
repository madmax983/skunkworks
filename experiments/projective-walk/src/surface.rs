use macroquad::prelude::*;

pub fn roman_surface(u: f32, v: f32) -> Vec3 {
    let pi = std::f32::consts::PI;
    let u = u * pi;
    let v = v * pi;

    // Roman Surface
    // x = sin(2u) * sin^2(v)
    // y = sin(u) * sin(2v)
    // z = cos(u) * sin(2v)

    let x = (2.0 * u).sin() * v.sin().powi(2);
    let y = u.sin() * (2.0 * v).sin();
    let z = u.cos() * (2.0 * v).sin();

    Vec3::new(x, y, z) * 4.0 // Scale up
}

pub fn generate_mesh(res: usize) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=res {
        for j in 0..=res {
            let u = i as f32 / res as f32;
            let v = j as f32 / res as f32;

            let pos = roman_surface(u, v);

            // Color based on u,v to show topology
            let color = Color::new(u, v, 0.5, 1.0);

            vertices.push(Vertex {
                position: pos,
                uv: vec2(u, v),
                color: color.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0), // Placeholder
            });
        }
    }

    for i in 0..res {
        for j in 0..res {
            let tl = (i * (res + 1) + j) as u16;
            let tr = (i * (res + 1) + (j + 1)) as u16;
            let bl = ((i + 1) * (res + 1) + j) as u16;
            let br = ((i + 1) * (res + 1) + (j + 1)) as u16;

            indices.push(tl);
            indices.push(bl);
            indices.push(tr);

            indices.push(tr);
            indices.push(bl);
            indices.push(br);
        }
    }

    (vertices, indices)
}
