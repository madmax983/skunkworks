# Func Arena 🏟️

Welcome to the **Code Colosseum**, where functions fight for dominance!

This experiment gamifies the codebase by turning Rust functions into RPG characters. It parses the source code, extracts metrics, and simulates a turn-based battle.

## 📊 The Stats

Every function is scanned and assigned stats based on its structure:

- **HP (Health Points)**: Derived from **Lines of Code**.
  - *Logic:* Longer functions are harder to kill (refactor/rewrite).
  - *Formula:* `(Lines * 5) + 50`.

- **Attack (Complexity)**: Derived from **Cyclomatic Complexity** (Branching).
  - *Logic:* Complex code with many loops/ifs is dangerous and prone to bugs (damage).
  - *Formula:* `Complexity Score * 2`.

- **Defense (Shield)**: Derived from **Argument Count**.
  - *Logic:* Functions with many dependencies (arguments) are harder to hit cleanly.
  - *Formula:* `Args * 2`.

- **Speed (Initiative)**: Derived from **Name Length**.
  - *Logic:* Short names are typed faster!
  - *Formula:* `40 - NameLength`.

## 🎮 Controls

- **SPACE**: Step one turn manually.
- **ENTER**: Toggle Auto-Play.
- **R**: Restart with two new random functions.
- **Q / ESC**: Quit.

## 🚀 Usage

Run from the workspace root:

```bash
cargo run -p func-arena -- [path_to_scan]
```

Example:
```bash
cargo run -p func-arena -- experiments/func-arena/src
```

## 🧠 Theory

This experiment explores:
1.  **Gamification of Metrics**: Can we make "bad code" (high complexity) feel powerful in a game context? (Yes, it has high Attack!)
2.  **Static Analysis**: Using `syn` to parse Rust code into an AST.
3.  **Procedural Generation**: The "content" of the game is the code itself.
