use lyon::math::{point, Point as LyonPoint}; // LyonPoint used in type annotation
use lyon::path::iterator::PathIterator;
use lyon::path::{Path, PathEvent};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor, VertexBuffers,
};
use macroquad::prelude::*;
use rusttype::{Font, OutlineBuilder, Point, PositionedGlyph, Scale};

pub struct TextMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

struct LyonPathBuilder {
    builder: lyon::path::path::Builder,
}

impl LyonPathBuilder {
    fn new() -> Self {
        Self {
            builder: Path::builder(),
        }
    }

    fn build(self) -> Path {
        self.builder.build()
    }
}

impl OutlineBuilder for LyonPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.begin(point(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(point(x, y));
    }

    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.builder.quadratic_bezier_to(point(cx, cy), point(x, y));
    }

    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        self.builder
            .cubic_bezier_to(point(cx1, cy1), point(cx2, cy2), point(x, y));
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

// Vertex constructor for Lyon
struct MyVertexConstructor;

impl FillVertexConstructor<Vertex> for MyVertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let pos = vertex.position();

        Vertex {
            position: vec3(pos.x, -pos.y, 0.0), // Top face at Z=0
            uv: vec2(pos.x / 100.0, pos.y / 100.0),
            color: WHITE.into(),
            normal: vec4(0.0, 0.0, 1.0, 0.0), // Z-up normal
        }
    }
}

impl TextMesh {
    pub fn from_text(font_path: &str, text: &str) -> anyhow::Result<Self> {
        let font_bytes = std::fs::read(font_path)?;
        let font =
            Font::try_from_bytes(&font_bytes).ok_or(anyhow::anyhow!("Error loading font"))?;

        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();
        let mut index_offset = 0;

        let scale = Scale::uniform(100.0);
        let v_metrics = font.v_metrics(scale);

        let start = Point {
            x: 0.0,
            y: v_metrics.ascent,
        };

        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, start).collect();

        for glyph in glyphs {
            let mut pen = LyonPathBuilder::new();
            glyph.build_outline(&mut pen);
            let path = pen.build();

            // 1. Tessellate Top Face
            let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
            let mut tessellator = FillTessellator::new();

            tessellator
                .tessellate_path(
                    &path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, MyVertexConstructor),
                )
                .map_err(|e| anyhow::anyhow!("Tessellation error: {:?}", e))?;

            for v in geometry.vertices {
                all_vertices.push(v);
            }
            for i in geometry.indices {
                all_indices.push(i + index_offset);
            }
            index_offset = all_vertices.len() as u16;

            // 2. Extrude Walls
            let depth = 20.0;
            // Iterate flattened path to generate walls
            // Closure to add wall quad
            let mut add_wall_vertices = |p1: LyonPoint, p2: LyonPoint| {
                let idx = all_vertices.len() as u16;
                // Wall vertices
                let v0 = Vertex {
                    position: vec3(p1.x, -p1.y, 0.0),
                    uv: vec2(0., 0.),
                    color: GRAY.into(),
                    normal: vec4(0., 0., 0., 0.),
                };
                let v1 = Vertex {
                    position: vec3(p2.x, -p2.y, 0.0),
                    uv: vec2(1., 0.),
                    color: GRAY.into(),
                    normal: vec4(0., 0., 0., 0.),
                };
                let v2 = Vertex {
                    position: vec3(p2.x, -p2.y, -depth),
                    uv: vec2(1., 1.),
                    color: DARKGRAY.into(),
                    normal: vec4(0., 0., 0., 0.),
                };
                let v3 = Vertex {
                    position: vec3(p1.x, -p1.y, -depth),
                    uv: vec2(0., 1.),
                    color: DARKGRAY.into(),
                    normal: vec4(0., 0., 0., 0.),
                };

                all_vertices.push(v0);
                all_vertices.push(v1);
                all_vertices.push(v2);
                all_vertices.push(v3);

                all_indices.push(idx);
                all_indices.push(idx + 1);
                all_indices.push(idx + 2);

                all_indices.push(idx);
                all_indices.push(idx + 2);
                all_indices.push(idx + 3);
            };

            for event in path.iter().flattened(0.5) {
                match event {
                    PathEvent::Line { from, to } => {
                        add_wall_vertices(from, to);
                    }
                    PathEvent::End { last, first, close } => {
                        if close {
                            add_wall_vertices(last, first);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            vertices: all_vertices,
            indices: all_indices,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_mesh_generation() {
        let font_path = "assets/font.ttf";

        let mesh_data = TextMesh::from_text(font_path, "A");

        if let Err(e) = &mesh_data {
            println!("Error: {:?}", e);
        }

        assert!(mesh_data.is_ok());
        let mesh = mesh_data.unwrap();

        assert!(mesh.vertices.len() > 0);
        assert!(mesh.indices.len() > 0);
    }
}
