# Sentry's Journal

**[Unwrap Panics in TUI Enter Handlers]**
**Learning:** We found a panic risk in `handle_genome_enter` where `pairs.next().unwrap()` is called without first checking if the parser output actually has a next element. Even if it parses successfully, it might return an empty sequence depending on the grammar, leading to an index out of bounds or `unwrap()` crash on empty strings or comment-only strings.
**Action:** Replace `unwrap()` with a safe `.next()` guard or pattern matching, and write tests to handle edge cases like empty strings.
**[Unwrap Panics in apply_glitch_fx]**
**Learning:** Found a panic risk in `apply_glitch_fx` where `buffer.cell_mut((x,y)).unwrap()` was called inside a grid traversal loop. If the calculated coordinates somehow fell out of bounds (which is possible if the underlying window resizes out of sync with the logic, or given bounds logic quirks in `ratatui`), it would panic and crash the TUI.
**Action:** Replace `unwrap()` with a safe `if let Some(cell) = buffer.cell_mut((x,y))` guard, and write tests to handle out of bounds or empty buffer edge cases without panicking.

**[Acoustic Compiler AST Parsing]**
**Learning:** When navigating Pest AST pairs using `.into_inner().next()`, assuming the inner pairs exist via `.unwrap()` is dangerous because grammar definitions might change or incomplete syntax streams could bypass initial validation (though unlikely, defense-in-depth is best).
**Action:** Always replace iterator `.unwrap()` calls in AST parsing code with safe fallback error propagation like `.ok_or_else(|| anyhow!("expected node"))?`.

**[Unwrap Panics in AST Parsing]**
**Learning:** Found and removed dozens of `.unwrap()` calls on iterators when parsing Pest AST nodes. Even if a grammar enforces a structure, parsing errors or mid-parse failures should be gracefully bubbled up rather than causing a fatal panic.
**Action:** Replace `inner.next().unwrap()` with `inner.next().ok_or_else(|| anyhow!("Expected ..."))?` in compiler passes to gracefully handle incomplete ASTs or parsing errors, especially when parsing nested blocks or definition arguments.
**[Quipu Recursive Formatting Helpers]**
**Learning:** Achieving 100% line coverage for internal debugging helper structs (`DebugCappedCord` and `DebugSubsidiaries`) which cap `fmt::Debug` recursion limits is extremely difficult from outside the crate due to privacy bounds, macro resolution, and how deeply nested structs hit recursion caps. Tarpaulin struggles to mark lines 326, 330, 331, 343, and 351 as covered despite multiple angles of attack.
**Action:** Accept >95% coverage on recursive debug wrappers as long as the primary logic (like `checked_add`, edge case construction, and trait derivations) are rigorously tested and prevent panics.
**[Unwrap Panics on Stack Pops]**
**Learning:** Found and removed dozens of `.unwrap()` calls on stack pops in various module execution contexts (like `nova_genetics`, `nova_fluid`, etc.). A malformed DNA script running in the ChimeraVM can cause the stack to be smaller than expected. Popping from an empty stack causes fatal crashes.
**Action:** Always replace `.unwrap()` with idiomatic rust like `let Some(val) = vm.stack.pop() else { return; };` to silently stop execution of the op or return an error/`None`. Never trust the stack has elements just because the script called the OpCode.
