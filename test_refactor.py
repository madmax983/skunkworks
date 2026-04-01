import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    lines = f.readlines()

for line in lines:
    if "ViewMode::" in line and "app_state.grid_cursor" in line:
        pass
