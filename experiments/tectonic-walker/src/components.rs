use bevy::prelude::*;

#[derive(Component)]
pub struct Bone {
    pub length: f32,
}

#[derive(Component)]
pub struct Joint {
    pub angle: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}

#[derive(Component)]
pub struct Effector;

#[derive(Component)]
pub struct IKTarget;

#[derive(Component)]
pub struct CommitNode {
    pub hash: String,
    pub stress_level: f32,
    pub index: usize,
}

#[derive(Component)]
pub struct ChainRoot;
