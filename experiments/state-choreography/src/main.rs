use bevy::prelude::*;
use bevy::color::palettes::css::*;
use bevy_prototype_lyon::prelude::*;

mod components;
mod ik;
mod choreographer;

use components::*;
use ik::*;
use choreographer::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(IKPlugin)
        .add_plugins(ChoreographerPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_skeleton_lines)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Root (Torso)
    let root = commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle { radius: 15.0, center: Vec2::ZERO }),
            spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
            ..default()
        },
        Fill::color(Color::from(WHITE)),
    )).id();

    // Head
    spawn_limb(&mut commands, root, Limb::Head, vec![(30.0, 0.5)], Vec3::new(0.0, 50.0, 0.0));

    // Left Arm
    let l_shoulder = commands.spawn(SpatialBundle::from_transform(Transform::from_translation(Vec3::new(-20.0, 10.0, 0.0)))).id();
    commands.entity(root).add_child(l_shoulder);
    spawn_limb(&mut commands, l_shoulder, Limb::LeftArm, vec![(35.0, 2.5), (30.0, 2.5)], Vec3::new(-50.0, 50.0, 0.0));

    // Right Arm
    let r_shoulder = commands.spawn(SpatialBundle::from_transform(Transform::from_translation(Vec3::new(20.0, 10.0, 0.0)))).id();
    commands.entity(root).add_child(r_shoulder);
    spawn_limb(&mut commands, r_shoulder, Limb::RightArm, vec![(35.0, 2.5), (30.0, 2.5)], Vec3::new(50.0, 50.0, 0.0));

    // Left Leg
    let l_hip = commands.spawn(SpatialBundle::from_transform(Transform::from_translation(Vec3::new(-15.0, -20.0, 0.0)))).id();
    commands.entity(root).add_child(l_hip);
    spawn_limb(&mut commands, l_hip, Limb::LeftLeg, vec![(40.0, 1.5), (40.0, 1.5)], Vec3::new(-20.0, -100.0, 0.0));

    // Right Leg
    let r_hip = commands.spawn(SpatialBundle::from_transform(Transform::from_translation(Vec3::new(15.0, -20.0, 0.0)))).id();
    commands.entity(root).add_child(r_hip);
    spawn_limb(&mut commands, r_hip, Limb::RightLeg, vec![(40.0, 1.5), (40.0, 1.5)], Vec3::new(20.0, -100.0, 0.0));

    // Instructions
    commands.spawn(
        TextBundle::from_section(
            "State Choreography: Watch the procedural dance.\nStates cycle automatically.",
            TextStyle {
                font_size: 20.0,
                color: Color::from(WHITE),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn spawn_limb(
    commands: &mut Commands,
    parent: Entity,
    limb: Limb,
    bones: Vec<(f32, f32)>, // (Length, AngleLimit)
    target_pos: Vec3,
) {
    let target = commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle { radius: 5.0, center: Vec2::ZERO }),
            spatial: SpatialBundle::from_transform(Transform::from_translation(target_pos)),
            ..default()
        },
        Fill::color(Color::srgba(1.0, 0.0, 0.0, 0.5)),
    )).id();

    let mut joints = Vec::new();
    let mut current_parent = parent;
    let mut prev_length = 0.0;

    for (length, limit) in bones {
        let joint = commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, prev_length, 0.0))),
            Joint {
                min_angle: -limit,
                max_angle: limit,
                ..default()
            },
            Bone { length },
        )).id();

        commands.entity(current_parent).add_child(joint);
        joints.push(joint);
        current_parent = joint;
        prev_length = length;
    }

    // Effector
    let effector = commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, prev_length, 0.0))),
    )).id();
    commands.entity(current_parent).add_child(effector);

    commands.spawn((
        IKChain {
            joints,
            effector,
            target,
            iterations: 10,
        },
        limb
    ));
}


fn draw_skeleton_lines(
    mut gizmos: Gizmos,
    chains: Query<(&IKChain, &Limb)>,
    transforms: Query<&GlobalTransform>,
    parents: Query<&Parent>,
) {
    for (chain, limb) in chains.iter() {
        let color = match limb {
            Limb::LeftArm | Limb::RightArm => Color::from(AQUA),
            Limb::LeftLeg | Limb::RightLeg => Color::from(YELLOW),
            Limb::Head => Color::from(FUCHSIA),
        };

        let mut prev_pos = if let Ok(joint_parent) = parents.get(chain.joints[0]) {
             if let Ok(t) = transforms.get(joint_parent.get()) {
                 t.translation().truncate()
             } else {
                 Vec2::ZERO
             }
        } else {
            Vec2::ZERO
        };

        for &joint in &chain.joints {
            if let Ok(t) = transforms.get(joint) {
                let pos = t.translation().truncate();
                gizmos.line_2d(prev_pos, pos, color);
                gizmos.circle_2d(pos, 3.0, Color::from(GREEN)); // Joint
                prev_pos = pos;
            }
        }

        if let Ok(t) = transforms.get(chain.effector) {
            let pos = t.translation().truncate();
            gizmos.line_2d(prev_pos, pos, color);
            gizmos.circle_2d(pos, 5.0, color); // Effector
        }
    }
}
