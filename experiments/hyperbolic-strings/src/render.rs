use ratatui::{
    style::Color,
    widgets::canvas::{Context, Line},
};
use poincare_disk::{Geodesic, Point, Mobius};
use crate::physics::HyperbolicString;

pub struct Camera {
    pub transform: Mobius,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Mobius::identity(),
        }
    }
}

pub fn draw_hyperbolic_string(ctx: &mut Context, string: &HyperbolicString, camera: &Camera) {
    let view_transform = camera.transform.inverse();

    // Draw boundary circle
    draw_circle(ctx, 0.0, 0.0, 1.0, Color::DarkGray);

    for i in 0..string.nodes.len() - 1 {
        let p1_world = string.nodes[i].pos;
        let p2_world = string.nodes[i+1].pos;

        // Apply camera view transform
        let p1 = view_transform.apply(p1_world);
        let p2 = view_transform.apply(p2_world);

        // Cull if outside visible range? (Not needed for disk model, everything is in |z|<1)

        draw_hyperbolic_segment(ctx, p1, p2, Color::Cyan);
    }

    // Draw nodes
    for node in &string.nodes {
        let p = view_transform.apply(node.pos);
        ctx.print(p.re, p.im, ".");
    }
}

fn draw_hyperbolic_segment(ctx: &mut Context, p1: Point, p2: Point, color: Color) {
    let geo = Geodesic::new(p1, p2);

    if let Some((center, radius)) = geo.euclidean_circle() {
        // Draw arc from p1 to p2
        // We need angles of p1 and p2 relative to center
        let ang1 = (p1 - center).arg();
        let ang2 = (p2 - center).arg();

        // Determine direction and sweep
        let mut diff = ang2 - ang1;
        while diff > std::f64::consts::PI {
            diff -= 2.0 * std::f64::consts::PI;
        }
        while diff < -std::f64::consts::PI {
            diff += 2.0 * std::f64::consts::PI;
        }

        let steps = 20;
        for i in 0..steps {
            let t1 = ang1 + diff * (i as f64) / (steps as f64);
            let t2 = ang1 + diff * ((i + 1) as f64) / (steps as f64);

            let x1 = center.re + radius * t1.cos();
            let y1 = center.im + radius * t1.sin();
            let x2 = center.re + radius * t2.cos();
            let y2 = center.im + radius * t2.sin();

            ctx.draw(&Line {
                x1,
                y1,
                x2,
                y2,
                color,
            });
        }
    } else {
        // Straight line
        ctx.draw(&Line {
            x1: p1.re,
            y1: p1.im,
            x2: p2.re,
            y2: p2.im,
            color,
        });
    }
}

fn draw_circle(ctx: &mut Context, cx: f64, cy: f64, r: f64, color: Color) {
    let steps = 64;
    for i in 0..steps {
        let t1 = 2.0 * std::f64::consts::PI * (i as f64) / (steps as f64);
        let t2 = 2.0 * std::f64::consts::PI * ((i + 1) as f64) / (steps as f64);
        let x1 = cx + r * t1.cos();
        let y1 = cy + r * t1.sin();
        let x2 = cx + r * t2.cos();
        let y2 = cy + r * t2.sin();
        ctx.draw(&Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });
    }
}
