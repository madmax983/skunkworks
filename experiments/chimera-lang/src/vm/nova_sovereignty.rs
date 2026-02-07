#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "nova")]
use crate::vm::nova_market::MarketState;
#[cfg(feature = "nova")]
use crate::vm::GRID_SIZE;

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Policy {
    pub tax_rate: i64,
    pub grants: HashSet<usize>,
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyState {
    pub policies: HashMap<usize, Policy>, // Key: Owner Strand ID
    pub ownership: Vec<Vec<Option<usize>>>, // Grid of Owner IDs
}

#[cfg(feature = "nova")]
impl Default for SovereigntyState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "nova")]
impl SovereigntyState {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            ownership: vec![vec![None; GRID_SIZE]; GRID_SIZE],
        }
    }

    pub fn claim_territory(&mut self, owner: usize, radius: i64, cx: usize, cy: usize) -> usize {
        let mut count = 0;
        // Simple circular claim
        let r_sq = radius * radius;
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let dy = (y as i64) - (cy as i64);
                let dx = (x as i64) - (cx as i64);
                if dy * dy + dx * dx <= r_sq {
                    self.ownership[y][x] = Some(owner);
                    count += 1;
                }
            }
        }
        // Ensure policy exists
        self.policies.entry(owner).or_default();
        count
    }

    pub fn set_tax(&mut self, owner: usize, rate: i64) {
        let policy = self.policies.entry(owner).or_default();
        policy.tax_rate = rate.max(0);
    }

    pub fn grant_access(&mut self, owner: usize, target: usize) {
        let policy = self.policies.entry(owner).or_default();
        policy.grants.insert(target);
    }

    pub fn revoke_access(&mut self, owner: usize, target: usize) {
        if let Some(policy) = self.policies.get_mut(&owner) {
            policy.grants.remove(&target);
        }
    }

    pub fn get_owner(&self, y: usize, x: usize) -> Option<usize> {
        if y < GRID_SIZE && x < GRID_SIZE {
            self.ownership[y][x]
        } else {
            None
        }
    }

    /// Attempts to enter a territory.
    ///
    /// If the territory is owned by another strand, checks for grants.
    /// If no grant, attempts to pay tax from the mover's wallet to the owner's wallet.
    ///
    /// Returns Ok(()) if movement is allowed (free, granted, or paid).
    /// Returns Err(msg) if blocked (insufficient funds).
    pub fn try_enter_territory(
        &self,
        market: &mut MarketState,
        strand_idx: usize,
        to_y: usize,
        to_x: usize,
    ) -> Result<(), String> {
        if to_y >= GRID_SIZE || to_x >= GRID_SIZE {
            return Ok(()); // Out of bounds handled elsewhere or allowed
        }

        if let Some(owner) = self.ownership[to_y][to_x] {
            if owner == strand_idx {
                return Ok(()); // Own territory
            }

            if let Some(policy) = self.policies.get(&owner) {
                if policy.grants.contains(&strand_idx) {
                    return Ok(()); // Granted access
                }

                let tax = policy.tax_rate;
                if tax > 0 {
                    if market.debit(strand_idx, tax) {
                        market.credit(owner, tax);
                        return Ok(());
                    } else {
                        return Err(format!(
                            "BORDER CONTROL: Insufficient funds for tax {} (Owner: {})",
                            tax, owner
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
