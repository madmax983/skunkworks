use chimera_lang::compiler::compile;
use chimera_lang::vm::ChimeraVM;

fn main() {
    let source = r#"
        strand main {
            "Hello from Script" print
            42 print
        }
    "#;

    // Compile the source string into DNA
    // The second argument is an optional path for imports (None here)
    let dna = compile(source, None).expect("Failed to compile");

    let mut vm = ChimeraVM::new(dna);

    // Run until halted or for a max number of steps
    for _ in 0..100 {
        if vm.halted {
            break;
        }
        vm.step();
    }

    for line in &vm.output {
        println!("{}", line);
    }
}
