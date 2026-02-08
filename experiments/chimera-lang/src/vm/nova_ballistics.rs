#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub power: i64,
    pub ttl: usize,
    pub owner: usize, // Strand index
    pub last_hit: Option<(usize, usize)>,
}

pub fn update_projectiles(vm: &mut ChimeraVM) {
    let mut surviving_projectiles = Vec::new();

    for mut p in std::mem::take(&mut vm.projectiles) {
        // Update position
        p.x += p.vx;
        p.y += p.vy;
        p.ttl = p.ttl.saturating_sub(1);

        if p.ttl == 0 {
            continue;
        }

        // Check bounds
        if p.x < 0.0 || p.x >= GRID_SIZE as f64 || p.y < 0.0 || p.y >= GRID_SIZE as f64 {
            continue;
        }

        let ix = p.x as usize;
        let iy = p.y as usize;

        // Check collision with Grid
        if !matches!(vm.grid[iy][ix], Value::Int(0)) {
            // Avoid re-triggering same optical component
            if let Some(last) = p.last_hit {
                if last == (iy, ix) {
                    surviving_projectiles.push(p);
                    continue;
                }
            }

            if let Value::Str(s) = &vm.grid[iy][ix] {
                if s.starts_with("MIRROR:") || s.starts_with("REFLECTOR:") {
                    // MIRROR prefix kept for backward compat if any, but new opcode uses REFLECTOR
                    let val_str = if s.starts_with("MIRROR:") {
                        s.trim_start_matches("MIRROR:")
                    } else {
                        s.trim_start_matches("REFLECTOR:")
                    };

                    if let Ok(ori) = val_str.parse::<i32>() {
                        match ori {
                            0 => p.vx = -p.vx, // | Vertical: Reflect X
                            1 => p.vy = -p.vy, // - Horizontal: Reflect Y
                            2 => {
                                // / Diagonal: Swap and negate
                                let temp = p.vx;
                                p.vx = -p.vy;
                                p.vy = -temp;
                            }
                            3 => {
                                // \ Diagonal: Swap
                                std::mem::swap(&mut p.vx, &mut p.vy);
                            }
                            _ => {}
                        }
                        p.last_hit = Some((iy, ix));
                        surviving_projectiles.push(p);
                        continue;
                    }
                } else if s.starts_with("PRISM:") {
                    // Split!
                    let angle_offset = 0.5; // radians approx 30 deg
                    let base_angle = p.vy.atan2(p.vx);
                    let speed = (p.vx * p.vx + p.vy * p.vy).sqrt();

                    // 1. Center (Original)
                    let mut p_center = p.clone();
                    p_center.last_hit = Some((iy, ix));
                    surviving_projectiles.push(p_center);

                    // 2. Left
                    let a1 = base_angle - angle_offset;
                    let p1 = Projectile {
                        x: p.x,
                        y: p.y,
                        vx: speed * a1.cos(),
                        vy: speed * a1.sin(),
                        power: p.power,
                        ttl: p.ttl,
                        owner: p.owner,
                        last_hit: Some((iy, ix)),
                    };
                    surviving_projectiles.push(p1);

                    // 3. Right
                    let a2 = base_angle + angle_offset;
                    let p2 = Projectile {
                        x: p.x,
                        y: p.y,
                        vx: speed * a2.cos(),
                        vy: speed * a2.sin(),
                        power: p.power,
                        ttl: p.ttl,
                        owner: p.owner,
                        last_hit: Some((iy, ix)),
                    };
                    surviving_projectiles.push(p2);
                    continue;
                } else if s.starts_with("LENS:") {
                    if let Ok(pow) = s.trim_start_matches("LENS:").parse::<i64>() {
                        p.power = p.power.max(pow); // Amplify
                        p.ttl += 10; // Refocus/Extend range
                        p.last_hit = Some((iy, ix));
                        surviving_projectiles.push(p);
                        continue;
                    }
                }
            }

            // Impact! (Non-optical or failed optical parse)
            vm.grid[iy][ix] = Value::Int(0); // Destroy block
            vm.output
                .push(format!("IMPACT: Projectile hit {},{}", ix, iy));

            // Explosion effect (damage neighbors if power is high)
            if p.power > 1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        if let Some((ny, nx)) = vm.normalize_coords(iy as i64 + dy, ix as i64 + dx)
                        {
                            if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                                // Simple damage model: reduce value or clear
                                vm.grid[ny][nx] = Value::Int(0);
                            }
                        }
                    }
                }
            }
            continue; // Projectile destroyed
        }

        // Check collision with Organelles
        let mut hit_organelle = false;
        vm.organelles.retain(|org| {
            if hit_organelle {
                return true;
            } // Already hit something this tick
            if org.context_loc == (iy, ix) {
                hit_organelle = true;
                vm.output
                    .push(format!("IMPACT: Organelle hit at {},{}", ix, iy));
                false // Kill organelle
            } else {
                true
            }
        });

        if hit_organelle {
            continue; // Projectile destroyed
        }

        surviving_projectiles.push(p);
    }

    vm.projectiles = surviving_projectiles;
}

pub fn exec_fire(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., power, dy, dx ] (top)
    if vm.stack.len() >= 3 {
        let dx_val = vm.stack.pop().unwrap();
        let dy_val = vm.stack.pop().unwrap();
        let pow_val = vm.stack.pop().unwrap();

        if let (Value::Int(dx), Value::Int(dy), Value::Int(pow)) = (dx_val, dy_val, pow_val) {
            let (cy, cx) = vm.context_loc;

            // Normalize direction vector
            let len = ((dx * dx + dy * dy) as f64).sqrt();
            let (vx, vy) = if len > 0.0 {
                ((dx as f64) / len, (dy as f64) / len)
            } else {
                (0.0, 0.0)
            };

            let p = Projectile {
                x: cx as f64,
                y: cy as f64,
                vx,
                vy,
                power: pow.clamp(1, 5),
                ttl: 20,
                owner: vm.ip.0,
                last_hit: None,
            };

            vm.projectiles.push(p);
            vm.energy = vm.energy.saturating_sub(5 + pow);
            vm.output
                .push(format!("FIRE: Vector ({:.1}, {:.1})", vx, vy));
        } else {
            vm.output.push("Error: Type mismatch for fire".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for fire".to_string());
    }
    None
}

pub fn exec_salvo(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., power, count ]
    if vm.stack.len() >= 2 {
        let count_val = vm.stack.pop().unwrap();
        let pow_val = vm.stack.pop().unwrap();

        if let (Value::Int(count), Value::Int(pow)) = (count_val, pow_val) {
            let (cy, cx) = vm.context_loc;
            let cnt = count.clamp(1, 8);
            let mut rng = rand::thread_rng();

            for _ in 0..cnt {
                let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                let vx = angle.cos();
                let vy = angle.sin();

                let p = Projectile {
                    x: cx as f64,
                    y: cy as f64,
                    vx,
                    vy,
                    power: pow.clamp(1, 5),
                    ttl: 20,
                    owner: vm.ip.0,
                    last_hit: None,
                };
                vm.projectiles.push(p);
            }

            vm.energy = vm.energy.saturating_sub((5 + pow) * cnt);
            vm.output.push(format!("SALVO: Fired {} projectiles", cnt));
        } else {
            vm.output.push("Error: Type mismatch for salvo".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for salvo".to_string());
    }
    None
}
