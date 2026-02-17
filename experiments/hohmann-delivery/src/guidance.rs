use crate::physics::Body;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct TransferWindow {
    pub delta_v_departure: f32,
    pub delta_v_arrival: f32,
    pub time_of_flight: f32,
    pub phase_angle_required: f32,
}

pub fn calculate_hohmann_transfer(
    origin: &Body,
    destination: &Body,
    gm: f32,
) -> Option<TransferWindow> {
    let pos1 = origin.pos;
    let pos2 = destination.pos;
    let r1 = pos1.length();
    let r2 = pos2.length();

    if r1 < 0.001 || r2 < 0.001 {
        return None;
    }

    // Semi-major axis of transfer orbit
    let a_transfer = (r1 + r2) / 2.0;

    // Velocities
    let v_c1 = (gm / r1).sqrt();
    let v_c2 = (gm / r2).sqrt();

    // Velocity at periapsis/apoapsis of transfer orbit
    // Vis-viva equation: v^2 = GM (2/r - 1/a)
    let v_transfer_at_r1 = (gm * (2.0 / r1 - 1.0 / a_transfer)).sqrt();
    let v_transfer_at_r2 = (gm * (2.0 / r2 - 1.0 / a_transfer)).sqrt();

    // Delta V
    let delta_v1 = (v_transfer_at_r1 - v_c1).abs();
    let delta_v2 = (v_c2 - v_transfer_at_r2).abs();

    // Time of flight (half period)
    let t_transfer = PI * (a_transfer.powi(3) / gm).sqrt();

    // Required Phase Angle
    // Angle covered by target planet during transfer
    let mean_motion_target = (gm / r2.powi(3)).sqrt();
    let angle_target_travel = mean_motion_target * t_transfer;

    // For outward transfer, ship travels 180 degrees (PI).
    // Target travels angle_target_travel.
    // If we want them to meet at 180 deg, target must start at 180 - angle_target_travel relative to origin.
    // Formula: phi = PI - angle_target_travel
    let mut phase_angle_required = PI - angle_target_travel;

    // Normalize phase angle to (-PI, PI]
    while phase_angle_required > PI {
        phase_angle_required -= 2.0 * PI;
    }
    while phase_angle_required <= -PI {
        phase_angle_required += 2.0 * PI;
    }

    Some(TransferWindow {
        delta_v_departure: delta_v1,
        delta_v_arrival: delta_v2,
        time_of_flight: t_transfer,
        phase_angle_required,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hohmann_earth_mars() {
        let earth = Body::new(vec2(1.0, 0.0), vec2(0.0, 1.0), 1.0, 1.0, BLUE);
        let mars = Body::new(
            vec2(1.524, 0.0),
            vec2(0.0, (1.0 / 1.524f32).sqrt()),
            0.1,
            0.5,
            RED,
        );

        let gm = 1.0;

        let window = calculate_hohmann_transfer(&earth, &mars, gm).expect("Should return a window");

        // Expected values (approx)
        // Delta V1 approx 0.099
        println!("Delta V1: {}", window.delta_v_departure);
        assert!((window.delta_v_departure - 0.099).abs() < 0.01);

        // Delta V2 approx 0.088 (wait, previous calc was 0.089?)
        // Let's recheck: v_c2 = sqrt(1/1.524) = 0.810.
        // v_apogee = sqrt(1 * (2/1.524 - 1/1.262)) = 0.721.
        // diff = 0.089.
        println!("Delta V2: {}", window.delta_v_arrival);
        assert!((window.delta_v_arrival - 0.088).abs() < 0.01); // 0.01 tolerance allows 0.089
    }
}
