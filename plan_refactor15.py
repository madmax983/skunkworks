# Let's write the `is_grid_navigable` method in state.rs:
import re
with open("experiments/chimera-lang/src/tui/state.rs", "r") as f:
    text = f.read()

# Is `is_grid_navigable` already there? No.
print(text.find("is_grid_navigable"))
