use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct LorenzParams {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Default for LorenzParams {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

pub fn lorenz_step(pos: Vec3, params: LorenzParams, dt: f32) -> Vec3 {
    let dx = params.sigma * (pos.y - pos.x);
    let dy = pos.x * (params.rho - pos.z) - pos.y;
    let dz = pos.x * pos.y - params.beta * pos.z;

    vec3(dx, dy, dz) * dt
}

pub fn lorenz_velocity(pos: Vec3, params: LorenzParams) -> Vec3 {
    let dx = params.sigma * (pos.y - pos.x);
    let dy = pos.x * (params.rho - pos.z) - pos.y;
    let dz = pos.x * pos.y - params.beta * pos.z;

    vec3(dx, dy, dz)
}
