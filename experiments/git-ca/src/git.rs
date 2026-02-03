use crate::grid::{Grid, HEIGHT, WIDTH};
use anyhow::Result;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::process::Command;

pub fn get_head_diff_hash() -> Result<u64> {
    // run git show HEAD --stat -p (full diff)
    let output = Command::new("git")
        .args(["show", "HEAD", "--stat", "-p", "--pretty=format:%H"])
        .output()?;

    let content = String::from_utf8(output.stdout)?;
    // If output is empty, maybe try just HEAD
    if content.trim().is_empty() {
        return Ok(0);
    }
    Ok(hash_string(&content))
}

pub fn hash_string(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        // hash * 33 + c
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64);
    }
    hash
}

pub fn seed_grid(grid: &mut Grid, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if rng.gen_bool(0.2) {
                // 20% density
                grid.toggle(x, y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn test_hashing_determinism() {
        let input = "diff --git a/src/main.rs b/src/main.rs";
        let h1 = hash_string(input);
        let h2 = hash_string(input);
        assert_eq!(h1, h2);

        let h3 = hash_string("different");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_seeding_determinism() {
        let mut g1 = Grid::new();
        let mut g2 = Grid::new();
        let seed = 12345;

        seed_grid(&mut g1, seed);
        seed_grid(&mut g2, seed);

        assert_eq!(g1, g2);

        let mut g3 = Grid::new();
        seed_grid(&mut g3, 67890);
        assert_ne!(g1, g3); // Unlikely to be same
    }
}
