use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes;
use rand::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Octopus Librarian".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_world)
        .add_systems(Update, (choreographer, ik_solver, draw_tentacles))
        .run();
}

#[derive(Component)]
struct Librarian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TentacleState {
    Idle,
    Reaching,
    Reading,
    Retracting,
}

#[derive(Component)]
struct Tentacle {
    joints: Vec<Vec2>,
    segment_length: f32,
    base_pos: Vec2,
    target: Vec2,
    state: TentacleState,
    assigned_book: Option<Entity>,
    idle_offset: f32,
}

#[derive(Component)]
struct Book {
    path: PathBuf,
    original_pos: Vec2,
}

#[derive(Component)]
struct TentacleVisual;

fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn Librarian Body
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 30.0,
                center: Vec2::ZERO,
            }),
            ..default()
        },
        Fill::color(Color::srgb(0.5, 0.0, 0.5)),
        Stroke::new(Color::srgb(0.8, 0.4, 0.8), 2.0),
        Librarian,
    ));

    // Spawn Tentacles
    let num_tentacles = 8;
    for i in 0..num_tentacles {
        let angle = (i as f32 / num_tentacles as f32) * std::f32::consts::TAU;
        let base_pos = Vec2::new(angle.cos() * 30.0, angle.sin() * 30.0);

        // Initial joints
        let mut joints = Vec::new();
        let num_segments = 30;
        let segment_len = 10.0;
        let mut curr = base_pos;
        for _ in 0..=num_segments {
            joints.push(curr);
            curr += Vec2::new(angle.cos() * segment_len, angle.sin() * segment_len);
        }

        let tentacle_entity = commands.spawn((
            Tentacle {
                joints,
                segment_length: segment_len,
                base_pos,
                target: curr, // Initially target end
                state: TentacleState::Idle,
                assigned_book: None,
                idle_offset: rand::random::<f32>() * 100.0,
            },
            SpatialBundle::default(),
        )).id();

        // Visual for tentacle
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
                ..default()
            },
            Stroke::new(Color::srgb(0.6, 0.2, 0.6), 3.0),
            TentacleVisual,
        )).set_parent(tentacle_entity);
    }

    // Scan for books (experiments)
    let root = PathBuf::from("experiments");
    let mut books = Vec::new();

    if root.exists() {
        for entry in WalkDir::new(root).min_depth(1).max_depth(2) {
            if let Ok(entry) = entry {
                if entry.file_type().is_dir() {
                    // Check if it has a Cargo.toml
                    let cargo_path = entry.path().join("Cargo.toml");
                    if cargo_path.exists() {
                        books.push(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    // Spiral Layout
    let a = 150.0;
    let b = 15.0; // Distance between turns
    for (i, book_path) in books.iter().enumerate() {
        let angle = i as f32 * 0.5; // radians per book
        let radius = a + b * angle;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let pos = Vec2::new(x, y);

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(20.0, 30.0),
                    origin: RectangleOrigin::Center,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(x, y, -1.0)),
                ..default()
            },
            Fill::color(Color::srgb(0.2, 0.6, 1.0)),
            Stroke::new(Color::srgb(0.8, 0.8, 0.8), 1.0),
            Book {
                path: book_path.clone(),
                original_pos: pos,
            },
        ));
    }
}

fn choreographer(
    time: Res<Time>,
    mut tentacles: Query<&mut Tentacle>,
    books: Query<(Entity, &Book, &Transform)>,
) {
    let t = time.elapsed_seconds();
    let mut rng = rand::thread_rng();

    // Collect book entities to pick from
    let book_entities: Vec<(Entity, Vec2)> = books.iter()
        .map(|(e, b, _)| (e, b.original_pos))
        .collect();

    if book_entities.is_empty() {
        return;
    }

    for mut tentacle in tentacles.iter_mut() {
        match tentacle.state {
            TentacleState::Idle => {
                // Wiggle
                let angle = t + tentacle.idle_offset;
                let radius = 100.0 + (t * 2.0 + tentacle.idle_offset).sin() * 20.0;
                let wiggle = Vec2::new(angle.cos(), angle.sin()) * radius;
                tentacle.target = tentacle.base_pos + wiggle;

                // Randomly start reaching
                if rng.gen_bool(0.005) { // 0.5% chance per frame
                    if let Some(&(book_entity, _)) = book_entities.choose(&mut rng) {
                        tentacle.assigned_book = Some(book_entity);
                        // Start reaching from current target pos
                        tentacle.state = TentacleState::Reaching;
                    }
                }
            }
            TentacleState::Reaching => {
                if let Some(book_entity) = tentacle.assigned_book {
                    // Find the book pos
                    if let Ok((_, _, transform)) = books.get(book_entity) {
                        let book_pos = transform.translation.truncate();
                        let dir = book_pos - tentacle.target;
                        let dist = dir.length();

                        if dist < 10.0 {
                            tentacle.state = TentacleState::Reading;
                        } else {
                            // Move target towards book
                            let speed = 300.0 * time.delta_seconds();
                            tentacle.target += dir.normalize_or_zero() * speed.min(dist);
                        }
                    } else {
                        // Book gone?
                        tentacle.state = TentacleState::Retracting;
                    }
                } else {
                    tentacle.state = TentacleState::Retracting;
                }
            }
            TentacleState::Reading => {
                 // Wiggle slightly around the book
                 if let Some(book_entity) = tentacle.assigned_book {
                     if let Ok((_, _, transform)) = books.get(book_entity) {
                         let book_pos = transform.translation.truncate();
                         let wiggle = Vec2::new((t * 5.0).cos(), (t * 5.0).sin()) * 5.0;
                         tentacle.target = book_pos + wiggle;
                     }
                 }

                 // Chance to stop
                 if rng.gen_bool(0.01) {
                     tentacle.state = TentacleState::Retracting;
                     tentacle.assigned_book = None;
                 }
            }
            TentacleState::Retracting => {
                // Move back to idle area
                let angle = t + tentacle.idle_offset;
                let idle_pos = tentacle.base_pos + Vec2::new(angle.cos(), angle.sin()) * 100.0;

                let dir = idle_pos - tentacle.target;
                let dist = dir.length();

                if dist < 10.0 {
                    tentacle.state = TentacleState::Idle;
                } else {
                    let speed = 200.0 * time.delta_seconds();
                    tentacle.target += dir.normalize_or_zero() * speed.min(dist);
                }
            }
        }
    }
}

fn ik_solver(
    _time: Res<Time>,
    mut tentacles: Query<&mut Tentacle>,
) {
    for mut tentacle in tentacles.iter_mut() {
        let target = tentacle.target;
        let base = tentacle.base_pos;
        let segment_len = tentacle.segment_length;
        let num_joints = tentacle.joints.len();

        // FABRIK Iterations
        for _ in 0..10 {
            // Forward Reaching (Target -> Base)
            // Set end effector to target
            if num_joints > 0 {
                tentacle.joints[num_joints - 1] = target;
            }

            for i in (0..num_joints - 1).rev() {
                let p_next = tentacle.joints[i+1];
                let p_curr = tentacle.joints[i];
                let dist = p_next.distance(p_curr);
                if dist > 0.0001 {
                     let dir = (p_curr - p_next).normalize();
                     tentacle.joints[i] = p_next + dir * segment_len;
                }
            }

            // Backward Reaching (Base -> Target)
            // Set base to fixed position
            tentacle.joints[0] = base;

            for i in 0..num_joints - 1 {
                let p_curr = tentacle.joints[i];
                let p_next = tentacle.joints[i+1];
                let dist = p_next.distance(p_curr);
                if dist > 0.0001 {
                    let dir = (p_next - p_curr).normalize();
                    tentacle.joints[i+1] = p_curr + dir * segment_len;
                }
            }
        }
    }
}

fn draw_tentacles(
    tentacles: Query<(&Tentacle, &Children)>,
    mut paths: Query<(&mut Path, &mut Stroke), With<TentacleVisual>>,
) {
    for (tentacle, children) in tentacles.iter() {
        for &child in children.iter() {
            if let Ok((mut path, mut stroke)) = paths.get_mut(child) {
                // Update Path
                let mut builder = PathBuilder::new();
                if let Some(first) = tentacle.joints.first() {
                    builder.move_to(*first);
                    // Simple lines for now
                    for joint in tentacle.joints.iter().skip(1) {
                        builder.line_to(*joint);
                    }
                }
                *path = builder.build();

                // Update Color based on state
                match tentacle.state {
                    TentacleState::Idle => stroke.color = Color::srgb(0.6, 0.2, 0.6),
                    TentacleState::Reaching => stroke.color = Color::srgb(0.8, 0.4, 0.8),
                    TentacleState::Reading => stroke.color = Color::srgb(0.0, 1.0, 0.5), // Green when reading
                    TentacleState::Retracting => stroke.color = Color::srgb(0.4, 0.4, 0.4),
                }
            }
        }
    }
}
