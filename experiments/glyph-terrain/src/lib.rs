use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct HeightMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
}

impl HeightMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0.0
        }
    }
}

pub fn load_font(data: &[u8]) -> Option<FontRef<'_>> {
    FontRef::try_from_slice(data).ok()
}

pub fn generate_glyph_heightmap(
    font: &impl Font,
    c: char,
    width: usize,
    height: usize,
) -> HeightMap {
    let mut map = HeightMap::new(width, height);

    let scale_factor = (height as f32).min(width as f32) * 0.6;
    let scale = PxScale::from(scale_factor);
    let scaled_font = font.as_scaled(scale);

    let empty_glyph = scaled_font.glyph_id(c).with_scale(scale);
    let bounds = if let Some(g) = font.outline_glyph(empty_glyph) {
        g.px_bounds()
    } else {
        return map;
    };

    let glyph_width = bounds.width();
    let glyph_height = bounds.height();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    let shift_x = center_x - (bounds.min.x + glyph_width / 2.0);
    let shift_y = center_y - (bounds.min.y + glyph_height / 2.0);

    let mut positioned_glyph = scaled_font.glyph_id(c).with_scale(scale);
    positioned_glyph.position = ab_glyph::point(shift_x, shift_y);

    if let Some(outlined) = font.outline_glyph(positioned_glyph) {
         outlined.draw(|x, y, v| {
             if x < width as u32 && y < height as u32 {
                 map.data[(y as usize) * width + (x as usize)] = v;
             }
         });
    }

    map
}

pub fn create_grid_mesh(subdivisions: usize, size: f32) -> Mesh {
    let mut vertices = Vec::with_capacity(subdivisions * subdivisions);
    let mut indices = Vec::with_capacity((subdivisions - 1) * (subdivisions - 1) * 6);

    let step = size / (subdivisions as f32 - 1.0);
    let offset = size / 2.0;

    for z in 0..subdivisions {
        for x in 0..subdivisions {
            let pos = vec3(
                (x as f32) * step - offset,
                0.0,
                (z as f32) * step - offset,
            );

            let u = x as f32 / (subdivisions as f32 - 1.0);
            let v = z as f32 / (subdivisions as f32 - 1.0);

            vertices.push(Vertex {
                position: pos,
                uv: vec2(u, v),
                color: WHITE.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
        }
    }

    for z in 0..subdivisions - 1 {
        for x in 0..subdivisions - 1 {
            let i0 = (z * subdivisions + x) as u16;
            let i1 = (z * subdivisions + x + 1) as u16;
            let i2 = ((z + 1) * subdivisions + x) as u16;
            let i3 = ((z + 1) * subdivisions + x + 1) as u16;

            indices.push(i0);
            indices.push(i2);
            indices.push(i1);

            indices.push(i1);
            indices.push(i2);
            indices.push(i3);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

pub fn update_mesh_heights(
    mesh: &mut Mesh,
    heightmap: &HeightMap,
    noise_seed: u32,
    noise_scale: f64,
    noise_amp: f32,
    glyph_amp: f32
) {
    let perlin = Perlin::new(noise_seed);
    let w = heightmap.width as f32;
    let h = heightmap.height as f32;

    for vertex in &mut mesh.vertices {
        let u = vertex.uv.x;
        let v = vertex.uv.y;

        let x = (u * (w - 1.0)).round() as usize;
        let y = (v * (h - 1.0)).round() as usize;

        let glyph_h = heightmap.get(x, y);

        let noise_val = perlin.get([vertex.position.x as f64 * noise_scale, vertex.position.z as f64 * noise_scale]);

        vertex.position.y = (noise_val as f32 * noise_amp) + (glyph_h * glyph_amp);
    }

    // Calculate normals and bake lighting into color
    let mut normals = vec![Vec3::ZERO; mesh.vertices.len()];

    for i in (0..mesh.indices.len()).step_by(3) {
        let i0 = mesh.indices[i] as usize;
        let i1 = mesh.indices[i+1] as usize;
        let i2 = mesh.indices[i+2] as usize;

        let v0 = mesh.vertices[i0].position;
        let v1 = mesh.vertices[i1].position;
        let v2 = mesh.vertices[i2].position;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).normalize_or_zero();

        normals[i0] += normal;
        normals[i1] += normal;
        normals[i2] += normal;
    }

    let light_dir = vec3(0.5, 1.0, 0.5).normalize();

    for (i, v) in mesh.vertices.iter_mut().enumerate() {
        let n = normals[i].normalize_or_zero();
        v.normal = vec4(n.x, n.y, n.z, 0.0);

        let diffuse = n.dot(light_dir).max(0.2);

        let h = v.position.y;
        let base_color = if h < 0.5 {
             Color::new(0.0, 0.2, 0.8, 0.9)
        } else if h < 2.0 {
             Color::new(0.8, 0.7, 0.5, 1.0)
        } else if h < 6.0 {
             Color::new(0.1, 0.6, 0.1, 1.0)
        } else if h < 10.0 {
             Color::new(0.5, 0.5, 0.5, 1.0)
        } else {
             Color::new(0.9, 0.9, 1.0, 1.0)
        };

        let r = (base_color.r * diffuse).min(1.0);
        let g = (base_color.g * diffuse).min(1.0);
        let b = (base_color.b * diffuse).min(1.0);

        v.color = Color::new(r, g, b, base_color.a).into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heightmap_generation() {
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");
        let font = load_font(font_data).expect("Failed to load font");
        let map = generate_glyph_heightmap(&font, 'A', 64, 64);

        assert_eq!(map.width, 64);
        assert_eq!(map.height, 64);
        let has_data = map.data.iter().any(|&v| v > 0.0);
        assert!(has_data, "Heightmap should contain rendered glyph data");
    }

    #[test]
    fn test_grid_mesh() {
        let mesh = create_grid_mesh(10, 10.0);
        assert_eq!(mesh.vertices.len(), 100);
        assert_eq!(mesh.indices.len(), 9 * 9 * 6);
    }
}
