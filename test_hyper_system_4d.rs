use hyper_system::Vec4;
use std::f32::consts::PI;

fn main() {
    // 1. Create a point in 4D space
    let point = Vec4::new(1.0, 0.0, 0.0, 0.0);

    // 2. Rotate it in the XW plane (swapping X and W)
    // This is a rotation that doesn't exist in 3D space!
    let rotated = point.rotate_xw(PI / 2.0);

    // 3. Project it down to 3D for rendering
    // We place a "camera" on the W-axis at w=5.0
    let projected_3d = rotated.project_to_3d(5.0);

    println!("Projected: {:?}", projected_3d);
}
