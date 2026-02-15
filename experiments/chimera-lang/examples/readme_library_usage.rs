use chimera_lang::prelude::*;

fn main() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    // ... configure VM ...
    vm.step();
}
