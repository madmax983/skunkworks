# Bard's Journal 🎻

## Philosophy
- If it isn't documented, it doesn't exist.
- A good example is worth 1,000 lines of explanation.
- Error messages are the first line of documentation—make them helpful.
- Your target audience is a tired developer at 3 AM. Be kind to them.

## Critical Learnings
(Entries will be added here as new "magic" behaviors or critical misunderstandings are discovered.)

## 2024-05-22 - Binary Crate Doc Tests
**Confusion:** Doc tests in `src/main.rs` (or modules of a binary crate) fail to compile because they cannot link against the binary itself.
**Clarification:** Use ````rust,ignore` for examples in binary crates, or structure the app as a library + thin binary wrapper if runnable examples are critical.
