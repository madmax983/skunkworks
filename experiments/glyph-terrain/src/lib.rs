use ab_glyph::{Font, FontRef, Point};
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct GlyphMesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u16>,
}

impl GlyphMesh {
    pub fn bounds(&self) -> (Vec3, Vec3) {
        if self.vertices.is_empty() {
            return (Vec3::ZERO, Vec3::ZERO);
        }
        let mut min = self.vertices[0];
        let mut max = self.vertices[0];
        for v in &self.vertices {
            min = min.min(*v);
            max = max.max(*v);
        }
        (min, max)
    }

    pub fn center(&mut self) {
        let (min, max) = self.bounds();
        let center = (min + max) * 0.5;
        let offset = Vec3::new(center.x, 0.0, center.z);
        for v in &mut self.vertices {
            v.x -= offset.x;
            v.z -= offset.z;
        }
    }
}

pub fn load_font(data: &[u8]) -> Option<FontRef<'_>> {
    FontRef::try_from_slice(data).ok()
}

pub fn extract_outline(font: &impl Font, c: char) -> Option<Path> {
    // Note: font.outline() ignores scale.
    let glyph_id = font.glyph_id(c);

    if let Some(outline) = font.outline(glyph_id) {
        let mut builder = Path::builder();
        let mut last_end: Option<Point> = None;

        for curve in outline.curves {
            let (p0, p1_end) = match curve {
                ab_glyph::OutlineCurve::Line(p0, p1) => (p0, p1),
                ab_glyph::OutlineCurve::Quad(p0, _, p2) => (p0, p2),
                ab_glyph::OutlineCurve::Cubic(p0, _, _, p3) => (p0, p3),
            };

            let start_new = if let Some(le) = last_end {
                (p0.x - le.x).abs() > 0.001 || (p0.y - le.y).abs() > 0.001
            } else {
                true
            };

            if start_new {
                if last_end.is_some() {
                    builder.end(true);
                }
                builder.begin(point(p0.x, p0.y));
            }

            match curve {
                ab_glyph::OutlineCurve::Line(_, p1) => {
                    builder.line_to(point(p1.x, p1.y));
                }
                ab_glyph::OutlineCurve::Quad(_, p1, p2) => {
                    builder.quadratic_bezier_to(point(p1.x, p1.y), point(p2.x, p2.y));
                }
                ab_glyph::OutlineCurve::Cubic(_, p1, p2, p3) => {
                    builder.cubic_bezier_to(
                        point(p1.x, p1.y),
                        point(p2.x, p2.y),
                        point(p3.x, p3.y),
                    );
                }
            }

            last_end = Some(p1_end);
        }

        if last_end.is_some() {
            builder.end(true);
        }

        Some(builder.build())
    } else {
        None
    }
}

pub fn tessellate_path(path: &Path) -> GlyphMesh {
    let mut geometry: VertexBuffers<Vec3, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    let options = FillOptions::default().with_tolerance(0.1);

    // Scaling factor to bring font units (e.g. 2048) down to reasonable world units (e.g. 10.0)
    let scale = 0.01;

    tessellator
        .tessellate_path(
            path,
            &options,
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                let p = vertex.position();
                Vec3::new(p.x * scale, 0.0, -p.y * scale)
            }),
        )
        .unwrap();

    let mut mesh = GlyphMesh {
        vertices: geometry.vertices,
        indices: geometry.indices,
    };
    mesh.center();
    mesh
}

pub fn apply_terrain(mesh: &mut GlyphMesh, seed: u32, frequency: f64, amplitude: f32) {
    let perlin = Perlin::new(seed);
    for v in &mut mesh.vertices {
        // Use x and z for noise coordinates
        let val = perlin.get([v.x as f64 * frequency, v.z as f64 * frequency]);
        // Add height (y)
        v.y += (val as f32) * amplitude;
    }
}

pub fn glyph_mesh(font: &impl Font, c: char) -> GlyphMesh {
    if let Some(path) = extract_outline(font, c) {
        tessellate_path(&path)
    } else {
        GlyphMesh {
            vertices: vec![],
            indices: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_mesh_generation() {
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");
        let font = load_font(font_data).expect("Failed to load font");
        let mesh = glyph_mesh(&font, 'A');

        assert!(mesh.vertices.len() > 0, "Mesh should have vertices");
        assert!(mesh.indices.len() > 0, "Mesh should have indices");
    }

    #[test]
    fn test_terrain_generation() {
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");
        let font = load_font(font_data).expect("Failed to load font");
        let mut mesh = glyph_mesh(&font, 'B');

        // Initial Y should be 0.0
        for v in &mesh.vertices {
            assert_eq!(v.y, 0.0);
        }

        apply_terrain(&mut mesh, 123, 1.0, 5.0);

        // After terrain, some Y should be non-zero
        let has_height = mesh.vertices.iter().any(|v| v.y.abs() > 0.001);
        assert!(has_height, "Terrain should add height variation");
    }
}
