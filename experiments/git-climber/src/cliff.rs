use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::process::Command;
use rand::Rng;

#[derive(Component)]
pub struct Cliff;

#[derive(Component)]
#[allow(dead_code)]
pub struct CommitLedge {
    pub hash: String,
    pub message: String,
}

pub fn generate_cliff(mut commands: Commands) {
    let output = Command::new("git")
        .args(&["log", "--oneline", "-n", "50"])
        .output();

    let commits_str = match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).to_string()
        }
        _ => {
            // Fallback for no git repo
            let mut s = String::new();
            let mut rng = rand::thread_rng();
            for i in 0..50 {
                s.push_str(&format!("{:x} Commit #{}\n", rng.gen::<u32>(), i));
            }
            s
        }
    };

    let commits = parse_git_log(&commits_str);

    let mut y_pos = -200.0;
    let mut rng = rand::thread_rng();

    for (hash, message) in commits {
        let width = 100.0 + (message.len() as f32 * 5.0).min(300.0);
        let height = 20.0;
        let x_offset = rng.gen_range(-50.0..50.0);

        // Visuals
        let shape = shapes::Rectangle {
            extents: Vec2::new(width, height),
            origin: RectangleOrigin::Center,
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(x_offset, y_pos, 0.0)),
                ..default()
            },
            Fill::color(Color::srgba(0.3, 0.3, 0.35, 1.0)),
            Stroke::new(Color::BLACK, 2.0),
            Collider::cuboid(width / 2.0, height / 2.0),
            CommitLedge { hash, message },
            Cliff,
        ));

        // Gap between ledges
        y_pos += 80.0 + rng.gen_range(0.0..40.0);
    }

    // Ground
    let ground_width = 1000.0;
    let ground_height = 50.0;
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(ground_width, ground_height),
                origin: RectangleOrigin::Center,
            }),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, -250.0, 0.0)),
            ..default()
        },
        Fill::color(Color::srgba(0.1, 0.1, 0.1, 1.0)),
        Collider::cuboid(ground_width / 2.0, ground_height / 2.0),
    ));
}

fn parse_git_log(log: &str) -> Vec<(String, String)> {
    log.lines()
        .rev()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() < 2 {
                None
            } else {
                Some((parts[0].to_string(), parts[1].to_string()))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_log() {
        let input = "a1b2c3d Initial commit\ne4f5g6h Second commit";
        let parsed = parse_git_log(input);

        // Reversed order
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].0, "e4f5g6h");
        assert_eq!(parsed[0].1, "Second commit");
        assert_eq!(parsed[1].0, "a1b2c3d");
        assert_eq!(parsed[1].1, "Initial commit");
    }
}
