use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn main() {
    println!("🗣️ Echo's Story Demo");
    run_demo();
}

fn run_demo() {
    // 1. Initialize empty VM
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);

    // 2. Write "Story Elements" to the Petri Dish
    // We'll put them in a row at y=0
    println!("✍️  Writing story elements to Petri Dish...");
    // Let's create a story: "Once upon a time, there were 10 dragons."
    // In Chimera: push(10) print()
    vm.grid[0][0] = Value::Str("push".to_string());
    vm.grid[0][1] = Value::Int(10); // Argument for push
    vm.grid[0][2] = Value::Str("print".to_string());

    // 3. Create a "Reader" strand that incubates the story
    // incubate(len, y, x) -> creates new strand from grid cells
    // We push args in reverse order because stack: len, y, x (top)
    let reader_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(3)],
            }, // len
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::Incubate,
                args: vec![],
            },
        ],
    };

    vm.dna.helix.strands.push(reader_strand);

    println!("🧪 Incubating narrative...");

    // Run the VM
    let mut steps = 0;
    while !vm.halted && steps < 20 {
        vm.step();
        steps += 1;
    }

    println!("📜 Output Log:");
    for line in &vm.output {
        println!("  {}", line);
    }

    // Verify results
    if vm.output.iter().any(|s| s.contains("10")) {
        println!("✅ Success: The story was told (10 was printed)!");
    } else {
        println!("⚠️  Warning: The story was not told.");
    }
}
