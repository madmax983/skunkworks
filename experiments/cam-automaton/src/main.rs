use macroquad::prelude::*;
use nalgebra::Vector2 as Vec2N;
use rapier2d::prelude::*;

mod mechanism;
mod physics;
mod puppet;

use mechanism::Mechanism;
use physics::PhysicsWorld;
use puppet::Puppet;

#[macroquad::main("Cam Automaton")]
async fn main() {
    let mut world = PhysicsWorld::new();

    // 1. Setup Camshaft
    let shaft_pos = Vec2N::new(0.0, -5.0);
    let shaft = Mechanism::create_camshaft(&mut world, shaft_pos);

    // Cam 1 (Left Leg Driver) - Simple Eccentric
    let cam_shape = SharedShape::ball(2.0);
    Mechanism::add_cam(&mut world, shaft, cam_shape, Vec2N::new(0.0, 1.0));

    // Cam 2 (Right Leg Driver) - Opposite Phase
    let cam_shape2 = SharedShape::ball(2.0);
    Mechanism::add_cam(&mut world, shaft, cam_shape2, Vec2N::new(0.0, -1.0));

    // 2. Setup Followers
    // Left Follower
    let f1 = Mechanism::create_follower(&mut world, -2.0, -2.0);
    // Right Follower
    let f2 = Mechanism::create_follower(&mut world, 2.0, -2.0);

    // 3. Setup Puppet
    let puppet = Puppet::spawn(&mut world, 0.0, 5.0);

    // 4. Connect Linkages
    Puppet::attach_rod(&mut world, f1, puppet.left_leg);
    Puppet::attach_rod(&mut world, f2, puppet.right_leg);

    // 5. Main Loop
    loop {
        clear_background(LIGHTGRAY);

        // Update Camshaft Rotation
        let time = get_time() as f32;
        let speed = 2.0;
        let angle = time * speed;

        if let Some(body) = world.rigid_body_set.get_mut(shaft) {
            body.set_next_kinematic_rotation(Rotation::new(angle));
        }

        // Step Physics
        world.step();

        // Render
        set_camera(&Camera2D {
            zoom: vec2(0.05, 0.05), // Zoom out
            target: vec2(0.0, 0.0),
            ..Default::default()
        });

        // Draw Ground/Reference
        draw_line(-10.0, -5.0, 10.0, -5.0, 0.1, BLACK);

        // Iterate bodies and draw
        for (_handle, body) in world.rigid_body_set.iter() {
            // Draw Colliders
            for collider_handle in body.colliders() {
                if let Some(collider) = world.collider_set.get(*collider_handle) {
                    let shape = collider.shared_shape();
                    let iso = collider.position_wrt_parent().unwrap();
                    let collider_pos = body.position() * iso;
                    let c_pos = collider_pos.translation.vector;
                    let c_rot = collider_pos.rotation.angle();

                    if let Some(ball) = shape.as_ball() {
                        draw_circle(c_pos.x, c_pos.y, ball.radius, RED);
                        // Draw a line to show rotation
                        let end_x = c_pos.x + ball.radius * c_rot.cos();
                        let end_y = c_pos.y + ball.radius * c_rot.sin();
                        draw_line(c_pos.x, c_pos.y, end_x, end_y, 0.1, BLACK);
                    } else if let Some(cuboid) = shape.as_cuboid() {
                        let w = cuboid.half_extents.x * 2.0;
                        let h = cuboid.half_extents.y * 2.0;

                        // draw_rectangle_ex centers at top-left by default, but we can change offset
                        // We need to draw at center position
                        // macroquad x,y is usually top-left unless transformed
                        // But wait, draw_rectangle_ex takes x,y as center if we use params?
                        // No, docs say "Draws a rectangle with the given position and size."
                        // Usually position is top-left.
                        // We want center. So x - w/2, y - h/2.
                        // But we also have rotation.
                        // draw_rectangle_ex applies rotation around the center if we set offset?
                        // Let's use `draw_poly`? No.
                        // Let's use `draw_rectangle_ex` with `offset: vec2(0.5, 0.5)`.
                        // If offset is (0.5, 0.5), the x,y argument is the center.

                        draw_rectangle_ex(
                            c_pos.x,
                            c_pos.y,
                            w,
                            h,
                            DrawRectangleParams {
                                offset: vec2(0.5, 0.5),
                                rotation: c_rot,
                                color: BLUE,
                            },
                        );
                    }
                }
            }
        }

        // Draw Joints/Rods
        for (_handle, joint) in world.impulse_joint_set.iter() {
            let b1 = world.rigid_body_set[joint.body1].translation();
            let b2 = world.rigid_body_set[joint.body2].translation();
            draw_line(b1.x, b1.y, b2.x, b2.y, 0.05, GREEN);
        }

        next_frame().await
    }
}
