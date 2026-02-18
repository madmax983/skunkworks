# Lexical Drift ⚛️📜

> "Languages aren't designed—they evolve."

**Lexical Drift** is a "Moonshot" experiment that simulates the phonological evolution of Rust source code over time. It applies linguistic principles (Grimm's Law, Great Vowel Shift, Syncope, Lenition) to identifiers, transforming them as if the codebase were an ancient text being transmitted through centuries of oral tradition.

## Concept

This tool parses Rust code, identifies variables and function names, and applies a series of sound change rules to them. The result is a compilable (syntax-wise) but "evolved" version of the code that looks like a future dialect of Rust.

## Features

- **Phonological Engine**: Implements real linguistic sound changes:
    - **Grimm's Law**: `p` -> `f`, `t` -> `th`, `k` -> `h`.
    - **Great Vowel Shift**: `a` -> `e`, `e` -> `i`, `i` -> `ai`.
    - **Syncope**: Loss of unstressed vowels.
    - **Lenition**: Weakening of intervocalic consonants (`t` -> `d` -> `th`).
    - **Palatalization**: `k` -> `ch` before front vowels.
- **Source Mutation**: Uses `syn` to parse and `proc-macro2` spans to surgically replace identifiers in the original source text, preserving layout and comments.
- **TUI Visualization**: A `ratatui` interface to watch the code age in real-time.

## usage

```bash
cargo run -p lexical-drift
```

Controls:
- `n`: Advance to the next generation (century).
- `Up`/`Down`: Scroll the source code.
- `q`: Quit.

## The Stack

- **Parsing**: `syn`, `quote`, `proc-macro2`
- **Visualization**: `ratatui`, `crossterm`
- **Logic**: Custom phonology engine, `rand`

## Philosophical Note

Code is often treated as static text. But what if it were a living organism, subject to the same entropic forces as natural language? `lexical-drift` explores this question by turning refactoring into a natural process of erosion and mutation.
