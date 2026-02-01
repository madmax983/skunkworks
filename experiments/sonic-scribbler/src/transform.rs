use crate::font::PathCommand;
use glam::Vec2;

pub type Point = Vec2;

#[derive(Debug, Clone, Copy)]
pub struct AudioState {
    pub amplitude: f32,
    pub frequency: f32,
    pub phase: f32,
}

pub fn distort(commands: &[PathCommand], audio: &AudioState) -> Vec<PathCommand> {
    commands
        .iter()
        .map(|cmd| match cmd {
            PathCommand::MoveTo(p) => PathCommand::MoveTo(warp(*p, audio)),
            PathCommand::LineTo(p) => PathCommand::LineTo(warp(*p, audio)),
            PathCommand::QuadTo(c, e) => PathCommand::QuadTo(warp(*c, audio), warp(*e, audio)),
            PathCommand::CurveTo(c1, c2, e) => {
                PathCommand::CurveTo(warp(*c1, audio), warp(*c2, audio), warp(*e, audio))
            }
            PathCommand::Close => PathCommand::Close,
        })
        .collect()
}

fn warp(p: Point, audio: &AudioState) -> Point {
    // Normalize x somewhat. Font units are often up to 2048.
    // We want the wave to propagate along X.
    let x_norm = p.x * 0.005;

    // Main wave: y displacement
    let dy = (x_norm * audio.frequency + audio.phase).sin() * audio.amplitude * 100.0;

    // Secondary wave: x displacement (compression/expansion)
    let dx = (p.y * 0.005 * audio.frequency + audio.phase * 1.5).cos() * audio.amplitude * 50.0;

    Vec2::new(p.x + dx, p.y + dy)
}

pub fn flatten(commands: &[PathCommand]) -> Vec<(Vec2, Vec2)> {
    let mut lines = Vec::new();
    let mut start = Vec2::ZERO;
    let mut current = Vec2::ZERO;

    for cmd in commands {
        match cmd {
            PathCommand::MoveTo(p) => {
                start = *p;
                current = *p;
            }
            PathCommand::LineTo(p) => {
                lines.push((current, *p));
                current = *p;
            }
            PathCommand::QuadTo(c, e) => {
                let steps = 5;
                let mut prev = current;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let next = quad_bezier(current, *c, *e, t);
                    lines.push((prev, next));
                    prev = next;
                }
                current = *e;
            }
            PathCommand::CurveTo(c1, c2, e) => {
                let steps = 5;
                let mut prev = current;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let next = cubic_bezier(current, *c1, *c2, *e, t);
                    lines.push((prev, next));
                    prev = next;
                }
                current = *e;
            }
            PathCommand::Close => {
                if current != start {
                    lines.push((current, start));
                }
                current = start;
            }
        }
    }
    lines
}

fn quad_bezier(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    p0 * (mt * mt) + p1 * (2.0 * mt * t) + p2 * (t * t)
}

fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;
    p0 * (mt2 * mt) + p1 * (3.0 * mt2 * t) + p2 * (3.0 * mt * t2) + p3 * (t2 * t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::PathCommand;

    #[test]
    fn test_flatten() {
        let cmds = vec![
            PathCommand::MoveTo(Vec2::new(0.0, 0.0)),
            PathCommand::LineTo(Vec2::new(10.0, 10.0)),
        ];
        let lines = flatten(&cmds);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].0, Vec2::new(0.0, 0.0));
        assert_eq!(lines[0].1, Vec2::new(10.0, 10.0));
    }

    #[test]
    fn test_distort() {
        let cmds = vec![PathCommand::MoveTo(Vec2::new(0.0, 0.0))];
        let audio = AudioState {
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
        };
        let distorted = distort(&cmds, &audio);
        if let PathCommand::MoveTo(p) = distorted[0] {
            // With phase 0, sin(0) is 0, cos(0) is 1.
            // dx = cos(0) * 1.0 * 50.0 = 50.0
            // dy = sin(0) * ... = 0.0
            assert!(p.x.abs() > 0.0, "p.x should be modified");
        } else {
            panic!("Wrong command type");
        }
    }
}
