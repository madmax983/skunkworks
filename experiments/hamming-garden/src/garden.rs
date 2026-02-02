use crate::hamming::{self, HammingStatus};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Cell {
    #[allow(dead_code)]
    pub original_nibble: u8, // The original 4 bits (0-15)
    pub encoded_byte: u8, // The current 8 bits (potentially corrupted)
    pub status: HammingStatus,
}

impl Cell {
    pub fn new(nibble: u8) -> Self {
        let encoded = hamming::encode(nibble & 0x0F);
        Self {
            original_nibble: nibble & 0x0F,
            encoded_byte: encoded,
            status: HammingStatus::Clean,
        }
    }
}

pub struct Garden {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub entropy_rate: f64,  // Probability of a bit flip per tick per cell
    pub repair_rate: usize, // Number of cells visited by gardener per tick
    pub total_flips: u64,
    pub total_repairs: u64,
    pub total_deaths: u64, // Unrecoverable errors
}

impl Garden {
    pub fn new(width: usize, height: usize, initial_text: &str) -> Self {
        let mut cells = Vec::with_capacity(width * height);

        // Convert text to nibbles
        for b in initial_text.bytes() {
            let high = (b >> 4) & 0x0F;
            let low = b & 0x0F;
            cells.push(Cell::new(high));
            cells.push(Cell::new(low));
        }

        // Fill the rest with zeros if text is too short
        while cells.len() < width * height {
            cells.push(Cell::new(0));
        }

        // Truncate if too long
        cells.truncate(width * height);

        Self {
            cells,
            width,
            height,
            entropy_rate: 0.0001,
            repair_rate: 10,
            total_flips: 0,
            total_repairs: 0,
            total_deaths: 0,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Entropy Phase: Random bit flips
        // Iterate over all cells? Or pick random cells?
        // Iterating over all might be slow if grid is huge, but for TUI it's fine.
        // Let's pick a number of flips based on rate.
        let num_flips = (self.cells.len() as f64 * self.entropy_rate * 8.0) as usize;

        for _ in 0..num_flips {
            let idx = rng.gen_range(0..self.cells.len());
            let bit = rng.gen_range(0..8);

            // Only flip if not dead? Or can dead cells rot further?
            // "Dead" status is just a flag from the last decode.
            // The byte exists.
            self.cells[idx].encoded_byte ^= 1 << bit;
            self.total_flips += 1;
        }

        // Gardener Phase: Visit random cells and repair
        for _ in 0..self.repair_rate {
            let idx = rng.gen_range(0..self.cells.len());
            let cell = &mut self.cells[idx];

            let (decoded, status) = hamming::decode(cell.encoded_byte);

            match status {
                HammingStatus::Clean => {
                    cell.status = HammingStatus::Clean;
                }
                HammingStatus::Corrected(_) => {
                    // Gardener repairs it
                    // We re-encode the corrected data (or just use the corrected bit pattern?
                    // decode returns the 4-bit data. We should re-encode it to be safe and clean)
                    cell.encoded_byte = hamming::encode(decoded);
                    cell.status = status; // Mark as recently corrected
                    self.total_repairs += 1;
                }
                HammingStatus::DoubleError => {
                    // Gardener cannot repair.
                    cell.status = HammingStatus::DoubleError;
                    self.total_deaths += 1;
                    // Does the gardener give up? Yes.
                    // The byte remains corrupted.
                }
            }
        }
    }
}
