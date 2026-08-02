# Chimera Esolang (Genesis) ⚛️

Genesis is a highly experimental esoteric language designed to cross paradigms: Orca grid layouts, Forth stacks, Prolog logic, Raku operators, and Genetic mutations, compiled down into `chimera-lang` DNA.

## 🧬 Grammar and Syntax

A Genesis program consists of declarations.

### Strands (Forth/Genetics)
Defines a sequence of instructions.
```genesis
strand main {
    "Hello Genesis"
    print
}
```

### Organelles (Concurrent Agents)
Defines autonomous sub-units.
```genesis
organelle worker {
    dup
    mutate
}
```

### Grids (Orca inspired)
Defines a spatial layout.
```genesis
grid layout {
    | A B C |
    | D E F |
}
```

### Rules (Prolog inspired)
Defines logic rules.
```genesis
rule ancestor(?X) :- parent(?X).
```

### Operators
Genesis supports Forth-like stack manipulation (`dup`, `drop`, `swap`, `over`, `rot`), standard math operators (`+`, `-`, `*`, `/`, `%`), meta operators (`<<`, `>>`), and genetic operators (`mutate`, `crossover`, `transcribe`).

## 🚀 Usage
```rust
# fn main() {
use chimera_esolang::parse;
use chimera_esolang::compile;
use chimera_lang::vm::ChimeraVM;

let source = "strand main { \"Hello Genesis\" print }";
let program = parse(source).unwrap();
let dna = compile(&program).unwrap();
let vm = ChimeraVM::new(dna);
# }
```

## ⚠️ Warning
Running Genesis triggers Mad Scientist mode (`PrologueEsolang`). Extreme genetic chaos and random Orca signal bursts will flood the grid.
