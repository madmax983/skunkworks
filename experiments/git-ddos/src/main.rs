use ::rand::Rng;
use anyhow::Result;
use git2::Repository;
use macroquad::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const MAX_PACKETS: usize = 2000;

#[derive(Clone, Copy)]
struct Packet {
    pos: Vec2,
    vel: Vec2,
    active: bool,
    target_idx: Option<usize>,
}

#[derive(Clone)]
struct FileTarget {
    path: String,
    pos: Vec2,
    radius: f32,
    commits: usize,
}

struct Firewall {
    pos: Vec2,
    radius: f32,
    health: f32,
}

fn parse_git_history() -> Result<Vec<FileTarget>> {
    let repo = Repository::discover(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut file_counts: HashMap<String, usize> = HashMap::new();

    for oid in revwalk.take(500) {
        if let Ok(oid) = oid {
            if let Ok(commit) = repo.find_commit(oid) {
                if commit.parent_count() > 0 {
                    if let Ok(parent) = commit.parent(0) {
                        if let (Ok(tree), Ok(parent_tree)) = (commit.tree(), parent.tree()) {
                            if let Ok(diff) =
                                repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)
                            {
                                let _ = diff.print(git2::DiffFormat::NameOnly, |delta, _, _| {
                                    if let Some(path) = delta.new_file().path() {
                                        let path_str = path.to_string_lossy().to_string();
                                        *file_counts.entry(path_str).or_insert(0) += 1;
                                    }
                                    true
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let mut targets = Vec::new();
    let cx = WIDTH / 2.0;
    let cy = HEIGHT / 2.0;

    // Distribute targets in a circle around the center
    let max_commits = file_counts.values().copied().max().unwrap_or(1) as f32;
    let n = file_counts.len();
    for (i, (path, commits)) in file_counts.into_iter().enumerate() {
        let angle = (i as f32 / n as f32) * std::f32::consts::TAU;
        // Hotter files are closer to the center, colder are farther out
        let norm = commits as f32 / max_commits;
        let dist = 300.0 - (norm * 250.0);

        let pos = vec2(cx + angle.cos() * dist, cy + angle.sin() * dist);

        let radius = 5.0 + (commits as f32).min(30.0);

        targets.push(FileTarget {
            path,
            pos,
            radius,
            commits,
        });
    }

    // Sort by commits descending so we render big ones first or last depending on need
    targets.sort_by(|a, b| b.commits.cmp(&a.commits));

    Ok(targets)
}

#[macroquad::main("Git DDoS 🧬")]
async fn main() {
    let mut targets = parse_git_history().unwrap_or_default();
    if targets.is_empty() {
        // Fallback if no git repo
        targets.push(FileTarget {
            path: "Dummy".into(),
            pos: vec2(WIDTH / 2.0, HEIGHT / 2.0),
            radius: 40.0,
            commits: 10,
        });
    }

    let mut packets = vec![
        Packet {
            pos: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            active: false,
            target_idx: None,
        };
        MAX_PACKETS
    ];

    let mut rng = ::rand::thread_rng();
    let mut firewalls = Vec::new();

    for _ in 0..10 {
        firewalls.push(Firewall {
            pos: vec2(
                rng.gen_range(100.0..WIDTH - 100.0),
                rng.gen_range(100.0..HEIGHT - 100.0),
            ),
            radius: rng.gen_range(20.0..60.0),
            health: 100.0,
        });
    }

    let mut rng = ::rand::thread_rng();
    let spawn_point = vec2(50.0, 50.0);

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        // Spawn packets
        let mut to_spawn = 10;
        for p in packets.iter_mut() {
            if !p.active && to_spawn > 0 {
                p.pos = spawn_point + vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
                p.vel = vec2(rng.gen_range(1.0..3.0), rng.gen_range(1.0..3.0));
                p.active = true;

                // Assign target based on weight (commits)
                if !targets.is_empty() {
                    let total_commits: usize = targets.iter().map(|t| t.commits).sum();
                    if total_commits > 0 {
                        let mut pick = rng.gen_range(0..total_commits);
                        let mut selected = 0;
                        for (i, t) in targets.iter().enumerate() {
                            if pick < t.commits {
                                selected = i;
                                break;
                            }
                            pick -= t.commits;
                        }
                        p.target_idx = Some(selected);
                    } else {
                        p.target_idx = Some(rng.gen_range(0..targets.len()));
                    }
                }

                to_spawn -= 1;
            }
        }

        // Parallel update
        let fw_clone = firewalls
            .iter()
            .map(|f| (f.pos, f.radius))
            .collect::<Vec<_>>();
        let targets_clone = targets.clone();

        packets.par_iter_mut().for_each(|p| {
            if !p.active {
                return;
            }

            if let Some(t_idx) = p.target_idx {
                if let Some(target) = targets_clone.get(t_idx) {
                    let to_target = target.pos - p.pos;
                    let dist = to_target.length();

                    if dist < target.radius {
                        p.active = false;
                        return;
                    }

                    let desired_vel = to_target.normalize() * 4.0;
                    let steer = desired_vel - p.vel;
                    p.vel += steer * 0.05;
                }
            }

            // Repel from firewalls
            for fw in &fw_clone {
                let to_fw = p.pos - fw.0;
                let dist = to_fw.length();
                if dist < fw.1 + 10.0 {
                    let repel = to_fw.normalize() * (fw.1 + 10.0 - dist) * 0.5;
                    p.vel += repel;
                }
            }

            // Speed limit
            if p.vel.length() > 5.0 {
                p.vel = p.vel.normalize() * 5.0;
            }

            p.pos += p.vel;

            if p.pos.x < -100.0
                || p.pos.x > WIDTH + 100.0
                || p.pos.y < -100.0
                || p.pos.y > HEIGHT + 100.0
            {
                p.active = false;
            }
        });

        // Firewalls degrade if hit by too many packets
        for fw in firewalls.iter_mut() {
            let hits = packets
                .iter()
                .filter(|p| p.active && p.pos.distance(fw.pos) < fw.radius + 15.0)
                .count();
            if hits > 0 {
                fw.health -= hits as f32 * 0.1;
                fw.radius = (fw.radius - 0.1).max(0.0);
            }
        }
        firewalls.retain(|f| f.health > 0.0);

        // Draw targets (codebase)
        for t in &targets {
            draw_circle(t.pos.x, t.pos.y, t.radius, Color::new(0.0, 0.8, 0.3, 0.4));
            draw_circle_lines(
                t.pos.x,
                t.pos.y,
                t.radius,
                1.0,
                Color::new(0.0, 1.0, 0.4, 0.8),
            );

            // Only draw text if it's a big target to avoid clutter
            if t.radius > 15.0 {
                let text_size = measure_text(&t.path, None, 16, 1.0);
                draw_text(
                    &t.path,
                    t.pos.x - text_size.width / 2.0,
                    t.pos.y + t.radius + 15.0,
                    16.0,
                    WHITE,
                );
            }
        }

        // Draw firewalls
        for fw in &firewalls {
            let c = Color::new(1.0, 0.2, 0.2, fw.health / 100.0);
            draw_circle(fw.pos.x, fw.pos.y, fw.radius, c);
            draw_circle_lines(fw.pos.x, fw.pos.y, fw.radius, 2.0, RED);
        }

        // Draw packets
        for p in &packets {
            if p.active {
                draw_circle(p.pos.x, p.pos.y, 2.0, Color::new(0.2, 0.6, 1.0, 0.8));
            }
        }

        next_frame().await;
    }
}
