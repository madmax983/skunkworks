import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# Instead of repeating the ViewMode::X matching logic over and over, we can extract the `grid_cursor` moving out of the `match key_code` into a helper function or a single `if` statement check if we are in one of the views that support grid moving.
# But actually, let's see how `crossterm::event::KeyCode` is used.
# If we check if a view is a "grid view", we can handle cursor movement uniformly!
