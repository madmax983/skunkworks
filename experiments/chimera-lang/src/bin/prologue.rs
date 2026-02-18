use chimera_lang::prelude::*;
use chimera_lang::tui::{run_tui, ViewMode};

fn main() -> anyhow::Result<()> {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    run_tui(vm, Some(ViewMode::Prologue))
}
