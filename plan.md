1. **Optimize String Iteration in `exec_brainfuck`**
   - In `experiments/chimera-lang/src/vm/nova_brainfuck.rs`, `exec_brainfuck` iterates over strings by doing `let code_chars: Vec<char> = code.chars().collect();`. This creates an unnecessary intermediate heap allocation.
   - It also does `let mut input_chars: VecDeque<u8> = input.bytes().collect::<VecDeque<_>>();`. We can replace this with `let mut input_bytes = input.bytes();` and consume the iterator directly.
   - We can change `code_chars` to `let code_bytes = code.as_bytes();` since Brainfuck operates on ASCII chars (`[`, `]`, `+`, `-`, `>`, `<`, `.`, `,`). This avoids `char` decoding and the intermediate `Vec` completely. Wait, `String::as_bytes()` works perfectly for brainfuck commands since all valid commands are 1-byte ASCII.
   - Let's check `exec_brainfuck` logic: it does jumping by precomputing `jumps`. We can iterate over `code.as_bytes()` to precompute `jumps`, and then index `code.as_bytes()[pc]` during execution. This avoids the `Vec<char>` allocation.

   **Pre-commit checks**
   - Run `pre_commit_instructions` to ensure proper testing, verification, review, and reflection are done.

   **Measurement**
   - Avoids `O(n)` heap allocations per Brainfuck execution where `n` is code length.
