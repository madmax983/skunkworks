use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Bone {
    pub length: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Joint {
    pub current_angle: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}

impl Default for Joint {
    fn default() -> Self {
        Self {
            current_angle: 0.0,
            min_angle: -std::f32::consts::PI,
            max_angle: std::f32::consts::PI,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct IKChain {
    pub joints: Vec<Entity>, // Ordered: Root -> Leaf
    pub effector: Entity,
    pub target: Entity,
    pub iterations: usize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Limb {
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    Head,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabanState {
    Idle,
    ReachHigh,
    Crouch,
    TPose,
    Arabesque,
    Jump,
}

#[derive(Resource, Debug)]
pub struct Choreographer {
    pub current_state: LabanState,
    pub transition_timer: Timer,
    pub next_state: LabanState,
    pub state_start_time: f32,
}
