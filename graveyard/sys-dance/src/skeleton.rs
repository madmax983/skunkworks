use bevy::prelude::*;

#[derive(Component)]
pub struct SkeletonRoot;

#[derive(Component)]
pub struct Bone {
    pub length: f32,
    pub thickness: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct IKChain {
    pub target: Vec2,
    pub bone1: Entity, // Upper Arm / Thigh
    pub bone2: Entity, // Lower Arm / Calf
    pub len1: f32,
    pub len2: f32,
    pub bend_dir: f32, // 1.0 or -1.0
}
