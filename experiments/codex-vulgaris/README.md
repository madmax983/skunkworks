# Codex Vulgaris 📜

> "The language of the machine is not immune to the ravages of time."

**Codex Vulgaris** is a philological experiment that treats source code identifiers as living words in a natural language. It applies historical sound change laws (Grimm's Law, Great Vowel Shift, Lenition) to simulate how your code might sound if it evolved over centuries of oral tradition.

## Concept

Code is usually static, but what if it drifted?
- `calculate_sum` -> *Grimm's Law* -> `halhulate_thum`
- `let mut variable` -> *Vowel Shift* -> `let mut veriable`

This tool visualizes this decay (or evolution) in real-time.

## Features

- **Phonological Engine:** Implements `Grimm's Law`, `Great Vowel Shift`, `Lenition` (intervocalic voicing), and `Assimilation`.
- **Lexical Analysis:** Uses `logos` to tokenize Rust code, separating immutable keywords from evolving identifiers.
- **Etymological Dictionary:** Tracks the original form of every mutated word.
- **Time Travel:** Move forward and backward through "Eras" of linguistic change.

## Controls

- `Left` / `Right`: Travel through time (Eras).
- `?`: Toggle Help.
- `q`: Quit.

## The Linguistic Laws

1.  **Grimm's Law:** Proto-Germanic shift. Unvoiced stops become fricatives (`p`->`f`, `t`->`th`, `k`->`h`).
2.  **Great Vowel Shift:** English vowel raising (`a`->`e`->`i`->`ai`).
3.  **Lenition:** Softening of consonants between vowels (`p`->`b`, `t`->`d`).
4.  **Assimilation:** Consonants adapting to neighbors (`n` -> `m` before `p`).

## Running

```bash
cargo run -p codex-vulgaris
```
