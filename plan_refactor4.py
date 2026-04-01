import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# I will write a Rust helper in the same file `navigation.rs` that returns a boolean if a view supports navigating the grid (x, y).
# Actually, if we look at `ViewMode` enum, it has many variants.
# We can just extract the grid cursor moving logic to a simple check before or after the match.
# Wait, if we extract it, we replace all those identical `if app_state.grid_cursor.1 < 15` branches with a single `if is_grid_navigable(mode)` block!

# Let's count how many branches are literally just `{ if app_state.grid_cursor.X ... }`
# And there's also Pandemonium cursor.
