use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
#[allow(dead_code)]
pub struct Dancer {
    pub id: usize,
    pub color: Color,
}

#[derive(Component)]
pub struct Limb {
    pub kind: LimbKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum LimbKind {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Component, Default)]
pub struct DancePose {
    pub left_arm_angle: f32,
    pub right_arm_angle: f32,
    pub left_leg_angle: f32,
    pub right_leg_angle: f32,
    pub head_tilt: f32,
    pub torso_bend: f32,
}

#[derive(Component)]
pub struct TargetPosition(pub Vec3);

#[derive(Component)]
pub struct PoseTransition {
    pub speed: f32,
    #[allow(dead_code)]
    pub easing: f32,
}

impl Default for PoseTransition {
    fn default() -> Self {
        Self {
            speed: 5.0,
            easing: 1.0,
        }
    }
}

pub fn spawn_dancer(
    commands: &mut Commands,
    position: Vec3,
    id: usize,
    color: Color,
) -> Entity {
    let shape = shapes::Circle {
        radius: 10.0,
        ..default()
    };

    let root = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            Dancer { id, color },
            DancePose::default(),
            PoseTransition::default(),
            TargetPosition(position),
        ))
        .id();

    // Head
    let head = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 30.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Fill::color(color),
            Stroke::new(Color::WHITE, 2.0),
            Limb { kind: LimbKind::Head },
        ))
        .id();

    // Torso (Line)
    let mut torso_builder = PathBuilder::new();
    torso_builder.move_to(Vec2::new(0.0, 20.0));
    torso_builder.line_to(Vec2::new(0.0, -10.0));
    let torso_path = torso_builder.build();

    let torso = commands
        .spawn((
            ShapeBundle {
                path: torso_path,
                spatial: SpatialBundle::default(),
                ..default()
            },
            Stroke::new(color, 4.0),
            Limb { kind: LimbKind::Torso },
        ))
        .id();

    // Helper to spawn a limb
    let mut spawn_limb = |kind: LimbKind, pos: Vec2, build_path: &dyn Fn() -> Path| {
        commands
            .spawn((
                ShapeBundle {
                    path: build_path(),
                    spatial: SpatialBundle {
                        transform: Transform::from_translation(pos.extend(0.0)),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(color, 3.0),
                Limb { kind: kind },
            ))
            .id()
    };

    let arm_builder = || {
        let mut b = PathBuilder::new();
        b.move_to(Vec2::ZERO);
        b.line_to(Vec2::new(0.0, -20.0));
        b.build()
    };

    let leg_builder = || {
        let mut b = PathBuilder::new();
        b.move_to(Vec2::ZERO);
        b.line_to(Vec2::new(0.0, -25.0));
        b.build()
    };

    let left_arm = spawn_limb(LimbKind::LeftArm, Vec2::new(-5.0, 15.0), &arm_builder);
    let right_arm = spawn_limb(LimbKind::RightArm, Vec2::new(5.0, 15.0), &arm_builder);
    let left_leg = spawn_limb(LimbKind::LeftLeg, Vec2::new(-5.0, -10.0), &leg_builder);
    let right_leg = spawn_limb(LimbKind::RightLeg, Vec2::new(5.0, -10.0), &leg_builder);

    commands.entity(root).push_children(&[head, torso, left_arm, right_arm, left_leg, right_leg]);

    root
}
