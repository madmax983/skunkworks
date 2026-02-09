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
pub struct CodeNode {
    pub name: String,
    pub node_type: NodeType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeType {
    Function,
    Variable,
    ControlFlow,
    Root,
}

#[derive(Component)]
pub struct ChainRoot;
