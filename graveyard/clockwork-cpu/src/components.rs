use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Gear {
    pub teeth: u32,
    pub radius: f32,
}

#[derive(Component)]
pub struct EscapeWheel; // Marker for the main clock wheel

#[derive(Component)]
pub struct Verge;

#[derive(Component)]
pub struct Foliot {
    pub inertia: f32,
}

#[derive(Component)]
pub struct Cam;

#[derive(Component)]
pub struct CpuLever {
    pub label: String,
    pub active_angle_range: (f32, f32), // Range in radians where this lever is lifted
}

#[derive(Component)]
pub struct CpuStateDisplay;
