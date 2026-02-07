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
            // Impact!
            vm.grid[iy][ix] = Value::Int(0); // Destroy block
            vm.output.push(format!("IMPACT: Projectile hit {},{}", ix, iy));

            // Explosion effect (damage neighbors if power is high)
            if p.power > 1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 { continue; }
                        if let Some((ny, nx)) = vm.normalize_coords(iy as i64 + dy, ix as i64 + dx) {
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
            if hit_organelle { return true; } // Already hit something this tick
            if org.context_loc == (iy, ix) {
                hit_organelle = true;
                vm.output.push(format!("IMPACT: Organelle hit at {},{}", ix, iy));
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
            let len = ((dx*dx + dy*dy) as f64).sqrt();
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
            };

            vm.projectiles.push(p);
            vm.energy = vm.energy.saturating_sub(5 + pow);
            vm.output.push(format!("FIRE: Vector ({:.1}, {:.1})", vx, vy));
        } else {
            vm.output.push("Error: Type mismatch for fire".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for fire".to_string());
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
                };
                vm.projectiles.push(p);
            }

            vm.energy = vm.energy.saturating_sub((5 + pow) * cnt);
            vm.output.push(format!("SALVO: Fired {} projectiles", cnt));
        } else {
            vm.output.push("Error: Type mismatch for salvo".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for salvo".to_string());
    }
    None
}
