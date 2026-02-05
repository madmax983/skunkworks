# Git Rogue

A TUI-based rogue-like explorer for your git history. Crawl through commits, battle bugs, and discover features!

## 🎮 How to Play

Run the game in any git repository:

```bash
cargo run --release
```

**Objective:** Navigate from HEAD back to the initial commit (or as far as you can go) without running out of HP.

### Mechanics

*   **🐛 Bugs (Damage):** Commits containing `fix`, `bug`, `panic`, or `error` are unstable! They deal damage to your HP.
*   **✨ Features (XP/Heal):** Commits containing `feat`, `add`, or `new` are rewarding! They grant XP and restore a small amount of HP.
*   **🐉 Merge Dragons:** Merge commits are dangerous bosses. Beware!

### Controls

*   `1-9`: Go to **Parent** (Move backward in time).
*   `Shift + 1-9`: Go to **Child** (Move forward in time).
*   `Q`: Quit the game.

## 🎨 UI Polish (Mosaic)

This tool features a polished TUI interface designed by Mosaic:
*   **Visual Feedback:** HP Bar changes color (Green -> Yellow -> Red) as you take damage.
*   **Aesthetics:** Rounded borders and emoji indicators (📍, 🚪, 📜) for a modern terminal look.
*   **Readability:** Log messages are color-coded to highlight damage (Red) and rewards (Green).

## License

MIT
