use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

pub fn apply_altar_runes(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune != "⛩" {
        return;
    }

    // Check for Elemental Convergence Ritual
    // Pattern:
    //   Δ (Fire)
    // ◊ ⛩ ○ (Earth, Altar, Air)
    //   ∇ (Water)

    // Coordinates:
    // N: y-1, x
    // S: y+1, x
    // W: y, x-1
    // E: y, x+1

    let n_pos = normalize_coords(y as i64 - 1, x as i64);
    let s_pos = normalize_coords(y as i64 + 1, x as i64);
    let w_pos = normalize_coords(y as i64, x as i64 - 1);
    let e_pos = normalize_coords(y as i64, x as i64 + 1);

    if let (Some((ny, nx)), Some((sy, sx)), Some((wy, wx)), Some((ey, ex))) = (n_pos, s_pos, w_pos, e_pos) {
        let n_val = &vm.grid[ny][nx];
        let s_val = &vm.grid[sy][sx];
        let w_val = &vm.grid[wy][wx];
        let e_val = &vm.grid[ey][ex];

        let has_fire = match n_val { Value::Str(s) => s == "Δ", _ => false };
        let has_water = match s_val { Value::Str(s) => s == "∇", _ => false };
        let has_earth = match w_val { Value::Str(s) => s == "◊", _ => false };
        let has_air = match e_val { Value::Str(s) => s == "○", _ => false };

        if has_fire && has_water && has_earth && has_air {
            // Consume Elements
            vm.grid[ny][nx] = Value::Int(0);
            vm.grid[sy][sx] = Value::Int(0);
            vm.grid[wy][wx] = Value::Int(0);
            vm.grid[ey][ex] = Value::Int(0);

            // Effect: Summon Spirit (Φ)
            // Spawn at Altar location (overwrite Altar?) or adjacent?
            // Let's spawn a "Spirit" Agent at the Altar's location.
            // This replaces the Altar.
            vm.grid[y][x] = Value::Str("Φ".to_string());

            // Also grant massive energy
            vm.energy = vm.energy.saturating_add(500);

            vm.output.push(format!("ALTAR: Elemental Convergence at {},{}! Spirit Summoned.", x, y));
            return;
        }
    }

    // Check for Void Ritual
    // Pattern: Surrounded by Void (Ø)
    if let (Some((ny, nx)), Some((sy, sx)), Some((wy, wx)), Some((ey, ex))) = (n_pos, s_pos, w_pos, e_pos) {
        let n_val = &vm.grid[ny][nx];
        let s_val = &vm.grid[sy][sx];
        let w_val = &vm.grid[wy][wx];
        let e_val = &vm.grid[ey][ex];

        let n_void = match n_val { Value::Str(s) => s == "Ø", _ => false };
        let s_void = match s_val { Value::Str(s) => s == "Ø", _ => false };
        let w_void = match w_val { Value::Str(s) => s == "Ø", _ => false };
        let e_void = match e_val { Value::Str(s) => s == "Ø", _ => false };

        if n_void && s_void && w_void && e_void {
             // Consume Voids
            vm.grid[ny][nx] = Value::Int(0);
            vm.grid[sy][sx] = Value::Int(0);
            vm.grid[wy][wx] = Value::Int(0);
            vm.grid[ey][ex] = Value::Int(0);

            // Effect: Open Rift (Value::Str("ꝏ")) or just clear area
            vm.grid[y][x] = Value::Str("ꝏ".to_string()); // Infinity/Portal

            vm.output.push(format!("ALTAR: Void Ritual at {},{}! The Abyss Gazes Back.", x, y));
        }
    }
}
