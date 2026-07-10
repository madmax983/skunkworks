use chimera_lang::ast::{Dna, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

fn main() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Hello".to_string())] },
        Gene { op: OpCode::Print, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
        Gene { op: OpCode::Print, args: vec![] },
    ];
    let dna = Dna::from_genes(genes);
    let mut vm = ChimeraVM::new(dna);

    vm.step(); // Push "Hello"
    vm.step(); // Print "Hello"
    vm.step(); // Push 42
    vm.step(); // Print 42

    for line in &vm.output {
        println!("{}", line);
    }
}
