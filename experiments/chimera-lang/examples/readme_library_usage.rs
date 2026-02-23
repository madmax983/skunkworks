use chimera_lang::prelude::*;

fn main() {
    println!("🧬 Echo's Improved Library Usage Example");

    // Old way (Verbose):
    // let genes = vec![
    //     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
    //     Gene { op: OpCode::Print, args: vec![] },
    // ];

    // New way (Simplified):
    let genes = vec![
        // Use .into() for simple Nucleotide conversion
        Gene::new(OpCode::Push, vec![42.into()]),
        // Use .into() for OpCodes with no arguments
        OpCode::Print.into(),
        // Strings work too
        Gene::new(OpCode::Push, vec!["Hello World".into()]),
        OpCode::Print.into(),
    ];

    // Helper to create a single-strand DNA
    let dna = Dna::from_genes(genes);

    let mut vm = ChimeraVM::new(dna);

    println!("Executing VM steps...");
    // Run until the strand is finished
    // The VM automatically halts when the IP goes past the last strand
    while !vm.halted {
        vm.step();
    }

    println!("VM Output:");
    for line in &vm.output {
        println!("  > {}", line);
    }
}
