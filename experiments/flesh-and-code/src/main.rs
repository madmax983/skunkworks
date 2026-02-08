use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::fs;

mod parser;
mod ik;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .init_resource::<CodeCursor>()
        .init_resource::<TokenPositions>()
        .add_systems(Startup, setup)
        .add_systems(Update, (navigate_code, ik::solve_ik, update_limbs))
        .run();
}

#[derive(Component)]
#[allow(dead_code)]
struct CodeToken {
    index: usize,
}

#[derive(Resource)]
struct CodeCursor {
    index: usize,
    timer: Timer,
}

impl Default for CodeCursor {
    fn default() -> Self {
        Self {
            index: 0,
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default)]
struct TokenPositions {
    positions: Vec<Vec2>,
}

#[derive(Component)]
struct LimbVisuals {
    root: Entity,
    joints: Vec<Entity>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut token_positions: ResMut<TokenPositions>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load Code
    let path = "experiments/flesh-and-code/src/parser.rs";
    let code = fs::read_to_string(path).unwrap_or_else(|_| {
        fs::read_to_string("src/parser.rs").unwrap_or_else(|_| "// Error reading file".to_string())
    });

    let tokens = parser::tokenize(&code);
    let font = asset_server.load("DejaVuSans.ttf");
    let char_width = 10.0;
    let line_height = 20.0;

    // Calculate token positions
    let mut positions = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        let x = token.col as f32 * char_width - 400.0;
        let y = -(token.line as f32 * line_height) + 300.0;

        positions.push(Vec2::new(x, y));

        let color = match token.kind {
            parser::TokenType::Keyword => Color::srgb(1.0, 0.4, 0.4),
            parser::TokenType::Ident => Color::srgb(0.4, 0.8, 1.0),
            parser::TokenType::String => Color::srgb(0.4, 1.0, 0.4),
            parser::TokenType::Comment => Color::srgb(0.5, 0.5, 0.5),
            parser::TokenType::Punct => Color::WHITE,
            _ => Color::WHITE,
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(token.text.clone(), TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color,
                }),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            CodeToken { index: i },
        ));
    }

    token_positions.positions = positions;

    // Spawn Creature
    spawn_creature(&mut commands, Vec2::new(-450.0, 350.0));
}

fn spawn_creature(commands: &mut Commands, start_pos: Vec2) {
    // 1. Target
    let target = commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(start_pos.x, start_pos.y, 10.0)),
        VisibilityBundle::default(), // Invisible target
    )).id();

    // 2. Root (Body)
    let root = commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle { radius: 8.0, center: Vec2::ZERO }),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(start_pos.x, start_pos.y, 5.0)),
            ..default()
        },
        Fill::color(Color::srgb(0.8, 0.2, 0.8)),
    )).id();

    // 3. Joints (Arm)
    let num_segments = 12;
    let segment_length = 15.0;
    let mut joints = Vec::new();
    let mut parent = root;

    for i in 0..num_segments {
        // First segment attached to root
        let x_offset = if i == 0 { 0.0 } else { segment_length };

        let joint = commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(x_offset, 0.0, 0.0)),
        )).id();

        commands.entity(parent).add_child(joint);
        joints.push(joint);
        parent = joint;
    }

    let effector = parent; // The last entity is the tip

    // 4. IK Chain Component
    commands.spawn(ik::IkChain {
        target,
        joints: joints.clone(),
        end_effector: effector,
    });

    // 5. Visuals (Line renderer)
    commands.spawn((
        ShapeBundle {
            path: PathBuilder::new().build(),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 6.0)),
            ..default()
        },
        Stroke::new(Color::srgb(1.0, 0.0, 1.0), 3.0),
        LimbVisuals { root, joints },
    ));
}

fn navigate_code(
    time: Res<Time>,
    mut cursor: ResMut<CodeCursor>,
    token_positions: Res<TokenPositions>,
    chains: Query<&ik::IkChain>,
    mut transforms: Query<&mut Transform>,
) {
    cursor.timer.tick(time.delta());

    if cursor.timer.finished() {
        cursor.index = (cursor.index + 1) % token_positions.positions.len().max(1);
    }

    // Smoothly move target towards current token position
    if let Some(&pos) = token_positions.positions.get(cursor.index) {
        for chain in chains.iter() {
            if let Ok(mut target_tf) = transforms.get_mut(chain.target) {
                let current = target_tf.translation.truncate();
                let diff = pos - current;
                // Move fast but smooth
                let speed = 10.0;
                let new_pos = current + diff * speed * time.delta_seconds();
                target_tf.translation = new_pos.extend(target_tf.translation.z);
            }
        }
    }
}

fn update_limbs(
    mut path_query: Query<(&mut Path, &LimbVisuals)>,
    transforms: Query<&GlobalTransform>,
) {
    for (mut path, visuals) in path_query.iter_mut() {
        let mut builder = PathBuilder::new();

        if let Ok(root_tf) = transforms.get(visuals.root) {
            let start = root_tf.translation().truncate();
            builder.move_to(start);

            for &joint in &visuals.joints {
                if let Ok(joint_tf) = transforms.get(joint) {
                    builder.line_to(joint_tf.translation().truncate());
                }
            }
        }

        *path = builder.build();
    }
}
