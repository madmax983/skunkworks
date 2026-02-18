use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

#[derive(Resource)]
pub struct TerrainMap {
    pub width: usize,
    pub height: usize,
    pub heights: Vec<f32>,
    pub sediment: Vec<f32>, // Accumulated sediment
    pub water: Vec<f32>,    // Current water depth
}

impl TerrainMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            heights: vec![0.0; width * height],
            sediment: vec![0.0; width * height],
            water: vec![0.0; width * height],
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height { return 0.0; }
        self.heights[y * self.width + x]
    }
}

#[derive(Component)]
pub struct TerrainMesh;

pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_map: Res<TerrainMap>,
) {
    let mesh = create_mesh(&terrain_map);
    let handle = meshes.add(mesh);

    commands.spawn((
        PbrBundle {
            mesh: handle,
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.9,
                reflectance: 0.1,
                ..default()
            }),
            transform: Transform::from_xyz(
                -(terrain_map.width as f32) / 2.0,
                0.0,
                -(terrain_map.height as f32) / 2.0
            ),
            ..default()
        },
        TerrainMesh,
    ));
}

pub fn update_terrain_mesh(
    terrain_map: Res<TerrainMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Handle<Mesh>, With<TerrainMesh>>,
) {
    if !terrain_map.is_changed() {
        return;
    }

    for handle in query.iter() {
        if let Some(mesh) = meshes.get_mut(handle) {
            update_mesh_vertices(mesh, &terrain_map);
        }
    }
}

fn create_mesh(terrain: &TerrainMap) -> Mesh {
    let w = terrain.width;
    let h = terrain.height;

    let mut positions = Vec::with_capacity(w * h);
    let mut normals = Vec::with_capacity(w * h);
    let mut uvs = Vec::with_capacity(w * h);
    let mut colors = Vec::with_capacity(w * h);
    let mut indices = Vec::new();

    for y in 0..h {
        for x in 0..w {
            positions.push([x as f32, terrain.heights[y * w + x], y as f32]);
            normals.push([0.0, 1.0, 0.0]); // Recalculate later
            uvs.push([x as f32 / w as f32, y as f32 / h as f32]);
            colors.push([0.5, 0.5, 0.5, 1.0]);
        }
    }

    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let i = (y * w + x) as u32;
            indices.push(i);
            indices.push(i + w as u32);
            indices.push(i + 1);

            indices.push(i + 1);
            indices.push(i + w as u32);
            indices.push(i + w as u32 + 1);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    recalculate_normals(&mut mesh, terrain);
    mesh
}

fn update_mesh_vertices(mesh: &mut Mesh, terrain: &TerrainMap) {
    let w = terrain.width;
    let h = terrain.height;

    if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for y in 0..h {
            for x in 0..w {
                positions[y * w + x][1] = terrain.heights[y * w + x];
            }
        }
    }

    // Update colors based on height/sediment
    if let Some(bevy::render::mesh::VertexAttributeValues::Float32x4(colors)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR)
    {
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let height = terrain.heights[idx];
                let sediment = terrain.sediment[idx];
                let water = terrain.water[idx];

                // Simple gradient
                let mut color = if height < 5.0 {
                    [0.2, 0.6, 0.2, 1.0] // Green
                } else if height < 20.0 {
                    [0.5, 0.5, 0.5, 1.0] // Grey
                } else {
                    [0.9, 0.9, 0.9, 1.0] // Snow
                };

                // Visualize sediment (Brown)
                if sediment > 0.1 {
                    color = [0.6, 0.4, 0.2, 1.0];
                }

                // Visualize water (Blue)
                if water > 0.1 {
                     color = [0.2, 0.4, 0.8, 1.0];
                }

                colors[idx] = color;
            }
        }
    }

    recalculate_normals(mesh, terrain);
}

fn recalculate_normals(mesh: &mut Mesh, terrain: &TerrainMap) {
    // Simple finite difference normal calculation
    let w = terrain.width;
    let h = terrain.height;

    // Create temporary normals vector
    let mut new_normals = vec![[0.0, 1.0, 0.0]; w * h];

    for y in 1..h-1 {
        for x in 1..w-1 {
            let h_l = terrain.heights[y * w + (x - 1)];
            let h_r = terrain.heights[y * w + (x + 1)];
            let h_d = terrain.heights[(y - 1) * w + x];
            let h_u = terrain.heights[(y + 1) * w + x];

            // Tangent vectors
            let tx = Vec3::new(2.0, h_r - h_l, 0.0).normalize();
            let ty = Vec3::new(0.0, h_u - h_d, 2.0).normalize();

            let normal = ty.cross(tx).normalize();
            new_normals[y * w + x] = [normal.x, normal.y, normal.z];
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
}
