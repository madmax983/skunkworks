#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use rand::Rng;
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::f64::consts::PI;

#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    pub angle: f64, // Radians
    pub dist: f64,  // 0.0 (center) to 1.0 (edge)
    pub color: u32, // RGB packed
    pub power: i64,
    pub name: String,
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sky {
    pub stars: Vec<Star>,
    pub rotation: f64,
    pub tick_rate: f64,
}

#[cfg(feature = "nova")]
impl Default for Sky {
    fn default() -> Self {
        Self::new()
    }
}

impl Sky {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::new();

        // Generate 12 zodiac stars + random ones
        for i in 0..12 {
            stars.push(Star {
                angle: (i as f64) * (PI / 6.0),
                dist: rng.gen_range(0.5..0.9),
                color: rng.gen::<u32>() & 0xFFFFFF,
                power: rng.gen_range(50..200),
                name: format!("Zodiac-{}", i),
            });
        }

        // Random stars
        for i in 0..20 {
            stars.push(Star {
                angle: rng.gen_range(0.0..2.0 * PI),
                dist: rng.gen_range(0.1..1.0),
                color: rng.gen::<u32>() & 0xFFFFFF,
                power: rng.gen_range(10..100),
                name: format!("Star-{}", i),
            });
        }

        Self {
            stars,
            rotation: 0.0,
            tick_rate: 0.01, // ~0.57 degrees per tick
        }
    }

    pub fn tick(&mut self) {
        self.rotation += self.tick_rate;
        if self.rotation > 2.0 * PI {
            self.rotation -= 2.0 * PI;
        }
    }

    /// Projects a star's position onto the grid.
    /// Returns (y, x) if within bounds.
    pub fn project(&self, star: &Star) -> Option<(usize, usize)> {
        let angle = star.angle + self.rotation;
        let center = 7.5;
        let scale = 8.0; // Radius of 8 cells

        let x = center + star.dist * scale * angle.cos();
        let y = center + star.dist * scale * angle.sin();

        let ix = x.round() as i64;
        let iy = y.round() as i64;

        if (0..16).contains(&ix) && (0..16).contains(&iy) {
            Some((iy as usize, ix as usize))
        } else {
            None
        }
    }
}

/// **OpCode:** `Gaze`
/// **Stack:** `[ ... ] -> [ ..., intensity, color ]`
#[cfg(feature = "nova")]
pub fn exec_gaze(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let mut total_power = 0;
    let mut dom_color = 0;
    let mut max_power = 0;

    for star in &vm.sky.stars {
        if let Some((sy, sx)) = vm.sky.project(star) {
            if sy == cy && sx == cx {
                total_power += star.power;
                if star.power > max_power {
                    max_power = star.power;
                    dom_color = star.color;
                }
            }
        }
    }

    vm.stack.push(Value::Int(total_power));
    vm.stack.push(Value::Int(dom_color as i64));

    // Gazing costs time but grants insight
    vm.energy = vm.energy.saturating_sub(1);

    None
}

/// **OpCode:** `Starfall`
/// **Stack:** `[ ... ] -> [ ... ]`
#[cfg(feature = "nova")]
pub fn exec_starfall(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let mut hit = false;
    let mut star_name = String::new();
    let mut power = 0;

    // Find star overhead
    for star in &vm.sky.stars {
        if let Some((sy, sx)) = vm.sky.project(star) {
            if sy == cy && sx == cx {
                hit = true;
                star_name = star.name.clone();
                power = star.power;
                break;
            }
        }
    }

    if hit {
        vm.energy = vm.energy.saturating_sub(50); // High cost to summon
        vm.output.push(format!(
            "STARFALL: Summoned {} with power {}",
            star_name, power
        ));

        // Effect: Massive damage + terraform
        // 1. Crater (Value 0)
        vm.grid[cy][cx] = Value::Int(0);

        // 2. Shockwave (Damage nearby organisms/organelles not implemented yet, just clear grid)
        for dy in -1..=1 {
            for dx in -1..=1 {
                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    vm.entropy_grid[ny][nx] += 20;
                    if power > 100 {
                        vm.grid[ny][nx] = Value::Int(0);
                    }
                }
            }
        }
    } else {
        vm.output.push("STARFALL: No star overhead".to_string());
    }

    None
}

/// **OpCode:** `Align`
/// **Stack:** `[ ... ] -> [ ..., angle_to_nearest ]`
#[cfg(feature = "nova")]
pub fn exec_align(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let center = 7.5;

    // Calculate angle of current position relative to center
    let dx = (cx as f64) - center;
    let dy = (cy as f64) - center;
    let my_angle = dy.atan2(dx); // -PI to PI

    // Find nearest star angle
    let mut min_diff = f64::MAX;
    let mut target_angle = 0.0;

    for star in &vm.sky.stars {
        let star_angle = (star.angle + vm.sky.rotation).rem_euclid(2.0 * PI);
        // Normalize my_angle to 0..2PI
        let my_angle_norm = my_angle.rem_euclid(2.0 * PI);

        let diff = (star_angle - my_angle_norm)
            .abs()
            .min(2.0 * PI - (star_angle - my_angle_norm).abs());

        if diff < min_diff {
            min_diff = diff;
            target_angle = star_angle;
        }
    }

    // Return angle in degrees (0-360)
    let degrees = (target_angle * 180.0 / PI) as i64;
    vm.stack.push(Value::Int(degrees));

    None
}
