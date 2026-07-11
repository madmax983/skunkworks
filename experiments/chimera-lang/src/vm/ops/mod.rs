//! Basic Operation execution logic for the Chimera Virtual Machine.
//!
//! This module groups the fundamental low-level instructions (OpCodes) used by Chimera strands
//! into logical categories. Instead of cluttering the main VM code with hundreds of execution
//! branches, these submodules handle specific domains of operations:
//!
//! * [`bio`] - Biological operations (e.g., replication, mutation, apoptosis).
//! * [`flow`] - Control flow operations (e.g., jumps, branches, loops).
//! * [`grid`] - Grid interaction operations (e.g., reading/writing cells, moving).
//! * [`io`] - Input/Output operations (e.g., printing to logs).
//! * [`math`] - Mathematical operations (e.g., arithmetic, comparisons).
//! * [`stack`] - Stack manipulation operations (e.g., push, pop, dup, swap).

/// The `bio` module.
pub(crate) mod bio;
/// The `core_dispatch` module.
pub(crate) mod core_dispatch;
/// The `flow` module.
pub(crate) mod flow;
/// The `grid` module.
pub(crate) mod grid;
/// The `io` module.
pub(crate) mod io;
/// The `math` module.
pub(crate) mod math;
/// The `misc` module.
pub(crate) mod misc;
/// The `stack` module.
pub(crate) mod stack;

#[cfg(feature = "nova")]
/// The `nova_dispatch` module.
pub(crate) mod nova_dispatch;

pub(crate) enum Dispatch {
    Handled,
    Jump(usize, usize),
    Unhandled,
}

impl From<Option<(usize, usize)>> for Dispatch {
    fn from(opt: Option<(usize, usize)>) -> Self {
        match opt {
            Some((i, j)) => Dispatch::Jump(i, j),
            None => Dispatch::Handled,
        }
    }
}
