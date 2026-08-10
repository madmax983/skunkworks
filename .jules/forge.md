# Forge's Journal

## Removing Panics in Stack Pops
**Learning:** Even when stack bounds are checked via `stack.len() >= N`, calling `.unwrap()` is dangerous because it assumes internal logic or bounds checking won't change. However, when writing guard clauses (e.g. `let Some(val) = stack.pop() else { return None; }`), if the enclosing function returns an `Option`, using the `?` operator (e.g., `let val = stack.pop()?;`) is significantly cleaner and idiomatic.
**Action:** When popping from a stack in a function returning `Option`, use the `?` try operator. For functions returning `()` or `bool`, use `let Some(...) = stack.pop() else { return ...; };`. Rely on `cargo clippy --fix` to automate `?` suggestions.
**[Flattened Math Operations]**
**Learning:** Refactoring deeply nested match statements in a virtual machine's op dispatcher often requires extracting specific sub-operations (like arithmetic and comparison logic) into isolated helper methods. Utilizing `matches!` macros, `if let` guard clauses, and early returns simplifies the reading path significantly and prevents the 'Pyramid of Doom'.
**Action:** Always verify stack state before mutation in extracted helpers and ensure error messages are identical to preserve exact VM semantics.
**[Replacing Enum Match Boilerplate with New/From]**
**Learning:** Repetitive initialization logic that maps an enum to default struct fields (like default amplitude for different audio events) clutters the main logic loop.
**Action:** Extract this logic into an implementation block `fn new(kind: AudioEvent) -> Self` or similar constructor. This separates the definition of defaults from the processing loop.
**[Unnecessary String Reference in `draw_text`]**
**Learning:** Functions accepting `impl AsRef<str>` (such as `draw_text` in `macroquad`) do not require explicitly borrowing strings (`&format!(...)`). Explicitly borrowing creates a `&String` which is then coerced into a `&str`, making it an unnecessary borrow that clippy will flag.
**Action:** When passing a `String` (like the output of `format!()`) to a function accepting `impl AsRef<str>`, pass the `String` directly without the `&` reference operator.
**[Refactor duplicated logic in UI rendering]**
**Learning:** Extract repeated logic like styling cells based on values to helper functions.
**Action:** Check for duplicated formatting logic within a UI/rendering component and extract to private helper methods.
**[Extract God Function and Flatten Match Boilerplate]**
**Learning:** Deeply nested match blocks with duplicate branches on enums or primitives create visual noise and errors over time. A massive `match` statement in a single God Function can often be radically simplified.
**Action:** Search for large `matches!` invocations or repetitive `match` statements where a `HashSet`, static array scan, or functional extractor can replace the boilerplate. Extract logic out of God Functions into static helper functions when possible.
