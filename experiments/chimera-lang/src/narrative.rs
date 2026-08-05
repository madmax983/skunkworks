//! # Narrative API
//!
//! Provides a fluent builder pattern to simplify the construction of a `ChimeraVM`
//! populated with narrative elements, logic grids, and genes.
//!
//! This module abstracts away the low-level complexities of raw grid and AST manipulation.
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::ChimeraVM;

/// A high-level builder API to easily construct Chimera VMs with story elements.
/// This prevents the need for manual grid manipulation and reversed stack pushing.
pub struct NarrativeBuilder {
    vm: ChimeraVM,
    current_x: usize,
    current_y: usize,
}

impl Default for NarrativeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NarrativeBuilder {
    /// Creates a new, empty NarrativeBuilder.
    pub fn new() -> Self {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        Self {
            vm: ChimeraVM::new(dna),
            current_x: 0,
            current_y: 0,
        }
    }

    /// Pushes a string command/value onto the current grid row.
    pub fn push_str(mut self, s: &str) -> Self {
        self.vm.grid[self.current_y][self.current_x] = Value::Str(s.to_string());
        self.current_x += 1;
        self
    }

    /// Pushes an integer value onto the current grid row.
    pub fn push_int(mut self, i: i64) -> Self {
        self.vm.grid[self.current_y][self.current_x] = Value::Int(i);
        self.current_x += 1;
        self
    }

    /// Adds a reader strand that will execute the current row of commands.
    pub fn add_reader_strand(mut self) -> Self {
        let reader_strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(self.current_x as i64)],
                }, // len
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(self.current_y as i64)],
                }, // y
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)], // x=0 for start of this row
                }, // x
                Gene {
                    op: OpCode::Incubate,
                    args: vec![],
                },
            ],
        };
        self.vm.dna.helix.strands.push(reader_strand);
        self.current_y += 1;
        self.current_x = 0;
        self
    }

    /// Builds and returns the resulting ChimeraVM.
    pub fn build(self) -> ChimeraVM {
        self.vm
    }
}
