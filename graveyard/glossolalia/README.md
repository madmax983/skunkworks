# Glossolalia 🗣️⚛️

> "The whole earth was of one language, and of one speech." - Genesis 11:1

**Glossolalia** is a Moonshot experiment combining **Phonological Evolution** and **Code Obfuscation**. It treats source code as a living language, subjecting its identifiers and keywords to historical sound changes (like Grimm's Law, the Great Vowel Shift, and Palatalization).

## Concept

What if `fn main()` evolved over centuries like `pater` evolved into `father`?
Glossolalia simulates this drift, creating new "dialects" of Rust.

- **Proto-Code**: The original source.
- **Sound Change Engine**: Applies rules like $P \to F$, $T \to Th$, $A \to E$.
- **Obfuscator**: Maintains a dictionary of evolved words to ensure consistency (e.g., all instances of `let` evolve together).

## Usage

Run the TUI:

```bash
cargo run -p glossolalia
```

- **Space**: Advance one generation (apply more sound changes).
- **R**: Reset to Proto-Code.
- **Q**: Quit.

## Tech Stack

- **Ratatui**: Terminal UI.
- **Logos**: Lexical analysis.
- **Rand**: Probabilistic evolution.
