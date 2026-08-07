1. **Goal**: Add `prolouge` block to `chimera-lang` to fulfill the "Prolouge: Mad Scientist of Language" role.
2. **Steps**:
   - Run a `bash` script to update `experiments/chimera-lang/src/prologue_esolang_grammar.pest` to include `prolouge_block = { "prolouge" ~ "{" ~ (!"}" ~ ANY)* ~ "}" }` and add it to `section`.
   - Run a `bash` script to update `experiments/chimera-lang/src/opcode.rs` using `sed` with anchor `Orca,` to add `Prolouge,`.
   - Run a `bash` script to update `experiments/chimera-lang/src/prologue_esolang_compiler.rs` using `sed` with anchor `Rule::lisp_block => {` to match `Rule::prolouge_block` and push `Gene::new(OpCode::Prolouge, vec![])`.
   - Run a `bash` script to update `experiments/chimera-lang/src/vm/mod.rs` using `sed` with anchor `OpCode::Nop => None,` to handle `OpCode::Prolouge` execution by calling `self.output.push(...)`.
   - Run `cargo check -p chimera-lang` to verify compilation.
   - Run a `bash` script using `sed` to insert the new test before the last closing brace in `experiments/chimera-lang/tests/prologue_esolang_mad_scientist_test.rs`.
   - Run `cargo test -p chimera-lang` and `cargo clippy -p chimera-lang` to ensure tests pass and code is clean.
   - Run `pre_commit_instructions` tool to get the pre-commit steps.
   - Run the `submit` tool to finalize the task.
