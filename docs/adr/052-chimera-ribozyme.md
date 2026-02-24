# 052. Chimera Ribozyme System

## Status
Accepted

## Context
The Chimera language is fundamentally stack-based and concatenative (like Forth or Joy). This is excellent for genetic algorithms because it allows for easy mutation and recombination (any sequence of instructions is valid). However, it lacks expressiveness for higher-order logic, metaprogramming, and complex data transformations.

As the complexity of Chimera organisms grew, we needed a way to:
1.  Treat code as data (Homoiconicity) to allow organisms to rewrite their own logic safely.
2.  Perform operations on collections (Junctions) without manual loops.
3.  Support a more human-readable syntax for complex logic (Lisp-like S-expressions) that compiles down to the same DNA.

## Decision
We decided to introduce the **Ribozyme** feature set, which adds Functional Programming capabilities to the VM.

Key components:
*   **Lisp Parser (`lisp.rs`):** A recursive descent parser that compiles S-expressions into Chimera DNA. This allows writing code in a Lisp-like syntax (e.g., `(map (push 1 add) [1 2 3])`) which is compiled to `[1 2 3] [1 add] map`.
*   **Functional OpCodes:**
    *   `Eval`: Executes a string as code.
    *   `Map`, `Fold`, `Filter`, `Zip`: Higher-order functions that operate on Junctions (lists).
    *   `Curry`, `Chain`: Function composition tools.
    *   `Quote`: Prevents execution of the next term, treating it as data.

## Consequences

### Positive
*   **Expressiveness:** Complex algorithms (sorting, searching, transforming) can be written in a few instructions using `Map`/`Filter`.
*   **Metaprogramming:** Organisms can generate code strings and `Eval` them, allowing for runtime adaptation.
*   **Dual Syntax:** Developers can choose between Concatenative (Stack) and Applicative (Lisp) styles depending on the problem.

### Negative
*   **Complexity:** The VM now includes a full Lisp parser and runtime, increasing the binary size and maintenance burden.
*   **Performance:** `Eval` requires parsing strings at runtime, which is slower than executing pre-compiled DNA.
*   **Security:** `Eval` opens up potential vectors for executing malicious code if input is not sanitized (though the VM is sandboxed).
