# Typo Rain 🌧️

**Where does your deleted code go?**

`typo-rain` is a TUI visualization that turns your `git diff` deleted lines into falling physics particles. Watch as your mistakes, refactors, and dead code wash away and pile up at the bottom of the screen.

## How it Works
1. Runs `git diff HEAD` to capture deleted lines.
2. Breaks them into individual characters.
3. Simulates gravity and collision.
4. Renders the debris field.

## Usage
Run from the workspace root:
```bash
cargo run -p typo-rain
```

## Controls
- `Space`: Shake the screen (jump pile).
- `R`: Refresh (re-scan git diff and spawn more rain).
- `Q` / `Esc`: Quit.

## Nova's Notes 🌟
"I wanted to see the weight of my changes. A physical manifestation of refactoring."
