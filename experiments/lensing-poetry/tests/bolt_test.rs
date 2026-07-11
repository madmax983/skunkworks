use lensing_poetry::physics::{integrate, Body};
use macroquad::prelude::*;

#[test]
fn test_bolt_zero_alloc() {
    let mut bodies_orig = vec![
        Body {
            pos: vec2(0.0, 0.0),
            vel: vec2(1.0, 0.0),
            acc: Vec2::ZERO,
            mass: 10.0,
            radius: 1.0,
            color: WHITE,
        },
        Body {
            pos: vec2(10.0, 0.0),
            vel: vec2(0.0, 1.0),
            acc: Vec2::ZERO,
            mass: 5.0,
            radius: 1.0,
            color: WHITE,
        },
        Body {
            pos: vec2(0.0, 10.0),
            vel: vec2(-1.0, 0.0),
            acc: Vec2::ZERO,
            mass: 5.0,
            radius: 1.0,
            color: WHITE,
        },
    ];
    let mut bodies_new = bodies_orig.clone();

    // Step original manually
    let mut accelerations = [Vec2::ZERO; 3];
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            let r_vec = bodies_orig[j].pos - bodies_orig[i].pos;
            let dist_sq = r_vec.length_squared();
            let dist_soft = (dist_sq + 0.0001 * 0.0001).sqrt();
            let dist_cubed = dist_soft * dist_soft * dist_soft;
            let f = 1.0 * bodies_orig[j].mass / dist_cubed;
            accelerations[i] += r_vec * f;
        }
    }
    for i in 0..3 {
        let dv = accelerations[i] * 0.1;
        bodies_orig[i].vel += dv;
        let dp = bodies_orig[i].vel * 0.1;
        bodies_orig[i].pos += dp;
    }

    // Step new
    integrate(&mut bodies_new, 0.1);

    for i in 0..3 {
        assert!((bodies_orig[i].pos.x - bodies_new[i].pos.x).abs() < f32::EPSILON);
        assert!((bodies_orig[i].pos.y - bodies_new[i].pos.y).abs() < f32::EPSILON);
        assert!((bodies_orig[i].vel.x - bodies_new[i].vel.x).abs() < f32::EPSILON);
        assert!((bodies_orig[i].vel.y - bodies_new[i].vel.y).abs() < f32::EPSILON);
    }
}
