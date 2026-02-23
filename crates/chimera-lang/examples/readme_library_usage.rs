use chimera_lang::prelude::*;

fn main() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    // ... configure VM ...
    vm.step();
}
